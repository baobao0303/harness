# Test Infrastructure Specification (TEST_INFRA.md)

This document outlines the testing philosophy, feature inventory, test architecture, real-world application scenarios, and coverage thresholds for the `pm-skills` integration and synchronization test suite.

---

## 1. Test Philosophy

Our approach to testing the `pm-skills` registration and execution pipeline adheres to the following core tenets:

*   **Opaque-Box E2E Testing**: We treat the synchronization script (`sync-pm-skills.py`) and command runner wrapper (`pm-skills-runner`) as black boxes. We verify system behavior solely by executing the entry points in a shell environment and inspecting the resulting state in the SQLite database (`harness.db`) and stdout/stderr outputs.
*   **Test Isolation**: Each test runs with an isolated database file (configured via the `HARNESS_DB` environment variable) and optionally a custom mock `pm-skills` folder to prevent cross-contamination and ensure determinism.
*   **Zero-Hardcoding (Integrity)**: All assertions verify actual system outputs, CLI response states, and database values. Hardcoding expected test outputs or using mock facades is strictly prohibited.
*   **Coverage Tiers**: Testing is structured into four distinct tiers:
    1.  **Feature Coverage**: Core behavior validation under normal conditions.
    2.  **Boundary & Corner Cases**: Empty inputs, malformed structures, extreme values, and syntax errors.
    3.  **Cross-Feature Combinations**: Pairwise interaction of different features.
    4.  **Real-World Application Scenarios**: Multi-step, stateful operations reflecting actual developer/agent workflows.

---

## 2. Feature Inventory

The test suite validates six key features:

| Feature ID | Feature Name | Description |
|------------|--------------|-------------|
| **F1** | Sync Script Scanning & Registry | Automatically scan the 9 plugins under `pm-skills/` and register all 42 commands in the `harness-cli` tool registry. |
| **F2** | Argument Hint Parsing | Parse the markdown YAML frontmatter `argument-hint` and convert it into valid `harness-cli` tool argument specifications (`--args`). |
| **F3** | Responsibility Mapping | Map each plugin to the designated `harness-cli` responsibility (e.g., `Task specification`, `Task state`, `Project memory`, `Tool access`). |
| **F4** | Sync Idempotency | Running the synchronization script multiple times must succeed, producing identical database states and preventing duplicates. |
| **F5** | Command Runner Execution & Output | The wrapper script must execute registered commands and output the exact markdown prompt/workflow content. |
| **F6** | Command Runner Argument Validation | The runner wrapper must validate incoming command-line arguments against registered specifications and enforce boundaries. |

---

## 3. Test Architecture

The E2E test suite is implemented in Python and executed via a unified test runner.

### Directory Structure

```text
harness/
  ├── TEST_INFRA.md              # This file
  ├── TEST_READY.md              # Test status, checklist, and runner commands
  ├── scripts/
  │    ├── sync-pm-skills.py      # Target synchronization script (Milestone 3)
  │    └── pm-skills-runner       # Target command runner wrapper (Milestone 3)
  └── tests/
       └── e2e/
            ├── run_tests.py      # Test runner script
            └── test_suite.py     # E2E test cases (70+ tests covering Tiers 1-4)
```

### Execution Flow

```text
[test_suite.py]
  │
  ├── 1. Setup Temp Test DB & Mock plugin dirs (if needed)
  ├── 2. Run sync-pm-skills.py (with HARNESS_DB pointing to Temp DB)
  ├── 3. Execute harness-cli query commands to assert DB state
  ├── 4. Run pm-skills-runner to execute registered commands
  ├── 5. Assert outputs, exit codes, and error messages
  └── 6. Teardown Temp files/DBs
```

---

## 4. Coverage Thresholds

To ensure a robust integration, we enforce the following minimum test counts and constraints:

*   **Tier 1: Feature Coverage**
    *   Minimum of 5 test cases per feature.
    *   Total: **>= 30 test cases**.
*   **Tier 2: Boundary & Corner Cases**
    *   Minimum of 5 test cases per feature.
    *   Total: **>= 30 test cases**.
*   **Tier 3: Cross-Feature Combinations**
    *   Pairwise interactions (e.g., Idempotency + Custom Argument Hint Parsing).
    *   Total: **>= 6 test cases**.
*   **Tier 4: Real-World Scenarios**
    *   Stateful, end-to-end user journeys.
    *   Total: **>= 5 test cases**.

**Overall minimum test count: 71 test cases.**

---

## 5. Real-World Application Scenarios

The suite includes 5 realistic workflows:

1.  **Fresh Repo Bootstrap**: Installing the harness in a new repository, running initialization, and doing the initial sync.
2.  **Plugin Addition/Evolution**: Simulating the addition of a new plugin and command to the `pm-skills` directory, running the sync script, and checking if the new command is registered alongside the existing 42 commands.
3.  **Frontmatter Update Propagation**: Modifying an `argument-hint` in a command markdown file and running sync to verify the new argument schema updates in the database.
4.  **Agent Multi-Step Execution Flow**: Bootstrapping, running sync twice (idempotency check), executing commands with valid and boundary inputs via the runner wrapper, and checking the system state.
5.  **Broken Manifest & Recovery**: Syncing when one of the plugin manifests is corrupted, verifying that sync reports a clean error, fixing the manifest, re-syncing, and confirming that the system is fully operational.
