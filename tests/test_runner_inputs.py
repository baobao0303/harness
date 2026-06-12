import os
import sys
import subprocess

def run_runner(args, env):
    repo_root = "/Users/bao312/Desktop/BrewCompany/harness"
    runner_path = os.path.join(repo_root, "scripts/pm-skills-runner")
    cmd = [sys.executable, runner_path] + args
    res = subprocess.run(cmd, env=env, capture_output=True, text=True)
    return res.returncode, res.stdout, res.stderr

def main():
    repo_root = "/Users/bao312/Desktop/BrewCompany/harness"
    env = os.environ.copy()
    env["HARNESS_REPO_ROOT"] = repo_root
    env["PM_SKILLS_DIR"] = "/Users/bao312/Desktop/BrewCompany/pm-skills"
    
    test_cases = [
        # 1. Valid inputs
        {
            "name": "Valid Sprint with Enum (plan)",
            "args": ["sprint", "plan", "Sprint 1 plan details"],
            "expected_code": 0
        },
        {
            "name": "Valid Sprint with Enum (retro)",
            "args": ["sprint", "retro", "Sprint 1 retro details"],
            "expected_code": 0
        },
        {
            "name": "Valid Sprint with Enum case-insensitive (PLAN)",
            "args": ["sprint", "PLAN", "Sprint 1 plan details"],
            "expected_code": 0
        },
        {
            "name": "Valid Sprint skipping optional Enum",
            "args": ["sprint", "Sprint 1 plain context"],
            "expected_code": 0
        },
        {
            "name": "Valid draft-nda with spaces and special characters",
            "args": ["draft-nda", "Mutual NDA between Acme Corp & Beta LLC (jurisdiction: NY; 5 yrs)!"],
            "expected_code": 0
        },
        {
            "name": "Valid battlecard with two required arguments",
            "args": ["battlecard", "Our CRM Product", "Salesforce CRM"],
            "expected_code": 0
        },
        # 2. Empty / Missing arguments
        {
            "name": "Missing required arg for draft-nda",
            "args": ["draft-nda"],
            "expected_code": 1
        },
        {
            "name": "Missing second required arg for battlecard",
            "args": ["battlecard", "Our CRM Product"],
            "expected_code": 1
        },
        {
            "name": "Missing required arg for sprint",
            "args": ["sprint"],
            "expected_code": 1
        },
        # 3. Invalid Enum / Extra arguments
        {
            "name": "Invalid Enum value treated as context (extra arg error)",
            "args": ["sprint", "invalid_enum_mode", "Some sprint context"],
            "expected_code": 1
        },
        {
            "name": "Unexpected extra arguments for draft-nda",
            "args": ["draft-nda", "Acme Corp vs Beta LLC", "Extra Argument"],
            "expected_code": 1
        },
        # 4. Unknown commands
        {
            "name": "Non-existent command",
            "args": ["nonexistent-command", "arg1"],
            "expected_code": 1
        }
    ]
    
    report = []
    report.append("# Runner Wrapper Validation Report\n")
    report.append("This report logs the manual execution checks performed against `harness/scripts/pm-skills-runner` under different input configurations.\n")
    report.append("| Test Case Name | Arguments | Exit Code | Stderr Snippet | Status |")
    report.append("| :--- | :--- | :---: | :--- | :---: |")
    
    for tc in test_cases:
        code, stdout, stderr = run_runner(tc["args"], env)
        stderr_clean = stderr.strip().replace("\n", " ")
        if len(stderr_clean) > 80:
            stderr_clean = stderr_clean[:77] + "..."
            
        status = "PASS" if code == tc["expected_code"] else "FAIL"
        report.append(f"| {tc['name']} | `{tc['args']}` | {code} | `{stderr_clean}` | {status} |")
        
    report.append("\n## Detailed Execution Logs\n")
    for tc in test_cases:
        code, stdout, stderr = run_runner(tc["args"], env)
        report.append(f"### Test: {tc['name']}")
        report.append(f"- **Command**: `pm-skills-runner {' '.join(tc['args'])}`")
        report.append(f"- **Exit Code**: {code}")
        if stdout:
            # Print first 5 lines of stdout
            stdout_lines = stdout.strip().splitlines()
            stdout_summary = "\n".join(stdout_lines[:5])
            if len(stdout_lines) > 5:
                stdout_summary += "\n..."
            report.append("- **Stdout**:\n```\n" + stdout_summary + "\n```")
        if stderr:
            report.append("- **Stderr**:\n```\n" + stderr.strip() + "\n```")
        report.append("\n" + "-"*40 + "\n")
        
    report_path = os.path.join(repo_root, "tests/runner_inputs_report.md")
    with open(report_path, "w", encoding="utf-8") as f:
        f.write("\n".join(report))
        
    print("Runner validation report successfully written to tests/runner_inputs_report.md")

if __name__ == "__main__":
    main()
