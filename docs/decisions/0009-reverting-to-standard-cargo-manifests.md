# 0009 Reverting to Standard Cargo Manifests

Date: 2026-05-24

## Status

Accepted

## Context

In Architecture Decision 0007, we renamed `Cargo.toml` and `Cargo.lock` to `harness.toml` and `harness.lock` at the root directory to implement deep branding.

However, this choice introduced significant development friction:
1. **Cargo 1.95+ Restrictions:** Cargo version 1.95 and above explicitly restricts manifest file names, forcing them to be exactly `Cargo.toml`. Commands like `cargo test --manifest-path harness.toml` fail immediately.
2. **IDE & Tooling Breakdown:** The Rust Analyzer extension (used in VS Code, Cursor, and other modern IDEs) is unable to analyze the codebase or provide auto-completion because it depends on finding a `Cargo.toml` file at the root of the workspace.
3. **Friction in Scripts:** Build scripts had to use temporary copy-and-delete workarounds (`cp harness.toml Cargo.toml`) to successfully invoke Cargo commands.

We need a way to maintain our deep binary branding without violating Cargo's standard conventions.

## Decision

1. **Revert Manifest Filenames:** Rename the files from `harness.toml` and `harness.lock` back to standard `Cargo.toml` and `Cargo.lock` at the root of the repository.
2. **Keep Binary Branding:** Continue to brand the binary as `harness` by setting the package name as `harness` in `Cargo.toml`. Cargo automatically produces a binary named `harness` during compilation.
3. **Simplify Build Scripts:** Remove all temporary copies and cleanups of manifest files in `scripts/build-harness-cli-release.sh`.
4. **Supersede ADR 0007:** Mark Decision 0007 as `superseded` in our catalog.

## Alternatives Considered

1. **Keep Custom Filenames and Symlink permanently:** Rejected because symlinks are not fully cross-platform (especially on Windows) and still trigger warning/error flags in Cargo 1.95+.
2. **Maintain standard filenames but rename binary post-compilation:** Rejected because setting `name = "harness"` under `[package]` in `Cargo.toml` is a standard, native Cargo mechanism that requires no post-processing.

## Consequences

Positive:

- 100% compatibility with standard Cargo tooling (e.g. `cargo build`, `cargo test`, `cargo check` work natively).
- Full restore of **Rust Analyzer** and IDE auto-completions, removing huge developer friction.
- Simpler, cleaner build and CI/CD scripts without file-copying hacks.

Tradeoffs:

- The configuration file is named `Cargo.toml` rather than `harness.toml`, but this conforms to industry standards and avoids compiler errors.

## Follow-Up

- Update `docs/decisions/README.md` to reflect that ADR 0007 is superseded by ADR 0009.
- Import brownfield decisions using `harness import brownfield` once renamed files are finalized.
