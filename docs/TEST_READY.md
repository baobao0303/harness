# Test Readiness Report (TEST_READY.md)

This file documents the E2E test runner, execution instructions, expected outcomes, feature coverage checklist, and a summary of the test suite structure.

---

## 1. Test Runner Execution Command

The E2E test suite can be run from the `harness` directory with the following command:

```bash
python3 tests/e2e/run_tests.py
```

---

## 2. Expected Outcomes

When executed, the test runner is expected to load 71 tests spanning all coverage tiers and output a summary ending in:

```text
======================================================================
Test Suite Summary
======================================================================
Total Tests Run: 71
Passed: 71
Failures: 0
Errors: 0
======================================================================
Outcome: PASSED
```

---

## 3. Coverage Summary Table

Below is the breakdown of the 71 test cases across the 4 Tiers of validation for the 6 core features:

| Feature / Tier | Tier 1: Feature Coverage | Tier 2: Boundary & Corner Cases | Tier 3: Cross-Feature Combinations | Tier 4: Real-World Scenarios | Total Tests |
| --- | :---: | :---: | :---: | :---: | :---: |
| **F1: Sync Scanning & Registry** | 5 | 5 | — | — | **10** |
| **F2: Argument Hint Parsing** | 5 | 5 | — | — | **10** |
| **F3: Responsibility Mapping** | 5 | 5 | — | — | **10** |
| **F4: Sync Idempotency** | 5 | 5 | — | — | **10** |
| **F5: Runner Wrapper Execution** | 5 | 5 | — | — | **10** |
| **F6: Runner Argument Validation** | 5 | 5 | — | — | **10** |
| **Cross-Feature Pairwise Interactions** | — | — | 6 | — | **6** |
| **Real-World User Journeys** | — | — | — | 5 | **5** |
| **Total Test Cases** | **30** | **30** | **6** | **5** | **71** |

---

## 4. Feature Checklist

All 6 required features are fully covered by the test cases:

- [x] **Feature 1: Sync script scanning and registering 42 commands**
  - Verified scanning detects all 9 plugins.
  - Verified it registers exactly 42 custom commands.
  - Verified integration with `harness-cli query tools`.
- [x] **Feature 2: Argument hint parsing**
  - Verified mapping of `<arg>` to required strings.
  - Verified mapping of `[a|b]` to optional enums.
  - Verified cleaning of special characters to valid alphanumeric schemas.
- [x] **Feature 3: Responsibility mapping**
  - Verified `pm-product-discovery`, `pm-product-strategy` map to `Task specification`.
  - Verified `pm-execution` maps to `Task state`.
  - Verified `pm-data-analytics`, `pm-market-research`, `pm-marketing-growth`, `pm-go-to-market` map to `Project memory`.
  - Verified `pm-toolkit`, `pm-ai-shipping` map to `Tool access`.
- [x] **Feature 4: Idempotency of synchronization script**
  - Verified double synchronization does not create database duplicate rows.
  - Verified updates to descriptions, responsibilities, and argument schemas propagate correctly on subsequent syncs.
  - Verified deprecated command removal when deleted from disk.
- [x] **Feature 5: Command runner wrapper execution and output**
  - Verified execution outputs markdown content (Workflow / description) of target commands.
  - Verified exit code 0 on successful execution.
- [x] **Feature 6: Command runner wrapper arguments validation**
  - Verified validation of required arguments.
  - Verified case-insensitive enum validation.
  - Verified exit code 1 and error reporting on validation failure.
