use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead};
use std::str::FromStr;

use crate::application::{
    BacklogAddInput, BacklogCloseInput, DecisionAddInput, HarnessService, InitResult, IntakeInput,
    StoryAddInput, StoryUpdateInput, TraceInput,
};
use crate::domain::{BoolFlag, CsvList, InputType, RiskLane};

#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Option<Value>,
    id: Option<Value>,
}

#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Value>,
}

pub fn run_mcp_server(service: HarnessService) -> crate::infrastructure::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = match line {
            Ok(value) => value,
            Err(_) => break,
        };

        let request: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(value) => value,
            Err(error) => {
                let response = JsonRpcResponse {
                    jsonrpc: "2.0",
                    result: None,
                    error: Some(json!({
                        "code": -32700,
                        "message": format!("Parse error: {}", error)
                    })),
                    id: None,
                };
                println!("{}", serde_json::to_string(&response).unwrap());
                continue;
            }
        };

        if request.jsonrpc != "2.0" {
            let response = JsonRpcResponse {
                jsonrpc: "2.0",
                result: None,
                error: Some(json!({
                    "code": -32600,
                    "message": "Invalid Request: expected jsonrpc '2.0'"
                })),
                id: request.id,
            };
            println!("{}", serde_json::to_string(&response).unwrap());
            continue;
        }

        let id = request.id.clone();
        if id.is_none() {
            // It is a notification. Do NOT respond.
            let _ = handle_request(&service, request);
            continue;
        }

        match handle_request(&service, request) {
            Ok(Ok(result)) => {
                let response = JsonRpcResponse {
                    jsonrpc: "2.0",
                    result: Some(result),
                    error: None,
                    id,
                };
                println!("{}", serde_json::to_string(&response).unwrap());
            }
            Ok(Err((code, message))) => {
                let response = JsonRpcResponse {
                    jsonrpc: "2.0",
                    result: None,
                    error: Some(json!({
                        "code": code,
                        "message": message
                    })),
                    id,
                };
                println!("{}", serde_json::to_string(&response).unwrap());
            }
            Err(error) => {
                let response = JsonRpcResponse {
                    jsonrpc: "2.0",
                    result: None,
                    error: Some(json!({
                        "code": -32603,
                        "message": format!("Internal error: {}", error)
                    })),
                    id,
                };
                println!("{}", serde_json::to_string(&response).unwrap());
            }
        }
    }

    Ok(())
}

fn handle_request(
    service: &HarnessService,
    request: JsonRpcRequest,
) -> crate::infrastructure::Result<std::result::Result<Value, (i32, String)>> {
    match request.method.as_str() {
        "initialize" => Ok(Ok(json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "harness-mcp-server",
                "version": env!("CARGO_PKG_VERSION")
            }
        }))),
        "tools/list" => Ok(Ok(json!({
            "tools": get_tools_definition()
        }))),
        "tools/call" => {
            let params = request.params.unwrap_or(Value::Null);
            let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let arguments = params.get("arguments").cloned().unwrap_or(Value::Null);

            match call_tool(service, name, arguments) {
                Ok(val) => Ok(Ok(val)),
                Err(err) => Err(err),
            }
        }
        _ => Ok(Err((
            -32601,
            format!("Method not found: {}", request.method),
        ))),
    }
}

