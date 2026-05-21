# Harness Experimental - Tổng quan

## Harness là gì?

**Harness** là một repository-level operating framework giúp coding agents (Claude Code, Codex, Cursor...) hiểu project context trước khi thay đổi code.

> The app is what users touch. The harness is what agents touch.

## Vấn đề mà Harness giải quyết

Khi một coding agent enters một repo, nó thường chỉ có:
- Chat prompt
- Shallow snapshot của files

Điều này dẫn đến các failure modes:
- Agent edit code trước khi hiểu product intent
- Constraints quan trọng chỉ nằm trong chat history
- Validation expectations mơ hồ
- Architecture tradeoffs được lặp lại thay vì inherited
- Large requests không được break thành reviewable story-sized work

## Mental Model

```
Human intent → Feature intake → Story packet → Agent work → Validation proof → Harness delta
```

## Cấu trúc chính

```
harness/
├── AGENTS.md                    # Stable agent shim - entrypoint cho agents
├── docs/
│   ├── HARNESS.md               # Human-agent collaboration model
│   ├── FEATURE_INTAKE.md        # Work classification (tiny/normal/high-risk)
│   ├── ARCHITECTURE.md          # Architecture discovery & boundary rules
│   ├── TEST_MATRIX.md           # Behavior-to-proof validation expectations
│   ├── HARNESS_BACKLOG.md       # Backlog cho harness improvements
│   ├── decisions/               # Decision records & tradeoffs
│   ├── stories/                 # Story packets
│   └── product/                 # Product contract files
├── src/                         # Rust CLI source
└── scripts/
    ├── harness                  # Main entrypoint (shell script)
    └── bin/harness-cli          # Prebuilt Rust binary
```

## Các thành phần quan trọng

### 1. Feature Intake (docs/FEATURE_INTAKE.md)
Phân loại công việc theo độ rủi ro:
- **Tiny**: Low-risk, straightforward changes
- **Normal**: Standard feature/fix work
- **High-risk**: Significant changes cần nhiều review hơn

### 2. Story Packets (docs/stories/)
Mỗi task được break thành "story-sized" work packets chứa:
- Mục tiêu và scope
- Validation proof expectations
- Status tracking

### 3. Test Matrix (docs/TEST_MATRIX.md)
Mapping behavior → proof expectations để validate work done.

### 4. Decision Records (docs/decisions/)
Lưu lại các quyết định kiến trúc và tradeoffs để future agents inherit.

### 5. Durable Layer (SQLite)
Dùng `harness-cli` để query/store:
- Intake classifications
- Story status
- Execution traces
- Backlog items

## CLI Commands

```bash
# Initialize
scripts/bin/harness-cli init

# Record work
scripts/bin/harness-cli intake --type <type> --summary <text> --lane <lane>
scripts/bin/harness-cli story add --id <id> --title <text> --lane <lane>
scripts/bin/harness-cli story update --id <id> --status <status>
scripts/bin/harness-cli trace --summary <text> --outcome <outcome>

# Query
scripts/bin/harness-cli query matrix
scripts/bin/harness-cli query backlog
scripts/bin/harness-cli query stats
scripts/bin/harness-cli query friction
```

## Validation Ladder

Khi implementation bắt đầu, expected validation commands:

```text
validate:quick     → format, lint, typecheck, unit tests
test:integration   → backend, database, provider checks
test:e2e           → user-visible end-to-end flows
test:platform      → shell, mobile, desktop, deployment smoke
test:release       → full suite, log checks, performance smoke
```

## Growth Rule

> The harness grows from friction.

Khi agent gặp confusion, cần record lại friction:
```bash
scripts/bin/harness-cli backlog add --title "<short name>" --pain "<what was hard>"
```

## Current State

**Harness v0** - Hiện tại repository này chỉ chứa harness infrastructure, chưa có application implementation. Product contract sẽ được thêm khi user cung cấp specification.

## Mục tiêu cuối cùng

Một repo bắt đầu có "harness" khi nó giúp agent trả lời được các câu hỏi thực tế:
- What should I read first?
- What type of work is this?
- Which product contract does it affect?
- How risky is the change?
- What proof will show the work is done?
- What decision should future agents inherit?