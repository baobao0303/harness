#Phase 4 — Mechanical Verification: Finalized Scope

**Target repo:** `repository-harness` (feature branch off `main`)
**Validation:** `harness-benchmark` re-run after implementation
**Current harness maturity:** H3 partial (Phase 3 active observability complete)
**Target maturity:** H3 (full) → H4 (partial: story verification, auto-scoring, pre-close gate)

---

## Benchmark Triage After First Re-Run

The first Phase 4 benchmark showed one isolated compliance miss and several
command-shape friction loops:

- T4 authentication included decision text in the trace but did not create a
  durable decision record. High-risk work that changes auth, authorization,
  data ownership, API behavior, architecture, or validation must add a
  `docs/decisions/NNNN-*.md` record and a durable `decision` row with
  `scripts/bin/harness-cli decision add`. Trace `--decisions` is evidence, not
  the decision log.
- Rust CLI proof flags require numeric booleans. Use
  `--unit 1 --integration 1 --e2e 0 --platform 0`; do not use `yes` or `no`.
- `story verify <id>` runs the story's configured `verify_command` and records
  pass/fail. It accepts only the story id. Proof flags belong to
  `story update`.
- Agents should prefer the command examples in `docs/HARNESS.md` and
  `scripts/README.md` before repeated help probing. Re-run help only when the
  command shape is still unknown.

---

## What Phase 4 Is

Phase 4 turns the harness from a system that *observes* agent work into one
that *verifies* it. Phase 3 gave agents a way to check whether their trace was
good enough. Phase 4 gives them a way to check whether their *implementation*
meets the story contract — and warns them before they close a task without
running the check.

The `decision` table already has a `verify_command` column and `decision verify`
already runs it (Phase 1 infrastructure). The `story` table does not. Phase 4
extends the same pattern to stories, adds automatic trace scoring on write, and
introduces a pre-close verification gate.

Phase 4 is **Rust CLI code + schema migration + documentation**.

---

## Research Grounding

Five of the nine Arxiv papers surveyed in Phase 0 converge on verification as
the next capability:

| Paper | Recommendation |
|---|---|
| Runtime Substrate (2605.13357) | H3→H4 = "the harness can verify, not just observe" |
| AHE (2604.25850) | The `verify_command` column exists but has no story-level execution path |
| NLAHs (2603.25723) | NL policies need enforcement — validation gates before state transitions |
| "The Last Harness" (2604.21003) | The Evaluator role in the Worker→Evaluator→Evolution loop must be mechanical |
| Continual Harness (2605.09998) | Self-improvement requires knowing whether traces are *accurate*, which requires verification |

---

## Why This Order Matters

```
US-012 Story verify_command Field
  ↓ schema migration adds the column; CLI accepts the flag
  ↓ stories can now carry a mechanical proof command
US-015 Story Verify Command
  ↓ agents can run the proof command and record the result
  ↓ the Evaluator role becomes mechanical
US-016 Auto Trace Scoring on Write
  ↓ agents get immediate trace quality feedback when recording
  ↓ removes the need to remember to run score-trace separately
US-017 Pre-Close Verification Gate
  ↓ combines trace scoring + verification into a single checkpoint
  ↓ agents are warned before closing a task without proof
```

US-012 must be first because it creates the schema column. US-015 depends on it
to have something to execute. US-016 is independent of US-012/US-015 but ordered
here because it's simpler. US-017 depends on both US-015 (verification) and
US-016 (auto-scoring) to compose them into a single gate.

---

## Stories

### US-012: Story `verify_command` Field

**Background:**

The `decision` table already has `verify_command`, `last_verified_at`, and
`last_verified_result` columns (Phase 1 schema, `001-init.sql` lines 75-79).
The `decision verify <id>` CLI command already runs the command via `sh -c`,
records pass/fail, and updates the timestamp (`infrastructure.rs` line 508+).

The `story` table has proof columns (`unit_proof`, `integration_proof`,
`e2e_proof`, `platform_proof`) and a free-text `evidence` field. But it has no
`verify_command` column. Stories cannot carry a mechanical check command that
proves the story's acceptance criteria are met.

