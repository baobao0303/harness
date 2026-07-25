# Agent Instructions

Add project-specific agent instructions here.

<!-- HARNESS:BEGIN -->

## Harness

This repo uses Harness. Before work, read:

- `README.md`
- `docs/HARNESS.md`
- `docs/FEATURE_INTAKE.md`
- `docs/ARCHITECTURE.md`
- `docs/workflows/README.md` — pick the right workflow before starting
- `scripts/bin/harness-cli query matrix` on macOS/Linux, or
  `.\scripts\bin\harness-cli.exe query matrix` on Windows

Use the Rust Harness CLI at `scripts/bin/harness-cli` on macOS/Linux or
`scripts/bin/harness-cli.exe` on Windows as the main operational tool.

## Environment Configuration
- Never commit `.env` or private keys to Git.
- Reference `.env.example` for required environment variables (`HARNESS_SKILL_SERVER`, `HARNESS_MODEL`, `HARNESS_DB`).
- Copy `.env.example` to `.env` when setting up local environments.

<!-- HARNESS:END -->
