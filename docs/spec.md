# Technical Specification: Tích Hợp JEV Decision Model Vào Hệ Thống Harness + Hermes + Paseo Server

> **Vai trò:** Senior Software Architect / Staff Engineer  
> **Process Authority:** Hermes + `SOUL.md` (Điều phối 5 Agents: BA, Scrum Master, BE, FE, Reviewer)  
> **Management, Gateway & Approval Server:** **Paseo Server** (Trung tâm gọi JEV, Quản lý Pool, Dashboard giám sát Agent & Cổng duyệt Human-in-the-Loop)  
> **Orchestration & Audit Layer:** Harness (`harness-cli`, `harness.db`, Workflows, Durable State)  
> **Decision & Classification Engine:** JEV / Kev (System One API `POST /v1/systemone`)  
> **Input Contract:** Approved Specification  
> **Primary Endpoint:** `http://100.124.36.46:20128/v1/systemone` | **Model:** `oc/jev-1.13-free`

---

## 1. 📌 Kiến Trúc Trung Tâm Hóa: Paseo Server Làm Gateway Điều Phối

Đưa toàn bộ việc kết nối **JEV Decision Engine** lên **Paseo Server** làm cổng tập trung (Gateway). Điều này giúp:
1. **Kiểm soát nghẽn Pool 100%:** Chỉ duy nhất Paseo Server quản lý Concurrency Lock, Rate Limiter và Decision Cache khi gọi JEV.
2. **Giao tiếp 2 chiều rõ ràng giữa Paseo và Hermes:** Thông qua **REST API + WebSocket Stream**.

```text
                               ┌────────────────────────────────┐
                               │       HUMAN / DEVELOPER        │
                               │  (Phê duyệt Task trên Paseo)   │
                               └───────────────┬────────────────┘
                                               │
                                      [Bấm APPROVE / REJECT]
                                               ▼
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                                     PASEO SERVER                                       │
│  1. JEV Central Gateway: Trực tiếp gọi JEV, quản lý Rate Limit, Cache & Fallback       │
│  2. Human Approval Gate: Hiển thị Task chờ duyệt (Pending Approval)                    │
│  3. Agent Dispatcher: Gửi lệnh kích hoạt sang Hermes qua REST API                      │
│  4. Live Event Receiver: Nhận WebSocket Stream tiến độ & log từ 5 Agents              │
└──────────────┬───────────────────────────────▲───────────────────────────────┬─────────┘
               │ (1. Gọi JEV phân loại)        │ (4. WebSocket Stream)         │ (3. HTTP Dispatch)
               ▼                               │ (Status, Log, Heartbeat)      ▼
┌──────────────────────────────┐               │               ┌─────────────────────────┐
│     JEV DECISION ENGINE      │               │               │     HERMES RUNTIME      │
│     (POST /v1/systemone)     │               │               │        (SOUL.md)        │
│  - Phân loại Work Item       │               │               │                         │
│  - Gán Risk Lane & Priority  │               │               │ ┌─────────────────────┐ │
│  - Trả kết quả về Paseo      │               │               │ │ BA ──► Scrum ──► BE │ │
└──────────────┬───────────────┘               │               │ │         │           │ │
               │                               │               │ │         ▼           │ │
               │ (Ghi nhận kết quả)            │               │ │       FE ──► Review │ │
               ▼                               │               │ └─────────────────────┘ │
┌──────────────────────────────────────────────┴───────────────┴─────────────────────────┐
│                               HARNESS DURABLE LAYER                                    │
│  - Database: harness.db (SQLite)                                                       │
│  - Quản lý: Work Items, Test Matrix, Decisions, Audit Traces                           │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. 🔌 Cơ Chế Giao Tiếp 2 Chiều: Paseo Server ⇄ Hermes (5 Agents)

Hai bên nói chuyện với nhau thông qua **2 kênh giao tiếp chuẩn Web**:

```text
                                  [PASEO SERVER]
                                   │          ▲
                                   │          │
   (1) HTTP POST /api/tasks/dispatch          │ (2) WebSocket /ws/agent-events
   (Kích hoạt Agent sau khi BẠN DUYỆT)        │ (Agent gửi log, tiến độ, heartbeat)
                                   │          │
                                   ▼          │
                                  [HERMES RUNTIME]
                                (5 Agents Fleet)
