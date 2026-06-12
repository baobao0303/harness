# Workflows Index

This directory contains reusable workflow guides for AI agents working in this repository.
Each workflow describes **when to use it**, **step-by-step instructions**, and **the expected output format**.

---

## Available Workflows

| Workflow | File | Use when |
| --- | --- | --- |
| **Quick Task** | `quick-task.md` | Tiny or normal lane — no story file needed |
| **Dev Story** | `dev-story.md` | Implementing a normal-lane story end to end |
| **Code Review** | `code-review.md` | Reviewing a PR or auditing agent work |
| **Feature Intake** | `feature-intake.md` | Converting a new feature request into story packets |
| **Bug Investigation** | `bug-investigation.md` | Investigating and fixing unexpected behavior |

---

## How to pick a workflow

```text
New request arrives
  → Is it a bug?         → bug-investigation.md
  → Is it a review?      → code-review.md
  → Is it a new feature? → feature-intake.md
  → Is it tiny/normal work with an existing story? → dev-story.md
  → Is it a small isolated change?                 → quick-task.md
```

---

## Relationship to Harness docs

Workflows are step-by-step execution guides.
They are not policies — the authoritative rules live in:

- `docs/HARNESS.md` — human-agent collaboration model and task loop
- `docs/FEATURE_INTAKE.md` — risk classification and lane rules
- `docs/ARCHITECTURE.md` — architecture boundaries and dependency rules
- `docs/TEST_MATRIX.md` — behavior-to-proof validation expectations
