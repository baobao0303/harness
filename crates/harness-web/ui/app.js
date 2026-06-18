// Harness Workspace - Client Application Script (SPA)

// Global state cache for local search queries
let cache = {
    intakes: [],
    stories: [],
    decisions: [],
    traces: [],
    backlog: [],
    interventions: [],
    tools: []
};

// Active editing state IDs
let editingIntakeId = null;
let editingStoryId = null;
let editingDecisionId = null;
let editingTraceId = null;
let editingBacklogId = null;


document.addEventListener("DOMContentLoaded", () => {
    // Initial theme setup
    initTheme();

    // Set up SPA navigation
    initNavigation();

    // Load initial data
    loadData();

    // Auto-refresh data every 3 seconds for real-time telemetry
    setInterval(loadData, 3000);

    // Global keyboard shortcuts
    window.addEventListener("keydown", handleGlobalKeys);

    // Initialize WebSocket console log stream
    initTerminalWS();
});

// Theme Management
function initTheme() {
    const savedTheme = localStorage.getItem("harness-theme") || "dark";
    setTheme(savedTheme);
}

function setTheme(theme) {
    const body = document.body;
    const themeIcon = document.querySelector(".theme-icon");
    const themeText = document.querySelector(".theme-text");

    if (theme === "light") {
        body.classList.remove("dark");
        body.classList.add("light");
        if (themeIcon) themeIcon.innerText = "☀️";
        if (themeText) themeText.innerText = "Light Mode";
    } else {
        body.classList.remove("light");
        body.classList.add("dark");
        if (themeIcon) themeIcon.innerText = "🌙";
        if (themeText) themeText.innerText = "Dark Mode";
    }
    localStorage.setItem("harness-theme", theme);
}

function toggleTheme() {
    const currentTheme = document.body.classList.contains("dark") ? "light" : "dark";
    setTheme(currentTheme);
}

// SPA Routing and Navigation
function initNavigation() {
    const navItems = document.querySelectorAll(".nav-item");
    navItems.forEach(item => {
        item.addEventListener("click", (e) => {
            e.preventDefault();
            const tabId = item.getAttribute("data-tab");
            switchTab(tabId);
        });
    });
}

function switchTab(tabId) {
    // 1. Toggle active nav item
    document.querySelectorAll(".nav-item").forEach(el => {
        el.classList.remove("active");
        if (el.getAttribute("data-tab") === tabId) {
            el.classList.add("active");
        }
    });

    // 2. Toggle active tab panel
    document.querySelectorAll(".tab-panel").forEach(el => {
        el.classList.remove("active");
    });
    const targetPanel = document.getElementById(`panel-${tabId}`);
    if (targetPanel) {
        targetPanel.classList.add("active");
    }

    // 3. Update breadcrumbs
    const pageTitles = {
        dashboard: "Dashboard",
        intakes: "Intakes Backlog",
        matrix: "Stories & Proofs",
        decisions: "Design Decisions (ADRs)",
        graph: "Traceability Graph",
        traces: "Agent Traces Timeline",
        agents: "Agent Directory",
        interventions: "Interventions & Backlog",
        sql: "SQL Console"
    };
    document.getElementById("current-page-title").innerText = pageTitles[tabId] || "Dashboard";

    // 4. Update contextual header action buttons
    renderHeaderActions(tabId);

    // 5. Render graph if active
    if (tabId === "graph") {
        renderDependencyGraph();
    }
}

function renderHeaderActions(tabId) {
    const container = document.getElementById("header-action-container");
    if (!container) return;
    container.innerHTML = "";

    if (tabId === "intakes") {
        container.innerHTML = `<button class="cta-btn" onclick="openModal('intake-modal')">+ Register Intake</button>`;
    } else if (tabId === "matrix") {
        container.innerHTML = `<button class="cta-btn" onclick="openModal('story-modal')">+ Create Story</button>`;
    } else if (tabId === "decisions") {
        container.innerHTML = `<button class="cta-btn" onclick="openModal('decision-modal')">+ Create Decision</button>`;
    } else if (tabId === "traces") {
        container.innerHTML = `<button class="cta-btn" onclick="openModal('trace-modal')">+ Record Trace</button>`;
    } else if (tabId === "interventions") {
        container.innerHTML = `<button class="cta-btn" onclick="openModal('backlog-modal')">+ Add Proposal</button>`;
    }
}

// Telemetry Data Loading & Polling
async function loadData() {
    try {
        await Promise.all([
            loadStats(),
            loadIntakes(),
            loadMatrix(),
            loadDecisions(),
            loadTraces(),
            loadBacklog(),
            loadInterventions(),
            loadTools()
        ]);
        updateDbStatus(true, "Database online");
        renderDashboard();
    } catch (error) {
        console.error("Lỗi khi đồng bộ dữ liệu telemetry:", error);
        updateDbStatus(false, "Database connection failure");
    }
}

function updateDbStatus(success, message) {
    const dot = document.getElementById("db-status-dot");
    const text = document.getElementById("db-status-text");

    if (text) text.innerText = message;
    if (dot) {
        if (success) {
            dot.style.backgroundColor = "var(--success-color)";
            dot.style.boxShadow = "0 0 0 0 rgba(16, 185, 129, 0.7)";
        } else {
            dot.style.backgroundColor = "var(--danger-color)";
            dot.style.boxShadow = "0 0 0 0 rgba(239, 68, 68, 0.7)";
        }
    }
}

// API Loaders
async function loadStats() {
    const res = await fetch("/api/stats");
    if (!res.ok) throw new Error("API stats error");
    const stats = await res.json();
    document.getElementById("stat-intakes").innerText = stats.intakes;
    document.getElementById("stat-stories").innerText = stats.stories;
    document.getElementById("stat-decisions").innerText = stats.decisions;
    document.getElementById("stat-traces").innerText = stats.traces;
}

async function loadIntakes() {
    const res = await fetch("/api/intakes");
    if (!res.ok) throw new Error("API intakes error");
    cache.intakes = await res.json();
    renderIntakes();
}

async function loadMatrix() {
    const res = await fetch("/api/matrix");
    if (!res.ok) throw new Error("API matrix error");
    cache.stories = await res.json();
    renderMatrix();
}

async function loadDecisions() {
    const res = await fetch("/api/decisions");
    if (!res.ok) throw new Error("API decisions error");
    cache.decisions = await res.json();
    renderDecisions();
}

async function loadTraces() {
    const res = await fetch("/api/traces");
    if (!res.ok) throw new Error("API traces error");
    cache.traces = await res.json();
    renderTraces();
}

async function loadBacklog() {
    const res = await fetch("/api/backlog");
    if (!res.ok) throw new Error("API backlog error");
    cache.backlog = await res.json();
    renderBacklog();
}

async function loadInterventions() {
    const res = await fetch("/api/interventions");
    if (!res.ok) throw new Error("API interventions error");
    cache.interventions = await res.json();
    renderInterventions();
}

async function loadTools() {
    const res = await fetch("/api/tools");
    if (!res.ok) throw new Error("API tools error");
    cache.tools = await res.json();
}

