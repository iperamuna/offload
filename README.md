# Offload 🚀

<div align="center">
  <img src="src/assets/logo.svg" width="128" height="128" alt="Offload Logo" />
  <p><strong>Smart macOS Drive Space Offloader & Dynamic Symlink Manager</strong></p>
  <p>Reclaim gigabytes on your internal Mac SSD by offloading heavy projects, models, and virtual disks to external APFS drives — with zero path breakage.</p>

  [![macOS](https://img.shields.io/badge/Platform-macOS%20(Apple%20Silicon%20%26%20Intel)-black?logo=apple)](https://apple.com)
  [![Tauri 2](https://img.shields.io/badge/Tauri-2.0-24C8D8?logo=tauri&logoColor=white)](https://tauri.app)
  [![Vue 3](https://img.shields.io/badge/Vue-3.5-4FC08D?logo=vuedotjs&logoColor=white)](https://vuejs.org)
  [![Tailwind CSS](https://img.shields.io/badge/TailwindCSS-3.4-38B2AC?logo=tailwindcss&logoColor=white)](https://tailwindcss.com)
  [![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
</div>

---

## 🌟 Why Offload?

Modern Macs with soldered 256GB or 512GB internal SSDs quickly run out of space due to:
- Large development repositories (`node_modules`, `vendor`, build caches)
- Docker / OrbStack raw disk images (`data.img.raw`)
- Android Studio SDKs, system images, and Virtual Devices (AVDs)
- Local AI model weights (Ollama, HuggingFace, PyTorch)
- iOS and iPad backup images (`MobileSync/Backup`)

**Offload** moves the physical weight of these directories to your external APFS drive (`DataDisc2` or any mounted volume), creates transparent symbolic links in place, and preserves latency-sensitive files (`node_modules`, `.env`) on your internal SSD using the **`--except`** hybrid engine.

---

## 📸 Key Features

- **🍏 Native macOS Menu Bar Popover**:
  - Resides entirely in the top menu bar (`LSUIElement = true`) with zero Dock presence.
  - Features a native chamfered caret notch pointing directly to the tray icon and crisp 14px macOS corner radius (`rounded-xl`).
  - Auto-dismisses smoothly when clicking away (blur).

- **🎨 Bespoke Human-Crafted Appearance (Light & Dark)**:
  - **100% Solid Opaque Canvas**: No washed-out transparency or desktop wallpaper bleed-through.
  - **Light Theme**: Solid paper white (`#ffffff`), zinc-50 inset wells, dark charcoal typography (`#18181b`), and subtle hairline borders matching macOS native popovers.
  - **Dark Theme**: Solid obsidian titanium (`#18181b`), zinc-900 wells, and high-contrast clean typography without garish neon AI-slop glows.
  - **Theme Switcher**: 1-click header button cycling **System (Auto)** 💻 → **Light** ☀️ → **Dark** 🌙, also available in Settings.

- **📊 Accurate Real Storage Gauges**:
  - Queries `/System/Volumes/Data` directly on macOS APFS, giving exact, unsealed user volume metrics (e.g. 397 GiB used / 34 GiB free / 93% full) instead of misleading root snapshot stats.

- **📁 Configurable Destination Drive & Subfolder**:
  - Select any detected external volume (`/Volumes/DataDisc2`, `/Volumes/DataDisc`, or browse a custom folder).
  - Customize the **Folder Inside Selected Volume** (default: `Indunilperamuna-Macos` or any custom folder).
  - Live full resolved path preview (`/Volumes/DataDisc2/Indunilperamuna-Macos/`) with direct Finder reveal.

- **⚡ Hybrid Internal SSD Stashing (`--except`)**:
  - Automatically identifies performance-critical files like `node_modules`, `.env`, `.git`, `vendor`, and `storage/logs`.
  - Leaves those files on the internal SSD while moving the gigabytes of remaining assets to the external disk.
  - Seamlessly links them back together into one unified tree.

- **🔄 1-Click Reverse Restore**:
  - Revert any offloaded project back to a standard local folder on your internal SSD with automatic re-integration of stashed files.

- **🛡️ Smart macOS Permissions Handling**:
  - Tests Full Disk Access (FDA) and Removable Volume permissions.
  - **Zero Noise**: If both permissions are granted, the Permission Setup screen and header alert icon are completely hidden.
  - If access is required, a subtle pulsating amber shield guides the user directly to `System Settings → Privacy & Security → Full Disk Access`.

- **📂 Active Symlinks Manager**:
  - View all currently offloaded items with their disk footprints and target locations.
  - Instant "Reveal in Finder" and "Reverse to SSD" actions.

- **🛡️ APFS Sparse & Atomic Swap Safe**:
  - Automatically preserves APFS sparse-file hole punching (`rsync -aP -S`) critical for Docker and VM images.
  - Uses atomic inode renames to permanently prevent macOS Finder `.DS_Store` `Directory not empty` removal errors.

---

## 🏗️ Architecture & Stack

```
Offload.app (Tauri 2 Menu Bar Accessory)
├── Top Caret Notch & Popover Frame (macOS 14px rounded-xl)
├── Frontend (Vue 3.5 + Vite 6 + Tailwind CSS 3.4)
│   ├── Lucide Icons & Responsive Views
│   ├── Segmented Navigation: [Offload] | [Symlinks] | [Settings]
│   └── System / Light / Dark Theme Engine
│
├── Native Bridge (Tauri v2 + Rust)
│   ├── TrayIconBuilder (Tray click positioning & Blur dismiss)
│   ├── IPC Handlers (get_disk_stats, scan_folder, check_permissions)
│   └── macOS TCC System Settings URL Dispatcher
│
└── System Core (macOS APFS CLI Engine)
    ├── ~/.local/bin/offload (Sparse-aware rsync + atomic APFS swap)
    └── Persistent Exception Store (~/.offload-internal/)
```

---

## 🍺 Installation via Homebrew

You can install Offload using the [Homebrew Cask](Docs/HOMEBREW_GUIDE.md):

```bash
# Add the tap and install
brew install --cask iperamuna/tap/offload
```

Or download the latest `.dmg` installer directly from [GitHub Releases](https://github.com/iperamuna/offload/releases).

---

## 🚀 Getting Started

### Prerequisites

- macOS 12 (Monterey) or later (Apple Silicon M1/M2/M3/M4 or Intel)
- [Rust & Cargo](https://rustup.rs) (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- [Bun](https://bun.sh) or Node.js (`v18+`)

### Development

```bash
# Clone or navigate to the directory
cd /Users/indunilperamuna/TauriApps/Offload

# Install frontend dependencies
bun install

# Run in development mode with live menu bar popover
bun run tauri dev
```

### Production Build

```bash
# Compile optimized native macOS application (.app and .dmg)
bun run tauri build
```

Compiled output:
- Application: `src-tauri/target/release/bundle/macos/Offload.app`
- Installer: `src-tauri/target/release/bundle/dmg/Offload_1.0.0_aarch64.dmg`

---

## ⌨️ Command-Line Interface (CLI)

The desktop app is powered by the standalone CLI tool located at `~/.local/bin/offload`. You can also use it directly from your terminal:

```bash
# Offload current folder to configured destination
cd ~/Development/my-app && offload

# Offload specific folder with custom destination and exceptions
offload ~/Herd/my-site --dest-base="/Volumes/DataDisc2/Indunilperamuna-Macos" --except="{node_modules},{.env}"

# Bring an offloaded directory back to internal SSD
offload ~/Development/my-app --reverse
```

See [OFFLOAD_README.md](Docs/OFFLOAD_README.md) for full CLI documentation.

---

## 📄 License

MIT © [Indunil Peramuna](https://github.com/iperamuna). See [LICENSE](LICENSE) for details.
