use axum::{
    extract::{State, Path, ws::{Message as WsMessage, WebSocket, WebSocketUpgrade}},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, delete},
    Json, Router,
};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tokio::process::Command;
use tokio::time::{sleep, Duration};
use std::fs;

struct AppState {
    db_path: PathBuf,
}

#[derive(Serialize)]
struct Stats {
    intakes: i64,
    stories: i64,
    decisions: i64,
    backlog: i64,
    traces: i64,
}

#[derive(Serialize)]
struct AgentStatus {
    agent: String,
    status: String,  // idle, working, assigned
    current_task: Option<String>,
    last_update: Option<String>,
}

#[derive(Serialize)]
struct AgentStatusResponse {
    agents: Vec<AgentStatus>,
    timestamp: String,
}

#[derive(Serialize)]
struct StoryItem {
    id: String,
    title: String,
    status: String,
    unit: i64,
    integration: i64,
    e2e: i64,
    platform: i64,
    evidence: Option<String>,
    priority: String,
    risk_lane: String,
}

#[derive(Serialize)]
struct DecisionItem {
    id: String,
    title: String,
    created_at: String,
    status: String,
    doc_path: Option<String>,
    verify_command: Option<String>,
    last_verified_at: Option<String>,
    last_verified_result: Option<String>,
}

#[derive(Serialize)]
struct TraceItem {
    id: i64,
    created_at: String,
    task_summary: String,
    intake_id: Option<i64>,
    story_id: Option<String>,
    agent: Option<String>,
    outcome: Option<String>,
    duration_seconds: Option<i64>,
    token_estimate: Option<i64>,
    harness_friction: Option<String>,
    notes: Option<String>,
    decisions_made: Option<String>,
}

#[derive(Serialize)]
struct IntakeItem {
    id: i64,
    created_at: String,
    input_type: String,
    summary: String,
    risk_lane: String,
    notes: Option<String>,
    story_id: Option<String>,
}

#[derive(Serialize)]
struct BacklogItem {
    id: i64,
    created_at: String,
    title: String,
    discovered_while: Option<String>,
    current_pain: Option<String>,
    suggested_improvement: Option<String>,
    risk: Option<String>,
    status: String,
    notes: Option<String>,
    priority: String,
}

#[derive(Serialize)]
struct InterventionItem {
    id: i64,
    created_at: String,
    trace_id: Option<i64>,
    story_id: Option<String>,
    r#type: String,
    description: String,
    source: String,
    impact: Option<String>,
}

#[derive(Serialize)]
struct ToolItem {
    name: String,
    created_at: String,
    provider: String,
    command: String,
    description: String,
    args: Option<String>,
    responsibility: String,
    since: String,
}

#[derive(Deserialize)]
struct IntakeRequest {
    input_type: String,
    summary: String,
    risk_lane: String,
    notes: Option<String>,
    story_id: Option<String>,
}

#[derive(Deserialize)]
struct StoryRequest {
    id: String,
    title: String,
    lane: String,
    priority: String,
}

#[derive(Deserialize)]
struct TraceRequest {
    task_summary: String,
    story_id: Option<String>,
    agent: Option<String>,
    outcome: String,
    decisions_made: Option<String>,
}

#[derive(Deserialize)]
struct BacklogRequest {
    title: String,
    discovered_while: Option<String>,
    current_pain: Option<String>,
    suggested_improvement: Option<String>,
    risk: Option<String>,
    notes: Option<String>,
    priority: String,
}

#[derive(Deserialize)]
struct DecisionRequest {
    id: String,
    title: String,
    status: String,
    doc_path: Option<String>,
    verify_command: Option<String>,
}

#[derive(Deserialize)]
struct StoryUpdateRequest {
    title: String,
    status: String,
    priority: String,
    unit: i64,
    integration: i64,
    e2e: i64,
    evidence: Option<String>,
    risk_lane: String,
}

#[derive(Deserialize)]
struct IntakeUpdateRequest {
    input_type: String,
    summary: String,
    risk_lane: String,
    notes: Option<String>,
    story_id: Option<String>,
}

#[derive(Deserialize)]
struct DecisionUpdateRequest {
    title: String,
    status: String,
    doc_path: Option<String>,
    verify_command: Option<String>,
    last_verified_result: Option<String>,
}

#[derive(Deserialize)]
struct TraceUpdateRequest {
    task_summary: String,
    outcome: String,
    story_id: Option<String>,
    agent: Option<String>,
    notes: Option<String>,
    decisions_made: Option<String>,
}

