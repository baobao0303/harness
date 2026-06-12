# Workflow: Quick Task

Use this workflow for **tiny or normal** lane work that does not require a story file.
Typical examples: copy fixes, doc clarifications, config tweaks, narrow refactors.

---

## When to use

- 0–1 risk flags from `docs/FEATURE_INTAKE.md`
- No auth, no data model change, no public API change
- Blast radius is one file or one isolated function

---

## Steps

### 1. Classify

Read the request and run the risk checklist from `docs/FEATURE_INTAKE.md`.

```bash
# Record the classification
scripts/bin/harness-cli intake \
  --type change-request \
  --summary "<one-line description>" \
  --lane tiny
```

### 2. Locate

Find the affected file(s). Do not change anything outside the stated scope.

### 3. Implement

Make the minimal change. Prefer the smallest diff that satisfies the request.

### 4. Validate

Run whatever quick checks exist:

```bash
# Run format, lint, typecheck, unit tests if available
validate:quick
```

If no validation commands exist yet, note that in the trace.

### 5. Record trace

```bash
scripts/bin/harness-cli trace \
  --summary "<what changed>" \
  --outcome success \
  --agent "<your agent name>"
```

### 6. Done check

- [ ] Change is complete or blocker is documented
- [ ] Affected docs are still accurate
- [ ] Trace is recorded
- [ ] Friction logged if any confusion was found

---

## Output format

End your response with:

```
Done. Changed: <file or area>.
Validation: <what was run or why it was skipped>.
Trace: recorded.
```
