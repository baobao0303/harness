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
# Compile standalone release binary
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

All commands are executed via `./scripts/harness` (which wraps `scripts/bin/harness-cli`):

### 🔍 Database & Query Commands (`harness query`)

Dùng để tra cứu toàn bộ trạng thái hoạt động lưu trong SQLite `harness.db`:

| Lệnh Query | Chức năng / Mô tả |
| :--- | :--- |
| `./scripts/harness query stats` | Hiển thị tóm tắt tổng số lượng records của tất cả các bảng. |
| `./scripts/harness query matrix` | Tra cứu ma trận kiểm chứng (Test Matrix) & danh sách Stories. |
| `./scripts/harness query backlog` | Xem danh sách các đề xuất cải tiến Backlog (Harness Backlog). |
| `./scripts/harness query decisions` | Tra cứu danh sách các Quyết định Kiến trúc (ADRs / Decisions). |
| `./scripts/harness query intakes` | Tra cứu các yêu cầu công việc mới đã phân loại rủi ro (Intakes). |
| `./scripts/harness query traces` | Tra cứu nhật ký dấu vết hoạt động thực thi (Execution Traces) của Agent. |
| `./scripts/harness query friction` | Hiển thị danh sách các trace gặp khó khăn/ma sát trong quá trình chạy. |
| `./scripts/harness query tools` | Hiển thị danh sách công cụ machine-readable đã đăng ký. |
| `./scripts/harness query interventions` | Xem lịch sử can thiệp của Human / CI / Reviewer. |
| `./scripts/harness query sql "<SQL>"` | Chạy truy vấn SQL trực tiếp tùy chỉnh trên SQLite `harness.db`. |

#### Truy vấn trực tiếp bằng `sqlite3`:
```bash
# Xem danh sách các bảng trong database
sqlite3 harness.db ".tables"

# Xem danh sách tất cả các stories trong Test Matrix
sqlite3 harness.db "SELECT id, title, risk_lane, status FROM story;"

# Xem lịch sử trace công việc
sqlite3 harness.db "SELECT id, task_summary, agent, outcome FROM trace;"
```

---

### 👑 Sub-Agent & Skill Orchestration
```bash
# List active sub-agent topologies
./scripts/harness subagent list

# Spawn a sub-agent with role, model tier, and prompt
./scripts/harness subagent spawn --role "Implementer" --model "pro" --prompt "Build feature X"

# List all available skills
./scripts/harness skill list

# Find best matching skill for an intent
./scripts/harness skill find "generate e2e tests"

# Sync skills from Remote Skill Server
./scripts/harness skill sync
```

---

### 🌳 Git Worktree Isolation
```bash
# Spawn isolated environment for task-101
./scripts/harness worktree spawn --task 101

# Remove worktree after completion
./scripts/harness worktree remove --task 101
```

---

### 🎨 Visual Telemetry & Trace Export
```bash
# Export traces to tldraw JSON snapshot
./scripts/harness export-trace --format tldraw --out diagram.tldr

# Export traces to Mermaid flowchart
./scripts/harness export-trace --format mermaid
```

---

### ⚙️ Governance, Audit & Work Registration
```bash
# Run codebase drift audit and entropy score
./scripts/harness audit

# View harness configuration
./scripts/harness config list

# Register new feature intake
./scripts/harness intake --type new_spec --summary "Feature description" --lane normal

# Add new story packet
./scripts/harness story add --id US-101 --title "User Login" --lane normal

# Update story status & evidence
./scripts/harness story update --id US-101 --status implemented --evidence "Cargo test passed"
```

---

## 🧠 Thư viện Kỹ năng (Skills Library) & Tích hợp IDE

Harness đi kèm hệ thống **Skills** chuẩn hóa cho AI Agent. Mỗi skill định nghĩa quy trình chi tiết giúp Agent thực hiện tác vụ chính xác.

### Danh sách Skill theo giai đoạn

