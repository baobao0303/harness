# Overview

## Current Behavior

Harness operational records live in a local SQLite database managed by
`scripts/harness`. The Rust CLI is the main tool for operational records in
installed projects.

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

The Rust CLI preserves the existing database schema and command semantics while
making the implementation typed and releaseable.

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
- Do not require target projects to install Rust.
