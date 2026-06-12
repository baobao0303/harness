Phase 3 — Active Observability: Finalized Scope

**Target repo:** `repository-harness` (feature branch off `main`)
**Validation:** `harness-benchmark` re-run after implementation
**Current harness maturity:** H2 (achieved — Phase 2 specification layer complete)
**Target maturity:** H2→H3 transition (active scoring, friction context, feedback loop)

---

## What Phase 3 Is

Phase 3 turns the passive specifications from Phase 2 into active, self-checking
tools. Phase 2 told agents *what* a good trace looks like. Phase 3 gives them a
command that *checks* whether their trace is good enough. Phase 2 said friction
should be captured. Phase 3 shows friction alongside its task context so patterns
become visible. Phase 2 added `predicted_impact` and `actual_outcome` columns to
the backlog. Phase 3 documents the workflow that makes those columns useful.

Phase 3 is **Rust CLI code + documentation**. No schema migrations. The existing
schema already has every column needed.

---

## Why This Order Matters

```
US-011 Backlog Outcome Workflow
  ↓ documents how to use predicted_impact / actual_outcome
  ↓ the feedback loop that validates whether harness changes helped
US-008 Trace Quality Scoring
  ↓ agents can self-check trace tier against lane requirement
  ↓ mechanically enforces TRACE_SPEC.md without relying on agent memory
US-009 Enriched Friction Query
  ↓ friction entries gain lane and task-type context
  ↓ enables pattern recognition across friction entries
```

US-011 is first because it's the smallest (documentation + query filter, no new
Rust command) and establishes the feedback discipline that US-008 and US-009 will
benefit from. US-008 is the core Phase 3 CLI feature. US-009 extends existing
query output.

---

## Stories

### US-011: Backlog Outcome Workflow

**Problem:** The `backlog` table already has `predicted_impact` and
`actual_outcome` columns (Phase 1 schema). The `backlog add --predicted` and
`backlog close --outcome` flags already work. But there is no documented
workflow for when to fill these fields, no way to filter open vs closed items,
and no way to view predicted-vs-actual comparisons. The columns exist but the
feedback loop doesn't.