// Renderers
function renderDashboard() {
    // 1. Render active stories list (Limit 5)
    const tbody = document.getElementById("dash-stories-body");
    if (tbody) {
        tbody.innerHTML = "";
        const activeStories = cache.stories.slice(0, 5);
        if (activeStories.length === 0) {
            tbody.innerHTML = `<tr><td colspan="4" class="loading-cell">No active stories.</td></tr>`;
        } else {
            activeStories.forEach(s => {
                const tr = document.createElement("tr");
                tr.innerHTML = `
                    <td><strong>${s.id}</strong></td>
                    <td>${s.title}</td>
                    <td><span class="priority-badge p-${s.priority.toLowerCase()}">${s.priority}</span></td>
                    <td><span class="badge badge-${s.status.toLowerCase()}">${s.status}</span></td>
                `;
                tbody.appendChild(tr);
            });
        }
    }

    // 2. Render recent traces timeline (Limit 5)
    const container = document.getElementById("dash-traces-container");
    if (container) {
        container.innerHTML = "";
        const recentTraces = cache.traces.slice(0, 5);
        if (recentTraces.length === 0) {
            container.innerHTML = `<p class="loading-cell">No execution logs found.</p>`;
        } else {
            recentTraces.forEach(t => {
                const row = document.createElement("div");
                row.className = "timeline-row";
                
                const timeStr = formatShortTime(t.created_at);
                const agentClass = getAgentClass(t.agent);
                const outcomeClass = getOutcomeClass(t.outcome);

                row.innerHTML = `
                    <div class="timeline-left">
                        <span class="trace-avatar ${agentClass}">${t.agent ? t.agent.substring(0,2).toUpperCase() : "AG"}</span>
                        <span class="timeline-text" title="${t.task_summary}">${t.task_summary}</span>
                    </div>
                    <div>
                        <span class="badge ${outcomeClass}">${t.outcome}</span>
                    </div>
                `;
                container.appendChild(row);
            });
        }
    }
    
    // 3. Render friction and blockers analytics
    renderFrictionAnalytics();
}

function renderIntakes() {
    const tbody = document.getElementById("intakes-body");
    if (!tbody) return;
    tbody.innerHTML = "";

    if (cache.intakes.length === 0) {
        tbody.innerHTML = `<tr><td colspan="7" class="loading-cell">No registered feature intakes.</td></tr>`;
        return;
    }

    cache.intakes.forEach(i => {
        const tr = document.createElement("tr");
        tr.innerHTML = `
            <td><strong>${i.id}</strong></td>
            <td class="evidence-text">${formatShortTime(i.created_at)}</td>
            <td><code class="evidence-text">${i.input_type}</code></td>
            <td>${i.summary}</td>
            <td><span class="risk-tag risk-${i.risk_lane.toLowerCase()}">${i.risk_lane}</span></td>
            <td>${i.story_id ? `<strong class="trace-link" onclick="switchTab('matrix')">${i.story_id}</strong>` : "-"}</td>
            <td class="actions-cell">
                <button class="action-icon-btn edit" onclick="startEditIntake(${i.id})" title="Sửa">✏️</button>
                <button class="action-icon-btn delete" onclick="deleteIntake(${i.id})" title="Xoá">❌</button>
            </td>
        `;
        tbody.appendChild(tr);
    });
}

function renderMatrix() {
    const tbody = document.getElementById("matrix-body");
    if (!tbody) return;
    tbody.innerHTML = "";

    if (cache.stories.length === 0) {
        tbody.innerHTML = `<tr><td colspan="10" class="loading-cell">No user stories found in the matrix.</td></tr>`;
        return;
    }

    cache.stories.forEach(s => {
        const tr = document.createElement("tr");
        const proofHtml = (v) => v > 0 
            ? `<span class="proof-badge proof-pass">PASS</span>` 
            : `<span class="proof-badge proof-fail">FAIL</span>`;

        tr.innerHTML = `
            <td><strong>${s.id}</strong></td>
            <td>${s.title}</td>
            <td><span class="priority-badge p-${s.priority.toLowerCase()}">${s.priority}</span></td>
            <td><span class="risk-tag risk-${s.risk_lane.toLowerCase()}">${s.risk_lane}</span></td>
            <td><span class="badge badge-${s.status.toLowerCase()}">${s.status}</span></td>
            <td class="text-center">${proofHtml(s.unit)}</td>
            <td class="text-center">${proofHtml(s.integration)}</td>
            <td class="text-center">${proofHtml(s.e2e)}</td>
            <td><span class="evidence-text">${s.evidence || "-"}</span></td>
            <td class="actions-cell">
                <button class="action-icon-btn edit" onclick="startEditStory('${s.id}')" title="Sửa">✏️</button>
                <button class="action-icon-btn delete" onclick="deleteStory('${s.id}')" title="Xoá">❌</button>
            </td>
        `;
        tbody.appendChild(tr);
    });
}

function renderDecisions() {
    const container = document.getElementById("decisions-container");
    if (!container) return;
    container.innerHTML = "";

    if (cache.decisions.length === 0) {
        container.innerHTML = `<p class="loading-cell">No design decisions registered.</p>`;
        return;
    }

    cache.decisions.forEach(d => {
        const card = document.createElement("div");
        card.className = "decision-card";
        
        let verifyHtml = "";
        if (d.verify_command) {
            const resultBadge = d.last_verified_result === "pass"
                ? `<span class="verify-badge verify-pass">PASS</span>`
                : d.last_verified_result === "fail"
                    ? `<span class="verify-badge verify-fail">FAIL</span>`
                    : `<span class="verify-badge verify-pending">PENDING</span>`;

            verifyHtml = `
                <div class="verify-row">
                    <div>Command: <code>${d.verify_command}</code></div>
                    <div style="margin-top:0.2rem">Result: ${resultBadge} ${d.last_verified_at ? `<span class="evidence-text">(${formatShortTime(d.last_verified_at)})</span>` : ""}</div>
                </div>
            `;
        }

        card.innerHTML = `
            <div class="decision-header">
                <span class="decision-id">ADR-${d.id}</span>
                <span class="decision-status-row">
                    <span class="status-label status-${d.status.toLowerCase()}">${d.status}</span>
                </span>
            </div>
            <h4 class="decision-title">${d.title}</h4>
            <span class="decision-date">Created at: ${formatShortTime(d.created_at)}</span>
            ${verifyHtml}
            ${d.doc_path ? `<a href="/${d.doc_path}" target="_blank" class="doc-link">View Documentation ↗</a>` : ""}
            <div class="card-actions-bar">
                ${d.verify_command ? `<button class="action-icon-btn edit" onclick="verifyDecision('${d.id}')" title="Chạy lệnh kiểm chứng (Verify)">⚡</button>` : ""}
                <button class="action-icon-btn edit" onclick="startEditDecision('${d.id}')" title="Sửa">✏️</button>
                <button class="action-icon-btn delete" onclick="deleteDecision('${d.id}')" title="Xoá">❌</button>
            </div>
        `;
        container.appendChild(card);
    });
}

function renderTraces() {
    const container = document.getElementById("traces-container");
    if (!container) return;
    container.innerHTML = "";

    if (cache.traces.length === 0) {
        container.innerHTML = `<p class="loading-cell">No agent execution traces logged.</p>`;
        return;
    }

    cache.traces.forEach(t => {
        const item = document.createElement("div");
        item.className = "trace-item";

        const dotClass = t.outcome === "completed" ? "completed"
            : t.outcome === "partial" ? "partial"
            : t.outcome === "blocked" ? "blocked"
            : "failed";

        const agentClass = getAgentClass(t.agent);
        const outcomeClass = getOutcomeClass(t.outcome);

        item.innerHTML = `
            <div class="trace-dot ${dotClass}"></div>
            <div class="trace-card">
                <div class="trace-header">
                    <div class="trace-agent-capsule">
                        <span class="trace-avatar ${agentClass}">${t.agent ? t.agent.substring(0,2).toUpperCase() : "AG"}</span>
                        <span class="trace-agent-name">${t.agent || "Antigravity"}</span>
                    </div>
                    <span class="trace-time">${formatShortTime(t.created_at)}</span>
                </div>
                <div class="trace-summary">${t.task_summary}</div>
                <div class="trace-metadata">
                    <span>Outcome: <span class="badge ${outcomeClass}">${t.outcome}</span></span>
                    ${t.story_id ? `<span>Story: <strong class="trace-link" onclick="switchTab('matrix')">${t.story_id}</strong></span>` : ""}
                    ${t.duration_seconds ? `<span>Duration: <code class="evidence-text">${t.duration_seconds}s</code></span>` : ""}
                    ${t.token_estimate ? `<span>Tokens: <code class="evidence-text">${t.token_estimate}</code></span>` : ""}
                </div>
                ${t.notes ? `<div style="font-size:0.75rem; color:var(--text-muted); margin-top:0.4rem; border-left:2px solid var(--border-color); padding-left:0.5rem">${t.notes}</div>` : ""}
                <div class="card-actions-bar">
                    <button class="action-icon-btn edit" onclick="startEditTrace(${t.id})" title="Sửa">✏️</button>
                    <button class="action-icon-btn delete" onclick="deleteTrace(${t.id})" title="Xoá">❌</button>
                </div>
            </div>
        `;
        container.appendChild(item);
    });
}

