# Agent Instructions

Add project-specific agent instructions here.

<!-- HARNESS:BEGIN -->

## Harness

This repo uses **Harness** — a durable, SQLite-backed operating framework for human-agent pair programming.

Before work, read:

- `README.md`
- `docs/HARNESS.md`
- `docs/FEATURE_INTAKE.md`
- `docs/ARCHITECTURE.md`
- All files in `.agents/rules/`

Use the Rust Harness CLI as the main operational tool. Run it through the stable repo-local entrypoint `scripts/harness`, which uses the prebuilt Rust binary at `scripts/bin/harness-cli`.

### Agent Workspace Structure (`.agents/`)

This directory is an enterprise-grade cognitive core containing:

- `.agents/rules/`: Authoritative operational guidelines (Harness, Scrum, Edge Case Hunter, Adversarial Review).
- `.agents/workflow/`: High-discipline execution workflows (`quick-dev.md`, `dev-story.md`, `code-review.md`).
- `.agents/skills/`: Comprehensive library of 44 BMad modular task skills.
- `.agents/agents/`: Repository of specialized personas categorized by discipline (Engineering, Product, etc.).
- `.agents/commands/`: Team coordination command configurations.

<!-- AGENT-SHIMS:BEGIN -->

## Claude Code

Before working in this repository:

1. Read all files in `.agents/rules/` for operating and validation rules
2. Run `scripts/harness query matrix` to see current validation status
3. Run `scripts/harness query stats` for project overview
4. Follow the mandatory task loop for every request

## Cursor

Before working in this repository:

1. Read all files in `.agents/rules/` for operating and validation rules
2. Reference `docs/FEATURE_INTAKE.md` for work classification
3. Use `scripts/harness query` commands for status checks
4. Follow the mandatory task loop for every request

## Windsurf

Before working in this repository:

1. Read all files in `.agents/rules/` for operating and validation rules
2. Reference `docs/FEATURE_INTAKE.md` for work classification
3. Use `scripts/harness query` commands for status checks
4. Follow the mandatory task loop for every request

## GitHub Copilot

Before suggesting code changes:

1. Read all files in `.agents/rules/` for operating and validation rules
2. Reference story packets in `docs/stories/` for implementation guidance
3. Use `scripts/harness query` commands for context
4. Always record traces upon task completion

## Antigravity

Before working in this repository:

1. Read all files in `.agents/rules/` for operating and validation rules
2. Run `scripts/harness query matrix` for validation status
3. Follow the mandatory task loop for every request

## Codex

Before working in this repository:

1. Read all files in `.agents/rules/` for operating and validation rules
2. Reference `docs/FEATURE_INTAKE.md` for work classification
3. Use `scripts/harness query` commands for context
4. Follow the mandatory task loop for every request

## All Other Agents

This repository uses Harness. Before working:

1. Read all files in `.agents/rules/` — these are the authoritative operating guides
2. Read `docs/HARNESS.md` for the human-agent collaboration model
3. Read `docs/FEATURE_INTAKE.md` for work classification
4. Use `scripts/harness` CLI for all task tracking

Treat harness docs as authoritative. Always record traces upon task completion. Report friction to backlog.

<!-- AGENT-SHIMS:END -->
<!-- HARNESS-SKILLS:BEGIN -->

## Available Harness Skills

Invoke a skill by reading its instructions: `.agents/skills/<name>/SKILL.md`
Or run directly: `./scripts/harness skill run <name>`