**Reason:**

AHE (arXiv:2604.25850) says "every edit is a falsifiable contract." NLAHs
(arXiv:2603.25723) says NL policies need enforceable validation gates. The
`decision` table already implements this pattern — stories should too.

In the Phase 3 benchmark, T4 (authentication, high_risk) was the only task
that failed its trace tier requirement. There was no mechanism for the agent
to mechanically verify the story was complete beyond checking trace quality.
A `verify_command` on the story would let the agent (or benchmark) run
`npm test -- --run auth` to confirm the implementation works.

**Solution:**

1. New migration file `scripts/schema/002-story-verify.sql`:
   ```sql
   ALTER TABLE story ADD COLUMN verify_command TEXT;
   ALTER TABLE story ADD COLUMN last_verified_at TEXT;
   ALTER TABLE story ADD COLUMN last_verified_result TEXT
     CHECK(last_verified_result IN ('pass','fail') OR last_verified_result IS NULL);
   ```
2. Update `harness-cli story add` to accept `--verify <command>`.
3. Update `harness-cli story update` to accept `--verify <command>`.
4. Update `StoryAddInput` and `StoryUpdateInput` in `application.rs`.
5. Update `StoryAddArgs` and `StoryUpdateArgs` in `interface.rs`.
6. Update SQL `INSERT` and `UPDATE` in `infrastructure.rs`.

**Acceptance Criteria:**

| # | Criterion | How to verify |
|---|-----------|---------------|
| 1 | `scripts/schema/002-story-verify.sql` exists and adds `verify_command`, `last_verified_at`, and `last_verified_result` columns to the `story` table. | Read the file. Confirm the three ALTER TABLE statements and the CHECK constraint on `last_verified_result`. |
| 2 | `harness-cli migrate` applies migration 002 on an existing database. | Run `harness-cli init` (creates v1 DB), then `harness-cli migrate`. Verify `schema_version` contains version 2 and `story` table has the three new columns via `harness-cli query sql "PRAGMA table_info(story)"`. |
| 3 | `harness-cli story add --id US-099 --title "Test" --lane normal --verify "echo ok"` stores the verify_command. | Run the command, then `harness-cli query sql "SELECT verify_command FROM story WHERE id='US-099'"`. Expect `echo ok`. |
| 4 | `harness-cli story update --id US-099 --verify "npm test"` updates the verify_command on an existing story. | Run the command, then query again. Expect `npm test`. |
| 5 | `harness-cli init` on a fresh database creates tables with the v2 columns present. | Delete the DB, run `init`. Confirm `story` table has `verify_command`, `last_verified_at`, `last_verified_result` via PRAGMA. |
| 6 | `cargo test` passes with tests covering the migration and the new fields. | Run `cargo test` in the workspace root. |

**Lane:** Normal (schema migration + CLI changes across all four layers).

---

### US-015: Story Verify Command

**Background:**

`decision verify <id>` already exists. It reads `verify_command` from the
`decision` table, runs it via `sh -c` from the repo root, stores `pass` or
`fail` in `last_verified_result`, and updates `last_verified_at`
(`infrastructure.rs` lines 508-540).

After US-012, stories will have the same three columns. But there is no
`story verify` CLI command to execute the check.

**Reason:**

Runtime Substrate (arXiv:2605.13357) defines H4 as "the harness can run or
orchestrate proof checks consistently." "The Last Harness" (arXiv:2604.21003)
describes the Evaluator role — a mechanical agent that checks whether work
meets its contract. `story verify` is this Evaluator for story-level work.

**Solution:**

1. Add `Verify { id: String }` variant to `StoryAction` enum in `interface.rs`.
2. Add `verify_story(&self, id: &str)` to `HarnessService` in `application.rs`.
3. Add `verify_story(&self, id: &str)` to `HarnessRepository` trait and
   `SqliteHarnessRepository` in `infrastructure.rs`.
4. Implementation mirrors `verify_decision`: read `verify_command` from story,
   run `sh -c <command>` from repo root, store result and timestamp.
