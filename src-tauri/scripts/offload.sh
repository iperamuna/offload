#!/usr/bin/env bash
#
# offload
# CLI utility to:
# 1. Offload any directory to DataDisc2, keeping specified items on internal SSD via --except.
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
            # Strip outer quotes or curly braces
            RAW_EXCEPT="${RAW_EXCEPT//\{/}"
            RAW_EXCEPT="${RAW_EXCEPT//\}/}"
            IFS=',' read -ra ADDS <<< "${RAW_EXCEPT}"
            for item in "${ADDS[@]}"; do
                # Trim leading/trailing whitespace
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

refresh_dock_bookmark() {
    local target_path="$1"
    # If the modified folder is in ~/ (such as ~/Downloads, ~/Desktop, ~/Documents), refresh macOS Dock's bookmark
    python3 -c "
import plistlib, os, subprocess, sys

target = sys.argv[1]
try:
    cmd = '''swift -e '
import Foundation
let path = NSString(string: \"''' + target + '''\").expandingTildeInPath
let url = URL(fileURLWithPath: path)
let b = try! url.bookmarkData(options: [], includingResourceValuesForKeys: nil, relativeTo: nil)
FileHandle.standardOutput.write(b)
' '''
    proc = subprocess.run(cmd, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=5)
    if proc.returncode == 0 and len(proc.stdout) > 50:
        dock_plist = os.path.expanduser('~/Library/Preferences/com.apple.dock.plist')
        with open(dock_plist, 'rb') as fp:
            pl = plistlib.load(fp)
        updated = False
        target_norm = os.path.normpath(os.path.expanduser(target))
        for item in pl.get('persistent-others', []):
            td = item.get('tile-data', {})
            raw_url = td.get('file-data', {}).get('_CFURLString', '')
            tile_path = raw_url.replace('file://', '')
            if os.path.normpath(tile_path) == target_norm or td.get('file-label') == os.path.basename(target_norm):
                td['book'] = proc.stdout
                updated = True
        if updated:
            with open(dock_plist, 'wb') as fp:
                plistlib.dump(pl, fp)
            subprocess.run(['killall', 'Dock'], timeout=3)
except Exception:
    pass
" "${target_path}" 2>/dev/null || true
}

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

    # Relative path
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
    echo -e "  ${BOLD}Source on DataDisc2:${NC}    ${BLUE}${DEST_DIR}${NC}"
    echo -e "  ${BOLD}Data Size to Bring Back:${NC}${GREEN}$(du -sh "${DEST_DIR}" 2>/dev/null | awk '{print $1}')${NC}"
    echo -e "  ${BOLD}Action:${NC}                 Copy back to SSD $\rightarrow$ Reintegrate internal files $\rightarrow$ Remove DataDisc2 copy"
    echo ""

    if ! confirm_action "Are you sure you want to reverse this offload and bring data back to internal SSD?"; then
        echo -e "${YELLOW}[*] Operation cancelled.${NC}"
        exit 0
    fi

    echo ""
    echo -e "[*] Creating temporary holding directory on internal SSD..."
    local TEMP_RESTORE="${REAL_LINK_PATH}.restore_tmp.$$"
    mkdir -p "${TEMP_RESTORE}"

    echo -e "[*] Transferring data from DataDisc2 back to internal SSD..."
    # Copy from external disk, but do not overwrite links to internal except store
    rsync -aP "${DEST_DIR}/" "${TEMP_RESTORE}/"

    # Reintegrate any excepted files stored internally
    if [ -d "${STORE_DIR}" ]; then
        echo -e "[*] Reintegrating files that were kept on internal SSD..."
        (
            cd "${STORE_DIR}"
            find . -type f -o -type d | while read -r subitem; do
                [ "${subitem}" = "." ] && continue
                subitem="${subitem#./}"
                if [ -e "${STORE_DIR}/${subitem}" ]; then
                    # Remove the back-symlink in restored folder if present
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

    echo -e "[*] Cleaning up external copy on DataDisc2..."
    chmod -R -N "${DEST_DIR}" 2>/dev/null || true
    chflags -R nouchg "${DEST_DIR}" 2>/dev/null || true
    find "${DEST_DIR}" -name ".DS_Store" -delete 2>/dev/null || true
    rm -rf "${DEST_DIR}" 2>/dev/null || true

    refresh_dock_bookmark "${REAL_LINK_PATH}"

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

echo -e "  ${BOLD}Action:${NC}                Copy to DataDisc2 $\rightarrow$ Link internal exceptions $\rightarrow$ Symlink"
echo ""

if [ -e "${DEST_DIR}" ]; then
    echo -e "${YELLOW}[!] Warning: Destination path already exists on DataDisc2:${NC}"
    echo "    ${DEST_DIR}"
    echo ""
fi

if ! confirm_action "Are you sure you want to offload this folder to DataDisc2?"; then
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

echo -e "${GREEN}[✓] Data transfer to DataDisc2 verified.${NC}"

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

# Clear any macOS ACLs (e.g. deny delete on Downloads/Desktop/Documents), immutable flags (uchg), and purge .DS_Store
chmod -h -N "${TARGET_DIR}" 2>/dev/null || true
chflags -R nouchg "${TARGET_DIR}" 2>/dev/null || true
find "${TARGET_DIR}" -name ".DS_Store" -delete 2>/dev/null || true

# Atomic rename on APFS moves the folder out of the path instantly
# so Finder cannot regenerate .DS_Store into the path while rm is running
TRASH_TMP="${TARGET_DIR}.offload_trash.$$"
if mv "${TARGET_DIR}" "${TRASH_TMP}" 2>/dev/null; then
    # Create symbolic link pointing to destination immediately
    echo -e "[*] Creating symbolic link pointing to ${DEST_DIR}..."
    ln -s "${DEST_DIR}" "${TARGET_DIR}"
    # Delete temporary folder
    chmod -R -N "${TRASH_TMP}" 2>/dev/null || true
    chflags -R nouchg "${TRASH_TMP}" 2>/dev/null || true
    find "${TRASH_TMP}" -name ".DS_Store" -delete 2>/dev/null || true
    rm -rf "${TRASH_TMP}" 2>/dev/null || true
else
    # Fallback direct removal
    chmod -R -N "${TARGET_DIR}" 2>/dev/null || true
    chflags -R nouchg "${TARGET_DIR}" 2>/dev/null || true
    find "${TARGET_DIR}" -name ".DS_Store" -delete 2>/dev/null || true
    rm -rf "${TARGET_DIR}" 2>/dev/null || true
    echo -e "[*] Creating symbolic link pointing to ${DEST_DIR}..."
    ln -s "${DEST_DIR}" "${TARGET_DIR}"
fi

# Symlink excepted files back into destination directory so they are accessible seamlessly
if [ ${#VALID_EXCEPTS[@]} -gt 0 ]; then
    echo -e "[*] Linking internal excepted items into external directory structure..."
    for exc in "${VALID_EXCEPTS[@]}"; do
        mkdir -p "$(dirname "${DEST_DIR}/${exc}")"
        ln -s "${STORE_DIR}/${exc}" "${DEST_DIR}/${exc}"
        echo -e "    [✓] Linked internal: ${exc} -> ${STORE_DIR}/${exc}"
    done
fi

# Refresh Dock bookmark if target was in the user's home (e.g. ~/Downloads)
refresh_dock_bookmark "${TARGET_DIR}"

# Industry Standard APFS Reclamation:
# When heavy files are deleted on APFS, local Time Machine snapshots can lock the freed blocks.
# Running tmutil thinlocalsnapshots asks macOS to purge retention on deleted blocks so internal space is immediately freed.
echo -e "[*] Asking macOS APFS to reclaim purgeable snapshot blocks..."
tmutil thinlocalsnapshots / 9999999999 4 2>/dev/null || true

echo ""
echo -e "${GREEN}[✓] Successfully offloaded:${NC}"
echo "    ${TARGET_DIR} -> ${DEST_DIR}"

if [ "$PWD" = "${TARGET_DIR}" ]; then
    echo -e "${YELLOW}[i] Tip: You were inside the moved directory. Run 'cd .' or open a new subshell to refresh.${NC}"
fi

echo -e "${BOLD}${GREEN}=== Done! Space reclaimed on your internal SSD ===${NC}"
