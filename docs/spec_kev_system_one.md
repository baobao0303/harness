# Kev: TypeSafe System One Small Decision Models Specification

**Tài liệu**: Đặc tả Kỹ thuật Tích hợp & Vận hành Họ Mô hình Quyết định Kev (TypeSafe System One)  
**Phiên bản**: 2.0.0  
**Tác giả**: Supervisor & SLPC Architecture Team  
**Mục tiêu**: Chuẩn hóa wire protocol, schema dữ liệu, cơ chế suy luận prefill-only và chính sách điều phối rủi ro hai cấp (Two-Tier Supervision) cho Lambda & Autonomous Agents.

---

## 1. Bản Chất Kiến Trúc & Sự Khác Biệt (Paradigm Shift)

Khác biệt cốt lõi giữa **Kev (TypeSafe System One)** và các mô hình sinh ngôn ngữ tự do (Autoregressive Generative LLMs như ChatGPT, Claude):

| Tiêu chí | Generative LLM (ChatGPT / Claude) | Kev (TypeSafe System One) |
| :--- | :--- | :--- |
| **Cơ chế suy luận** | Autoregressive (giải mã tuần tự từng token). | **Prefill-Only** (một forward pass duy nhất qua mạng nơ-ron). |
| **Dạng Prompt** | Văn bản tự do (free-form prose, markdown). | **Cấu trúc 2 phần**: `state` (ngữ cảnh) + `questions` (bộ câu hỏi). |
| **Độ trễ (Latency)** | Cao (1,000ms – 5,000ms phụ thuộc độ dài output). | **Siêu tốc (<50ms)** nhờ không tốn chi phí decoding. |
| **Dữ liệu đầu ra** | Chuỗi văn bản / Markdown cần parse regex/JSON. | **Xác suất hiệu chuẩn (Calibrated Probabilities)** và JSON định kiểu chặt. |
| **Độ tin cậy** | Có thể bị ảo giác (hallucination), không rõ độ tự tin. | Đạt chuẩn xác suất Brier score cực thấp, kèm `confidence` chính xác. |
| **Xử lý đa tác vụ** | Phải hỏi từng câu hoặc dùng chain-of-thought dài. | **Gộp đồng thời nhiều câu hỏi** trong 1 request duy nhất. |

---

## 2. Giao Thức Wire Protocol (`POST /v1/systemone`)

Mọi yêu cầu gửi tới server Kev đều tuân theo chuẩn **TypeSafe System One Envelope**.

### 2.1. Cấu Trúc Request (`SystemOneRequest`)
Payload bao gồm đúng 2 trường:
1. `state` (`string`): Toàn bộ ngữ cảnh, tài liệu, prompt, log lỗi hoặc tin nhắn người dùng.
2. `questions` (`map<string, Question>`): Bảng ánh xạ các câu hỏi cần mô hình ra quyết định.

### 2.2. Ba Kiểu Câu Hỏi Cốt Lõi (Question Primitives)

```text
                                  ┌───────────────────────────────┐
                                  │   SystemOneRequest Envelope   │
                                  │   - state: "Ngữ cảnh bài toán"│
                                  └───────────────┬───────────────┘
                                                  │
                 ┌────────────────────────────────┼────────────────────────────────┐
                 ▼                                ▼                                ▼
   ┌───────────────────────────┐    ┌───────────────────────────┐    ┌───────────────────────────┐
   │     1. NOUL (Yes/No)      │    │    2. CHOICE (Routing)    │    │     3. SCORE (Rating)     │
   │ - instructions            │    │ - instructions            │    │ - instructions            │
   │                           │    │ - criteria: map<k, desc>  │    │ - criteria: array<level>  │
   └─────────────┬─────────────┘    └─────────────┬─────────────┘    └─────────────┬─────────────┘
                 │                                │                                │
                 ▼                                ▼                                ▼
   ┌───────────────────────────┐    ┌───────────────────────────┐    ┌───────────────────────────┐
   │ noul: 0.0 - 1.0           │    │ choice: "option_key"      │    │ score: 0.0 - N.0          │
   │ (Xác suất tiên nghiệm)    │    │ confidence: 0.97          │    │ legend: map<0..N, label>  │
   │                           │    │ probabilities: map<k, p>  │    │ (Điểm số định lượng)      │
   └───────────────────────────┘    └───────────────────────────┘    └───────────────────────────┘
```

