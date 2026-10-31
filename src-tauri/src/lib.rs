use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};
use tauri_plugin_dialog::{DialogExt, FilePath};

pub struct WindowControlState {
    pub is_dialog_open: AtomicBool,
    pub is_pinned: AtomicBool,
    pub is_dragging: AtomicBool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiskInfo {
    pub total: String,
    pub used: String,
    pub free: String,
    pub percent: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiskStats {
    pub internal: DiskInfo,
    pub external: DiskInfo,
    pub external_connected: bool,
    pub target_volume_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MountedVolume {
    pub name: String,
    pub path: String,
    pub free: String,
    pub total: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubItem {
    pub name: String,
    pub size: String,
    pub is_recommended: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FolderScanResult {
    pub size: String,
    pub items: Vec<SubItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SymlinkItem {
    pub name: String,
    pub path: String,
    pub destination: String,
    pub size: String,
    pub is_missing_symlink: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionStatus {
    pub full_disk_access: bool,
    pub external_drive_access: bool,
    pub all_granted: bool,
}

// 1. Check macOS Permissions (Full Disk Access & External Volume)
#[tauri::command]
fn check_permissions(target_mount: Option<String>) -> Result<PermissionStatus, String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/Users/indunilperamuna".into());
    let mount = target_mount.unwrap_or_else(|| "/Volumes/DataDisc2".into());

    // Test FDA: Try reading protected MobileSync or Safari directory
    let fda_test_path = format!("{home}/Library/Application Support/MobileSync");
    let full_disk_access = std::fs::read_dir(Path::new(&fda_test_path)).is_ok()
        || std::fs::read_dir(Path::new(&format!("{home}/Library/Safari"))).is_ok();

    // Test External Drive Access: Try writing a small probe file
    let external_drive_access = if Path::new(&mount).exists() {
        let probe_file = format!("{mount}/.offload_perm_probe");
        if std::fs::write(&probe_file, b"ok").is_ok() {
            let _ = std::fs::remove_file(&probe_file);
            true
        } else {
            false
        }
    } else {
        false
    };

    let all_granted = full_disk_access && external_drive_access;

    Ok(PermissionStatus {
        full_disk_access,
        external_drive_access,
        all_granted,
    })
}

// 2. Open macOS System Settings directly to Privacy Pane
#[tauri::command]
fn open_privacy_settings(pane: Option<String>) -> Result<(), String> {
    let url = match pane.as_deref() {
        Some("removable") => "x-apple.systempreferences:com.apple.preference.security?Privacy_RemovableVolumes",
        _ => "x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles",
    };

    Command::new("open")
        .arg(url)
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(())
}

// 3. Get Live Disk Space (Targets /System/Volumes/Data on macOS)
#[tauri::command]
fn get_disk_stats(target_mount: Option<String>) -> Result<DiskStats, String> {
    let mount_to_check = target_mount.unwrap_or_else(|| "/Volumes/DataDisc2".into());

    let output = Command::new("df")
        .args(["-h", "/System/Volumes/Data", &mount_to_check])
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    let mut internal = DiskInfo {
        total: "460Gi".into(),
        used: "397Gi".into(),
        free: "34Gi".into(),
        percent: 93,
    };

    let mut external = DiskInfo {
        total: "578Gi".into(),
        used: "188Gi".into(),
        free: "390Gi".into(),
        percent: 33,
    };
    let mut external_connected = false;

    for line in lines.iter().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 6 {
            let size = parts[1].to_string();
            let used = parts[2].to_string();
            let avail = parts[3].to_string();
            let cap_str = parts[4].replace('%', "");
            let percent: u32 = cap_str.parse().unwrap_or(0);
            let mount = parts[parts.len() - 1];

            if mount == "/System/Volumes/Data" || mount == "/" {
                internal = DiskInfo {
                    total: size,
                    used,
                    free: avail,
                    percent,
                };
            } else if mount == mount_to_check || mount_to_check.starts_with(mount) {
                external = DiskInfo {
                    total: size,
                    used,
                    free: avail,
                    percent,
                };
                external_connected = true;
            }
        }
    }

    let target_volume_name = Path::new(&mount_to_check)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "External Disc".into());

    Ok(DiskStats {
        internal,
        external,
        external_connected,
        target_volume_name,
    })
}

// 4. Discover all mounted external volumes in /Volumes
#[tauri::command]
fn list_mounted_volumes() -> Result<Vec<MountedVolume>, String> {
    let mut volumes = Vec::new();

    let output = Command::new("df")
        .args(["-h"])
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 6 {
            let mount = parts[parts.len() - 1];
            if mount.starts_with("/Volumes/") && !mount.contains("Recovery") && !mount.contains("TimeMachine") {
                let path = Path::new(mount);
                if path.exists() && !path.is_symlink() {
                    let name = path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| mount.into());

                    volumes.push(MountedVolume {
                        name,
                        path: mount.to_string(),
                        free: parts[3].to_string(),
                        total: parts[1].to_string(),
                    });
                }
            }
        }
    }

    Ok(volumes)
}

// 5. Scan Folder & Detect Exceptions
#[tauri::command]
fn scan_folder(path: String) -> Result<FolderScanResult, String> {
    let folder_path = Path::new(&path);
    if !folder_path.exists() {
        return Err("Path does not exist".into());
    }

    let du_output = Command::new("du")
        .args(["-sh", &path])
        .output()
        .map_err(|e| e.to_string())?;

    let du_str = String::from_utf8_lossy(&du_output.stdout);
    let total_size = du_str
        .split_whitespace()
        .next()
        .unwrap_or("~")
        .to_string();

    let mut items = Vec::new();

    if let Ok(entries) = std::fs::read_dir(folder_path) {
        for entry in entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            let sub_path = entry.path();

            let is_recommended = matches!(
                file_name.as_str(),
                "node_modules"
                    | ".env"
                    | ".env.local"
                    | ".git"
                    | "vendor"
                    | "target"
                    | "build"
                    | "dist"
                    | "storage"
                    | ".idea"
                    | ".vscode"
            );

            let sub_size = if let Ok(sub_du) = Command::new("du")
                .args(["-sh", &sub_path.to_string_lossy()])
                .output()
            {
                let s = String::from_utf8_lossy(&sub_du.stdout);
                s.split_whitespace().next().unwrap_or("~").to_string()
            } else {
                "~".to_string()
            };

            items.push(SubItem {
                name: file_name,
                size: sub_size,
                is_recommended,
            });
        }
    }

    items.sort_by(|a, b| b.is_recommended.cmp(&a.is_recommended));

    Ok(FolderScanResult {
        size: total_size,
        items,
    })
}

// Helper to guess original local path for an offloaded target folder
fn guess_local_path(target_path: &Path, dest_base: &Path, home: &str) -> String {
    let rel = match target_path.strip_prefix(dest_base) {
        Ok(r) => r.to_string_lossy().to_string(),
        Err(_) => target_path.file_name().unwrap_or_default().to_string_lossy().to_string(),
    };

    if rel.starts_with("OrbStackData/") {
        format!("{home}/Library/Group Containers/HUAQ24HBR6.dev.orbstack/{}", &rel["OrbStackData/".len()..])
    } else if rel.starts_with("Android/sdk") {
        format!("{home}/Library/{rel}")
    } else if rel.starts_with("Android/avd") {
        format!("{home}/.android/{}", &rel["Android/".len()..])
    } else if rel.starts_with("MobileSync/") {
        format!("{home}/Library/Application Support/{rel}")
    } else if rel.starts_with("ollama/") {
        format!("{home}/.ollama/{}", &rel["ollama/".len()..])
    } else if rel.starts_with("ApplicationSupport/") {
        format!("{home}/Library/Application Support/{}", &rel["ApplicationSupport/".len()..])
    } else if rel.starts_with("private/") {
        format!("/{}", rel)
    } else {
        format!("{home}/{rel}")
    }
}

// 6. List Active Symlinks and detect any missing symlinks on target volume
#[tauri::command]
fn list_offloaded_items(
    target_mount: Option<String>,
    subfolder: Option<String>,
) -> Result<Vec<SymlinkItem>, String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/Users/indunilperamuna".into());
    let filter = target_mount.clone().unwrap_or_else(|| "/Volumes/".into());
    let mut results: Vec<SymlinkItem> = Vec::new();
    let mut mapped_targets: std::collections::HashSet<std::path::PathBuf> = std::collections::HashSet::new();

