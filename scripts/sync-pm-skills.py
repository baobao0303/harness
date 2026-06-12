#!/usr/bin/env python3
import os
import sys
import json
import re
import subprocess

def parse_frontmatter(content):
    lines = content.splitlines()
    if not lines or lines[0].strip() != '---':
        return {}
    frontmatter = {}
    for i in range(1, len(lines)):
        line = lines[i].strip()
        if line == '---':
            break
        if ':' in line:
            key, val = line.split(':', 1)
            frontmatter[key.strip()] = val.strip().strip('"').strip("'")
    return frontmatter

def parse_argument_hint(hint_str):
    if not hint_str:
        return ""
    matches = re.finditer(r'(\[([^\]]+)\]|<([^>]+)>)', hint_str)
    args = []
    for m in matches:
        raw_token = m.group(1)
        if raw_token.startswith('['):
            required = "optional"
            content = m.group(2)
        else:
            required = "required"
            content = m.group(3)
        
        parts = content.split(';', 1)
        main_part = parts[0].strip()
        help_text = parts[1].strip() if len(parts) > 1 else ""
        
        if '|' in main_part:
            arg_type = "enum"
            name = main_part.replace('|', '_')
            if not help_text:
                help_text = main_part
        else:
            arg_type = "string"
            name = main_part
        
        name = name.lower()
        name = re.sub(r'[^a-z0-9_]', '_', name)
        name = re.sub(r'_+', '_', name).strip('_')
        
        if not name:
            name = "arg"
        
        if help_text:
            args.append(f"{name}:{arg_type}:{required}:{help_text}")
        else:
            args.append(f"{name}:{arg_type}:{required}")
            
    return ",".join(args)

def normalize_description(desc):
    desc = desc.strip()
    if len(desc) < 10:
        desc = desc + " (harness)"
    if len(desc) < 10:
        desc = desc + "." * (10 - len(desc))
    elif len(desc) > 200:
        desc = desc[:197] + "..."
    return desc

def get_responsibility(plugin_name):
    mapping = {
        'pm-product-discovery': 'Task specification',
        'pm-product-strategy': 'Task specification',
        'pm-execution': 'Task state',
        'pm-data-analytics': 'Project memory',
        'pm-market-research': 'Project memory',
        'pm-marketing-growth': 'Project memory',
        'pm-go-to-market': 'Project memory',
        'pm-toolkit': 'Tool access',
        'pm-ai-shipping': 'Tool access'
    }
    return mapping.get(plugin_name.lower(), 'Tool access')

def find_cli_path(repo_root):
    paths_to_try = [
        os.path.join(repo_root, "scripts/bin/harness-cli"),
        os.path.join(repo_root, "harness/scripts/bin/harness-cli"),
        os.path.abspath(os.path.join(os.path.dirname(__file__), "bin/harness-cli")),
        "harness-cli"
    ]
    for path in paths_to_try:
        if os.path.isfile(path) and os.access(path, os.X_OK):
            return os.path.abspath(path)
    return "harness-cli"