**Evidence:** After Phase 2, we know lane accuracy went from 5/6 to 6/6 and
trace quality from 2.0 to 2.6. But this comparison was done manually by reading
benchmark JSON. No backlog item recorded the prediction ("context rules will
improve lane accuracy") or the outcome ("lane accuracy improved 5/6 → 6/6").
The harness can't learn from its own improvements.

**What gets created/changed:**
1. Update `docs/HARNESS.md` Growth Rule section to document the
   predicted-impact → actual-outcome workflow.
2. Add `query backlog --open` filter (shows `proposed` and `accepted` items
   only).
3. Add `query backlog --closed` filter (shows `implemented` and `rejected`
   items only).
4. Update `docs/GLOSSARY.md` with "backlog outcome loop" term.

**Acceptance Criteria:**

| # | Criterion | How to verify |
|---|-----------|---------------|
| 1 | `docs/HARNESS.md` Growth Rule section documents when to fill `--predicted` on `backlog add` and `--outcome` on `backlog close`. | Read `docs/HARNESS.md`, confirm the workflow is explicit: predicted at creation, outcome at close with measured evidence. |
| 2 | `scripts/bin/harness-cli query backlog --open` returns only rows where `status IN ('proposed', 'accepted')`. | Run `scripts/bin/harness-cli init && scripts/bin/harness-cli backlog add --title "test" --predicted "x"` then `scripts/bin/harness-cli backlog close --id 1 --outcome "y"`. Verify `query backlog --open` returns 0 rows and `query backlog` returns 1 row. |
| 3 | `scripts/bin/harness-cli query backlog --closed` returns only rows where `status IN ('implemented', 'rejected')`. | After the above, verify `query backlog --closed` returns 1 row with both `predicted_impact` and `actual_outcome` visible. |
| 4 | `query backlog` (no filter) continues to return all items as it does today. | Existing behavior unchanged. |
| 5 | `docs/GLOSSARY.md` includes the term "backlog outcome loop" with a definition. | Read the glossary. |
| 6 | `cargo test` passes with tests covering the `--open` and `--closed` filters. | Run `cargo test` in the workspace root. |

**Lane:** Tiny (additive documentation + minor query filter, no schema change,
no architectural decision).

---

### US-008: Trace Quality Scoring Command

**Problem:** TRACE_SPEC.md defines three quality tiers (minimal, standard,
detailed) with specific field requirements per tier, and a lane-to-tier mapping
(tiny→minimal, normal→standard, high_risk→detailed). But the agent has no way to
check whether its trace meets the required tier.

**Evidence from Phase 2 benchmark:**

| Task | Lane | Trace Tier | Required Tier | Gap |
|------|------|-----------|---------------|-----|
| T1 | tiny | standard (2) | minimal (1) | Exceeds — fine |
| T2 | normal | detailed (3) | standard (2) | Exceeds — fine |
| T3 | normal | detailed (3) | standard (2) | Exceeds — fine |
| T4 | **high_risk** | **standard (2)** | **detailed (3)** | **Below requirement** |
| T5 | normal | detailed (3) | standard (2) | Exceeds — fine |
| T6 | normal | detailed (3) | standard (2) | Exceeds — fine |

T4 is the only task that fails its trace tier requirement. The agent wrote a
standard trace for a high-risk task that requires detailed. The missing fields
were `decisions_made` (empty) and `duration_seconds`/`token_estimate` (null).
The agent had no way to know it fell short — the scoring only happens externally
in the benchmark's `check-quality.sh`, which uses blunt string-length heuristics
rather than the actual TRACE_SPEC.md rules.

**What gets created/changed:**
1. New CLI subcommand: `scripts/bin/harness-cli score-trace` (scores the most recent
   trace) and `scripts/bin/harness-cli score-trace --id N` (scores a specific trace).
2. Scoring logic in the Rust CLI that evaluates trace fields against
   TRACE_SPEC.md tier rules.
3. When the trace is linked to an intake record, the command looks up the lane
   and compares the achieved tier against the required tier.
4. Output includes the tier achieved, the tier required, and any missing fields.

**Scoring Mechanism (field-presence rules from TRACE_SPEC.md):**

```
Minimal (score 1):
  ✓ task_summary IS NOT NULL AND length(task_summary) >= 10
  ✓ outcome IS NOT NULL

Standard (score 2) — all of Minimal plus:
  ✓ intake_id IS NOT NULL (when intake was recorded)
  ✓ agent IS NOT NULL AND agent != ''
  ✓ actions_taken IS NOT NULL AND length(actions_taken) > 2
  ✓ files_read IS NOT NULL AND length(files_read) > 2
  ✓ files_changed IS NOT NULL (can be empty JSON "[]" if no changes)
  ✓ errors IS NOT NULL OR harness_friction IS NOT NULL
    (at least one must be present)

Detailed (score 3) — all of Standard plus:
  ✓ decisions_made IS NOT NULL AND length(decisions_made) > 2
  ✓ errors IS NOT NULL (explicit, even if "none")
  ✓ harness_friction IS NOT NULL (explicit, even if "none")
  ✓ duration_seconds IS NOT NULL
    OR notes contains explanation for missing duration
  ✓ token_estimate IS NOT NULL
    OR notes contains explanation for missing token estimate
```

The `notes` fallback for duration/token reflects TRACE_SPEC.md: "or a note
explaining why duration is unavailable."

**Example Output:**

```
$ scripts/bin/harness-cli score-trace
Trace #7 (latest):
  Tier achieved: standard (2/3)
  Lane: high_risk → required tier: detailed (3/3)
  ✗ BELOW REQUIREMENT

  Missing for detailed:
    - decisions_made: empty
    - duration_seconds: null (no explanation in notes)
    - token_estimate: null (no explanation in notes)

$ scripts/bin/harness-cli score-trace --id 5
Trace #5:
  Tier achieved: detailed (3/3)
  Lane: normal → required tier: standard (2/3)
  ✓ MEETS REQUIREMENT
```

**Acceptance Criteria:**

| # | Criterion | How to verify |
|---|-----------|---------------|
| 1 | `scripts/bin/harness-cli score-trace` (no args) scores the most recent trace in the database. | Create a harness DB with `init`, record a trace with `trace --summary "test task" --outcome completed`, run `score-trace`. Output shows the trace ID, tier achieved, and field breakdown. |
| 2 | `scripts/bin/harness-cli score-trace --id N` scores a specific trace by ID. | Record two traces, then `score-trace --id 1`. Output shows trace #1's score, not the latest. |
| 3 | A minimal trace (only summary + outcome) scores tier 1. | Record `trace --summary "short test task" --outcome completed`. `score-trace` outputs `minimal (1/3)`. |
| 4 | A standard trace (summary + outcome + agent + actions + files_read + files_changed + friction) scores tier 2. | Record a trace with all standard fields. `score-trace` outputs `standard (2/3)`. |
| 5 | A detailed trace (all standard fields + decisions + errors + friction + duration + tokens) scores tier 3. | Record a trace with all detailed fields. `score-trace` outputs `detailed (3/3)`. |
| 6 | When the trace has a linked `intake_id`, the command looks up the intake's `risk_lane` and compares the achieved tier against the lane requirement. | Record an intake with `--lane high_risk`, then record a standard trace with `--intake <id>`. `score-trace` output includes `Lane: high_risk → required tier: detailed` and `BELOW REQUIREMENT`. |
| 7 | When the trace has no linked intake, the command scores the tier but does not report a lane requirement. | Record a trace without `--intake`. `score-trace` outputs the tier but says `Lane: unknown (no linked intake)`. |
| 8 | The command lists specific missing fields when the trace is below its tier ceiling or below its lane requirement. | A standard trace missing `decisions_made` and `duration_seconds` lists both as missing for detailed tier. |
| 9 | `scripts/bin/harness-cli score-trace` exits with code 0 when the trace meets its lane requirement (or has no lane), and exits with code 1 when below requirement. | Allows scripted use: an agent can run `score-trace && echo OK || echo FIX`. |
| 10 | `cargo test` passes with unit tests covering all three tiers, the lane lookup, the missing-field output, and the exit code behavior. | Run `cargo test` in the workspace root. |
| 11 | TRACE_SPEC.md Review Checklist is updated to reference `score-trace` as a mechanical check. | Read TRACE_SPEC.md, confirm it says "run `scripts/bin/harness-cli score-trace` to verify the trace meets the lane requirement." |

**Lane:** Normal (new Rust CLI command, touches `interface.rs`, `application.rs`,
`infrastructure.rs`, and `domain.rs`; no schema migration).

---

### US-009: Enriched Friction Query

**Problem:** `scripts/bin/harness-cli query friction` currently shows 4 columns:
`id`, `created_at`, `task_summary`, `harness_friction`. This tells you *what*
the friction was but not *what kind of task* produced it. You can't tell whether
friction came from a tiny, normal, or high-risk task, or what type of work
triggered it.

**Evidence:** In the Phase 2 benchmark, 5 of 6 tasks recorded friction. But
looking at `query friction` output alone, you can't see that T4 (high-risk
authentication) and T1 (tiny project setup) had very different friction
profiles. Without the lane and input type, you're missing the context needed to
spot patterns like "high-risk tasks always have friction about missing decision
guidance" or "tiny tasks never have meaningful friction."