| Skill | Description |
| --- | --- |
| harness-advanced-elicitation | Push the LLM to reconsider, refine, and improve its recent output. Use when user asks for deeper critique or mentions a known deeper critique method, e.g. socratic, first principles, pre-mortem, red team. |
| harness-agent-analyst | Strategic business analyst and requirements expert. Use when the user asks to talk to Mary or requests the business analyst. |
| harness-agent-architect | System architect and technical design leader. Use when the user asks to talk to Winston or requests the architect. |
| harness-agent-dev | Senior software engineer for story execution and code implementation. Use when the user asks to talk to Amelia or requests the developer agent. |
| harness-agent-pm | Product manager for PRD creation and requirements discovery. Use when the user asks to talk to John or requests the product manager. |
| harness-agent-tech-writer | Technical documentation specialist and knowledge curator. Use when the user asks to talk to Paige or requests the tech writer. |
| harness-agent-ux-designer | UX designer and UI specialist. Use when the user asks to talk to Sally or requests the UX designer. |
| harness-brainstorming | Facilitate interactive brainstorming sessions using diverse creative techniques and ideation methods. Use when the user says help me brainstorm or help me ideate. |
| harness-check-implementation-readiness | Validate PRD, UX, Architecture and Epics specs are complete. Use when the user says "check implementation readiness". |
| harness-checkpoint-preview | LLM-assisted human-in-the-loop review. Make sense of a change, focus attention where it matters, test. Use when the user says "checkpoint", "human review", or "walk me through this change". |
| harness-correct-course | Manage significant changes during sprint execution. Use when the user says "correct course" or "propose sprint change" |
| harness-create-architecture | Create architecture solution design decisions for AI agent consistency. Use when the user says "lets create architecture" or "create technical architecture" or "create a solution design" |
| harness-create-epics-and-stories | Break requirements into epics and user stories. Use when the user says "create the epics and stories list" |
| harness-create-prd | DEPRECATED — consolidated into harness-prd create intent - this skill will be removed in v7 in favor of `harness-prd`. |
| harness-create-story | Creates a dedicated story file with all the context the agent will need to implement it later. Use when the user says "create the next story" or "create story [story identifier]" |
| harness-create-ux-design | Plan UX patterns and design specifications. Use when the user says "lets create UX design" or "create UX specifications" or "help me plan the UX" |
| harness-customize | Authors and updates customization overrides for installed Harness skills. Use when the user says 'customize harness', 'override a skill', 'change agent behavior', or 'customize a workflow'. |
| harness-distillator | Lossless LLM-optimized compression of source documents. Use when the user requests to 'distill documents' or 'create a distillate'. |
| harness-document-project | Document brownfield projects for AI context. Use when the user says "document this project" or "generate project docs" |
| harness-edit-prd | DEPRECATED — consolidated into harness-prd update intent - this skill will be removed in v7 in favor of `harness-prd`. |
| harness-editorial-review-prose | Clinical copy-editor that reviews text for communication issues. Use when user says review for prose or improve the prose |
| harness-editorial-review-structure | Structural editor that proposes cuts, reorganization, and simplification while preserving comprehension. Use when user requests structural review or editorial review of structure |
| harness-generate-project-context | Create project-context.md with AI rules. Use when the user says "generate project context" or "create project context" |
| harness-help | Analyzes current state and user query to answer Harness questions or recommend the next skill(s) to use. Use when user asks for help, harness help, what to do next, or what to start with in Harness. |
| harness-index-docs | Generates or updates an index.md to reference all docs in the folder. Use if user requests to create or update an index of all files in a specific folder |
| harness-investigate | Forensic case investigation with evidence-graded findings, calibrated to the input. Use when the user asks to investigate a bug, trace what caused an incident, walk through unfamiliar code, or build a mental model of a code area before working on it. |
| harness-party-mode | Orchestrates group discussions between installed HARNESS agents, enabling natural multi-agent conversations where each agent is a real subagent with independent thinking. Use when user requests party mode, wants multiple agent perspectives, group discussion, roundtable, or multi-agent conversation about their project. |
| harness-prd | Create, update, or validate a PRD. Use when the user wants help producing, editing, or validating a PRD. |
| harness-product-brief | Create, update, or validate a product brief. Use when the user wants help producing, editing, or validating a brief. |
| harness-qa-generate-e2e-tests | Generate end to end automated tests for existing features. Use when the user says "create qa automated tests for [feature]" |
| harness-retrospective | Post-epic review to extract lessons and assess success. Use when the user says "run a retrospective" or "lets retro the epic [epic]" |
| harness-shard-doc | Splits large markdown documents into smaller, organized files based on level 2 (default) sections. Use if the user says perform shard document |
| harness-technical-research | Conduct technical research on technologies and architecture. Use when the user says they would like to do or produce a technical research report |
| harness-validate-prd | DEPRECATED — consolidated into harness-prd validate intent - this skill will be removed in v7 in favor of `harness-prd`. |

<!-- HARNESS-SKILLS:END -->

<!-- HARNESS:END -->
