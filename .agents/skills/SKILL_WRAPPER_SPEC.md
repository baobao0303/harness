# Spec: Harness Skill Wrapper Contract

To allow **Harness** to list and execute skills, they must adhere to this standardized skill contract.

---

## Folder Location
Each skill must live in its own directory under `.agents/skills/`:
```
.agents/skills/<skill-name>/
```

---

## 1. Skill Metadata (`SKILL.md`)
Every skill folder must contain a `SKILL.md` markdown file. 
At the top of `SKILL.md`, it must have a standard YAML frontmatter defining:
* `name`: The system-level name of the skill (matches folder name).
* `description`: A single-line explanation of the skill's purpose and usage instructions.

Example YAML Frontmatter:
```yaml
---
name: harness-qa-generate-e2e-tests
description: 'Generate end to end automated tests for existing features. Use when the user says "create qa automated tests for [feature]"'
---
```

---

## 2. Invocation Wrapper (`run.sh`)
To execute the skill, the directory must contain a POSIX shell script wrapper named:
```
run.sh
```

### Script Execution & Context:
* **Working Directory**: The wrapper script is executed with its working directory set to `.agents/skills/<skill-name>/`.
* **CommandLine Arguments**:
  * **Arg 1**: `STORY_ID` (optional). If the skill is executed for a specific story verification, the `story_id` is passed as the first positional argument.

### Standard Output (stdout):
The script must print a valid JSON object to `stdout` containing the test results:
```json
{
  "unit_passed": true,
  "integration_passed": true,
  "e2e_passed": true,
  "platform_passed": false
}
```

### Exit Codes:
* **`0`**: Execution succeeded and completed successfully.
* **`non-zero`**: An error occurred. Details of the error should be printed to `stderr`.

---

## Example `run.sh` Wrapper
Here is a sample bash implementation:
```bash
#!/bin/bash
set -euo pipefail

# 1. Capture parameters
STORY_ID="${1:-}"

# 2. Invoke the underlying python or node tool
# python3 verify.py "$STORY_ID"

# 3. Output standard JSON format
echo '{"unit_passed":true,"integration_passed":true,"e2e_passed":true,"platform_passed":false}'
```
