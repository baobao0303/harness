// Harness Dashboard Application Script

document.addEventListener("DOMContentLoaded", () => {
    loadData();
});

// Load stats and test matrix
async function loadData() {
    try {
        await Promise.all([loadStats(), loadMatrix()]);
        updateDbStatus(true, "Kết nối Database thành công");
    } catch (error) {
        console.error("Lỗi khi tải dữ liệu:", error);
        updateDbStatus(false, "Không thể kết nối Database");
    }
}

// Update Database status indicator
function updateDbStatus(success, message) {
    const statusText = document.getElementById("db-status");
    const pulseDot = document.querySelector(".pulse-dot");
    
    if (statusText) statusText.innerText = message;
    
    if (pulseDot) {
        if (success) {
            pulseDot.style.backgroundColor = "var(--pulse-green)";
        } else {
            pulseDot.style.backgroundColor = "var(--danger-color)";
        }
    }
}

// Fetch stats counts
async function loadStats() {
    const response = await fetch("/api/stats");
    if (!response.ok) throw new Error("API stats trả về lỗi");
    
    const stats = await response.json();
    document.getElementById("count-intakes").innerText = stats.intakes;
    document.getElementById("count-stories").innerText = stats.stories;
    document.getElementById("count-decisions").innerText = stats.decisions;
    document.getElementById("count-traces").innerText = stats.traces;
}

// Fetch matrix data and render table rows
async function loadMatrix() {
    const response = await fetch("/api/matrix");
    if (!response.ok) throw new Error("API matrix trả về lỗi");
    
    const stories = await response.json();
    const matrixBody = document.getElementById("matrix-body");
    
    if (!matrixBody) return;
    matrixBody.innerHTML = "";
    
    if (stories.length === 0) {
        matrixBody.innerHTML = `<tr><td colspan="8" class="loading-cell">Không có story nào được ghi nhận.</td></tr>`;
        return;
    }
    
    stories.forEach(story => {
        const tr = document.createElement("tr");
        
        // Priority color class mapping
        const priorityClass = `p-${story.priority.toLowerCase()}`;
        
        // Proof cell helper
        const proofHtml = (val) => {
            return val > 0 
                ? `<span class="proof-tag proof-pass">PASS</span>` 
                : `<span class="proof-tag proof-fail">FAIL</span>`;
        };
        
        tr.innerHTML = `
            <td><strong>${story.id}</strong></td>
            <td>${story.title}</td>
            <td><span class="priority-tag ${priorityClass}">${story.priority}</span></td>
            <td><span class="badge badge-${story.status}">${story.status}</span></td>
            <td>${proofHtml(story.unit)}</td>
            <td>${proofHtml(story.integration)}</td>
            <td>${proofHtml(story.e2e)}</td>
            <td><span class="evidence-text">${story.evidence || "-"}</span></td>
        `;
        matrixBody.appendChild(tr);
    });
}

// Modal Toggle Actions
function openModal(id) {
    const modal = document.getElementById(id);
    if (modal) modal.style.display = "flex";
}

function closeModal(id) {
    const modal = document.getElementById(id);
    if (modal) modal.style.display = "none";
}

// Handle Form Submissions

async function submitIntake(event) {
    event.preventDefault();
    const payload = {
        input_type: document.getElementById("intake-type").value,
        summary: document.getElementById("intake-summary").value,
        risk_lane: document.getElementById("intake-lane").value,
        notes: document.getElementById("intake-notes").value || null
    };
    
    try {
        const response = await fetch("/api/intake", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload)
        });
        
        if (response.ok) {
            closeModal("intake-modal");
            document.getElementById("intake-form").reset();
            loadData();
        } else {
            alert("Lỗi khi thêm intake");
        }
    } catch (e) {
        console.error(e);
    }
}

async function submitStory(event) {
    event.preventDefault();
    const payload = {
        id: document.getElementById("story-id").value.trim(),
        title: document.getElementById("story-title").value.trim(),
        lane: document.getElementById("story-lane").value,
        priority: document.getElementById("story-priority").value
    };
    
    try {
        const response = await fetch("/api/story", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload)
        });
        
        if (response.ok) {
            closeModal("story-modal");
            document.getElementById("story-form").reset();
            loadData();
        } else {
            alert("Lỗi khi thêm story");
        }
    } catch (e) {
        console.error(e);
    }
}

async function submitTrace(event) {
    event.preventDefault();
    const payload = {
        task_summary: document.getElementById("trace-summary").value.trim(),
        story_id: document.getElementById("trace-story-id").value.trim() || null,
        agent: document.getElementById("trace-agent").value.trim() || null,
        outcome: document.getElementById("trace-outcome").value
    };
    
    try {
        const response = await fetch("/api/trace", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload)
        });
        
        if (response.ok) {
            closeModal("trace-modal");
            document.getElementById("trace-form").reset();
            loadData();
        } else {
            alert("Lỗi khi ghi nhận trace");
        }
    } catch (e) {
        console.error(e);
    }
}
