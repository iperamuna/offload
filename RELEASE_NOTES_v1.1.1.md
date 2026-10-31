# Offload v1.1.1 — macOS TCC Persistence & Dock Bookmark Sync 🚀

Reclaim gigabytes of space on your internal Mac SSD by offloading heavy directories, development environments, virtual machine disks, and AI models to external APFS drives — with zero path breakage.

---

### ✨ What's New in v1.1.1

- **🛡️ Full Disk Access (FDA) Permission Persistence Across Updates**:
  - Pinned the codesign designated requirement to `identifier "com.indunilperamuna.offload"` instead of an ephemeral ad-hoc `cdhash`.
  - Upgrading Offload via Homebrew or DMG no longer causes macOS to revoke Full Disk Access permissions.

- **📌 Native macOS Dock Downloads Bookmark & Stack Sync**:
  - Automatically regenerates macOS Dock security-scoped alias bookmarks when offloading system folders like `~/Downloads`.
  - Preserves the native folder stack icon and click functionality in the Dock without displaying a broken question mark (`?`).

- **🏷️ Dynamic Version Indicator**:
  - Updated the header badge to dynamically query and reflect the active application version (`v1.1.1`).

---

### 🍺 Installation & Upgrade via Homebrew

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
| **`Offload_1.1.1_Apple_Silicon.dmg`** | Apple Silicon (ARM64) | M1, M2, M3, M4, Pro, Max, Ultra |
| **`Offload_1.1.1_Apple_Intel.dmg`** | Intel (x86_64) | Core i5, i7, i9, Xeon |

---

### 🔑 Security & Integrity Verification

Every release is ad-hoc signed with a designated bundle requirement and verified with SHA-256 and MD5 checksums generated at build time.