function renderBacklog() {
    const bodies = {
        "P0": document.getElementById("kanban-body-p0"),
        "P1": document.getElementById("kanban-body-p1"),
        "P2": document.getElementById("kanban-body-p2")
    };
    
    const counts = {
        "P0": document.getElementById("count-p0"),
        "P1": document.getElementById("count-p1"),
        "P2": document.getElementById("count-p2")
    };

    // Reset all columns
    Object.keys(bodies).forEach(p => {
        if (bodies[p]) bodies[p].innerHTML = "";
        if (counts[p]) counts[p].innerText = "0";
    });

    const colCounts = { "P0": 0, "P1": 0, "P2": 0 };

    cache.backlog.forEach(b => {
        const pKey = b.priority ? b.priority.toUpperCase() : "P2";
        const columnBody = bodies[pKey] || bodies["P2"];
        if (!columnBody) return;

        colCounts[pKey] = (colCounts[pKey] || 0) + 1;

        const card = document.createElement("div");
        card.className = "kanban-card";
        card.setAttribute("draggable", "true");
        card.setAttribute("ondragstart", `dragBacklog(event, ${b.id})`);
        
        card.innerHTML = `
            <div class="kanban-card-title">${b.title}</div>
            <div style="font-size: 0.72rem; color: var(--text-muted); margin-bottom: 0.4rem; line-height: 1.4;">
                ${b.discovered_while ? `<div>🔍 ${b.discovered_while}</div>` : ""}
                ${b.current_pain ? `<div style="color: var(--danger-color);">⚠️ ${b.current_pain}</div>` : ""}
            </div>
            <div class="kanban-card-meta">
                <span class="badge badge-planned" style="font-size: 0.62rem; padding: 1px 4px;">${b.status}</span>
                <div style="display: flex; gap: 0.3rem;">
                    <span style="cursor: pointer; opacity: 0.7;" onclick="startEditBacklog(${b.id})" title="Sửa">✏️</span>
                    <span style="cursor: pointer; opacity: 0.7;" onclick="deleteBacklog(${b.id})" title="Xoá">❌</span>
                </div>
            </div>
        `;
        columnBody.appendChild(card);
    });

    // Update headers counts
    Object.keys(counts).forEach(p => {
        if (counts[p]) counts[p].innerText = colCounts[p];
    });
}

function renderInterventions() {
    const container = document.getElementById("interventions-container");
    if (!container) return;
    container.innerHTML = "";

    if (cache.interventions.length === 0) {
        container.innerHTML = `<p class="loading-cell">No human or CI interventions logged.</p>`;
        return;
    }

    cache.interventions.forEach(i => {
        const card = document.createElement("div");
        card.className = `intervention-card ${i.type.toLowerCase()}`;
        
        const isPending = !i.description.includes("[APPROVED]") && !i.description.includes("[REJECTED]") && !i.description.includes("[APPROVE]") && !i.description.includes("[REJECT]");
        const actionHtml = isPending ? `
            <div class="hil-btn-group">
                <button class="hil-btn approve" onclick="resolveIntervention(${i.id}, 'approve')">Approve</button>
                <button class="hil-btn reject" onclick="resolveIntervention(${i.id}, 'reject')">Reject</button>
            </div>
        ` : "";

        card.innerHTML = `
            <div class="intervention-header">
                <span><strong>${i.source.toUpperCase()} ${i.type.toUpperCase()}</strong></span>
                <span>${formatShortTime(i.created_at)}</span>
            </div>
            <div class="intervention-desc">${i.description}</div>
            ${i.impact ? `<div class="evidence-text" style="font-size:0.72rem; color:var(--text-muted); margin-top:0.2rem">Impact: ${i.impact}</div>` : ""}
            ${actionHtml}
        `;
        container.appendChild(card);
    });
}

// Form Submission Actions
async function submitIntake(event) {
    event.preventDefault();
    const payload = {
        input_type: document.getElementById("intake-type").value,
        summary: document.getElementById("intake-summary").value,
        risk_lane: document.getElementById("intake-lane").value,
        notes: document.getElementById("intake-notes").value || null
    };

    const isEdit = editingIntakeId !== null;
    const url = isEdit ? `/api/intake/${editingIntakeId}` : "/api/intake";
    const method = isEdit ? "PUT" : "POST";

    try {
        const res = await fetch(url, {
            method: method,
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload)
        });

        if (res.ok) {
            closeModal("intake-modal");
            loadData();
        } else {
            alert(isEdit ? "Lỗi khi cập nhật feature intake." : "Lỗi khi thêm feature intake. Vui lòng thử lại.");
        }
    } catch (e) {
        console.error(e);
    }
}

async function submitStory(event) {
    event.preventDefault();
    const isEdit = editingStoryId !== null;

    let url, method, payload;
    if (isEdit) {
        const item = cache.stories.find(x => x.id === editingStoryId);
        url = `/api/story/${editingStoryId}`;
        method = "PUT";
        payload = {
            title: document.getElementById("story-title").value.trim(),
            status: item ? item.status : "planned",
            priority: document.getElementById("story-priority").value,
            unit: item ? item.unit : 0,
            integration: item ? item.integration : 0,
            e2e: item ? item.e2e : 0,
            evidence: item ? item.evidence : null,
            risk_lane: document.getElementById("story-lane").value
        };
    } else {
        url = "/api/story";
        method = "POST";
        payload = {
            id: document.getElementById("story-id").value.trim(),
            title: document.getElementById("story-title").value.trim(),
            lane: document.getElementById("story-lane").value,
            priority: document.getElementById("story-priority").value
        };
    }

    try {
        const res = await fetch(url, {
            method: method,
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload)
        });

        if (res.ok) {
            closeModal("story-modal");
            loadData();
        } else {
            alert(isEdit ? "Lỗi khi cập nhật user story." : "Lỗi khi tạo user story. ID có thể đã bị trùng.");
        }
    } catch (e) {
        console.error(e);
    }
}

async function submitTrace(event) {
    event.preventDefault();
    const isEdit = editingTraceId !== null;

    const summary = document.getElementById("trace-summary").value.trim();
    const storyId = document.getElementById("trace-story-id").value.trim() || null;
    const agent = document.getElementById("trace-agent").value.trim() || null;
    const outcome = document.getElementById("trace-outcome").value;

    let url, method, payload;
    if (isEdit) {
        const item = cache.traces.find(x => x.id === editingTraceId);
        url = `/api/trace/${editingTraceId}`;
        method = "PUT";
        payload = {
            task_summary: summary,
            outcome: outcome,
            story_id: storyId,
            agent: agent,
            notes: item ? item.notes : null
        };
    } else {
        url = "/api/trace";
        method = "POST";
        payload = {
            task_summary: summary,
            story_id: storyId,
            agent: agent,
            outcome: outcome
        };
    }

    try {
        const res = await fetch(url, {
            method: method,
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload)
        });

        if (res.ok) {
            closeModal("trace-modal");
            loadData();
        } else {
            alert(isEdit ? "Lỗi khi cập nhật agent trace." : "Lỗi khi ghi nhận agent trace.");
        }
    } catch (e) {
        console.error(e);
    }
}

