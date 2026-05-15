# harness-experimental

Harness v0 for agent-driven software development.

This is not an app template. It is a repository-level operating harness for
turning human intent or a product spec into agent-ready work: product
contracts, story packets, validation expectations, architecture decisions, and
eventually implementation.

The app is what users touch. The harness is what agents touch.

## Why This Exists

Coding agents are becoming useful enough to participate in real software work,
but the model alone is not the whole system. A repository also needs clear
instructions, shared product truth, validation loops, internal tools, and
decision records so an agent can understand what matters before it changes
code.

Harness Engineering is the practice of designing that operating environment.
The goal is not simply to make AI write code faster. The goal is to make
AI-assisted software development more reliable, inspectable, and easier for
humans to steer.

OpenAI describes this shift as an agent-first world where humans steer and
agents execute:

https://openai.com/index/harness-engineering/

## Current State

This repository is in Harness v0.

There is no application implementation and no baked-in product specification
yet. The current work is the reusable project harness: the file structure,
agent operating model, feature intake process, story templates, and validation
expectations that help humans and agents turn a future user-provided spec into
implementation work.

## What Counts As A Harness

A repository starts to have a harness when it helps an agent answer practical
engineering questions without relying only on chat history:

- What should I read first?
- What type of work is this?
- Which product contract does it affect?
- How risky is the change?
- What proof will show the work is done?
- What decision or lesson should future agents inherit?

In this repo, those answers live in `AGENTS.md`, `docs/HARNESS.md`,
`docs/FEATURE_INTAKE.md`, `docs/ARCHITECTURE.md`, `docs/TEST_MATRIX.md`,
`docs/stories/`, `docs/decisions/`, and `docs/templates/`.

## Try The Flow

The fastest way to understand the harness is to inspect a tiny example:

- `docs/demo/README.md`: shows how a simple product idea becomes product docs,
  stories, validation expectations, and decisions before implementation starts.

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
    demo/
    templates/
  scripts/
    README.md
```

## Working Rule

Implementation prompts do not go straight to code. They first pass through
feature intake, become story-sized work when needed, and then carry both
product validation and harness maintenance expectations.

## Install Harness Into A Project

From a target project directory, run:

```bash
curl -fsSL "https://raw.githubusercontent.com/hoangnb24/harness-experimental/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --yes
```

If the target already has `AGENTS.md`, `docs/`, or `scripts/`, choose one:

```bash
# Keep existing files and add only missing Harness files
curl -fsSL "https://raw.githubusercontent.com/hoangnb24/harness-experimental/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --merge --yes

# Back up and replace AGENTS.md, docs/, and scripts/
curl -fsSL "https://raw.githubusercontent.com/hoangnb24/harness-experimental/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --override --yes
```

Or install into a specific path:

```bash
curl -fsSL "https://raw.githubusercontent.com/hoangnb24/harness-experimental/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --directory /path/to/project --yes
```

If the target already contains `AGENTS.md`, `docs/`, or `scripts/`, interactive
installs ask whether to `1. Merge`, `2. Override`, or `3. Stop`. Non-interactive
installs using `--yes` stop before writing unless `--merge` or `--override` is
provided. Use `--dry-run` to preview changes. The installer itself and this
repository's installer story are not copied into the target project.
