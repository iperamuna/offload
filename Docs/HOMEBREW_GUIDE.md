# Homebrew Distribution Guide 🍺

This guide walks you through setting up a **Homebrew Tap** so anyone on macOS can install **Offload** with a single command:

```bash
brew install --cask iperamuna/tap/offload
```

---

## 📋 Overview: Two Distribution Methods

| Method | Setup Effort | Approval Required | Best For |
| :--- | :--- | :--- | :--- |
| **1. Personal Tap (`iperamuna/homebrew-tap`)** *(Recommended)* | 5 minutes | None (Instant) | Full release control, instant updates, custom casks |
| **2. Homebrew Core (`homebrew/homebrew-cask`)** | PR review | Yes (Homebrew team) | Widespread public discovery (after achieving 30+ GitHub stars) |

---

## Method 1: Creating Your Personal Tap (Recommended)

Homebrew allows creating your own "Tap" repository on GitHub. Once created, users can install your app directly via `brew install --cask iperamuna/tap/offload`.

### Step 1: Create the Tap Repository on GitHub

1. Go to [github.com/new](https://github.com/new).
2. Set the repository name to:
   ```text
   homebrew-tap
   ```
   *(Homebrew requires the repository name to follow the convention `homebrew-<name>`, so `homebrew-tap` lets users refer to it as `iperamuna/tap`).*
3. Set visibility to **Public**.
4. Check **Add a README file** and create the repository.

---

### Step 2: Clone the Tap Repository Locally

```bash
git clone https://github.com/iperamuna/homebrew-tap.git
cd homebrew-tap
mkdir -p Casks
```

---

### Step 3: Create the Cask Formula (`Casks/offload.rb`)

Inside `homebrew-tap`, create the file `Casks/offload.rb`:

```ruby
cask "offload" do
  version "1.0.0"
  sha256 "REPLACE_WITH_ACTUAL_SHA256"

  url "https://github.com/iperamuna/offload/releases/download/v#{version}/Offload_#{version}_aarch64.dmg"
  name "Offload"
  desc "Smart macOS drive space offloader and symlink manager"
  homepage "https://github.com/iperamuna/offload"

  livecheck do
    url :url
    strategy :github_latest
  end

  auto_updates true
  depends_on macos: ">= :monterey"

  app "Offload.app"

  zap trash: [
    "~/.offload-internal",
    "~/Library/Application Support/com.indunilperamuna.offload",
    "~/Library/Preferences/com.indunilperamuna.offload.plist",
    "~/Library/Saved Application State/com.indunilperamuna.offload.savedState",
  ]
end
```

---

### Step 4: Calculate the SHA-256 Checksum

After you build or release `Offload_1.0.0_aarch64.dmg`, calculate its SHA-256 hash:

```bash
# On your local machine after running `bun run tauri build`:
shasum -a 256 src-tauri/target/release/bundle/dmg/Offload_1.0.0_aarch64.dmg
```

*Example output:*
```text
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
```

Replace `REPLACE_WITH_ACTUAL_SHA256` in `Casks/offload.rb` with this hash.

---

### Step 5: Commit and Push the Tap

```bash
git add Casks/offload.rb
git commit -m "feat: add offload v1.0.0 cask"
git push origin main
```

---

### Step 6: Test Installation on Any Mac

Now anyone can install Offload using Homebrew:

```bash
# 1-step install:
brew install --cask iperamuna/tap/offload

# Or tap first:
brew tap iperamuna/tap
brew install --cask offload
```

To update in the future:
```bash
brew upgrade --cask offload
```

---

## 🔄 Automating Tap Updates on Release

You can configure GitHub Actions to automatically update `Casks/offload.rb` in `iperamuna/homebrew-tap` every time a new release tag is pushed.

In your main `.github/workflows/release.yml`, you can add a job using `mislav/bump-homebrew-formula-action`:

```yaml
  update-cask:
    needs: build-macos
    runs-on: ubuntu-latest
    steps:
      - name: Bump Homebrew Cask
        uses: mislav/bump-homebrew-formula-action@v3
        with:
          formula-name: offload
          homebrew-tap: iperamuna/homebrew-tap
          download-url: https://github.com/iperamuna/offload/releases/download/${{ github.ref_name }}/Offload_1.0.0_aarch64.dmg
        env:
          COMMITTER_TOKEN: ${{ secrets.PERSONAL_ACCESS_TOKEN }}
```

---

## Method 2: Submitting to Official Homebrew Core (`homebrew/homebrew-cask`)

Once **Offload** gains traction (typically 30+ stars or notable community adoption), you can submit it to the official Homebrew repository:

1. Fork `https://github.com/Homebrew/homebrew-cask`.
2. Create a branch: `git checkout -b add-offload`.
3. Add `Casks/o/offload.rb` with the cask formula.
4. Run brew audit:
   ```bash
   brew audit --cask --new offload
   brew style --cask offload
   ```
5. Open a Pull Request on `Homebrew/homebrew-cask`.
6. Once merged, anyone can install simply by typing:
   ```bash
   brew install --cask offload
   ```

