#!/usr/bin/env bash
# ==============================================================================
# Harness - Global System PATH Installer
# ==============================================================================
# Downloads the verified, platform-specific prebuilt Rust binary and installs it
# globally so you can run 'harness' anywhere in your terminal.
# ==============================================================================

set -euo pipefail

# Output coloring helpers
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0;0m' # No Color

log() {
  printf "${GREEN}[+]${NC} %s\n" "$*"
}

info() {
  printf "${BLUE}[i]${NC} %s\n" "$*"
}

warn() {
  printf "${YELLOW}[!]${NC} %s\n" "$*" >&2
}

fail() {
  printf "${RED}[x] Error:${NC} %s\n" "$*" >&2
  exit 1
}

# Print beautiful banner
banner() {
  cat <<EOF
${CYAN}  _    _                                 
 | |  | |                                
 | |__| | __ _ _ __ _ __   ___  ___ ___  
 |  __  |/ _\` | '__| '_ \ / _ \/ __/ __| 
 | |  | | (_| | |  | | | |  __/\__ \__ \ 
 |_|  |_|\__,_|_|  |_| |_|\___||___/___/ ${NC}
${MAGENTA}       Global System PATH Installer${NC}
==============================================
EOF
}

usage() {
  banner
  cat <<EOF
Usage: install-global.sh [options]

Downloads and installs the prebuilt Harness Rust CLI globally.

Options:
  -h, --help      Show this help message.

Environment Variables:
  HARNESS_GITHUB_REPO    GitHub repository to download from (default: baobao0303/harness)
  HARNESS_CLI_BASE_URL   Base download URL (default: GitHub Releases)
  HARNESS_INSTALL_DIR    Target installation directory (default: auto-detected /usr/local/bin or ~/.local/bin)

Examples:
  ./install-global.sh
  HARNESS_GITHUB_REPO="myfork/harness" ./install-global.sh
EOF
}

# Parse options
if [ "$#" -gt 0 ]; then
  case "$1" in
    -h|--help)
      usage
      exit 0
      ;;
    *)
      warn "Unknown option: $1"
      usage
      exit 1
      ;;
  esac
fi

# Configuration
GITHUB_REPO="${HARNESS_GITHUB_REPO:-baobao0303/harness}"
CLI_BASE_URL="${HARNESS_CLI_BASE_URL:-https://github.com/${GITHUB_REPO}/releases/latest/download}"

# 1. Platform Detection
detect_platform() {
  local os arch
  os="$(uname -s)"
  arch="$(uname -m)"

  case "$os:$arch" in
    Darwin:arm64)  printf 'macos-arm64' ;;
    Darwin:x86_64) printf 'macos-x64' ;;
    Linux:x86_64)  printf 'linux-x64' ;;
    Linux:aarch64|Linux:arm64) printf 'linux-arm64' ;;
    *)
      fail "Unsupported platform: $os/$arch. Harness currently supports macOS (arm64/x64) and Linux (x64/arm64)."
      ;;
  esac
}

# 2. Checksum Verification
sha256_file() {
  local file="$1"
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$file" | awk '{ print $1 }'
  elif command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$file" | awk '{ print $1 }'
  else
    fail "shasum or sha256sum is required to verify the Harness download integrity."
  fi
}

# 3. Resolve Destination Directory
resolve_destination() {
  if [ -n "${HARNESS_INSTALL_DIR:-}" ]; then
    printf '%s' "$HARNESS_INSTALL_DIR"
    return
  fi

  local global_dir="/usr/local/bin"
  local user_dir="$HOME/.local/bin"

  # We try to install globally in /usr/local/bin if writable or if running as root
  if [ -w "$global_dir" ] || [ "${EUID:-$(id -u)}" -eq 0 ]; then
    printf '%s' "$global_dir"
  elif [ -w "$user_dir" ] || mkdir -p "$user_dir" 2>/dev/null; then
    printf '%s' "$user_dir"
  else
    printf '%s' "$HOME/bin"
  fi
}

main() {
  banner
  info "Harness Global Terminal Installer starting..."
  
  command -v curl >/dev/null 2>&1 || fail "curl is required to run this installer."

  local platform binary_name binary_url checksum_url dest_dir target tmp_dir binary_tmp checksum_tmp expected actual
  platform="$(detect_platform)"
  binary_name="harness-$platform"
  binary_url="$CLI_BASE_URL/$binary_name"
  checksum_url="$binary_url.sha256"

  dest_dir="$(resolve_destination)"
  target="$dest_dir/harness"

  info "Detected platform: ${GREEN}$platform${NC}"
  info "Repository target: ${GREEN}$GITHUB_REPO${NC}"
  info "Target installation directory: ${GREEN}$dest_dir${NC}"

  # Create temp directory for downloading
  tmp_dir="$(mktemp -d)"
  binary_tmp="$tmp_dir/$binary_name"
  checksum_tmp="$tmp_dir/$binary_name.sha256"

  log "Downloading prebuilt release binary..."
  curl -fsSL "$binary_url" -o "$binary_tmp" || fail "Failed to download release binary from $binary_url"
  
  log "Downloading SHA256 verification hash..."
  curl -fsSL "$checksum_url" -o "$checksum_tmp" || fail "Failed to download checksum from $checksum_url"

  # Verify checksum
  log "Verifying SHA256 integrity..."
  expected="$(awk '{ print $1; exit }' "$checksum_tmp")"
  [ -n "$expected" ] || fail "Downloaded checksum file is empty."
  actual="$(sha256_file "$binary_tmp")"
  
  if [ "$actual" != "$expected" ]; then
    rm -rf "$tmp_dir"
    fail "Checksum mismatch for $binary_name! Expected $expected, got $actual"
  fi
  log "Integrity verified successfully."

  # Copy to destination (use sudo if target directory is not writable by current user)
  log "Installing binary to $target..."
  if [ ! -w "$dest_dir" ]; then
    warn "Directory $dest_dir is not writable by the current user."
    info "Requesting sudo permissions to copy binary..."
    sudo mkdir -p "$dest_dir"
    sudo cp "$binary_tmp" "$target"
    sudo chmod 755 "$target"
  else
    mkdir -p "$dest_dir"
    cp "$binary_tmp" "$target"
    chmod 755 "$target"
  fi

  rm -rf "$tmp_dir"
  log "Installation completed successfully!"
  echo ""

  # Provide intelligent feedback on PATH settings depending on host shell
  local active_shell
  active_shell="$(basename "${SHELL:-bash}")"

  case ":$PATH:" in
    *:"$dest_dir":*)
      log "${GREEN}Harness is now globally available!${NC} Try running:"
      printf "  ${CYAN}harness query stats${NC}\n\n"
      ;;
    *)
      warn "The directory $dest_dir is not currently in your system PATH."
      info "To run harness globally, please add it to your PATH."
      
      case "$active_shell" in
        zsh)
          info "For Zsh, add this line to your ~/.zshrc:"
          printf "  ${YELLOW}export PATH=\"\$PATH:$dest_dir\"${NC}\n"
          printf "  Then reload with: ${CYAN}source ~/.zshrc${NC}\n\n"
          ;;
        fish)
          info "For Fish, run this command in your terminal:"
          printf "  ${YELLOW}fish_add_path $dest_dir${NC}\n\n"
          ;;
        *)
          info "For Bash, add this line to your ~/.bashrc (or ~/.bash_profile on macOS):"
          printf "  ${YELLOW}export PATH=\"\$PATH:$dest_dir\"${NC}\n"
          printf "  Then reload with: ${CYAN}source ~/.bashrc${NC}\n\n"
          ;;
      esac
      ;;
  esac

  info "Verifying global execution..."
  if "$target" --version >/dev/null 2>&1; then
    "$target" --version
  else
    echo "harness installed (run 'harness init' inside a repository to activate)."
  fi
}

main "$@"
