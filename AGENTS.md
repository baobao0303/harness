# Agent Operating Guide

This repository is in Harness v0. There is no product implementation yet.

The current job of agents is to preserve and grow the collaboration harness
before writing application code. Do not scaffold application source folders,
platform shells, package scripts, CI, or tests unless a later story explicitly
moves the project into implementation.

## Source Of Truth

Read in this order:

1. `README.md` for project status.
2. `docs/HARNESS.md` for the human-agent operating model.
3. `docs/FEATURE_INTAKE.md` before turning any prompt into work.
4. The user-provided spec or prompt, when one exists.
5. `docs/product/` for current product contracts.
6. `docs/ARCHITECTURE.md` before proposing implementation shape.
7. `docs/stories/` for story packets and backlog.
8. `docs/TEST_MATRIX.md` for proof status.
9. `docs/decisions/` for why important choices were made.

## Durable Layer

Operational data lives in a SQLite database (`harness.db`) managed by
`scripts/harness`. Agents should use the CLI to record and query structured
data instead of editing markdown tables by hand.

Initialize the database if it does not exist:

```bash
scripts/harness init
```

Common commands:

```bash
scripts/harness intake  --type <type> --summary <text> --lane <lane>
scripts/harness story   add --id <id> --title <text> --lane <lane>
scripts/harness story   update --id <id> --status <status>
scripts/harness trace   --summary <text> --outcome <outcome>
scripts/harness query   matrix
scripts/harness query   backlog
scripts/harness query   stats
```

The database is `.gitignore`d because each project instance generates its own
operational data. The schema under `scripts/schema/` is version-controlled.

Policy docs (`HARNESS.md`, `FEATURE_INTAKE.md`, `ARCHITECTURE.md`) remain as
human-readable references. The database stores the records agents produce.

This harness does not ship with a project-specific `SPEC.md`. When the human
provides a spec for a new project, treat that spec as input material for the
first buildout. Derive product docs, story packets, architecture decisions, and
validation expectations from it. Product docs, stories, tests, and decisions
then become the living contract that agents should update as the system evolves.

## Task Loop

For every task:

1. Classify the request with `docs/FEATURE_INTAKE.md`.
2. Record the classification: `scripts/harness intake --type <type> --summary <text> --lane <lane>`.
3. Locate the affected product docs and story files.
4. Check proof status: `scripts/harness query matrix`.
5. Work only inside the selected lane: tiny, normal, or high-risk.
6. Before finishing, ask:
   - Did product truth change?
   - Did validation expectations change?
   - Did architecture rules change?
   - Did we discover a repeated failure pattern?
   - Did the next agent need a clearer instruction?
7. Record a trace: `scripts/harness trace --summary <text> --outcome <result>`.
8. If harness friction was found, either fix it directly or record it:
   `scripts/harness backlog add --title <text> --pain <text>`.

## Harness Change Policy

Agents may update directly:

- Story status and evidence via `scripts/harness story update`.
- Test matrix rows via `scripts/harness story add` and `scripts/harness story update`.
- Links from story packets to product docs.
- Validation notes and reports.
- Small clarifications tied to the current task.
- Intake records, traces, and backlog items via `scripts/harness`.

Agents should ask for human confirmation before:

- Changing architecture direction.
- Removing validation requirements.
- Changing the source-of-truth hierarchy.
- Changing risk classification rules.
- Replacing the feature workflow.

## Done Definition

A task is done only when:

- The requested change is completed or the blocker is documented.
- Relevant docs, stories, and test matrix entries remain current.
- Validation commands were run when they exist.
- A trace has been recorded: `scripts/harness trace --summary <text> --outcome <result>`.
- Missing harness capabilities were recorded: `scripts/harness backlog add`.
- The final response says what changed and what was not attempted.