async function submitBacklog(event) {
    event.preventDefault();
    const isEdit = editingBacklogId !== null;

    const title = document.getElementById("backlog-title").value.trim();
    const discovered = document.getElementById("backlog-discovered").value.trim() || null;
    const pain = document.getElementById("backlog-pain").value.trim() || null;
    const improvement = document.getElementById("backlog-improvement").value.trim() || null;
    const risk = document.getElementById("backlog-risk").value;
    const notes = document.getElementById("backlog-notes").value.trim() || null;
    const priority = document.getElementById("backlog-priority").value;

    let url, method, payload;
    if (isEdit) {
        const item = cache.backlog.find(x => x.id === editingBacklogId);
        url = `/api/backlog/${editingBacklogId}`;
        method = "PUT";
        payload = {
            title: title,
            discovered_while: discovered,
            current_pain: pain,
            suggested_improvement: improvement,
            risk: risk,
            priority: priority,
            status: item ? item.status : "proposed",
            notes: notes
        };
    } else {
        url = "/api/backlog";
        method = "POST";
        payload = {
            title: title,
            discovered_while: discovered,
            current_pain: pain,
            suggested_improvement: improvement,
            risk: risk,
            notes: notes,
            priority: priority
        };
    }

    try {
        const res = await fetch(url, {
            method: method,
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload)
        });

        if (res.ok) {
            closeModal("backlog-modal");
            loadData();
        } else {
            alert(isEdit ? "Lỗi khi cập nhật đề xuất backlog." : "Lỗi khi gửi đề xuất backlog.");
        }
    } catch (e) {
        console.error(e);
    }
}

async function submitDecision(event) {
    event.preventDefault();
    const isEdit = editingDecisionId !== null;

    const id = document.getElementById("decision-id").value.trim();
    const title = document.getElementById("decision-title").value.trim();
    const status = document.getElementById("decision-status").value;
    const docPath = document.getElementById("decision-doc").value.trim() || null;
    const verifyCommand = document.getElementById("decision-verify").value.trim() || null;

    let url, method, payload;
    if (isEdit) {
        const item = cache.decisions.find(x => x.id === editingDecisionId);
        url = `/api/decision/${editingDecisionId}`;
        method = "PUT";
        payload = {
            title: title,
            status: status,
            doc_path: docPath,
            verify_command: verifyCommand,
            last_verified_result: item ? item.last_verified_result : null
        };
    } else {
        url = "/api/decision";
        method = "POST";
        payload = {
            id: id,
            title: title,
            status: status,
            doc_path: docPath,
            verify_command: verifyCommand
        };
    }

    try {
        const res = await fetch(url, {
            method: method,
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload)
        });

        if (res.ok) {
            closeModal("decision-modal");
            loadData();
        } else {
            alert(isEdit ? "Lỗi khi cập nhật design decision." : "Lỗi khi tạo design decision. ID có thể đã bị trùng.");
        }
    } catch (e) {
        console.error(e);
    }
}

// Modal visibility helpers
function openModal(id) {
    const modal = document.getElementById(id);
    if (modal) modal.style.display = "flex";
}

function closeModal(id) {
    const modal = document.getElementById(id);
    if (modal) modal.style.display = "none";

    // Reset editing states, headers, and form values
    if (id === "intake-modal") {
        editingIntakeId = null;
        const titleEl = document.getElementById("intake-modal-title");
        if (titleEl) titleEl.innerText = "🆕 Register Feature Intake";
        const btn = document.querySelector("#intake-modal .submit-btn");
        if (btn) btn.innerText = "Register";
        const form = document.getElementById("intake-form");
        if (form) form.reset();
    } else if (id === "story-modal") {
        editingStoryId = null;
        const titleEl = document.getElementById("story-modal-title");
        if (titleEl) titleEl.innerText = "🆕 Create User Story";
        const btn = document.querySelector("#story-modal .submit-btn");
        if (btn) btn.innerText = "Create Story";
        const idInput = document.getElementById("story-id");
        if (idInput) {
            idInput.disabled = false;
        }
        const form = document.getElementById("story-form");
        if (form) form.reset();
    } else if (id === "trace-modal") {
        editingTraceId = null;
        const titleEl = document.getElementById("trace-modal-title");
        if (titleEl) titleEl.innerText = "🆕 Record Agent Trace";
        const btn = document.querySelector("#trace-modal .submit-btn");
        if (btn) btn.innerText = "Record Trace";
        const form = document.getElementById("trace-form");
        if (form) form.reset();
    } else if (id === "backlog-modal") {
        editingBacklogId = null;
        const titleEl = document.getElementById("backlog-modal-title");
        if (titleEl) titleEl.innerText = "🆕 Add Backlog Improvement Proposal";
        const btn = document.querySelector("#backlog-modal .submit-btn");
        if (btn) btn.innerText = "Submit Proposal";
        const form = document.getElementById("backlog-form");
        if (form) form.reset();
    } else if (id === "decision-modal") {
        editingDecisionId = null;
        const titleEl = document.getElementById("decision-modal-title");
        if (titleEl) titleEl.innerText = "🆕 Create Design Decision";
        const btn = document.querySelector("#decision-modal .submit-btn");
        if (btn) btn.innerText = "Record Decision";
        const idInput = document.getElementById("decision-id");
        if (idInput) {
            idInput.disabled = false;
        }
        const form = document.getElementById("decision-form");
        if (form) form.reset();
    }
}

// Global Keyboard Shortcuts (e.g. Cmd+K search)
function handleGlobalKeys(e) {
    if ((e.metaKey || e.ctrlKey) && e.key === "k") {
        e.preventDefault();
        showSearchAlert();
    }
    if (e.key === "Escape") {
        closeSearchAlert();
    }
}

// Search interface overlay
function showSearchAlert() {
    const overlay = document.getElementById("search-alert");
    if (overlay) {
        overlay.style.display = "flex";
        const box = document.getElementById("search-input-box");
        if (box) {
            box.value = "";
            box.focus();
        }
        runSearch("");
    }
}

function closeSearchAlert() {
    const overlay = document.getElementById("search-alert");
    if (overlay) overlay.style.display = "none";
}

