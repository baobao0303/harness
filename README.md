# Harness 🚀 — Modular Rust CLI & Software Development Lifecycle Operating Framework

> **Modular Rust CLI engine (`harness-cli`), SQLite durable layer (`harness.db`), and Company Software Development & Release Operating Framework.**

`harness` is a repository-level operating framework for autonomous AI agents, sub-agent orchestration, and software development lifecycle management compliant with **Company Standards (SM-QT-01..04, SM-QĐi-01, SM-QĐ-002, DDD-COMPANY-2026)** and **ISO/IEC 27001:2022**.

---

## 🏗️ 1. Modular Rust CLI Architecture (`crates/harness-cli/src/`)

The `harness-cli` crate is refactored into a clean, modular, domain-driven architecture following Hexagonal / Layered Architecture principles. Code resides under `crates/harness-cli/src/`:

```text
crates/harness-cli/src/
├── main.rs                 # Binary entry point parsing CLI arguments via clap
├── lib.rs                  # Module declarations (domain, application, infrastructure, interface)
├── domain/                 # Core domain logic, entities, types, errors, and validation rules
│   ├── mod.rs
│   ├── types.rs            # Core domain enums (WorkItemType, WorkItemState, Priority, Severity, etc.)
│   ├── entities.rs         # Domain entities (WorkItem, Story, Intake, Decision, BacklogItem, Trace, Tool)
│   ├── validation.rs       # State machine transitions, Title syntax formatting, & Description rules
│   ├── errors.rs           # Domain-specific error types (StateMachineError, SyntaxValidationError)
│   ├── registry.rs         # Pre-defined tool responsibilities and definitions
│   └── scoring.rs          # Trace quality tier scoring and entropy calculations
├── application/            # Use cases, DTOs, and application services
│   ├── mod.rs
│   ├── service.rs          # Service methods orchestrating domain rules with persistence
│   ├── dto.rs              # Data Transfer Objects for command/query operations
│   └── errors.rs           # Application service error handling
├── infrastructure/         # I/O, SQLite database access, processes, and brownfield import
│   ├── mod.rs
│   ├── db/
│   │   ├── mod.rs          # Database connection pool and schema migration runner
│   │   ├── queries.rs      # SQL queries and CRUD operations
│   │   └── schema.rs       # Embedded migration scripts runner
│   ├── process.rs          # External process verification execution & Git worktree helpers
│   ├── brownfield.rs       # Legacy markdown parsing (TEST_MATRIX.md, ADRs, Backlog)
│   └── errors.rs           # Database and I/O error mappings
└── interface/              # User interface layer & CLI subcommands using clap
    ├── mod.rs              # Top-level CLI command router and execution handler
    ├── args.rs             # Clap CLI structs, enums, subcommands, and flags
    ├── handlers.rs         # Subcommand execution handlers bridging CLI to application services
    ├── formatters.rs       # Output formatters (Tables, JSON, plain text)
    ├── stubs.rs            # Worktree and Subagent CLI command handlers
    └── errors.rs           # Command line interface error representations
```

### Module Responsibilities:
- **`lib.rs` & `main.rs`**: `lib.rs` exports the core modules (`domain`, `application`, `infrastructure`, `interface`). `main.rs` parses arguments with `harness_cli::interface::Cli::parse()` and calls `interface::run(cli)`.
- **`domain/`**: Contains pure business logic without I/O dependencies. Defines data enums, domain entities, validation logic (state machine transition rules, title syntax checks, description formatting checks), and error types.
- **`application/`**: Implements application use cases (`HarnessService`). Receives DTOs, applies domain rules, and coordinates database operations.
- **`infrastructure/`**: Handles external side effects. Manages SQLite database connection (`harness.db`), WAL journal mode, migration execution (`001-init.sql` to `006-work-item.sql`), process invocation for story/decision verification, and brownfield markdown imports.
- **`interface/`**: Implements the command-line adapter using `clap`. Parses flags, converts command input into application service calls, and formats terminal output.

---

## 🗄️ 2. SQLite Database Schema & Migration `006-work-item.sql`

Harness uses SQLite (`harness.db`) as its durable operational data layer configured with `PRAGMA journal_mode = WAL;` and `PRAGMA foreign_keys = ON;`.

