# Stress Test Results: `sync-pm-skills.py` performance

## Bootstrap Sync (First Run)

- **Description**: Syncing against a clean SQLite database to register all 42 custom tools.
- **Execution Time**: 0.0435 seconds
- **Registered**: 42 tools
- **Exit Code**: 0

## Idempotency (No-op) Stress Test (20 Iterations)

| Metric | Value |
| :--- | :--- |
| Average Time | 0.0385s |
| Minimum Time | 0.0377s |
| Maximum Time | 0.0445s |
| Standard Deviation | 0.0016s |

Individual run times (seconds):
0.0378, 0.0378, 0.0377, 0.0378, 0.0378, 0.0379, 0.0380, 0.0377, 0.0389, 0.0393, 0.0445, 0.0403, 0.0385, 0.0385, 0.0378, 0.0379, 0.0381, 0.0382, 0.0378, 0.0377

## Mutation Scenarios

### Scenario A: Register 1 New Tool
- **Execution Time**: 0.0422s
- **Action**: Registers a single new custom tool, leaving the other 42 skipped.
- **Output**: `Sync complete. Registered: 1, Updated: 0, Skipped: 1, Removed: 0`

### Scenario B: Update 1 Existing Tool
- **Execution Time**: 0.0460s
- **Action**: Modifies config of one tool, triggers remove + register for that tool, leaving 42 skipped.
- **Output**: `Sync complete. Registered: 1, Updated: 1, Skipped: 1, Removed: 0`

### Scenario C: Remove 1 Deprecated Tool
- **Execution Time**: 0.0420s
- **Action**: Detects a registered tool that no longer exists in pm-skills, removes it.
- **Output**: `Sync complete. Registered: 0, Updated: 0, Skipped: 1, Removed: 1`

### Scenario D: Bulk Mutation (10 Updates, 5 Registrations, 5 Removals)
- **Execution Time**: 0.0620s
- **Action**: Performs a mix of CLI register/remove calls in a single execution.
- **Output**: `Sync complete. Registered: 5, Updated: 0, Skipped: 0, Removed: 1`
