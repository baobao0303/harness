# Step 4: Lane-Constrained Development

**Goal:** Implement the code and test modifications, adhering strictly to safety boundaries and local verification requirements.

---

## 🏃 Active Development Loop

1. **Mark In Progress**: Before modifying code, mark the active Story ID as `in_progress` in the SQLite DB:
   ```bash
   ./scripts/harness story update --id <story_id> --status "in_progress"
   ```
2. **Apply Lane Constraints**:
   * **Tiny Lane**: Direct patch, keep files clean, run formatting.
   * **Normal Lane**: Strictly implement within existing boundaries. Ensure unit and integration tests are co-located or added.
   * **High-Risk Lane**: Follow the high-risk design spec closely. Seek human feedback at every milestone.
3. **Continuous Local Validation**:
   * **Formatting Check**: Use the custom manifest workaround for formatting verification:
     ```bash
     cp harness.toml Cargo.toml && cp harness.lock Cargo.lock && cargo fmt --check && rm Cargo.toml Cargo.lock
     ```
   * **Syntax Checker**: Ensure shell scripts are POSIX compliant:
     ```bash
     bash -n scripts/*.sh
     ```

---

## 🛑 CHECKPOINT
Present a summary of the implementation to the human:
* **Files Modified**: List files changed or created.
* **Local Verifications Run**: Format check results and script check outcomes.
* **Test Code Added**: Highlight any new unit or integration tests created.

**DO NOT ADVANCE** to Step 5 until the human confirms the implementation is ready for review.
Once approved, load and execute:
👉 **[step-05-review.md](file:///Users/bao312/Desktop/harness/.agents/workflow/steps/step-05-review.md)**
