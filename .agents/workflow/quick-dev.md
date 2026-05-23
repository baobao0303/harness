# Harness Quick Dev Workflow (QDV)

**Goal:** Turn user intent, quick bug fixes, and minor copy edits (TINY lane tasks) into hardened, validated, and recorded code changes with zero planning overhead.

---

## 🏃 Workflow Steps

### Step 1: Clarify & Intake
1. Read the user's request and check for any ambiguous parts.
2. Formulate a brief, concrete implementation checklist.
3. Record the intake classification in the durable SQLite DB:
   ```bash
   ./scripts/harness intake --type "change_request" --summary "<short_description>" --lane "tiny"
   ```

### Step 2: Implementation
1. Identify the files that need modifications.
2. Implement the changes using clean, high-quality code.
3. Follow the project's existing coding standards (e.g., no `any` in TypeScript, clear struct layouts in Rust, strict POSIX compliance in shell scripts).

### Step 3: Local Verification
1. Run linting and formatting syntax checks on modified files:
   * **Rust**: `cp harness.toml Cargo.toml && cp harness.lock Cargo.lock && cargo fmt --check && rm Cargo.toml Cargo.lock`
   * **Shell Scripts**: `bash -n scripts/*.sh`
2. Run standard project tests to ensure no regressions occur.

### Step 4: Record Trace & Growth
1. Log the execution trace in the SQLite DB to document your work:
   ```bash
   ./scripts/harness trace \
     --summary "<what_was_done>" \
     --intake <intake_id> \
     --outcome "completed" \
     --actions "patched_code,verified_tests" \
     --changed "<files_changed>" \
     --agent <your_agent_name>
   ```
2. If you encountered any toolchain friction, config issues, or compilation pain, add it directly to the harness backlog:
   ```bash
   ./scripts/harness backlog add --title "<name>" --pain "<what>" --suggestion "<how>" --risk "tiny"
   ```

### Step 5: Present Results
Present a concise summary to the user:
- What changes were made.
- What verification steps passed.
- The recorded trace ID.
