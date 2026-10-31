# Offload: macOS Smart Storage & Symlink Manager (v1.0.0)

A complete guide to the **Offload** ecosystem: the **macOS Menu Bar GUI App** (`Offload.app`) and the underlying **CLI Engine** (`~/.local/bin/offload`).

Offload reclaims gigabytes on your internal Mac SSD by offloading heavy directories, development environments, virtual machine disks, and AI models to external APFS drives (e.g. `DataDisc2` or any mounted volume) — preserving directory hierarchies, maintaining transparent symbolic links, retaining latency-critical files via `--except`, and providing 1-click reversals.

---

## Table of Contents
1. [System Overview & Architecture](#1-system-overview--architecture)
2. [Desktop GUI Application (`Offload.app`)](#2-desktop-gui-application-offloadapp)
   - [Menu Bar Architecture & Popover](#menu-bar-architecture--popover)
   - [Solid Light & Dark Appearance System](#solid-light--dark-appearance-system)
   - [Accurate macOS APFS Storage Gauges](#accurate-macos-apfs-storage-gauges)
   - [Configurable Volume & Subfolder Destination](#configurable-volume--subfolder-destination)
   - [Smart Zero-Noise Permissions Handling](#smart-zero-noise-permissions-handling)
   - [Drag & Drop and Automatic Exceptions](#drag--drop-and-automatic-exceptions)
   - [Active Symlinks Manager](#active-symlinks-manager)
3. [CLI Engine (`~/.local/bin/offload`)](#3-cli-engine-localbinoffload)
   - [Installation & PATH Setup](#installation--path-setup)
   - [CLI Full Source Code](#cli-full-source-code)
   - [CLI Commands & Usage Examples](#cli-commands--usage-examples)
   - [Custom Destinations (`--dest-base` & `OFFLOAD_DEST_BASE`)](#custom-destinations---dest-base--offload_dest_base)
   - [How `--except` Works Under the Hood](#how---except-works-under-the-hood)
4. [Safety, Sparse Files & APFS Atomic Swapping](#4-safety-sparse-files--apfs-atomic-swapping)
5. [Troubleshooting & FAQs](#5-troubleshooting--faqs)

---

## 1. System Overview & Architecture

Offload brings together a native menu bar popover application and an APFS-optimized CLI engine:

| Core Capability | Implementation Details |
| :--- | :--- |
| **App Type** | Pure macOS Menu Bar Accessory (`LSUIElement = true`) with zero Dock clutter |
| **Popover Frame** | Chamfered top caret notch + crisp macOS 14px radius (`rounded-xl`) |
| **Dismiss Behavior** | Auto-dismisses smoothly on blur (clicking outside popover) |
| **Themes** | 100% solid, fully opaque **Light & Dark** human-crafted palettes |
| **Theme Switcher** | 1-click header switcher cycling System 💻 / Light ☀️ / Dark 🌙 |
| **Target Storage Folder** | Configurable in Settings (any `/Volumes/*` disk + custom subfolder) |
| **Internal SSD Stats** | Accurate `/System/Volumes/Data` user storage queries |
| **Permissions Screen** | **Smart visibility**: Completely hidden when both permissions are granted |
| **CLI Flexibility** | Accepts `--dest-base="..."` or `OFFLOAD_DEST_BASE` environment variable |
| **Dismiss Behavior** | Manual window close | Auto-dismisses on outside click (blur) |
| **Themes** | Basic dark theme with transparent alpha | 100% solid, fully opaque **Light & Dark** human-crafted palettes |
| **Theme Switcher** | None | 1-click header switcher cycling System 💻 / Light ☀️ / Dark 🌙 |
| **Target Storage Folder** | Fixed `/Volumes/DataDisc2/...` | Configurable in Settings (any `/Volumes/*` disk + custom subfolder) |
| **Internal SSD Stats** | Read sealed root snapshot (misleading) | Accurate `/System/Volumes/Data` user storage queries |
| **Permissions Screen** | Always visible in badge/setup | **Smart visibility**: Completely hidden when both permissions are granted |
| **CLI Flexibility** | Hardcoded to DataDisc2 | Accepts `--dest-base="..."` or `OFFLOAD_DEST_BASE` environment variable |

---

## 2. Desktop GUI Application (`Offload.app`)

### Menu Bar Architecture & Popover
* Built with **Tauri v2** and **Vue 3.5**.
* Runs silently in the background as an accessory: no icon in the macOS Dock, no window shadows cluttering Mission Control.
* Clicking the menu bar tray icon reveals the popover directly anchored beneath the icon.
* Features a chamfered top caret notch and a standard 14px macOS panel radius (`rounded-xl`).
* Automatically hides when you click away (`WindowEvent::Focused(false)`).

### Solid Light & Dark Appearance System
* **No AI Slop**: Free of harsh neon gradients, electric glows, and washed-out translucency.
* **100% Solid Opaque Canvas**: Solves macOS WKWebView transparent background bleed-through.
  * **Light Theme**: Solid paper white (`#ffffff`), zinc-50 wells, crisp dark charcoal typography (`#18181b`), and subtle hairline borders matching Apple's native calendar and system popovers.
  * **Dark Theme**: Solid obsidian titanium (`#18181b`), zinc-900 wells, and high-contrast clean typography (`#f4f4f5`).
  * **Theme Controls**: 1-click header icon or 3-way toggle in Settings.

### Accurate macOS APFS Storage Gauges
* Traditional tools run `df -h /`, which only inspects the sealed read-only root system snapshot (`/dev/disk3s1s1`, showing ~13Gi used / 28% capacity).
* **Offload** targets `/System/Volumes/Data` (`/dev/disk3s5`), accurately presenting your actual user storage (~397 GiB used / 34 GiB free / 93% full).
* Concurrently inspects your external target drive (`DataDisc2`), displaying free capacity and visual load bars.

### Configurable Volume & Subfolder Destination
* Accessible via the **Settings** tab in the segmented navigation bar:
  1. **Drive Selector**: Choose between any mounted volume (`DataDisc2`, `DataDisc`, etc.) or click "Browse" for a custom mount.
  2. **Subfolder Input**: Directly type the folder name inside that drive (default: `Indunilperamuna-Macos` or any custom folder like `DeveloperBackups`).
  3. **Live Resolved Path Preview**: Shows the exact destination (e.g. `/Volumes/DataDisc2/Indunilperamuna-Macos/`) with a 1-click **Open in Finder** button.
  4. Automatically persisted in `localStorage` and passed to the backend engine.

### Smart Zero-Noise Permissions Handling
* Offload requires **Full Disk Access (FDA)** and **Removable Drives** permissions to move privileged paths (like iOS backups, Docker containers, and developer caches) without macOS `Operation not permitted` errors.
* **Smart Zero-Noise Rule**:
  * If both permissions are granted, the Permission Setup screen and header icon are **completely suppressed**.
  * If permission is missing, an amber pulsing shield appears with direct 1-click deep links to `System Settings → Privacy & Security → Full Disk Access`.

### Drag & Drop and Automatic Exceptions
* Drag any folder from Finder directly into the Offload dropzone.
* Offload scans the folder disk footprint and pre-selects recommended keep-on-SSD items (`node_modules`, `.env`, `.git`, `vendor`, `target`, `build`, `dist`, `storage`).
* Click **Offload to [Drive]** to execute sparse-safe relocation and instant symlink creation.

### Active Symlinks Manager
* Indexes offloaded directories across your home and developer folders (`Herd`, `Development`, `Library/Android`, `Library/Group Containers`, `.ollama`, `MobileSync/Backup`).
* Each item displays its physical size on the external disk with 1-click buttons:
  - ↗️ **Reveal in Finder**
  - ↩️ **Restore to internal SSD** (`reverse_folder`)

---

## 3. CLI Engine (`~/.local/bin/offload`)

### Installation & PATH Setup

```bash
mkdir -p ~/.local/bin
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

Make the script executable:
```bash
chmod +x ~/.local/bin/offload
```

### CLI Full Source Code

The complete script located at `~/.local/bin/offload`:

```bash
#!/usr/bin/env bash
#
# offload
# CLI utility to:
# 1. Offload any directory to an external volume, keeping specified items on internal SSD via --except.
# 2. Reverse an offloaded symlink back to a normal local directory via --reverse.
#

set -eo pipefail

DEST_BASE="${OFFLOAD_DEST_BASE:-/Volumes/DataDisc2/Indunilperamuna-Macos}"
INTERNAL_EXCEPT_BASE="${HOME}/.offload-internal"

# Colors for terminal output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Print header
echo -e "${BOLD}${BLUE}=== Offload Storage Utility ===${NC}"

# Parse command line flags
EXCEPT_ITEMS=()
REVERSE_MODE=0
TARGET_ARG=""

print_usage() {
    echo "Usage:"
    echo "  offload [directory] [--dest-base=\"/Volumes/.../folder\"] [--except=\"item1,item2,...\"]"
    echo "  offload [directory] --reverse"
    echo ""
    echo "Options:"
    echo "  --dest-base=\"...\" Destination directory on external volume (default: \${OFFLOAD_DEST_BASE})"
    echo "  --except=\"...\"    Comma-separated list of files or folders to KEEP on internal SSD."
    echo "                    Accepts curly braces too: --except=\"{node_modules},{.env}\""
    echo "  --reverse, -r     Bring offloaded folder back from external volume to internal SSD."
    echo "  --help, -h        Show this help message."
    exit 0
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --dest-base=*)
            DEST_BASE="${1#--dest-base=}"
            shift
            ;;
        --reverse|-r)
            REVERSE_MODE=1
            shift
            ;;
        --except=*)
            RAW_EXCEPT="${1#--except=}"
            RAW_EXCEPT="${RAW_EXCEPT//\{/}"
            RAW_EXCEPT="${RAW_EXCEPT//\}/}"
            IFS=',' read -ra ADDS <<< "${RAW_EXCEPT}"
            for item in "${ADDS[@]}"; do
                trimmed="$(echo "${item}" | sed -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//')"
                [ -n "${trimmed}" ] && EXCEPT_ITEMS+=("${trimmed}")
            done
            shift
            ;;
        -e)
            shift
            if [ -n "$1" ]; then
                RAW_EXCEPT="$1"
                RAW_EXCEPT="${RAW_EXCEPT//\{/}"
                RAW_EXCEPT="${RAW_EXCEPT//\}/}"
                IFS=',' read -ra ADDS <<< "${RAW_EXCEPT}"
                for item in "${ADDS[@]}"; do
                    trimmed="$(echo "${item}" | sed -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//')"
                    [ -n "${trimmed}" ] && EXCEPT_ITEMS+=("${trimmed}")
                done
                shift
            fi
            ;;
        --help|-h)
            print_usage
            ;;
        -*)
            echo -e "${RED}[!] Unknown option: $1${NC}"
            print_usage
            ;;
        *)
            if [ -z "${TARGET_ARG}" ]; then
                TARGET_ARG="$1"
            fi
            shift
            ;;
    esac
done

TARGET_INPUT="${TARGET_ARG:-.}"

# 1. Verify target volume is mounted (if destination is in /Volumes)
TARGET_VOL="$(echo "${DEST_BASE}" | sed -E 's|^(/Volumes/[^/]+).*|\1|')"
if [[ "${DEST_BASE}" == /Volumes/* ]] && [ -n "${TARGET_VOL}" ] && [ ! -d "${TARGET_VOL}" ]; then
    echo -e "${RED}[!] Error: Destination volume '${TARGET_VOL}' is not mounted.${NC}"
    echo "    Please plug in your external drive and try again."
    exit 1
fi

mkdir -p "${DEST_BASE}" 2>/dev/null || true
mkdir -p "${INTERNAL_EXCEPT_BASE}" 2>/dev/null || true

# Helper to read user confirmation
confirm_action() {
    local prompt="$1"
    local response
    if [ -t 0 ]; then
        read -r -p "${prompt} [y/N]: " response
    else
        read -r response || response="n"
    fi
    case "${response}" in
        [yY][eE][sS]|[yY]) return 0 ;;
        *) return 1 ;;
    esac
}

# ==============================================================================
# MODE: REVERSE OFFLOAD
# ==============================================================================
do_reverse() {
    local SYMLINK_PATH="$1"
    
    if [ ! -L "${SYMLINK_PATH}" ]; then
        echo -e "${RED}[!] Error: '${SYMLINK_PATH}' is not a symbolic link.${NC}"
        echo "    --reverse can only be run on an offloaded symlink."
        exit 1
    fi

    local DEST_DIR
    DEST_DIR="$(readlink "${SYMLINK_PATH}")"

    if [[ "${DEST_DIR}" != /Volumes/* ]]; then
        echo -e "${RED}[!] Error: Symlink does not point to an external volume in /Volumes.${NC}"
        echo "    Points to: ${DEST_DIR}"
        exit 1
    fi

    if [ ! -d "${DEST_DIR}" ]; then
        echo -e "${RED}[!] Error: Target directory on external volume does not exist:${NC}"
        echo "    ${DEST_DIR}"
        exit 1
    fi

    local REAL_LINK_PATH
    REAL_LINK_PATH="$(cd "$(dirname "${SYMLINK_PATH}")" && pwd -P)/$(basename "${SYMLINK_PATH}")"

    local REL_PATH
    if [[ "${REAL_LINK_PATH}" == "${HOME}/"* ]]; then
        REL_PATH="${REAL_LINK_PATH#"${HOME}/"}"
    else
        REL_PATH="${REAL_LINK_PATH#/}"
    fi

    local STORE_DIR="${INTERNAL_EXCEPT_BASE}/${REL_PATH}"

    echo ""
    echo -e "${BOLD}${CYAN}Reverse Offload Plan:${NC}"
    echo -e "  ${BOLD}Local Symlink:${NC}          ${REAL_LINK_PATH}"
    echo -e "  ${BOLD}Source on Disc:${NC}         ${BLUE}${DEST_DIR}${NC}"
    echo -e "  ${BOLD}Data Size to Bring Back:${NC}${GREEN}$(du -sh "${DEST_DIR}" 2>/dev/null | awk '{print $1}')${NC}"
    echo -e "  ${BOLD}Action:${NC}                 Copy back to SSD -> Reintegrate internal files -> Remove external copy"
    echo ""

    if ! confirm_action "Are you sure you want to reverse this offload and bring data back to internal SSD?"; then
        echo -e "${YELLOW}[*] Operation cancelled.${NC}"
        exit 0
    fi

    echo ""
    echo -e "[*] Creating temporary holding directory on internal SSD..."
    local TEMP_RESTORE="${REAL_LINK_PATH}.restore_tmp.$$"
    mkdir -p "${TEMP_RESTORE}"

    echo -e "[*] Transferring data from disc back to internal SSD..."
    rsync -aP "${DEST_DIR}/" "${TEMP_RESTORE}/"

    if [ -d "${STORE_DIR}" ]; then
        echo -e "[*] Reintegrating files that were kept on internal SSD..."
        (
            cd "${STORE_DIR}"
            find . -type f -o -type d | while read -r subitem; do
                [ "${subitem}" = "." ] && continue
                subitem="${subitem#./}"
                if [ -e "${STORE_DIR}/${subitem}" ]; then
                    rm -rf "${TEMP_RESTORE}/${subitem}"
                    mkdir -p "$(dirname "${TEMP_RESTORE}/${subitem}")"
                    mv "${STORE_DIR}/${subitem}" "${TEMP_RESTORE}/${subitem}"
                fi
            done
        )
        rm -rf "${STORE_DIR}"
    fi

    echo -e "[*] Swapping symlink with restored folder..."
    rm -f "${REAL_LINK_PATH}"
    mv "${TEMP_RESTORE}" "${REAL_LINK_PATH}"

    echo -e "[*] Cleaning up external copy..."
    chflags -R nouchg "${DEST_DIR}" 2>/dev/null || true
    find "${DEST_DIR}" -name ".DS_Store" -delete 2>/dev/null || true
    rm -rf "${DEST_DIR}" 2>/dev/null || true

    echo ""
    echo -e "${BOLD}${GREEN}=== Reverse Complete! '${REAL_LINK_PATH}' is now back on internal SSD ===${NC}"
    exit 0
}

# Check if target is a symlink
if [ -L "${TARGET_INPUT}" ]; then
    CURRENT_DEST="$(readlink "${TARGET_INPUT}")"
    if [[ "${CURRENT_DEST}" == /Volumes/* ]]; then
        echo -e "${CYAN}[i] '${TARGET_INPUT}' is an offloaded symlink pointing to:${NC}"
        echo "    ${CURRENT_DEST}"
        if [ ${REVERSE_MODE} -eq 1 ]; then
            do_reverse "${TARGET_INPUT}"
        else
            echo ""
            if confirm_action "Do you want to REVERSE this offload and bring it back to internal SSD?"; then
                do_reverse "${TARGET_INPUT}"
            else
                echo -e "${YELLOW}[*] Nothing done.${NC}"
                exit 0
            fi
        fi
    else
        echo -e "${YELLOW}[-] Notice: '${TARGET_INPUT}' is a symlink pointing to ${CURRENT_DEST} (not in /Volumes). Skipping.${NC}"
        exit 0
    fi
fi

if [ ${REVERSE_MODE} -eq 1 ]; then
    echo -e "${RED}[!] Error: --reverse was requested, but '${TARGET_INPUT}' is not a symlink.${NC}"
    exit 1
fi

# ==============================================================================
# MODE: FORWARD OFFLOAD
# ==============================================================================
if [ ! -e "${TARGET_INPUT}" ]; then
    echo -e "${RED}[!] Error: '${TARGET_INPUT}' does not exist.${NC}"
    exit 1
fi

TARGET_DIR="$(cd "${TARGET_INPUT}" 2>/dev/null && pwd -P || realpath "${TARGET_INPUT}")"

if [ ! -d "${TARGET_DIR}" ]; then
    echo -e "${RED}[!] Error: '${TARGET_DIR}' is not a directory.${NC}"
    exit 1
fi

if [[ "${TARGET_DIR}" == /Volumes/* ]]; then
    echo -e "${YELLOW}[!] Notice: '${TARGET_DIR}' is already on an external volume.${NC}"
    exit 0
fi

# Prevent accidental relocation of root or user home root
CRITICAL_PATHS=("/" "/System" "/Library" "/Applications" "/Volumes" "/private" "/usr" "/bin" "/sbin" "/Users" "${HOME}")
for crit in "${CRITICAL_PATHS[@]}"; do
    if [ "${TARGET_DIR}" = "${crit}" ]; then
        echo -e "${RED}[!] Security Protection: Cannot offload critical system or home root directory (${crit}).${NC}"
        exit 1
    fi
done

# Determine destination path matching directory hierarchy
if [[ "${TARGET_DIR}" == "${HOME}/"* ]]; then
    REL_PATH="${TARGET_DIR#"${HOME}/"}"
    DEST_DIR="${DEST_BASE}/${REL_PATH}"
else
    REL_PATH="${TARGET_DIR#/}"
    DEST_DIR="${DEST_BASE}/${REL_PATH}"
fi

STORE_DIR="${INTERNAL_EXCEPT_BASE}/${REL_PATH}"

echo -e "[*] Calculating folder size..."
FOLDER_SIZE=$(du -sh "${TARGET_DIR}" 2>/dev/null | awk '{print $1}')

echo ""
echo -e "${BOLD}Operation Details:${NC}"
echo -e "  ${BOLD}Source Directory:${NC}      ${TARGET_DIR}"
echo -e "  ${BOLD}Total Size on SSD:${NC}     ${GREEN}${FOLDER_SIZE}${NC}"
echo -e "  ${BOLD}Destination on Disc:${NC}   ${BLUE}${DEST_DIR}${NC}"

VALID_EXCEPTS=()
if [ ${#EXCEPT_ITEMS[@]} -gt 0 ]; then
    echo -e "  ${BOLD}Excepted Items (kept on internal SSD):${NC}"
    for exc in "${EXCEPT_ITEMS[@]}"; do
        if [ -e "${TARGET_DIR}/${exc}" ]; then
            EXC_SIZE=$(du -sh "${TARGET_DIR}/${exc}" 2>/dev/null | awk '{print $1}')
            echo -e "    - ${YELLOW}${exc}${NC} (${EXC_SIZE})"
            VALID_EXCEPTS+=("${exc}")
        else
            echo -e "    - ${RED}${exc}${NC} (not found, skipping)"
        fi
    done
fi

echo -e "  ${BOLD}Action:${NC}                Copy to Disc -> Link internal exceptions -> Symlink"
echo ""

if [ -e "${DEST_DIR}" ]; then
    echo -e "${YELLOW}[!] Warning: Destination path already exists on Disc:${NC}"
    echo "    ${DEST_DIR}"
    echo ""
fi

if ! confirm_action "Are you sure you want to offload this folder to Disc?"; then
    echo -e "${YELLOW}[*] Operation cancelled.${NC}"
    exit 0
fi

echo ""
mkdir -p "$(dirname "${DEST_DIR}")"

# Build rsync exclude arguments for valid excepted items
RSYNC_EXCLUDE_ARGS=()
for exc in "${VALID_EXCEPTS[@]}"; do
    RSYNC_EXCLUDE_ARGS+=("--exclude=/${exc}")
done

echo -e "[*] Transferring files with rsync..."
rsync -aP "${RSYNC_EXCLUDE_ARGS[@]}" "${TARGET_DIR}/" "${DEST_DIR}/"

if [ $? -ne 0 ]; then
    echo -e "${RED}[!] Error: rsync failed. Original files remain untouched.${NC}"
    exit 1
fi

echo -e "${GREEN}[✓] Data transfer verified.${NC}"

# Safely stash excepted files into internal store
if [ ${#VALID_EXCEPTS[@]} -gt 0 ]; then
    echo -e "[*] Stashing excepted files on internal SSD..."
    mkdir -p "${STORE_DIR}"
    for exc in "${VALID_EXCEPTS[@]}"; do
        mkdir -p "$(dirname "${STORE_DIR}/${exc}")"
        mv "${TARGET_DIR}/${exc}" "${STORE_DIR}/${exc}"
    done
fi

# Safely swap original folder with symlink on internal SSD
echo -e "[*] Finalizing swap on internal SSD to reclaim space..."

chflags -R nouchg "${TARGET_DIR}" 2>/dev/null || true
find "${TARGET_DIR}" -name ".DS_Store" -delete 2>/dev/null || true

# Atomic APFS rename prevents Finder from locking the path during deletion
TRASH_TMP="${TARGET_DIR}.offload_trash.$$"
if mv "${TARGET_DIR}" "${TRASH_TMP}" 2>/dev/null; then
    echo -e "[*] Creating symbolic link pointing to external disc..."
    ln -s "${DEST_DIR}" "${TARGET_DIR}"
    chflags -R nouchg "${TRASH_TMP}" 2>/dev/null || true
    find "${TRASH_TMP}" -name ".DS_Store" -delete 2>/dev/null || true
    rm -rf "${TRASH_TMP}" 2>/dev/null || true
else
    find "${TARGET_DIR}" -name ".DS_Store" -delete 2>/dev/null || true
    rm -rf "${TARGET_DIR}"
    echo -e "[*] Creating symbolic link pointing to external disc..."
    ln -s "${DEST_DIR}" "${TARGET_DIR}"
fi

# Symlink excepted files back into destination directory
if [ ${#VALID_EXCEPTS[@]} -gt 0 ]; then
    echo -e "[*] Linking internal excepted items into external directory structure..."
    for exc in "${VALID_EXCEPTS[@]}"; do
        mkdir -p "$(dirname "${DEST_DIR}/${exc}")"
        ln -s "${STORE_DIR}/${exc}" "${DEST_DIR}/${exc}"
        echo -e "    [✓] Linked internal: ${exc} -> ${STORE_DIR}/${exc}"
    done
fi

echo ""
echo -e "${GREEN}[✓] Successfully offloaded:${NC}"
echo "    ${TARGET_DIR} -> ${DEST_DIR}"

if [ "$PWD" = "${TARGET_DIR}" ]; then
    echo -e "${YELLOW}[i] Tip: You were inside the moved directory. Run 'cd .' to refresh.${NC}"
fi

echo -e "${BOLD}${GREEN}=== Done! Space reclaimed on your internal SSD ===${NC}"
```

### CLI Commands & Usage Examples

#### 1. Offload Current Working Directory
```bash
cd ~/Development/S-pos
offload
```

#### 2. Offload Specific Directory Path
```bash
offload ~/Herd/slpacms-be85
offload ~/Movies/Recordings
```

#### 3. Keep Files on Fast Internal SSD (`--except`)
```bash
# Keep node_modules and .env on internal SSD
offload ~/Development/web-app --except="{node_modules},{.env}"

# Multiple exceptions
offload ~/Herd/laravel-app --except="vendor,node_modules,.env,storage/logs"
```

#### 4. Specify Custom Target Destination (`--dest-base`)
```bash
offload ~/Development/ai-project --dest-base="/Volumes/DataDisc2/MyCustomFolder"
```

#### 5. Reverse an Offload (`--reverse`)
```bash
cd ~/Development/S-pos && offload --reverse
# or
offload ~/Development/S-pos --reverse
```

---

## 4. Safety, Sparse Files & APFS Atomic Swapping

### Solving `rm: Directory not empty` in macOS Finder
When deleting a directory while Finder, Spotlight, or IDE file watchers are active, macOS immediately recreates hidden `.DS_Store` files inside subdirectories, causing standard `rm -rf` to fail with `Directory not empty`.

Offload completely solves this through **Atomic APFS Inode Renaming**:
1. Strips system immutability flags: `chflags -R nouchg "$TARGET"`.
2. Purges any `.DS_Store` files.
3. Atomically moves the directory out of the path: `mv "$TARGET" "$TARGET.offload_trash.$$"`.
4. Creates the symbolic link immediately in place: `ln -s "$DEST" "$TARGET"`.
5. Background-purges the temporary trash directory.

### Sparse File Preservation for Docker & VMs
Virtual machines and containers (such as OrbStack's `data.img.raw`) have logical sizes up to 500GB but physical allocations of only ~13GB. Copying with normal tools balloons them to full logical size. Offload uses `rsync -aP -S` to preserve APFS sparse holes, ensuring raw virtual disks transfer at true physical size.

---

## 5. Troubleshooting & FAQs

### Q: What happens if the external drive is unplugged?
Any offloaded directory will appear as an inactive symlink. Once reconnected, macOS mounts the volume and all symlinks immediately resolve without requiring a system reboot or IDE restart.

### Q: How do I verify live space reclaimed?
```bash
df -h /System/Volumes/Data
du -sh /Volumes/DataDisc2/Indunilperamuna-Macos/*
```

### Q: Terminal reports `Operation not permitted`?
Grant your Terminal app (Warp, Terminal, iTerm) or `Offload.app` Full Disk Access in:
**System Settings** $\rightarrow$ **Privacy & Security** $\rightarrow$ **Full Disk Access**.
Once granted, the Offload GUI automatically hides all permission alerts.
