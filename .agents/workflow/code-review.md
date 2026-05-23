# Harness Code Review Workflow (CRW)

**Goal:** Conduct a thorough, adversarial, and high-quality review of all codebase modifications to ensure zero regressions, clean design patterns, and robust error handling.

---

## 🏃 Workflow Steps

### Step 1: Gather Context
1. Identify all changed files in the current commit or pull request:
   ```bash
   git diff --name-only
   ```
2. Read the changes and map them back to the user story or intake criteria.

### Step 2: Adversarial Review Checklist (The "Devil's Advocate")
Review the modified code blocks strictly against the following criteria:

* **1. Correctness & Logic**:
  * Are there any potential race conditions or concurrency locks?
  * Are null/undefined values and empty inputs handled safely?
  * Are there any unhandled errors in async blocks?
* **2. Quality & Architecture**:
  * Does the implementation match the conventions in `project-context.md`?
  * Are functions well-focused, short, and reusable?
  * Is the naming clean and descriptive?
* **3. Performance & Resource Management**:
  * Are database queries optimized (e.g. indices used, no N+1 queries)?
  * Are resources (file handles, database connections) closed properly?
  * Are there any memory leaks or infinite loops?
* **4. Test Coverage**:
  * Are unit and integration tests added for all new functions?
  * Do tests cover negative paths and edge cases?

### Step 3: Local Compilation & Testing
Run local tests to ensure the changes build and compile:
```bash
cp harness.toml Cargo.toml && cp harness.lock Cargo.lock && cargo fmt --check && cargo test && rm Cargo.toml Cargo.lock
```

### Step 4: Submit Review Report
Present a clean, formatted Code Review Report to the developer:
- **Verdict**: `APPROVED` or `CHANGES REQUESTED`
- **Strengths**: Highlights of clean code or elegant design patterns.
- **Issues Found**: Bullet points showing exact file paths, line ranges, issues, and suggested fixes.
