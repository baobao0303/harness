-- Clean up existing data to avoid constraint errors
DELETE FROM intake;
DELETE FROM story;
DELETE FROM decision;
DELETE FROM backlog;
DELETE FROM trace;
DELETE FROM intervention;

-- Seed Intakes
INSERT INTO intake (id, input_type, summary, risk_lane, story_id, notes) VALUES
(1, 'new_spec', 'Thiết kế giao diện dashboard cao cấp theo phong cách Paperclip', 'normal', 'US-001', 'Cần hỗ trợ hiển thị real-time các agent traces và ma trận test proof.'),
(2, 'spec_slice', 'Bổ sung thêm API endpoints cho traces, backlog và interventions', 'tiny', 'US-002', 'Để UI dashboard có thể fetch đầy đủ thông tin từ DB.'),
(3, 'change_request', 'Chuyển đổi giao diện sang SPA thay vì render gộp toàn bộ', 'high_risk', 'US-003', 'Có nguy cơ ảnh hưởng tới các scripts auto-test nếu cấu trúc DOM thay đổi nhiều.');

-- Seed Stories
INSERT INTO story (id, title, risk_lane, status, unit_proof, integration_proof, e2e_proof, platform_proof, evidence, priority) VALUES
('US-001', 'Cấu hình và khởi chạy web server local', 'tiny', 'implemented', 1, 1, 1, 0, 'Chạy test cargo test --package harness-web thành công.', 'P0'),
('US-002', 'Triển khai các API endpoints bổ sung', 'tiny', 'implemented', 1, 1, 0, 0, 'Các endpoint /api/traces, /api/intakes trả về JSON hợp lệ.', 'P1'),
('US-003', 'Xây dựng UI layout 2 cột phong cách Paperclip', 'normal', 'in_progress', 0, 0, 0, 0, 'Đang xây dựng style.css và index.html.', 'P0'),
('US-004', 'Tích hợp SPA logic và auto-polling vào app.js', 'normal', 'planned', 0, 0, 0, 0, 'Chưa bắt đầu.', 'P1');

-- Seed Decisions (ADRs)
INSERT OR REPLACE INTO decision (id, title, status, doc_path, verify_command, last_verified_at, last_verified_result) VALUES
('0001', 'Harness First Development', 'accepted', 'docs/decisions/0001-harness-first-development.md', NULL, NULL, NULL),
('0002', 'Post-Spec Product Lifecycle', 'accepted', 'docs/decisions/0002-post-spec-product-lifecycle.md', NULL, NULL, NULL),
('0003', 'Generic Spec-Intake in Harness', 'accepted', 'docs/decisions/0003-generic-spec-intake-harness.md', NULL, NULL, NULL),
('0004', 'SQLite Durable Layer', 'accepted', 'docs/decisions/0004-sqlite-durable-layer.md', 'cargo check', '2026-06-12 14:30:00', 'pass'),
('0005', 'Prebuilt Rust Harness CLI', 'accepted', 'docs/decisions/0005-prebuilt-rust-harness-cli.md', './scripts/harness --version', '2026-06-12 14:31:00', 'pass');

-- Seed Traces
INSERT INTO trace (id, task_summary, intake_id, story_id, agent, outcome, duration_seconds, token_estimate, notes) VALUES
(1, 'Đọc spec-intake.md và phân tích các trường dữ liệu cần thiết', 1, 'US-001', 'PM Agent', 'completed', 12, 1500, 'Đã hoàn thành phân tích các cột trong DB schema.'),
(2, 'Viết mã nguồn Rust cho struct và endpoint', 2, 'US-002', 'Backend Agent', 'completed', 45, 5200, 'Đã bổ sung struct TraceItem, IntakeItem, BacklogItem, InterventionItem và các API GET.'),
(3, 'Cài đặt router Axum và chạy kiểm tra cargo check', 2, 'US-002', 'Backend Agent', 'completed', 15, 2100, 'cargo check hoàn thành không có lỗi.'),
(4, 'Phác thảo cấu trúc file HTML mới cho dashboard', 3, 'US-003', 'Frontend Agent', 'partial', 30, 4000, 'Đang hoàn thiện phần sidebar và responsive styles.'),
(5, 'Viết các test scenarios cho màn hình dashboard mới', 3, 'US-003', 'QA Agent', 'completed', 25, 3000, 'Đã chuẩn bị sẵn các test script kiểm thử DOM.');

-- Seed Interventions
INSERT INTO intervention (trace_id, story_id, type, description, source, impact) VALUES
(2, 'US-002', 'override', 'Thiết lập mức độ ưu tiên của US-002 từ P2 lên P1 để mở chặn cho UI', 'human', 'Giúp đẩy nhanh tiến độ làm UI frontend'),
(4, 'US-003', 'correction', 'Sửa lại mã màu của các badge PASS/FAIL trong CSS để đồng bộ với Paperclip', 'reviewer', 'Màu sắc trực quan và rõ ràng hơn');

-- Seed Backlog
INSERT INTO backlog (title, discovered_while, current_pain, suggested_improvement, risk, status, priority) VALUES
('Hỗ trợ Tailwind v4 trực tiếp trong static files', 'Xây dựng CSS cho UI', 'Viết vanilla CSS tốn nhiều thời gian hơn so với các class tiện ích của Tailwind', 'Tích hợp Tailwind CDN hoặc thiết lập build step', 'tiny', 'proposed', 'P2'),
('Bổ sung WebSockets để truyền tải Agent Traces real-time', 'Auto-polling traces', 'Tải lại toàn bộ dữ liệu mỗi 3 giây tạo nhiều request thừa lên sqlite', 'Dùng server-sent events (SSE) hoặc websocket', 'normal', 'proposed', 'P1');
