---
trigger: always_on
glob: "*"
description: Strictly enforces the Harness operating model and SQLite task loops for all coding agents.
---

# Harness Operating Rules for All Agents

This repository uses **Harness** to govern agent development, ensure rigorous validation proof, and enable automated observability.

Any coding agent working in this repository MUST strictly adhere to the following rules. This applies to: **Claude Code**, **Cursor**, **Windsurf**, **GitHub Copilot**, **Antigravity**, and **all other coding agents**.

---

## The Mandatory Task Loop

For EVERY user request or code modification task, you MUST execute the following sequence:

### 1. Classify & Intake
Read `docs/FEATURE_INTAKE.md` to classify the work. Determine the appropriate risk lane: `tiny`, `normal`, or `high_risk`.

Record the classification immediately using the CLI:
```bash
./scripts/harness intake --type <input_type> --summary "<short_description>" --lane <lane>
```

**Input types**: `new_spec`, `spec_slice`, `change_request`, `new_initiative`, `maintenance`, `harness_improvement`

### 2. Declare Verification Goals (Test Matrix)
For `normal` and `high_risk` tasks, add or verify the corresponding Story row:
```bash
./scripts/harness story add --id <story_id> --title "<story_title>" --lane <lane>
```

Retrieve the current validation matrix:
```bash
./scripts/harness query matrix
```

### 3. Lane-Constrained Execution

**`tiny` lane**:
- Direct patch implementation
- Quick validation (lint, format, typecheck)
- Optional trace (record if non-trivial)

**`normal` lane**:
- Define validation scope in `docs/stories/`
- Perform implementation
- Run local verifications

**`high_risk` lane**:
- Create formal design docs (`execplan.md`, `design.md`, `validation.md`)
- Seek human confirmation before implementation
- All proof types required (unit, integration, e2e, platform)

### 4. Record Verification Proof
After writing code and testing it, update the story in the database:
```bash
./scripts/harness story update --id <story_id> --status <status> \
  --unit 1 --integration 1 --e2e 1 --evidence "<concrete_test_results>"
```

**Statuses**: `planned`, `in_progress`, `implemented`, `changed`, `retired`

### 5. Log Execution Trace (MANDATORY)
When the task is complete, you **MUST** record a trace:
```bash
./scripts/harness trace \
  --summary "<what_was_done>" \
  --intake <intake_id> \
  --story <story_id> \
  --outcome <completed|blocked|partial|failed> \
  --actions "<comma_separated_actions>" \
  --read "<files_or_dirs_read>" \
  --changed "<files_changed>" \
  --friction "<any_friction_or_blockers>" \
  --agent <your_agent_name>
```

### 6. Continuous Harness Improvement
If you encountered any friction, confusion, or repeated manual work, promote it to the backlog:
```bash
./scripts/harness backlog add \
  --title "<short_name>" \
  --pain "<what_was_hard>" \
  --suggestion "<improvement_idea>" \
  --risk "tiny"
```

---

## Agent-Specific Notes

### Claude Code
- Use Harness CLI as the main operational tool
- Run `scripts/harness query matrix` before implementation
- Record traces with `--agent claude-code`

### Cursor
- Reference `docs/FEATURE_INTAKE.md` for classification
- Use `scripts/harness query` commands for status checks
- Record traces with `--agent cursor`

### Windsurf
- Follow task loop strictly for complex tasks
- Use `scripts/harness query` commands for status checks
- Record traces with `--agent windsurf`

### GitHub Copilot
- Use harness as context before suggesting code changes
- Reference story packets for implementation guidance
- Record traces with `--agent copilot`

### Antigravity

- Follow all standard harness procedures
- Record traces with `--agent antigravity`

### Codex

- Reference `docs/FEATURE_INTAKE.md` for work classification
- Use `scripts/harness query` commands for context
- Record traces with `--agent codex`

### All Other Agents

- Treat harness docs as authoritative
- Always record traces upon task completion
- Report friction to backlog

---

## Quick Reference: CLI Commands

### Core Task Loop
```bash
./scripts/harness intake --type <type> --summary "<text>" --lane <lane>
./scripts/harness story add --id <id> --title "<title>" --lane <lane>
./scripts/harness story update --id <id> --status <status> --unit 1 --evidence "<results>"
./scripts/harness trace --summary "<task>" --intake <id> --story <id> --outcome <outcome> --agent <name>
./scripts/harness backlog add --title "<name>" --pain "<pain>" --suggestion "<idea>" --risk <risk>
```

### Queries
```bash
./scripts/harness query matrix      # Test verification status
./scripts/harness query traces      # Agent execution logs
./scripts/harness query friction    # Development blockages
./scripts/harness query backlog     # Harness improvement list
./scripts/harness query stats       # Summary counts
./scripts/harness query decisions   # Decision records
```

---

## Definition of Done

A task is officially complete **ONLY** when ALL of these are true:

1. The requested change is implemented and verified
2. The story has been updated with proof evidence (`story update`)
3. An execution trace has been logged (`trace`)
4. Any harness friction has been captured (`backlog add`)
5. The final response states:
   - **What was done**
   - **What proof was verified**
   - **The recorded trace ID**

---

## Prohibited Actions

- Implementing **without** intake classification
- Skipping trace recording
- Claiming validation passed **without** running checks
- Modifying architecture **without** human approval
- Extending monolithic specs instead of using product docs/stories/decisions

