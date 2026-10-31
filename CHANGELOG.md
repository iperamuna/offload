# Changelog

All notable changes to the **Offload** project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.0.0] - 2026-10-31

### Initial Release 🎉

#### Added
- **Native macOS Menu Bar Popover**:
  - Resides entirely in the top menu bar (`LSUIElement = true`) with zero Dock clutter.
  - Features a chamfered top caret notch and standard macOS 14px corner radius (`rounded-xl`).
  - Auto-dismisses on blur (clicking outside the popover).
- **Dual Live Storage Meters**:
  - Real-time gauge of internal Mac SSD and external target disk.
  - Queries `/System/Volumes/Data` directly on macOS APFS, giving exact, unsealed user volume metrics (~397 GiB used / 34 GiB free) instead of sealed root snapshot stats.
- **Configurable Target Volume & Subfolder**:
  - Dedicated **Settings** tab with a volume dropdown for all mounted disks (`/Volumes/*`) or custom browse.
  - Editable **Folder Inside Selected Volume** input (default: `Indunilperamuna-Macos` or any custom folder).
  - Live full resolved path preview (`/Volumes/DataDisc2/Indunilperamuna-Macos/`) with direct Finder reveal.
- **Bespoke Light & Dark Appearance System**:
  - 100% solid, fully opaque card canvas, eliminating transparent webview bleed-through on macOS.
  - Light mode matches native macOS calendar popover styling (solid paper white `#ffffff`, zinc-50 wells, `#18181b` dark charcoal typography).
  - Dark mode features clean obsidian titanium (`#18181b`) without neon gradient slop.
  - 1-click theme switcher cycling **System (Auto)** 💻, **Light** ☀️, and **Dark** 🌙.
- **Smart Zero-Noise Permissions Handling**:
  - Completely suppresses the Permission Setup screen and header icon whenever both permissions (Full Disk Access and Removable Drives) are granted.
  - Pulsating amber alert badge only appears when access is required, linking to `System Settings → Privacy & Security → Full Disk Access`.
- **Drag & Drop Folder Inspector**:
  - Instant calculation of folder disk usage.
  - Automatic detection and recommendation of exception candidates (`node_modules`, `.env`, `.git`, `vendor`, `storage/logs`).
- **Full Directory Offload with Optional Stashing (`--except`)**:
  - Reclaims 100% of folder space by offloading entire directory cleanly by default.
  - Provides optional manual exclusion for files/folders to remain on the internal SSD when desired.
  - Automated bidirectional symlink resolution.
- **Active Symlinks Manager**:
  - Automatically indexes existing offloaded directories.
  - One-click "Reveal in Finder" and "Reverse to SSD" actions.
- **Robust APFS Backend**:
  - `rsync -aP -S` with sparse-file hole punching preservation for Docker virtual disks and VM images.
  - Atomic APFS inode renames (`mv` + `chflags` + `.DS_Store` purge) to permanently prevent macOS Finder `Directory not empty` removal errors.
  - Standalone command-line utility at `~/.local/bin/offload` with `--dest-base="..."` and `OFFLOAD_DEST_BASE` support.
