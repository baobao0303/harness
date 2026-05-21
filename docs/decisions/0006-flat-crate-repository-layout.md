# 0006 Flat Crate Repository Layout

Date: 2026-05-23

## Status

Accepted

## Context

The repository originally used a single-member Cargo workspace structure with Rust source files nested inside `crates/harness-cli`. This structure had several drawbacks for a single-binary development-assisting harness CLI:

- Added unnecessary workspace configuration overhead.
- Increased learning curve and navigation friction due to nested folders (`crates/harness-cli/src`).
- Made local cargo test and development commands more complex (requiring workspace-specific flags or directory changes).

To simplify development, packaging, and integration, the user requested that we refactor the folder structure to depart from the nested workspace.

## Decision

Reorganize the repository to a flat crate structure (Option 1).

Specifically:
- Move all Rust source files from `crates/harness-cli/src/` directly to `src/` at the repository root level.
- Move the package configuration to a root package manifest.
- Delete the obsolete nested `crates/` directory entirely.
- Update release workflows and local scripts (`scripts/harness`) to build and execute the root package.

## Alternatives Considered

1. **Feature-based (component) workspace**: Organizes crates by product capabilities (e.g. `crates/harness-intake`). Rejected as over-engineered for a simple single-binary CLI.
2. **Layer-based clean architecture workspace**: Organizes domain, application, infrastructure, and CLI interface into separate compiler-enforced crates. Rejected due to high boilerplate overhead.
3. **Keep the current workspace layout**: Rejected due to the navigation friction and workspace complexity.

## Consequences

Positive:

- Standard, extremely clean single-crate structure that matches Rust CLI conventions.
- Reduced directory nesting and cognitive load for humans and agents alike.
- Simpler development, testing, and compilation commands.

Tradeoffs:

- Prevents having distinct sub-crates with their own isolated dependencies in the workspace if we want to expand to library/tool divisions later.

## Follow-Up

- Compile the new root package directly during releases.
- Rename the package and binary configuration to fully brand the executable as `harness`.
