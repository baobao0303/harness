# US-003 Global Terminal Installer for Releases

## Status

implemented

## Lane

normal

## Product Contract

A user can install the Harness CLI globally on their terminal using a simple shell installer, enabling them to run `harness` directly from any project directory without prepending directory scripts.

## Relevant Product Docs

- [README.md](../../README.md)
- [HARNESS.md](../HARNESS.md)
- [scripts/README.md](../../scripts/README.md)

## Acceptance Criteria

- **Automated Platform Detection**: The script automatically detects the host architecture and OS, supporting macOS (arm64, x64) and Linux (x64, arm64).
- **Integrity Validation**: Computes and matches the SHA256 checksum of the downloaded prebuilt binary against the released `.sha256` asset to guarantee secure installations.
- **Configurable Release Sources**: Supports custom repository targets via `HARNESS_GITHUB_REPO` (defaulting to `baobao0303/harness`) and custom release base URLs via `HARNESS_CLI_BASE_URL`.
- **Intelligent PATH Detection**: Identifies whether the installation folder is present in the host system's `$PATH` variable and, if missing, prints exact configuration commands customized for the user's active shell (Zsh, Bash, or Fish).
- **Graceful Directory Resolution**: Installs into `/usr/local/bin` by default, falling back gracefully to user-local directories (`~/.local/bin` or `~/bin`) and prompting for sudo only when system-wide paths require root elevation.
- **Continuous Integration**: Validated by release jobs in `.github/workflows/harness-cli-release.yml` using strict POSIX shell format analysis.

## Design Notes

- **Installer Path**: `scripts/install-global.sh`
- **Default Release Origin**: `baobao0303/harness`
- **One-liner Lanch Command**: `curl -fsSL https://raw.githubusercontent.com/baobao0303/harness/main/scripts/install-global.sh | bash`

## Validation

| Layer | Expected proof |
| --- | --- |
| Unit | Shell format checker `bash -n scripts/install-global.sh` passes successfully. |
| Integration | Execution of the installer with local variables pointing to precompiled binaries inside `dist/` installs Harness globally and correctly prints version statistics. |
| E2E | Running the global installer configures the binary, provides proper export messages, and prints the operational databases status. |
| Platform | Validated on macOS terminal environment. |

## Harness Delta

Integrates global terminal deployment capability, expanding Harness's availability from local workspaces to developers' systems and simplifying repository setup.

## Evidence

- **Shell Format Syntax Verification**:
  ```bash
  $ bash -n scripts/install-global.sh
  # Completed with exit code 0 (no syntax issues)
  ```

- **Mock Installation Integration & End-to-End Test**:
  Executed local testing using environment overrides:
  ```bash
  $ HARNESS_INSTALL_DIR="/tmp/mock-harness-install" \
    HARNESS_CLI_BASE_URL="file:///tmp/mock-release" \
    bash scripts/install-global.sh
  ```
  Resulting Terminal Output:
  ```text
    _    _                                 
   | |  | |                                
   | |__| | __ _ _ __ _ __   ___  ___ ___  
   |  __  |/ _` | '__| '_ \ / _ \/ __/ __| 
   | |  | | (_| | |  | | | |  __/\__ \__ \ 
   |_|  |_|\__,_|_|  |_| |_|\___||___/___/ 
         Global System PATH Installer
  ==============================================
  [i] Harness Global Terminal Installer starting...
  [i] Detected platform: macos-arm64
  [i] Repository target: baobao0303/harness
  [i] Target installation directory: /tmp/mock-harness-install
  [+] Downloading prebuilt release binary...
  [+] Downloading SHA256 verification hash...
  [+] Verifying SHA256 integrity...
  [+] Integrity verified successfully.
  [+] Installing binary to /tmp/mock-harness-install/harness...
  [+] Installation completed successfully!

  [!] The directory /tmp/mock-harness-install is not currently in your system PATH.
  [i] To run harness globally, please add it to your PATH.
  [i] For Zsh, add this line to your ~/.zshrc:
    export PATH="$PATH:/tmp/mock-harness-install"
    Then reload with: source ~/.zshrc

  [i] Verifying global execution...
  mock harness cli version 0.2.0
  ```

