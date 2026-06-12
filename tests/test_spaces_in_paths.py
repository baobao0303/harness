import os
import sys
import shutil
import tempfile
import subprocess

def main():
    repo_root = "/Users/bao312/Desktop/BrewCompany/harness"
    cli_path = os.path.join(repo_root, "scripts/bin/harness-cli")
    sync_script = os.path.join(repo_root, "scripts/sync-pm-skills.py")
    runner_path = os.path.join(repo_root, "scripts/pm-skills-runner")
    
    # 1. Setup temporary database
    db_fd, db_path = tempfile.mkstemp(suffix=".space.db")
    os.close(db_fd)
    
    # 2. Setup temporary directory with spaces in name
    temp_parent = tempfile.mkdtemp()
    spaced_dir = os.path.join(temp_parent, "pm skills spaced dir")
    os.makedirs(spaced_dir)
    
    # Copy pm-toolkit to the spaced directory
    src_plugin = "/Users/bao312/Desktop/BrewCompany/pm-skills/pm-toolkit"
    dest_plugin = os.path.join(spaced_dir, "pm-toolkit")
    shutil.copytree(src_plugin, dest_plugin)
    
    env = os.environ.copy()
    env["HARNESS_DB"] = db_path
    env["HARNESS_REPO_ROOT"] = repo_root
    env["PM_SKILLS_DIR"] = spaced_dir
    
    report = []
    report.append("# Space in Paths Handling Verification\n")
    report.append(f"Testing environment with `PM_SKILLS_DIR` set to a path containing spaces: `{spaced_dir}`\n")
    
    try:
        # Initialize DB
        subprocess.run([cli_path, "init"], env=env, capture_output=True, check=True)
        subprocess.run([cli_path, "migrate"], env=env, capture_output=True, check=True)
        
        # Run sync script
        res_sync = subprocess.run([sys.executable, sync_script], env=env, capture_output=True, text=True)
        report.append("## Sync Script Output")
        report.append(f"- Exit Code: {res_sync.returncode}")
        report.append("- Stdout:")
        report.append("```\n" + res_sync.stdout.strip() + "\n```")
        if res_sync.stderr:
            report.append("- Stderr:")
            report.append("```\n" + res_sync.stderr.strip() + "\n```")
            
        # Run runner wrapper
        res_run = subprocess.run([sys.executable, runner_path, "draft-nda", "Acme Corp and John Doe"], env=env, capture_output=True, text=True)
        report.append("\n## Runner Wrapper Output (for draft-nda)")
        report.append(f"- Exit Code: {res_run.returncode}")
        report.append("- Stdout:")
        # Show first 5 lines of stdout
        stdout_lines = res_run.stdout.strip().splitlines()
        stdout_summary = "\n".join(stdout_lines[:5])
        if len(stdout_lines) > 5:
            stdout_summary += "\n..."
        report.append("```\n" + stdout_summary + "\n```")
        if res_run.stderr:
            report.append("- Stderr:")
            report.append("```\n" + res_run.stderr.strip() + "\n```")
            
        success = (res_sync.returncode == 0 and res_run.returncode == 0)
        report.append(f"\n**Overall Verdict**: {'PASSED' if success else 'FAILED'}")
        
    finally:
        # Cleanup
        if os.path.exists(db_path):
            os.remove(db_path)
        if os.path.exists(temp_parent):
            shutil.rmtree(temp_parent)
            
    report_path = os.path.join(repo_root, "tests/spaces_in_paths_report.md")
    with open(report_path, "w", encoding="utf-8") as f:
        f.write("\n".join(report))
    print(f"Spaces in paths report successfully written to {report_path}")

if __name__ == "__main__":
    main()
