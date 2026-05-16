# Harness

The project goal is to provide a reusable operating harness that lets humans and
agents turn a future product spec into safe, validated work.

The app is what users touch. The harness is what agents touch.

## Mental Model

```text
------------------+
| Human intent    |
+------------------+
         |
         v
+------------------+
| Feature intake   |
+------------------+
         |
         v
+------------------+
| Story packet     |
+------------------+
         |
         v
+------------------+
| Agent work loop  |
+------------------+
         |
         v
+------------------+
| Product delta    |
+------------------+
         |
         v
+------------------+
| Validation proof |
+------------------+
         |
         v
+------------------+
| Harness delta    |
+------------------+
         |
         v
+------------------+
| Next intent      |
+------------------+
```

Every task has two possible outputs:

1. Product delta: app code, tests, API shape, data model, or product docs.
2. Harness delta: docs, templates, validation expectations, backlog items, or
   decision records that make the next task easier.

## Harness v0 Scope

Harness v0 includes:

- Agent entrypoint.
- Empty product documentation structure.
- Feature intake and risk lanes.
- Story templates.
- Decision log template.
- Validation report template.
- Test matrix placeholder.
- Harness growth backlog.
- Durable layer: SQLite database and CLI for operational records.

Harness v0 deliberately excludes:

- A project-specific `SPEC.md`.
- Pre-sliced product domains.
- A locked application stack.
- App source scaffolding.
- Package scripts.
- Test runner config.
- CI workflows.

Those should arrive only when a selected story needs them.

## Durable Layer

Policy documents describe how to work. The durable layer stores what happened.

Operational data — intake classifications, story status, decision outcomes,
backlog items, and execution traces — lives in a SQLite database (`harness.db`)
managed by `scripts/harness`. The database is local to each project instance
and `.gitignore`d. The schema is version-controlled under `scripts/schema/`.

This separation keeps policy docs stable and human-readable while giving agents
a structured, queryable record of operational state. It also prepares the
harness for future observability and automated evolution without adding more
markdown files.

See `AGENTS.md > Durable Layer` for CLI usage.

## Source Hierarchy

```text
User-provided spec or prompt
  input material for first buildout or future changes

docs/product/*
  current product contract derived from accepted input

docs/stories/*
  story-sized work packets and historical evidence

docs/TEST_MATRIX.md
  behavior-to-proof control panel

docs/decisions/*
  why the contract changed
```

Before implementation, product docs describe intent. After implementation,
product docs plus executable tests become the living contract.

## Spec Lifecycle

Harness v0 starts without a tracked project spec. When the human provides a
specification, treat it as input material, not as a permanent operating manual.
Use it to populate product docs, story packets, architecture decisions, and
validation expectations during the first buildout.

After the specification has been decomposed, do not keep extending it as the
living product plan. Ongoing work should update the smaller product docs,
stories, test matrix, and decision records.

Ongoing work should enter the harness as one of these input types:

- New spec: a project specification that needs to become product docs and
  initial story candidates.
- Spec slice: a selected behavior from the provided spec.
- Change request: a bounded behavior change, bug fix, or product refinement.
- New initiative: a larger product area that needs multiple stories.
- Maintenance request: dependency, architecture, performance, security, or
  operational work.
- Harness improvement: a process, template, proof, or agent-instruction change.

The spec-to-work loop is:

```text
human intent or supplied spec
  -> classify input type
  -> update or create product contract
  -> create story packet or initiative notes when needed
  -> define validation proof
  -> implement or document the blocker
  -> update product docs, stories, test matrix, and decisions
  -> capture harness friction
```

Large product areas should use scoped initiative notes instead of a second
monolithic specification. An initiative should explain the goal, affected
product docs, candidate stories, validation shape, open decisions, and exit
criteria. If initiative work becomes a repeated pattern, add a template or
proposal to `docs/HARNESS_BACKLOG.md`.

## Growth Rule

The harness grows from friction.

When an agent is confused, repeats manual reasoning, needs a new validation
command, discovers a missing rule, or sees a recurring failure pattern, it must
either improve the harness directly or record the friction:

```bash
scripts/harness backlog add --title "<short name>" --pain "<what was hard>"
```

The `harness_friction` field on traces also captures per-task friction so
patterns can be queried later:

```bash
scripts/harness query friction
```

## Future Validation Ladder

No validation scripts exist yet. When implementation begins, the expected ladder
is:

```text
validate:quick
  format, lint, typecheck, unit tests, architecture check

test:integration
  backend, database, provider, or service checks as the stack requires

test:e2e
  user-visible end-to-end flows

test:platform
  shell, mobile, desktop, or deployment smoke checks as the stack requires

test:release
  full suite, log checks, and performance smoke
```

Agents must not claim these commands pass until they exist and have been run.
