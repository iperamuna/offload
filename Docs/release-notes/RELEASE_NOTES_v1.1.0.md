# Offload v1.1.0 — Smart Relocation & Symlink Sync 🚀

Reclaim gigabytes of space on your internal Mac SSD by offloading heavy directories, development environments, virtual machine disks, and AI models to external APFS drives — with zero path breakage.

---

### ✨ What's New in v1.1.0

- **🎯 Drag & Drop Focus Retention**:
  - The Menu Bar popover now reliably remains open and active while you drag folders from macOS Finder into the dropzone, even when unpinned.
  - Automatically handles native drag enter, drag over, and drag leave states.

- **❌ Direct Selection Cancel & Clear**:
  - Added a dedicated **Cancel** action alongside **Change** whenever a folder is dropped or browsed.
  - Instantly clears selected folder, analyzed subitems, and exception chips without needing to dismiss or reload the app.

- **🔄 Target Volume Symlink Detection & 1-Click Reconnect**:
  - Clicking the **Refresh** icon (🔄) on the top bar or switching destination volumes inspects the external storage volume.
  - Detects existing offloaded folders that may be missing active symbolic links on your current Mac.
  - Highlights missing links with an amber badge and gives a 1-click **Link (🔗)** action to re-establish the symlink instantly.

- **🍏 Enhanced Directory ACL Handling**:
  - Automatically strips macOS `group:everyone deny delete` access control lists before swapping folders, allowing standard system directories like `~/Downloads` and `~/Desktop` to be offloaded without terminal permission errors.

---

### 🍺 Installation & Upgrade via Homebrew

Homebrew automatically detects your Mac's CPU architecture and installs or upgrades the native build:

```bash
# Update tap and upgrade
brew update
brew upgrade --cask offload

# Or fresh install
brew install --cask iperamuna/tap/offload
```

---

### 📦 Direct Downloads

| Asset | Architecture | Target Hardware |
| :--- | :--- | :--- |
| **`Offload_1.1.0_Apple_Silicon.dmg`** | Apple Silicon (ARM64) | M1, M2, M3, M4, Pro, Max, Ultra |
| **`Offload_1.1.0_Apple_Intel.dmg`** | Intel (x86_64) | Core i5, i7, i9, Xeon |

---

### ℹ️ First Launch on macOS (Gatekeeper Note)

If downloaded directly outside Homebrew:
1. Right-click (or Control-click) `Offload.app` in `/Applications` or inside the DMG.
2. Select **Open** from the context menu.
3. Click **Open** on the confirmation dialog.