    let candidate_parents = vec![
        home.clone(),
        format!("{home}/Herd"),
        format!("{home}/Development"),
        format!("{home}/Htdocs"),
        format!("{home}/Projects"),
        format!("{home}/.android"),
        format!("{home}/Library/Android"),
        format!("{home}/Library/Application Support"),
        format!("{home}/Library/Application Support/MobileSync"),
        format!("{home}/Library/Group Containers"),
        format!("{home}/Library/Group Containers/HUAQ24HBR6.dev.orbstack"),
        format!("{home}/.ollama"),
        format!("{home}/.orbstack"),
    ];

    for parent in &candidate_parents {
        let p = Path::new(parent);
        if !p.exists() || p.is_symlink() {
            continue;
        }

        if let Ok(entries) = std::fs::read_dir(p) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_symlink() {
                    if let Ok(target) = std::fs::read_link(&path) {
                        let abs_target = if target.is_relative() {
                            p.join(&target)
                        } else {
                            target
                        };

                        let target_str = abs_target.to_string_lossy().to_string();
                        if target_str.contains(&filter) || target_str.starts_with("/Volumes/") {
                            let name = path
                                .file_name()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_else(|| "Unknown".into());

                            let size = if let Ok(du) = Command::new("du")
                                .args(["-sh", &target_str])
                                .output()
                            {
                                let s = String::from_utf8_lossy(&du.stdout);
                                s.split_whitespace().next().unwrap_or("~").to_string()
                            } else {
                                "~".to_string()
                            };

                            if let Ok(canonical) = std::fs::canonicalize(&abs_target) {
                                mapped_targets.insert(canonical);
                            } else {
                                mapped_targets.insert(abs_target.clone());
                            }

                            results.push(SymlinkItem {
                                name,
                                path: path.to_string_lossy().to_string(),
                                destination: target_str,
                                size,
                                is_missing_symlink: false,
                            });
                        }
                    }
                }
            }
        }
    }

    // Now inspect target volume/subfolder for any unlinked offloaded directories
    let dest_base = match &subfolder {
        Some(sub) if !sub.trim().is_empty() => std::path::PathBuf::from(&filter).join(sub.trim()),
        _ => std::path::PathBuf::from(&filter),
    };

    if dest_base.exists() && dest_base.is_dir() {
        let mut unlinked_dirs: Vec<std::path::PathBuf> = Vec::new();

        fn check_dest_dir(
            dir: &Path,
            mapped: &std::collections::HashSet<std::path::PathBuf>,
            unlinked: &mut Vec<std::path::PathBuf>,
        ) {
            let canonical = std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
            if mapped.contains(&canonical) {
                return;
            }

            if let Ok(entries) = std::fs::read_dir(dir) {
                let subdirs: Vec<std::path::PathBuf> = entries
                    .flatten()
                    .map(|e| e.path())
                    .filter(|p| {
                        p.is_dir()
                            && !p
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .starts_with('.')
                    })
                    .collect();

                if subdirs.is_empty() {
                    unlinked.push(dir.to_path_buf());
                } else {
                    for sub in subdirs {
                        check_dest_dir(&sub, mapped, unlinked);
                    }
                }
            }
        }

        if let Ok(entries) = std::fs::read_dir(&dest_base) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir()
                    && !path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .starts_with('.')
                {
                    check_dest_dir(&path, &mapped_targets, &mut unlinked_dirs);
                }
            }
        }

        for unlinked in unlinked_dirs {
            let guessed = guess_local_path(&unlinked, &dest_base, &home);
            let name = unlinked
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "Unknown".into());

            let dest_str = unlinked.to_string_lossy().to_string();
            let size = if let Ok(du) = Command::new("du")
                .args(["-sh", &dest_str])
                .output()
            {
                let s = String::from_utf8_lossy(&du.stdout);
                s.split_whitespace().next().unwrap_or("~").to_string()
            } else {
                "~".to_string()
            };

            results.push(SymlinkItem {
                name,
                path: guessed,
                destination: dest_str,
                size,
                is_missing_symlink: true,
            });
        }
    }

    Ok(results)
}