1. **`noul` (Yes / No Hypothesis)**:
   - Đánh giá tính đúng/sai của một mệnh đề đối với `state`.
   - Trả về số thực `noul` nằm trong khoảng $[0.0, 1.0]$ đại diện cho xác suất khẳng định (Yes).
2. **`choice` (Multi-Choice Intent Routing)**:
   - Phân loại ngữ cảnh vào một trong các nhãn định sẵn.
   - Trường `criteria`: Bản đồ ánh xạ giữa `id_lựa_chọn` và `mô_tả_tiêu_chí`.
   - Trả về nhãn được chọn (`choice`), độ tự tin (`confidence`), và phân phối xác suất trên toàn bộ các nhãn (`probabilities`).
3. **`score` (Ordinal Scale / Multi-level Rating)**:
   - Đánh giá mức độ nghiêm trọng, độ khẩn cấp hoặc chất lượng theo thang điểm tăng dần.
   - Trường `criteria`: Mảng chuỗi định nghĩa ý nghĩa từng mốc điểm (mốc $0, 1, 2, 3\dots$).
   - Trả về điểm số liên tục (`score`) kèm bản đồ giải thích mốc (`legend`).

---

## 3. Request & Response Thực Tế (Golden Master Example)

### 3.1. Request Mẫu (Gộp cả 3 loại câu hỏi trong 1 forward pass)

```bash
curl -s -X POST http://100.124.36.46:8008/v1/systemone \
  -H "Content-Type: application/json" \
  -d '{
    "state": "User message: Sắp hết hạn hợp đồng chưa? Cho tôi hủy gói Premium ngay lập tức.",
    "questions": {
      "is_churn_risk": {
        "type": "noul",
        "instructions": "Does the user intend to cancel or stop using the service?"
      },
      "intent_category": {
        "type": "choice",
        "instructions": "Classify the primary intent of this support request.",
        "criteria": {
          "billing": "Inquiries about invoices, payments, subscriptions or cancellation",
          "technical": "Issues with bugs, app errors, or deployment",
          "feature_request": "Asking for new functionality or integrations"
        }
      },
      "urgency_rating": {
        "type": "score",
        "instructions": "How urgent is this customer request?",
        "criteria": [
          "Low: can wait days",
          "Medium: normal queue within 24h",
          "High: needs attention today",
          "Critical: immediate action required"
        ]
      }
    }
  }' | jq .
```

### 3.2. Output Thực Tế Trả Về Từ Mô Hình

```json
{
  "model": "kev-latest",
  "answers": {
    "is_churn_risk": {
      "type": "noul",
      "noul": 0.7616
    },
    "intent_category": {
      "type": "choice",
      "choice": "billing",
      "confidence": 0.9773,
      "probabilities": {
        "billing": 0.9849,
        "feature_request": 0.0086,
        "technical": 0.0065
      }
    },
    "urgency_rating": {
      "type": "score",
      "score": 1.992,
      "legend": {
        "0": "Low: can wait days",
        "1": "Medium: normal queue within 24h",
        "2": "High: needs attention today",
        "3": "Critical: immediate action required"
      }
    }
  }
}
```

**Phân tích ngữ nghĩa kết quả**:
* `is_churn_risk = 0.7616`: $76.16\%$ xác suất khách hàng sắp rời bỏ dịch vụ.
* `intent_category = "billing"`: $97.73\%$ độ tự tin xếp vào luồng hỗ trợ thanh toán / hóa đơn.
* `urgency_rating = 1.992`: Xấp xỉ mốc $2.0$ (Mức "High: needs attention today" - Cần xử lý ngay trong ngày).

---