### Migration History (`scripts/schema/`):
- **`001-init.sql`**: Initialized `schema_version`, `intake`, `story`, `decision`, `backlog`, and `trace` tables.
- **`002-story-verify.sql`**: Added `verify_command`, `last_verified_at`, `last_verified_result` columns to `story`.
- **`003-tool-registry.sql`**: Created machine-readable `tool` registry table.
- **`004-intervention.sql`**: Added `intervention` table for tracking human, reviewer, CI, or agent interventions.
- **`005-priority.sql`**: Added `priority` classification (`P0`, `P1`, `P2`, `P3`) to `story` and `backlog`.
- **`006-work-item.sql`**: Created the unified `work_item` table supporting Company Portfolio Work Item hierarchy.

### Unified `work_item` Table Schema (`006-work-item.sql`):

```sql
CREATE TABLE IF NOT EXISTS work_item (
    id                   INTEGER PRIMARY KEY AUTOINCREMENT,
    type                 TEXT    NOT NULL
                         CHECK(type IN (
                             'Epic', 'Feature', 'User Story', 'Technical Story', 'Task', 'Bug', 'Testcase'
                         )),
    title                TEXT    NOT NULL,
    description          TEXT,
    state                TEXT    NOT NULL DEFAULT 'New'
                         CHECK(state IN (
                             'New', 'Accepted', 'Active', 'Resolved', 'Closed', 'Blocked', 'Removed'
                         )),
    assigned_to          TEXT,
    story_points         INTEGER CHECK(story_points IS NULL OR story_points >= 0),
    remaining_work       REAL    CHECK(remaining_work IS NULL OR remaining_work >= 0),
    priority             TEXT    NOT NULL DEFAULT 'P2'
                         CHECK(priority IN ('P0', 'P1', 'P2', 'P3')),
    severity             TEXT    CHECK(severity IS NULL OR severity IN ('Low', 'Medium', 'High', 'Critical')),
    parent_id            INTEGER REFERENCES work_item(id) ON DELETE SET NULL,
    area_path            TEXT,
    iteration_path       TEXT,
    tags                 TEXT,
    acceptance_criteria  TEXT,
    repro_steps          TEXT,
    actual_result        TEXT,
    expected_result      TEXT,
    steps                TEXT,
    created_at           TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at           TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_work_item_type ON work_item(type);
CREATE INDEX IF NOT EXISTS idx_work_item_state ON work_item(state);
CREATE INDEX IF NOT EXISTS idx_work_item_parent ON work_item(parent_id);
CREATE INDEX IF NOT EXISTS idx_work_item_assigned ON work_item(assigned_to);
```

---

## 🏷️ 3. Company Portfolio Work Item Types

Harness implements Company's multi-tiered Portfolio Backlog hierarchy (`spec_new.md` Section 5.2 / 6.3):

```text
                               ┌───────────────────────────┐
                               │           EPIC            │ (Created by CPO)
                               └─────────────┬─────────────┘
                                             │
                               ┌─────────────▼─────────────┐
                               │          FEATURE          │ (Created by BA)
                               └─────────────┬─────────────┘
                                             │
            ┌────────────────────────────────┴────────────────────────────────┐
            ▼                                                                 ▼
 ┌────────────────────┐ (Created by BA)                            ┌────────────────────┐ (Created by Dev/Team)
 │     USER STORY     │                                            │  TECHNICAL STORY   │
 └──────┬──────┬──────┘                                            └──────┬──────┬──────┘
        │      │      ┌───────────────────────────────────────────────────┘      │
        │      └──────┼──────────────────────────┐                               │
        ▼             ▼                          ▼                               ▼
 ┌────────────┐┌────────────┐             ┌────────────┐                  ┌────────────┐
 │    TASK    ││    BUG     │             │  TESTCASE  │                  │    TASK    │
 └────────────┘└────────────┘             └────────────┘                  └────────────┘
```

1. **`Epic`**: Strategic high-level business goal spanning multiple Sprints. Created and managed by CPO. Contains User Stories and Technical Stories.
2. **`Feature`**: Specific product capability within an Epic. Created by BA. Contains User Stories.
3. **`User Story`**: Functional requirement from the end-user perspective. Created by BA. Contains Tasks, Bugs, and Testcases.
4. **`Technical Story`**: Non-functional or infrastructure engineering work (e.g. test epics, release plans, database migrations, refactoring). Created by Dev/Team. Positioned at the **same hierarchy level as User Story**. Contains Tasks and Bugs.
5. **`Task`**: Granular technical execution unit (coding, code review, setup) assigned to a Developer or Tester.
6. **`Bug`**: Defect or error discovered during testing or operation. Created by Tester (QA). Resolved by Dev.
7. **`Testcase`**: Structured verification script with execution steps and expected outcomes. Created by Tester.