function runSearch(query) {
    const list = document.getElementById("search-results-list");
    if (!list) return;
    list.innerHTML = "";

    const cleanQuery = query.trim().toLowerCase();
    if (!cleanQuery) {
        list.innerHTML = `<p class="search-hint">Type to search stories, traces, intakes, decisions or backlog items...</p>`;
        return;
    }

    // Check if input is a command
    if (cleanQuery.startsWith("/")) {
        const commands = [
            { cmd: "/goto dashboard", desc: "Chuyển sang màn hình Dashboard", action: () => switchTab("dashboard") },
            { cmd: "/goto intakes", desc: "Chuyển sang màn hình Intakes Backlog", action: () => switchTab("intakes") },
            { cmd: "/goto matrix", desc: "Chuyển sang màn hình Stories & Proofs", action: () => switchTab("matrix") },
            { cmd: "/goto decisions", desc: "Chuyển sang màn hình Design Decisions (ADRs)", action: () => switchTab("decisions") },
            { cmd: "/goto graph", desc: "Chuyển sang màn hình Traceability Graph", action: () => switchTab("graph") },
            { cmd: "/goto traces", desc: "Chuyển sang màn hình Agent Traces", action: () => switchTab("traces") },
            { cmd: "/goto agents", desc: "Chuyển sang màn hình Agent Directory", action: () => switchTab("agents") },
            { cmd: "/goto interventions", desc: "Chuyển sang màn hình Interventions & Backlog", action: () => switchTab("interventions") },
            { cmd: "/goto sql", desc: "Chuyển sang màn hình SQL Console", action: () => switchTab("sql") },
            { cmd: "/create intake", desc: "Mở form đăng ký Feature Intake mới", action: () => openModal("intake-modal") },
            { cmd: "/create story", desc: "Mở form đăng ký User Story mới", action: () => openModal("story-modal") },
            { cmd: "/create decision", desc: "Mở form đăng ký ADR Decision mới", action: () => openModal("decision-modal") },
            { cmd: "/create backlog", desc: "Mở form đề xuất cải tiến Backlog mới", action: () => openModal("backlog-modal") },
        ];

        // Search decisions for verification commands
        cache.decisions.forEach(d => {
            if (d.verify_command) {
                commands.push({
                    cmd: `/verify ${d.id}`,
                    desc: `Chạy lệnh kiểm chứng cho ADR-${d.id} (${d.title})`,
                    action: () => verifyDecision(d.id)
                });
            }
        });

        // Search active HIL interventions
        cache.interventions.forEach(i => {
            const isPending = !i.description.includes("[APPROVED]") && !i.description.includes("[REJECTED]") && !i.description.includes("[APPROVE]") && !i.description.includes("[REJECT]");
            if (isPending) {
                commands.push({
                    cmd: `/approve ${i.id}`,
                    desc: `Phê duyệt can thiệp #${i.id} (${i.description.substring(0, 30)}...)`,
                    action: () => resolveIntervention(i.id, 'approve')
                });
                commands.push({
                    cmd: `/reject ${i.id}`,
                    desc: `Từ chối can thiệp #${i.id} (${i.description.substring(0, 30)}...)`,
                    action: () => resolveIntervention(i.id, 'reject')
                });
            }
        });

        const filtered = commands.filter(c => c.cmd.toLowerCase().includes(cleanQuery));
        if (filtered.length === 0) {
            list.innerHTML = `<p class="search-hint">Không tìm thấy lệnh nào khớp với "${query}"</p>`;
            return;
        }

        filtered.slice(0, 10).forEach(c => {
            const row = document.createElement("div");
            row.className = "search-row-item command";
            row.onclick = () => {
                closeSearchAlert();
                c.action();
            };
            row.innerHTML = `
                <div>
                    <div class="search-item-title" style="color: var(--primary-color); font-family: monospace;">${c.cmd}</div>
                    <div class="search-item-desc">${c.desc}</div>
                </div>
                <span class="search-item-type" style="background: rgba(59, 130, 246, 0.15); color: var(--primary-color);">Lệnh</span>
            `;
            list.appendChild(row);
        });
        return;
    }

    const matches = [];

    // Search Stories
    cache.stories.forEach(s => {
        if (s.id.toLowerCase().includes(cleanQuery) || s.title.toLowerCase().includes(cleanQuery)) {
            matches.push({ type: "Story", title: s.id, desc: s.title, tab: "matrix" });
        }
    });

    // Search Decisions
    cache.decisions.forEach(d => {
        if (d.id.toLowerCase().includes(cleanQuery) || d.title.toLowerCase().includes(cleanQuery)) {
            matches.push({ type: "Decision", title: `ADR-${d.id}`, desc: d.title, tab: "decisions" });
        }
    });

    // Search Traces
    cache.traces.forEach(t => {
        if (t.task_summary.toLowerCase().includes(cleanQuery) || (t.agent && t.agent.toLowerCase().includes(cleanQuery))) {
            matches.push({ type: "Trace", title: t.agent || "Agent Trace", desc: t.task_summary, tab: "traces" });
        }
    });

    // Search Intakes
    cache.intakes.forEach(i => {
        if (i.summary.toLowerCase().includes(cleanQuery) || i.input_type.toLowerCase().includes(cleanQuery)) {
            matches.push({ type: "Intake", title: `Intake #${i.id}`, desc: i.summary, tab: "intakes" });
        }
    });

    // Search Backlog
    cache.backlog.forEach(b => {
        if (b.title.toLowerCase().includes(cleanQuery)) {
            matches.push({ type: "Backlog", title: "Backlog Proposal", desc: b.title, tab: "interventions" });
        }
    });

    if (matches.length === 0) {
        list.innerHTML = `<p class="search-hint">No matches found for "${query}"</p>`;
        return;
    }

    matches.slice(0, 10).forEach(m => {
        const row = document.createElement("div");
        row.className = "search-row-item";
        row.onclick = () => {
            closeSearchAlert();
            switchTab(m.tab);
        };
        row.innerHTML = `
            <div>
                <div class="search-item-title">${m.title}</div>
                <div class="search-item-desc">${m.desc}</div>
            </div>
            <span class="search-item-type">${m.type}</span>
        `;
        list.appendChild(row);
    });
}

// Utility styling functions
function formatShortTime(isoStr) {
    if (!isoStr) return "";
    try {
        const d = new Date(isoStr.replace(" ", "T")); // Handle sqlite spaces
        if (isNaN(d.getTime())) return isoStr;
        return d.toLocaleString("vi-VN", {
            hour: "2-digit",
            minute: "2-digit",
            day: "2-digit",
            month: "2-digit",
            year: "numeric"
        });
    } catch (e) {
        return isoStr;
    }
}

function getAgentClass(agent) {
    if (!agent) return "default-bg";
    const name = agent.toLowerCase();
    if (name.includes("pm")) return "pm-bg";
    if (name.includes("ba")) return "ba-bg";
    if (name.includes("fe") || name.includes("frontend")) return "fe-bg";
    if (name.includes("be") || name.includes("backend")) return "be-bg";
    if (name.includes("qa") || name.includes("quality")) return "qa-bg";
    return "default-bg";
}

function getOutcomeClass(outcome) {
    if (!outcome) return "badge-planned";
    const name = outcome.toLowerCase();
    if (name === "completed") return "proof-pass";
    if (name === "failed") return "proof-fail";
    if (name === "blocked") return "badge-in_progress";
    return "badge-changed";
}

// ==========================================
// CRUD Helper Functions
// ==========================================

// DELETE helpers
async function deleteIntake(id) {
    if (!confirm(`Bạn có chắc chắn muốn xoá Feature Intake #${id}?`)) return;
    try {
        const res = await fetch(`/api/intake/${id}`, { method: "DELETE" });
        if (res.ok) {
            loadData();
        } else {
            alert("Lỗi khi xoá Feature Intake.");
        }
    } catch (e) {
        console.error(e);
    }
}

async function deleteStory(id) {
    if (!confirm(`Bạn có chắc chắn muốn xoá User Story ${id}?`)) return;
    try {
        const res = await fetch(`/api/story/${id}`, { method: "DELETE" });
        if (res.ok) {
            loadData();
        } else {
            alert("Lỗi khi xoá User Story.");
        }
    } catch (e) {
        console.error(e);
    }
}

async function deleteDecision(id) {
    if (!confirm(`Bạn có chắc chắn muốn xoá Design Decision ADR-${id}?`)) return;
    try {
        const res = await fetch(`/api/decision/${id}`, { method: "DELETE" });
        if (res.ok) {
            loadData();
        } else {
            alert("Lỗi khi xoá Design Decision.");
        }
    } catch (e) {
        console.error(e);
    }
}

async function deleteTrace(id) {
    if (!confirm(`Bạn có chắc chắn muốn xoá Agent Trace #${id}?`)) return;
    try {
        const res = await fetch(`/api/trace/${id}`, { method: "DELETE" });
        if (res.ok) {
            loadData();
        } else {
            alert("Lỗi khi xoá Agent Trace.");
        }
    } catch (e) {
        console.error(e);
    }
}

async function deleteBacklog(id) {
    if (!confirm(`Bạn có chắc chắn muốn xoá Backlog Proposal #${id}?`)) return;
    try {
        const res = await fetch(`/api/backlog/${id}`, { method: "DELETE" });
        if (res.ok) {
            loadData();
        } else {
            alert("Lỗi khi xoá Backlog Proposal.");
        }
    } catch (e) {
        console.error(e);
    }
}

// EDIT starters (Form populator & opener)
function startEditIntake(id) {
    const item = cache.intakes.find(x => x.id === id);
    if (!item) return;

    editingIntakeId = id;
    document.getElementById("intake-modal-title").innerText = `✏️ Sửa Feature Intake #${id}`;
    const btn = document.querySelector("#intake-modal .submit-btn");
    if (btn) btn.innerText = "Lưu thay đổi";

    document.getElementById("intake-type").value = item.input_type;
    document.getElementById("intake-summary").value = item.summary;
    document.getElementById("intake-lane").value = item.risk_lane;
    document.getElementById("intake-notes").value = item.notes || "";

    openModal("intake-modal");
}