5. Add `StoryVerifyResult` to `application.rs` (mirrors `DecisionVerifyResult`).
6. Add `MissingStoryVerifyCommand(String)` variant to `HarnessInfraError`.
7. Print output: `Running: <command>` then `Story <id> verification: pass/fail`.
8. Exit code 0 for pass, 1 for fail.

**Acceptance Criteria:**

| # | Criterion | How to verify |
|---|-----------|---------------|
| 1 | `harness-cli story verify US-099` runs the story's `verify_command` and prints the result. | Add a story with `--verify "echo ok"`, run `story verify US-099`. Output: `Running: echo ok` then `Story US-099 verification: pass`. |
| 2 | The command updates `last_verified_at` and `last_verified_result` in the database. | After verify, `query sql "SELECT last_verified_at, last_verified_result FROM story WHERE id='US-099'"` shows a timestamp and `pass`. |
| 3 | A failing verify_command records `fail`. | Add story with `--verify "exit 1"`, run `story verify`. Output shows `fail`. DB shows `fail`. |
| 4 | A story with no verify_command produces an error. | Add story without `--verify`, run `story verify`. Error: `story US-100 has no verify_command`. |
| 5 | `story verify` exits with code 0 on pass and code 1 on fail. | `harness-cli story verify US-099 && echo OK` prints `OK` for passing command. `harness-cli story verify US-fail || echo FAILED` prints `FAILED` for failing command. |
| 6 | The command runs from the repo root directory. | Add story with `--verify "pwd"`, verify output shows the repo root path. |
| 7 | `cargo test` passes with tests covering pass, fail, and missing verify_command cases. | Run `cargo test`. |

**Lane:** Normal (new CLI subcommand, touches all four code layers).

---

### US-016: Auto Trace Scoring on Write

**Background:**

`harness-cli score-trace` exists as a separate command (Phase 3). Agents must
remember to run it after recording a trace. In the Phase 3 benchmark, trace
quality was 2.5/3.0 — agents sometimes forgot to self-check. The score-trace
command is available but not integrated into the trace recording workflow.

**Reason:**

AHE (arXiv:2604.25850) emphasizes immediate feedback over post-hoc evaluation.
The Context Engineering paper (arXiv:2603.05344) notes that agents follow
guidance best when it's presented at the point of action, not as a separate
step. Auto-scoring removes the "remember to run score-trace" failure mode.

**Solution:**

1. After `record_trace` succeeds in `HarnessService`, call `score_trace` with
   the newly created trace ID.
2. Print the score summary after the `Trace #N recorded.` confirmation.
3. If the trace is below its lane requirement, print a warning and the missing
   fields — but do NOT exit with code 1 (trace recording should always
   succeed; the warning is advisory).
4. Update the `Trace` command handler in `interface.rs` to call `score_trace`
   after recording and print the result using the existing `print_trace_score`
   function.

**Example Output:**

```
Trace #8 recorded.
  Tier achieved: standard (2/3)
  Lane: high_risk -> required tier: detailed (3/3)
  BELOW REQUIREMENT

  Missing for detailed:
    - decisions_made: empty
    - duration_seconds: null (no explanation in notes)
```

**Acceptance Criteria:**

| # | Criterion | How to verify |
|---|-----------|---------------|
| 1 | `harness-cli trace --summary "test" --outcome completed` prints the trace ID and the trace quality score. | Run the command. Output includes both `Trace #N recorded.` and `Tier achieved:`. |
| 2 | When the trace is linked to an intake via `--intake`, the output shows the lane requirement and whether it is met. | Record an intake with `--lane high_risk`, then `trace --summary "test" --outcome completed --intake 1`. Output includes `Lane: high_risk -> required tier: detailed`. |
| 3 | When the trace is below its lane requirement, the output shows `BELOW REQUIREMENT` and lists missing fields. | Record a minimal trace linked to a high_risk intake. Output includes `BELOW REQUIREMENT` and missing field list. |
| 4 | The trace is always recorded successfully regardless of the score. | Even when below requirement, the trace row exists in the database. |
| 5 | The `trace` command always exits with code 0 (scoring is advisory, not blocking). | `harness-cli trace --summary "test" --outcome completed; echo $?` outputs `0`. |
| 6 | `cargo test` passes with tests covering auto-scoring output for minimal, standard, and detailed traces. | Run `cargo test`. |

