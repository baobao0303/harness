# Workflow: Code Review

Use this workflow to review a pull request or diff for correctness, safety, and harness compliance.

---

## When to use

- Reviewing code before merge
- Auditing an agent's work after implementation
- Checking that a story's acceptance criteria are met by actual code

---

## Steps

### 1. Read the story

Find the story file linked to this PR.
Read the acceptance criteria and validation expectations.

```bash
# Check current story status
scripts/bin/harness-cli query matrix
```

### 2. Check scope

Verify the diff only touches what the story requires.

Flag scope creep if the diff changes:
- Files not mentioned in the story
- Behavior outside the acceptance criteria
- Test coverage that was not required

### 3. Check correctness

For each acceptance criterion, verify:

- [ ] The criterion is met by code in the diff
- [ ] The criterion is covered by at least one test
- [ ] The test name or assertion clearly maps to the criterion

### 4. Check safety

Run the risk checklist from `docs/FEATURE_INTAKE.md` against the actual diff.

Watch for hidden risk flags:
- Auth or session handling changes
- New or removed database columns
- API response shape changes
- Removed validation

### 5. Check architecture

Verify the diff follows `docs/ARCHITECTURE.md`:

- [ ] Inner layers do not import from outer layers
- [ ] Unknown inputs are parsed at boundaries
- [ ] Commands and queries are separated if both exist
- [ ] New domain concepts use meaningful types, not raw strings

### 6. Check harness

- [ ] Story file is updated with evidence
- [ ] No new friction patterns were left undocumented
- [ ] If architecture changed, a decision record exists or is proposed

### 7. Record trace

```bash
scripts/bin/harness-cli trace \
  --summary "reviewed <story-id>: <outcome summary>" \
  --outcome success \
  --agent "<your agent name>"
```

---

## Output format

End your review with:

```
Review: <story-id>
Result: approved | needs-changes | blocked
Criteria covered: <N>/<total>
Flags: <any risk or scope issues found>
```