function startEditStory(id) {
    const item = cache.stories.find(x => x.id === id);
    if (!item) return;

    editingStoryId = id;
    document.getElementById("story-modal-title").innerText = `✏️ Sửa User Story ${id}`;
    const btn = document.querySelector("#story-modal .submit-btn");
    if (btn) btn.innerText = "Lưu thay đổi";

    const idInput = document.getElementById("story-id");
    if (idInput) {
        idInput.value = item.id;
        idInput.disabled = true;
    }

    document.getElementById("story-title").value = item.title;
    document.getElementById("story-lane").value = item.risk_lane;
    document.getElementById("story-priority").value = item.priority;

    openModal("story-modal");
}

function startEditDecision(id) {
    const item = cache.decisions.find(x => x.id === id);
    if (!item) return;

    editingDecisionId = id;
    document.getElementById("decision-modal-title").innerText = `✏️ Sửa Design Decision ADR-${id}`;
    const btn = document.querySelector("#decision-modal .submit-btn");
    if (btn) btn.innerText = "Lưu thay đổi";

    const idInput = document.getElementById("decision-id");
    if (idInput) {
        idInput.value = item.id;
        idInput.disabled = true;
    }

    document.getElementById("decision-title").value = item.title;
    document.getElementById("decision-status").value = item.status;
    document.getElementById("decision-doc").value = item.doc_path || "";
    document.getElementById("decision-verify").value = item.verify_command || "";

    openModal("decision-modal");
}

function startEditTrace(id) {
    const item = cache.traces.find(x => x.id === id);
    if (!item) return;

    editingTraceId = id;
    document.getElementById("trace-modal-title").innerText = `✏️ Sửa Agent Trace #${id}`;
    const btn = document.querySelector("#trace-modal .submit-btn");
    if (btn) btn.innerText = "Lưu thay đổi";

    document.getElementById("trace-summary").value = item.task_summary;
    document.getElementById("trace-story-id").value = item.story_id || "";
    document.getElementById("trace-agent").value = item.agent || "";
    document.getElementById("trace-outcome").value = item.outcome;

    openModal("trace-modal");
}

function startEditBacklog(id) {
    const item = cache.backlog.find(x => x.id === id);
    if (!item) return;

    editingBacklogId = id;
    document.getElementById("backlog-modal-title").innerText = `✏️ Sửa Backlog Proposal #${id}`;
    const btn = document.querySelector("#backlog-modal .submit-btn");
    if (btn) btn.innerText = "Lưu thay đổi";

    document.getElementById("backlog-title").value = item.title;
    document.getElementById("backlog-discovered").value = item.discovered_while || "";
    document.getElementById("backlog-pain").value = item.current_pain || "";
    document.getElementById("backlog-improvement").value = item.suggested_improvement || "";
    document.getElementById("backlog-risk").value = item.risk || "tiny";
    document.getElementById("backlog-priority").value = item.priority;
    document.getElementById("backlog-notes").value = item.notes || "";

    openModal("backlog-modal");
}

// ==========================================
// Advanced Features Helpers
// ==========================================

// 1. WebSocket Agent Console Log Stream
let ws;
function initTerminalWS() {
    const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
    const wsUrl = `${protocol}//${window.location.host}/api/ws`;
    
    ws = new WebSocket(wsUrl);
    const statusEl = document.getElementById("ws-status");
    
    ws.onopen = () => {
        if (statusEl) {
            statusEl.innerText = "Online";
            statusEl.style.color = "var(--success-color)";
        }
        appendTerminalLog("[SYSTEM] Connected to live agent telemetry stream.", "system");
    };

    ws.onmessage = (event) => {
        appendTerminalLog(event.data, "agent");
        updateAgentWorkloads(event.data);
    };

    ws.onclose = () => {
        if (statusEl) {
            statusEl.innerText = "Disconnected";
            statusEl.style.color = "var(--danger-color)";
        }
        appendTerminalLog("[SYSTEM] Connection lost. Reconnecting in 5s...", "system");
        setTimeout(initTerminalWS, 5000);
    };

    ws.onerror = (err) => {
        console.error("WebSocket error:", err);
    };
}

function appendTerminalLog(text, type) {
    const container = document.getElementById("terminal-logs-body");
    if (!container) return;

    const line = document.createElement("div");
    line.className = `log-line ${type}`;
    
    const time = new Date().toLocaleTimeString("vi-VN", { hour12: false });
    line.innerText = `[${time}] ${text}`;
    
    container.appendChild(line);

    // Limit to 50 logs
    while (container.childNodes.length > 50) {
        container.removeChild(container.firstChild);
    }

    // Scroll to bottom
    const contentEl = document.querySelector(".terminal-content");
    if (contentEl) {
        contentEl.scrollTop = contentEl.scrollHeight;
    }
}

function toggleTerminalDrawer() {
    const drawer = document.getElementById("terminal-drawer");
    if (drawer) {
        drawer.classList.toggle("collapsed");
    }
}

// 2. Decision Auto-Verification Runner
async function verifyDecision(id) {
    openModal("verify-modal");
    const subtitle = document.getElementById("verify-modal-subtitle");
    const consoleEl = document.getElementById("verify-log-console");
    
    if (subtitle) subtitle.innerText = `Running verification checks for ADR-${id}...`;
    if (consoleEl) {
        consoleEl.innerHTML = `<pre class="terminal-text" style="color:#f59e0b">$ Running check command for ADR-${id}...\n[LOADING] Executing verification command on backend...</pre>`;
    }

    try {
        const res = await fetch(`/api/decision/${id}/verify`, { method: "POST" });
        const data = await res.json();
        
        if (subtitle) {
            subtitle.innerText = data.success 
                ? `✅ Verification PASSED for ADR-${id}` 
                : `❌ Verification FAILED for ADR-${id}`;
        }

        if (consoleEl) {
            const color = data.success ? "#34d399" : "#f87171";
            consoleEl.innerHTML = `<pre class="terminal-text" style="color: ${color}; font-family: var(--font-mono); font-size: 0.82rem; margin: 0; white-space: pre-wrap; word-break: break-all;">${escapeHtml(data.log)}</pre>`;
        }
        
        // Reload stats and decisions list
        loadData();
    } catch (e) {
        console.error(e);
        if (subtitle) subtitle.innerText = `Lỗi hệ thống khi verify ADR-${id}`;
        if (consoleEl) consoleEl.innerHTML = `<pre class="terminal-text" style="color:#ef4444">Lỗi kết nối tới endpoint verify.</pre>`;
    }
}

function escapeHtml(text) {
    if (!text) return "";
    return text
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/"/g, "&quot;")
        .replace(/'/g, "&#039;");
}

// 3. Human-in-the-Loop resolution
async function resolveIntervention(id, action) {
    if (!confirm(`Bạn có chắc chắn muốn ${action === 'approve' ? 'Phê duyệt' : 'Từ chối'} can thiệp này?`)) return;
    try {
        const res = await fetch(`/api/intervention/${id}/resolve`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ action })
        });

        if (res.ok) {
            loadData();
        } else {
            alert("Lỗi khi xử lý can thiệp.");
        }
    } catch (e) {
        console.error(e);
    }
}

