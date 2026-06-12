# Workflow: New Feature Intake

Use this workflow when the human provides a **new feature request, spec, or initiative**.
This workflow produces harness-ready artifacts before any implementation starts.

---

## When to use

- A new product spec or feature idea arrives
- A large initiative needs to be broken into stories
- The current product docs do not cover the requested area

---

## Steps

### 1. Understand intent

Read the request fully. Do not start classifying until you understand:

- What the user wants to happen (behavior)
- Who the user is (role, persona, or surface)
- What success looks like (acceptance signal)

If the request is unclear, ask one focused question. Do not ask multiple questions at once.

### 2. Classify input type

Use `docs/FEATURE_INTAKE.md` to pick the input type:

| Input type | Signal |
| --- | --- |
| New spec | Covers an entirely new product area |
| Spec slice | One behavior from an existing or new spec |
| Change request | Refines or fixes existing behavior |
| New initiative | Multiple stories, multiple domains |
| Maintenance | Technical, operational, or dependency work |
| Harness improvement | Process, template, or agent-instruction change |

### 3. Run risk checklist

Score all 10 flags from `docs/FEATURE_INTAKE.md`.
Choose a lane: tiny, normal, or high-risk.

```bash
scripts/bin/harness-cli intake \
  --type <input-type> \
  --summary "<one-line description of the feature>" \
  --lane <lane>
```

### 4. Update product docs

Identify the correct product doc in `docs/product/`.
If no doc exists for this area, create one.

Product docs describe **current accepted behavior**, not future plans.
Write only what is now true after this feature lands.

### 5. Create story packet(s)

For each behavior unit:

- Copy `docs/templates/story.md` to `docs/stories/<story-id>.md`
- Fill in: product contract, acceptance criteria, design notes, validation expectations
- Link the story to the affected product doc

For high-risk work, use `docs/templates/high-risk-story/` instead.

### 6. Add to matrix

Register each story in the durable layer:

```bash
scripts/bin/harness-cli story add \
  --id <story-id> \
  --title "<story title>" \
  --lane <lane>
```

### 7. Record decision if needed

If this feature requires a meaningful architecture or contract choice:

```bash
# Copy docs/templates/decision.md to docs/decisions/NNNN-<slug>.md
# Fill in context, options considered, decision, and consequences
```

### 8. Record trace

```bash
scripts/bin/harness-cli trace \
  --summary "intake complete for <feature name>: <N> stories created" \
  --outcome success \
  --agent "<your agent name>"
```

---

## Output format

End intake with a summary:

```
Intake complete.
Input type: <type>
Lane: <lane>
Risk flags: <list>
Product docs updated: <list>
Stories created: <list of story IDs>
Decisions recorded: <list or none>
Next step: implement <first story id> or wait for human to prioritize.
```
