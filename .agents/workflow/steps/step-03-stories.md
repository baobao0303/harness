# Step 3: Story Decomposition & Planning

**Goal:** Break down the initiative or spec slice into clear, independent, and verifiable stories, registering them in the SQLite DB test matrix.

---

## 📅 Story Decomposition Rules

* **Single Goal Slices**: Every story should target a single cohesive user-facing or technical deliverable.
* **Acceptance Criteria (AC)**: Every story must have concrete, testable ACs (preferably using Given/When/Then formatting).
* **Story Packets**:
  * **Normal Lane**: Create or update a story packet file under `docs/stories/US-XXX-description.md` using the template at `docs/templates/story.md`.
  * **High-Risk Lane**: Create a dedicated folder `docs/stories/epics/E-XX/` with comprehensive design, execution, and validation plans.
  * **Tiny Lane**: Skip creating a story markdown file, but register it if non-trivial.

---

## 💾 Registering Stories to SQLite DB
Register each story in the database so it appears in the test matrix:
```bash
./scripts/harness story add --id "US-XXX" --title "<story_title>" --lane "<lane>"
```
*Verify registration by querying the database:*
```bash
./scripts/harness query matrix
```

---

## 🛑 CHECKPOINT
Present a summary of the sprint plan to the human:
* **Decomposed Stories**: List the Story IDs and titles (e.g. `US-005: Add caching Layer`).
* **Story Files Created**: Paths to the story markdown files.
* **DB Matrix Output**: Print the output of `./scripts/harness query matrix` showing the planned stories.

**DO NOT ADVANCE** to Step 4 until the human approves the stories and priority.
Once approved, load and execute:
👉 **[step-04-dev.md](file:///Users/bao312/Desktop/harness/.agents/workflow/steps/step-04-dev.md)**