// 7. Relink a missing symlink to destination
#[tauri::command]
fn relink_symlink(local_path: String, destination: String) -> Result<(), String> {
    let dest_p = Path::new(&destination);
    if !dest_p.exists() {
        return Err(format!("Target directory does not exist on disk: {destination}"));
    }

    let local_p = Path::new(&local_path);
    if local_p.is_symlink() || local_p.exists() {
        return Err(format!("A file or symlink already exists at: {local_path}"));
    }

    if let Some(parent) = local_p.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create parent folder: {e}"))?;
    }

    #[cfg(unix)]
    std::os::unix::fs::symlink(dest_p, local_p)
        .map_err(|e| format!("Failed to create symbolic link: {e}"))?;

    // Refresh Dock bookmark if needed (e.g. ~/Downloads)
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("python3")
            .arg("-c")
            .arg(r#"
import plistlib, os, subprocess, sys

target = sys.argv[1]
try:
    cmd = f'''swift -e '
import Foundation
let path = NSString(string: "{target}").expandingTildeInPath
let url = URL(fileURLWithPath: path)
let b = try! url.bookmarkData(options: [], includingResourceValuesForKeys: nil, relativeTo: nil)
FileHandle.standardOutput.write(b)
' '''
    proc = subprocess.run(cmd, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=5)
    if proc.returncode == 0 and len(proc.stdout) > 50:
        dock_plist = os.path.expanduser('~/Library/Preferences/com.apple.dock.plist')
        with open(dock_plist, 'rb') as fp:
            pl = plistlib.load(fp)
        updated = False
        target_norm = os.path.normpath(os.path.expanduser(target))
        for item in pl.get('persistent-others', []):
            td = item.get('tile-data', {})
            raw_url = td.get('file-data', {}).get('_CFURLString', '')
            tile_path = raw_url.replace('file://', '')
            if os.path.normpath(tile_path) == target_norm or td.get('file-label') == os.path.basename(target_norm):
                td['book'] = proc.stdout
                updated = True
        if updated:
            with open(dock_plist, 'wb') as fp:
                plistlib.dump(pl, fp)
            subprocess.run(['killall', 'Dock'], timeout=3)
except Exception:
    pass
"#)
            .arg(&local_path)
            .output();
    }

    Ok(())
}