```

### Kênh 1: Paseo $\longrightarrow$ Hermes (REST API Dispatcher)
* Khi bạn bấm **APPROVE** một task trên Paseo Dashboard:
* Paseo Server gửi một HTTP POST request sang Hermes Endpoint:
  ```http
  POST http://hermes-runtime:8080/api/tasks/dispatch
  Content-Type: application/json

  {
    "task_id": "ECOM-CART-01",
    "target_agent": "BE",
    "lane": "normal",
    "priority": "P1",
    "spec_path": "docs/stories/ECOM-CART-01.md",
    "approved_by": "bao312"
  }
  ```
* Hermes nhận request, gán context và đánh thức **BE Agent** bắt đầu chạy.

### Kênh 2: Hermes $\longrightarrow$ Paseo (WebSocket Live Stream & Heartbeat)
* Khi 5 Agent (BA, Scrum, BE, FE, Reviewer) thực thi:
* Hermes mở kết nối **WebSocket (`ws://paseo-server:3000/ws/agent-events`)** để bắn event liên tục về Paseo:
  ```json
  {
    "event_type": "AGENT_PROGRESS",
    "agent_role": "BE",
    "task_id": "ECOM-CART-01",
    "status": "RUNNING",
    "current_action": "Viết Unit Test cho CartController.test.ts",
    "progress_percent": 65,
    "timestamp": "2026-09-26T22:20:00Z"
  }
  ```
* Dashboard Paseo cập nhật màn hình theo thời gian thực (Live UI) để bạn biết chính xác con agent nào đang làm gì.

---

## 3. 🎯 Paseo Trực Tiếp Gọi JEV Decision Engine (Centralized Gateway)

Thay vì để 5 Agent gọi JEV phân mảnh gây nghẽn pool:
1. **Spec nạp vào Paseo**: Paseo Server lấy spec và gọi JEV qua `http://100.124.36.46:20128/v1/systemone`.
2. **Paseo quản lý Rate Limit & Cache**:
   * Kiểm tra SHA-256 cache trước khi gọi.
   * Giới hạn Semaphore 1 request đồng thời.
3. **Paseo nhận kết quả**: JEV trả về `input_type`, `predicted_lane`, `priority`.
4. **Paseo gán trạng thái `PENDING_APPROVAL`**: Đưa lên Dashboard để bạn bấm duyệt.

---

## 4. 🚦 Quy Trình Toàn Cục Từ Đầu Đến Cuối (Full Lifecycle)

$$\text{Spec} \stackrel{\text{Paseo gọi JEV}}{\longrightarrow} \text{Pending Approval} \stackrel{\text{Bạn bấm Approve}}{\longrightarrow} \stackrel{\text{Paseo gọi HTTP Dispatch}}{\longrightarrow} \text{Hermes (BE/FE)} \stackrel{\text{WebSocket Stream}}{\longrightarrow} \text{Paseo Dashboard} \longrightarrow \text{Resolved}$$

1. **Tiếp nhận & Ra quyết định:** Spec vào Paseo $\rightarrow$ Paseo gọi JEV $\rightarrow$ JEV trả kết quả phân loại trong < 500ms.
2. **Cổng duyệt Human-in-the-Loop:** Paseo hiển thị thông tin task $\rightarrow$ Bạn kiểm tra và bấm **APPROVE**.
3. **Kích hoạt thực thi:** Paseo gửi HTTP POST sang Hermes $\rightarrow$ Hermes cấp quyền cho BE/FE Agent code.
4. **Giám sát thời gian thực:** Hermes bắn WebSocket event về Paseo $\rightarrow$ Paseo hiển thị thanh tiến độ và live log.
5. **Review & Hoàn tất:** Reviewer Agent duyệt code $\rightarrow$ Ghi trace vào `harness.db` $\rightarrow$ Paseo đóng task (`Closed`).
