---
inclusion: auto
---

# Harness Operating Rules

This repository uses **Harness** — a durable, SQLite-backed operating framework for human-agent pair programming.

## Quick Commands

```bash
./scripts/harness query stats       # Summary counts
./scripts/harness query matrix      # Test verification status
./scripts/harness query traces      # Agent execution logs
./scripts/harness query friction    # Development blockages
./scripts/harness query backlog     # Improvement list
./scripts/harness skill list        # Available skills
```

## Task Loop

1. Classify: `./scripts/harness intake --type <type> --summary "<text>" --lane <lane>`
2. Implement within lane constraints
3. Trace (MANDATORY): `./scripts/harness trace --summary "<task>" --outcome <outcome> --agent kiro`

## Invoke Skills

Use `./scripts/harness skill list` to see all available skills.
Skills are invoked by following the instructions in `.agents/skills/<name>/SKILL.md`.
