# Harness Technical Specification & Architecture Manual 🚀
> **Project**: Harness — Repository-Level Operating Framework & Autonomous Loop Engineering System
> **Version**: 1.0.0
> **Status**: Active Architecture Specification

---

## 📋 Table of Contents
1. [System Overview & Strategic Vision](#1-system-overview--strategic-vision)
   - 1.1 [Executive Summary](#11-executive-summary)
   - 1.2 [Problem Statement: Prompt Engineering vs. Loop Engineering](#12-problem-statement-prompt-engineering-vs-loop-engineering)
   - 1.3 [The 4-Tier Agentic Engineering Hierarchy](#13-the-4-tier-agentic-engineering-hierarchy)
   - 1.4 [Architectural Separation Rationales (Lý do & Triết lý Tách biệt Kiến trúc)](#14-architectural-separation-rationales-l%C3%BD-do--tri%E1%BA%BFt-l%C3%BD-t%C3%A1ch-bi%E1%BB%87t-ki%E1%BA%BFt-tr%C3%BAc)
   - 1.5 [Unified Loop Engineering & Chief of Staff Synergy Matrix](#15-unified-loop-engineering--chief-of-staff-synergy-matrix)
2. [System Architecture & Dual-Loop Mechanics](#2-system-architecture--dual-loop-mechanics)
3. [Sub-Agent Ecosystem & Topologies](#3-sub-agent-ecosystem--topologies)
   - 3.1 [Topology A: Implementer ➔ Verifier (Maker / Checker Split)](#31-topology-a-implementer--verifier-maker--checker-split)
   - 3.2 [Topology B: Explorer ➔ Implementer](#32-topology-b-explorer--implementer)
   - 3.3 [Topology C: Triage-Only Agent](#33-topology-c-triage-only-agent)
   - 3.4 [Topology D: Chief of Staff Orchestrator (Mina Pattern)](#34-topology-d-chief-of-staff-orchestrator-mina-pattern)
   - 3.5 [Model Selection & Sub-Agent Dispatch Protocol via `./scripts/harness`](#35-model-selection--sub-agent-dispatch-protocol-via-scriptsharness)
   - 3.6 [Sub-Agent Skill Discovery & Resolution Protocol (`harness skill find / search`)](#36-sub-agent-skill-discovery--resolution-protocol-harness-skill-find--search)
   - 3.7 [Remote Skill Server & Centralized Registry Architecture (`harness skill sync / pull`)](#37-remote-skill-server--centralized-registry-architecture-harness-skill-sync--pull)
4. [Git Worktree Isolation & Runtime Management](#4-git-worktree-isolation--runtime-management)
   - 4.1 [Worktree Directory Structure](#41-worktree-directory-structure)
   - 4.2 [Runtime Environment Isolation](#42-runtime-environment-isolation)
   - 4.3 [Repository-Level Memory Indexing](#43-repository-level-memory-indexing)
   - 4.4 [Automatic Lifecycle Pruning](#44-automatic-lifecycle-pruning)
   - 4.5 [Herdr Terminal Agent Multiplexer Integration (`herdr` CLI & Socket API)](#45-herdr-terminal-agent-multiplexer-integration-herdr-cli--socket-api)
5. [Loop Execution Engine & Verification Proof Loop](#5-loop-execution-engine--verification-proof-loop)
6. [Durable Memory & Observability (`harness.db`)](#6-durable-memory--observability-harnessdb)
   - 6.1 [Relational Schema Architecture](#61-relational-schema-architecture)
   - 6.2 [Visual Telemetry & `tldraw` Dynamic Diagramming (`harness trace export --format tldraw`)](#62-visual-telemetry--tldraw-dynamic-diagramming-harness-trace-export--format-tldraw)
7. [Loop Readiness Score & Audit Framework (`harness audit`)](#7-loop-readiness-score--audit-framework-harness-audit)
8. [CLI Command Reference & Extension Roadmap](#8-cli-command-reference--extension-roadmap)
9. [Appendices & Discussion Log (/learn)](#9-appendices--discussion-log-learn)

---

## 1. System Overview & Strategic Vision

### 1.1. Executive Summary
`harness` là một khung vận hành cấp repository (Repository-Level Operating Framework) và hệ thống **Loop Engineering** thế hệ mới. Khung làm việc này biến mọi dự án mã nguồn thành một môi trường sẵn sàng cho các AI Coding Agent (Cursor, Claude Code, Codex, Antigravity, Windsurf,...) thực thi công việc tự động, an toàn và bền vững mà không phụ thuộc vào tương tác prompt thủ công từ con người.

### 1.2. Problem Statement: Prompt Engineering vs. Loop Engineering
Hầu hết các dự án hiện nay gặp thất bại khi vận hành AI Agent ở quy mô lớn do 5 điểm nghẽn chính:
1. **Lịch sử chat ngắn hạn**: AI Agent bị trôi context khi thực hiện các nhiệm vụ dài hạn.
2. **Sửa code mù (No-contract editing)**: Agent thay đổi mã nguồn trước khi hiểu rõ ý đồ sản phẩm và hợp đồng kiến trúc.
3. **Đụng độ môi trường (Code collision)**: Nhiều Agent cùng làm việc trên một thư mục mã nguồn gây đè mã và hỏng môi trường local.
4. **Không có vòng lặp tự sửa lỗi (Missing self-correction loop)**: Khi test lỗi, con người phải đọc log và gõ lại prompt thủ công cho Agent.
5. **Prompt thủ công tốn thời gian**: Con người đóng vai trò là "động cơ gõ prompt" thay vì thiết kế hệ thống tự vận hành.

### 1.3. The 4-Tier Agentic Engineering Hierarchy
Harness nâng cấp quy trình làm việc với AI qua 4 cấp độ kỹ thuật:

```text
┌─────────────────────────────────────────────────────────────────────────┐
│ LEVEL 4: LOOP ENGINEERING (Hệ thống tự điều phối, prompt & loop test)  │ ◄── [HARNESS]
├─────────────────────────────────────────────────────────────────────────┤
│ LEVEL 3: HARNESS ENGINEERING (Hợp đồng sản phẩm, safety gates, rules)  │ ◄── [HARNESS]
├─────────────────────────────────────────────────────────────────────────┤
│ LEVEL 2: CONTEXT ENGINEERING (RAG, inject tài liệu, prompt window)      │
├─────────────────────────────────────────────────────────────────────────┤
│ LEVEL 1: PROMPT ENGINEERING (Gõ câu lệnh thủ công từng bước)            │
└─────────────────────────────────────────────────────────────────────────┘
```

### 1.4. Architectural Separation Rationales (Lý do & Triết lý Tách biệt Kiến trúc)

> **Cốt lõi**: *"Ứng dụng (App) là thứ người dùng chạm vào. Khung vận hành (Harness) là thứ AI Agent chạm vào."*

Harness thiết lập 4 nguyên tắc tách biệt kiến trúc chiến lược nhằm giải thích lý do tại sao hệ thống được tinh gọn:

#### A. Tại sao tách rời và loại bỏ hoàn toàn Web UI (`harness-web`)?
1. **Agent-First, Not Human-Web-First**: AI Agent tương tác trực tiếp qua giao diện dòng lệnh (CLI), file I/O, và truy vấn SQLite. Giao diện Web UI tạo ra sự cồng kềnh không cần thiết (phụ thuộc Axum/Tokio web, tài nguyên tĩnh HTML/JS/CSS, HTTP overhead) nhưng không mang lại giá trị nào cho quá trình suy luận và thực thi của Agent.
2. **Headless & High Performance**: Việc loại bỏ Web UI giúp Harness CLI giữ dung lượng siêu nhẹ, khởi động tức thì, chạy mượt mà dưới dạng background service/cron job hoặc gọi trực tiếp qua script wrapper `./scripts/harness`.
3. **Giảm thiểu ma sát bảo trì (Maintenance Overhead)**: Loại bỏ nguy cơ xung đột phụ thuộc giữa thư viện web và core CLI, giúp codebase giữ độ tập trung 100% vào Rust CLI (`harness-cli`) và bộ nhớ SQLite (`harness.db`).

#### B. Tại sao tách biệt Policy (`docs/`) và Durable State (`harness.db`)?
1. **Policy Docs (`docs/`)**: Dành cho con người và Agent cùng đọc (human-readable), mang tính ổn định cao, mô tả hợp đồng sản phẩm (`product/`), phân loại rủi ro (`FEATURE_INTAKE.md`), và ranh giới kiến trúc (`ARCHITECTURE.md`).
2. **Durable State (`harness.db`)**: Dành cho máy tính/CLI truy vấn tần suất cao (machine-readable), ghi nhận trạng thái động như intake logs, story progress, trace runs, backlog metrics. Việc tách rời giúp tránh tình trạng rác commit Git do liên tục chỉnh sửa các file Markdown tĩnh.

#### C. Tại sao tách biệt môi trường bằng Git Worktrees (`.worktrees/task-<id>`)?
1. **Tránh đụng độ mã nguồn (Zero Code Collision)**: Khi nhiều Sub-agent cùng chạy song song, làm việc trên một thư mục chính sẽ gây ra race-condition (đè code lên nhau) và làm bẩn trạng thái `git status` của con người.
2. **Khả năng khôi phục an toàn (Safe Reversion)**: Nếu Sub-agent làm hỏng mã nguồn hoặc thất bại trong kiểm thử, chỉ cần xóa thư mục Worktree tương ứng mà không làm ảnh hưởng tới nhánh chính `main` hay công việc chưa commit của lập trình viên.

#### D. Tại sao tách biệt vai trò Sub-Agents (Implementer ➔ Verifier, Explorer ➔ Implementer)?
1. **Triệt tiêu thiên vị định kiến (Confirmation Bias Prevention)**: Agent tự viết code thường có xu hướng tin rằng code của mình đúng. Việc tách riêng một **Verifier Agent** độc lập để rà soát Git Diff và chạy kiểm thử giúp đảm bảo tính khách quan và chất lượng khắt khe.
2. **Tiết kiệm dung lượng Cửa sổ Ngữ cảnh (Context Window Economy)**: With codebase lớn, việc cho Agent **Explorer** duyệt và nén kiến thức thành sơ đồ 20 dòng sẽ giúp Agent **Implementer** chỉ nhận đúng context cần thiết, tránh trôi lặp token và suy giảm trí nhớ.

### 1.5. Unified Loop Engineering & Chief of Staff Synergy Matrix

Nghiên cứu từ bài mở rộng *Loop Engineering — Design the system that prompts your agents* (từ repository `cobusgreyling/loop-engineering`) được tổng hợp và kết hợp chặt chẽ với mô hình **Chief of Staff (Mina)** thành ma trận hiệp đồng 5 khối:

| 5 Khối Loop Engineering (cobusgreyling) | Vai trò Chief of Staff (Mina) | Vai trò Sub-Agents (Implementer / Verifier) | Cơ chế Harness Vận hành |
| :--- | :--- | :--- | :--- |
| **Block 1: Input / Requirements Intake** | Tiếp nhận ý đồ (high-level vision) từ con người, phân tách thành User Stories & Specs. | Không trực tiếp làm việc với con người; nhận Stories đã phân tách từ Mina. | Lệnh `harness intake` phân loại rủi ro (Tiny, Normal, High-Risk) và lưu DB `harness.db`. |
| **Block 2: Prompt & Persona Engineering** | Xác định phong cách làm việc, nạp `AGENTS.md` và chọn vai trò cho các Sub-Agent. | Nhận Persona Header (`.agents/personas/<role>.md`) và `SKILL.md` tự động. | **Progressive Disclosure**: Nạp quy tắc lũy tiến theo từng bước, tránh cạn token window. |
| **Block 3: Task Execution & Topologies** | Quản lý tiến độ tổng thể, tháo gỡ điểm nghẽn (unblock), điều phối tác vụ dài hạn. | Realize implementation trong Git Worktrees cách ly (`.worktrees/task-<id>`). | Topologies A/B/C/D điều phối các Sub-Agents thực thi tác vụ dài hạn (Long-Horizontal Tasks). |
| **Block 4: Automated Verification Loop** | Đánh giá báo cáo từ Verifier trước khi nghiệm thu bàn giao cho người dùng. | **Verifier** audit git diff, chạy unit/integration tests, đóng gói log lỗi nếu fail. | **Auto-Feedback Loop**: Tự động gửi lại log lỗi để Sub-Agent tự sửa (tối đa N retries). |
| **Block 5: Memory & Telemetry Trace Output** | Đọc dữ liệu lịch sử trong `harness.db` để theo dõi hiệu suất và ma sát (friction). | Ghi vết hành động (trace) và các ma sát vào `harness.db` SQLite database. | Xuất sơ đồ `tldraw` (`.tldr`) tương tác để xem lại toàn bộ luồng trao đổi giữa các Agents. |

---

## 2. System Architecture & Dual-Loop Mechanics

### 2.1. High-Level Infinity Loop Diagram
Hệ thống vận hành theo mô hình **Dual-Loop (Vòng lặp vô cực kép)** kết nối giữa **Planning/Scheduling** và **Execution/Verification**:

```text
                    +-----------------------------------+
                    |    Central Orchestrator (Main)    |
                    +-----------------------------------+
                                  /       \
                                 /         \
                   +-----------------+   +--------------------+
                   |   Scheduling    |   |     Skills         |
                   | (Cron & Queue)  |   | (Inject Rules/PRD) |
                   +-----------------+   +--------------------+
                            |                      |
                            v                      v
                   +-----------------+   +--------------------+
                   | Persistent State|   | Git Worktrees      |
                   |  (harness.db)   |   | (Sub-Agent Env)    |
                   +-----------------+   +--------------------+
```

### 2.2. Comprehensive Task Lifecycle Sequence

```text
Human Intent / Spec
       │
       ▼
┌─────────────────┐
│  harness intake │ ──► Phân loại rủi ro (Tiny, Normal, High-Risk) & ghi DB
└────────┬────────┘
         │
         ▼
┌─────────────────────────┐
│ Worktree Spawn          │ ──► Tạo thư mục cách ly .worktrees/task-<id>
└────────────┬────────────┘
             │
             ▼
┌─────────────────────────┐
│ Persona Injection       │ ──► Inject Persona Header (.agents/personas/<role>.md)
└────────────┬────────────┘
             │
             ▼
┌─────────────────────────┐     ┌──────────────────────────────────────────────┐
│ Skill Resolution Engine │ ──► │ 1. Local Match (.agents/skills/<name>/)      │
└────────────┬────────────┘     │ 2. Remote Server Fallback (harness skill pull│
             │                  │ 3. Mid-Session Search (harness skill search) │
             │                  └──────────────────────────────────────────────┘
             ▼
┌─────────────────────────┐
│ Sub-Agent Execution     │ ──► Sub-agent nhận Context & thực thi viết code trong Worktree
└────────────┬────────────┘
         │
         ▼
┌─────────────────┐     (Thất bại & test lỗi)
│  harness story  │ ───────────────┐
│     verify      │                │  Auto-Feedback Loop:
└────────┬────────┘                ▼  Đóng gói log lỗi thành prompt mới
         │ (Pass test)     ┌───────────────┐  gửi lại cho Sub-agent (tối đa N retries)
         │                 │ Sub-Agent Fix │
         │                 └───────┬───────┘
         │                         │
         │                         └───────────┘
         ▼
┌─────────────────┐
│  Git Merge &    │ ──► Merge code vào main branch & xóa Worktree
│  Cleanup        │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Trace & Memory  │ ──► Ghi nhận nhật ký trace & ma sát vào harness.db
└─────────────────┘
```

---

## 3. Sub-Agent Ecosystem & Topologies

Harness quy định **6 Loại Sub-Agents Chuyên biệt** được phân bổ theo 4 mô hình kiến trúc (Topologies) nhằm tối ưu hóa ngữ cảnh, tiết kiệm token và đảm bảo chất lượng kiểm thử:

### 📊 Bảng Danh mục Sub-Agents được Setup trong Harness:

| Tên Sub-Agent | Vai trò (Role) | Model Tier | Quyền hạn (Permissions) | Mô hình (Topology) | Nhiệm vụ chính |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Chief of Staff (Mina)** | High-Level Orchestrator | `pro` | Read/Write (Task Registry) | Topology D | Tiếp nhận ý đồ người dùng, phân tách task, unblock và điều phối Main Agents. |
| **Implementer** | Coder / Maker | `pro` | Read/Write (Worktree) | Topology A / B | Viết mã nguồn, tái cấu trúc và sửa lỗi trong Git Worktree cách ly. |
| **Verifier** | QA / Auditor / Checker | `flash` | Read-Only (Verify Test) | Topology A | Audit git diff, chạy linter/unit test, đóng gói log lỗi cho Auto-Feedback. |
| **Explorer** | Codebase Navigator | `flash` | Read-Only | Topology B | Đọc & khảo sát codebase lớn, nén ngữ cảnh thành 20 dòng để tránh cạn token. |
| **Triage Monitor** | Health & Issue Scanner | `flash_lite` | Read-Only | Topology C | Quét định kỳ CI log, GitHub issues, cập nhật mức độ ưu tiên backlog vào DB. |
| **Security & Skill Auditor**| Remote Skill Sync & Audit | `flash` / `pro` | Read/Write (Skills) | Remote Pipeline | Kiểm tra tuân thủ an ninh, đồng bộ 100+ Skill từ Remote Skill Server (`harness skill sync`). |

---

```text
Topology A: Maker / Checker Split
┌───────────────────────┐       Git Diff       ┌───────────────────────┐
│ Sub-Agent Implementer │ ───────────────────► │   Sub-Agent Verifier  │
│ (Viết code & commit)  │                      │ (Review & Chạy Test)  │
└───────────────────────┘                      └───────────────────────┘

Topology B: Explorer / Implementer Split
┌───────────────────────┐     Context Summary  ┌───────────────────────┐
│  Sub-Agent Explorer   │ ───────────────────► │ Sub-Agent Implementer │
│ (Read-only codebase)  │                      │  (Chỉ nhận file cần)  │
└───────────────────────┘                      └───────────────────────┘

Topology C: Triage-Only Agent
┌──────────────────────────────────────────────────────────────────────┐
│  Sub-Agent Triage (Đọc định kỳ CI, issues & backlog, ghi report)    │
└──────────────────────────────────────────────────────────────────────┘
```

### 3.1. Topology A: Implementer ➔ Verifier (Maker / Checker Split)
- **Implementer**: Tập trung viết code và sửa lỗi theo yêu cầu câu chuyện.
- **Verifier**: Agent độc lập chịu trách nhiệm audit diff, kiểm tra quy tắc kiến trúc và thực thi bộ kiểm thử (`harness story verify`). Verifier có quyền từ chối (reject) và gửi lại yêu cầu sửa đổi cho Implementer.

### 3.2. Topology B: Explorer ➔ Implementer
- Dùng cho các codebase lớn. Agent **Explorer** chỉ có quyền đọc (read-only) để duyệt mã nguồn, trích xuất cấu trúc và nén lại thành bản tóm tắt gọn nhẹ. Agent **Implementer** chỉ nhận bản tóm tắt này để tiết kiệm token và giữ context sạch.

### 3.3. Topology C: Triage-Only Agent
- Chạy định kỳ qua Cron/Timer. Chỉ làm nhiệm vụ đọc issues, CI log failures, và cập nhật bảng ưu tiên backlog trong `harness.db` mà không can thiệp mã nguồn.

### 3.4. Topology D: Chief of Staff Orchestrator (Mina Pattern)

Mô hình **Chief of Staff (Mina)** là một cấp điều phối quản lý cao nhất nằm giữa Con người (Human) và các Main Coding Agents ở từng dự án/tác vụ:

```text
               +-----------------------------------+
               |        Human (Strategic Vision)   |
               +-----------------------------------+
                                 │
                                 ▼ (High-Level Plans / Style)
               +-----------------------------------+
               |    Chief of Staff Agent (Mina)    |
               +-----------------------------------+
                  /              │              \
                 v               v               v
         ┌──────────────┐┌──────────────┐┌──────────────┐
         │ Coding Agent ││ Coding Agent ││ Review Agent │
         │  (Project A) ││  (Project B) ││ (Gemini/Flash)│
         └──────────────┘└──────────────┘└──────────────┘
```

#### Các Đặc trưng Kỹ thuật của Chief of Staff (Mina):
1. **Ủy quyền Cấp cao (High-Level Delegation)**:
   - Con người chỉ trao đổi với Mina về tầm nhìn (vision), ý tưởng thiết kế, và phong cách làm việc.
   - Mina tự phân tách kế hoạch thành các task nhỏ, tự giải đáp thắc mắc và tháo gỡ điểm nghẽn (unblock) cho các Coding Agent bên dưới.
   - Các Coding Agent ở project bên dưới tương tác với Mina với giả định rằng họ đang tương tác trực tiếp với người dùng.

2. **Progressive Disclosure Memory (Markdown-driven)**:
   - Quản lý 100% bằng các file Markdown nhẹ (`AGENTS.md`, `identity.md`, `project-index.md`).
   - Loại bỏ hoàn toàn các framework cồng kềnh/ẩn mã (black-box).
   - Chỉ nạp thông tin/quy tắc của project khi có yêu cầu (on-demand) nhằm chống tràn cửa sổ ngữ cảnh (Context Window Overflow).

3. **Điều phối Main Agents thay vì Nested Sub-Agents**:
   - Sử dụng công cụ điều phối (ví dụ Hurder / CLI binaries) để spawn các **Main Agent độc lập** trên các môi trường riêng thay vì lạm dụng sub-agents lồng nhau trong cùng 1 session chat.
   - Phân bổ Model tối ưu theo thế mạnh chuyên môn: Gemini 3.6 Flash / Sonnet cho Review/Audit bug siêu tốc, Codex / Grok cho Heavy Implementation.

4. **Quản lý Tác vụ Dài hạn (Long-Horizontal Execution)**:
   - Nhờ Chief of Staff giữ ngữ cảnh và lịch sử dự án bền vững trong `harness.db`, hệ thống có thể duy trì hoạt động trong thời gian dài (multi-day horizontal tasks) mà không bị đứt gãy hay cạn kiệt context.

### 3.5. Model Selection & Sub-Agent Dispatch Protocol via `./scripts/harness`

Khi vận hành từ giao diện chat hoặc script điều phối (`./scripts/harness`), mỗi Agent cần được trang bị 4 yếu tố cấu hình để chọn đúng Model và kích hoạt Sub-Agents thành công:

#### A. Các Yếu tố Cần thiết cho Mỗi Agent (Per-Agent Requirements):
1. **Model Tier & Selection (`--model` / `ModelTier`)**:
   - **`flash` / `flash_lite`**: Phân bổ cho các Agent đóng vai trò **Explorer, Auditor, Triage, Reviewer** (tốc độ phản hồi cực nhanh, tối ưu chi phí token).
   - **`pro`**: Phân bổ cho các Agent đóng vai trò **Implementer, Architect, Chief of Staff (Mina)** (yêu cầu khả năng suy luận sâu và viết mã phức tạp).
   - **`inherit`**: Kế thừa trực tiếp Model từ phiên chat chính (Main Session).
2. **Context Scope Pinning (`--workdir`)**:
   - Chỉ định chính xác đường dẫn thư mục công tác (`.worktrees/task-<id>`) cho từng Sub-Agent để cách ly phạm vi đọc/ghi tệp tin.
3. **Persona & Skill Injection Header**:
   - Tự động nạp cấu hình vai trò từ `.agents/personas/<role>.md` và bộ quy tắc nghiệp vụ trong `.agents/skills/` vào prompt khởi tạo.
4. **Structured JSON Output Protocol**:
   - Định dạng đầu ra giữa các Agent theo chuẩn JSON (`harness trace --json`, `harness story update --json`) để máy tính xử lý và chuyển giao context không bị lỗi.

#### B. Cơ chế Chọn Model & Spawn Sub-Agent từ Chat / Script:

1. **Chọn Model trực tiếp trong Chat / CLI Session**:
   ```bash
   # Thiết lập Model cụ thể thông qua biến môi trường hoặc cờ CLI
   HARNESS_MODEL=gemini-3.6-flash ./scripts/harness agent run --role reviewer
   ```

2. **Cách Orchestrator Gọi Sub-Agent với Model Chỉ định**:
   Main Agent (hoặc Chief of Staff) kích hoạt Sub-Agent thông qua tool call `invoke_subagent` hoặc lệnh CLI wrapper:
   ```bash
   # Ví dụ lệnh spawn Sub-Agent với Model Tier chỉ định qua script
   ./scripts/harness subagent spawn \
       --role "Verifier" \
       --model "flash" \
       --workdir ".worktrees/task-US-014" \
       --prompt "Verify git diff and run cargo test"
   ```

### 3.6. Sub-Agent Skill Discovery & Resolution Protocol (`harness skill find / search`)

Để Sub-Agent không bị nạp thừa dữ liệu (gây lãng phí token) nhưng vẫn tìm đúng quy trình nghiệp vụ cần thiết khi thực thi nhiệm vụ, Harness xây dựng **Cơ chế Khám phá & Nạp Kỹ năng (Skill Discovery & Resolution Pipeline)** qua 3 cấp độ:

```text
Ý đồ Nhiệm vụ (Sub-Agent Prompt)
             │
             ▼
┌─────────────────────────────────────────────────────────────┐
│ 1. Explicit Preloading (--skills "skill-a,skill-b")         │
└────────────────────────────┬────────────────────────────────┘
                             │ (Nếu không chỉ định rõ)
                             ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. Dynamic Auto-Match (harness skill find "<intent>")       │
│    So khớp từ khóa nhiệm vụ với frontmatter trong SKILL.md  │
└────────────────────────────┬────────────────────────────────┘
                             │ (Khi Sub-Agent bị rào cản/stuck)
                             ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. On-Demand Mid-Session Search (harness skill search)      │
│    Sub-Agent tự gọi CLI để tìm và nạp bổ sung Skill         │
└────────────────────────────┘
```

#### A. Các Phương thức Tìm kiếm & Nạp Skill cho Sub-Agent:

1. **Nạp Chỉ định Trực tiếp (Explicit Preloading)**:
   Orchestrator chỉ định danh sách Skill cần thiết ngay trong lệnh spawn:
   ```bash
   ./scripts/harness subagent spawn \
       --role "Implementer" \
       --skills "harness-create-story,harness-qa-generate-e2e-tests" \
       --workdir ".worktrees/task-US-014" \
       --prompt "Implement authentication module"
   ```

2. **Tự động Tìm kiếm Theo Ngữ cảnh (Dynamic Auto-Matching)**:
   Khi không chỉ định cờ `--skills`, Harness CLI tự động quét bộ nhớ `.agents/skills/` và so khớp mô tả trong frontmatter `description` để chọn ra tối đa 2-3 Skill phù hợp nhất với prompt:
   ```bash
   # Lệnh CLI tìm kiếm Skill dựa trên ý định
   ./scripts/harness skill find "viết E2E test cho API authentication"
   # Đầu ra: .agents/skills/harness-qa-generate-e2e-tests/SKILL.md
   ```

3. **Tự Tìm kiếm Bổ sung Giữa Phiên (On-Demand Mid-Session Search)**:
   Trong quá trình thực thi, nếu Sub-Agent phát sinh nhu cầu mới (ví dụ: cần tạo bản ghi ADR hoặc thiết kế DB), Sub-Agent tự gọi lệnh CLI để tra cứu và đọc file `SKILL.md` bổ sung:
   ```bash
   # Sub-Agent thực thi lệnh tra cứu trong Worktree
   ./scripts/harness skill search "architecture decision record"
   ```

#### B. Nguyên tắc Nạp Lũy tiến (Progressive Skill Loading Rule):
- **Không nạp tràn**: Chỉ nạp file `SKILL.md` (chứa quy tắc chính). Không nạp các thư mục con `scripts/`, `templates/`, `references/` trừ khi Sub-Agent chủ động yêu cầu.
- **Cách ly phạm vi Skill**: Mọi Skill được tra cứu từ thư mục gốc dự án nhưng được thực thi hoàn toàn bên trong môi trường cách ly của Worktree (`.worktrees/task-<id>`).

### 3.7. Remote Skill Server & Centralized Registry Architecture (`harness skill sync / pull`)

Sử dụng **Server bên ngoài (Remote Skill Hub / Skill Server)** để quản lý, lưu trữ và tự động cập nhật bộ kỹ năng cho tất cả các máy local và Agent workspaces là một **kiến trúc mở rộng doanh nghiệp (Enterprise-Grade Architecture Extension)** rất hiệu quả.

```text
               ┌──────────────────────────────────────────────┐
               │    Remote Skill Server (Skill Registry)      │
               │    https://skills-hub.yourdomain.com         │
               │  - Quản lý tập trung 100+ Skills (Git/REST)   │
               │  - Phân quyền Access Control & Compliance     │
               └──────────────────────┬───────────────────────┘
                                      │
                 ┌────────────────────┴────────────────────┐
                 │ Sync / Pull API (HTTP / REST / MCP Gateway)│
                 └────────────────────┬────────────────────┘
                                      │
                                      ▼
             ┌──────────────────────────────────────────────────┐
             │       Local Project Workspaces (Harness)         │
             │                                                  │
             │  1. Local Cache (~/.harness/skills/ or .agents/)│
             │  2. Remote Fallback (Khi local tìm không thấy)   │
             │  3. Auto-Sync (`harness skill sync` khi init)    │
             └──────────────────────────────────────────────────┘
```

#### A. 4 Lợi ích Kiến trúc của Remote Skill Server:
1. **Quản lý & Cập nhật Tập trung (Centralized Governance & Updates)**:
   - Khi đội ngũ phát triển cập nhật quy chuẩn code, tiêu chuẩn an ninh hoặc quy trình mới, chỉ cần update file `SKILL.md` trên Remote Skill Server. Tất cả các dự án local của Agent sẽ tự động nhận phiên bản mới nhất mà không phải chỉnh sửa thủ công từng repo.
2. **Cơ chế Remote Fallback Search (Dự phòng Từ xa)**:
   - Khi Sub-Agent thực hiện `harness skill search "<query>"`, nếu chưa có file Skill tương ứng ở local (`.agents/skills/`), Harness CLI sẽ tự động gửi truy vấn tới Remote Skill Server để tải và nạp Skill đó tức thì vào context.
3. **Bộ nhớ Đệm Cục bộ & Hoạt động Ngoại tuyến (Offline Caching)**:
   - Tất cả các Skill tải từ Remote Server được lưu đệm tại `~/.harness/skills/` hoặc `.agents/skills/`. Nếu mất kết nối mạng, Agent vẫn sử dụng bộ đệm offline bình thường.
4. **Phân quyền & Kiểm soát Tuân thủ (Compliance & Access Control)**:
   - Server từ xa có thể phân quyền Skill theo từng dự án hoặc từng vai trò Agent (ví dụ: chỉ Agent Auditor mới được cấp quyền tải các Skill kiểm định bảo mật nâng cao).

#### B. Các Lệnh CLI Hỗ trợ Remote Skill Server:

```bash
# 1. Cấu hình URL của Remote Skill Server
./scripts/harness config set skill_server "https://skills-hub.yourdomain.com/api/v1"

# 2. Đồng bộ toàn bộ Skill mới nhất từ Server về máy local
./scripts/harness skill sync

# 3. Pull một Skill cụ thể từ Remote Server về dự án
./scripts/harness skill pull "harness-security-audit"

# 4. Push một Skill mới tạo ở local lên Remote Server
./scripts/harness skill push "custom-team-skill"
```

#### C. Quy chuẩn Bảo mật Biến Môi trường (`.env` & `.env.example` Protocol):

1. **Không commit `.env` lên Git**:
   - Các thông tin cấu hình như URL Private Skill Server (`HARNESS_SKILL_SERVER=https://skills-hub.yourdomain.com/api/v1`), Token xác thực (`HARNESS_SKILL_SERVER_TOKEN`), và API Keys bắt buộc phải lưu trong file `.env` cục bộ.
   - File `.env` được ghi chú bắt buộc trong `.gitignore` để triệt tiêu nguy cơ rò rỉ mã bảo mật lên GitHub.

2. **Duy trì Tệp Mẫu `.env.example` trên GitHub**:
   - Dự án duy trì tệp mẫu `.env.example` trên Git chứa danh sách toàn bộ các biến môi trường cùng mô tả hướng dẫn, giúp các lập trình viên hoặc AI Agent khác khi clone repo về đều biết chính xác cần thiết lập những tham số nào.

3. **Tệp Mẫu `.env.example` chuẩn**:
   ```ini
   # Remote Skill Server Endpoint
   HARNESS_SKILL_SERVER=https://skills-hub.yourdomain.com/api/v1
   HARNESS_SKILL_SERVER_TOKEN=your_private_auth_token_here

   # Default Model Selection for Harness CLI & Sub-Agents
   HARNESS_MODEL=gemini-3.6-flash
   HARNESS_DB=./harness.db
   ```

---

## 4. Git Worktree Isolation & Runtime Management

Để đảm bảo các Agent có thể hoạt động song song mà không xung đột, Harness quản lý môi trường theo 4 nguyên tắc:

### 4.1. Worktree Directory Structure
```text
your-project/
  ├── .git/
  ├── .worktrees/
  │    ├── task-US-014/     # Git Worktree cho Agent Dev 1
  │    └── task-US-015/     # Git Worktree cho Agent Dev 2
  ├── harness.db
  └── scripts/harness
```

### 4.2. Runtime Environment Isolation
Không chỉ cách ly file, Harness phân bổ tài nguyên runtime độc lập cho từng Worktree:
- **Port Allocation**: Tự động cấp phát cổng mạng riêng (ví dụ: `BASE_PORT + index * 10`) cho các server test của agent.
- **`TMPDIR` Isolation**: Cấp phát thư mục tạm riêng biệt cho từng process của agent để tránh đè tệp cache/lock.

### 4.3. Repository-Level Memory Indexing
Mặc dù môi trường file bị chia tách theo Worktrees, dữ liệu bộ nhớ `harness.db` được index theo **Repository ID** (URL Git remote) thay vì đường dẫn thư mục cục bộ. Điều này giúp mọi Sub-agent đều truy cập và thừa hưởng cùng một kho tri thức chung.

### 4.4. Automatic Lifecycle Pruning
Ngay khi một nhiệm vụ hoàn thành và được kiểm thử thành công, Harness tự động thực hiện:
```bash
git worktree remove .worktrees/task-<id> --force
```
Điều này ngăn chặn việc tích tụ các thư mục rác và đảm bảo dọn dẹp môi trường local.

### 4.5. Herdr Terminal Agent Multiplexer Integration (`herdr` CLI & Socket API)

Thay vì dựng giao diện Web UI cồng kềnh, Harness tích hợp trực tiếp với **[Herdr](https://herdr.dev/)** — một bộ multiplexer terminal thuần nhị phân (Agent Multiplexer) dành riêng cho các AI Coding Agents (tương tự như `tmux` nhưng hiểu ngữ cảnh trạng thái Agent):

```text
                  ┌──────────────────────────────────────────────┐
                  │          Herdr Agent Multiplexer             │
                  │   - Persistent PTY Sessions (No Laptop)      │
                  │   - Statuses: Working | Blocked | Done       │
                  │   - Remote SSH / Mobile Attach Support       │
                  └──────────────────────┬───────────────────────┘
                                         │
                   ┌─────────────────────┴─────────────────────┐
                   │ Herdr Socket API & CLI Engine Integration │
                   └─────────────────────┬─────────────────────┘
                                         │
                  ┌──────────────────────┴──────────────────────┐
                  ▼                                             ▼
┌───────────────────────────────────┐       ┌───────────────────────────────────┐
│ Worktree Pane 1: .worktrees/task-1│       │ Worktree Pane 2: .worktrees/task-2│
│ Executing: ./scripts/harness dev  │       │ Executing: ./scripts/harness audit│
└───────────────────────────────────┘       └───────────────────────────────────┘
```

#### Các Lợi ích Kỹ thuật của việc Kết hợp Herdr với Harness:
1. **Duy trì Phiên chạy Bền vững (Persistent PTY & Remote SSH Attach)**:
   - Các Sub-Agents của Harness chạy trong các PTY pane độc lập do Herdr quản lý trên server / Mac Mini.
   - Lập trình viên hoặc Chief of Staff có thể gấp laptop lại, SSH từ điện thoại/tablet vào Herdr để theo dõi tiến độ mà các tiến trình của Agent không bao giờ bị chết.
2. **Trạng thái Trực quan của Agent (Semantic Agent State)**:
   - Herdr tự động nhận biết và hiển thị trạng thái trực quan của từng Agent trên giao diện TUI:
     - **`working`**: Agent đang suy luận & viết mã nguồn trong Worktree.
     - **`blocked`**: Agent đang dừng lại chờ duyệt quyền hoặc chờ feedback.
     - **`done`**: Agent đã vượt qua bài test và hoàn thành nhiệm vụ.
3. **Điều phối Tự động qua Socket API (`herdr` Orchestration Engine)**:
   - Chief of Staff (Mina) hoặc Harness CLI có thể gọi Herdr Socket API (`/docs/socket-api/`) để tự động mở tab mới, chia pane, điều phối các Main Agents (Claude Code, OpenCode, Codex, Harness CLI) vận hành song song trên từng Worktree cách ly.

---

## 5. Loop Execution Engine & Verification Proof Loop

### 5.1. The Goal Execution Pattern (`/goal`)
Harness triển khai mô hình **Run-until-done**: Agent được giao một mục tiêu rõ ràng kèm điều kiện dừng có thể kiểm chứng được (Verifiable Stop Condition). Agent sẽ lặp đi lặp lại chu kỳ `Reason -> Act -> Observe` cho tới khi điều kiện dừng thỏa mãn.

### 5.2. Auto-Feedback Loop & Circuit Breakers
Khi bước kiểm thử tự động thất bại:
1. Harness trích xuất nguyên văn lỗi từ stdout/stderr của lệnh test.
2. Đóng gói log lỗi kèm file vị trí thành prompt phản hồi tự động.
3. **Circuit Breaker (Ngắt mạch an toàn)**: Nếu số lần lặp lại vượt quá ngưỡng $N$ retries (mặc định: 3-5 lần) hoặc vượt quá hạn mức ngân sách token, Harness sẽ dừng loop, giữ nguyên trạng thái và thông báo cho người dùng hỗ trợ.

### 5.3. Progressive Verification Ladder
Bảng kiểm chứng chất lượng theo các cấp độ:

```text
Level 1: Format & Lint       (cargo fmt / eslint / clippy)
Level 2: Unit Tests          (cargo test / npm test)
Level 3: Architecture Check  (Boundary parsing & Layering rules)
Level 4: Integration Tests   (Database & API responses)
Level 5: End-to-End Tests    (User flow verification)
```

---

## 6. Durable Memory & Observability (`harness.db`)

Cơ sở dữ liệu SQLite `harness.db` là bộ nhớ bền vững cốt lõi của Harness.

### 6.1. Relational Schema Architecture
```sql
-- Intakes: Quản lý đăng ký phân loại rủi ro
CREATE TABLE intakes (
    id TEXT PRIMARY KEY,
    input_type TEXT NOT NULL,
    summary TEXT NOT NULL,
    risk_lane TEXT NOT NULL,
    notes TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Stories: Quản lý các gói công việc và tiến độ kiểm thử
CREATE TABLE stories (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    lane TEXT NOT NULL,
    priority TEXT NOT NULL,
    status TEXT NOT NULL,
    unit_proof TEXT,
    integ_proof TEXT,
    e2e_proof TEXT,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Tracing & Friction: Lưu lịch sử hoạt động và khó khăn phát sinh
CREATE TABLE traces (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_summary TEXT NOT NULL,
    story_id TEXT,
    agent TEXT,
    outcome TEXT NOT NULL,
    harness_friction TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### 6.2. Visual Telemetry & `tldraw` Dynamic Diagramming (`harness trace export --format tldraw`)

Harness tích hợp khả năng xuất sơ đồ tương tác đa Agent trực quan dưới định dạng **`tldraw` (`.tldr` JSON Snapshot)** để mở trực tiếp trên [offline.tldraw.com](https://offline.tldraw.com) hoặc canvas cục bộ mà **hoàn toàn offline và bảo mật tuyệt đối (Local-First)**.

```text
┌─────────────────────────────────────────────────────────────┐
│  Harness Trace Database (harness.db traces table)           │
│  - Dispatch logs, Sub-agent communications, Test outcomes   │
└────────────────────────────┬────────────────────────────────┘
                             │
                             ▼ (Execute CLI Exporter)
┌─────────────────────────────────────────────────────────────┐
│  ./scripts/harness trace export --format tldraw             │
└────────────────────────────┬────────────────────────────────┘
                             │
                             ▼ (Generates JSON Snapshot)
┌─────────────────────────────────────────────────────────────┐
│  agent-trace-<id>.tldr  (Interactive Visual Canvas)         │
│  - Mở trực tiếp trên offline.tldraw.com / local canvas      │
│  - Kẻ sơ đồ luồng trao đổi giữa Chief of Staff & Sub-Agents │
└─────────────────────────────────────────────────────────────┘
```

#### Các Đặc trưng Kỹ thuật của `tldraw` Visual Diagramming:
1. **Trực quan hóa Luồng Tương tác (Multi-Agent Interaction Graph)**:
   - Tự động chuyển đổi các bản ghi trace giao tiếp giữa **Chief of Staff (Mina)** và các **Sub-Agents (Implementer, Verifier, Explorer)** thành sơ đồ khối (boxes/shapes) và mũi tên kết nối (arrows/bindings).
2. **Local-First & Bảo mật Ngoại tuyến (Offline Safety)**:
   - Định dạng tệp `.tldr` là JSON thuần túy (tuân theo `TLRecord` & `TldrawFile` schema).
   - Mở và chỉnh sửa kéo-thả hoàn toàn offline tại [offline.tldraw.com](https://offline.tldraw.com) mà không lo rò rỉ mã nguồn hay ngữ cảnh trao đổi của Agent lên Internet.
3. **Lệnh CLI Xuất Sơ đồ**:
   ```bash
   # 1. Xuất sơ đồ tldraw tương tác từ trace hoạt động gần nhất
   ./scripts/harness trace export --format tldraw --out trace-diagram.tldr

   # 2. Xuất sơ đồ Mermaid cho tài liệu Markdown
   ./scripts/harness trace export --format mermaid --out trace-diagram.md
   ```

---

## 7. Loop Readiness Score & Audit Framework (`harness audit`)

Harness đi kèm một bộ đánh giá **Loop Readiness Score** (thang điểm từ 10 đến 100) để xác định độ sẵn sàng của repository trước khi cho phép AI Agent vận hành tự động không giám sát:

```text
[Loop Readiness Score Rubric]
├── Constraints (30%): Có quy tắc mã nguồn, linter và loại bỏ warning chưa?
├── Governance (30%): Đã có AGENTS.md, FEATURE_INTAKE.md và ARCHITECTURE.md chưa?
└── Harness Runtime (40%): Đã cài đặt harness.db, CLI và bộ kiểm thử tự động chưa?
```

- **0 - 40 điểm (Unsafe)**: Cần bổ sung tài liệu hợp đồng và linter.
- **41 - 75 điểm (Partial)**: Sẵn sàng cho tác vụ Tiny / Normal có giám sát.
- **76 - 100 điểm (Loop-Ready)**: Sẵn sàng cho Loop Engineering tự động chạy nền.

---

## 8. CLI Command Reference & Extension Roadmap

### 8.1. Summary of Stable Harness CLI Commands
| Nhóm Lệnh | Lệnh CLI | Chức Năng |
| :--- | :--- | :--- |
| **Khởi Tạo** | `harness init` | Khởi tạo SQLite `harness.db` & generate skill files cho IDE. |
| **Phân Loại** | `harness intake --type <t> --summary <s> --lane <l>` | Đăng ký tác vụ và phân loại mức độ rủi ro. |
| **Quản Lý Worktree**| `harness worktree create --id <id>` | Khởi tạo Git Worktree cách ly cho sub-agent. |
| **Quản Lý Worktree**| `harness worktree prune` | Thu hồi tất cả các worktrees đã hoàn thành. |
| **Kiểm Thử Auto** | `harness story verify --id <id> --auto-fix` | Chạy bộ test và kích hoạt Auto-Feedback Loop nếu lỗi. |
| **Đánh Giá Readiness**| `harness audit` | Kiểm tra và chấm điểm Loop Readiness Score (10-100). |
| **Truy Vấn** | `harness query matrix / stats / friction` | Truy vấn dữ liệu ma trận kiểm thử và ma sát. |

### 8.2. Phased Roadmap
- **Phase 1 (Completed)**: Tối ưu CLI Rust thuần túy (`harness-cli`), loại bỏ Web UI, khởi tạo DB schema bền vững.
- **Phase 2 (Current)**: Hoàn thiện `spec.md`, chuẩn hóa `.agents/skills/`, tích hợp bộ quy tắc Worktrees isolation.
- **Phase 3 (Next)**: Triển khai lệnh `harness worktree` và `harness story verify --auto-fix` trong Rust CLI.
- **Phase 4 (Future)**: Xây dựng bộ tự động lập lịch Cron (`harness scheduler`) hỗ trợ đa nền tảng.

---

## 9. Appendices & Discussion Log (/learn)

*Nhật ký lưu trữ các quyết định kiến trúc và bài học rút ra giữa người dùng và Agent:*

- **[2026-07-25]**: Loại bỏ Crate `harness-web` (Web UI). Tập trung 100% tài nguyên vào Rust CLI (`harness-cli`), SQLite `harness.db`, và hệ thống script điều phối.
- **[2026-07-25]**: Giữ lại `.agents/` và `.cursor/` trên máy cục bộ để phục vụ IDE/Agent nhưng loại bỏ hoàn toàn khỏi Git tracking (`.gitignore`).
- **[2026-07-25]**: Tích hợp khái niệm **Loop Engineering** (Level 4 trong tháp Agentic Engineering) vào dự án Harness.
- **[2026-07-25]**: Phân tích tài nguyên từ `cobusgreyling/loop-engineering` — tích hợp 5 Building Blocks, mô hình tách biệt Implementer ➔ Verifier, và cơ chế tính điểm **Loop Readiness Score (`harness audit`)** vào [spec.md](file:///Users/bao312/Desktop/harness/spec.md).
- **[2026-07-25]**: Chuẩn hóa toàn bộ tài liệu kiến trúc [spec.md](file:///Users/bao312/Desktop/harness/spec.md) thành bản đặc tả kỹ thuật chi tiết chuẩn sản phẩm (Production-Grade Architecture Specification Manual).
- **[2026-07-25]**: Bổ sung Mục 1.4 vào [spec.md](file:///Users/bao312/Desktop/harness/spec.md) giải thích rõ ràng **4 Lý do Tách biệt Kiến trúc**: (A) Loại bỏ Web UI vì Agent-First/Headless, (B) Tách Policy (`docs/`) và Durable State (`harness.db`), (C) Tách biệt môi trường bằng Git Worktrees, (D) Tách biệt vai trò Sub-Agents (Implementer vs. Verifier, Explorer vs. Implementer).
- **[2026-07-25]**: Nghiên cứu & tích hợp mô hình **Chief of Staff Orchestrator (Mina Pattern)** từ vietsub video YouTube vào Mục 3.4 của [spec.md](file:///Users/bao312/Desktop/harness/spec.md) — xác định cơ chế High-Level Delegation, Progressive Disclosure qua Markdown, điều phối Main Agents độc lập qua Hurder/CLI, và thực thi các tác vụ dài hạn (Long-Horizontal Tasks).
- **[2026-07-25]**: Bổ sung Mục 3.5 vào [spec.md](file:///Users/bao312/Desktop/harness/spec.md) quy định **Model Selection & Sub-Agent Dispatch Protocol via `./scripts/harness`** — phân bổ Model Tiers (`flash` cho Auditor/Reviewer vs `pro` cho Implementer/Mina), pin cờ `--workdir`, inject Persona Header, và chuẩn hóa JSON Protocol giữa các agents khi gọi qua script CLI.
- **[2026-07-25]**: Bổ sung Mục 3.6 vào [spec.md](file:///Users/bao312/Desktop/harness/spec.md) quy định **Sub-Agent Skill Discovery & Resolution Protocol (`harness skill find / search`)** — gồm 3 cấp độ: (1) Explicit Preloading (`--skills`), (2) Dynamic Auto-Match (`harness skill find`), và (3) Mid-Session On-Demand Search (`harness skill search`). Tránh nạp tràn context window, áp dụng triết lý Progressive Skill Loading.
- **[2026-07-25]**: Bổ sung Mục 3.7 vào [spec.md](file:///Users/bao312/Desktop/harness/spec.md) tích hợp kiến trúc **Remote Skill Server & Centralized Registry (`harness skill sync / pull`)** — hỗ trợ quản lý 100+ Skill tập trung từ xa, tự động đồng bộ về local cache (`~/.harness/skills/`), cơ chế dự phòng Remote Fallback khi local chưa có Skill, và phân quyền kiểm soát tuân thủ (Enterprise Compliance).
- **[2026-07-25]**: Quy định chuẩn bảo mật môi trường: Đưa các thông tin nhạy cảm (Private Skill Server URL, Auth Tokens, Model Keys) vào file cục bộ `.env` (gitignored), duy trì tệp mẫu `.env.example` trên Git để hướng dẫn cấu hình, và bổ sung hướng dẫn vào [AGENTS.md](file:///Users/bao312/Desktop/harness/AGENTS.md) & [spec.md](file:///Users/bao312/Desktop/harness/spec.md#L375-L397).
- **[2026-07-25]**: Bổ sung Mục 6.2 vào [spec.md](file:///Users/bao312/Desktop/harness/spec.md) tích hợp tính năng **Visual Telemetry & `tldraw` Dynamic Diagramming (`harness trace export --format tldraw`)** — tự động chuyển đổi nhật ký tương tác Sub-Agents thành tệp `.tldr` JSON Snapshot để mở, xem và tương tác sơ đồ kéo-thả hoàn toàn offline tại [offline.tldraw.com](https://offline.tldraw.com).
- **[2026-07-25]**: Chuẩn hóa Bảng Danh mục Sub-Agents (`Sub-Agent Ecosystem Registry Table`) tại Mục 3 trong [spec.md](file:///Users/bao312/Desktop/harness/spec.md#L165-L180) tổng hợp 6 Sub-Agents được setup chính thức (Chief of Staff, Implementer, Verifier, Explorer, Triage Monitor, Security & Skill Auditor).
- **[2026-07-25]**: Bổ sung Bảng Ma trận Hiệp đồng 5 Khối Loop Engineering (`cobusgreyling/loop-engineering`) kết hợp với **Chief of Staff (Mina)** vào Mục 1.5 trong [spec.md](file:///Users/bao312/Desktop/harness/spec.md#L85-L102), xác định rõ ràng sự phân công giữa Mina, Sub-Agents và cơ chế vận hành của Harness CLI từ Intake tới Telemetry Trace.
- **[2026-07-25]**: Bổ sung Mục 4.5 vào [spec.md](file:///Users/bao312/Desktop/harness/spec.md) tích hợp **Herdr Terminal Agent Multiplexer (`herdr` CLI & Socket API)** — giải pháp multiplexer thuần nhị phân chạy bền vững trên server/Mac Mini, hỗ trợ SSH attach từ điện thoại/tablet, tự động hiển thị semantic agent state (`working`, `blocked`, `done`), và cung cấp Socket API để Chief of Staff tự động chia pane/tab điều phối các Sub-Agents trên từng Worktree.