**Lane:** Tiny (extends existing trace command output, no schema change, no new
command).

---

### US-017: Pre-Close Verification Gate

**Background:**

When an agent records a trace with `--story US-012`, the trace is linked to
that story. But there is no check for whether the story's `verify_command` has
been run. An agent can close a task (record a trace with `--outcome completed`)
without ever verifying the story's acceptance criteria.

**Reason:**

NLAHs (arXiv:2603.25723) describes validation gates — checkpoints before state
transitions that enforce NL policy compliance. "The Last Harness"
(arXiv:2604.21003) says the Evaluator should catch incomplete work before the
final response. The pre-close gate combines US-015 (verification) and US-016
(auto-scoring) into a single checkpoint: when recording a trace, the agent is
warned if the linked story has an unverified `verify_command`.

**Solution:**

1. In the `Trace` command handler (after recording and auto-scoring), check if
   the trace has a `--story` argument.
2. If a story is linked, query the story's `verify_command` and
   `last_verified_result`.
3. If `verify_command` is not null and `last_verified_result` is null (never
   verified) or `fail` (last run failed), print a warning:
   ```
   Warning: Story US-012 has verify_command but verification has not passed.
   Run: harness-cli story verify US-012
   ```
4. The warning is advisory — the trace is still recorded. Exit code remains 0.

**Acceptance Criteria:**

| # | Criterion | How to verify |
|---|-----------|---------------|
| 1 | When recording a trace with `--story US-099` where the story has a `verify_command` that has never been run, a warning is printed. | Add story with `--verify "echo ok"`. Record trace with `--story US-099 --summary "test" --outcome completed`. Output includes `Warning: Story US-099 has verify_command but verification has not passed.` |
| 2 | When the story's verify_command was already run and passed, no warning is printed. | Run `story verify US-099` (passes), then record trace with `--story US-099`. No warning in output. |
| 3 | When the story's last verification result is `fail`, the warning is printed. | Run `story verify US-fail` (fails), then record trace with `--story US-fail`. Warning is printed. |
| 4 | When the story has no verify_command, no warning is printed. | Add story without `--verify`. Record trace with that story. No warning. |
| 5 | When the trace has no `--story` flag, no verification check occurs. | Record trace without `--story`. No warning. |
| 6 | The trace is always recorded regardless of the warning. | After a warning, the trace row exists in the database. Exit code is 0. |
| 7 | `cargo test` passes with tests covering all four cases: no story, no verify_command, unverified, and previously passed. | Run `cargo test`. |

**Lane:** Tiny (extends existing trace command output, no schema change, no new
command).

---

## Out of Scope for Phase 4

| Item | Why deferred | Phase |
|------|-------------|-------|
| Benchmark comparison attribution (US-014) | Lives in `harness-benchmark`, not `repository-harness`. | Benchmark work |
| Machine-readable tool registry | NexAU gap, lower priority than verification | Phase 5 |
| Executable agent skills | Platform-dependent, moving target | Phase 5 |
| Sub-agents | No use case yet | Phase 5+ |
| Automated improvement proposals | Requires verification data first | Phase 5 (H5) |
| Config parameter search (Harbor) | Need more benchmark runs | Phase 6+ |
| Context rule enforcement / measurement | Secondary to verification | Phase 5 |
| Drift detection / entropy score | Interesting but not blocking | Phase 5 |
| Batch verification across all stories | Useful but not core — can be composed via `query sql` + shell | Phase 5 |
| Installer propagation of Phase 3/4 docs (US-007) | Separate PR | Separate |

---

## Implementation Sequence

