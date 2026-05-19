# Design

## Domain Model

Core harness concepts should become typed Rust values:

- `InputType`: new spec, spec slice, change request, new initiative,
  maintenance, harness improvement.
- `RiskLane`: tiny, normal, high risk.
- `StoryStatus`: planned, in progress, implemented, changed, retired.
- `DecisionStatus`: proposed, accepted, superseded, rejected.
- `TraceOutcome`: completed, blocked, partial, failed.

String normalization should live at the interface boundary before values enter
application use cases.

## Application Flow

Use cases should mirror the existing command groups:

- `init`: create or inspect the durable database.
- `migrate`: apply pending schema migrations.
- `import brownfield`: seed or refresh the durable database from existing
  Harness v0 markdown, including the test matrix, decision records, and harness
  backlog items.
- `intake`: record classified work.
- `story`: add or update story proof state.
- `decision`: add decisions and run verification commands.
- `backlog`: add or close harness improvement proposals.
- `trace`: record execution traces.
- `query`: render matrix, backlog, decisions, intakes, traces, friction, stats,
  and SQL query output.

The first implementation slice should port a narrow vertical path, such as
`intake` plus `query intakes`, before broad command migration.

## Interface Contract

The public command contract remains:

```bash
scripts/harness <command> [flags]
```

Rust command parsing should use typed subcommands. Help output can improve, but
existing command names, flag names, accepted status tokens, and exit behavior
should remain compatible unless a separate decision accepts a breaking change.

## Data Model

The Rust CLI must read and write the current SQLite schema under
`scripts/schema/`. It should preserve `HARNESS_DB` as the database override.

SQLite access should use parameterized statements instead of string-built SQL.
Migration behavior must keep schema versions idempotent.

## UI / Platform Impact

The installer must detect supported platforms and download the matching
prebuilt binary. The installed repository should still expose `scripts/harness`
as the command agents run.

Supported platform targets, binary naming, cache path, and checksum format
should be selected during implementation.

## Observability

The CLI should keep writing trace records through the durable layer. Installer
download failures should produce actionable terminal errors that explain the
platform, URL, and fallback path.

## Alternatives Considered

1. Keep Bash only. Simpler distribution, weaker testability.
2. Build Rust locally in every target repo. Better source transparency, much
   worse install friction.
3. Require a global binary. Cleaner release model, weaker repo-local harness
   contract.
4. Prebuilt binary downloaded by installer. Chosen because it preserves
   repo-local usage while allowing typed internals.