fn get_tools_definition() -> Value {
    json!([
        {
            "name": "harness_init",
            "description": "Khởi tạo cơ sở dữ liệu SQLite (harness.db) trong thư mục dự án hiện tại.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        },
        {
            "name": "harness_migrate",
            "description": "Chạy các tệp tin schema migrations chưa áp dụng trong scripts/schema.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        },
        {
            "name": "harness_import_brownfield",
            "description": "Nhập dữ liệu cũ từ docs/TEST_MATRIX.md, docs/decisions, và docs/HARNESS_BACKLOG.md vào cơ sở dữ liệu.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        },
        {
            "name": "harness_intake",
            "description": "Đăng ký phân loại rủi ro cho một yêu cầu hoặc tính năng mới.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "type": {
                        "type": "string",
                        "description": "Loại intake: new_spec, spec_slice, change_request, new_initiative, maintenance, harness_improvement"
                    },
                    "summary": {
                        "type": "string",
                        "description": "Mô tả ngắn gọn về yêu cầu tính năng"
                    },
                    "lane": {
                        "type": "string",
                        "description": "Làn an toàn: tiny, normal, high_risk"
                    },
                    "flags": {
                        "type": "string",
                        "description": "Cờ rủi ro triggered (dạng comma-separated, ví dụ: auth,data_model)"
                    },
                    "docs": {
                        "type": "string",
                        "description": "Tệp tài liệu sản phẩm bị ảnh hưởng (comma-separated)"
                    },
                    "story_id": {
                        "type": "string",
                        "description": "Liên kết tới Story ID nếu đã có"
                    },
                    "notes": {
                        "type": "string",
                        "description": "Ghi chú bổ sung"
                    }
                },
                "required": ["type", "summary", "lane"]
            }
        },
        {
            "name": "harness_story_add",
            "description": "Thêm một Story mới vào ma trận kiểm thử (Test Matrix) trong cơ sở dữ liệu.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "Story ID (ví dụ: US-001)"
                    },
                    "title": {
                        "type": "string",
                        "description": "Tiêu đề của câu chuyện"
                    },
                    "lane": {
                        "type": "string",
                        "description": "Làn an toàn của Story: tiny, normal, high_risk"
                    },
                    "contract": {
                        "type": "string",
                        "description": "Đường dẫn tới tệp tin đặc tả tài liệu sản phẩm liên kết"
                    },
                    "notes": {
                        "type": "string",
                        "description": "Ghi chú thiết kế hoặc kế hoạch kiểm chứng"
                    }
                },
                "required": ["id", "title", "lane"]
            }
        },
        {
            "name": "harness_story_update",
            "description": "Cập nhật tiến trình kiểm chứng (unit, integration, e2e) và trạng thái Story.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "Story ID cần cập nhật"
                    },
                    "status": {
                        "type": "string",
                        "description": "Trạng thái mới: planned, in_progress, implemented, changed, retired"
                    },
                    "evidence": {
                        "type": "string",
                        "description": "Minh chứng kết quả chạy kiểm thử thành công"
                    },
                    "unit": {
                        "type": "boolean",
                        "description": "1/true nếu đã vượt qua Unit test proof"
                    },
                    "integration": {
                        "type": "boolean",
                        "description": "1/true nếu đã vượt qua Integration test proof"
                    },
                    "e2e": {
                        "type": "boolean",
                        "description": "1/true nếu đã vượt qua E2E test proof"
                    },
                    "platform": {
                        "type": "boolean",
                        "description": "1/true nếu đã vượt qua Platform verification proof"
                    }
                },
                "required": ["id"]
            }
        },
        {
            "name": "harness_decision_add",
            "description": "Đăng ký một quyết định thiết kế kiến trúc (ADR) mới.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "Mã quyết định (ví dụ: 0001)"
                    },
                    "title": {
                        "type": "string",
                        "description": "Tiêu đề quyết định"
                    },
                    "status": {
                        "type": "string",
                        "description": "Trạng thái quyết định (mặc định: accepted)"
                    },
                    "doc": {
                        "type": "string",
                        "description": "Đường dẫn tài liệu markdown ADR"
                    },
                    "verify": {
                        "type": "string",
                        "description": "Lệnh bash tự động kiểm chứng quyết định"
                    },
                    "predicted": {
                        "type": "string",
                        "description": "Tác động dự báo tới hệ thống"
                    },
                    "notes": {
                        "type": "string",
                        "description": "Ghi chú kiến trúc"
                    }
                },
                "required": ["id", "title"]
            }
        },
        {
            "name": "harness_decision_verify",
            "description": "Chạy lệnh kiểm chứng tự động cho quyết định kiến trúc.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "Mã quyết định cần kiểm chứng"
                    }
                },
                "required": ["id"]
            }
        },
        {
            "name": "harness_backlog_add",
            "description": "Thêm một đề xuất cải tiến quy trình Harness khi phát hiện khó khăn (friction).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "title": {
                        "type": "string",
                        "description": "Tên đề xuất cải tiến ngắn gọn"
                    },
                    "while": {
                        "type": "string",
                        "description": "Phát hiện khi đang thực hiện tác vụ nào"
                    },
                    "pain": {
                        "type": "string",
                        "description": "Khó khăn hoặc thao tác thủ công bị lặp lại hiện tại"
                    },
                    "suggestion": {
                        "type": "string",
                        "description": "Ý tưởng cải tiến quy trình tối ưu hơn"
                    },
                    "risk": {
                        "type": "string",
                        "description": "Mức độ rủi ro đề xuất: tiny, normal, high_risk"
                    },
                    "predicted": {
                        "type": "string",
                        "description": "Tác động mong đợi sau khi cải tiến"
                    },
                    "notes": {
                        "type": "string",
                        "description": "Ghi chú backlog"
                    }
                },
                "required": ["title"]
            }
        },
        {
            "name": "harness_backlog_close",
            "description": "Đóng đề xuất backlog đã hoàn thành.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": {
                        "type": "integer",
                        "description": "ID số của backlog item cần đóng"
                    },
                    "status": {
                        "type": "string",
                        "description": "Trạng thái mới (mặc định: implemented)"
                    },
                    "outcome": {
                        "type": "string",
                        "description": "Kết quả hoặc minh chứng triển khai cải tiến thành công"
                    }
                },
                "required": ["id"]
            }
        },
        {
            "name": "harness_trace",
            "description": "Ghi nhận nhật ký dấu vết hoạt động thực thi tác vụ của AI Agent.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "summary": {
                        "type": "string",
                        "description": "Tóm tắt tác vụ đã thực hiện"
                    },
                    "intake": {
                        "type": "integer",
                        "description": "ID số của Intake liên kết"
                    },
                    "story": {
                        "type": "string",
                        "description": "Story ID liên kết"
                    },
                    "agent": {
                        "type": "string",
                        "description": "Tên Agent thực hiện (ví dụ: antigravity, claude-code)"
                    },
                    "outcome": {
                        "type": "string",
                        "description": "Kết quả: completed, blocked, partial, failed"
                    },
                    "duration": {
                        "type": "integer",
                        "description": "Thời gian thực thi tác vụ (giây)"
                    },
                    "tokens": {
                        "type": "integer",
                        "description": "Ước tính số lượng token tiêu thụ"
                    },
                    "friction": {
                        "type": "string",
                        "description": "Ghi nhận bất kỳ khó khăn hoặc lặp lại thao tác nào"
                    },
                    "actions": {
                        "type": "string",
                        "description": "Các hành động đã thực hiện (comma-separated, ví dụ: edit,run_test)"
                    },
                    "read": {
                        "type": "string",
                        "description": "Các tệp tin đã đọc (comma-separated)"
                    },
                    "changed": {
                        "type": "string",
                        "description": "Các tệp tin đã chỉnh sửa (comma-separated)"
                    },
                    "decisions": {
                        "type": "string",
                        "description": "Các quyết định kiến trúc đã đưa ra (comma-separated)"
                    },
                    "errors": {
                        "type": "string",
                        "description": "Các lỗi gặp phải khi chạy lệnh (comma-separated)"
                    },
                    "notes": {
                        "type": "string",
                        "description": "Ghi chú lưu vết"
                    }
                },
                "required": ["summary"]
            }
        },
        {
            "name": "harness_query_stats",
            "description": "Xem tổng số lượng dữ liệu thống kê của dự án Harness.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        },
        {
            "name": "harness_query_matrix",
            "description": "Xem ma trận kiểm chứng chất lượng và tiến độ toàn bộ Story.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        },
        {
            "name": "harness_query_decisions",
            "description": "Xem nhật ký toàn bộ quyết định kiến trúc.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        },
        {
            "name": "harness_query_intakes",
            "description": "Xem danh sách 20 phân loại Intake gần đây.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        },
        {
            "name": "harness_query_traces",
            "description": "Xem danh sách 20 nhật ký dấu vết hoạt động gần đây.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        },
        {
            "name": "harness_query_friction",
            "description": "Xem danh sách toàn bộ các dấu vết có ghi nhận khó khăn (friction).",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        },
        {
            "name": "harness_query_sql",
            "description": "Chạy truy vấn SQL trực tiếp để lọc dữ liệu tùy biến từ database.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Lệnh SQL đầy đủ (ví dụ: SELECT * FROM story WHERE status='in_progress')"
                    }
                },
                "required": ["query"]
            }
        }
    ])
}