// 4. Interactive SVG Traceability Dependency Graph
function renderDependencyGraph() {
    const svg = document.getElementById("dependency-svg");
    if (!svg) return;
    svg.innerHTML = "";

    // Define Arrow Marker
    const defs = document.createElementNS("http://www.w3.org/2000/svg", "defs");
    defs.innerHTML = `
        <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
            <path d="M 0 1 L 10 5 L 0 9 z" fill="var(--text-muted)"></path>
        </marker>
        <marker id="arrow-active" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
            <path d="M 0 1 L 10 5 L 0 9 z" fill="var(--primary-color)"></path>
        </marker>
    `;
    svg.appendChild(defs);

    const intakes = cache.intakes;
    const stories = cache.stories;
    const decisions = cache.decisions;

    // Node maps for coords
    const nodes = {};
    
    // Y-positions spacing
    const intakeYStep = 100;
    const storyYStep = 100;
    const decisionYStep = 90;

    // Draw Intakes (Column 1, X = 100)
    intakes.forEach((item, i) => {
        nodes[`intake-${item.id}`] = {
            id: `intake-${item.id}`,
            label: `Intake #${item.id}`,
            sub: item.input_type,
            x: 100,
            y: 50 + i * intakeYStep,
            color: "var(--warning-color)",
            linkedStory: item.story_id
        };
    });

    // Draw Stories (Column 2, X = 400)
    stories.forEach((item, i) => {
        nodes[`story-${item.id}`] = {
            id: `story-${item.id}`,
            label: item.id,
            sub: item.title.substring(0, 15) + "...",
            x: 400,
            y: 50 + i * storyYStep,
            color: item.status === "implemented" ? "var(--success-color)" : "var(--primary-color)"
        };
    });

    // Extract Story -> Decision links from Traces
    const storyToDecisions = {};
    cache.traces.forEach(t => {
        if (t.story_id && t.decisions_made) {
            try {
                const decs = typeof t.decisions_made === 'string' ? JSON.parse(t.decisions_made) : t.decisions_made;
                if (Array.isArray(decs)) {
                    if (!storyToDecisions[t.story_id]) {
                        storyToDecisions[t.story_id] = new Set();
                    }
                    decs.forEach(d => storyToDecisions[t.story_id].add(d));
                }
            } catch(e) {}
        }
    });

    // Draw Decisions (Column 3, X = 700)
    decisions.forEach((item, i) => {
        nodes[`decision-${item.id}`] = {
            id: `decision-${item.id}`,
            label: `ADR-${item.id}`,
            sub: item.title.substring(0, 15) + "...",
            x: 700,
            y: 50 + i * decisionYStep,
            color: item.status === "accepted" ? "var(--success-color)" : "var(--warning-color)"
        };
    });

    // Build Links array
    const links = [];

    // Link: Intake -> Story
    Object.values(nodes).forEach(n => {
        if (n.id.startsWith("intake-") && n.linkedStory) {
            const targetStoryId = `story-${n.linkedStory}`;
            if (nodes[targetStoryId]) {
                links.push({
                    id: `link-${n.id}-${targetStoryId}`,
                    source: n.id,
                    target: targetStoryId
                });
            }
        }
    });

    // Link: Story -> Decision
    stories.forEach(s => {
        const linkedDecs = storyToDecisions[s.id];
        if (linkedDecs) {
            linkedDecs.forEach(dId => {
                const targetDecId = `decision-${dId}`;
                if (nodes[targetDecId]) {
                    links.push({
                        id: `link-story-${s.id}-${targetDecId}`,
                        source: `story-${s.id}`,
                        target: targetDecId
                    });
                }
            });
        }
    });

    // Draw Link paths
    links.forEach(l => {
        const src = nodes[l.source];
        const tgt = nodes[l.target];
        if (!src || !tgt) return;

        const path = document.createElementNS("http://www.w3.org/2000/svg", "path");
        path.setAttribute("id", l.id);
        path.setAttribute("class", "graph-edge");
        
        // Bezier curve layout
        const dx = tgt.x - src.x;
        const curveness = dx * 0.4;
        const d = `M ${src.x} ${src.y} C ${src.x + curveness} ${src.y}, ${tgt.x - curveness} ${tgt.y}, ${tgt.x} ${tgt.y}`;
        
        path.setAttribute("d", d);
        path.setAttribute("marker-end", "url(#arrow)");
        svg.appendChild(path);
    });

    // Draw Node elements
    Object.values(nodes).forEach(n => {
        const g = document.createElementNS("http://www.w3.org/2000/svg", "g");
        g.setAttribute("class", "graph-node");
        g.setAttribute("id", n.id);

        // Circle
        const circle = document.createElementNS("http://www.w3.org/2000/svg", "circle");
        circle.setAttribute("cx", n.x);
        circle.setAttribute("cy", n.y);
        circle.setAttribute("r", "8");
        circle.setAttribute("fill", "var(--bg-card)");
        circle.setAttribute("stroke", n.color);
        g.appendChild(circle);

        // Title Label
        const textLabel = document.createElementNS("http://www.w3.org/2000/svg", "text");
        textLabel.setAttribute("x", n.x + 16);
        textLabel.setAttribute("y", n.y - 2);
        textLabel.textContent = n.label;
        g.appendChild(textLabel);

        // Sub Label
        const textSub = document.createElementNS("http://www.w3.org/2000/svg", "text");
        textSub.setAttribute("x", n.x + 16);
        textSub.setAttribute("y", n.y + 10);
        textSub.setAttribute("class", "node-type");
        textSub.textContent = n.sub;
        g.appendChild(textSub);

        // Hover Interactive behaviors
        g.addEventListener("mouseenter", () => highlightDependencies(n.id, links));
        g.addEventListener("mouseleave", () => resetHighlight());

        svg.appendChild(g);
    });
}

function highlightDependencies(nodeId, links) {
    const activeLinks = new Set();
    const activeNodes = new Set();
    activeNodes.add(nodeId);

    links.forEach(l => {
        if (l.source === nodeId || l.target === nodeId) {
            activeLinks.add(l.id);
            activeNodes.add(l.source);
            activeNodes.add(l.target);
        }
    });

    activeLinks.forEach(id => {
        const el = document.getElementById(id);
        if (el) {
            el.classList.add("active");
            el.setAttribute("marker-end", "url(#arrow-active)");
        }
    });

    activeNodes.forEach(id => {
        const el = document.getElementById(id);
        if (el) el.classList.add("active");
    });
}

function resetHighlight() {
    document.querySelectorAll(".graph-edge").forEach(el => {
        el.classList.remove("active");
        el.setAttribute("marker-end", "url(#arrow)");
    });
    document.querySelectorAll(".graph-node").forEach(el => {
        el.classList.remove("active");
    });
}

// --- SQL Console query execution ---
async function executeSQLQuery() {
    const queryInput = document.getElementById("sql-query-input");
    const container = document.getElementById("sql-results-container");
    const errorBox = document.getElementById("sql-console-error");
    
    if (!queryInput || !container || !errorBox) return;
    
    const query = queryInput.value.trim();
    if (!query) {
        alert("Vui lòng nhập câu lệnh SQL.");
        return;
    }
    
    container.innerHTML = `<p class="loading-cell">Đang thực thi câu lệnh SQL...</p>`;
    errorBox.style.display = "none";
    
    try {
        const res = await fetch("/api/query", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ query })
        });
        
        if (res.ok) {
            const data = await res.json();
            if (data.columns.length === 0) {
                container.innerHTML = `<p style="color: var(--success-color); margin: 0;">Truy vấn thành công. Trả về 0 cột.</p>`;
                return;
            }
            
            let html = `<table class="sql-results-table"><thead><tr>`;
            data.columns.forEach(col => {
                html += `<th>${col}</th>`;
            });
            html += `</tr></thead><tbody>`;
            
            if (data.rows.length === 0) {
                html += `<tr><td colspan="${data.columns.length}" style="text-align: center; color: var(--text-muted);">Không có kết quả trùng khớp.</td></tr>`;
            } else {
                data.rows.forEach(row => {
                    html += `<tr>`;
                    row.forEach(val => {
                        html += `<td>${val === null ? '<span style="color: var(--text-muted);">NULL</span>' : (typeof val === 'object' ? JSON.stringify(val) : val)}</td>`;
                    });
                    html += `</tr>`;
                });
            }
            html += `</tbody></table>`;
            container.innerHTML = html;
        } else {
            const errText = await res.text();
            errorBox.innerText = `Lỗi thực thi: ${errText}`;
            errorBox.style.display = "block";
            container.innerHTML = `<p class="search-hint" style="color: var(--danger-color);">Truy vấn thất bại.</p>`;
        }
    } catch (e) {
        console.error(e);
        errorBox.innerText = `Lỗi kết nối: ${e.message}`;
        errorBox.style.display = "block";
        container.innerHTML = `<p class="search-hint" style="color: var(--danger-color);">Truy vấn thất bại.</p>`;
    }
}

