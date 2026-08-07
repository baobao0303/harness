# Runner Wrapper Validation Report

This report logs the manual execution checks performed against `harness/scripts/pm-skills-runner` under different input configurations.

| Test Case Name | Arguments | Exit Code | Stderr Snippet | Status |
| :--- | :--- | :---: | :--- | :---: |
| Valid Sprint with Enum (plan) | `['sprint', 'plan', 'Sprint 1 plan details']` | 0 | `` | PASS |
| Valid Sprint with Enum (retro) | `['sprint', 'retro', 'Sprint 1 retro details']` | 0 | `` | PASS |
| Valid Sprint with Enum case-insensitive (PLAN) | `['sprint', 'PLAN', 'Sprint 1 plan details']` | 0 | `` | PASS |
| Valid Sprint skipping optional Enum | `['sprint', 'Sprint 1 plain context']` | 0 | `` | PASS |
| Valid draft-nda with spaces and special characters | `['draft-nda', 'Mutual NDA between Acme Corp & Beta LLC (jurisdiction: NY; 5 yrs)!']` | 0 | `` | PASS |
| Valid battlecard with two required arguments | `['battlecard', 'Our CRM Product', 'Salesforce CRM']` | 0 | `` | PASS |
| Missing required arg for draft-nda | `['draft-nda']` | 1 | `Validation Error: Missing required argument: parties` | PASS |
| Missing second required arg for battlecard | `['battlecard', 'Our CRM Product']` | 1 | `Validation Error: Missing required argument: productb` | PASS |
| Missing required arg for sprint | `['sprint']` | 1 | `Validation Error: Missing required argument: sprint_context` | PASS |
| Invalid Enum value treated as context (extra arg error) | `['sprint', 'invalid_enum_mode', 'Some sprint context']` | 1 | `Validation Error: Unexpected extra arguments: ['Some sprint context']` | PASS |
| Unexpected extra arguments for draft-nda | `['draft-nda', 'Acme Corp vs Beta LLC', 'Extra Argument']` | 1 | `Validation Error: Unexpected extra arguments: ['Extra Argument']` | PASS |
| Non-existent command | `['nonexistent-command', 'arg1']` | 1 | `Error: Command 'nonexistent-command' not found under pm-skills directory.` | PASS |

## Detailed Execution Logs

### Test: Valid Sprint with Enum (plan)
- **Command**: `pm-skills-runner sprint plan Sprint 1 plan details`
- **Exit Code**: 0
- **Status**: PASS
- **Stdout**:
```
Description: Sprint planning

Sprint planning workflow content.
```

----------------------------------------

### Test: Valid Sprint with Enum (retro)
- **Command**: `pm-skills-runner sprint retro Sprint 1 retro details`
- **Exit Code**: 0
- **Status**: PASS
- **Stdout**:
```
Description: Sprint planning

Sprint planning workflow content.
```

----------------------------------------

### Test: Valid Sprint with Enum case-insensitive (PLAN)
- **Command**: `pm-skills-runner sprint PLAN Sprint 1 plan details`
- **Exit Code**: 0
- **Status**: PASS
- **Stdout**:
```
Description: Sprint planning

Sprint planning workflow content.
```

----------------------------------------

### Test: Valid Sprint skipping optional Enum
- **Command**: `pm-skills-runner sprint Sprint 1 plain context`
- **Exit Code**: 0
- **Status**: PASS
- **Stdout**:
```
Description: Sprint planning

Sprint planning workflow content.
```

----------------------------------------

### Test: Valid draft-nda with spaces and special characters
- **Command**: `pm-skills-runner draft-nda Mutual NDA between Acme Corp & Beta LLC (jurisdiction: NY; 5 yrs)!`
- **Exit Code**: 0
- **Status**: PASS
- **Stdout**:
```
Description: Draft NDA

Draft NDA workflow content.
```

----------------------------------------

### Test: Valid battlecard with two required arguments
- **Command**: `pm-skills-runner battlecard Our CRM Product Salesforce CRM`
- **Exit Code**: 0
- **Status**: PASS
- **Stdout**:
```
Description: Battlecard comparison

Battlecard workflow content.
```

----------------------------------------

### Test: Missing required arg for draft-nda
- **Command**: `pm-skills-runner draft-nda`
- **Exit Code**: 1
- **Status**: PASS
- **Stderr**:
```
Validation Error: Missing required argument: parties
```

----------------------------------------

### Test: Missing second required arg for battlecard
- **Command**: `pm-skills-runner battlecard Our CRM Product`
- **Exit Code**: 1
- **Status**: PASS
- **Stderr**:
```
Validation Error: Missing required argument: productb
```

----------------------------------------

### Test: Missing required arg for sprint
- **Command**: `pm-skills-runner sprint`
- **Exit Code**: 1
- **Status**: PASS
- **Stderr**:
```
Validation Error: Missing required argument: sprint_context
```

----------------------------------------

### Test: Invalid Enum value treated as context (extra arg error)
- **Command**: `pm-skills-runner sprint invalid_enum_mode Some sprint context`
- **Exit Code**: 1
- **Status**: PASS
- **Stderr**:
```
Validation Error: Unexpected extra arguments: ['Some sprint context']
```

----------------------------------------

### Test: Unexpected extra arguments for draft-nda
- **Command**: `pm-skills-runner draft-nda Acme Corp vs Beta LLC Extra Argument`
- **Exit Code**: 1
- **Status**: PASS
- **Stderr**:
```
Validation Error: Unexpected extra arguments: ['Extra Argument']
```

----------------------------------------

### Test: Non-existent command
- **Command**: `pm-skills-runner nonexistent-command arg1`
- **Exit Code**: 1
- **Status**: PASS
- **Stderr**:
```
Error: Command 'nonexistent-command' not found under pm-skills directory.
```

----------------------------------------
