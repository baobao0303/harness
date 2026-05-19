# Scripts

This directory contains harness automation tools.

## Harness CLI

The `harness` script is the primary interface for the durable layer. It wraps
SQLite so agents and humans can record and query operational data.

```bash
scripts/harness init          # Create the database
scripts/harness intake ...    # Record a feature intake classification
scripts/harness story ...     # Add or update a story (test matrix row)
scripts/harness decision ...  # Add a decision or run its verification
scripts/harness backlog ...   # Add or close a backlog item
scripts/harness trace ...     # Record an agent execution trace
scripts/harness query ...     # Query harness data
scripts/harness migrate       # Apply pending schema migrations
```

Run `scripts/harness help` or `scripts/harness query help` for full usage.

The schema lives in `scripts/schema/` and is version-controlled. The database
file (`harness.db`) is `.gitignore`d.

Requires: `sqlite3`.

When the Rust delegated CLI is available for every routine command, `sqlite3`
is still required by the Bash fallback and by humans who inspect the database
directly. The future prebuilt Rust binary should reduce this runtime
requirement for normal harness use.

### Rust CLI Migration

`scripts/harness` can delegate migrated command slices to the Rust CLI when a
compiled binary exists at `target/debug/harness-cli` or at the path provided by
`HARNESS_RUST_CLI`.

Current migrated commands:

```bash
scripts/harness init
scripts/harness migrate
scripts/harness import brownfield
scripts/harness intake ...
scripts/harness story add ...
scripts/harness story update ...
scripts/harness decision add ...
scripts/harness decision verify ...
scripts/harness backlog add ...
scripts/harness backlog close ...
scripts/harness trace ...
scripts/harness query matrix
scripts/harness query backlog
scripts/harness query decisions
scripts/harness query intakes
scripts/harness query traces
scripts/harness query friction
scripts/harness query stats
scripts/harness query sql ...
```

`scripts/harness import brownfield` seeds or refreshes the durable database
from existing Harness v0 markdown in `docs/TEST_MATRIX.md`,
`docs/decisions/`, and `docs/HARNESS_BACKLOG.md`. This keeps already-installed
Harness repos on the Rust CLI path without losing their populated operating
docs.

Set `HARNESS_DISABLE_RUST_CLI=1` to force the Bash implementation while parity
work is in progress.

## Installer

The upstream installer applies the Harness v0 operating files and folder
structure to a target project directory. It defaults to the current directory,
accepts a target path, and asks interactive users whether to `1. Merge`,
`2. Override`, or `3. Stop` when the target already contains `AGENTS.md`,
`docs/`, or `scripts/`.
Non-interactive installs stop on those protected paths unless `--merge` or
`--override` is provided. Use `--merge` as the safe update path for repositories
that already have Harness: it keeps existing files in place and creates only
missing Harness files. Use `--override` only when replacing the protected
Harness surface is intentional.

```bash
curl -fsSL "https://raw.githubusercontent.com/hoangnb24/harness-experimental/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --yes
```

```bash
curl -fsSL "https://raw.githubusercontent.com/hoangnb24/harness-experimental/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --merge --yes
```

The installer must stay limited to harness files. Do not use it to scaffold
application source folders, package scripts, CI, tests, platform shells, or fake
validation commands. The installer script is not part of the installed project
payload.

By default the installer also downloads the prebuilt Rust Harness CLI for the
current platform into `scripts/bin/harness-cli` and verifies its `.sha256`
checksum before making it executable. Set `HARNESS_CLI_BASE_URL` to point at an
alternate release artifact directory, or pass `--skip-cli-download` to install
only the Bash fallback.

## Schema Migrations

Migration files live under `scripts/schema/` and are named `NNN-description.sql`
where `NNN` is a zero-padded version number. Run `scripts/harness migrate` to
apply pending migrations.

## Future Command Contract

Expected future checks:

```text
validate:quick
  format, lint, typecheck, unit tests, architecture check

test:integration
  backend contract and integration checks

test:e2e
  user-visible end-to-end flows

test:platform
  platform shell smoke checks, if the project has a native shell

test:release
  full suite, log checks, and performance smoke
```

## Release Packaging

Build the current-platform Rust CLI release artifact from the source repo:

```bash
scripts/build-harness-cli-release.sh
```

The script writes `dist/harness-cli-<platform>` and
`dist/harness-cli-<platform>.sha256`. Supported labels are:

- `macos-arm64`
- `macos-x64`
- `linux-x64`
- `linux-arm64`

For cross-compilation, pass a Cargo target triple:

```bash
scripts/build-harness-cli-release.sh --target x86_64-unknown-linux-gnu
```

GitHub releases are produced by
`.github/workflows/harness-cli-release.yml`. Push a tag matching `v*` or
`harness-cli-v*` to run the verification job, build all supported targets on
native hosted runners, and upload these release assets:

- `harness-cli-macos-arm64`
- `harness-cli-macos-arm64.sha256`
- `harness-cli-macos-x64`
- `harness-cli-macos-x64.sha256`
- `harness-cli-linux-x64`
- `harness-cli-linux-x64.sha256`
- `harness-cli-linux-arm64`
- `harness-cli-linux-arm64.sha256`
