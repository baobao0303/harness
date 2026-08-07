# Project: Harness CLI Refactoring & Company Spec Alignment

## Architecture
- Refactored `crates/harness-cli` following Clean Architecture / DDD (domain, application, infrastructure, interface).
- SQLite DB with unified `work_item` table supporting Epic, Feature, User Story, Task, Bug, Testcase, with state machines and input syntax validation.
- CLI commands (`harness work-item add/update`, `harness query matrix`, `harness tool register/remove`).
- Python test suite with dynamic root path resolution instead of hardcoded paths.

## Milestones
| # | Name | Scope | Dependencies | Status |
|---|------|-------|-------------|--------|
| 1 | M1_Fix_Test_Paths | Fix hardcoded absolute paths in Python tests | None | DONE |
| 2 | M2_Rust_CLI_Refactor | Refactor `crates/harness-cli` into modular sub-packages (domain, application, infrastructure, interface) | None | DONE |
| 3 | M3_Company_WorkItems_StateMachines | Implement unified `work_item` schema, state machines, and syntax validation per `spec_new.md` | M2 | DONE |
| 4 | M4_CLI_Commands_Compatibility | Update/implement CLI commands (`work-item`, `query matrix`, `tool register/remove`) | M3 | DONE |
| 5 | M5_Rewrite_README | Rewrite `README.md` reflecting new structure, schema, rules, and commands | M4 | DONE |
| 6 | M6_E2E_Verification | Full test suite execution, verification, and forensic audit | M1, M4, M5 | DONE |

## Interface Contracts
### Rust CLI Domain & DB Layer
- `work_item` table schema: id, type (Epic|Feature|User Story|Task|Bug|Testcase), title, description, state, assigned_to, story_points, remaining_work, priority, severity, created_at, updated_at.
- State machines: Enforced per Section 5.4 of `spec_new.md`. Reject invalid state transitions with clear error.
- Title/Description format validation: Enforced per Section 5.5 of `spec_new.md`. Reject invalid inputs with clear error.

## Code Layout
- `crates/harness-cli/src/domain/`: Domain entities, value objects, state machines, validation logic.
- `crates/harness-cli/src/application/`: Application use cases, service logic.
- `crates/harness-cli/src/infrastructure/`: SQLite DB access, migrations, system interaction.
- `crates/harness-cli/src/interface/`: CLI argument parsing, command handlers, formatting output.
- `tests/`: Python test scripts using dynamic root directory resolution.
