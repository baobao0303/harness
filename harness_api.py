#!/usr/bin/env python3
"""
Harness API Backend - FastAPI wrapper for harness-cli.exe
Provides REST API + WebSocket for the static UI
"""
import asyncio
import json
import subprocess
import sqlite3
from pathlib import Path
from typing import Optional, List, Dict, Any
from contextlib import asynccontextmanager

from fastapi import FastAPI, WebSocket, WebSocketDisconnect, HTTPException
from fastapi.staticfiles import StaticFiles
from fastapi.responses import FileResponse, JSONResponse
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel

HARNESS_ROOT = Path("D:/harness")
CLI_PATH = HARNESS_ROOT / "scripts/bin/harness-cli.exe"
DB_PATH = HARNESS_ROOT / "harness.db"
UI_DIR = HARNESS_ROOT / "crates/harness-web/ui"

# WebSocket connection manager
class ConnectionManager:
    def __init__(self):
        self.active_connections: List[WebSocket] = []

    async def connect(self, websocket: WebSocket):
        await websocket.accept()
        self.active_connections.append(websocket)

    def disconnect(self, websocket: WebSocket):
        if websocket in self.active_connections:
            self.active_connections.remove(websocket)

    async def broadcast(self, message: str):
        for conn in self.active_connections:
            try:
                await conn.send_text(message)
            except:
                pass

manager = ConnectionManager()

def run_cli(args: List[str]) -> str:
    """Run harness-cli.exe and return stdout"""
    result = subprocess.run(
        [str(CLI_PATH)] + args,
        capture_output=True,
        text=True,
        cwd=HARNESS_ROOT
    )
    if result.returncode != 0:
        raise HTTPException(status_code=500, detail=result.stderr)
    return result.stdout

def query_db(sql: str, params: tuple = ()) -> List[Dict]:
    """Query SQLite database directly"""
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    cursor = conn.cursor()
    cursor.execute(sql, params)
    rows = [dict(row) for row in cursor.fetchall()]
    conn.close()
    return rows

# Pydantic models
class IntakeRequest(BaseModel):
    input_type: str
    summary: str
    risk_lane: str
    notes: Optional[str] = None
    story_id: Optional[str] = None

class StoryRequest(BaseModel):
    id: str
    title: str
    lane: str
    priority: str

class TraceRequest(BaseModel):
    task_summary: str
    story_id: Optional[str] = None
    agent: Optional[str] = None
    outcome: str
    decisions_made: Optional[str] = None

class BacklogRequest(BaseModel):
    title: str
    discovered_while: Optional[str] = None
    current_pain: Optional[str] = None
    suggested_improvement: Optional[str] = None
    risk: Optional[str] = None
    notes: Optional[str] = None
    priority: str

class DecisionRequest(BaseModel):
    id: str
    title: str
    status: str
    doc_path: Optional[str] = None
    verify_command: Optional[str] = None

# Background task to simulate live telemetry
async def telemetry_broadcaster():
    """Broadcast fake telemetry for demo - replace with real data source"""
    counter = 0
    while True:
        await asyncio.sleep(10)
        counter += 1
        msg = f"[TELEMETRY] Heartbeat #{counter} - System healthy"
        await manager.broadcast(msg)

@asynccontextmanager
async def lifespan(app: FastAPI):
    # Startup
    asyncio.create_task(telemetry_broadcaster())
    yield
    # Shutdown
    pass

app = FastAPI(title="Harness API", lifespan=lifespan)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# ====== REST API ENDPOINTS ======

@app.get("/api/stats")
async def get_stats():
    stats = {
        "intakes": query_db("SELECT COUNT(*) as c FROM intake")[0]["c"],
        "stories": query_db("SELECT COUNT(*) as c FROM story")[0]["c"],
        "decisions": query_db("SELECT COUNT(*) as c FROM decision")[0]["c"],
        "backlog": query_db("SELECT COUNT(*) as c FROM backlog")[0]["c"],
        "traces": query_db("SELECT COUNT(*) as c FROM trace")[0]["c"],
    }
    return stats

@app.get("/api/matrix")
async def get_matrix():
    rows = query_db("""
        SELECT id, title, status, unit_proof, integration_proof, e2e_proof, 
               platform_proof, evidence, priority, risk_lane 
        FROM story
    """)
    return rows

@app.get("/api/decisions")
async def get_decisions():
    rows = query_db("""
        SELECT id, title, created_at, status, doc_path, verify_command, 
               last_verified_at, last_verified_result 
        FROM decision
    """)
    return rows

@app.get("/api/traces")
async def get_traces():
    rows = query_db("""
        SELECT id, created_at, task_summary, intake_id, story_id, agent, 
               outcome, duration_seconds, token_estimate, harness_friction, 
               notes, decisions_made 
        FROM trace ORDER BY id DESC LIMIT 50
    """)
    return rows

@app.get("/api/intakes")
async def get_intakes():
    rows = query_db("""
        SELECT id, created_at, input_type, summary, risk_lane, notes, story_id 
        FROM intake ORDER BY id DESC
    """)
    return rows

@app.get("/api/backlog")
async def get_backlog():
    rows = query_db("""
        SELECT id, created_at, title, discovered_while, current_pain, 
               suggested_improvement, risk, status, notes, priority 
        FROM backlog ORDER BY id DESC
    """)
    return rows

@app.get("/api/interventions")
async def get_interventions():
    rows = query_db("""
        SELECT id, created_at, trace_id, story_id, type, description, source, impact 
        FROM intervention ORDER BY id DESC
    """)
    return rows

