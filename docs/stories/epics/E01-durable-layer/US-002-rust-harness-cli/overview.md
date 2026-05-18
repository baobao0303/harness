# Overview

## Current Behavior

Harness operational records live in a local SQLite database managed by
`scripts/harness`. The CLI is implemented as a Bash script and is copied into
target projects by `scripts/install-harness.sh`.

The current command path is:

```bash
scripts/harness <command>
```

## Target Behavior

Harness ships a Rust implementation of the durable-layer CLI as a prebuilt
binary downloaded by the installer. The repository-local command path remains
stable:

```bash
scripts/harness <command>
```

The Rust CLI preserves the existing database schema and command semantics before
the Bash implementation is retired.

## Affected Users

- Humans installing Harness into a project.
- Coding agents following `AGENTS.md` and recording intake, story, decision,
  backlog, and trace data.
- Maintainers releasing Harness CLI updates.

## Affected Product Docs

- `AGENTS.md`
- `README.md`
- `docs/HARNESS.md`
- `docs/ARCHITECTURE.md`
- `scripts/README.md`
- `docs/decisions/0004-sqlite-durable-layer.md`
- `docs/decisions/0005-prebuilt-rust-harness-cli.md`

## Non-Goals

- Do not scaffold application code.
- Do not change the SQLite durable-layer schema unless a separate migration
  story requires it.
- Do not remove the current Bash CLI until Rust command parity is proven.
- Do not require target projects to install Rust.
