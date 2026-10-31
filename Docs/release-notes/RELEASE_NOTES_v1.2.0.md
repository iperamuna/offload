# Offload v1.2.0 — Instant APFS Space Reclamation & Enhanced macOS Integration 🚀

Reclaim gigabytes of space on your internal Mac SSD by offloading heavy directories, development environments, virtual machine disks, and AI models to external APFS drives — with zero path breakage.

---

### ✨ What's New in v1.2.0

- **⚡ Instant APFS Snapshot Space Reclamation**:
  - Automatically invokes Apple's non-destructive snapshot thinning engine (`tmutil thinlocalsnapshots`) immediately after an offload or reverse operation.
  - Prevents APFS Time Machine local snapshots from holding onto deleted file blocks as "purgeable space", ensuring your reclaimed gigabytes are immediately reflected in macOS Finder and system storage meters.

- **📌 Native macOS Dock Downloads & Folders Bookmark Sync**:
  - Automatically updates macOS Dock security-scoped alias bookmarks when offloading system directories such as `~/Downloads` or `~/Desktop`.
  - Preserves native folder stack icons, fan expansion, and click functionality in the Dock without broken question mark (`?`) placeholders.

- **🛡️ Permanent Full Disk Access (FDA) Permission Retention**:
  - Pinned the codesign designated requirement to `identifier "com.indunilperamuna.offload"` across builds.
  - Upgrades via Homebrew Cask or DMG no longer cause macOS TCC to revoke or re-prompt for Full Disk Access.

- **🏷️ Dynamic Application Version Badge**:
  - The menu bar header badge now dynamically tracks and displays the active version at runtime (`v1.2.0`).

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
| **`Offload_1.2.0_Apple_Silicon.dmg`** | Apple Silicon (ARM64) | M1, M2, M3, M4, Pro, Max, Ultra |
| **`Offload_1.2.0_Apple_Intel.dmg`** | Intel (x86_64) | Core i5, i7, i9, Xeon |

---

### 🔑 Security & Integrity Verification

Every release is ad-hoc signed with a designated bundle requirement and verified with SHA-256 and MD5 checksums generated at build time.

