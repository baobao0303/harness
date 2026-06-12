# Hướng dẫn vẽ Sơ đồ UML bằng Mermaid trong Harness

Mermaid là một công cụ mạnh mẽ giúp chuyển đổi mã văn bản thành sơ đồ trực quan (Flowchart, Sequence, Class, State, ER Diagrams). 

Tất cả tài liệu của Harness sử dụng Mermaid vì nó **dễ đọc, lưu trữ dưới dạng text trong Git** và tự động render trực quan trên GitHub hoặc các IDE như Cursor/Windsurf.

---

## 📌 Các quy tắc chung để tránh lỗi cú pháp
1.  **Sử dụng dấu ngoặc kép**: Nếu nhãn của nút (node label) chứa các ký tự đặc biệt như dấu đóng mở ngoặc `( )`, `[ ]`, hoặc khoảng trắng, hãy bọc nhãn đó trong dấu ngoặc kép.
    *   *Sai*: `A[Hàm main()]`
    *   *Đúng*: `A["Hàm main()"]`
2.  **Tránh thẻ HTML**: Không sử dụng các thẻ HTML (`<br>`, `<b>`) trực tiếp trong nhãn trừ khi thực sự cần thiết, để tránh lỗi parser.
3.  **Hướng vẽ sơ đồ (Flowchart Direction)**:
    *   `TD` hoặc `TB`: Vẽ từ trên xuống dưới (Top-Down / Top-Bottom).
    *   `LR`: Vẽ từ trái sang phải (Left-to-Right).
    *   `RL`: Vẽ từ phải sang trái.

---

## 📊 1. Sơ đồ Luồng Công việc (Flowchart)
Dùng để vẽ quy trình nghiệp vụ, luồng xử lý hoặc cấu trúc thư mục/layer của hệ thống.

### Cú pháp & Ví dụ:
````markdown
```mermaid
graph TD
    %% Định nghĩa các node
    A[Bắt đầu] --> B{Phân loại rủi ro?}
    
    %% Nhánh rẽ quyết định
    B -- Tiny --> C["Chạy thẳng Dev"]
    B -- High-risk --> D["Tạo spec-intake & ADR"]
    
    %% Nối nút
    C --> E[Hoàn thành]
    D --> E
    
    %% Tô màu cho nút để tăng thẩm mỹ
    style B fill:#f9f,stroke:#333,stroke-width:2px
    style D fill:#fbb,stroke:#333,stroke-width:2px
```
````

---

## 🔄 2. Sơ đồ Tuần tự (Sequence Diagram)
Dùng để mô tả sự tương tác giữa các thành phần hệ thống hoặc giữa các Agent theo mốc thời gian.

### Cú pháp & Ví dụ:
````markdown
```mermaid
sequenceDiagram
    autonumber
    actor User as Con người
    participant PM as PM Agent
    participant DB as SQLite DB
    
    User->>PM: Yêu cầu tính năng mới
    Note over PM: Phân loại rủi ro và tạo Story
    PM->>DB: Thực hiện lệnh 'story add'
    DB-->>PM: Ghi nhận dữ liệu thành công (ID: US-001)
    PM-->>User: Báo cáo Story đã được đăng ký
```
````

---

## 🏫 3. Sơ đồ Lớp (Class Diagram)
Dùng để mô tả cấu trúc lớp dữ liệu (structs/classes) trong mã nguồn hoặc các mối quan hệ sở hữu.

### Cú pháp & Ví dụ:
````markdown
```mermaid
classDiagram
    class Story {
        +String id
        +String title
        +String lane
        +String priority
        +update_status(status) void
    }
    
    class VerificationProof {
        +Boolean unit
        +Boolean integration
        +Boolean e2e
        +String evidence
    }
    
    %% Quan hệ: Một Story có 1 VerificationProof (Composition)
    Story *-- VerificationProof
```
````

---

## 🔄 4. Sơ đồ Trạng thái (State Diagram)
Dùng để thể hiện vòng đời hoặc các trạng thái chuyển đổi của một thực thể (ví dụ: vòng đời của một User Story).

### Cú pháp & Ví dụ:
````markdown
```mermaid
stateDiagram-v2
    [*] --> Planned : Đăng ký (story add)
    Planned --> InProgress : Bắt đầu Dev (story update)
    InProgress --> Implemented : Chạy test nội bộ đạt
    
    state Implemented {
        [*] --> RunVerification
        RunVerification --> Pass : Hợp lệ
        RunVerification --> Fail : Lỗi
    }
    
    Fail --> InProgress : Sửa lỗi (Fixing)
    Pass --> [*] : Hoàn thành kiểm chứng
```
````

---

## 🗄️ 5. Sơ đồ Quan hệ Thực thể (ER Diagram - Database Schema)
Dùng để mô tả các bảng trong cơ sở dữ liệu (`harness.db`) và các mối quan hệ khóa ngoại.

### Cú pháp & Ví dụ:
````markdown
```mermaid
erDiagram
    INTAKE ||--o{ STORY : "phát sinh"
    STORY {
        string id PK
        string title
        string lane
        string priority
    }
    DECISION {
        string id PK
        string title
        string status
    }
    TRACE {
        integer id PK
        string summary
        string agent
        string story_id FK
    }
    
    STORY ||--o{ TRACE : "chứa các vết"
```
````

---

## 🎨 6. Cách tu chỉnh giao diện sơ đồ (Styling)
Bạn có thể tùy chỉnh màu sắc các nút trong `Flowchart` để sơ đồ trông hiện đại và đồng bộ với giao diện của Harness:
- **Màu xanh lục (Harness Green)**: `#d4edda` (Dành cho trạng thái thành công/pass).
- **Màu đỏ (Warning/High-risk)**: `#f8d7da` (Dành cho rủi ro cao hoặc lỗi).
- **Màu xanh dương (Info)**: `#cce5ff` (Dành cho nút thông tin).

Cách khai báo:
```mermaid
style NodeID fill:#d4edda,stroke:#28a745,stroke-width:2px
```
