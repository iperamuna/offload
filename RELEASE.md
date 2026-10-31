# Release & Distribution Guide

This document outlines the procedure for compiling, bundling, and distributing production releases of **Offload** for macOS.

---

## 📦 Build Checklist

Before creating a release:

1. **Verify Version Numbers**:
   Ensure the version is synchronized across all configuration files:
   - `package.json` (`"version": "1.0.0"`)
   - `src-tauri/tauri.conf.json` (`"version": "1.0.0"`)
   - `src-tauri/Cargo.toml` (`version = "1.0.0"`)

2. **Verify macOS Menu Bar Configuration**:
   Ensure `src-tauri/Info.plist` includes `<key>LSUIElement</key><true/>` so the app runs cleanly in the macOS menu bar without a Dock icon.

3. **Verify Build Dependencies**:
   ```bash
   cd /Users/indunilperamuna/TauriApps/Offload
   bun install
   ```

4. **Run Verification Check**:
   ```bash
   bun run build
   cd src-tauri && cargo check
   ```

---

## 🛠️ Compiling Production Bundles

Run the Tauri build pipeline:

```bash
cd /Users/indunilperamuna/TauriApps/Offload
bun run tauri build
```

This compiles:
- Optimized frontend assets via Vite.
- Release-mode Rust native binary with dead-code elimination, LTO, and tray integration.
- macOS `.app` bundle configured as a menu bar accessory.
- Disk image installer (`.dmg`).

### Build Outputs

| Artifact | Output Path |
| :--- | :--- |
| **macOS Menu Bar Application** | `src-tauri/target/release/bundle/macos/Offload.app` |
| **Disk Image Installer** | `src-tauri/target/release/bundle/dmg/Offload_1.0.0_aarch64.dmg` |

---

## 🔏 Code Signing & Notarization (Optional)

For distributing outside the App Store or sharing with other Mac users without Gatekeeper warnings:

### 1. Configure Environment Variables
Set your Apple Developer credentials:
```bash
export APPLE_CERTIFICATE="<base64-encoded-p12-certificate>"
export APPLE_CERTIFICATE_PASSWORD="<certificate-password>"
export APPLE_SIGNING_IDENTITY="Developer ID Application: Indunil Peramuna (TeamID)"
export APPLE_ID="iperamuna@gmail.com"
export APPLE_PASSWORD="app-specific-password"
export APPLE_TEAM_ID="YOUR_TEAM_ID"
```

### 2. Notarize Build
Tauri v2 automatically signs and submits the bundle to Apple Notary Service when the above environment variables are defined during `bun run tauri build`.

---

## 🚀 Creating a GitHub Release

1. Commit and tag the release:
   ```bash
   git add .
   git commit -m "feat: release v1.0.0"
   git tag -a v1.0.0 -m "Release v1.0.0: Initial Launch"
   git push origin main --tags
   ```
2. The GitHub Actions Release Workflow (`.github/workflows/release.yml`) will automatically compile the macOS `.dmg`, compute MD5 and SHA256 checksums, and publish the release with downloadable assets attached!
