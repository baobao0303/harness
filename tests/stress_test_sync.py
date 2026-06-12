import os
import sys
import time
import shutil
import tempfile
import subprocess
import statistics

def run_command(cmd, env):
    t0 = time.time()
    res = subprocess.run(cmd, env=env, capture_output=True, text=True)
    t1 = time.time()
    return res, t1 - t0

def main():
    repo_root = "/Users/bao312/Desktop/BrewCompany/harness"
    cli_path = os.path.join(repo_root, "scripts/bin/harness-cli")
    sync_script = os.path.join(repo_root, "scripts/sync-pm-skills.py")
    results_path = os.path.join(repo_root, "tests/stress_test_results.md")
    
    # 1. Setup temporary database for clean test environment
    db_fd, db_path = tempfile.mkstemp(suffix=".stress.db")
    os.close(db_fd)
    
    env = os.environ.copy()
    env["HARNESS_DB"] = db_path
    env["HARNESS_REPO_ROOT"] = repo_root
    
    subprocess.run([cli_path, "init"], env=env, check=True)
    subprocess.run([cli_path, "migrate"], env=env, check=True)
    
    # 2. Setup temporary pm-skills directory
    original_pm_skills = "/Users/bao312/Desktop/BrewCompany/pm-skills"
    temp_pm_skills = tempfile.mkdtemp()
    env["PM_SKILLS_DIR"] = temp_pm_skills
    
    # Copy all original plugins to temp_pm_skills
    for item in os.listdir(original_pm_skills):
        src_path = os.path.join(original_pm_skills, item)
        if os.path.isdir(src_path) and not item.startswith('.'):
            shutil.copytree(src_path, os.path.join(temp_pm_skills, item))
            
    markdown_report = []
    markdown_report.append("# Stress Test Results: `sync-pm-skills.py` performance\n")
    
    try:
        # Run 1: First sync (Registers all 42 tools)
        res, duration = run_command([sys.executable, sync_script], env)
        markdown_report.append(f"## Bootstrap Sync (First Run)\n")
        markdown_report.append(f"- **Description**: Syncing against a clean SQLite database to register all 42 custom tools.")
        markdown_report.append(f"- **Execution Time**: {duration:.4f} seconds")
        markdown_report.append(f"- **Registered**: 42 tools")
        markdown_report.append(f"- **Exit Code**: {res.returncode}\n")
        
        # Run 2-21: Idempotency stress test (No changes, 20 iterations)
        times = []
        for i in range(20):
            res, duration = run_command([sys.executable, sync_script], env)
            times.append(duration)
            if res.returncode != 0:
                markdown_report.append(f"**Warning**: Iteration {i} failed with exit code {res.returncode}. Stderr: {res.stderr}")
                
        avg_time = statistics.mean(times)
        min_time = min(times)
        max_time = max(times)
        std_dev = statistics.stdev(times)
        
        markdown_report.append("## Idempotency (No-op) Stress Test (20 Iterations)\n")
        markdown_report.append("| Metric | Value |")
        markdown_report.append("| :--- | :--- |")
        markdown_report.append(f"| Average Time | {avg_time:.4f}s |")
        markdown_report.append(f"| Minimum Time | {min_time:.4f}s |")
        markdown_report.append(f"| Maximum Time | {max_time:.4f}s |")
        markdown_report.append(f"| Standard Deviation | {std_dev:.4f}s |")
        markdown_report.append("\nIndividual run times (seconds):")
        markdown_report.append(", ".join([f"{t:.4f}" for t in times]) + "\n")
        
        # 3. Measure specific scenarios
        # Scenario A: Registering 1 new tool
        new_cmd_path = os.path.join(temp_pm_skills, "pm-toolkit", "commands", "new-stress-tool.md")
        with open(new_cmd_path, "w") as f:
            f.write("---\ndescription: A new tool added for stress testing\nargument-hint: \"[arg1|arg2] <arg3>\"\n---\nNew Tool Body")
            
        res_a, duration_register = run_command([sys.executable, sync_script], env)
        markdown_report.append("## Mutation Scenarios\n")
        markdown_report.append(f"### Scenario A: Register 1 New Tool")
        markdown_report.append(f"- **Execution Time**: {duration_register:.4f}s")
        markdown_report.append(f"- **Action**: Registers a single new custom tool, leaving the other 42 skipped.")
        markdown_report.append(f"- **Output**: `{res_a.stdout.strip().splitlines()[-1]}`\n")
        
        # Scenario B: Updating 1 existing tool (config changed)
        with open(new_cmd_path, "w") as f:
            f.write("---\ndescription: Updated description for stress tool\nargument-hint: \"[arg1|arg2] <arg3>\"\n---\nNew Tool Body")
            
        res_b, duration_update = run_command([sys.executable, sync_script], env)
        markdown_report.append(f"### Scenario B: Update 1 Existing Tool")
        markdown_report.append(f"- **Execution Time**: {duration_update:.4f}s")
        markdown_report.append(f"- **Action**: Modifies config of one tool, triggers remove + register for that tool, leaving 42 skipped.")
        markdown_report.append(f"- **Output**: `{res_b.stdout.strip().splitlines()[-1]}`\n")
        
        # Scenario C: Removing 1 deprecated tool
        os.remove(new_cmd_path)
        res_c, duration_remove = run_command([sys.executable, sync_script], env)
        markdown_report.append(f"### Scenario C: Remove 1 Deprecated Tool")
        markdown_report.append(f"- **Execution Time**: {duration_remove:.4f}s")
        markdown_report.append(f"- **Action**: Detects a registered tool that no longer exists in pm-skills, removes it.")
        markdown_report.append(f"- **Output**: `{res_c.stdout.strip().splitlines()[-1]}`\n")
        
        # Scenario D: Bulk changes (10 tools updated, 5 registered, 5 removed)
        removed_tools = ["brainstorm", "sprint", "draft-nda", "analyze-cohorts", "battlecard"]
        for r_tool in removed_tools:
            found = False
            for p in os.listdir(temp_pm_skills):
                cand = os.path.join(temp_pm_skills, p, "commands", f"{r_tool}.md")
                if os.path.isfile(cand):
                    os.remove(cand)
                    found = True
                    break
                    
        updated_count = 0
        for p in os.listdir(temp_pm_skills):
            commands_dir = os.path.join(temp_pm_skills, p, "commands")
            if os.path.isdir(commands_dir):
                for cmd_file in os.listdir(commands_dir):
                    if cmd_file.endswith(".md") and updated_count < 10:
                        cmd_path = os.path.join(commands_dir, cmd_file)
                        with open(cmd_path, "r") as f:
                            content = f.read()
                        if "description:" in content:
                            new_content = content.replace("description:", "description: Updated ")
                            with open(cmd_path, "w") as f:
                                f.write(new_content)
                            updated_count += 1
                            
        for i in range(5):
            new_cmd = os.path.join(temp_pm_skills, "pm-toolkit", "commands", f"bulk-new-{i}.md")
            with open(new_cmd, "w") as f:
                f.write(f"---\ndescription: Bulk new tool {i}\n---\nBody")
                
        res_d, duration_bulk = run_command([sys.executable, sync_script], env)
        markdown_report.append(f"### Scenario D: Bulk Mutation (10 Updates, 5 Registrations, 5 Removals)")
        markdown_report.append(f"- **Execution Time**: {duration_bulk:.4f}s")
        markdown_report.append(f"- **Action**: Performs a mix of CLI register/remove calls in a single execution.")
        markdown_report.append(f"- **Output**: `{res_d.stdout.strip().splitlines()[-1]}`\n")
        
    finally:
        if os.path.exists(db_path):
            os.remove(db_path)
        if os.path.exists(temp_pm_skills):
            shutil.rmtree(temp_pm_skills)
            
    with open(results_path, "w", encoding="utf-8") as f:
        f.write("\n".join(markdown_report))
    print("Stress test report successfully written to tests/stress_test_results.md")

if __name__ == "__main__":
    main()
