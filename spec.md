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
2. [System Architecture & Dual-Loop Mechanics](#2-system-architecture--dual-loop-mechanics)
3. [Sub-Agent Ecosystem & Topologies](#3-sub-agent-ecosystem--topologies)
   - 3.1 [Topology A: Implementer ➔ Verifier (Maker / Checker Split)](#31-topology-a-implementer--verifier-maker--checker-split)
   - 3.2 [Topology B: Explorer ➔ Implementer](#32-topology-b-explorer--implementer)
   - 3.3 [Topology C: Triage-Only Agent](#33-topology-c-triage-only-agent)
   - 3.4 [Topology D: Chief of Staff Orchestrator (Mina Pattern)](#34-topology-d-chief-of-staff-orchestrator-mina-pattern)
4. [Git Worktree Isolation & Runtime Management](#4-git-worktree-isolation--runtime-management)
5. [Loop Execution Engine & Verification Proof Loop](#5-loop-execution-engine--verification-proof-loop)
6. [Durable Memory & Observability (`harness.db`)](#6-durable-memory--observability-harnessdb)
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
2. **Tiết kiệm dung lượng Cửa sổ Ngữ cảnh (Context Window Economy)**: Với codebase lớn, việc cho Agent **Explorer** duyệt và nén kiến thức thành sơ đồ 20 dòng sẽ giúp Agent **Implementer** chỉ nhận đúng context cần thiết, tránh trôi lặp token và suy giảm trí nhớ.

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
┌─────────────────┐
│ Worktree Spawn  │ ──► Tạo thư mục cách ly .worktrees/task-<id>
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Persona & Skill │ ──► Tự động inject Persona (Dev, QA, BA) & SKILL.md
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Sub-Agent Exec  │ ──► Sub-agent thực thi viết code trong Worktree
└────────┬────────┘
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

Harness hỗ trợ 3 mô hình phân chia Sub-Agent (Sub-Agent Topologies) nhằm tối ưu hóa ngữ cảnh và độ chính xác:

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