#[derive(Deserialize)]
struct BacklogUpdateRequest {
    title: String,
    discovered_while: Option<String>,
    current_pain: Option<String>,
    suggested_improvement: Option<String>,
    risk: Option<String>,
    priority: String,
    status: String,
    notes: Option<String>,
}

#[derive(Deserialize)]
struct QueryRequest {
    query: String,
}

#[derive(Serialize)]
struct QueryResponse {
    columns: Vec<String>,
    rows: Vec<Vec<serde_json::Value>>,
}

#[tokio::main]
async fn main() {
    let repo_root = std::env::var("HARNESS_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        });
    let db_path = std::env::var("HARNESS_DB")
        .map(PathBuf::from)
        .unwrap_or_else(|_| repo_root.join("harness.db"));

    println!("Starting Harness Web Server...");
    println!("Database path: {}", db_path.display());

    let state = Arc::new(AppState { db_path });

    // Serve static files from the UI directory
    let ui_dir = std::env::current_dir().unwrap().join("crates/harness-web/ui");
    println!("Serving UI static files from: {}", ui_dir.display());

    let personas_dir = repo_root.join(".agents/personas");
    println!("Serving Agent Personas from: {}", personas_dir.display());

    let app = Router::new()
        .route("/api/stats", get(get_stats))
        .route("/api/matrix", get(get_matrix))
        .route("/api/decisions", get(get_decisions))
        .route("/api/traces", get(get_traces))
        .route("/api/intakes", get(get_intakes))
        .route("/api/backlog", get(get_backlog))
        .route("/api/interventions", get(get_interventions))
        .route("/api/tools", get(get_tools))
        .route("/api/intake", post(create_intake))
        .route("/api/story", post(create_story))
        .route("/api/trace", post(create_trace))
        .route("/api/backlog", post(create_backlog))
        .route("/api/decision", post(create_decision))
        .route("/api/intake/:id", delete(delete_intake).put(update_intake))
        .route("/api/story/:id", delete(delete_story).put(update_story))
        .route("/api/trace/:id", delete(delete_trace).put(update_trace))
        .route("/api/backlog/:id", delete(delete_backlog).put(update_backlog))
        .route("/api/decision/:id", delete(delete_decision).put(update_decision))
        .route("/api/decision/:id/verify", post(verify_decision))
        .route("/api/ws", get(ws_handler))
        .route("/api/intervention/:id/resolve", post(resolve_intervention))
        .route("/api/query", post(execute_sql_query))
        .route("/api/agent-status", get(get_agent_status))
        .route("/api/discussion", get(get_discussion))
        .nest_service("/personas", ServeDir::new(personas_dir))
        .fallback_service(ServeDir::new(ui_dir))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Harness Frontend Dashboard is active at: http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn get_stats(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let intakes: i64 = conn.query_row("SELECT COUNT(*) FROM intake", [], |r| r.get(0)).unwrap_or(0);
    let stories: i64 = conn.query_row("SELECT COUNT(*) FROM story", [], |r| r.get(0)).unwrap_or(0);
    let decisions: i64 = conn.query_row("SELECT COUNT(*) FROM decision", [], |r| r.get(0)).unwrap_or(0);
    let backlog: i64 = conn.query_row("SELECT COUNT(*) FROM backlog", [], |r| r.get(0)).unwrap_or(0);
    let traces: i64 = conn.query_row("SELECT COUNT(*) FROM trace", [], |r| r.get(0)).unwrap_or(0);

    Json(Stats {
        intakes,
        stories,
        decisions,
        backlog,
        traces,
    })
    .into_response()
}

async fn get_discussion() -> impl IntoResponse {
    let messages_path = PathBuf::from("/d/harness/.agents/comms/messages/messages.json");
    let messages = if messages_path.exists() {
        match tokio::fs::read_to_string(&messages_path).await {
            Ok(content) => match serde_json::from_str::<Vec<serde_json::Value>>(&content) {
                Ok(msgs) => msgs,
                Err(_) => vec![],
            },
            Err(_) => vec![],
        }
    } else {
        vec![]
    };
    Json(json!({ "messages": messages })).into_response()
}

async fn get_agent_status() -> impl IntoResponse {
    let state_path = PathBuf::from("/d/harness/.agents/comms/state/agent_status.json");
    let mut agents = Vec::new();
    let default_agents = ["pm", "ba", "fe", "be", "qa", "aud"];

    if state_path.exists() {
        if let Ok(content) = fs::read_to_string(&state_path) {
            if let Ok(state) = serde_json::from_str::<serde_json::Value>(&content) {
                for agent_name in default_agents {
                    let agent_data = state.get(agent_name).cloned().unwrap_or(serde_json::json!({}));
                    agents.push(AgentStatus {
                        agent: agent_name.to_uppercase(),
                        status: agent_data.get("status").and_then(|v| v.as_str()).unwrap_or("idle").to_string(),
                        current_task: agent_data.get("current_task").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        last_update: agent_data.get("last_update").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    });
                }
            }
        }
    }

    // Fallback if file doesn't exist
    if agents.is_empty() {
        for agent_name in default_agents {
            agents.push(AgentStatus {
                agent: agent_name.to_uppercase(),
                status: "idle".to_string(),
                current_task: None,
                last_update: None,
            });
        }
    }

    Json(AgentStatusResponse {
        agents,
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
    .into_response()
}

async fn get_matrix(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let mut stmt = match conn.prepare(
        "SELECT id, title, status, unit_proof, integration_proof, e2e_proof, platform_proof, evidence, priority, risk_lane FROM story"
    ) {
        Ok(s) => s,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let rows = stmt.query_map([], |row| {
        Ok(StoryItem {
            id: row.get(0)?,
            title: row.get(1)?,
            status: row.get(2)?,
            unit: row.get(3)?,
            integration: row.get(4)?,
            e2e: row.get(5)?,
            platform: row.get(6)?,
            evidence: row.get(7)?,
            priority: row.get(8)?,
            risk_lane: row.get(9)?,
        })
    });

    let items: Vec<StoryItem> = match rows {
        Ok(r) => r.filter_map(|x| x.ok()).collect(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    Json(items).into_response()
}

async fn get_decisions(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let mut stmt = match conn.prepare(
        "SELECT id, title, created_at, status, doc_path, verify_command, last_verified_at, last_verified_result FROM decision"
    ) {
        Ok(s) => s,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let rows = stmt.query_map([], |row| {
        Ok(DecisionItem {
            id: row.get(0)?,
            title: row.get(1)?,
            created_at: row.get(2)?,
            status: row.get(3)?,
            doc_path: row.get(4)?,
            verify_command: row.get(5)?,
            last_verified_at: row.get(6)?,
            last_verified_result: row.get(7)?,
        })
    });

    let items: Vec<DecisionItem> = match rows {
        Ok(r) => r.filter_map(|x| x.ok()).collect(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    Json(items).into_response()
}

async fn create_intake(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<IntakeRequest>,
) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let res = conn.execute(
        "INSERT INTO intake (input_type, summary, risk_lane, notes, story_id) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![payload.input_type, payload.summary, payload.risk_lane, payload.notes, payload.story_id],
    );

    match res {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn create_story(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<StoryRequest>,
) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let res = conn.execute(
        "INSERT INTO story (id, title, risk_lane, priority, status) VALUES (?1, ?2, ?3, ?4, 'planned')",
        params![payload.id, payload.title, payload.lane, payload.priority],
    );

    match res {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn create_trace(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TraceRequest>,
) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let res = conn.execute(
        "INSERT INTO trace (task_summary, story_id, agent, outcome, decisions_made) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![payload.task_summary, payload.story_id, payload.agent, payload.outcome, payload.decisions_made],
    );

    match res {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn get_traces(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let mut stmt = match conn.prepare(
        "SELECT id, created_at, task_summary, intake_id, story_id, agent, outcome, duration_seconds, token_estimate, harness_friction, notes, decisions_made FROM trace ORDER BY id DESC LIMIT 50"
    ) {
        Ok(s) => s,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let rows = stmt.query_map([], |row| {
        Ok(TraceItem {
            id: row.get(0)?,
            created_at: row.get(1)?,
            task_summary: row.get(2)?,
            intake_id: row.get(3)?,
            story_id: row.get(4)?,
            agent: row.get(5)?,
            outcome: row.get(6)?,
            duration_seconds: row.get(7)?,
            token_estimate: row.get(8)?,
            harness_friction: row.get(9)?,
            notes: row.get(10)?,
            decisions_made: row.get(11)?,
        })
    });

    let items: Vec<TraceItem> = match rows {
        Ok(r) => r.filter_map(|x| x.ok()).collect(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    Json(items).into_response()
}

async fn get_intakes(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let mut stmt = match conn.prepare(
        "SELECT id, created_at, input_type, summary, risk_lane, notes, story_id FROM intake ORDER BY id DESC"
    ) {
        Ok(s) => s,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let rows = stmt.query_map([], |row| {
        Ok(IntakeItem {
            id: row.get(0)?,
            created_at: row.get(1)?,
            input_type: row.get(2)?,
            summary: row.get(3)?,
            risk_lane: row.get(4)?,
            notes: row.get(5)?,
            story_id: row.get(6)?,
        })
    });

    let items: Vec<IntakeItem> = match rows {
        Ok(r) => r.filter_map(|x| x.ok()).collect(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    Json(items).into_response()
}

async fn get_backlog(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let mut stmt = match conn.prepare(
        "SELECT id, created_at, title, discovered_while, current_pain, suggested_improvement, risk, status, notes, priority FROM backlog ORDER BY id DESC"
    ) {
        Ok(s) => s,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let rows = stmt.query_map([], |row| {
        Ok(BacklogItem {
            id: row.get(0)?,
            created_at: row.get(1)?,
            title: row.get(2)?,
            discovered_while: row.get(3)?,
            current_pain: row.get(4)?,
            suggested_improvement: row.get(5)?,
            risk: row.get(6)?,
            status: row.get(7)?,
            notes: row.get(8)?,
            priority: row.get(9)?,
        })
    });

    let items: Vec<BacklogItem> = match rows {
        Ok(r) => r.filter_map(|x| x.ok()).collect(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    Json(items).into_response()
}

async fn get_interventions(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let mut stmt = match conn.prepare(
        "SELECT id, created_at, trace_id, story_id, type, description, source, impact FROM intervention ORDER BY id DESC"
    ) {
        Ok(s) => s,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let rows = stmt.query_map([], |row| {
        Ok(InterventionItem {
            id: row.get(0)?,
            created_at: row.get(1)?,
            trace_id: row.get(2)?,
            story_id: row.get(3)?,
            r#type: row.get(4)?,
            description: row.get(5)?,
            source: row.get(6)?,
            impact: row.get(7)?,
        })
    });

    let items: Vec<InterventionItem> = match rows {
        Ok(r) => r.filter_map(|x| x.ok()).collect(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    Json(items).into_response()
}

async fn get_tools(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let mut stmt = match conn.prepare(
        "SELECT name, created_at, provider, command, description, args, responsibility, since FROM tool ORDER BY name ASC"
    ) {
        Ok(s) => s,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let rows = stmt.query_map([], |row| {
        Ok(ToolItem {
            name: row.get(0)?,
            created_at: row.get(1)?,
            provider: row.get(2)?,
            command: row.get(3)?,
            description: row.get(4)?,
            args: row.get(5)?,
            responsibility: row.get(6)?,
            since: row.get(7)?,
        })
    });

    let items: Vec<ToolItem> = match rows {
        Ok(r) => r.filter_map(|x| x.ok()).collect(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    Json(items).into_response()
}

async fn create_backlog(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<BacklogRequest>,
) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let res = conn.execute(
        "INSERT INTO backlog (title, discovered_while, current_pain, suggested_improvement, risk, notes, priority, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'proposed')",
        params![
            payload.title,
            payload.discovered_while,
            payload.current_pain,
            payload.suggested_improvement,
            payload.risk,
            payload.notes,
            payload.priority
        ],
    );

    match res {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn create_decision(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DecisionRequest>,
) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let res = conn.execute(
        "INSERT OR REPLACE INTO decision (id, title, status, doc_path, verify_command) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![payload.id, payload.title, payload.status, payload.doc_path, payload.verify_command],
    );

    match res {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn delete_story(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    match conn.execute("DELETE FROM story WHERE id = ?1", params![id]) {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn update_story(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<StoryUpdateRequest>,
) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let res = conn.execute(
        "UPDATE story SET title = ?1, status = ?2, priority = ?3, unit_proof = ?4, integration_proof = ?5, e2e_proof = ?6, evidence = ?7, risk_lane = ?8 WHERE id = ?9",
        params![
            payload.title,
            payload.status,
            payload.priority,
            payload.unit,
            payload.integration,
            payload.e2e,
            payload.evidence,
            payload.risk_lane,
            id
        ],
    );

    match res {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn delete_intake(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    match conn.execute("DELETE FROM intake WHERE id = ?1", params![id]) {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn update_intake(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(payload): Json<IntakeUpdateRequest>,
) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let res = conn.execute(
        "UPDATE intake SET input_type = ?1, summary = ?2, risk_lane = ?3, notes = ?4, story_id = ?5 WHERE id = ?6",
        params![payload.input_type, payload.summary, payload.risk_lane, payload.notes, payload.story_id, id],
    );

    match res {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn delete_decision(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    match conn.execute("DELETE FROM decision WHERE id = ?1", params![id]) {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn update_decision(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<DecisionUpdateRequest>,
) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let res = conn.execute(
        "UPDATE decision SET title = ?1, status = ?2, doc_path = ?3, verify_command = ?4, last_verified_result = ?5 WHERE id = ?6",
        params![payload.title, payload.status, payload.doc_path, payload.verify_command, payload.last_verified_result, id],
    );

    match res {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn delete_trace(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    match conn.execute("DELETE FROM trace WHERE id = ?1", params![id]) {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn update_trace(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(payload): Json<TraceUpdateRequest>,
) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let res = conn.execute(
        "UPDATE trace SET task_summary = ?1, outcome = ?2, story_id = ?3, agent = ?4, notes = ?5, decisions_made = ?6 WHERE id = ?7",
        params![payload.task_summary, payload.outcome, payload.story_id, payload.agent, payload.notes, payload.decisions_made, id],
    );

    match res {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn delete_backlog(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    match conn.execute("DELETE FROM backlog WHERE id = ?1", params![id]) {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn update_backlog(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(payload): Json<BacklogUpdateRequest>,
) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let res = conn.execute(
        "UPDATE backlog SET title = ?1, discovered_while = ?2, current_pain = ?3, suggested_improvement = ?4, risk = ?5, priority = ?6, status = ?7, notes = ?8 WHERE id = ?9",
        params![
            payload.title,
            payload.discovered_while,
            payload.current_pain,
            payload.suggested_improvement,
            payload.risk,
            payload.priority,
            payload.status,
            payload.notes,
            id
        ],
    );

    match res {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

#[derive(Deserialize)]
struct ResolveRequest {
    action: String,
}

async fn resolve_intervention(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(payload): Json<ResolveRequest>,
) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let mut stmt = match conn.prepare("SELECT type, story_id, description FROM intervention WHERE id = ?1") {
        Ok(s) => s,
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    };

    let intervention_info: Option<(String, Option<String>, String)> = stmt.query_row(params![id], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?))
    }).ok();

    if let Some((itype, story_id, desc)) = intervention_info {
        let resolution_suffix = format!(" [{}]", payload.action.to_uppercase());
        let updated_desc = format!("{}{}", desc, resolution_suffix);

        let _ = conn.execute(
            "UPDATE intervention SET description = ?1 WHERE id = ?2",
            params![updated_desc, id],
        );

        let trace_summary = format!(
            "Human resolved intervention #{} ({}): {} for Story {:?}",
            id, itype, payload.action.to_uppercase(), story_id
        );
        let _ = conn.execute(
            "INSERT INTO trace (task_summary, story_id, agent, outcome) VALUES (?1, ?2, 'Human', 'completed')",
            params![trace_summary, story_id],
        );

        StatusCode::OK.into_response()
    } else {
        (StatusCode::NOT_FOUND, "Intervention not found").into_response()
    }
}

#[derive(Serialize)]
struct VerifyResponse {
    success: bool,
    log: String,
}

async fn verify_decision(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let verify_command: Option<String> = match conn.query_row(
        "SELECT verify_command FROM decision WHERE id = ?1",
        params![id],
        |row| row.get(0)
    ) {
        Ok(cmd) => cmd,
        Err(_) => None,
    };

    let cmd = match verify_command {
        Some(c) if !c.trim().is_empty() => c,
        _ => return (StatusCode::BAD_REQUEST, "No verify command configured for this decision").into_response(),
    };

    let output = match Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .current_dir(state.db_path.parent().unwrap_or(&std::path::PathBuf::from(".")))
        .output()
        .await
    {
        Ok(out) => out,
        Err(e) => {
            return Json(VerifyResponse {
                success: false,
                log: format!("Failed to execute command: {}", e),
            }).into_response();
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let combined_log = format!("$ {}\n{}{}", cmd, stdout, stderr);
    let success = output.status.success();

    let result_str = if success { "pass" } else { "fail" };

    let _ = conn.execute(
        "UPDATE decision SET last_verified_at = datetime('now'), last_verified_result = ?1 WHERE id = ?2",
        params![result_str, id],
    );

    let trace_summary = format!("Automated verification run for ADR-{} (result: {})", id, result_str.to_uppercase());
    let _ = conn.execute(
        "INSERT INTO trace (task_summary, agent, outcome, notes) VALUES (?1, 'Governance QA', ?2, ?3)",
        params![trace_summary, if success { "completed" } else { "failed" }, combined_log],
    );

    Json(VerifyResponse {
        success,
        log: combined_log,
    }).into_response()
}

async fn execute_sql_query(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<QueryRequest>,
) -> impl IntoResponse {
    let query_upper = payload.query.trim().to_uppercase();
    if !query_upper.starts_with("SELECT") && !query_upper.starts_with("PRAGMA") {
        return (StatusCode::BAD_REQUEST, "Chỉ cho phép thực hiện các câu lệnh SELECT hoặc PRAGMA để đảm bảo an toàn dữ liệu.").into_response();
    }

    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let mut stmt = match conn.prepare(&payload.query) {
        Ok(s) => s,
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    };

    let col_count = stmt.column_count();
    let mut columns = Vec::new();
    for i in 0..col_count {
        columns.push(stmt.column_name(i).unwrap_or("").to_string());
    }

    let mut rows = Vec::new();
    let mut stmt_rows = match stmt.query([]) {
        Ok(r) => r,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    while let Ok(Some(row)) = stmt_rows.next() {
        let mut row_vals = Vec::new();
        for i in 0..col_count {
            let val = match row.get_ref(i) {
                Ok(rusqlite::types::ValueRef::Null) => serde_json::Value::Null,
                Ok(rusqlite::types::ValueRef::Integer(n)) => serde_json::Value::Number(serde_json::Number::from(n)),
                Ok(rusqlite::types::ValueRef::Real(f)) => serde_json::Value::Number(serde_json::Number::from_f64(f).unwrap_or(serde_json::Number::from(0))),
                Ok(rusqlite::types::ValueRef::Text(s)) => {
                    let s_str = std::str::from_utf8(s).unwrap_or("");
                    serde_json::Value::String(s_str.to_string())
                }
                Ok(rusqlite::types::ValueRef::Blob(b)) => serde_json::Value::String(format!("Blob: {} bytes", b.len())),
                Err(_) => serde_json::Value::Null,
            };
            row_vals.push(val);
        }
        rows.push(row_vals);
    }

    Json(QueryResponse { columns, rows }).into_response()
}

async fn ws_handler(
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(handle_ws_socket)
}

async fn handle_ws_socket(mut socket: WebSocket) {
    let mock_logs = vec![
        "PM Agent: Đọc spec-intake.md và phân tích các trường yêu cầu...",
        "PM Agent: Khởi tạo User Story US-006 trong bảng Story...",
        "BA Agent: Viết mã nguồn Rust cho struct và endpoint...",
        "BE Agent: Cài đặt router Axum và chạy kiểm tra cargo check...",
        "BE Agent: Khởi chạy cargo build --package harness-web thành công.",
        "FE Agent: Phác thảo cấu trúc file HTML mới cho dashboard...",
        "FE Agent: Cập nhật CSS phong cách Paperclip, thiết lập glassmorphism...",
        "FE Agent: Ghép nối SPA router và local cache vào app.js...",
        "QA Agent: Viết các test scenarios cho màn hình dashboard mới...",
        "QA Agent: Chạy kiểm thử tự động - 3 bài test thông qua (PASS).",
        "Auditor: Quá trình kiểm tra độ lệch cấu trúc (drift audit) hoàn thành. Độ hỗn loạn (entropy): 0.05.",
    ];

    let mut index = 0;
    loop {
        let msg = mock_logs[index % mock_logs.len()];
        if socket.send(WsMessage::Text(msg.into())).await.is_err() {
            break;
        }
        index += 1;
        sleep(Duration::from_secs(4)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_serialization() {
        let stats = Stats {
            intakes: 10,
            stories: 5,
            decisions: 2,
            backlog: 1,
            traces: 20,
        };
        let serialized = serde_json::to_string(&stats).unwrap();
        assert!(serialized.contains("\"intakes\":10"));
        assert!(serialized.contains("\"stories\":5"));
        assert!(serialized.contains("\"decisions\":2"));
        assert!(serialized.contains("\"backlog\":1"));
        assert!(serialized.contains("\"traces\":20"));
    }
}