function clearSQLConsole() {
    const queryInput = document.getElementById("sql-query-input");
    const container = document.getElementById("sql-results-container");
    const errorBox = document.getElementById("sql-console-error");
    if (queryInput) queryInput.value = "";
    if (errorBox) errorBox.style.display = "none";
    if (container) {
        container.innerHTML = `<p class="search-hint" style="margin: 0; text-align: center; color: var(--text-muted);">Nhập câu lệnh SELECT và nhấn Run Query để xem kết quả.</p>`;
    }
}

// --- Friction Analytics Render ---
function renderFrictionAnalytics() {
    const barsContainer = document.getElementById("friction-bars-container");
    const logsBody = document.getElementById("friction-logs-body");
    if (!barsContainer || !logsBody) return;
    
    const frictionCounts = {
        "PM Agent": 0,
        "BA Agent": 0,
        "Frontend Agent": 0,
        "Backend Agent": 0,
        "QA Agent": 0,
        "Auditor": 0
    };
    
    let totalFriction = 0;
    const frictionTraces = [];
    
    cache.traces.forEach(t => {
        if (t.harness_friction) {
            frictionTraces.push(t);
            const agentKey = t.agent || "Other";
            frictionCounts[agentKey] = (frictionCounts[agentKey] || 0) + 1;
            totalFriction++;
        }
    });
    
    barsContainer.innerHTML = "";
    Object.keys(frictionCounts).forEach(agent => {
        const count = frictionCounts[agent];
        const pct = totalFriction > 0 ? (count / totalFriction) * 100 : 0;
        
        const barItem = document.createElement("div");
        barItem.className = "friction-bar-item";
        barItem.innerHTML = `
            <div class="friction-bar-label">
                <span>${agent}</span>
                <span class="friction-count">${count} frictions</span>
            </div>
            <div class="friction-bar-track">
                <div class="friction-bar-fill" style="width: ${pct}%;"></div>
            </div>
        `;
        barsContainer.appendChild(barItem);
    });
    
    logsBody.innerHTML = "";
    if (frictionTraces.length === 0) {
        logsBody.innerHTML = `<tr><td colspan="3" style="padding: 1rem; text-align: center; color: var(--text-muted);">Không phát hiện ma sát nào trong traces.</td></tr>`;
    } else {
        frictionTraces.forEach(t => {
            const tr = document.createElement("tr");
            tr.style.borderBottom = "1px solid var(--border-color)";
            tr.innerHTML = `
                <td style="padding: 0.6rem 0.8rem; font-weight: 600; color: var(--warning-color);">${t.agent || "Unknown"}</td>
                <td style="padding: 0.6rem 0.8rem; font-family: monospace;">${t.story_id || "Global"}</td>
                <td style="padding: 0.6rem 0.8rem; color: var(--text-muted);">${t.harness_friction}</td>
            `;
            logsBody.appendChild(tr);
        });
    }
}

// --- Drag and Drop backlog matrix ---
function dragBacklog(ev, id) {
    ev.dataTransfer.setData("text", id);
    const card = ev.target;
    setTimeout(() => card.style.opacity = "0.5", 0);
}

document.addEventListener("dragend", (e) => {
    if (e.target.className === "kanban-card") {
        e.target.style.opacity = "1";
    }
    document.querySelectorAll(".kanban-column").forEach(col => col.classList.remove("dragover"));
});

function allowDrop(ev) {
    ev.preventDefault();
    const col = ev.currentTarget;
    if (col) col.classList.add("dragover");
}

document.addEventListener("dragleave", (e) => {
    if (e.target.className && e.target.className.includes("kanban-column")) {
        e.target.classList.remove("dragover");
    }
});

async function dropBacklog(ev, targetPriority) {
    ev.preventDefault();
    const id = ev.dataTransfer.getData("text");
    const item = cache.backlog.find(b => b.id == id);
    if (!item) return;

    if (item.priority === targetPriority) return;

    const payload = {
        title: item.title,
        discovered_while: item.discovered_while,
        current_pain: item.current_pain,
        suggested_improvement: item.suggested_improvement,
        risk: item.risk,
        priority: targetPriority,
        status: item.status,
        notes: item.notes
    };

    try {
        const res = await fetch(`/api/backlog/${id}`, {
            method: "PUT",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload)
        });
        if (res.ok) {
            loadData();
        } else {
            alert("Lỗi khi chuyển đổi mức độ ưu tiên backlog.");
            loadData();
        }
    } catch (e) {
        console.error(e);
        loadData();
    }
}

// --- WebSocket active workloads updater ---
function updateAgentWorkloads(logText) {
    const agents = {
        "pm": document.getElementById("agent-status-pm"),
        "ba": document.getElementById("agent-status-ba"),
        "fe": document.getElementById("agent-status-fe"),
        "be": document.getElementById("agent-status-be"),
        "qa": document.getElementById("agent-status-qa"),
        "aud": document.getElementById("agent-status-aud")
    };

    // Reset all to idle
    Object.keys(agents).forEach(k => {
        if (agents[k]) {
            agents[k].className = "agent-status-badge idle";
            agents[k].innerHTML = `<span class="status-dot"></span>Idle`;
        }
    });

    let activeKey = null;
    if (logText.includes("PM Agent:")) activeKey = "pm";
    else if (logText.includes("BA Agent:")) activeKey = "ba";
    else if (logText.includes("FE Agent:")) activeKey = "fe";
    else if (logText.includes("BE Agent:")) activeKey = "be";
    else if (logText.includes("QA Agent:")) activeKey = "qa";
    else if (logText.includes("Auditor:")) activeKey = "aud";

    if (activeKey && agents[activeKey]) {
        agents[activeKey].className = "agent-status-badge running";
        agents[activeKey].innerHTML = `<span class="status-dot"></span>Running`;
        
        const workInfoEl = document.getElementById(`agent-work-${activeKey}`);
        if (workInfoEl) {
            workInfoEl.innerText = logText.substring(logText.indexOf(":") + 1).trim();
        }
    }
}

// --- Poll real agent status from /api/agent-status ---
async function pollAgentStatus() {
    try {
        const res = await fetch("/api/agent-status");
        if (!res.ok) return;
        const data = await res.json();
        const agents = {
            "pm": document.getElementById("agent-status-pm"),
            "ba": document.getElementById("agent-status-ba"),
            "fe": document.getElementById("agent-status-fe"),
            "be": document.getElementById("agent-status-be"),
            "qa": document.getElementById("agent-status-qa"),
            "aud": document.getElementById("agent-status-aud")
        };
        
        data.agents.forEach(a => {
            const key = a.agent.toLowerCase();
            const el = agents[key];
            if (!el) return;
            
            const isRunning = a.status === "working" || a.status === "assigned";
            el.className = `agent-status-badge ${isRunning ? "running" : "idle"}`;
            el.innerHTML = `<span class="status-dot"></span>${isRunning ? "Running" : "Idle"}`;
            
            const workEl = document.getElementById(`agent-work-${key}`);
            if (workEl && a.current_task) {
                workEl.innerText = `Task: ${a.current_task}`;
            }
        });
    } catch (e) {
        console.debug("Agent status poll failed:", e);
    }
}

// Start polling every 5 seconds
setInterval(pollAgentStatus, 5000);
pollAgentStatus(); // initial