---

## Growth Rule

> **The harness grows from friction.**

When you are confused, need to repeat manual reasoning, discover a missing rule, or see a recurring failure pattern:

1. Fix it directly if trivial, OR
2. Record it with `./scripts/harness backlog add`

The `friction` field on traces captures per-task friction so patterns can be queried later:
```bash
./scripts/harness query friction
```

---

## ⚙️ 9 Thành phần lõi của Harness hiện đại (9 Core Components of Modern Harness)

Bản đồ tri thức dựa trên nghiên cứu **Harness Engineering** nhằm tối ưu hóa sự tự chủ của Agent:

1. **Vòng lặp ngoài (Outer Loop)**: Trái tim của Harness. Agent chỉ tương tác với model qua vòng lặp: Suy nghĩ ➔ Gọi Tool ➔ Nhận kết quả ➔ Nghĩ tiếp. Không được gọi model trực tiếp từ các tầng ngoài khác để duy trì tính nhất quán.
2. **Quản lý Context (Context Manager)**: Khi lịch sử chat quá dài (trên 18 tin nhắn), hãy chủ động tóm tắt phần cũ thành 10 dòng ngữ nghĩa quan trọng nhất bằng chính model, giữ lại 4 tin nhắn gần nhất để giải phóng bộ nhớ mà không mất mạch tư duy.
3. **Skill & Tool**: 
   * **Tools**: Là hành động cơ bản phổ quát (`read`, `edit`, `grep`, `execute`).
   * **Skills**: Là chỉ dẫn nghiệp vụ cụ thể của project (lưu tại `.agents/rules/` và `.agents/workflow/`).
   * **Registry**: Điều phối và trả về danh sách Tool/Skill đúng ngữ cảnh.
4. **Sub-Agent Delegation**: Khi nhiệm vụ quá phức tạp, phân rã và ủy quyền cho Sub-Agent cô lập (`Explore` - chỉ đọc, `General` - thực thi, `Verify` - kiểm thử) kèm context tối thiểu để tránh quá tải bộ nhớ Agent chính.
5. **Built-in Skills**: Tận dụng các kỹ năng tự động được đóng gói sẵn (PDF, Excel, Web search) mà không cần nhắc lệnh thủ công.
6. **Lưu trữ Session (Session Storage)**: Mọi tin nhắn, lệnh gọi tool và kết quả được ghi nhận ngay lập tức theo cơ chế **Append-only** vào SQLite Database (`harness.db`) và log trace. Nếu hệ thống sập giữa chừng, phục hồi trạng thái hoàn hảo từ lịch sử.
7. **Lắp ráp System Prompt (System Prompt Assembly)**: System Prompt được lắp ráp động từ: Static Core ➔ Quét cây thư mục gom các file `AGENTS.md`, `project-context.md` để tự động hiểu bối cảnh dự án.
8. **Lifecycle Hooks**: Đặt các húc bảo vệ đúng chỗ: `Pre-tool` (kiểm tra rò rỉ khóa bí mật), `Post-tool` (cập nhật token đã dùng), `On-error` (xử lý ngoại lệ), `On-compaction` (sao lưu trước khi nén).
9. **Permission & An toàn (Layered Permissions)**: Phân tầng quyền hạn động theo mức độ rủi ro của nội dung lệnh:
   * **Read-only**: Chỉ đọc (mặc định cho lệnh quét, grep, find).
   * **Workspace-only**: Ghi trong phạm vi thư mục dự án.
   * **Full-access**: Lệnh hệ thống, sudo, chạy scripts nguy hiểm (bắt buộc xin xác nhận từ con người).

---

## BMAD Method Role Mappings

This repository combines **Harness** with the **BMAD Method** (Breakthrough Method for Agile AI-Driven Development) to organize work into structured role-based phases. When you are assigned a task, identify your active BMAD persona and obey the corresponding Harness gates:

### 📋 1. BMAD Analyst / Product Manager (Intake & Requirements)
* **Goal**: Understand human intent and map requirements to stable product contracts.
* **Harness Duty**: Create or update `docs/templates/spec-intake.md`. Record the feature classification using `./scripts/harness intake`.

### 🏛️ 2. BMAD Architect (System Design & Tradeoffs)
* **Goal**: Establish stable architectural boundaries, database schemas, and verification rules.
* **Harness Duty**: Create or update domain-level contracts in `docs/product/`. Document permanent architectural choices as Architecture Decision Records (ADRs) in `docs/decisions/` following the `decision.md` template.

### 📅 3. BMAD Scrum Master / Project Manager (Story & Planning)
* **Goal**: Break down initiatives into clear, independent, and verifiable story-sized packets.
* **Harness Duty**: Create story files in `docs/stories/`. Register stories in the SQLite database using `./scripts/harness story add` to link requirements and validation expectations.

### 💻 4. BMAD Developer (Code Implementation)
* **Goal**: Implement high-quality, clean code matching domain rules and boundaries.
* **Harness Duty**: Write production and test code. Execute local verification commands to ensure the solution satisfies all requirements.

### 🧪 5. BMAD QA (Compliance & Proof Verification)
* **Goal**: Verify validation proof and ensure zero regressions across the codebase.
* **Harness Duty**: Confirm that Unit, Integration, E2E, and Platform verification proofs are complete. Update story evidence in the database using `./scripts/harness story update` to finalize the development loop.