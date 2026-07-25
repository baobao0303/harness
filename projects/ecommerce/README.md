# Harness 🚀 — Autonomous AI Agent Operating Framework & Loop Engine

> **Turn any codebase into an autonomous, sub-agent-driven workspace powered by Loop Engineering & Chief of Staff Orchestration.**

`harness` is a repository-level operating framework for **Claude Code, Antigravity, Cursor, Windsurf, GitHub Copilot, Codex**, and custom AI Agents. It provides a durable layer (`harness.db`), sub-agent topology allocation, Git worktree isolation, dynamic skill resolution, and live `tldraw` visual telemetry.

---

## 🌟 Architectural Highlights

### 1. 👑 Chief of Staff (Mina) & Sub-Agent Ecosystem
Rather than relying on a single monolithic LLM prompt, Harness splits execution across **6 specialized sub-agents**:

| Sub-Agent Role | Model Tier | Responsibility | Working Environment |
| :--- | :--- | :--- | :--- |
| **👑 Chief of Staff (Mina)** | `pro` | Strategic vision, task decomposition, progressive disclosure memory. | Main Workspace |
| **💻 Implementer (Coder)** | `pro` | Code writing, bug fixes, feature implementation. | `.worktrees/task-<id>` |
| **🧪 Verifier (QA)** | `flash` | Running build/test verification, packaging error tracebacks. | `.worktrees/task-<id>` |
| **🔍 Explorer (Navigator)** | `flash` | Codebase exploration, symbol discovery, context window compression. | Read-Only Subagent |
| **📈 Triage Monitor** | `flash_lite` | Background task monitoring, recurring health checks. | Background Cron |
| **🛡️ Skill Auditor** | `pro` | Skill verification, pulling skills from Remote Skill Server. | Global Registry |

---

### 2. 🔁 5 Loop Engineering Building Blocks

1. **System & Prompt Infrastructure**: Structured `AGENTS.md` shim and progressive disclosure docs in `docs/`.
2. **Model Selection & Topology**: Cost/speed balancing by routing deep tasks to `pro` tier and quick lookups to `flash` tier.
3. **Context Window Management**: Compressed context indices avoiding token bloat.
4. **Tools & Execution Engine**: Headless Rust CLI (`harness-cli`), Herdr PTY Terminal Multiplexer, and Git Worktree isolation.
5. **Verification, Evaluation & Governance**: Automated verification commands (`harness story verify`), circuit breaker retries, and SQLite telemetry (`harness.db`).

---

### 3. 🎨 Visual Telemetry & Live `tldraw` Diagramming

Harness integrates directly with the **tldraw Desktop App** (via local HTTP API at `http://localhost:7236`) and [offline.tldraw.com](https://offline.tldraw.com):
- **Live Diagram Rendering**: Agents programmatically draw multi-agent topology boxes, arrows, and status badges on your active tldraw canvas.
- **Trace Export**: Convert SQLite execution traces into `.tldr` JSON file snapshots with `./scripts/harness export-trace --format tldraw --out trace.tldr`.

---

## 🚀 Quickstart & Setup Guide

### 1. Environment Configuration

Copy the `.env.example` template to `.env` to configure your environment variables:
```bash
cp .env.example .env
```

Default `.env` settings:
```env
HARNESS_SKILL_SERVER="https://skills-hub.yourdomain.com/api/v1"
HARNESS_SKILL_SERVER_TOKEN=""
HARNESS_MODEL="gemini-3.6-flash"
HARNESS_DB="./harness.db"
```

> [!IMPORTANT]
> Never commit `.env` to Git. `.env` is listed in `.gitignore`.

---

### 2. Compile & Initialize Harness

Build the fast release binary into `scripts/bin/harness-cli` and initialize the SQLite database:

```bash
# Compile standalone release binary (3.2MB)
mkdir -p scripts/bin && cargo build --release --bin harness-cli && cp target/release/harness-cli scripts/bin/harness-cli

# Initialize database schema
./scripts/harness init
```

Verify that the CLI is operating instantaneously:
```bash
./scripts/harness audit
```
*(Expected output: Entropy score `0/100`)*

---

## 🛠️ Complete CLI Command Reference

All commands are executed via `./scripts/harness`:

### 👑 Sub-Agent & Skill Orchestration
```bash
# List active sub-agent topologies
./scripts/harness subagent list

# Spawn a sub-agent with role, model tier, and prompt
./scripts/harness subagent spawn --role "Implementer" --model "pro" --prompt "Build feature X"

# Find best matching skill for an intent
./scripts/harness skill find "generate e2e tests"

# Sync skills from Remote Skill Server
./scripts/harness skill sync
```

### 🌳 Git Worktree Isolation
```bash
# Spawn isolated environment for task-101
./scripts/harness worktree spawn --task 101

# Remove worktree after completion
./scripts/harness worktree remove --task 101
```

### 🎨 Visual Telemetry & Export
```bash
# Export traces to tldraw JSON snapshot
./scripts/harness export-trace --format tldraw --out diagram.tldr

# Export traces to Mermaid flowchart
./scripts/harness export-trace --format mermaid
```

### ⚙️ Governance & Audit
```bash
# Run codebase drift audit and entropy score
./scripts/harness audit

# View harness configuration
./scripts/harness config list

# Query test matrix & backlog
./scripts/harness query matrix
./scripts/harness query backlog
```

---

## 📊 End-to-End Task Execution Flow

```text
               👤 Human Strategic Goal
                         │
                         ▼
             👑 Chief of Staff (Mina)
             (Decomposes to Stories & Specs)
                         │
         ┌───────────────┼───────────────┐
         ▼               ▼               ▼
   💻 Implementer   🧪 Verifier     🔍 Explorer
   (.worktrees/)   (Cargo Test)   (Read-Only Search)
         │               │               │
         └───────────────┼───────────────┘
                         ▼
             🗄️ Durable Layer (harness.db)
                         │
                         ▼
          🎨 Live tldraw Telemetry Canvas
```

1. **Intake**: Human submits task ➔ Chief of Staff classifies risk lane (`tiny`, `normal`, `high-risk`) via `./scripts/harness intake`.
2. **Isolation**: Chief of Staff spawns Implementer sub-agent inside an isolated Git worktree: `./scripts/harness worktree spawn --task <id>`.
3. **Execution**: Implementer writes code; Explorer navigates symbols.
4. **Verification**: Verifier runs `./scripts/harness story verify --id <id>`. If tests fail, stderr traceback is automatically packaged into auto-feedback prompt for self-correction.
5. **Telemetry**: Traces are recorded into `harness.db` and rendered on **tldraw Desktop canvas** via `./scripts/harness export-trace --format tldraw`.

---

## 📄 Documentation Index

- [spec.md](file:///Users/bao312/Desktop/harness/spec.md) — Full System Architecture Manual & Technical Specification.
- [AGENTS.md](file:///Users/bao312/Desktop/harness/AGENTS.md) — Agent Instructions & Shim.
- [docs/HARNESS.md](file:///Users/bao312/Desktop/harness/docs/HARNESS.md) — Human-AI Collaboration Guide.
- [docs/FEATURE_INTAKE.md](file:///Users/bao312/Desktop/harness/docs/FEATURE_INTAKE.md) — Feature Intake & Risk Lanes.
- [docs/ARCHITECTURE.md](file:///Users/bao312/Desktop/harness/docs/ARCHITECTURE.md) — Architecture & Boundary Rules.
