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

## Installer

The upstream installer applies the Harness v0 operating files and folder
structure to a target project directory. It defaults to the current directory,
accepts a target path, and asks interactive users whether to `1. Merge`,
`2. Override`, or `3. Stop` when the target already contains `AGENTS.md`,
`docs/`, or `scripts/`.
Non-interactive installs stop on those protected paths unless `--merge` or
`--override` is provided.

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
