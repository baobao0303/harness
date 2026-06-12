use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::services::ServeDir;

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

#[derive(Deserialize)]
struct IntakeRequest {
    input_type: String,
    summary: String,
    risk_lane: String,
    notes: Option<String>,
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

    let app = Router::new()
        .route("/api/stats", get(get_stats))
        .route("/api/matrix", get(get_matrix))
        .route("/api/decisions", get(get_decisions))
        .route("/api/intake", post(create_intake))
        .route("/api/story", post(create_story))
        .route("/api/trace", post(create_trace))
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

async fn get_matrix(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let conn = match Connection::open(&state.db_path) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let mut stmt = match conn.prepare(
        "SELECT id, title, status, unit_proof, integration_proof, e2e_proof, platform_proof, evidence, priority FROM story"
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
        "INSERT INTO intake (input_type, summary, risk_lane, notes) VALUES (?1, ?2, ?3, ?4)",
        params![payload.input_type, payload.summary, payload.risk_lane, payload.notes],
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
        "INSERT INTO trace (task_summary, story_id, agent, outcome) VALUES (?1, ?2, ?3, ?4)",
        params![payload.task_summary, payload.story_id, payload.agent, payload.outcome],
    );

    match res {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
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
