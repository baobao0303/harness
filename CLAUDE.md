# Project Rules

<!-- HARNESS:BEGIN -->
## Harness

Claude Code loads this file into every session, but it does not auto-load
`AGENTS.md`. The bare `@` lines below import the always-required harness
context (the "Must in all lanes" set from `docs/CONTEXT_RULES.md`) at
context-load time. Never wrap them in backticks; that disables the import.

@AGENTS.md

@docs/FEATURE_INTAKE.md

@docs/TASK_FLOW.md

Also run `scripts/harness query matrix` before starting work.

Lane-dependent context (`README.md`, `docs/HARNESS.md`, `docs/ARCHITECTURE.md`,
`docs/CONTEXT_RULES.md`, product docs, stories, decisions) is intentionally not
imported — read it per lane, as `docs/CONTEXT_RULES.md` prescribes.

For specific phases of task execution, adopt the appropriate persona:
- PM phase ➡️ `.agents/personas/pm.md`
- BA phase ➡️ `.agents/personas/ba.md`
- FE phase ➡️ `.agents/personas/fe.md`
- BE phase ➡️ `.agents/personas/be.md`
- QA phase ➡️ `.agents/personas/qa.md`
<!-- HARNESS:END -->
