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
| Missing required arg for draft-nda | `['draft-nda']` | 1 | `Validation Error: Missing required argument: parties_and_context` | PASS |
| Missing second required arg for battlecard | `['battlecard', 'Our CRM Product']` | 1 | `Validation Error: Missing required argument: competitor` | PASS |
| Missing required arg for sprint | `['sprint']` | 1 | `Validation Error: Missing required argument: context` | PASS |
| Invalid Enum value treated as context (extra arg error) | `['sprint', 'invalid_enum_mode', 'Some sprint context']` | 1 | `Validation Error: Unexpected extra arguments: ['Some sprint context']` | PASS |
| Unexpected extra arguments for draft-nda | `['draft-nda', 'Acme Corp vs Beta LLC', 'Extra Argument']` | 1 | `Validation Error: Unexpected extra arguments: ['Extra Argument']` | PASS |
| Non-existent command | `['nonexistent-command', 'arg1']` | 1 | `Error: Command 'nonexistent-command' not found under pm-skills directory.` | PASS |

## Detailed Execution Logs

### Test: Valid Sprint with Enum (plan)
- **Command**: `pm-skills-runner sprint plan Sprint 1 plan details`
- **Exit Code**: 0
- **Stdout**:
```
Description: Sprint lifecycle — plan a sprint, run a retrospective, or generate release notes

# /sprint -- Sprint Lifecycle

Three modes covering the sprint lifecycle: **plan** for sprint planning, **retro** for retrospectives, **release-notes** for shipping communication.
...
```

----------------------------------------

### Test: Valid Sprint with Enum (retro)
- **Command**: `pm-skills-runner sprint retro Sprint 1 retro details`
- **Exit Code**: 0
- **Stdout**:
```
Description: Sprint lifecycle — plan a sprint, run a retrospective, or generate release notes

# /sprint -- Sprint Lifecycle

Three modes covering the sprint lifecycle: **plan** for sprint planning, **retro** for retrospectives, **release-notes** for shipping communication.
...
```

----------------------------------------

### Test: Valid Sprint with Enum case-insensitive (PLAN)
- **Command**: `pm-skills-runner sprint PLAN Sprint 1 plan details`
- **Exit Code**: 0
- **Stdout**:
```
Description: Sprint lifecycle — plan a sprint, run a retrospective, or generate release notes

# /sprint -- Sprint Lifecycle

Three modes covering the sprint lifecycle: **plan** for sprint planning, **retro** for retrospectives, **release-notes** for shipping communication.
...
```

----------------------------------------

### Test: Valid Sprint skipping optional Enum
- **Command**: `pm-skills-runner sprint Sprint 1 plain context`
- **Exit Code**: 0
- **Stdout**:
```
Description: Sprint lifecycle — plan a sprint, run a retrospective, or generate release notes

# /sprint -- Sprint Lifecycle

Three modes covering the sprint lifecycle: **plan** for sprint planning, **retro** for retrospectives, **release-notes** for shipping communication.
...
```

----------------------------------------

### Test: Valid draft-nda with spaces and special characters
- **Command**: `pm-skills-runner draft-nda Mutual NDA between Acme Corp & Beta LLC (jurisdiction: NY; 5 yrs)!`
- **Exit Code**: 0
- **Stdout**:
```
Description: Draft a Non-Disclosure Agreement between two parties with jurisdiction-appropriate clauses

# /draft-nda -- NDA Drafting

Draft a professional Non-Disclosure Agreement customized to your situation. Covers information types, jurisdiction, term, and clearly marks clauses that need legal review.
...
```

----------------------------------------

### Test: Valid battlecard with two required arguments
- **Command**: `pm-skills-runner battlecard Our CRM Product Salesforce CRM`
- **Exit Code**: 0
- **Stdout**:
```
Description: Create a sales-ready competitive battlecard — positioning, feature comparison, objection handling, and win strategies

# /battlecard -- Competitive Battlecard

Create a concise, sales-ready battlecard that helps your team win deals against a specific competitor. Includes positioning, feature comparison, objection handling, and conversation strategies.
...
```

----------------------------------------

### Test: Missing required arg for draft-nda
- **Command**: `pm-skills-runner draft-nda`
- **Exit Code**: 1
- **Stderr**:
```
Validation Error: Missing required argument: parties_and_context
```

----------------------------------------

### Test: Missing second required arg for battlecard
- **Command**: `pm-skills-runner battlecard Our CRM Product`
- **Exit Code**: 1
- **Stderr**:
```
Validation Error: Missing required argument: competitor
```

----------------------------------------

### Test: Missing required arg for sprint
- **Command**: `pm-skills-runner sprint`
- **Exit Code**: 1
- **Stderr**:
```
Validation Error: Missing required argument: context
```

----------------------------------------

### Test: Invalid Enum value treated as context (extra arg error)
- **Command**: `pm-skills-runner sprint invalid_enum_mode Some sprint context`
- **Exit Code**: 1
- **Stderr**:
```
Validation Error: Unexpected extra arguments: ['Some sprint context']
```

----------------------------------------

### Test: Unexpected extra arguments for draft-nda
- **Command**: `pm-skills-runner draft-nda Acme Corp vs Beta LLC Extra Argument`
- **Exit Code**: 1
- **Stderr**:
```
Validation Error: Unexpected extra arguments: ['Extra Argument']
```

----------------------------------------

### Test: Non-existent command
- **Command**: `pm-skills-runner nonexistent-command arg1`
- **Exit Code**: 1
- **Stderr**:
```
Error: Command 'nonexistent-command' not found under pm-skills directory.
```

----------------------------------------
