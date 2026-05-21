# Agent Instructions

Add project-specific agent instructions here.

<!-- HARNESS:BEGIN -->

## Harness

This repo uses **Harness** — a durable, SQLite-backed operating framework for human-agent pair programming.

Before work, read:

- `README.md`
- `docs/HARNESS.md`
- `docs/FEATURE_INTAKE.md`
- `docs/ARCHITECTURE.md`
- `.agents/rules/harness.md`

Use the Rust Harness CLI as the main operational tool. Run it through the stable repo-local entrypoint `scripts/harness`, which uses the prebuilt Rust binary at `scripts/bin/harness-cli`.

<!-- AGENT-SHIMS:BEGIN -->

## Claude Code

Before working in this repository:

1. Read `.agents/rules/harness.md` for operating rules
2. Run `scripts/harness query matrix` to see current validation status
3. Run `scripts/harness query stats` for project overview
4. Follow the mandatory task loop for every request

## Cursor

Before working in this repository:

1. Read `.agents/rules/harness.md` for operating rules
2. Reference `docs/FEATURE_INTAKE.md` for work classification
3. Use `scripts/harness query` commands for status checks
4. Follow the mandatory task loop for every request

## Windsurf

Before working in this repository:

1. Read `.agents/rules/harness.md` for operating rules
2. Reference `docs/FEATURE_INTAKE.md` for work classification
3. Use `scripts/harness query` commands for status checks
4. Follow the mandatory task loop for every request

## GitHub Copilot

Before suggesting code changes:

1. Read `.agents/rules/harness.md` for operating rules
2. Reference story packets in `docs/stories/` for implementation guidance
3. Use `scripts/harness query` commands for context
4. Always record traces upon task completion

## Antigravity

Before working in this repository:

1. Read `.agents/rules/harness.md` for operating rules
2. Run `scripts/harness query matrix` for validation status
3. Follow the mandatory task loop for every request

## Codex

Before working in this repository:

1. Read `.agents/rules/harness.md` for operating rules
2. Reference `docs/FEATURE_INTAKE.md` for work classification
3. Use `scripts/harness query` commands for context
4. Follow the mandatory task loop for every request

## All Other Agents

This repository uses Harness. Before working:

1. Read `.agents/rules/harness.md` — this is the authoritative operating guide
2. Read `docs/HARNESS.md` for the human-agent collaboration model
3. Read `docs/FEATURE_INTAKE.md` for work classification
4. Use `scripts/harness` CLI for all task tracking

Treat harness docs as authoritative. Always record traces upon task completion. Report friction to backlog.

<!-- AGENT-SHIMS:END -->
<!-- HARNESS:END -->