@app.get("/api/tools")
async def get_tools():
    rows = query_db("""
        SELECT name, created_at, provider, command, description, args, responsibility, since 
        FROM tool
    """)
    return rows

@app.post("/api/intake")
async def create_intake(req: IntakeRequest):
    query_db("""
        INSERT INTO intake (input_type, summary, risk_lane, notes, story_id) 
        VALUES (?, ?, ?, ?, ?)
    """, (req.input_type, req.summary, req.risk_lane, req.notes, req.story_id))
    return {"status": "created"}

@app.post("/api/story")
async def create_story(req: StoryRequest):
    query_db("""
        INSERT INTO story (id, title, risk_lane, priority, status) 
        VALUES (?, ?, ?, ?, 'planned')
    """, (req.id, req.title, req.lane, req.priority))
    return {"status": "created"}

@app.post("/api/trace")
async def create_trace(req: TraceRequest):
    query_db("""
        INSERT INTO trace (task_summary, story_id, agent, outcome, decisions_made) 
        VALUES (?, ?, ?, ?, ?)
    """, (req.task_summary, req.story_id, req.agent, req.outcome, req.decisions_made))
    # Broadcast to WebSocket clients
    await manager.broadcast(f"[TRACE] {req.agent or 'agent'}: {req.task_summary} -> {req.outcome}")
    return {"status": "created"}

@app.post("/api/backlog")
async def create_backlog(req: BacklogRequest):
    query_db("""
        INSERT INTO backlog (title, discovered_while, current_pain, suggested_improvement, 
                             risk, notes, priority, status) 
        VALUES (?, ?, ?, ?, ?, ?, ?, 'proposed')
    """, (req.title, req.discovered_while, req.current_pain, req.suggested_improvement,
          req.risk, req.notes, req.priority))
    return {"status": "created"}

@app.post("/api/decision")
async def create_decision(req: DecisionRequest):
    query_db("""
        INSERT INTO decision (id, title, status, doc_path, verify_command) 
        VALUES (?, ?, ?, ?, ?)
    """, (req.id, req.title, req.status, req.doc_path, req.verify_command))
    return {"status": "created"}

@app.put("/api/intake/{intake_id}")
async def update_intake(intake_id: int, req: IntakeRequest):
    query_db("""
        UPDATE intake SET input_type=?, summary=?, risk_lane=?, notes=?, story_id=? 
        WHERE id=?
    """, (req.input_type, req.summary, req.risk_lane, req.notes, req.story_id, intake_id))
    return {"status": "updated"}

@app.put("/api/story/{story_id}")
async def update_story(story_id: str, req: dict):
    # Flexible update for story fields
    fields = []
    values = []
    for key, val in req.items():
        if key in ["title", "status", "priority", "unit", "integration", "e2e", "platform", "evidence", "risk_lane"]:
            db_key = {
                "unit": "unit_proof", "integration": "integration_proof", 
                "e2e": "e2e_proof", "platform": "platform_proof"
            }.get(key, key)
            fields.append(f"{db_key}=?")
            values.append(val)
    if fields:
        values.append(story_id)
        query_db(f"UPDATE story SET {', '.join(fields)} WHERE id=?", tuple(values))
    return {"status": "updated"}

@app.delete("/api/intake/{intake_id}")
async def delete_intake(intake_id: int):
    query_db("DELETE FROM intake WHERE id=?", (intake_id,))
    return {"status": "deleted"}

@app.delete("/api/story/{story_id}")
async def delete_story(story_id: str):
    query_db("DELETE FROM story WHERE id=?", (story_id,))
    return {"status": "deleted"}

@app.delete("/api/trace/{trace_id}")
async def delete_trace(trace_id: int):
    query_db("DELETE FROM trace WHERE id=?", (trace_id,))
    return {"status": "deleted"}

@app.delete("/api/backlog/{backlog_id}")
async def delete_backlog(backlog_id: int):
    query_db("DELETE FROM backlog WHERE id=?", (backlog_id,))
    return {"status": "deleted"}

@app.delete("/api/decision/{decision_id}")
async def delete_decision(decision_id: str):
    query_db("DELETE FROM decision WHERE id=?", (decision_id,))
    return {"status": "deleted"}

@app.post("/api/query")
async def execute_sql(q: dict):
    """Execute arbitrary SQL query"""
    query = q.get("query", "")
    if not query.strip().upper().startswith("SELECT"):
        raise HTTPException(status_code=400, detail="Only SELECT queries allowed")
    try:
        rows = query_db(query)
        if rows:
            columns = list(rows[0].keys())
        else:
            columns = []
        return {"columns": columns, "rows": rows}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# ====== WEBSOCKET ======

@app.websocket("/api/ws")
async def websocket_endpoint(websocket: WebSocket):
    await manager.connect(websocket)
    try:
        await websocket.send_text("[SYSTEM] Connected to live agent telemetry stream.")
        while True:
            # Keep connection alive, listen for client messages if needed
            data = await websocket.receive_text()
            # Echo back or process
            await websocket.send_text(f"[ECHO] {data}")
    except WebSocketDisconnect:
        manager.disconnect(websocket)

# ====== STATIC FILES (UI) ======

app.mount("/static", StaticFiles(directory=UI_DIR), name="static")

@app.get("/")
async def root():
    return FileResponse(UI_DIR / "index.html")

@app.get("/{path:path}")
async def catch_all(path: str):
    # SPA fallback - serve index.html for client-side routing
    file_path = UI_DIR / path
    if file_path.exists() and file_path.is_file():
        return FileResponse(file_path)
    return FileResponse(UI_DIR / "index.html")

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=3000)