**What gets changed:**
1. `query friction` output gains two new columns: `risk_lane` and `input_type`,
   joined from the intake table via the trace's `intake_id`.
2. The `FrictionRecord` domain struct gains `risk_lane` and `input_type` fields.
3. The SQL query joins `trace` → `intake` to get these fields.

**New output format:**

```
id | created_at          | risk_lane  | input_type       | task_summary              | harness_friction
7  | 2026-05-28 11:44:00 | high_risk  | change_request   | Added auth middleware...   | Decision guidance unclear...
5  | 2026-05-28 11:30:00 | normal     | spec_slice       | Built folder support...    | Trace spec tier mapping...
3  | 2026-05-28 11:15:00 | tiny       | maintenance      | Set up project structure   | none
```

When a trace has no linked intake, the lane and type columns show `—` (dash).

**Acceptance Criteria:**

| # | Criterion | How to verify |
|---|-----------|---------------|
| 1 | `scripts/bin/harness-cli query friction` output includes `risk_lane` and `input_type` columns. | Create DB, record an intake (lane=normal, type=change_request), record a trace with friction linked to that intake. `query friction` output has 6 columns: id, created_at, risk_lane, input_type, task_summary, harness_friction. |
| 2 | Lane and type values come from the linked intake record via `intake_id`. | Verify the values match the intake, not the trace. |
| 3 | When a trace has no `intake_id`, the lane and type columns display `—`. | Record a trace with friction but no `--intake` flag. `query friction` shows `—` for both columns. |
| 4 | When a trace has `intake_id` but the intake was deleted (orphan), the columns display `—` without error. | Edge case: uses LEFT JOIN so missing intake doesn't crash. |
| 5 | The query still only returns traces where `harness_friction IS NOT NULL` (existing filter preserved). | Traces without friction are excluded, same as today. |
| 6 | `cargo test` passes with tests covering the joined output, the null-intake case, and the filter. | Run `cargo test` in the workspace root. |

**Lane:** Tiny (extends existing query with a JOIN, no new command, no schema
change).

---

## Out of Scope for Phase 3

