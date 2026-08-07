import os
import sys
import json
import tempfile
import shutil
import subprocess
from pathlib import Path

def setup_mock_pm_skills():
    temp_dir = tempfile.mkdtemp()
    plugins = {
        "pm-execution": {
            "manifest": {"name": "pm-execution", "version": "1.0.0", "description": "Mock execution plugin"},
            "commands": {
                "sprint": "---\ndescription: Sprint planning\nargument-hint: \"[plan|review|retro] <sprint_context>\"\n---\nSprint planning workflow content."
            }
        },
        "pm-toolkit": {
            "manifest": {"name": "pm-toolkit", "version": "1.0.0", "description": "Mock toolkit plugin"},
            "commands": {
                "draft-nda": "---\ndescription: Draft NDA\nargument-hint: \"<parties>\"\n---\nDraft NDA workflow content."
            }
        },
        "pm-product-strategy": {
            "manifest": {"name": "pm-product-strategy", "version": "1.0.0", "description": "Mock strategy plugin"},
            "commands": {
                "battlecard": "---\ndescription: Battlecard comparison\nargument-hint: \"<ProductA> <ProductB>\"\n---\nBattlecard workflow content."
            }
        }
    }
    for plugin, data in plugins.items():
        plugin_dir = os.path.join(temp_dir, plugin)
        os.makedirs(os.path.join(plugin_dir, ".claude-plugin"), exist_ok=True)
        with open(os.path.join(plugin_dir, ".claude-plugin", "plugin.json"), "w") as f:
            json.dump(data["manifest"], f)
        cmd_dir = os.path.join(plugin_dir, "commands")
        os.makedirs(cmd_dir, exist_ok=True)
        for cmd_name, content in data["commands"].items():
            with open(os.path.join(cmd_dir, f"{cmd_name}.md"), "w") as f:
                f.write(content)
    return temp_dir

def run_runner(args, env):
    repo_root = os.environ.get(
        "HARNESS_REPO_ROOT",
        str(Path(__file__).resolve().parent.parent)
    )
    runner_path = os.path.join(repo_root, "scripts/pm-skills-runner")
    cmd = [sys.executable, runner_path] + args
    res = subprocess.run(cmd, env=env, capture_output=True, text=True)
    return res.returncode, res.stdout, res.stderr

def main():
    repo_root = os.environ.get(
        "HARNESS_REPO_ROOT",
        str(Path(__file__).resolve().parent.parent)
    )
    env = os.environ.copy()
    env["HARNESS_REPO_ROOT"] = repo_root

    pm_skills_dir = env.get("PM_SKILLS_DIR")
    if not pm_skills_dir:
        parent_dir = str(Path(repo_root).parent / "pm-skills")
        pm_skills_dir = parent_dir

    created_temp_dir = None
    if not os.path.isdir(pm_skills_dir):
        created_temp_dir = setup_mock_pm_skills()
        env["PM_SKILLS_DIR"] = created_temp_dir
    else:
        env["PM_SKILLS_DIR"] = pm_skills_dir

    try:
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
                "expected_code": 1,
                "expected_stderr": "Validation Error: Missing required argument"
            },
            {
                "name": "Missing second required arg for battlecard",
                "args": ["battlecard", "Our CRM Product"],
                "expected_code": 1,
                "expected_stderr": "Validation Error: Missing required argument"
            },
            {
                "name": "Missing required arg for sprint",
                "args": ["sprint"],
                "expected_code": 1,
                "expected_stderr": "Validation Error: Missing required argument"
            },
            # 3. Invalid Enum / Extra arguments
            {
                "name": "Invalid Enum value treated as context (extra arg error)",
                "args": ["sprint", "invalid_enum_mode", "Some sprint context"],
                "expected_code": 1,
                "expected_stderr": "Unexpected extra arguments"
            },
            {
                "name": "Unexpected extra arguments for draft-nda",
                "args": ["draft-nda", "Acme Corp vs Beta LLC", "Extra Argument"],
                "expected_code": 1,
                "expected_stderr": "Unexpected extra arguments"
            },
            # 4. Unknown commands
            {
                "name": "Non-existent command",
                "args": ["nonexistent-command", "arg1"],
                "expected_code": 1,
                "expected_stderr": "not found under pm-skills directory"
            }
        ]

        report = []
        report.append("# Runner Wrapper Validation Report\n")
        report.append("This report logs the manual execution checks performed against `harness/scripts/pm-skills-runner` under different input configurations.\n")
        report.append("| Test Case Name | Arguments | Exit Code | Stderr Snippet | Status |")
        report.append("| :--- | :--- | :---: | :--- | :---: |")

        failed_count = 0
        execution_results = []

        for tc in test_cases:
            code, stdout, stderr = run_runner(tc["args"], env)
            stderr_clean = stderr.strip().replace("\n", " ")
            
            passed = (code == tc["expected_code"])
            if "expected_stderr" in tc and tc["expected_stderr"] not in stderr:
                passed = False

            status = "PASS" if passed else "FAIL"
            if not passed:
                failed_count += 1

            if len(stderr_clean) > 80:
                stderr_clean_snip = stderr_clean[:77] + "..."
            else:
                stderr_clean_snip = stderr_clean

            report.append(f"| {tc['name']} | `{tc['args']}` | {code} | `{stderr_clean_snip}` | {status} |")
            execution_results.append((tc, code, stdout, stderr, status))

        report.append("\n## Detailed Execution Logs\n")
        for tc, code, stdout, stderr, status in execution_results:
            report.append(f"### Test: {tc['name']}")
            report.append(f"- **Command**: `pm-skills-runner {' '.join(tc['args'])}`")
            report.append(f"- **Exit Code**: {code}")
            report.append(f"- **Status**: {status}")
            if stdout:
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
        if failed_count > 0:
            print(f"ERROR: {failed_count} test case(s) failed in test_runner_inputs.py!", file=sys.stderr)
            sys.exit(1)
        else:
            print("All test cases passed successfully in test_runner_inputs.py.")
            sys.exit(0)
    finally:
        if created_temp_dir and os.path.isdir(created_temp_dir):
            shutil.rmtree(created_temp_dir)

if __name__ == "__main__":
    main()