## 4. Hiện Thực Hóa Trong Rust Crate (`harness-cli`)

Các cấu trúc dữ liệu được hiện thực hóa kiểu tĩnh hoàn toàn (TypeSafe) tại `crates/harness-cli/src/domain/decision.rs`:

```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SystemOneQuestion {
    #[serde(rename = "choice")]
    Choice {
        instructions: String,
        criteria: HashMap<String, String>,
    },
    #[serde(rename = "noul")]
    Noul { instructions: String },
    #[serde(rename = "score")]
    Score {
        instructions: String,
        criteria: Vec<String>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemOneRequest {
    pub model: String,
    pub state: String,
    pub questions: HashMap<String, SystemOneQuestion>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemOneAnswer {
    #[serde(rename = "type")]
    pub answer_type: String,
    pub choice: Option<String>,
    pub confidence: Option<f64>,
    pub probabilities: Option<HashMap<String, f64>>,
    pub noul: Option<f64>,
    pub score: Option<f64>,
    pub legend: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemOneResponse {
    pub model: String,
    pub answers: HashMap<String, SystemOneAnswer>,
    pub usage: Option<serde_json::Value>,
    pub cost: Option<String>,
}
```

---

## 5. Chính Sách Định Tuyến Hai Cấp (Two-Tier Supervision Policy)

Hệ thống kết hợp kết quả từ Kev với quy tắc nghiệp vụ tại `domain/supervision.rs` để phân luồng cho Lambda và Agents:

```text
                         [Task / Payload đến từ Worker]
                                       │
                                       ▼
                   ┌───────────────────────────────────────┐
                   │    Kev TypeSafe System One Engine     │
                   │   POST /v1/systemone (sub-50ms)       │
                   └───────────────────┬───────────────────┘
                                       │
             ┌─────────────────────────┴─────────────────────────┐
             ▼                                                   ▼
[RiskLane == Green & Confidence >= 0.85]          [RiskLane == Yellow/Red HOẶC Conf < 0.85]
             │                                                   │
             ▼                                                   ▼
┌───────────────────────────────────────┐       ┌───────────────────────────────────────┐
│           FAST-PATH ROUTE             │       │         SLOW-PATH ESCALATION          │
│   route: "automated_worker"           │       │   route: "human_supervisor_approval"  │
│   - Thực thi tức thì bởi Lambda       │       │   - Chặn đứng mọi thay đổi code phá vỡ│
│   - Không cần phê duyệt thủ công      │       │   - Yêu cầu Lead Dev / QC nghiệm thu  │
└───────────────────────────────────────┘       └───────────────────────────────────────┘
```

### Quy tắc phân loại rủi ro (Risk Lanes):
1. **`Green` (`tiny`)**:
   - Thao tác an toàn: Sửa lỗi chính tả, tài liệu markdown, format code, chạy test read-only.
   - Điều kiện tự động thông qua: `is_actionable == true` VÀ `confidence >= 0.85`.
2. **`Yellow` (`normal`)**:
   - Thêm tính năng mới, tạo migration DB, refactor component nội bộ.
   - Bắt buộc: Phải qua cổng nghiệm thu tự động của QC trước khi merge.
3. **`Red` (`high_risk`)**:
   - Sửa đổi cơ chế xác thực, phân quyền, xoá bảng DB, thay đổi API contract công khai.
   - Bắt buộc: Chặn đứng tức thì, kích hoạt chuông cảnh báo tới Human Supervisor.

---

## 6. Lệnh CLI Vận Hành Trong `harness`

Harness CLI cung cấp 2 lệnh tương tác trực tiếp với giao thức System One:

```bash
# 1. Đánh giá rủi ro và trả về quyết định Traffic-Light kèm điểm tin cậy
harness supervise evaluate --text "Sửa lỗi chính tả trong README.md" --json

# 2. Định tuyến trực tiếp cho Worker / Lambda
harness supervise route --text "Sắp hết hạn hợp đồng chưa? Cho tôi hủy gói Premium ngay" --json
```