```
Step 1: US-012 — Story verify_command field
  - Create scripts/schema/002-story-verify.sql
  - Update domain.rs (no new types needed, verify columns are strings)
  - Update application.rs (StoryAddInput, StoryUpdateInput)
  - Update interface.rs (StoryAddArgs, StoryUpdateArgs)
  - Update infrastructure.rs (INSERT, UPDATE SQL, migrate logic)
  - Write unit tests for migration and new fields
  Estimated effort: ~3-4 hours

Step 2: US-015 — Story verify command
  - Add StoryAction::Verify to interface.rs
  - Add StoryVerifyResult to application.rs
  - Add verify_story to HarnessService and HarnessRepository
  - Add MissingStoryVerifyCommand to HarnessInfraError
  - Implementation mirrors verify_decision
  - Write unit tests for pass, fail, missing cases
  Estimated effort: ~2-3 hours

Step 3: US-016 — Auto trace scoring on write
  - Update Trace command handler in interface.rs
  - After record_trace, call score_trace with the returned ID
  - Print score using existing print_trace_score function
  - Do NOT change exit code (advisory only)
  - Write unit tests
  Estimated effort: ~1-2 hours

Step 4: US-017 — Pre-close verification gate
  - After auto-scoring in Trace handler, check --story link
  - Query story verify_command and last_verified_result
  - Print advisory warning if unverified or failed
  - Add query_story_verify_status helper to infrastructure
  - Write unit tests for all four cases
  Estimated effort: ~1-2 hours

Step 5: Cross-references and documentation
  - Update docs/HARNESS.md with story verification workflow
  - Update docs/HARNESS_COMPONENTS.md (Verification: Partial → Covered)
  - Update docs/HARNESS_MATURITY.md (H4 current status)
  - Update docs/GLOSSARY.md with "verification gate" term
  - Update AGENTS.md if new commands need to be in the reading list
  - Record Phase 4 trace
  Estimated effort: ~1-2 hours
```

**Total estimated effort:** ~8-13 hours

---

## Execution Workflow

1. **Branch:** `git checkout -b feature/phase-4-mechanical-verification main`
2. **Implement US-012 → US-015 → US-016 → US-017** (in order)
3. **Update cross-references** (HARNESS.md, HARNESS_COMPONENTS.md,
   HARNESS_MATURITY.md, GLOSSARY.md)
4. **Run `cargo test`** — all tests must pass
5. **Run `cargo clippy`** — no warnings
6. **Run benchmark** in `harness-benchmark`:
   - Install harness from feature branch
   - Run `./benchmark/run.sh --agent codex --harness feature/phase-4-mechanical-verification`
7. **Compare:** `./benchmark/compare.sh phase-3-active-observability phase-4`
8. **Merge:** Only when benchmark shows stable or improved results

---

## Expected Benchmark Deltas

| Metric | Phase 3 (current) | Phase 4 Target | Reasoning |
|--------|-------------------|----------------|-----------|
| Functional score | 37/37 (100%) | 37/37 (100%) | Phase 4 doesn't change app code |
| Harness compliance | 31/31 (100%) | 31/31 (100%) | Already perfect |
| Trace quality | 2.5/3.0 | 2.8-3.0/3.0 | Auto-scoring on write gives immediate feedback; agents fix traces before closing |
| Lane accuracy | 6/6 | 6/6 | Already perfect |
| Wall time | 1749s | ~1800-1900s | Slight increase from running verify commands |
| Token cost | $21.97 | ~$22-23 | Slight increase from reading verify output |

---

## What Would Signal Success

1. `cargo test` passes with coverage for all four stories.
2. `story verify` correctly runs verify_command and records pass/fail.
3. `trace` command auto-scores and prints the tier summary.
4. `trace --story` warns when the linked story is unverified.
5. Benchmark trace quality rises to ≥2.8/3.0 (agents use auto-score feedback).
6. No regression in functional score, harness compliance, or lane accuracy.

## What Would Signal Failure

- `story verify` disagrees with `decision verify` behavior (inconsistent
  verification patterns).
- Auto-scoring breaks existing `trace` command behavior or exit codes.
- Benchmark trace quality stays at 2.5 (auto-scoring feedback ignored by agent).
- `cargo test` or `cargo clippy` failures.
- Functional score drops (verification overhead confused the agent).