fn call_tool(
    service: &HarnessService,
    name: &str,
    arguments: Value,
) -> crate::infrastructure::Result<Value> {
    match name {
        "harness_init" => {
            let result = service.init()?;
            let msg = match result {
                InitResult::Created { db_path } => {
                    format!(
                        "Creating harness database at {}\nSchema version 1 applied.",
                        db_path.display()
                    )
                }
                InitResult::Existing { db_path, version } => {
                    format!(
                        "Database already exists at {}\nCurrent schema version: {}",
                        db_path.display(),
                        version
                    )
                }
                InitResult::MigratedExisting { db_path } => {
                    format!("Database already exists at {}\nNo schema version found. Applying schema version 1.\nSchema version 1 applied.", db_path.display())
                }
            };
            Ok(json!({
                "content": [{"type": "text", "text": msg}]
            }))
        }
        "harness_migrate" => {
            let result = service.migrate()?;
            let mut msg = format!("Current schema version: {}\n", result.current_version);
            if result.applied.is_empty() {
                msg.push_str("Already up to date.");
            } else {
                for version in &result.applied {
                    msg.push_str(&format!("Applying migration {}...\n", version));
                }
                msg.push_str(&format!("Applied {} migration(s).", result.applied.len()));
            }
            Ok(json!({
                "content": [{"type": "text", "text": msg}]
            }))
        }
        "harness_import_brownfield" => {
            let result = service.import_brownfield()?;
            let msg = format!(
                "Brownfield import complete.\nStories imported or updated: {}\nDecisions imported or updated: {}\nBacklog items discovered: {}",
                result.stories, result.decisions, result.backlog_items
            );
            Ok(json!({
                "content": [{"type": "text", "text": msg}]
            }))
        }
        "harness_intake" => {
            let input_type = arguments.get("type").and_then(|v| v.as_str()).unwrap_or("");
            let summary = arguments
                .get("summary")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let lane = arguments.get("lane").and_then(|v| v.as_str()).unwrap_or("");
            let flags = arguments
                .get("flags")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let docs = arguments
                .get("docs")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let story_id = arguments
                .get("story_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let notes = arguments
                .get("notes")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let id = service.record_intake(IntakeInput {
                input_type: InputType::from_str(input_type).unwrap_or(InputType::ChangeRequest),
                summary: summary.to_string(),
                risk_lane: RiskLane::from_str(lane).unwrap_or(RiskLane::Tiny),
                risk_flags: CsvList::from_optional(flags),
                affected_docs: CsvList::from_optional(docs),
                story_id,
                notes,
            })?;

            Ok(json!({
                "content": [{"type": "text", "text": format!("Intake #{} recorded.", id)}]
            }))
        }
        "harness_story_add" => {
            let id = arguments.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let title = arguments
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let lane = arguments.get("lane").and_then(|v| v.as_str()).unwrap_or("");
            let contract = arguments
                .get("contract")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let notes = arguments
                .get("notes")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            service.add_story(StoryAddInput {
                id: id.to_string(),
                title: title.to_string(),
                risk_lane: RiskLane::from_str(lane).unwrap_or(RiskLane::Tiny),
                contract_doc: contract,
                notes,
            })?;

            Ok(json!({
                "content": [{"type": "text", "text": format!("Story {} added.", id)}]
            }))
        }
        "harness_story_update" => {
            let id = arguments.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let status = arguments
                .get("status")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let evidence = arguments
                .get("evidence")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let unit = arguments
                .get("unit")
                .and_then(|v| v.as_bool())
                .map(|b| BoolFlag(if b { 1 } else { 0 }));
            let integration = arguments
                .get("integration")
                .and_then(|v| v.as_bool())
                .map(|b| BoolFlag(if b { 1 } else { 0 }));
            let e2e = arguments
                .get("e2e")
                .and_then(|v| v.as_bool())
                .map(|b| BoolFlag(if b { 1 } else { 0 }));
            let platform = arguments
                .get("platform")
                .and_then(|v| v.as_bool())
                .map(|b| BoolFlag(if b { 1 } else { 0 }));

            service.update_story(StoryUpdateInput {
                id: id.to_string(),
                status,
                evidence,
                unit,
                integration,
                e2e,
                platform,
            })?;

            Ok(json!({
                "content": [{"type": "text", "text": format!("Story {} updated.", id)}]
            }))
        }
        "harness_decision_add" => {
            let id = arguments.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let title = arguments
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let status = arguments
                .get("status")
                .and_then(|v| v.as_str())
                .unwrap_or("accepted");
            let doc = arguments
                .get("doc")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let verify = arguments
                .get("verify")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let predicted = arguments
                .get("predicted")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let notes = arguments
                .get("notes")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            service.add_decision(DecisionAddInput {
                id: id.to_string(),
                title: title.to_string(),
                status: status.to_string(),
                doc_path: doc,
                verify_command: verify,
                predicted_impact: predicted,
                notes,
            })?;

            Ok(json!({
                "content": [{"type": "text", "text": format!("Decision {} added.", id)}]
            }))
        }
        "harness_decision_verify" => {
            let id = arguments.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let result = service.verify_decision(id)?;
            let msg = format!(
                "Running: {}\nDecision {} verification: {}",
                result.command, id, result.result
            );
            Ok(json!({
                "content": [{"type": "text", "text": msg}]
            }))
        }
        "harness_backlog_add" => {
            let title = arguments
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let discovered_while = arguments
                .get("while")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let current_pain = arguments
                .get("pain")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let suggestion = arguments
                .get("suggestion")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let risk = arguments
                .get("risk")
                .and_then(|v| v.as_str())
                .map(|s| RiskLane::from_str(s).unwrap_or(RiskLane::Tiny));
            let predicted_impact = arguments
                .get("predicted")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let notes = arguments
                .get("notes")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let id = service.add_backlog(BacklogAddInput {
                title: title.to_string(),
                discovered_while,
                current_pain,
                suggestion,
                risk,
                predicted_impact,
                notes,
            })?;

            Ok(json!({
                "content": [{"type": "text", "text": format!("Backlog #{} added.", id)}]
            }))
        }
        "harness_backlog_close" => {
            let id = arguments.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
            let status = arguments
                .get("status")
                .and_then(|v| v.as_str())
                .unwrap_or("implemented");
            let outcome = arguments
                .get("outcome")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            service.close_backlog(BacklogCloseInput {
                id,
                status: status.to_string(),
                actual_outcome: outcome,
            })?;

            Ok(json!({
                "content": [{"type": "text", "text": format!("Backlog #{} closed as {}.", id, status)}]
            }))
        }
        "harness_trace" => {
            let summary = arguments
                .get("summary")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let intake_id = arguments.get("intake").and_then(|v| v.as_i64());
            let story_id = arguments
                .get("story")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let agent = arguments
                .get("agent")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let outcome = arguments
                .get("outcome")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let duration = arguments.get("duration").and_then(|v| v.as_i64());
            let tokens = arguments.get("tokens").and_then(|v| v.as_i64());
            let friction = arguments
                .get("friction")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let actions = arguments
                .get("actions")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let read = arguments
                .get("read")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let changed = arguments
                .get("changed")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let decisions = arguments
                .get("decisions")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let errors = arguments
                .get("errors")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let notes = arguments
                .get("notes")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let id = service.record_trace(TraceInput {
                task_summary: summary.to_string(),
                intake_id,
                story_id,
                agent,
                outcome,
                duration_seconds: duration,
                token_estimate: tokens,
                friction,
                notes,
                actions: CsvList::from_optional(actions),
                files_read: CsvList::from_optional(read),
                files_changed: CsvList::from_optional(changed),
                decisions: CsvList::from_optional(decisions),
                errors: CsvList::from_optional(errors),
            })?;

            Ok(json!({
                "content": [{"type": "text", "text": format!("Trace #{} recorded.", id)}]
            }))
        }
        "harness_query_stats" => {
            let stats = service.query_stats()?;
            let msg = format!(
                "=== Harness Stats ===\nintakes: {}\nstories: {}\ndecisions: {}\nbacklog_items: {}\ntraces: {}",
                stats.intakes, stats.stories, stats.decisions, stats.backlog_items, stats.traces
            );
            Ok(json!({
                "content": [{"type": "text", "text": msg}]
            }))
        }
        "harness_query_matrix" => {
            let records = service.query_matrix()?;
            let mut msg = String::from("id\ttitle\tstatus\tunit\tinteg\te2e\tplat\tevidence\n");
            for r in records {
                msg.push_str(&format!(
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                    r.id,
                    r.title,
                    r.status,
                    r.unit,
                    r.integration,
                    r.e2e,
                    r.platform,
                    r.evidence.unwrap_or_default()
                ));
            }
            Ok(json!({
                "content": [{"type": "text", "text": msg}]
            }))
        }
        "harness_query_decisions" => {
            let records = service.query_decisions()?;
            let mut msg =
                String::from("id\ttitle\tstatus\tlast_verified_at\tlast_verified_result\n");
            for r in records {
                msg.push_str(&format!(
                    "{}\t{}\t{}\t{}\t{}\n",
                    r.id,
                    r.title,
                    r.status,
                    r.last_verified_at.unwrap_or_default(),
                    r.last_verified_result.unwrap_or_default()
                ));
            }
            Ok(json!({
                "content": [{"type": "text", "text": msg}]
            }))
        }
        "harness_query_intakes" => {
            let records = service.query_intakes()?;
            let mut msg = String::from("id\tcreated_at\tinput_type\trisk_lane\tsummary\n");
            for r in records {
                msg.push_str(&format!(
                    "{}\t{}\t{}\t{}\t{}\n",
                    r.id, r.created_at, r.input_type, r.risk_lane, r.summary
                ));
            }
            Ok(json!({
                "content": [{"type": "text", "text": msg}]
            }))
        }
        "harness_query_traces" => {
            let records = service.query_traces()?;
            let mut msg = String::from("id\tcreated_at\toutcome\ttask_summary\tharness_friction\n");
            for r in records {
                msg.push_str(&format!(
                    "{}\t{}\t{}\t{}\t{}\n",
                    r.id,
                    r.created_at,
                    r.outcome.unwrap_or_default(),
                    r.task_summary,
                    r.harness_friction.unwrap_or_default()
                ));
            }
            Ok(json!({
                "content": [{"type": "text", "text": msg}]
            }))
        }
        "harness_query_friction" => {
            let records = service.query_friction()?;
            let mut msg = String::from("id\tcreated_at\ttask_summary\tharness_friction\n");
            for r in records {
                msg.push_str(&format!(
                    "{}\t{}\t{}\t{}\n",
                    r.id, r.created_at, r.task_summary, r.harness_friction
                ));
            }
            Ok(json!({
                "content": [{"type": "text", "text": msg}]
            }))
        }
        "harness_query_sql" => {
            let query = arguments
                .get("query")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let table = service.query_sql(query)?;

            let mut msg = table.headers.join("\t") + "\n";
            for row in table.rows {
                msg.push_str(&(row.join("\t") + "\n"));
            }
            Ok(json!({
                "content": [{"type": "text", "text": msg}]
            }))
        }
        _ => Ok(json!({
            "content": [{"type": "text", "text": format!("Unknown tool name: {}", name)}]
        })),
    }
}
