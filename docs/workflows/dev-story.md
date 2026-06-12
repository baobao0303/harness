# Workflow: Dev Story

Use this workflow to implement a **normal** lane story from start to done.
Follow this sequence for every story-sized behavior change.

---

## When to use

- 1–3 risk flags from `docs/FEATURE_INTAKE.md`
- A story file exists or needs to be created
- Blast radius is bounded to one product domain

---

## Steps

### 1. Intake

Read the request and classify it.

```bash
scripts/bin/harness-cli intake \
  --type spec-slice \
  --summary "<story goal in one sentence>" \
  --lane normal
```

### 2. Read context

Before touching code:

1. Read the story file at `docs/stories/<story-id>.md`
2. Read referenced product docs in `docs/product/`
3. Check current validation status:

```bash
scripts/bin/harness-cli query matrix
scripts/bin/harness-cli query stats
```

### 3. Clarify scope

If any acceptance criterion is ambiguous, stop and ask.
Do not guess scope — guessing leads to wasted work and broken contracts.

### 4. Implement

Work inside the lane boundaries:

- Implement the smallest vertical slice that satisfies the acceptance criteria
- Add unit tests for new domain logic
- Add integration tests for any new API surface
- Do not change behavior outside the story scope

### 5. Validate

```bash
# Run in this order — stop on first failure
validate:quick          # format, lint, typecheck, unit tests
test:integration        # backend, database checks
test:e2e                # user-visible flows (if applicable)
```

If a validation command does not exist yet, document what was run manually.

### 6. Update story

Update the story file with:

- Status: `implemented`
- Evidence: commands run, test output, or links

```bash
scripts/bin/harness-cli story update \
  --id <story-id> \
  --status implemented \
  --evidence "<validation commands that passed>"
```

### 7. Record trace

```bash
scripts/bin/harness-cli trace \
  --summary "<what was built>" \
  --outcome success \
  --agent "<your agent name>"
```

### 8. Done check

- [ ] All acceptance criteria are met
- [ ] Story file status is updated to `implemented`
- [ ] Validation commands passed (or absence is documented)
- [ ] Product docs are still accurate
- [ ] Decision recorded if architecture or contracts changed
- [ ] Trace is recorded
- [ ] Friction logged if confusion was found

---

## Output format

End your response with:

```
Done. Story: <story-id>.
Implemented: <what changed>.
Validation: <commands run and results>.
Trace: recorded.
```
