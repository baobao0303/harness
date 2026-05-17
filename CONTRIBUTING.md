# Contributing to harness-experimental

Thanks for helping improve the harness.

This repository is early. The most valuable contributions are practical patterns
that make coding agents safer, clearer, and easier to steer in real projects.

## Good Contribution Types

### 1. Real-world harness examples

Show how you installed or adapted the harness in a real project:

- What kind of project is it?
- Which agent/tool did you use? Claude Code, Codex, Cursor, something else?
- What did the harness help with?
- What was missing or confusing?

### 2. Agent failure cases

Share cases where an agent made a bad change because the repo lacked context:

- What did you ask the agent to do?
- What did it misunderstand?
- Which harness artifact could have prevented the issue?
- Can the lesson become a template, rule, or validation expectation?

### 3. Template improvements

Improve files in `docs/templates/` when you find a repeatable pattern for:

- product specs
- story packets
- decision records
- validation plans
- agent operating rules
- high-risk change reviews

### 4. Validation patterns

Add or refine expectations in `docs/TEST_MATRIX.md` for common stacks and work
types. The goal is not only "tests pass". The goal is clear proof that the work
matches the product contract.

### 5. Documentation clarity

If a concept is hard to understand, improve the explanation. Small docs changes
are welcome.

## Before Opening a Pull Request

1. Read `AGENTS.md`.
2. Classify the work using `docs/FEATURE_INTAKE.md`.
3. Keep changes focused and reviewable.
4. Update related docs if you change a harness rule or template.
5. Explain what proof shows the change is useful.

## Pull Request Checklist

Include this in your PR description:

```markdown
## Summary
-

## Type of contribution
- [ ] Real-world harness example
- [ ] Agent failure case
- [ ] Template improvement
- [ ] Validation pattern
- [ ] Documentation clarity
- [ ] Other

## Proof / validation
-

## Follow-up questions
-
```

## What Not To Add Yet

Avoid adding project-specific product specs to this harness unless they are part
of a clearly marked demo or example. This repo should stay reusable across many
projects.

Avoid adding tool-specific rules that only work for one coding agent unless the
tradeoff is explained and the generic harness behavior remains clear.
