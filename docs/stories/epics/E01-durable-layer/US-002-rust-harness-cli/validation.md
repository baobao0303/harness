# Validation

## Proof Strategy

Prove the Rust CLI command contract against a temporary SQLite database. Each
command should be tested through the stable `scripts/harness` entrypoint and
verified against durable records.

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
bash -n scripts/install-harness.sh
bash -n scripts/harness
bash -n scripts/build-harness-cli-release.sh
scripts/build-harness-cli-release.sh
scripts/harness query stats
tmpdir=$(mktemp -d)
HARNESS_DB="$tmpdir/harness.db" scripts/harness init
HARNESS_DB="$tmpdir/harness.db" scripts/harness migrate
HARNESS_DB="$tmpdir/harness.db" scripts/harness import brownfield
HARNESS_DB="$tmpdir/harness.db" scripts/harness intake --type "Harness improvement" --summary "Rust CLI intake smoke" --lane high-risk --flags "public contracts" --docs "docs/decisions/0005-prebuilt-rust-harness-cli" --story US-002
HARNESS_DB="$tmpdir/harness.db" scripts/harness story add --id US-SMOKE --title "Rust CLI smoke story" --lane high-risk --contract docs/decisions/0005-prebuilt-rust-harness-cli
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
- `cargo test --workspace`: passed, 10 tests, including regression coverage
  for repo-root decision verification and SQL `NULL` intake list storage.
- `bash -n scripts/install-harness.sh`: passed.
- `bash -n scripts/harness`: passed.
- `bash -n scripts/build-harness-cli-release.sh`: passed.
- `.github/workflows/harness-cli-release.yml`: added to verify the workspace,
  build the four supported CLI release targets on hosted native runners, and
  publish `harness-cli-<platform>` plus `.sha256` assets to the GitHub Release
  for `v*` or `harness-cli-v*` tags.
- `scripts/build-harness-cli-release.sh`: passed and wrote
  `dist/harness-cli-macos-arm64` plus checksum.
- Temporary database smoke passed through the Rust delegated command paths:
  `init`, `migrate`, `import brownfield`, `intake`, `story add`, `story
  update`, `decision add`, `decision verify`, `backlog add`, `backlog close`,
  `trace`, `query matrix`, `query backlog`, `query decisions`, `query
  intakes`, `query traces`, `query friction`, `query stats`, and `query sql`.
- Brownfield import fixture test passed: existing Harness v0 markdown seeded a
  story from `docs/TEST_MATRIX.md`, a decision from `docs/decisions/`, and
  multiple backlog items from `docs/HARNESS_BACKLOG.md`; rerunning the importer
  did not duplicate backlog rows.
- Installer E2E passed using the local `dist` release source. It downloaded
  `scripts/bin/harness-cli`, verified the checksum, ran `scripts/harness init`,
  recorded an intake, and queried stats without relying on a local Cargo build
  inside the target project.
- Checksum failure test passed: a corrupt `.sha256` file caused the installer
  to stop before accepting the binary.
- Existing `.gitignore` merge test passed: custom rules were preserved while
  `harness.db` and `scripts/bin/harness-cli` ignore rules were appended.
- Real tag release passed: `harness-cli-v0.1.2` completed the `Harness CLI
  Release` workflow, including verify, `macos-arm64`, `macos-x64`,
  `linux-x64`, `linux-arm64`, and publish jobs.
- GitHub Release verification passed: `harness-cli-v0.1.2` is the latest
  release and contains exactly eight expected assets:
  `harness-cli-macos-arm64`, `harness-cli-macos-arm64.sha256`,
  `harness-cli-macos-x64`, `harness-cli-macos-x64.sha256`,
  `harness-cli-linux-x64`, `harness-cli-linux-x64.sha256`,
  `harness-cli-linux-arm64`, and `harness-cli-linux-arm64.sha256`.
- Remote installer smoke passed from raw GitHub `main` plus
  `releases/latest/download`: the installer downloaded and verified the
  `macos-arm64` binary, preserved executable bits for `scripts/harness` and
  `scripts/bin/harness-cli`, and the installed command ran `init`, `intake`,
  `query stats`, and `import brownfield`.