fn get_or_install_offload_script() -> Result<String, String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/Users/indunilperamuna".into());
    let bin_dir = format!("{home}/.local/bin");
    let script_path = format!("{bin_dir}/offload");

    // Automatically ensure script exists and is up to date
    let embedded = include_str!("../scripts/offload.sh");
    let should_write = match std::fs::read_to_string(&script_path) {
        Ok(existing) => existing != embedded,
        Err(_) => true,
    };

    if should_write {
        let _ = std::fs::create_dir_all(&bin_dir);
        let _ = std::fs::write(&script_path, embedded);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&script_path, std::fs::Permissions::from_mode(0o755));
        }
    }

    Ok(script_path)
}

// 7. Execute Offload command
#[tauri::command]
fn offload_folder(
    path: String, 
    except_list: String,
    dest_base: Option<String>,
) -> Result<String, String> {
    let script_path = get_or_install_offload_script()?;

    let mut cmd = Command::new(&script_path);
    cmd.arg(&path);

    if let Some(dest) = dest_base {
        if !dest.trim().is_empty() {
            cmd.arg(format!("--dest-base={}", dest.trim()));
        }
    }

    if !except_list.trim().is_empty() {
        cmd.arg(format!("--except={}", except_list.trim()));
    }

    let mut child = cmd
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn offload utility: {e}"))?;

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        let _ = stdin.write_all(b"y\n");
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Error waiting on process: {e}"))?;

    if output.status.success() {
        // Industry Standard: Trigger APFS snapshot block thinning so reclaimed internal space
        // is immediately reflected in macOS Finder and system stats rather than lingering as purgeable.
        #[cfg(target_os = "macos")]
        {
            let _ = Command::new("tmutil")
                .args(["thinlocalsnapshots", "/", "9999999999", "4"])
                .output();
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

// 8. Execute Reverse command
#[tauri::command]
fn reverse_folder(path: String) -> Result<String, String> {
    let script_path = get_or_install_offload_script()?;

    let mut child = Command::new(&script_path)
        .args([&path, "--reverse"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn reverse command: {e}"))?;

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        let _ = stdin.write_all(b"y\n");
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Error waiting on process: {e}"))?;

    if output.status.success() {
        // Industry Standard: Trigger APFS snapshot block thinning so reclaimed internal space
        // is immediately reflected in macOS Finder and system stats rather than lingering as purgeable.
        #[cfg(target_os = "macos")]
        {
            let _ = Command::new("tmutil")
                .args(["thinlocalsnapshots", "/", "9999999999", "4"])
                .output();
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

// 9. Reveal in Finder
#[tauri::command]
fn open_in_finder(path: String) -> Result<(), String> {
    Command::new("open")
        .args(["-R", &path])
        .output()
        .map_err(|e| e.to_string())?;
    Ok(())
}

// 10. Quit App
#[tauri::command]
fn quit_app(app: tauri::AppHandle) {
    app.exit(0);
}

// 11. Window Pin & Dialog Controls
#[tauri::command]
fn set_pinned(state: tauri::State<'_, Arc<WindowControlState>>, pinned: bool) {
    state.is_pinned.store(pinned, Ordering::SeqCst);
}

#[tauri::command]
fn get_pinned(state: tauri::State<'_, Arc<WindowControlState>>) -> bool {
    state.is_pinned.load(Ordering::SeqCst)
}

#[tauri::command]
fn hide_window(window: tauri::WebviewWindow) {
    let _ = window.hide();
}

#[tauri::command]
async fn pick_folder(
    app: tauri::AppHandle,
    state: tauri::State<'_, Arc<WindowControlState>>,
    title: Option<String>,
    default_path: Option<String>,
) -> Result<Option<String>, String> {
    state.is_dialog_open.store(true, Ordering::SeqCst);

    let app_clone = app.clone();
    let res = tauri::async_runtime::spawn_blocking(move || {
        let mut builder = app_clone.dialog().file();
        if let Some(t) = title {
            builder = builder.set_title(t);
        }
        if let Some(dp) = default_path {
            builder = builder.set_directory(std::path::PathBuf::from(dp));
        }
        builder.blocking_pick_folder()
    })
    .await
    .map_err(|e| e.to_string())?;

    state.is_dialog_open.store(false, Ordering::SeqCst);

    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }

    let selected_path = match res {
        Some(FilePath::Path(p)) => Some(p.to_string_lossy().to_string()),
        Some(FilePath::Url(u)) => u.to_file_path().ok().map(|p| p.to_string_lossy().to_string()),
        None => None,
    };

    Ok(selected_path)
}

pub fn run() {
    let window_ctrl = Arc::new(WindowControlState {
        is_dialog_open: AtomicBool::new(false),
        is_pinned: AtomicBool::new(true), // Default to pinned so popover stays open while user interacts with Finder!
        is_dragging: AtomicBool::new(false),
    });

    tauri::Builder::default()
        .manage(window_ctrl.clone())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup({
            let window_ctrl = window_ctrl.clone();
            move |app| {
                // Set macOS accessory policy (Hides from Dock, pure Menu Bar app)
                #[cfg(target_os = "macos")]
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);

                // Context Menu for right click on Tray icon
                let show_item = MenuItem::with_id(app, "show", "Open Offload Popover", true, None::<&str>)?;
                let quit_item = MenuItem::with_id(app, "quit", "Quit Offload", true, None::<&str>)?;
                let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

                // Build Tray Icon
                let _tray = TrayIconBuilder::new()
                    .icon(app.default_window_icon().unwrap().clone())
                    .menu(&menu)
                    .show_menu_on_left_click(false)
                    .tooltip("Offload - macOS Space Manager")
                    .on_menu_event(|app, event| {
                        match event.id.as_ref() {
                            "quit" => {
                                app.exit(0);
                            }
                            "show" => {
                                if let Some(window) = app.get_webview_window("main") {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
                            _ => {}
                        }
                    })
                    .on_tray_icon_event(|tray, event| {
                        if let TrayIconEvent::Click {
                            button: MouseButton::Left,
                            button_state: MouseButtonState::Up,
                            rect,
                            ..
                        } = event {
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                if window.is_visible().unwrap_or(false) {
                                    let _ = window.hide();
                                } else {
                                    let (pos_x, pos_y, width, height) = match (rect.position, rect.size) {
                                        (tauri::Position::Physical(p), tauri::Size::Physical(s)) => (p.x, p.y, s.width as i32, s.height as i32),
                                        (tauri::Position::Logical(l), tauri::Size::Logical(s)) => (l.x as i32, l.y as i32, s.width as i32, s.height as i32),
                                        _ => (900, 25, 28, 22),
                                    };
                                    let popover_width = 380;
                                    let x = (pos_x + (width / 2)) - (popover_width / 2);
                                    let y = pos_y + height + 2;
                                    let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(x, y)));
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
                        }
                    })
                    .build(app)?;

                // Window Event Handling: Hide on blur (if unpinned, not in dialog, and not dragging) or close
                if let Some(window) = app.get_webview_window("main") {
                    let w = window.clone();
                    let ctrl = window_ctrl.clone();
                    window.on_window_event(move |event| {
                        match event {
                            tauri::WindowEvent::CloseRequested { api, .. } => {
                                api.prevent_close();
                                let _ = w.hide();
                            }
                            tauri::WindowEvent::DragDrop(tauri::DragDropEvent::Enter { .. })
                            | tauri::WindowEvent::DragDrop(tauri::DragDropEvent::Over { .. }) => {
                                ctrl.is_dragging.store(true, Ordering::SeqCst);
                            }
                            tauri::WindowEvent::DragDrop(tauri::DragDropEvent::Drop { .. })
                            | tauri::WindowEvent::DragDrop(tauri::DragDropEvent::Leave) => {
                                ctrl.is_dragging.store(false, Ordering::SeqCst);
                            }
                            tauri::WindowEvent::Focused(false) => {
                                if !ctrl.is_dialog_open.load(Ordering::SeqCst)
                                    && !ctrl.is_pinned.load(Ordering::SeqCst)
                                    && !ctrl.is_dragging.load(Ordering::SeqCst)
                                {
                                    let _ = w.hide();
                                }
                            }
                            _ => {}
                        }
                    });
                }

                Ok(())
            }
        })
        .invoke_handler(tauri::generate_handler![
            check_permissions,
            open_privacy_settings,
            get_disk_stats,
            list_mounted_volumes,
            scan_folder,
            list_offloaded_items,
            relink_symlink,
            offload_folder,
            reverse_folder,
            open_in_finder,
            quit_app,
            set_pinned,
            get_pinned,
            hide_window,
            pick_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
