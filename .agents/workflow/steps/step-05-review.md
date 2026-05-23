# Step 5: Verification & Adversarial Review

**Goal:** Execute full verification tests, run a hoài nghi (cynical) review of all changes to ensure zero gaps, and record proof evidence in the SQLite DB.

---

## 🧪 1. Run Complete Test Suite
Execute the project test suite using the custom manifest workaround:
```bash
cp harness.toml Cargo.toml && cp harness.lock Cargo.lock && cargo test && rm Cargo.toml Cargo.lock
```
*Ensure all 10+ tests pass with exit code 0.*

---

## 🧐 2. Cynical Review (Devil's Advocate)
Read the git diff hunks and evaluate against the adversarial review rules:
* **Gaps**: Are there any unhandled errors or missing else/default branches?
* **Edge Cases**: Run a boundary check (empty inputs, arithmetic bounds, off-by-one errors).
* **Workaround Checks**: Ensure that temporary `Cargo.toml` and `Cargo.lock` files are deleted and never committed.

---

## 💾 3. Record Verification Proof to DB
Once all checks pass, mark the story as `implemented` in the SQLite DB, providing concrete test results as evidence:
```bash
./scripts/harness story update --id <story_id> --status "implemented" \
  --unit 1 --integration 1 --e2e 0 --evidence "<concrete_test_suite_logs_and_results>"
```
*Verify the test matrix is updated:*
```bash
./scripts/harness query matrix
```

---

## 🛑 CHECKPOINT
Present a summary of the verification to the human:
* **Test Suite Result**: "Pass / Fail" with logs snippet.
* **Cynical Review Verdict**: List any remaining minor bugs or state "APPROVED".
* **DB Matrix Output**: Print the updated story row from `./scripts/harness query matrix`.

**DO NOT ADVANCE** to Step 6 until the human approves the test proof.
Once approved, load and execute:
👉 **[step-06-retro.md](file:///Users/bao312/Desktop/harness/.agents/workflow/steps/step-06-retro.md)**
