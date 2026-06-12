# Space in Paths Handling Verification

Testing environment with `PM_SKILLS_DIR` set to a path containing spaces: `/var/folders/5h/qdnkq5hn1fn1130ljb8vdys80000gn/T/tmps5x3s7ab/pm skills spaced dir`

## Sync Script Output
- Exit Code: 0
- Stdout:
```
Tool draft-nda registered.
Tool privacy-policy registered.
Tool tailor-resume registered.
Tool proofread registered.
Tool review-resume registered.
Using CLI path: /Users/bao312/Desktop/BrewCompany/harness/scripts/bin/harness-cli
Scanning PM skills directory: /var/folders/5h/qdnkq5hn1fn1130ljb8vdys80000gn/T/tmps5x3s7ab/pm skills spaced dir
Found 1 plugins: ['pm-toolkit']
Registering tool: /Users/bao312/Desktop/BrewCompany/harness/scripts/bin/harness-cli tool register --name draft-nda --command ./scripts/pm-skills-runner draft-nda --description Draft a Non-Disclosure Agreement between two parties with jurisdiction-appropriate clauses --responsibility Tool access --force --args parties_and_context:string:required
Registering tool: /Users/bao312/Desktop/BrewCompany/harness/scripts/bin/harness-cli tool register --name privacy-policy --command ./scripts/pm-skills-runner privacy-policy --description Draft a privacy policy covering data collection, usage, storage, and compliance requirements --responsibility Tool access --force --args product_and_data_handling_context:string:required
Registering tool: /Users/bao312/Desktop/BrewCompany/harness/scripts/bin/harness-cli tool register --name tailor-resume --command ./scripts/pm-skills-runner tailor-resume --description Tailor a PM resume to a specific job description — keyword alignment, experience reframing, and strategic optimization --responsibility Tool access --force --args resume:string:required,job_description:string:required
Registering tool: /Users/bao312/Desktop/BrewCompany/harness/scripts/bin/harness-cli tool register --name proofread --command ./scripts/pm-skills-runner proofread --description Check grammar, logic, and flow in any text — targeted fixes without rewriting --responsibility Tool access --force --args text_to_check:string:required
Registering tool: /Users/bao312/Desktop/BrewCompany/harness/scripts/bin/harness-cli tool register --name review-resume --command ./scripts/pm-skills-runner review-resume --description Comprehensive PM resume review against 10 best practices — structure, impact metrics, keywords, and actionable feedback --responsibility Tool access --force --args resume_as_text_or_file:string:required
Sync complete. Registered: 5, Updated: 0, Skipped: 0, Removed: 0
```

## Runner Wrapper Output (for draft-nda)
- Exit Code: 0
- Stdout:
```
Description: Draft a Non-Disclosure Agreement between two parties with jurisdiction-appropriate clauses

# /draft-nda -- NDA Drafting

Draft a professional Non-Disclosure Agreement customized to your situation. Covers information types, jurisdiction, term, and clearly marks clauses that need legal review.
...
```

**Overall Verdict**: PASSED