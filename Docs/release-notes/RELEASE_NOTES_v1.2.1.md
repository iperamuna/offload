# Offload v1.2.1 — Instant Launch & Background Hydration ⚡

Reclaim gigabytes of space on your internal Mac SSD by offloading heavy directories, development environments, virtual machine disks, and AI models to external APFS drives — with zero path breakage.

---

### ✨ What's New in v1.2.1

- **⚡ Instant Menu Bar Popover Launch**:
  - Eliminated the startup delay where clicking the tray icon felt sluggish or unresponsive on cold start.
  - Decoupled synchronous `du -sh` directory scans from popover mounting, allowing the UI to open immediately.

- **💾 Persistent Size Caching & Background Threading**:
  - Implemented a fast in-memory and persistent disk cache (`~/.offload_sizes.json`) for active offloaded directories.
  - Symlink lists display cached sizes instantly (<1ms), while fresh directory size calculations are refreshed in a dedicated background worker without freezing the interface.

- **🔄 Startup Loading Indicator**:
  - Added an initial loading spinner state for visual feedback while verifying APFS volume states.

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
| **`Offload_1.2.1_Apple_Silicon.dmg`** | Apple Silicon (ARM64) | M1, M2, M3, M4, Pro, Max, Ultra |
| **`Offload_1.2.1_Apple_Intel.dmg`** | Intel (x86_64) | Core i5, i7, i9, Xeon |

---

### 🔑 Security & Integrity Verification

Every release is ad-hoc signed with a designated bundle requirement and verified with SHA-256 and MD5 checksums generated at build time.
