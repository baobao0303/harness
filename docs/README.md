# Documentation Map

This directory holds the project harness and any product contract derived from a
future user-provided spec.

## Main Files

- `HARNESS.md`: how humans and agents collaborate.
- `FEATURE_INTAKE.md`: how prompts become tiny, normal, or high-risk work.
- `ARCHITECTURE.md`: architecture discovery and boundary rules.
- `TEST_MATRIX.md`: legacy proof map; current proof status is queried with
  `scripts/harness query matrix`.
- `HARNESS_BACKLOG.md`: legacy improvement list; current improvement records
  are stored with `scripts/harness backlog`.
- `GLOSSARY.md`: shared terms.

## Folders

- `product/`: current product truth, empty until a spec is derived.
- `stories/`: feature packets and backlog.
- `decisions/`: durable decisions and tradeoffs.
- `demo/`: concrete walkthroughs that show how the harness transforms input
  into agent-ready work.
- `templates/`: reusable spec-intake, story, plan, decision, and validation
  formats.

## Agent & Workspace Integration (`.agents/`)

The documentation files in `docs/` are dynamically driven and maintained by the specialized roles and step-file workflows inside the `.agents/` cognitive core:

* **Product Manager (PM)** (`.agents/agents/product/`) ➔ Owns `docs/stories/` and `docs/templates/` for story decomposition and spec intake.
* **Architect** (`.agents/agents/engineering/`) ➔ Owns `docs/decisions/` (ADRs) and `docs/product/` for structural contracts.
* **QA & Test Architect** (`.agents/agents/testing/`) ➔ Owns the validation matrix and test coverage mapping (`docs/TEST_MATRIX.md`).
* **Scrum Master** (`.agents/agents/project-management/`) ➔ Tracks active agile execution loops and `sprint-status.yaml`.
* **Master Dev Loop** (`.agents/workflow/harness-loop.md`) ➔ Orchestrates the 6-step sequential development loop (Intake ➔ Design ➔ Stories ➔ Dev ➔ Review ➔ Retro) that updates all matrix and database records.

## Current State

Harness v0 exists before implementation. These docs define how the project will
grow; they do not imply that app code, tests, CI, or deployment automation exist
yet.
