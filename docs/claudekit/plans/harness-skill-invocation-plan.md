# Plan: Harness Skill Invocation

## Spec Sufficient

Spec is implied by the conversation:
- **Goal**: Allow harness to invoke skills from `.agents/skills/` directory via wrapper scripts
- **Non-Goals**: Creating new skills, modifying existing skill definitions, CI/CD integration
- **Constraints**: Backward compatible (don't break existing commands), skills must have `run.sh` wrapper
- **Acceptance Criteria**:
  1. `harness skill list` shows available skills in `.agents/skills/`
  2. `harness skill run <name> [--story-id <id>] [--json]` invokes skill and returns output
  3. Skills can be invoked from `harness story verify`
  4. MCP server exposes `harness_skill_list` and `harness_skill_run` tools
  5. JSON output from skills is parsed correctly

## Task List

1. `src/domain.rs` — Add `SkillInfo` struct with `name`, `description`, `path`. Add `SkillResult` struct with `unit_passed`, `integration_passed`, `e2e_passed`, `platform_passed`. Test: `cargo test domain`.

2. `src/infrastructure.rs` — Implement `list_skills() -> Vec<SkillInfo>`: scan `.agents/skills/` directories, read SKILL.md for description, check for `run.sh`. Test: `ls .agents/skills/*/run.sh | wc -l`.

3. `src/infrastructure.rs` — Implement `invoke_skill(name: &str, story_id: Option<&str>) -> Result<SkillResult>`: spawn `.agents/skills/<name>/run.sh <story_id>`, capture stdout, parse JSON. Test: `.agents/skills/harness-qa-generate-e2e-tests/run.sh TEST`.

4. `src/infrastructure.rs` — Add `list_skills` and `invoke_skill` to `HarnessRepository` trait. Test: `cargo test`.

5. `src/application.rs` — Add `list_skills() -> Vec<SkillInfo>` and `invoke_skill(name: &str, story_id: Option<&str>) -> Result<SkillResult>` to `HarnessService`. Test: `cargo test`.

6. `src/interface.rs` — Add `Skill` subcommand to `Command` enum with `SkillList` and `SkillRun` actions. Add `SkillArgs` struct. Test: `cargo test cli_definition_is_valid`.

7. `src/interface.rs` — Add `DbArgs` and `StoryArgs` handling for `harness story verify --id <id> --skill <name>`. Test: `harness story verify --id US-001 --skill harness-qa-generate-e2e-tests`.

8. `src/mcp.rs` — Add `harness_skill_list` and `harness_skill_run` to `get_tools_definition()`. Add matching cases in `call_tool()`. Test: `harness mcp` responds to `tools/list`.

9. `scripts/schema/` — No schema changes needed (skill invocation is runtime, not stored in DB).

10. Documentation: Create `.agents/skills/SKILL_WRAPPER_SPEC.md` specifying the contract for `run.sh` (args, JSON output format). Test: Read the spec.

## Dependencies & Parallelism

- Tasks 1-5 are sequential (domain → infra repo → trait → app service).
- Tasks 6-7 can run in parallel (both depend on service from 5).
- Task 8 depends on 5.
- Task 10 is standalone documentation.
- Task 7 (story verify integration) depends on skill invocation working (5).

**Blocked by:** 1 (all downstream depend on domain types)
**Parallel with:** 6 and 7 (interface tasks depend on 5, can be parallel to each other if both call service)

## Acceptance

1. `harness skill list` outputs:
```
Available skills in .agents/skills/:
  harness-qa-generate-e2e-tests  - Generate E2E tests for story
  harness-check-implementation-readiness - Verify story is ready
  ...
```

2. `harness skill run harness-qa-generate-e2e-tests --story-id US-001` returns:
```json
{
  "unit_passed": true,
  "integration_passed": true,
  "e2e_passed": true,
  "platform_passed": false
}
```

3. `harness story verify --id US-001 --skill harness-qa-generate-e2e-tests` works

4. `harness mcp` shows `harness_skill_list` and `harness_skill_run` in tools/list

5. Skills without `run.sh` show error: "Skill harness-X has no run.sh wrapper"

## Risks

| Risk | Impact | Rollback |
|------|--------|----------|
| Skill `run.sh` hangs | Harness blocks forever | Add timeout support (later): `--timeout 300` |
| JSON output mismatch | Wrong proof flags | Log raw output, allow manual override |
| Skill not found | Command fails | `harness skill list` shows available skills |

## Verification

```bash
cd /Users/bao312/Desktop/harness

# Test skill list
harness skill list

# Test skill run (needs run.sh in skill)
harness skill run harness-qa-generate-e2e-tests --story-id US-TEST

# Test story verify
harness story verify --id US-TEST --skill harness-qa-generate-e2e-tests

# Test MCP
echo '{"jsonrpc":"2.0","method":"tools/list","id":1}' | harness mcp
echo '{"jsonrpc":"2.0","method":"tools/call","params":{"name":"harness_skill_list","arguments":{}},"id":2}' | harness mcp
```

## Skill Wrapper Contract

For a skill to be invoked by harness, it must have:

`.agents/skills/<skill-name>/run.sh`:
```bash
#!/bin/bash
set -euo pipefail
STORY_ID="${1:-}"

# Do skill work...
# Output JSON to stdout

echo '{"unit_passed":true,"integration_passed":true,"e2e_passed":true,"platform_passed":false}'
```

Exit code 0 = success, non-zero = failure with error message on stderr.