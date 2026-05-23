# Step 2: System Architecture & Boundaries

**Goal:** Establish technical boundaries, design database or API contract shapes, and document architectural choices as permanent decisions.

---

## 🏛️ Architecture Rules

* **Progressive Disclosure**: Do not create a single giant specification. Instead, document your design as modular contracts in `docs/product/` or decisions in `docs/decisions/`.
* **Permanent Choices**: If the changes alter data schemas, tech stacks, external API contracts, or core abstractions, they **MUST** be recorded as an Architecture Decision Record (ADR) under `docs/decisions/` using the `decision.md` template.
* **Keep Contracts Up to Date**: Ensure existing product contract files are updated to reflect the new boundaries.

---

## 📝 Creating an ADR (If Required)
If you determine that an ADR is needed:
1. Create a file `docs/decisions/XXXX-short-title.md` (e.g. `0008-use-unified-cache.md`).
2. Populate it with the standard structure:
   * **Context**: What problem are we solving? What options did we consider?
   * **Decision**: What did we decide and why?
   * **Consequences**: Positive tradeoffs and negative tradeoffs.

---

## 🛑 CHECKPOINT
Present a summary to the human:
* **System Boundaries Affected**: What components are modified or added.
* **Decision Records (ADRs) Created/Modified**: List the files or state "No architectural changes required."

**DO NOT ADVANCE** to Step 3 until the human reviews the architecture boundaries.
Once approved, load and execute:
👉 **[step-03-stories.md](file:///Users/bao312/Desktop/harness/.agents/workflow/steps/step-03-stories.md)**
