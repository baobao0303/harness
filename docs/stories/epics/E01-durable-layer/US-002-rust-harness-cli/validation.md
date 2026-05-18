# Validation

## Proof Strategy

Prove parity before replacement. Each migrated command should be tested against
a temporary SQLite database and compared to the current command contract.

The Bash CLI can remain as a reference implementation until the Rust CLI proves
the same durable-layer behavior.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Parse command flags into typed values; reject invalid lanes, statuses, booleans, and missing required flags. |
| Integration | Create a temp database, apply schema, run migrated use cases, and verify rows with SQLite queries. |
| E2E | Install Harness into a temp target, download or locate the prebuilt CLI, run `scripts/harness init`, `intake`, `query intakes`, and `trace`. |
| Platform | Verify supported macOS and Linux binary selection, checksum validation, and clear unsupported-platform errors. |
| Performance | Query commands should remain fast on small local databases; no benchmark gate until larger trace volumes exist. |
| Logs/Audit | Trace writes remain available through `scripts/harness trace` and `scripts/harness query traces`. |

## Fixtures

- Temporary target project with no existing Harness files.
- Temporary target project with existing Harness files for merge behavior.
- Temporary SQLite database seeded with `scripts/schema/001-init.sql`.
- Release-artifact fixture or local file server for installer download tests.

## Commands

```bash
cargo fmt --check
cargo test --workspace
bash -n scripts/harness scripts/install-harness.sh
scripts/build-harness-cli-release.sh
scripts/harness query stats
tmpdir=$(mktemp -d)
HARNESS_DB="$tmpdir/harness.db" scripts/harness init
HARNESS_DB="$tmpdir/harness.db" scripts/harness migrate
HARNESS_DB="$tmpdir/harness.db" scripts/harness intake --type "Harness improvement" --summary "Rust delegated intake smoke" --lane high-risk --flags "public contracts" --docs "docs/decisions/0005-prebuilt-rust-harness-cli" --story US-002
HARNESS_DB="$tmpdir/harness.db" scripts/harness story add --id US-SMOKE --title "Rust parity smoke story" --lane high-risk --contract docs/decisions/0005-prebuilt-rust-harness-cli
HARNESS_DB="$tmpdir/harness.db" scripts/harness story update --id US-SMOKE --status implemented --evidence "rust smoke" --unit 1 --integration 1
HARNESS_DB="$tmpdir/harness.db" scripts/harness decision add --id 9999-smoke --title "Smoke Decision" --status accepted --doc docs/decisions/0005-prebuilt-rust-harness-cli --verify "true"
HARNESS_DB="$tmpdir/harness.db" scripts/harness decision verify 9999-smoke
HARNESS_DB="$tmpdir/harness.db" scripts/harness backlog add --title "Smoke backlog" --pain "Need proof" --risk normal --predicted "Proof exists"
HARNESS_DB="$tmpdir/harness.db" scripts/harness backlog close --id 1 --status implemented --outcome "closed"
HARNESS_DB="$tmpdir/harness.db" scripts/harness trace --summary "Smoke trace" --intake 1 --story US-SMOKE --agent Codex --outcome completed --actions "one,two" --friction "none"
HARNESS_DB="$tmpdir/harness.db" scripts/harness query matrix
HARNESS_DB="$tmpdir/harness.db" scripts/harness query backlog
HARNESS_DB="$tmpdir/harness.db" scripts/harness query decisions
HARNESS_DB="$tmpdir/harness.db" scripts/harness query intakes
HARNESS_DB="$tmpdir/harness.db" scripts/harness query traces
HARNESS_DB="$tmpdir/harness.db" scripts/harness query friction
HARNESS_DB="$tmpdir/harness.db" scripts/harness query stats
HARNESS_DB="$tmpdir/harness.db" scripts/harness query sql "SELECT COUNT(*) AS story_count FROM story;"
rm -rf "$tmpdir"
target=$(mktemp -d)
scripts/install-harness.sh --directory "$target" --yes
"$target/scripts/harness" init
"$target/scripts/harness" intake --type "Harness improvement" --summary "installed binary smoke" --lane tiny
"$target/scripts/harness" query stats
test -x "$target/scripts/bin/harness-cli"
rm -rf "$target"
```

## Acceptance Evidence

- `cargo fmt --check`: passed.
- `cargo test --workspace`: passed, 8 tests.
- `bash -n scripts/harness scripts/install-harness.sh`: passed.
- `scripts/build-harness-cli-release.sh`: passed and wrote
  `dist/harness-cli-macos-arm64` plus checksum.
- Temporary database smoke passed through the Rust delegated command paths:
  `init`, `migrate`, `intake`, `story add`, `story update`, `decision add`,
  `decision verify`, `backlog add`, `backlog close`, `trace`, `query matrix`,
  `query backlog`, `query decisions`, `query intakes`, `query traces`, `query
  friction`, `query stats`, and `query sql`.
- Installer E2E passed using the local `dist` release source. It downloaded
  `scripts/bin/harness-cli`, verified the checksum, ran `scripts/harness init`,
  recorded an intake, and queried stats without relying on a local Cargo build
  inside the target project.
- Checksum failure test passed: a corrupt `.sha256` file caused the installer
  to stop before accepting the binary.
- `--skip-cli-download` test passed: installer skipped the binary and
  `scripts/harness init` still worked through Bash fallback.
- Existing `.gitignore` merge test passed: custom rules were preserved while
  `harness.db` and `scripts/bin/harness-cli` ignore rules were appended.

Remaining evidence needed before story completion:

- Publish release artifacts for all supported targets.
- `import brownfield` parity or an explicit decision to leave it as Bash-only.
