# harness-experimental

## Current State

This repository is in Harness v0.

There is no application implementation and no baked-in product specification
yet. The current work is the reusable project harness: the file structure,
agent operating model, feature intake process, story templates, and validation
expectations that help humans and agents turn a future user-provided spec into
implementation work.

## Product Sources

No product contract is currently defined.

When a user provides a project specification, add or reference it as the input
spec for the first buildout, then derive smaller living artifacts from it:

- `docs/product/`: current product contract files, created from the spec.
- `docs/stories/`: story packets and backlog created from selected work.
- `docs/TEST_MATRIX.md`: behavior-to-proof control panel.
- `docs/decisions/`: durable decisions and tradeoffs.

Do not keep a project-specific spec or product breakdown in this harness until
a real project supplies one.

## Harness Sources

- `AGENTS.md`: agent entrypoint and operating rules.
- `docs/HARNESS.md`: human-agent collaboration model.
- `docs/FEATURE_INTAKE.md`: tiny, normal, and high-risk work classification.
- `docs/ARCHITECTURE.md`: generic architecture discovery and boundary rules.
- `docs/HARNESS_BACKLOG.md`: proposed harness improvements.
- `docs/templates/`: reusable spec-intake, story, decision, and validation
  templates.

## Repository Structure

```text
project/
  AGENTS.md
  README.md
  docs/
    HARNESS.md
    FEATURE_INTAKE.md
    ARCHITECTURE.md
    TEST_MATRIX.md
    HARNESS_BACKLOG.md
    product/
    stories/
    decisions/
    templates/
  scripts/
    README.md
```

## Working Rule

Implementation prompts do not go straight to code. They first pass through
feature intake, become story-sized work when needed, and then carry both
product validation and harness maintenance expectations.
