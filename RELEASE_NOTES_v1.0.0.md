# Offload v1.0.0 — Initial Release 🚀

Reclaim gigabytes of space on your internal Mac SSD by offloading heavy directories, development environments, virtual machine disks, and AI models to external APFS drives — with zero path breakage.

---

### ✨ Highlights & Features

- **🍏 Native macOS Menu Bar Popover**:
  - Operates as a pure macOS accessory (`LSUIElement = true`) with zero Dock presence.
  - Native chamfered caret notch pointing directly to the tray icon with a crisp 14px macOS panel radius (`rounded-xl`).
  - Auto-dismisses smoothly on blur (clicking outside the popover).

- **🎨 Bespoke Light & Dark Appearance System**:
  - **100% Solid Opaque Canvas**: No washed-out transparency or desktop wallpaper bleed-through.
  - **Light Theme**: Solid paper white (`#ffffff`), zinc-50 inset wells, dark charcoal typography (`#18181b`), and subtle hairline borders matching macOS native popovers.
  - **Dark Theme**: Solid obsidian titanium (`#18181b`), zinc-900 wells, and high-contrast clean typography without neon gradient slop.
  - **1-Click Theme Switcher**: Easily toggle between **System (Auto)** 💻, **Light** ☀️, and **Dark** 🌙.

- **📊 Accurate Real Storage Gauges**:
  - Targets `/System/Volumes/Data` directly on macOS APFS, giving exact, unsealed user volume metrics (~397 GiB used / 34 GiB free) instead of misleading root snapshot stats.

- **📁 Configurable Destination Drive & Subfolder**:
  - Select any detected external volume (`/Volumes/DataDisc2`, `/Volumes/DataDisc`, or browse a custom folder).
  - Customize the **Folder Inside Selected Volume** in Settings (default: `Indunilperamuna-Macos` or any custom folder like `DeveloperBackups`).
  - Live full resolved path preview (`/Volumes/DataDisc2/Indunilperamuna-Macos/`) with direct Finder reveal.

- **⚡ Full Directory Offload with Optional Stashing (`--except`)**:
  - Offloads the entire folder cleanly to the external drive by default, reclaiming 100% of its disk space.
  - Provides optional manual exclusion (`--except`) for keeping specific files or folders (e.g. `.env`, caches) on the internal SSD if desired.
  - Seamlessly resolves all paths through transparent macOS APFS symbolic links.

- **🛡️ Smart Zero-Noise Permissions Handling**:
  - Tests Full Disk Access (FDA) and Removable Volume permissions.
  - **Zero Noise**: If both permissions are granted, the Permission Setup screen and header icon are completely hidden.
  - If access is required, a pulsating amber alert guides you directly to `System Settings → Privacy & Security → Full Disk Access`.

- **🛡️ APFS Sparse & Atomic Swap Safe**:
  - Automatically preserves APFS sparse-file hole punching (`rsync -aP -S`) critical for Docker and VM images.
  - Uses atomic inode renames to permanently prevent macOS Finder `.DS_Store` `Directory not empty` removal errors.

---

### 🍺 Installation via Homebrew (Multi-Arch)

Homebrew automatically detects your Mac's CPU architecture and installs the matching native build:

```bash
# Add tap and install
brew install --cask iperamuna/tap/offload
```

---

### 📦 Direct Downloads

| Asset | Architecture | Target Hardware |
| :--- | :--- | :--- |
| **`Offload_1.0.0_aarch64.dmg`** | ARM64 | Apple Silicon Macs (M1, M2, M3, M4, Pro, Max, Ultra) |
| **`Offload_1.0.0_x64.dmg`** | x86_64 | Intel Macs (Core i5, i7, i9, Xeon) |

*Corresponding `.md5` and `.sha256` checksum files are attached below for cryptographic verification.*

---

### ℹ️ First Launch on macOS (Gatekeeper Note)

Because Offload is distributed as an open-source tool outside the Mac App Store without an active Apple Developer Notarization ticket, macOS Gatekeeper may show a verification prompt on first download.

To open:
1. **Right-click** (or Control-click) `Offload.app` in `/Applications` or inside the DMG.
2. Select **Open** from the menu.
3. Click **Open** on the confirmation dialog.

*(Alternatively, run `xattr -cr /Applications/Offload.app` in Terminal, or install via Homebrew with `--no-quarantine`)*.
