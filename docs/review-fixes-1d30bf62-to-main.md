# Review Fixes: 1d30bf62 to main

Base: `1d30bf62a30cd7e65ebcefed765b3f924d381b49`
Starting head: `fd8151968e7e0623ce76beadb2c41641268c0691`
Branch: `review/main-1d30bf62-to-fd81519`
Harness intake: `#34`

## Pass 1

- Status: findings fixed; validation in progress.
- Command: `codex review --base 1d30bf62a30cd7e65ebcefed765b3f924d381b49`
- Findings:
  - P2: `decision verify` ran stored commands from the caller cwd instead of
    the Harness repo root.
  - P3: Rust intake insertion stored absent `--flags` and `--docs` lists as the
    text `"null"` instead of SQL `NULL`.
- Fixes:
  - Set decision verification commands to run with `self.repo_root` as cwd.
  - Store absent intake list fields with `CsvList::as_json_text()` so rusqlite
    binds SQL `NULL`.
  - Added regression coverage for both behaviors.
  - Updated US-002 validation evidence from 9 to 10 Rust tests.
- Validation:
  - `cargo fmt --check`
  - `cargo test --workspace` passed with 10 tests.
  - Separate `bash -n` checks for `scripts/install-harness.sh`,
    `scripts/bin/harness-cli`, and `scripts/build-harness-cli-release.sh`.
  - `git diff --check`
  - `scripts/bin/harness-cli query matrix`

## Pass 2

- Status: finding fixed; validation in progress.
- Command: `codex review --base 1d30bf62a30cd7e65ebcefed765b3f924d381b49`
- Findings:
  - P2: `--refresh-agent-shim` could overwrite an existing
    `$BACKUP_DIR/AGENTS.md` created earlier by `--override` or `--force`.
- Fixes:
  - Made `backup_agent_file` preserve an existing `AGENTS.md` backup instead
    of replacing it during the refresh step.
- Validation:
  - Temp `--override --refresh-agent-shim --yes` install preserved the original
    `AGENTS.md` in `.harness-backup/.../AGENTS.md`.
  - Separate `bash -n` checks for `scripts/install-harness.sh`,
    `scripts/bin/harness-cli`, and `scripts/build-harness-cli-release.sh`.
  - `cargo fmt --check`
  - `cargo test --workspace` passed with 10 tests.
  - `git diff --check`
  - `scripts/bin/harness-cli query matrix`

## Pass 3

- Status: interrupted by Codex usage limit.
- Command: `codex review --base 1d30bf62a30cd7e65ebcefed765b3f924d381b49`
- Result: review did not complete; Codex reported the usage limit and said to
  retry at 4:33 PM.
- Findings: unavailable.
- Fixes: none.
- Validation: final no-findings proof is still pending.

## Pass 4

- Status: finding fixed; validation in progress.
- Command: `codex review --base 1d30bf62a30cd7e65ebcefed765b3f924d381b49`
- Findings:
  - P2: `--merge` installs preserved normal Harness files but still overwrote
    an existing `scripts/bin/harness-cli` binary without `--force`.
- Fixes:
  - Made `install_harness_cli_binary` skip an existing downloaded CLI binary
    during merge mode unless `--force` is provided.
- Validation:
  - Temp existing-Harness `--merge --yes` install preserved the original
    `scripts/bin/harness-cli` checksum and content.
  - Separate `bash -n` checks for `scripts/install-harness.sh`,
    `scripts/bin/harness-cli`, and `scripts/build-harness-cli-release.sh`.
  - `cargo fmt --check`
  - `cargo test --workspace` passed with 10 tests.
  - `git diff --check`
  - `scripts/bin/harness-cli query matrix`

## Pass 5

- Status: findings fixed; validation in progress.
- Command: `codex review --base 1d30bf62a30cd7e65ebcefed765b3f924d381b49`
- Findings:
  - P2: source-checkout installs defaulted the CLI download source to local
    `dist/`, which is ignored and may be absent in a fresh clone.
  - P3: the release workflow used one `bash -n` invocation with three script
    paths, which only syntax-checks the first script.
- Fixes:
  - Default `HARNESS_CLI_BASE_URL` to the published release URL even when the
    Harness source files are local; keep local artifact directories available
    through explicit `HARNESS_CLI_BASE_URL=file:///.../dist`.
  - Changed the release workflow and validation docs to run `bash -n`
    separately for each shell script.
  - Updated US-002 durable evidence to name the source-checkout installer smoke
    and separate shell syntax checks.
- Validation:
  - Temp source-checkout install succeeded with local `dist/` temporarily moved
    away; the installer downloaded from the published release URL and installed
    an executable `scripts/bin/harness-cli`.
  - Separate `bash -n` checks for `scripts/install-harness.sh`,
    `scripts/bin/harness-cli`, and `scripts/build-harness-cli-release.sh`.
  - `cargo fmt --check`
  - `cargo test --workspace` passed with 10 tests.
  - `git diff --check`
  - `scripts/bin/harness-cli query matrix`

## Pass 6

- Status: clean review.
- Command: `codex review --base 1d30bf62a30cd7e65ebcefed765b3f924d381b49`
- Result: no discrete regression findings.
- Reviewer note: the Rust CLI, installer changes, and release workflow appeared
  internally consistent, and validation commands exercised during review passed.
- Fixes: none.
- Validation:
  - Separate `bash -n` checks for `scripts/install-harness.sh`,
    `scripts/bin/harness-cli`, and `scripts/build-harness-cli-release.sh`.
  - `cargo fmt --check`
  - `cargo test --workspace` passed with 10 tests.
  - `git diff --check`
  - `scripts/bin/harness-cli query matrix`