| Giai đoạn | Skill | Mô tả |
| :--- | :--- | :--- |
| **Khởi đầu** | `harness-help` | Phân tích trạng thái và gợi ý skill tiếp theo |
| | `harness-document-project` | Tạo tài liệu dự án cho AI context |
| | `harness-generate-project-context` | Tạo `project-context.md` |
| **Yêu cầu** | `harness-prd` | Tạo, sửa, hoặc validate PRD |
| | `harness-product-brief` | Tạo product brief |
| | `harness-advanced-elicitation` | Phê bình sâu (socratic, red team, pre-mortem) |
| | `harness-brainstorming` | Brainstorm ý tưởng |
| **Kiến trúc** | `harness-create-architecture` | Thiết kế kiến trúc hệ thống |
| | `harness-technical-research` | Nghiên cứu kỹ thuật |
| **Lập kế hoạch** | `harness-create-epics-and-stories` | Chia nhỏ requirements thành epics/stories |
| | `harness-create-story` | Tạo story file chi tiết |
| **Thiết kế** | `harness-create-ux-design` | Thiết kế UX/UI |
| **Triển khai** | `harness-check-implementation-readiness` | Kiểm tra sẵn sàng implement |
| | `harness-correct-course` | Điều chỉnh sprint khi có thay đổi |
| **Kiểm thử** | `harness-qa-generate-e2e-tests` | Tạo E2E tests tự động |
| **Review** | `harness-retrospective` | Retrospective sau epic |
| | `harness-checkpoint-preview` | Human-in-the-loop review |
| **Tài liệu** | `harness-index-docs` | Tạo index cho thư mục docs |
| | `harness-shard-doc` | Chia nhỏ tài liệu lớn |
| | `harness-distillator` | Nén tài liệu cho LLM |
| **Đặc biệt** | `harness-party-mode` | Multi-agent roundtable discussion |
| | `harness-investigate` | Điều tra bug forensic |
| | `harness-customize` | Tùy chỉnh skill behavior |

### Cách gọi Skill từ các IDE

Tất cả IDE đều phát hiện các skill thông qua tệp discovery tương ứng trong dự án:

| IDE | Cách kích hoạt | Discovery Format |
| :--- | :--- | :--- |
| **Kiro** | Gõ `#` trong ô Chat → Chọn skill | `.kiro/steering/*.md` |
| **Cursor** | Gõ `@` hoặc xem bảng Rules | `.cursor/rules/*.mdc` |
| **Windsurf** | Agent tự động phát hiện | `.windsurfrules` |
| **Claude Code** | Nhắc tên skill trong prompt | `AGENTS.md` |
| **GitHub Copilot** | Nhắc tên skill trong prompt | `AGENTS.md` |
| **CLI** | `harness skill list` / `harness skill find` | Terminal |

### Khởi tạo Skill Discovery Files cho IDE

Chạy script sau để tự động sinh file discovery cho toàn bộ IDE trong dự án:

```bash
scripts/install-ide-skills.sh
```

---

## 📥 Tùy chọn cài đặt & Cập nhật Harness

### 1. Cài đặt vào Dự án (Local Installation)

Tích hợp Harness trực tiếp vào codebase dự án hiện tại của bạn:

```bash
curl -fsSL "https://raw.githubusercontent.com/baobao0303/harness/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --yes
```

#### Các tùy chọn cập nhật:
- **Cập nhật bảo toàn (`--merge`)**: Tải bổ sung file mới mà không ghi đè tài liệu hiện có.
  ```bash
  curl -fsSL "https://raw.githubusercontent.com/baobao0303/harness/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --merge --yes
  ```
- **Ghi đè hoàn toàn (`--override`)**: Sao lưu thư mục cũ và cài mới hoàn toàn.
  ```bash
  curl -fsSL "https://raw.githubusercontent.com/baobao0303/harness/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --override --yes
  ```
- **Refresh Agent Shim (`--refresh-agent-shim`)**: Cập nhật tệp `AGENTS.md` theo chuẩn shim mới nhất.

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

1. **Intake**: Human submits task ➔ Chief of Staff classifies risk lane (`tiny`, `normal`, `high_risk`) via `./scripts/harness intake`.
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
