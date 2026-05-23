# Step 1: Feature Intake & Risk Classification

**Goal:** Correctly intake the user requirement, run a precise risk checklist, assign the safety lane, and register the task in the durable database.

---

## 📋 Intake Execution Flow

1. **Understand Intent**: Read the user's prompt or provided specification. Identify what parts of the system are affected.
2. **Determine Input Type**: Classify the change request into one of the following:
   * `new_spec` (New project specification)
   * `spec_slice` (Implementing a slice of an already accepted spec)
   * `change_request` (Bug fix or small adjustment)
   * `new_initiative` (Large product area)
   * `maintenance` (Technical, dependency, or security chore)
   * `harness_improvement` (Agile process or process metadata improvement)
3. **Run Risk Checklist**: Evaluate if the task triggers any of the following 10 risk flags:
   * [ ] **Auth**: login, logout, refresh tokens, passwords, sessions, JWT.
   * [ ] **Authorization**: roles, tenant scope, custom permissions.
   * [ ] **Data Model**: database schema, migrations, data deletion/retention.
   * [ ] **Audit/Security**: security logs, access audits, sensitive data.
   * [ ] **External Systems**: payments, email servers, cloud providers, SDKs.
   * [ ] **Public Contracts**: API endpoints shape, response envelopes.
   * [ ] **Cross-Platform**: native boundaries (macOS/Linux), shell installs.
   * [ ] **Existing Behavior**: changing already implemented or test-covered code.
   * [ ] **Weak Proof**: lack of automated test coverage in the affected area.
   * [ ] **Multi-Domain**: multiple independent product modules changing at once.

---

## 🚦 Safety Lane Assignment
Based on the flags counted:
* **0 - 1 Flags**: `tiny` or `normal` lane (based on size of code blast radius).
* **2 - 3 Flags**: `normal` lane (requires Story file and test matrix proof).
* **4+ Flags OR Hard Gates (Auth, Authz, Data loss, weakening verification)**: `high_risk` lane.

---

## 💾 Database Registration
Execute the CLI command to persist this intake event:
```bash
./scripts/harness intake --type "<input_type>" --summary "<short_description>" --lane "<lane>"
```
*Record the returned Intake ID (e.g. Intake #14).*

---

## 🛑 CHECKPOINT
Present a clean summary of the intake analysis to the human:
* **Input Type**: e.g., `change_request`
* **Risk Flags Triggered**: List flags or "None"
* **Safety Lane**: `tiny`, `normal`, or `high_risk`
* **Database Intake ID**: The recorded ID

**DO NOT ADVANCE** to Step 2 until the human confirms the lane assignment!
Once approved, load and execute:
👉 **[step-02-architecture.md](file:///Users/bao312/Desktop/harness/.agents/workflow/steps/step-02-architecture.md)**
