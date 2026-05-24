# Plan: Harness Story Verification via Skill Invocation

## Spec Sufficient

Spec is implied by the conversation:
- **Goal**: Auto-run tests via skill invocation when `harness story verify --id <id>` is called
- **Non-Goals**: Skill creation (skills exist in `.agents/skills/`), CI/CD integration
- **Constraints**: Must not break existing CLI, must be backward compatible
- **Acceptance Criteria**:
  1. `harness story add` accepts `--test-skill <skill-name>` and stores in DB
  2. `harness story verify --id <id>` invokes skill, parses JSON output, updates proof flags
  3. `harness query matrix` shows `test_skill` column
  4. `harness mcp` exposes `harness_story_verify` tool

## Task List

1. `scripts/schema/002-add-story-test-skill.sql (new)` — Add `test_skill TEXT` column to `story` table. Test: `sqlite3 harness.db ".schema story" | grep test_skill`.

2. `src/domain.rs` — Add `test_skill: Option<String>` field to `StoryMatrixRecord`. Test: `cargo test domain`.

3. `src/infrastructure.rs` — Update `query_matrix()` to SELECT test_skill. Update `add_story()` to INSERT test_skill. Test: `cargo test query_matrix`.

4. `src/application.rs` — Add `test_skill: Option<String>` to `StoryAddInput`. Test: `cargo test story_add`.

5. `src/interface.rs` — Add `--test-skill` arg to `StoryAddArgs`. Add `StoryVerify` subcommand to `StoryAction`. Test: `cargo test cli_definition_is_valid`.

6. `src/infrastructure.rs` — Implement `story_verify(id: &str) -> Result<StoryVerifyResult>`. Invoke skill as subprocess, parse JSON output. Test: `cargo test story_verify`.

7. `src/mcp.rs` — Add `harness_story_verify` tool to `get_tools_definition()` and `call_tool()`. Test: `harness mcp` responds to `tools/list`.

8. `scripts/schema/002-add-story-test-skill.sql` — Create and apply migration. Test: `harness migrate` shows applied.

9. `src/infrastructure.rs` — Add `story_verify` to `HarnessRepository` trait. Test: `cargo test`.

10. `src/application.rs` — Add `story_verify(id: &str) -> Result<StoryVerifyResult>` to `HarnessService`. Test: `cargo test`.

11. `src/interface.rs` — Handle `StoryAction::Verify` in match block, call `service.story_verify()`. Test: `harness story verify --id US-001`.

## Dependencies & Parallelism

- Tasks 1-4 are sequential (schema → domain → infrastructure → application).
- Tasks 5-6 can run in parallel (interface and infrastructure verify both depend on application).
- Tasks 7 depends on 9-10 (MCP calls service, service calls repo).
- Tasks 8 applies migration (standalone).
- Tasks 11 depends on 5, 10.

**Blocked by:** 4 (interface needs domain types)
**Parallel with:** 6 (interface and infrastructure verify can be implemented simultaneously after application)

## Acceptance

1. `story_verify_plan.md` created at `docs/claudekit/plans/`
2. All 11 tasks implemented
3. `cargo test` passes
4. `harness story add --id US-TEST --title "Test" --lane tiny --test-skill harness-qa-generate-e2e-tests` works
5. `harness story verify --id US-TEST` invokes skill, parses JSON, updates proof flags
6. `harness query matrix` shows `test_skill` column with skill name
7. `harness mcp` returns `harness_story_verify` in tools/list

## Risks

| Risk | Impact | Rollback |
|------|--------|----------|
| Migration modifies schema | Production data | `ALTER TABLE story DROP COLUMN test_skill;` |
| Skill invocation hangs | CI/test loop blocked | `harness story verify --id <id> --timeout 60` (add later) |
| JSON format mismatch | Wrong proof flags set | Manual `harness story update` to fix |

## Verification

Run:
```bash
cd /Users/bao312/Desktop/harness
cargo test
./scripts/harness init
./scripts/harness migrate
./scripts/harness story add --id US-PLAN --title "Plan test" --lane tiny --test-skill harness-qa-generate-e2e-tests
./scripts/harness story verify --id US-PLAN
./scripts/harness query matrix
echo $?
```