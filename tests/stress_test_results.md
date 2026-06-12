# Stress Test Results: `sync-pm-skills.py` performance

## Bootstrap Sync (First Run)

- **Description**: Syncing against a clean SQLite database to register all 42 custom tools.
- **Execution Time**: 0.1673 seconds
- **Registered**: 42 tools
- **Exit Code**: 0

## Idempotency (No-op) Stress Test (20 Iterations)

| Metric | Value |
| :--- | :--- |
| Average Time | 0.0323s |
| Minimum Time | 0.0316s |
| Maximum Time | 0.0346s |
| Standard Deviation | 0.0009s |

Individual run times (seconds):
0.0320, 0.0320, 0.0316, 0.0345, 0.0334, 0.0346, 0.0319, 0.0321, 0.0319, 0.0318, 0.0319, 0.0316, 0.0319, 0.0327, 0.0320, 0.0320, 0.0319, 0.0319, 0.0317, 0.0318

## Mutation Scenarios

### Scenario A: Register 1 New Tool
- **Execution Time**: 0.0380s
- **Action**: Registers a single new custom tool, leaving the other 42 skipped.
- **Output**: `Sync complete. Registered: 1, Updated: 0, Skipped: 42, Removed: 0`

### Scenario B: Update 1 Existing Tool
- **Execution Time**: 0.0395s
- **Action**: Modifies config of one tool, triggers remove + register for that tool, leaving 42 skipped.
- **Output**: `Sync complete. Registered: 1, Updated: 1, Skipped: 42, Removed: 0`

### Scenario C: Remove 1 Deprecated Tool
- **Execution Time**: 0.0356s
- **Action**: Detects a registered tool that no longer exists in pm-skills, removes it.
- **Output**: `Sync complete. Registered: 0, Updated: 0, Skipped: 42, Removed: 1`

### Scenario D: Bulk Mutation (10 Updates, 5 Registrations, 5 Removals)
- **Execution Time**: 0.1303s
- **Action**: Performs a mix of CLI register/remove calls in a single execution.
- **Output**: `Sync complete. Registered: 15, Updated: 10, Skipped: 27, Removed: 5`
