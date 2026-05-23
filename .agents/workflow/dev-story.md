# Harness Dev Story Workflow (DSW)

**Goal:** Safely implement fully planned, tested, and tracked user stories (NORMAL & HIGH-RISK lane tasks) with explicit SQLite matrix verification and human check-gates.

---

## 🏃 Workflow Steps

### Step 1: Pre-Development Check
1. Read the user story packet at `docs/stories/<story_id>-*.md` to fully absorb the:
   * Acceptance Criteria (AC).
   * Technical and Architectural design guidelines.
   * Expected validation proof (matrix details).
2. Verify that the story is added to the SQLite database and retrieve the current matrix state:
   ```bash
   ./scripts/harness query matrix
   ```
3. Set the story status in the database to `in_progress`:
   ```bash
   ./scripts/harness story update --id <story_id> --status "in_progress"
   ```

### Step 2: Implementation & Refactoring
1. Implement the requested feature in vertical slices.
2. Maintain high code quality, adding comprehensive test coverage for all modified blocks (unit, integration, and platform tests as expected).
3. Do not modify public contracts or architectures without explicit human confirmation.

### Step 3: Local Verification Gate
Ensure that all verification gates pass cleanly before finishing:
* **Code Formatting Check**:
  ```bash
  cp harness.toml Cargo.toml && cp harness.lock Cargo.lock && cargo fmt --check && rm Cargo.toml Cargo.lock
  ```
* **Test Suite Verification**:
  ```bash
  cp harness.toml Cargo.toml && cp harness.lock Cargo.lock && cargo test && rm Cargo.toml Cargo.lock
  ```
* **Shell Scripts Formatting**:
  ```bash
  bash -n scripts/*.sh
  ```

### Step 4: Record Proof Evidence
Update the story status in the database to `implemented` and submit concrete verification proof:
```bash
./scripts/harness story update --id <story_id> --status "implemented" \
  --unit 1 --integration 1 --e2e 0 --evidence "<concrete_test_suite_logs_and_results>"
```

### Step 5: Log Trace & Refinement
1. Log the execution trace in the SQLite DB:
   ```bash
   ./scripts/harness trace \
     --summary "<what_was_done>" \
     --intake <intake_id> \
     --story <story_id> \
     --outcome "completed" \
     --actions "developed_code,added_tests,updated_story_status" \
     --changed "<files_changed>" \
     --friction "<any_friction_or_pain>" \
     --agent <your_agent_name>
   ```
2. If you encountered any architectural, compilation, or tooling friction, add a backlog record immediately:
   ```bash
   ./scripts/harness backlog add --title "<name>" --pain "<what>" --suggestion "<how>" --risk "tiny"
   ```

### Step 6: Walkthrough & Delivery
1. Create or update the `walkthrough.md` artifact in the brain folder, outlining:
   * Changes made.
   * Verified tests and outcomes.
   * Embeddings/logs showing success.
2. Present the results to the user with references to the walkthrough.
