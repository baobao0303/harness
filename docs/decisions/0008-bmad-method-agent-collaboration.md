# 0008 BMAD Method Agent Collaboration

Date: 2026-05-23

## Status

Accepted

## Context

Harness v0 provides a robust database and stable local entrypoints to track, validate, and audit agent-driven repository changes. However, as task complexity scales, a single unstructured prompt session can lead to:
- Ambiguous developer-AI coordination splits.
- Architectural design drift because implementation begins before the contract is fully vetted.
- Inconsistent context engineering patterns in story documents.

The BMAD Method (Breakthrough Method for Agile AI-Driven Development) is an open-source framework that introduces structured, persona-based roles (Analyst, Architect, Developer, QA, Scrum Master) to manage agile development gates. Integrating BMAD Method directly with our Harness durable layer solves these coordination issues.

## Decision

Adopt the BMAD Method as our official Agent Collaboration and Role-Based Planning framework.

Specifically:
1. Map the core BMAD personas to our Harness task loop stages:
   - **BMAD Analyst / Product Manager**: Owns the *Intake & Discovery* phase. Responsible for writing the `spec-intake.md` and logging the initial `scripts/harness intake` record.
   - **BMAD Architect**: Owns the *System Vetting & Design* phase. Responsible for updating `docs/product/` contracts, drafting Architecture Decision Records (ADRs) in `docs/decisions/`, and setting up structural verification boundaries.
   - **BMAD Scrum Master**: Coordinates the *Story & Planning* phase. Responsible for declaring story packets in `docs/stories/`, and registering them in the proof matrix using `scripts/harness story add`.
   - **BMAD Developer**: Owns the *Code Implementation* phase. Responsible for writing vertical-slice feature code and running local verification commands.
   - **BMAD QA**: Owns the *Compliance & Verification* phase. Responsible for checking the proof matrix and logging execution evidence via `scripts/harness story update`.
2. Append the official BMAD Role Instructions to the authoritative agent operating rules (`.agents/rules/harness.md`).

## Alternatives Considered

1. **Keep unstructured single-persona agent loops**: Rejected because complex features require distinct gates for requirements, architecture, and QA which a single generic persona conflates.

## Consequences

Positive:

- Clearly delineated responsibilities for AI agents, making multi-phase development highly organized and predictable.
- Direct alignment of agile product management roles with a durable, verifiable database trace loop.
- Enforced architectural separation of design-before-code and test-before-merge.

Tradeoffs:

- Adds a small conceptual overhead for agents, requiring them to read and adopt their active BMAD role before commencing work.

## Follow-Up

- Incorporate BMAD role guidelines into the `AGENTS.md` instructions.
- Automatically scan and seed this decision record into `harness.db` using brownfield imports.