| Item | Why deferred | Phase |
|------|-------------|-------|
| Schema migrations | The existing schema already has every column needed for US-008, US-009, US-011. No ALTER TABLE required. | — |
| Benchmark comparison enhancement (US-010) | Lives in `harness-benchmark`, not `repository-harness`. | Benchmark work |
| Story `verify_command` field (US-012) | Useful for code stories but requires schema migration. Add when Phase 4 needs batch verification. | Phase 4 |
| Review event schema (US-013) | No benchmark measurement, no observed pain point. | Phase 4+ |
| Installer propagation of Phase 2 docs | US-007 (already recorded as backlog item). Separate from Phase 3 scope. | Separate PR |
| Keyword-based friction attribution | Too noisy with small data sets. Revisit when friction entries exceed ~50. | Phase 4+ |
| Automated trace scoring during `trace` recording | Phase 3 makes scoring available as a separate command. Auto-scoring on write is a Phase 4 convenience. | Phase 4 |

---

## Implementation Sequence

```
Step 1: US-011 — Backlog outcome workflow
  - Update docs/HARNESS.md
  - Add --open and --closed filters to query backlog
  - Add glossary term
  - Write unit tests
  Estimated effort: ~2-3 hours

Step 2: US-008 — Trace quality scoring command
  - Add ScoreTraceResult to domain.rs
  - Add score_trace method to application.rs and infrastructure.rs
  - Add score-trace subcommand to interface.rs
  - Write unit tests for all three tiers + lane lookup
  - Update TRACE_SPEC.md review checklist
  Estimated effort: ~4-6 hours

Step 3: US-009 — Enriched friction query
  - Update FrictionRecord in domain.rs
  - Update query_friction SQL to LEFT JOIN intake
  - Update print_friction in interface.rs
  - Write unit tests
  Estimated effort: ~2-3 hours

Step 4: Cross-references and documentation
  - Update AGENTS.md if new commands need to be in reading list
  - Update HARNESS_COMPONENTS.md responsibility statuses
  - Update HARNESS_MATURITY.md current assessment
  - Record Phase 3 trace
  Estimated effort: ~1-2 hours
```

**Total estimated effort:** ~10-14 hours

---

## Execution Workflow

1. **Branch:** `git checkout -b feature/phase-3-active-observability main`
2. **Implement US-011 → US-008 → US-009** (in order)
3. **Update cross-references** (AGENTS.md, HARNESS_COMPONENTS.md,
   HARNESS_MATURITY.md, GLOSSARY.md)
4. **Run `cargo test`** — all tests must pass
5. **Run `cargo clippy`** — no warnings
6. **Run benchmark** in `harness-benchmark`:
   - Install harness from feature branch
   - Run `./benchmark/run.sh --agent codex --harness feature/phase-3-active-observability`
7. **Compare:** `./benchmark/compare.sh phase-2-observability-taxonomy phase-3`
8. **Merge:** Only when benchmark shows stable or improved results

---

## Expected Benchmark Deltas

| Metric | Phase 2 (current) | Phase 3 Target | Reasoning |
|--------|-------------------|----------------|-----------|
| Functional score | 37/37 (100%) | 37/37 (100%) | Phase 3 doesn't change app code |
| Harness compliance | 30/31 (96.7%) | 31/31 (100%) | `score-trace` helps agent catch T4's missing decision; backlog workflow encourages complete harness loop |
| Trace quality | 2.6 / 3.0 | 2.8-3.0 / 3.0 | Agent can self-check trace tier before closing; T4 should reach detailed |
| Lane accuracy | 6/6 | 6/6 | Already perfect |
| Wall time | 1816s | ~1850-1950s | Slight increase from running `score-trace` |
| Token cost | $20.72 | ~$21-22 | Slight increase from reading score output |

---

## What Would Signal Success

1. `cargo test` passes with coverage for all three stories.
2. `score-trace` correctly identifies T4-equivalent traces as below requirement.
3. `query friction` shows lane context alongside friction text.
4. `query backlog --open` and `--closed` filter correctly.
5. Benchmark trace quality rises to ≥2.8/3.0 (T4 reaches detailed).
6. Benchmark harness compliance reaches 31/31 (T4 decision_recorded passes).

## What Would Signal Failure

- `score-trace` disagrees with TRACE_SPEC.md rules (scoring logic is wrong).
- Benchmark trace quality stays at 2.6 (agent doesn't run `score-trace`).
- Benchmark harness compliance doesn't improve (T4 decision gap persists).
- `cargo test` or `cargo clippy` failures.
