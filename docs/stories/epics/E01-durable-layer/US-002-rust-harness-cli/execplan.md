# Exec Plan

## Goal

Replace the growing Bash durable-layer CLI with a typed Rust implementation
while preserving the repository-local `scripts/harness` command contract.

## Scope

In scope:

- Add a Rust CLI implementation for the durable layer.
- Keep SQLite as the durable storage engine.
- Preserve existing command names and flags during the first migration.
- Ship the Rust CLI as a prebuilt binary downloaded by the installer.
- Keep `scripts/harness` as the stable command path.
- Add parity tests before retiring Bash behavior.

Out of scope:

- Application source scaffolding.
- A new product stack.
- A schema redesign unrelated to CLI migration.
- Requiring target projects to compile Rust locally.

## Risk Classification

Risk flags:

- Public contracts.
- Existing behavior.
- Weak proof.
- Platform behavior.

Hard gates:

- Changing the stable command path.
- Removing the Bash fallback before parity proof.
- Weakening durable-layer validation expectations.

## Work Phases

1. Design the Rust package layout and release artifact naming.
2. Add typed CLI parsing and domain/application boundaries for one vertical
   command slice.
3. Implement SQLite repository access with parameterized statements.
4. Add temp-database parity tests for the migrated slice.
5. Update the installer to download and verify a prebuilt binary.
6. Port remaining command groups incrementally.
7. Retire Bash internals only after all command groups have parity evidence.
8. Update docs, story evidence, and durable records.

## Stop Conditions

Pause for human confirmation if:

- The `scripts/harness` command path would need to change.
- Prebuilt binary distribution requires a new hosting provider.
- Checksum or release verification cannot be automated.
- Current SQLite schema compatibility cannot be preserved.
- Installer behavior would become unavailable offline without an accepted
  fallback.
