# 0007 Harness Manifest and Package Renaming

Date: 2026-05-23

## Status

Superseded by [ADR 0009](0009-reverting-to-standard-cargo-manifests.md)

## Context

After refactoring to the flat layout in Decision 0006, the package and binary still retained their old names (`harness-cli`), and the configuration files used standard Cargo names (`Cargo.toml` and `Cargo.lock`). The user explicitly requested to:
- Rename the package and binary to `harness` to align with the name of the framework.
- Rename the Cargo manifest files directly to `harness.toml` and `harness.lock` for deep branding.

Cargo by default expects the manifest to be named `Cargo.toml`. To support custom manifest names, Cargo requires explicitly passing `--manifest-path <path>`.

## Decision

1. Rename the package and compiled binary from `harness-cli` to `harness`.
2. Rename `Cargo.toml` to `harness.toml` and `Cargo.lock` to `harness.lock` at the root directory.
3. Modify the release script `scripts/build-harness-cli-release.sh` and CI/CD workflow `.github/workflows/harness-cli-release.yml` to compile using the `--manifest-path harness.toml` flag.
4. Keep the entrypoint `scripts/harness` fully compatible by searching for the compiled `harness` binary as well as any installed legacy `harness-cli` binaries.

## Alternatives Considered

1. **Rename only the package but keep standard Cargo.toml/Cargo.lock filenames**: Rejected because the user insisted on a fully branded manifest called `harness.toml`.

## Consequences

Positive:

- Full alignment of framework and manifest branding (`harness`).
- Manifest files now explicitly document the tool's name in their filenames.

Tradeoffs:

- Standard cargo commands run without arguments (e.g. `cargo build`) will fail to find `Cargo.toml` by default. Developers must run them with the `--manifest-path harness.toml` option (which is automated in our stable repo-local entrypoint `scripts/harness` and build scripts).

## Follow-Up

- Maintain robust distribution logic to package `harness-<platform>` prebuilts.
