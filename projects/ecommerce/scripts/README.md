# Scripts

This directory contains harness automation tools.

## Harness MCP Server (Recommended)

Harness provides a native Model Context Protocol (MCP) server that runs over standard I/O (stdio). This is the **primary, recommended, and modern way** to integrate Harness with AI Coding Agents (such as Claude Code, Cursor, Windsurf, or Copilot), enabling them to run Harness actions autonomously without requiring manual terminal commands.

### IDE Integration Setup

Configure the Harness MCP server in your IDE's `mcp_config.json` (e.g. Cursor or Windsurf settings):

```json
"harness": {
  "command": "sh",
  "args": [
    "-c",
    "harness mcp"
  ]
}
```

Or target the local/global binary path directly:

```json
"harness": {
  "command": "sh",
  "args": [
    "-c",
    "/Users/bao312/.local/bin/harness mcp"
  ]
}
```

### Available MCP Tools (18 Tools)

The MCP server exposes 18 powerful tools to AI Agents:

| Tool Name | Description |
| :--- | :--- |
| `harness_init` | Initialize the SQLite database (`harness.db`) in the workspace. |
| `harness_migrate` | Apply pending database schema migrations. |
| `harness_import_brownfield` | Seed DB from legacy markdown (`docs/TEST_MATRIX.md`, etc.). |
| `harness_intake` | Register feature risk classification (tiny, normal, high_risk). |
| `harness_story_add` | Add a Story to the Test Matrix. |
| `harness_story_update` | Update story status, evidence, and verification proofs. |
| `harness_decision_add` | Add an Architecture Decision Record (ADR). |
| `harness_decision_verify` | Run automated validation bash command for an ADR. |
| `harness_backlog_add` | Capture process pain / friction to the backlog. |
| `harness_backlog_close` | Resolve/close a backlog item. |
| `harness_trace` | Record a detailed execution trace of an agent's work. |
| `harness_query_stats` | View aggregated statistics of the workspace. |
| `harness_query_matrix` | View progress & validation matrix of all stories. |
| `harness_query_decisions` | Query registered architecture decisions. |
| `harness_query_intakes` | View 20 recent feature classification records. |
| `harness_query_traces` | View 20 recent agent execution trace records. |
| `harness_query_friction` | Search and filter recorded friction in traces. |
| `harness_query_sql` | Run direct SQL queries for advanced reporting. |

---

## Harness CLI (Terminal Interface)

For manual human use, you can also run local or global commands directly in the terminal:

```bash
# Global CLI
harness query stats

# Local fallback script
scripts/harness query stats
```

The CLI supports the exact same commands as the MCP tools. Run `harness help` or `harness query --help` for complete details.

The schema lives in `scripts/schema/` and is version-controlled. The database file (`harness.db`) is `.gitignore`d.

`scripts/harness import brownfield` seeds or refreshes the durable database from existing Harness v0 markdown in `docs/TEST_MATRIX.md`, `docs/decisions/`, and `docs/HARNESS_BACKLOG.md`. This keeps already-installed Harness repos on the Rust CLI path without losing their populated operating docs.

`HARNESS_RUST_CLI` can point `scripts/harness` at an alternate Rust CLI binary for local development or release verification.

## Installer

The upstream installer applies the Harness v0 operating files and folder
structure to a target project directory. It defaults to the current directory,
accepts a target path, and asks interactive users whether to `1. Merge`,
`2. Override`, or `3. Stop` when the target already contains `AGENTS.md`,
`docs/`, or `scripts/`.
Non-interactive installs stop on those protected paths unless `--merge` or
`--override` is provided. Use `--merge` as the safe update path for repositories
that already have Harness: it keeps existing files in place and creates only
missing Harness files. Add `--refresh-agent-shim` when an older install has the
full generated Harness guide in `AGENTS.md` and should move to the small stable
shim. Use `--override` only when replacing the protected Harness surface is
intentional.

```bash
curl -fsSL "https://raw.githubusercontent.com/baobao0303/harness/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --yes
```

```bash
curl -fsSL "https://raw.githubusercontent.com/baobao0303/harness/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --merge --yes
```

```bash
curl -fsSL "https://raw.githubusercontent.com/baobao0303/harness/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --merge --refresh-agent-shim --yes
```

`--refresh-agent-shim` backs up `AGENTS.md` before changing it. If the existing
file is recognized as the old Harness-generated operating guide, the installer
replaces it with the current shim. Otherwise it appends or replaces only the
marked `<!-- HARNESS:BEGIN -->` block so project-specific instructions remain
in place.

The installer must stay limited to harness files. Do not use it to scaffold
application source folders, package scripts, CI, tests, platform shells, or fake
validation commands. The installer script is not part of the installed project
payload.

By default the installer also downloads the prebuilt Rust Harness CLI for the
current platform into `scripts/bin/harness` and verifies its `.sha256`
checksum before making it executable. Set `HARNESS_CLI_BASE_URL` to point at an
alternate release artifact directory, such as a local `file:///.../dist`
directory created by `scripts/build-harness-cli-release.sh`.

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

The script writes `dist/harness-<platform>` and
`dist/harness-<platform>.sha256`. Supported labels are:

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
`harness-v*` to run the verification job, build all supported targets on
native hosted runners, and upload these release assets:

- `harness-macos-arm64`
- `harness-macos-arm64.sha256`
- `harness-macos-x64`
- `harness-macos-x64.sha256`
- `harness-linux-x64`
- `harness-linux-x64.sha256`
- `harness-linux-arm64`
- `harness-linux-arm64.sha256`
