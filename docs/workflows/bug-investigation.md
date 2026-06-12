# Workflow: Bug Investigation

Use this workflow when a bug is reported or unexpected behavior is discovered.
Do not jump to fixes until the root cause is understood.

---

## When to use

- A user reports unexpected behavior
- A test fails unexpectedly
- An agent produces incorrect output
- A log or trace shows an error

---

## Steps

### 1. Record the bug as intake

```bash
scripts/bin/harness-cli intake \
  --type change-request \
  --summary "bug: <one-line description of symptom>" \
  --lane normal
```

### 2. Reproduce

Before reading code:

1. Write down the exact steps to reproduce the bug
2. Write down the expected behavior
3. Write down the actual behavior

If you cannot reproduce it, say so. Do not guess what the bug is.

### 3. Narrow the blast radius

Answer these questions:

- Which product domain is affected?
- Which layer is the defect in (domain, application, infrastructure, interface)?
- Is this a data problem, a logic problem, or a contract problem?
- Does it affect only one user, or all users?

### 4. Trace the code path

Read the relevant code path from the entry point to the failure:

```text
interface layer → application layer → domain layer → infrastructure layer
```

Find the exact line or function where the behavior diverges from spec.

### 5. Check the product doc

Read the relevant section of `docs/product/` that describes the behavior.

- If the code diverges from the product doc → bug in code
- If the product doc is wrong → fix the doc, then decide if code needs to change
- If neither is clear → ask the human for clarification

### 6. Fix

Fix only what caused the bug. Do not refactor surrounding code.

### 7. Add regression test

If no test caught this bug, add one. The test name should describe the failure scenario:

```
it("should not allow <symptom> when <condition>")
```

### 8. Validate

```bash
validate:quick
test:integration    # if the bug involved cross-layer behavior
```

### 9. Update story or record friction

If the bug reveals a missing acceptance criterion in an existing story:

```bash
scripts/bin/harness-cli story update \
  --id <story-id> \
  --status implemented \
  --evidence "regression test added: <test name>"
```

If the bug reveals a harness gap (missing rule, missing template, unclear doc):

```bash
scripts/bin/harness-cli backlog add \
  --title "<short name>" \
  --pain "<what was hard or missing>"
```

### 10. Record trace

```bash
scripts/bin/harness-cli trace \
  --summary "fixed bug: <description>. Root cause: <cause>" \
  --outcome success \
  --agent "<your agent name>"
```

---

## Output format

End your response with:

```
Bug: <symptom>
Root cause: <exact cause>
Fix: <what changed>
Regression test: <test name or "none added — reason">
Validation: <commands run>
Trace: recorded.
```