---

## 🔄 4. State Machine Lifecycle Transitions

Harness enforces strict state transition matrices for each work item type per Section 5.4 & 6.4 of `spec_new.md`:

### 1. User Story Lifecycle Matrix
- **`New`** ➔ `Accepted`, `Removed` *(BA completes documentation & UI design)*
- **`Accepted`** ➔ `Active`, `Removed` *(Dev pulls into Sprint; requires at least 1 Active Task)*
- **`Active`** ➔ `Resolved`, `Blocked`, `Removed` *(Dev completes AC, 2 code reviews pass, Tasks/Bugs Closed, Unit/Integration tests pass, merged to Test environment)*
- **`Blocked`** ➔ `Active` *(Unblocked)*
- **`Resolved`** ➔ `Closed` *(CPO/SM/PM confirms AC during Sprint Review)*
- **`Resolved`** ➔ `Accepted` *(If Sprint Review evaluates story as not meeting AC, returned to Accepted for next Sprint)*
- **`Closed`** / **`Removed`** *(Terminal states)*

### 2. Task Lifecycle Matrix
- **`New`** ➔ `Active`, `Removed` *(Dev starts work)*
- **`Active`** ➔ `Resolved`, `Blocked`, `Removed` *(Dev completes work & opens Pull Request)*
- **`Blocked`** ➔ `Active` *(Unblocked)*
- **`Resolved`** ➔ `Closed` *(PR reviewed & approved)*
- **`Resolved`** ➔ `Active` *(If code review requests changes)*
- **`Closed`** / **`Removed`** *(Terminal states)*

### 3. Bug Lifecycle Matrix
- **`New`** ➔ `Active`, `Removed` *(Dev starts investigation/fix)*
- **`Active`** ➔ `Resolved`, `Removed` *(Dev fixes bug, merges code to Test environment)*
- **`Resolved`** ➔ `Closed` *(Tester verifies fix succeeds)*
- **`Resolved`** ➔ `New` *(If Tester verification fails, returned to New)*
- **`Closed`** / **`Removed`** *(Terminal states)*

### 4. Epic Lifecycle Matrix
- **`New`** ➔ `Active` *(Automatically activated when first child User Story becomes Active)*
- **`Active`** ➔ `Resolved`, `Blocked` *(Tester transitions when all child User Stories & Technical Stories are Closed and pass Dev testing)*
- **`Blocked`** ➔ `Active` *(Unblocked)*
- **`Resolved`** ➔ `Closed` *(CPO/SM/PM transitions after Production release)*
- **`Closed`** *(Terminal state)*

### 5. Feature Lifecycle Matrix
- **`New`** ➔ `Active`
- **`Active`** ➔ `Resolved`, `Blocked`, `Removed`
- **`Blocked`** ➔ `Active`
- **`Resolved`** ➔ `Closed`
- **`Closed`** / **`Removed`** *(Terminal states)*

### 6. Technical Story Lifecycle Matrix
- **`New`** ➔ `Active` *(Dev pulls into Sprint; at least 1 Task active)*
- **`Active`** ➔ `Resolved`, `Blocked` *(All child Tasks and Bugs are Closed)*
- **`Blocked`** ➔ `Active` *(Unblocked)*
- **`Resolved`** ➔ `Closed` *(Confirmed in Sprint Review)*
- **`Closed`** *(Terminal state)*

### 7. Testcase Lifecycle Matrix
- **`New`** ➔ `Active`
- **`Active`** ➔ `Resolved`
- **`Resolved`** ➔ `Closed`
- **`Closed`** *(Terminal state)*

---

## ✍️ 5. Title Syntax Formatting & Description Rules

Per Section 5.5 of `spec_new.md`, all work item creation and updates must satisfy strict title syntax formatting and description structure rules enforced in `domain/validation.rs`:

