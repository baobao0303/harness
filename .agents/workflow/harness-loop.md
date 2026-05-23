# Harness Unified Master Development Loop (HML)

**Goal:** Orchestrate a highly disciplined, end-to-end agile software development lifecycle (intake, design, breakdown, development, verification, retrospective) using a standardized **Step-File Parent-Child Architecture**.

---

## 🗺️ Progressive Disclosure Map

Instead of bloating the agent's active memory with the entire development guidelines, this workflow uses **Just-In-Time step loading**. Only the active sub-workflow file is loaded into context.

```text
               +--------------------------------------+
               |  harness-loop.md (Master Parent)     |
               +--------------------------------------+
                                   |
         +-------------------------+-------------------------+
         |                                                   |
         v                                                   v
+-----------------------------+                     +-----------------------------+
| step-01-intake.md           |                     | step-04-dev.md              |
| - Risk classification       |                     | - Code implementation       |
| - SQLite Intake record      |                     | - Lane constraints          |
+-----------------------------+                     +-----------------------------+
         |                                                   |
         v                                                   v
+-----------------------------+                     +-----------------------------+
| step-02-architecture.md     |                     | step-05-review.md           |
| - Boundary design           |                     | - Cynical review            |
| - Decision records (ADR)    |                     | - DB Story matrix updates   |
+-----------------------------+                     +-----------------------------+
         |                                                   |
         v                                                   v
+-----------------------------+                     +-----------------------------+
| step-03-stories.md          |                     | step-06-retro.md            |
| - Story packets creation    |                     | - SQLite Trace logging      |
| - harness story add         |                     | - Backlog friction capture  |
+-----------------------------+                     +-----------------------------+
```

---

## 🚨 Execution Constraints (Strict Enforcement)

* ⛔ **NEVER load multiple step files simultaneously**. Load only the active step file under `.agents/workflow/steps/`.
* ⛔ **ALWAYS execute the steps in exact sequential order**. Do not skip stages or jump directly to coding.
* ⛔ **ALWAYS halt at step checkpoints** and prompt the human for confirmation before advancing.
* ⛔ **ALWAYS maintain documentation sync** (matrix, stories, ADRs) alongside code.

---

## 🏃 Master Loop Execution

### START HERE

Load and read **Step 1** to begin the master development loop:
👉 **[step-01-intake.md](file:///Users/bao312/Desktop/harness/.agents/workflow/steps/step-01-intake.md)**
