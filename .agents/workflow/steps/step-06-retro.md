# Step 6: Sprint Retrospective & Trace Logging

**Goal:** Capture development friction to grow the harness, record the final mandatory execution trace in the SQLite DB, and deliver the completed task artifacts.

---

## 🏃 Finalization Steps

1. **Capture Friction (Harness Growth)**:
   * Evaluate if you encountered any toolchain, documentation, or planning friction.
   * If yes, register a backlog item in the durable DB:
     ```bash
     ./scripts/harness backlog add --title "<short_name>" --pain "<what>" --suggestion "<how>" --risk "tiny"
     ```
2. **Mandatory Trace Logging**:
   * Log the complete execution trace in the SQLite database:
     ```bash
     ./scripts/harness trace \
       --summary "<what_was_done>" \
       --intake <intake_id> \
       --story <story_id> \
       --outcome "completed" \
       --actions "<comma_separated_actions>" \
       --read "<files_read>" \
       --changed "<files_changed>" \
       --friction "<friction_encountered>" \
       --agent <your_agent_name>
     ```
3. **Compile Walkthrough**:
   * Create or update the `walkthrough.md` file in the brain directory, documenting:
     * Changes made.
     * What was tested and the validation results.
     * Concrete logs or screenshots.

---

## 🛑 FINAL CHECKPOINT & DONE DEFINITION
Deliver a clean final summary to the human:
1. **What was done**: Bullet points of implemented features.
2. **What proof was verified**: Unit / Integration / E2E test proof details.
3. **Recorded Trace ID**: The recorded trace number.
4. **Recorded Backlog ID**: The recorded backlog number (if any).

This completes the Master Development Loop! The repository is now fully verified and locked.
