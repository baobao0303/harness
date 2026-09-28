# KEV: Small Decision Models Architecture & Codebase Technical Documentation

> **Tài liệu phân tích kỹ thuật toàn diện về dự án [jaredpalmer/kev](https://github.com/jaredpalmer/kev)**  
> **Tác giả nguồn:** Jared Palmer  
> **Mục tiêu:** Mô hình quyết định nhỏ gọn (Decision Model) theo kiến trúc của Jev (TypeSafe System One), chạy suy luận prefill-only không sinh từ tự do (non-autoregressive), cho xác suất hiệu chuẩn cao (calibrated probabilities), hỗ trợ đa dạng bài toán phân loại và đánh giá.

---

## 1. 📌 Tổng Quan Về Kev

**Kev** là một họ mô hình quyết định (decision models) gọn nhẹ được xây dựng trên nền tảng **Qwen3.5** và **Qwen3.8**, hiện thực hóa kiến trúc được phân tích trong bài nghiên cứu *"Jev's Architecture Unmasked"*. 

Khác với các mô hình ngôn ngữ lớn sinh văn bản truyền thống (Autoregressive Generative LLMs) phải sinh tuần tự từng token qua giải mã, Kev xử lý bài toán ra quyết định chỉ trong **một lượt prefill (Forward pass duy nhất)**.

### 🌟 Điểm nổi bật chính
1. **Một lượt xử lý đa câu hỏi (Multi-question in single request):** Hỗ trợ cùng lúc các câu hỏi dạng Có/Không (`noul`), trắc nghiệm chọn 1 (`choice`) và thang điểm (`score`). Các câu hỏi dùng chung ngữ cảnh văn bản đầu vào (`state`) nhưng được cô lập với nhau.
2. **Xzác suất được hiệu chuẩn (Calibrated Probabilities):** Mỗi checkpoint được tối ưu tham số nhiệt độ (temperature scaling) trên tập dữ liệu kiểm thử độc lập, đảm bảo độ tin cậy của xác suất đầu ra (phục vụ việc tự động hóa tác vụ hoặc chuyển cho con người xử lý khi độ tự tin thấp).
3. **Tương thích hoàn toàn với TypeSafe System One API:** Có thể trỏ trực tiếp TypeSafe Python SDK vào server Kev nội bộ mà không cần sửa code.
4. **Prefill-Only & Tiết kiệm tài nguyên:** Không tốn chi phí decoding autoregressive; tận dụng Prefix Caching để tái sử dụng trạng thái KV của `state` cho hàng loạt câu hỏi.
5. **Đa dạng kích thước:** 4 phiên bản từ **0.8B** (chạy trên laptop / Apple Silicon), **4B**, **9B** đến **27B** (chạy trên 1 GPU 80GB).

---

## 2. 🏗️ Họ Mô Hình & Benchmark

| Mô hình | Base Model | Độ chính xác (Nguồn mới - Dev / Test) | Brier Score (Nguồn mới) | Phần cứng đề xuất |
| :--- | :--- | :--- | :--- | :--- |
| **Kev-0.8B** | `Qwen/Qwen3.5-0.8B-Base` | 0.648 / 0.697 | 0.481 / 0.416 | Apple Silicon Mac, GPU L4 (4GB VRAM) |
| **Kev-4B** | `Qwen/Qwen3.5-4B-Base` | 0.817 / 0.838 | 0.269 / 0.242 | 32GB Mac, GPU L40S, H100 |
| **Kev-9B** | `Qwen/Qwen3.5-9B-Base` | 0.822 / 0.852 | 0.286 / 0.237 | 32GB Mac, GPU L40S, H100 |
| **Kev-27B** | `Qwen/Qwen3.8-27B` (Post-trained) | **0.848 / 0.896** | **0.236 / 0.164** | B200, H200, H100 80GB |
| *Jev (Tham chiếu)* | *Hosted TypeSafe* | 0.857 / – | 0.211 / – | TypeSafe Cloud API |

* **Kev-0.8B, 4B, 9B:** Được huấn luyện từ mô hình nền tảng gốc (Base) thông qua LoRA adapter và Pointer Head.
* **Kev-27B:** Được huấn luyện từ phiên bản post-trained của Qwen3.8.

---

## 3. 🔬 Kiến Trúc Kỹ Thuật Chi Tiết (Deep Architecture)

```
                              [Input Request]
                                     |
               +---------------------+---------------------+
               |                     |                     |
         (State / Context)      (Question 1)          (Question 2)
               |                     |                     |
               +---------------------+---------------------+
                                     |
                        [Token Packaging & Encoding]
                [<state> tokens] [<q1> <opt>...</opt> <decide>] ...
                                     |
                         [Block-Causal Attention]
                    (Q1 & Q2 see State, but NOT each other)
                                     |
                          [Backbone LLM + LoRA]
                     (Hidden states at <decide> & </opt>)
                                     |
                             [PointerHead readout]
                   Score_i = (Q * h_decide) • (K * h_opt_i) / (sqrt(d_p) * T)
                                     |
                                 [Softmax]
                                     |
                       [Calibrated Probabilities]
```

### 3.1. Token Encoding & Special Delimiters (`kev/model.py`)
Kev tái sử dụng các token đặc biệt hiếm dùng của bộ từ vựng Qwen làm mốc phân tách cú pháp mà không cần thêm token mới vào embedding table:
* `<|fim_prefix|>`: Đánh dấu bắt đầu `state` (ngữ cảnh bài toán).
* `<|fim_middle|>`: Đánh dấu bắt đầu câu hỏi (`<q>`).
* `<|box_start|>`: Đánh dấu bắt đầu một lựa chọn (`<opt>`).
* `<|box_end|>`: Đánh dấu kết thúc một lựa chọn (`</opt>`).
* `<|fim_suffix|>`: Đánh dấu token ra quyết định (`<decide>`).

Hàm `user_tokens()` tự động escape các chuỗi đầu vào của người dùng dạng `<|name|>` thành `<¦name¦>` để tránh giả mạo token điều khiển.

### 3.2. Block-Causal Masking & Question Isolation
* **Chia sẻ State:** Tất cả các nhánh câu hỏi đều nhìn thấy toàn bộ token của `state` ($seg = 0$).
* **Cô lập giữa các câu hỏi:** Nhánh câu hỏi $k$ ($seg = k$) chỉ được chú ý (attend) đến `state` và chính nó, hoàn toàn không nhìn thấy token của nhánh câu hỏi khác.
* **Cơ chế Masking:**
  $$\text{Attend}(i, j) = \text{True} \iff j \le i \land (seg[j] = 0 \lor seg[j] = seg[i])$$

### 3.3. Option Isolation & Tính Bất Biến Thứ Tự (Permutation Invariance)
Trong chế độ `option_isolation = True`:
* Mỗi lựa chọn `<opt>...</opt>` là một nhánh con riêng biệt: chỉ nhìn thấy `state`, câu hỏi (`instruction`) và chính nó.
* Tất cả các option trong cùng câu hỏi chia sẻ chung dải `position_ids`.
* Token `<decide>` nằm ở vị trí cố định sau option dài nhất và attend đến tất cả các option.
* Nhờ vậy, biểu diễn vector của từng option và sự chú ý của `<decide>` lên chúng là **bất biến hoàn toàn với thứ tự các lựa chọn** (không bị thiên lệch vị trí).

### 3.4. Readout Head: `PointerHead` (`kev/model.py`)
Thay vì dùng Language Modeling Head dự đoán từ tiếp theo, Kev sử dụng một **Pointer Head** tích vô hướng (Dot-Product Attention):
* Chiếu trạng thái ẩn $h_{\text{decide}}$ qua ma trận $W_Q \in \mathbb{R}^{d \times d_p}$ tạo vector truy vấn $q$.
* Chiếu các trạng thái ẩn $h_{\text{opt}_i}$ tại vị trí đóng token `</opt>` qua ma trận $W_K \in \mathbb{R}^{d \times d_p}$ tạo các vector khóa $k_i$.
* Điểm số thô (logits) cho lựa chọn thứ $i$:
  $$z_i = \frac{q^T k_i}{\sqrt{d_p} \cdot T}$$
  Trong đó $d_p = 256$ (Pointer Dimension) và $T$ là hệ số nhiệt độ (Temperature) được hiệu chuẩn sẵn.
* Xác suất của từng option được tính qua hàm Softmax trên $z_i$.

---

## 4. 📊 Các Dạng Câu Hỏi Được Hỗ Trợ (`kev/api.py`)

Kev ánh xạ tất cả các dạng bài toán ra quyết định về cơ chế Pointer duy nhất:

### 1. `noul` (Câu hỏi Yes/No - Nhị phân)
* Bản chất là 2 options: `[False, True]`.
* Output trả về xác suất xảy ra: $P(\text{True}) \in [0.0, 1.0]$.
```json
"escalate": { "type": "noul", "instructions": "Does this need urgent human attention?" }
```

### 2. `choice` (Trắc nghiệm chọn một danh mục)
* Danh sách các nhãn kèm mô tả tiêu chí:
```json
"department": {
  "type": "choice",
  "instructions": "Which team should handle this?",
  "criteria": {
    "returns": "Exchanges, refunds, wrong or damaged items",
    "shipping": "Delivery status, delays, lost packages",
    "billing": "Charges, invoices, payment problems"
  }
}
```
* Output trả về: `choice` (nhãn có xác suất cao nhất), `confidence` ($P_{\max} - P_{\text{second}}$), và phân phối xác suất `probabilities` của toàn bộ các nhãn.

### 3. `score` (Thang đo / Điểm số định lượng có thứ tự)
* Danh sách các mức độ được sắp xếp tăng dần: `["Calm", "Frustrated", "Very angry"]`.
* Output trả về:
  * Điểm số kỳ vọng: $\text{score} = \sum_{i=0}^{K-1} i \cdot P(i)$
  * Phân phối xác suất của từng mức điểm $0, 1, \dots, K-1$.

---

## 5. 🛠️ Chi Tiết Các Module Mã Nguồn (Codebase Structure)

Dưới đây là chi tiết các tệp nguồn trong thư mục `kev/` của dự án:

| File / Module | Chức năng chi tiết |
| :--- | :--- |
| **`kev/model.py`** | Định nghĩa kiến trúc `DecisionModel`, `PointerHead`, thuật toán mã hóa `encode()`, ma trận `branch_mask_batch()`, logic cắt hàng `rows_of()` và cấu hình kích thước ngữ cảnh (SERVE_MAX_STATE = 64k). |
| **`kev/api.py`** | Định nghĩa Schema Pydantic cho Request/Response theo chuẩn TypeSafe System One (`/v1/systemone`), chuyển đổi Request thành Internal Record và ngược lại. |
| **`kev/serve.py`** | Máy chủ HTTP FastAPI serving mô hình, hỗ trợ Prefix Caching, xử lý song song batch inference, phân luồng Worker GPU. |
| **`kev/train.py`** | Script huấn luyện: Hỗ trợ Fine-tuning LoRA adapter hoặc Full Fine-tuning (`full_ft.py`), tích hợp FSDP2 khi chạy đa GPU bằng PyTorch Distributed. |
| **`kev/checkpoint.py`**| Quản lý tải / lưu Checkpoint, metadata `training_config.json`, tự động chuyển đổi adapter và trọng số Pointer Head. |
| **`kev/calibrate.py`** | Phân tích và hiệu chuẩn hệ số nhiệt độ (Temperature Fitting) trên tập kiểm thử nhằm cực tiểu hóa sai số xác suất (ECE & Brier Score). |
| **`kev/metrics.py`** | Bộ công cụ tính toán các chỉ số thống kê xác suất hoàn toàn bằng NumPy: ECE, Brier score, NLL, Risk-Coverage Curve, AURC, Paired Bootstrap test. |
| **`kev/benchmark.py`** | Chạy đánh giá toàn diện mô hình trên các tập dữ liệu chuẩn (Frozen Evaluation Suites) hoặc endpoint từ xa. |
| **`kev/mlx_model.py`** | Backend tối ưu riêng cho chip **Apple Silicon (Metal)** sử dụng thư viện `mlx-lm`, tăng tốc đáng kể so với PyTorch MPS khi chạy trên Mac. |
| **`kev/cuda_graphs.py`**| Tăng tốc độ trễ suy luận trên GPU NVIDIA thông qua kỹ thuật CUDA Graphs replay (loại bỏ CPU overhead). |
| **`modal_app.py`** | Ứng dụng Serverless triển khai trên nền tảng Cloud [Modal](https://modal.com), hỗ trợ Auto-scale về 0 khi không có tải. |
| **`skills/`** | Chứa các công cụ tự động hóa (`kev-finetune`, `kev-deploy`) tích hợp cho Coding Agents (Claude Code, Cursor, Antigravity) để thực hiện quy trình fine-tune và deploy tự động. |

---

## 6. 🚀 Hướng Dẫn Sử Dụng & Vận Hành

### 6.1. Cài đặt môi trường
Yêu cầu Python 3.12 hoặc 3.13 cùng công cụ quản lý gói `uv`:
```bash
git clone https://github.com/jaredpalmer/kev.git
cd kev
uv sync --extra serve
```

### 6.2. Chạy máy chủ Local
Khởi chạy Kev-4B trên cổng 8009 (tự động nhận diện Apple Silicon / CUDA):
```bash
uv run --extra serve python -m kev.serve --run jaredpalmer/kev-4b --port 8009
```

### 6.3. Gửi Request kiểm thử qua cURL
```bash
curl -s localhost:8009/v1/systemone -H 'content-type: application/json' -d '{
  "state": "Giày giao trễ 2 tuần và bị sai kích cỡ. Thẻ tín dụng của tôi còn bị trừ tiền 2 lần.",
  "model": "kev-latest",
  "questions": {
    "team": {
      "type": "choice",
      "instructions": "Bộ phận nào cần tiếp nhận xử lý?",
      "criteria": {
        "returns": "Đổi trả hàng, hoàn tiền, sai hàng hoặc hàng hỏng",
        "shipping": "Trạng thái giao hàng, chậm trễ, thất lạc bưu kiện",
        "billing": "Khiếu nại trừ tiền, hóa đơn, thanh toán"
      }
    },
    "urgent": {
      "type": "noul",
      "instructions": "Vấn đề này có cần người can thiệp gấp không?"
    },
    "frustration": {
      "type": "score",
      "instructions": "Mức độ bức xúc của khách hàng?",
      "criteria": ["Bình tĩnh", "Khó chịu", "Rất tức giận"]
    }
  }
}'
```

### 6.4. Tinh chỉnh trên tập dữ liệu riêng (Fine-tuning)
Chuẩn bị file `train.jsonl` (định dạng giống API request có thêm trường `"label"`), sau đó chạy lệnh:
```bash
uv run python -m kev.train \
  --data train.jsonl \
  --base Qwen/Qwen3.5-4B-Base \
  --init_from jaredpalmer/kev-4b \
  --epochs 2 \
  --lr 2e-5 \
  --batch 1 \
  --accum 8 \
  --dtype bf16 \
  --checkpointing 1 \
  --device cuda \
  --out runs/custom-model
```

---

## 7. 🎯 Kết Luận & Đánh Giá

* **Kev** là một giải pháp đột phá trong phân khúc mô hình ra quyết định và phân loại văn bản: thay vì dùng prompt generation cồng kềnh với LLM lớn, Kev mang lại **tốc độ cực nhanh (prefill-only), độ trễ thấp, xác suất được hiệu chuẩn chuẩn xác**, và chi phí vận hành rẻ hơn hàng chục lần.
* Kiến trúc **Pointer Head + Block-Causal Masking + Option Isolation** loại bỏ hoàn toàn các lỗi thiên lệch vị trí thường gặp ở LLM thông thường.
* Việc hỗ trợ đa nền tảng (Apple Silicon qua MLX, NVIDIA GPU qua CUDA Graphs/FSDP, Cloud qua Modal) giúp Kev dễ dàng tích hợp vào cả môi trường thử nghiệm cục bộ lẫn hệ thống production quy mô lớn.