def main():
    repo_root = os.environ.get("HARNESS_REPO_ROOT")
    if not repo_root:
        cur = os.path.abspath(os.path.dirname(__file__))
        while cur and os.path.basename(cur) != "BrewCompany":
            parent = os.path.dirname(cur)
            if parent == cur:
                break
            cur = parent
        repo_root = os.path.join(cur, "harness") if os.path.isdir(os.path.join(cur, "harness")) else cur
    
    pm_skills_dir = os.environ.get("PM_SKILLS_DIR")
    if not pm_skills_dir:
        parent_dir = os.path.dirname(repo_root)
        pm_skills_dir = os.path.join(parent_dir, "pm-skills")
        if not os.path.isdir(pm_skills_dir):
            pm_skills_dir = "/Users/bao312/Desktop/BrewCompany/pm-skills"

    cli_path = find_cli_path(repo_root)
    print(f"Using CLI path: {cli_path}")
    print(f"Scanning PM skills directory: {pm_skills_dir}")
    
    if not os.path.isdir(pm_skills_dir):
        print(f"Error: pm-skills directory '{pm_skills_dir}' not found.", file=sys.stderr)
        sys.exit(1)

    env = os.environ.copy()
    env["HARNESS_REPO_ROOT"] = repo_root
    
    try:
        res = subprocess.run([cli_path, "query", "tools", "--json"], capture_output=True, text=True, env=env, check=True)
        registered_tools = json.loads(res.stdout)
    except Exception as e:
        print(f"Warning: Could not fetch registered tools: {e}. Assuming empty.", file=sys.stderr)
        registered_tools = []

    tools_map = {t["name"]: t for t in registered_tools if t.get("provider") == "custom"}

    plugins = []
    for item in os.listdir(pm_skills_dir):
        plugin_path = os.path.join(pm_skills_dir, item)
        if os.path.isdir(plugin_path):
            manifest_path = os.path.join(plugin_path, ".claude-plugin", "plugin.json")
            if os.path.isfile(manifest_path):
                try:
                    with open(manifest_path, "r", encoding="utf-8") as f:
                        json.load(f)
                    plugins.append(item)
                except Exception as e:
                    print(f"Warning: Corrupt or invalid manifest in {item}: {e}. Skipping plugin.")

    print(f"Found {len(plugins)} plugins: {plugins}")

    registered_count = 0
    skipped_count = 0
    updated_count = 0
    removed_count = 0
    scanned_commands = set()

    for plugin in plugins:
        plugin_path = os.path.join(pm_skills_dir, plugin)
        commands_dir = os.path.join(plugin_path, "commands")
        if not os.path.isdir(commands_dir):
            continue

        responsibility = get_responsibility(plugin)

        for filename in os.listdir(commands_dir):
            if not filename.endswith(".md"):
                continue
            
            command_name = os.path.splitext(filename)[0]
            scanned_commands.add(command_name)
            command_file_path = os.path.join(commands_dir, filename)
            
            with open(command_file_path, "r", encoding="utf-8") as f:
                content = f.read()
            
            frontmatter = parse_frontmatter(content)
            description = frontmatter.get("description", "")
            argument_hint = frontmatter.get("argument-hint", "")
            
            if not description:
                description = f"Command {command_name} from plugin {plugin}"
                
            normalized_desc = normalize_description(description)
            parsed_args = parse_argument_hint(argument_hint)
            parsed_args_list = []
            if parsed_args:
                for arg in parsed_args.split(","):
                    parts = arg.split(":")
                    parsed_args_list.append({
                        "name": parts[0],
                        "type": parts[1],
                        "required": parts[2] == "required",
                        "help": parts[3] if len(parts) > 3 else ""
                    })
            
            runner_command = f"./scripts/pm-skills-runner {command_name}"

            existing_tool = tools_map.get(command_name)
            needs_register = True
            
            if existing_tool:
                existing_cmd = existing_tool.get("command", "")
                existing_desc = existing_tool.get("description", "")
                existing_resp = existing_tool.get("responsibility", "")
                existing_args = existing_tool.get("args", [])

                args_equal = len(existing_args) == len(parsed_args_list)
                if args_equal:
                    for a1, a2 in zip(existing_args, parsed_args_list):
                        if a1.get("name") != a2["name"] or a1.get("type") != a2["type"] or a1.get("required") != a2["required"]:
                            args_equal = False
                            break

                if (existing_cmd == runner_command and 
                    existing_desc == normalized_desc and 
                    existing_resp == responsibility and 
                    args_equal):
                    needs_register = False
                    skipped_count += 1
                else:
                    print(f"Updating tool '{command_name}' (config changed)")
                    subprocess.run([cli_path, "tool", "remove", "--name", command_name], env=env, check=False)
                    updated_count += 1

            if needs_register:
                cmd = [
                    cli_path, "tool", "register",
                    "--name", command_name,
                    "--command", runner_command,
                    "--description", normalized_desc,
                    "--responsibility", responsibility,
                    "--force"
                ]
                if parsed_args:
                    cmd.extend(["--args", parsed_args])
                
                print(f"Registering tool: {' '.join(cmd)}")
                subprocess.run(cmd, env=env, check=True)
                registered_count += 1
                
                # Dynamically update tools_map
                tools_map[command_name] = {
                    "name": command_name,
                    "provider": "custom",
                    "command": runner_command,
                    "description": normalized_desc,
                    "responsibility": responsibility,
                    "args": parsed_args_list
                }

    # 4. Remove custom tools starting with the runner that are no longer in pm-skills
    for name, tool_data in tools_map.items():
        if name not in scanned_commands:
            cmd_str = tool_data.get("command", "")
            if cmd_str.startswith("./scripts/pm-skills-runner") or "pm-skills-runner" in cmd_str:
                print(f"Removing deprecated tool: {name}")
                subprocess.run([cli_path, "tool", "remove", "--name", name], env=env, check=False)
                removed_count += 1

    print(f"Sync complete. Registered: {registered_count}, Updated: {updated_count}, Skipped: {skipped_count}, Removed: {removed_count}")

if __name__ == "__main__":
    main()