| Work Item Type | Title Syntax Format Rule | Example Title | Mandatory Description Format |
| :--- | :--- | :--- | :--- |
| **`Epic`** | Must contain dash separator (`-`, `–`, `—`) in format:<br>`[Giá trị nghiệp vụ] – [Tác động chính]` | `Tăng hiệu suất bán hàng – Hỗ trợ nhân viên CSKH` | High-level strategic goal overview |
| **`Feature`** | Must contain dash separator in format:<br>`[Khả năng cụ thể] – [Module/Tính năng]` | `Tìm kiếm khách hàng nâng cao – Module CRM` | Feature capability description |
| **`User Story`** | Must contain dash separator and keyword `có thể` / `co the` / `can` in format:<br>`[Module] – [Vai trò] có thể [hành động]` | `[Dashboard] – Quản lý có thể xem báo cáo doanh thu theo tháng` | **Bắt buộc (Mandatory):**<br>`As a [User Role],`<br>`I want [Action / Goal],`<br>`So that [Business Value].` |
| **`Task`** / **`Technical Story`** | Must contain dash separator in format:<br>`[Module/Tính năng] - [Động từ + hành động]` | `Login UI - Tạo giao diện đăng nhập` | Technical execution notes |
| **`Testcase`** | Must start with `[`, contain `]`, `:`, and keyword `when` / `khi` in format:<br>`[Module]: [Expected result] when [condition]` | `[Auth]: Hiển thị thông báo lỗi khi nhập sai mật khẩu 3 lần` | Step-by-step execution steps |
| **`Bug`** | Must start with `[`, contain `]`, `:`, and keyword `when` / `khi` in format:<br>`[Module]: [Error message] when [reason]` | `[Payment]: Lỗi 500 Internal Server khi ấn thanh toán với giỏ hàng > 100 sản phẩm` | Repro Steps, Actual Result, Expected Result, Severity |

---

## 💻 6. CLI Command Usage Examples

All CLI subcommands are available via `harness-cli` (or wrapped by `./scripts/harness`):

### 1. `harness work-item add`
Create new portfolio work items:
```bash
# Add a User Story (validates title syntax and As a... I want... So that... description format)
./scripts/harness work-item add \
  --type "User Story" \
  --title "[Checkout] – Khách hàng có thể thanh toán qua QR VNPay" \
  --description "As a Khách hàng, I want thanh toán qua QR VNPay, So that giao dịch nhanh chóng và an toàn." \
  --priority p1 \
  --story-points 5 \
  --area-path "Payment" \
  --iteration-path "Sprint 12" \
  --tags "Payment,QR,FE" \
  --acceptance-criteria "1. Mã QR hiển thị trong 3s. 2. Xử lý IPN callback thành công."

# Add a Bug
./scripts/harness work-item add \
  --type "Bug" \
  --title "[Payment]: Lỗi 500 Internal Server khi quét mã QR với giỏ hàng trống" \
  --priority p0 \
  --severity "Critical" \
  --parent-id 1 \
  --repro-steps "1. Mở giỏ hàng. 2. Xóa hết SP. 3. Quét QR." \
  --actual-result "Internal server error 500." \
  --expected-result "Hiển thị thông báo giỏ hàng trống."

# Add a Task
./scripts/harness work-item add \
  --type "Task" \
  --title "Payment Backend - Tích hợp VNPay SDK API" \
  --assigned-to "dev_lead" \
  --remaining-work 8.5 \
  --parent-id 1
```

### 2. `harness work-item update`
Update work item status and attributes:
```bash
# Transition User Story from New -> Accepted (by BA)
./scripts/harness work-item update --id 1 --state Accepted

# Transition User Story from Accepted -> Active (by Dev)
./scripts/harness work-item update --id 1 --state Active --assigned-to "john_doe"

# Transition User Story from Active -> Resolved (after PR & test merge)
./scripts/harness work-item update --id 1 --state Resolved

# Transition User Story from Resolved -> Closed (after Sprint Review approval)
./scripts/harness work-item update --id 1 --state Closed
```

### 3. `harness work-item list` & `show`
Query and inspect work items:
```bash
# List all work items
./scripts/harness work-item list

# Filter work items by type and state
./scripts/harness work-item list --type "User Story" --state Active

# Show detailed fields of a specific work item by ID
./scripts/harness work-item show --id 1
```

### 4. `harness query matrix` / `tools` / `stats`
Query operational data tables:
```bash
# Query story test matrix and validation proof status
./scripts/harness query matrix
./scripts/harness query matrix --numeric

# Query registered machine-readable external tools
./scripts/harness query tools
./scripts/harness query tools --json
./scripts/harness query tools --responsibility "test_execution"

# Query database record statistics across all tables
./scripts/harness query stats
```

### 5. `harness tool register` & `remove`
Manage custom tools in the machine-readable registry:
```bash
# Register a custom tool
./scripts/harness tool register \
  --name "cargo-test-runner" \
  --command "cargo test -- --nocapture" \
  --description "Executes Rust unit and integration test suite" \
  --responsibility "test_execution" \
  --args "target:string:optional:Target test binary or package"

# Remove a registered tool
./scripts/harness tool remove --name "cargo-test-runner"
```

### 6. `harness migrate` & `init`
Database initialization and schema migration runner:
```bash
# Create SQLite database file if it does not already exist
./scripts/harness init

# Execute pending database migrations (001-init.sql through 006-work-item.sql)
./scripts/harness migrate
```

## 📥 6.1. Step-by-Step Installation Guide for Linux (Từng bước cài đặt trên Linux)

### 🔹 Phương án 1: Cài đặt tự động bằng Script (Khuyên dùng)

#### **Bước 1: Kiểm tra các công cụ tiền đề (Prerequisites)**
Đảm bảo hệ thống Linux của bạn đã có `curl`, `bash`, và `git`:
```bash
sudo apt update && sudo apt install -y curl bash git sqlite3
```

#### **Bước 2: Tải và chạy script cài đặt tự động**
Chạy lệnh bên dưới để tải và tích hợp Harness CLI vào dự án hiện tại:
```bash
# Cài đặt trực tiếp vào thư mục dự án hiện tại:
curl -fsSL https://raw.githubusercontent.com/baobao0303/harness/main/scripts/install-harness.sh | bash -s -- --yes

# Hoặc nếu dự án đã có sẵn code, dùng cờ --merge để không đè file cũ:
curl -fsSL https://raw.githubusercontent.com/baobao0303/harness/main/scripts/install-harness.sh | bash -s -- --merge --yes
```

#### **Bước 3: Thiết lập biến môi trường (.env)**
Tạo file cấu hình môi trường từ mẫu:
```bash
cp .env.example .env
```
*(Chỉnh sửa `.env` để cấu hình `HARNESS_MODEL` và `HARNESS_DB` nếu cần)*

#### **Bước 4: Khởi tạo Cơ sở dữ liệu SQLite**
Khởi tạo file cơ sở dữ liệu `harness.db` và chạy các file Migration schema (001 -> 006):
```bash
./scripts/harness init
./scripts/harness migrate
```

#### **Bước 5: Kiểm tra cài đặt thành công**
Chạy lệnh kiểm tra thống kê cơ sở dữ liệu và danh sách công việc:
```bash
./scripts/harness query stats
./scripts/harness work-item list
```

---

### 🔹 Phương án 2: Biên dịch từ mã nguồn (Build from Source bằng Rust)

#### **Bước 1: Cài đặt Rust toolchain và các thư viện cần thiết**
```bash
# Cài đặt Rust & Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# Cài đặt build tool
sudo apt update && sudo apt install -y build-essential pkg-config libsqlite3-dev
```

#### **Bước 2: Clone repository về máy**
```bash
git clone https://github.com/baobao0303/harness.git
cd harness
```

#### **Bước 3: Biên dịch binary `harness-cli` ở chế độ Release**
```bash
cargo build --release
```

#### **Bước 4: Copy binary biên dịch vào thư mục thi hành `scripts/bin/`**
```bash
mkdir -p scripts/bin
cp target/release/harness-cli scripts/bin/harness-cli
chmod +x scripts/bin/harness-cli
```

#### **Bước 5: Tạo file `.env` và Khởi tạo Cơ sở dữ liệu**
```bash
cp .env.example .env
./scripts/harness init
./scripts/harness migrate
```

#### **Bước 6: Xác nhận hoạt động**
```bash
./scripts/harness --help
./scripts/harness query stats
```

---

## 🛠️ 7. Build and Test Execution Instructions

### Compilation Commands:
```bash
# 1. Build debug binary
cargo build

# 2. Build optimized release binary
cargo build --release

# 3. Copy binary to scripts/bin/ wrapper path
mkdir -p scripts/bin && cp target/release/harness-cli scripts/bin/harness-cli
```

### Test Suite Commands:
```bash
# 1. Execute Rust unit and integration tests (25+ tests)
cargo test

# 2. Execute runner input validation test suite
python3 tests/test_runner_inputs.py

# 3. Execute End-to-End (E2E) test runner suite
python3 tests/e2e/run_tests.py
```

---

## 📄 Compliance Index

- **`spec_new.md`**: Full Software Development & Release Specification Manual (SM-QT-01..04, SM-QĐi-01, SM-QĐ-002, DDD-COMPANY-2026).
- **`AGENTS.md`**: Repository Agent Instructions & Harness Integration Shim.
- **`docs/HARNESS.md`**: Operating Rules & Framework Conventions.
- **`scripts/schema/006-work-item.sql`**: Unified Work Item Database Migration.
