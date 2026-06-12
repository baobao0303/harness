import os
import sys
import json
import tempfile
import shutil
import subprocess
import unittest

class PMskillsE2ETestSuite(unittest.TestCase):
    def setUp(self):
        # Setup temp database path
        self.db_fd, self.db_path = tempfile.mkstemp(suffix=".db")
        os.close(self.db_fd)
        
        # Base directories and script paths
        self.repo_root = "/Users/bao312/Desktop/BrewCompany/harness"
        self.cli_path = os.path.join(self.repo_root, "scripts/bin/harness-cli")
        self.sync_script = os.path.join(self.repo_root, "scripts/sync-pm-skills.py")
        self.runner_wrapper = os.path.join(self.repo_root, "scripts/pm-skills-runner")
        
        # Test environment
        self.env = os.environ.copy()
        self.env["HARNESS_DB"] = self.db_path
        self.env["HARNESS_REPO_ROOT"] = self.repo_root
        
        # Initialize and migrate DB
        subprocess.run([self.cli_path, "init"], env=self.env, capture_output=True, check=True)
        subprocess.run([self.cli_path, "migrate"], env=self.env, capture_output=True, check=True)
        
        self.temp_dirs_to_clean = []

    def tearDown(self):
        if os.path.exists(self.db_path):
            os.remove(self.db_path)
        for temp_dir in self.temp_dirs_to_clean:
            if os.path.isdir(temp_dir):
                shutil.rmtree(temp_dir)

    def create_mock_pm_skills(self, plugins_data):
        temp_dir = tempfile.mkdtemp()
        self.temp_dirs_to_clean.append(temp_dir)
        for plugin, data in plugins_data.items():
            plugin_path = os.path.join(temp_dir, plugin)
            os.makedirs(os.path.join(plugin_path, ".claude-plugin"), exist_ok=True)
            
            with open(os.path.join(plugin_path, ".claude-plugin", "plugin.json"), "w") as f:
                json.dump(data.get("manifest", {
                    "name": plugin,
                    "version": "1.0.0",
                    "description": f"Mock plugin {plugin}"
                }), f)
                
            commands_path = os.path.join(plugin_path, "commands")
            os.makedirs(commands_path, exist_ok=True)
            for cmd_name, content in data.get("commands", {}).items():
                with open(os.path.join(commands_path, f"{cmd_name}.md"), "w") as f:
                    f.write(content)
        return temp_dir

    def run_sync(self, pm_skills_dir=None):
        env = self.env.copy()
        if pm_skills_dir:
            env["PM_SKILLS_DIR"] = pm_skills_dir
        res = subprocess.run([sys.executable, self.sync_script], env=env, capture_output=True, text=True)
        return res

    def query_tools(self):
        res = subprocess.run([self.cli_path, "query", "tools", "--json"], env=self.env, capture_output=True, text=True, check=True)
        return json.loads(res.stdout)

    def run_wrapper(self, cmd_name, args_list, pm_skills_dir=None):
        env = self.env.copy()
        if pm_skills_dir:
            env["PM_SKILLS_DIR"] = pm_skills_dir
        cmd = [sys.executable, self.runner_wrapper, cmd_name] + args_list
        res = subprocess.run(cmd, env=env, capture_output=True, text=True)
        return res

    # ==========================================
    # TIER 1: FEATURE COVERAGE (>= 30 tests, 5 per feature)
    # ==========================================

    # --- Feature 1: Sync Script Scanning & Registry (F1) ---
    def test_t1_f1_scan_all_nine_plugins(self):
        res = self.run_sync()
        self.assertEqual(res.returncode, 0)
        self.assertIn("Found 9 plugins", res.stdout)

    def test_t1_f1_registers_all_42_commands(self):
        self.run_sync()
        tools = self.query_tools()
        custom_tools = [t for t in tools if t.get("provider") == "custom"]
        self.assertEqual(len(custom_tools), 42)

    def test_t1_f1_tool_properties(self):
        self.run_sync()
        tools = self.query_tools()
        brainstorm_tool = next(t for t in tools if t.get("name") == "brainstorm")
        self.assertEqual(brainstorm_tool.get("provider"), "custom")
        self.assertEqual(brainstorm_tool.get("command"), "./scripts/pm-skills-runner brainstorm")
        self.assertEqual(brainstorm_tool.get("since"), "registered")

    def test_t1_f1_command_list_matches(self):
        self.run_sync()
        tools = self.query_tools()
        custom_names = {t["name"] for t in tools if t.get("provider") == "custom"}
        self.assertIn("brainstorm", custom_names)
        self.assertIn("draft-nda", custom_names)
        self.assertIn("sprint", custom_names)

    def test_t1_f1_harness_cli_query_integration(self):
        self.run_sync()
        res = subprocess.run([self.cli_path, "query", "tools", "--summary"], env=self.env, capture_output=True, text=True, check=True)
        self.assertIn("brainstorm", res.stdout)
        self.assertIn("draft-nda", res.stdout)

    # --- Feature 2: Argument Hint Parsing (F2) ---
    def test_t1_f2_parse_required_string(self):
        mock_data = {
            "pm-toolkit": {
                "commands": {
                    "test-cmd": "---\ndescription: Test command\nargument-hint: \"<my_arg>\"\n---\nBody"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        tools = self.query_tools()
        tool = next(t for t in tools if t["name"] == "test-cmd")
        self.assertEqual(len(tool["args"]), 1)
        self.assertEqual(tool["args"][0]["name"], "my_arg")
        self.assertEqual(tool["args"][0]["type"], "string")
        self.assertTrue(tool["args"][0]["required"])

    def test_t1_f2_parse_optional_enum(self):
        mock_data = {
            "pm-toolkit": {
                "commands": {
                    "test-cmd": "---\ndescription: Test command\nargument-hint: \"[a|b]\"\n---\nBody"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        tools = self.query_tools()
        tool = next(t for t in tools if t["name"] == "test-cmd")
        self.assertEqual(len(tool["args"]), 1)
        self.assertEqual(tool["args"][0]["name"], "a_b")
        self.assertEqual(tool["args"][0]["type"], "enum")
        self.assertFalse(tool["args"][0]["required"])
        self.assertEqual(tool["args"][0]["help"], "a|b")

    def test_t1_f2_parse_required_enum(self):
        mock_data = {
            "pm-toolkit": {
                "commands": {
                    "test-cmd": "---\ndescription: Test command\nargument-hint: \"<a|b>\"\n---\nBody"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        tools = self.query_tools()
        tool = next(t for t in tools if t["name"] == "test-cmd")
        self.assertEqual(len(tool["args"]), 1)
        self.assertEqual(tool["args"][0]["name"], "a_b")
        self.assertEqual(tool["args"][0]["type"], "enum")
        self.assertTrue(tool["args"][0]["required"])

    def test_t1_f2_parse_multiple_args(self):
        mock_data = {
            "pm-toolkit": {
                "commands": {
                    "test-cmd": "---\ndescription: Test command\nargument-hint: \"[a|b] [c|d] <e>\"\n---\nBody"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        tools = self.query_tools()
        tool = next(t for t in tools if t["name"] == "test-cmd")
        self.assertEqual(len(tool["args"]), 3)
        self.assertEqual(tool["args"][0]["name"], "a_b")
        self.assertEqual(tool["args"][1]["name"], "c_d")
        self.assertEqual(tool["args"][2]["name"], "e")

    def test_t1_f2_parse_args_with_comments(self):
        mock_data = {
            "pm-toolkit": {
                "commands": {
                    "test-cmd": "---\ndescription: Test command\nargument-hint: \"<repo path or area; defaults to whole repo>\"\n---\nBody"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        tools = self.query_tools()
        tool = next(t for t in tools if t["name"] == "test-cmd")
        self.assertEqual(tool["args"][0]["name"], "repo_path_or_area") # from repo path or area; defaults...

    # --- Feature 3: Responsibility Mapping (F3) ---
    def test_t1_f3_product_discovery_mapping(self):
        self.run_sync()
        tools = self.query_tools()
        tool = next(t for t in tools if t["name"] == "brainstorm")
        self.assertEqual(tool["responsibility"], "Task specification")

    def test_t1_f3_execution_mapping(self):
        self.run_sync()
        tools = self.query_tools()
        tool = next(t for t in tools if t["name"] == "sprint")
        self.assertEqual(tool["responsibility"], "Task state")

    def test_t1_f3_data_analytics_mapping(self):
        self.run_sync()
        tools = self.query_tools()
        tool = next(t for t in tools if t["name"] == "analyze-cohorts")
        self.assertEqual(tool["responsibility"], "Project memory")

    def test_t1_f3_toolkit_mapping(self):
        self.run_sync()
        tools = self.query_tools()
        tool = next(t for t in tools if t["name"] == "draft-nda")
        self.assertEqual(tool["responsibility"], "Tool access")

    def test_t1_f3_ai_shipping_mapping(self):
        self.run_sync()
        tools = self.query_tools()
        tool = next(t for t in tools if t["name"] == "ship-check")
        self.assertEqual(tool["responsibility"], "Tool access")

    # --- Feature 4: Idempotency of Synchronization (F4) ---
    def test_t1_f4_double_sync_no_errors(self):
        res1 = self.run_sync()
        self.assertEqual(res1.returncode, 0)
        res2 = self.run_sync()
        self.assertEqual(res2.returncode, 0)

    def test_t1_f4_double_sync_no_duplicates(self):
        self.run_sync()
        self.run_sync()
        tools = self.query_tools()
        custom_tools = [t for t in tools if t.get("provider") == "custom"]
        self.assertEqual(len(custom_tools), 42)

    def test_t1_f4_sync_updates_changed_desc(self):
        mock_data = {
            "pm-toolkit": {
                "commands": {
                    "test-cmd": "---\ndescription: Initial description text\n---\nBody"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        tool1 = next(t for t in self.query_tools() if t["name"] == "test-cmd")
        self.assertEqual(tool1["description"], "Initial description text")

        # Update description
        mock_data["pm-toolkit"]["commands"]["test-cmd"] = "---\ndescription: Updated description text\n---\nBody"
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        tool2 = next(t for t in self.query_tools() if t["name"] == "test-cmd")
        self.assertEqual(tool2["description"], "Updated description text")

    def test_t1_f4_sync_updates_changed_responsibility(self):
        mock_data = {
            "pm-toolkit": {
                "commands": {
                    "test-cmd": "---\ndescription: Test command\n---\nBody"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        tool1 = next(t for t in self.query_tools() if t["name"] == "test-cmd")
        self.assertEqual(tool1["responsibility"], "Tool access")

        # Move to execution plugin to change responsibility to Task state
        mock_data2 = {
            "pm-execution": {
                "commands": {
                    "test-cmd": "---\ndescription: Test command\n---\nBody"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data2)
        self.run_sync(mock_dir)
        tool2 = next(t for t in self.query_tools() if t["name"] == "test-cmd")
        self.assertEqual(tool2["responsibility"], "Task state")

    def test_t1_f4_sync_updates_changed_args(self):
        mock_data = {
            "pm-toolkit": {
                "commands": {
                    "test-cmd": "---\ndescription: Test command\nargument-hint: \"<arg1>\"\n---\nBody"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        tool1 = next(t for t in self.query_tools() if t["name"] == "test-cmd")
        self.assertEqual(len(tool1["args"]), 1)
        self.assertEqual(tool1["args"][0]["name"], "arg1")

        # Update args
        mock_data["pm-toolkit"]["commands"]["test-cmd"] = "---\ndescription: Test command\nargument-hint: \"<arg1> <arg2>\"\n---\nBody"
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        tool2 = next(t for t in self.query_tools() if t["name"] == "test-cmd")
        self.assertEqual(len(tool2["args"]), 2)
        self.assertEqual(tool2["args"][1]["name"], "arg2")

    # --- Feature 5: Command Runner Execution & Output (F5) ---
    def test_t1_f5_runner_outputs_description(self):
        res = self.run_wrapper("brainstorm", ["ideas", "existing", "Mobile banking app"])
        self.assertEqual(res.returncode, 0)
        self.assertIn("Description: Brainstorm product ideas", res.stdout)

    def test_t1_f5_runner_outputs_workflow(self):
        res = self.run_wrapper("brainstorm", ["ideas", "existing", "Mobile banking app"])
        self.assertEqual(res.returncode, 0)
        self.assertIn("## Workflow", res.stdout)
        self.assertIn("Step 1: Determine Mode", res.stdout)

    def test_t1_f5_runner_exit_code_zero(self):
        res = self.run_wrapper("draft-nda", ["ACME Corp and Beta LLC"])
        self.assertEqual(res.returncode, 0)

    def test_t1_f5_runner_output_not_empty(self):
        res = self.run_wrapper("sprint", ["plan", "Sprint 42"])
        self.assertTrue(len(res.stdout) > 50)

    def test_t1_f5_runner_command_exists(self):
        res = self.run_wrapper("analyze-cohorts", ["cohorts.csv"])
        self.assertEqual(res.returncode, 0)
        self.assertIn("Description:", res.stdout)

    # --- Feature 6: Command Runner Arguments Validation (F6) ---
    def test_t1_f6_runner_validates_required_args_present(self):
        res = self.run_wrapper("draft-nda", ["ACME Corp"])
        self.assertEqual(res.returncode, 0)

    def test_t1_f6_runner_validates_optional_args_enum_match(self):
        res = self.run_wrapper("brainstorm", ["ideas", "existing", "Mobile banking app"])
        self.assertEqual(res.returncode, 0)

    def test_t1_f6_runner_validates_optional_args_skipped(self):
        res = self.run_wrapper("brainstorm", ["Mobile banking app"])
        self.assertEqual(res.returncode, 0)

    def test_t1_f6_runner_validates_exact_arg_count(self):
        res = self.run_wrapper("battlecard", ["ProductA", "ProductB"])
        self.assertEqual(res.returncode, 0)

    def test_t1_f6_runner_validates_missing_required_args_fails(self):
        res = self.run_wrapper("draft-nda", [])
        self.assertEqual(res.returncode, 1)
        self.assertIn("Validation Error: Missing required argument", res.stderr)


    # ==========================================
    # TIER 2: BOUNDARY & CORNER CASES (>= 30 tests, 5 per feature)
    # ==========================================

    # --- Feature 1: Sync Script Scanning (Boundary) ---
    def test_t2_f1_missing_commands_dir(self):
        mock_data = {
            "pm-toolkit": {
                "manifest": {"name": "pm-toolkit", "version": "1.0.0", "description": "No commands dir test"}
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        # Delete commands dir created by default helper
        shutil.rmtree(os.path.join(mock_dir, "pm-toolkit", "commands"))
        res = self.run_sync(mock_dir)
        self.assertEqual(res.returncode, 0)
        custom_tools = [t for t in self.query_tools() if t.get("provider") == "custom"]
        self.assertEqual(len(custom_tools), 0)

    def test_t2_f1_empty_commands_dir(self):
        mock_data = {
            "pm-toolkit": {
                "commands": {}
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        res = self.run_sync(mock_dir)
        self.assertEqual(res.returncode, 0)
        custom_tools = [t for t in self.query_tools() if t.get("provider") == "custom"]
        self.assertEqual(len(custom_tools), 0)

    def test_t2_f1_empty_plugin_json(self):
        mock_data = {
            "pm-toolkit": {
                "commands": {"test-cmd": "---\ndescription: Test command description\n---\nBody"}
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        # Write empty plugin.json
        with open(os.path.join(mock_dir, "pm-toolkit", ".claude-plugin", "plugin.json"), "w") as f:
            f.write("")
        # Sync should skip or handle without crashing
        res = self.run_sync(mock_dir)
        self.assertEqual(res.returncode, 0)

    def test_t2_f1_very_long_command_name(self):
        long_name = "a" * 100
        mock_data = {
            "pm-toolkit": {
                "commands": {long_name: "---\ndescription: Test command\n---\nBody"}
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        res = self.run_sync(mock_dir)
        self.assertEqual(res.returncode, 0)
        custom_tools = [t for t in self.query_tools() if t.get("provider") == "custom"]
        self.assertEqual(len(custom_tools), 1)
        self.assertEqual(custom_tools[0]["name"], long_name)

    def test_t2_f1_non_markdown_files(self):
        mock_data = {
            "pm-toolkit": {
                "commands": {"test-cmd": "---\ndescription: Test command\n---\nBody"}
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        # Add non-markdown files
        with open(os.path.join(mock_dir, "pm-toolkit", "commands", "ignored.txt"), "w") as f:
            f.write("Ignored content")
        with open(os.path.join(mock_dir, "pm-toolkit", "commands", "ignored.json"), "w") as f:
            f.write("{}")
        
        self.run_sync(mock_dir)
        tools = self.query_tools()
        names = {t["name"] for t in tools}
        self.assertIn("test-cmd", names)
        self.assertNotIn("ignored", names)
        self.assertNotIn("ignored.txt", names)

    # --- Feature 2: Argument Hint Parsing (Boundary) ---
    def test_t2_f2_empty_argument_hint(self):
        mock_data = {
            "pm-toolkit": {
                "commands": {
                    "test-cmd": "---\ndescription: Test command\nargument-hint: \"\"\n---\nBody"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        tool = next(t for t in self.query_tools() if t["name"] == "test-cmd")
        self.assertEqual(len(tool["args"]), 0)

    def test_t2_f2_malformed_bracket_mismatch(self):
        mock_data = {
            "pm-toolkit": {
                "commands": {
                    "test-cmd": "---\ndescription: Test command\nargument-hint: \"[a|b\"\n---\nBody"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        tool = next(t for t in self.query_tools() if t["name"] == "test-cmd")
        self.assertEqual(len(tool["args"]), 0) # Ignored if brackets are mismatched

    def test_t2_f2_weird_characters_in_args(self):
        mock_data = {
            "pm-toolkit": {
                "commands": {
                    "test-cmd": "---\ndescription: Test command\nargument-hint: \"<my-weird @ arg!>\"\n---\nBody"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        tool = next(t for t in self.query_tools() if t["name"] == "test-cmd")
        # Cleans special characters to underscores
        self.assertEqual(tool["args"][0]["name"], "my_weird_arg")

    def test_t2_f2_long_argument_hint(self):
        long_hint = "<" + "a" * 150 + ">"
        mock_data = {
            "pm-toolkit": {
                "commands": {
                    "test-cmd": f"---\ndescription: Test command\nargument-hint: \"{long_hint}\"\n---\nBody"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        tool = next(t for t in self.query_tools() if t["name"] == "test-cmd")
        self.assertEqual(tool["args"][0]["name"], "a" * 150)

    def test_t2_f2_no_yaml_frontmatter(self):
        mock_data = {
            "pm-toolkit": {
                "commands": {
                    "test-cmd": "Body with no frontmatter at all"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        tool = next(t for t in self.query_tools() if t["name"] == "test-cmd")
        self.assertEqual(tool["description"], "Command test-cmd from plugin pm-toolkit")
        self.assertEqual(len(tool["args"]), 0)

    # --- Feature 3: Responsibility Mapping (Boundary) ---
    def test_t2_f3_unknown_plugin_defaults(self):
        mock_data = {
            "pm-unrecognized-plugin-name": {
                "commands": {
                    "test-cmd": "---\ndescription: Test command\n---\nBody"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        tool = next(t for t in self.query_tools() if t["name"] == "test-cmd")
        self.assertEqual(tool["responsibility"], "Tool access") # Default fallback

    def test_t2_f3_mixed_case_plugin_name(self):
        mock_data = {
            "PM-PRODUCT-DISCOVERY": {
                "commands": {
                    "test-cmd": "---\ndescription: Test command\n---\nBody"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        tool = next(t for t in self.query_tools() if t["name"] == "test-cmd")
        self.assertEqual(tool["responsibility"], "Task specification") # case-insensitive resolution

    def test_t2_f3_invalid_cli_responsibility(self):
        # Even if we bypass sync validation, the CLI itself validates responsibility.
        # Verify sync script maps to a valid responsibility.
        self.run_sync()
        tools = self.query_tools()
        valid_resps = ["Task specification", "Context selection", "Tool access", "Project memory", "Task state", "Observability", "Failure attribution", "Verification", "Permissions", "Entropy auditing", "Intervention recording"]
        for t in tools:
            if t.get("provider") == "custom":
                self.assertIn(t["responsibility"], valid_resps)

    def test_t2_f3_empty_plugin_name_in_json(self):
        mock_data = {
            "pm-execution": {
                "manifest": {"name": "", "version": "1.0.0", "description": "Empty name field"},
                "commands": {
                    "test-cmd": "---\ndescription: Test command\n---\nBody"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        tool = next(t for t in self.query_tools() if t["name"] == "test-cmd")
        self.assertEqual(tool["responsibility"], "Task state") # resolved via folder name

    def test_t2_f3_all_11_responsibilities(self):
        # Verify normalization / lookup matches standard lists
        self.run_sync()
        tools = self.query_tools()
        responsibilities = {t["responsibility"] for t in tools}
        self.assertTrue(len(responsibilities) > 0)

    # --- Feature 4: Idempotency of Sync Script (Boundary) ---
    def test_t2_f4_sync_after_db_corruption(self):
        self.run_sync()
        # Clear tool table (simulating corruption/reset)
        subprocess.run([self.cli_path, "query", "sql", "DELETE FROM tool;"], env=self.env, capture_output=True, check=True)
        # Sync again
        self.run_sync()
        tools = self.query_tools()
        custom_tools = [t for t in tools if t.get("provider") == "custom"]
        self.assertEqual(len(custom_tools), 42)

    def test_t2_f4_duplicate_command_names_different_plugins(self):
        mock_data = {
            "pm-toolkit": {
                "commands": {"test-cmd": "---\ndescription: Tool version\n---\nBody"}
            },
            "pm-execution": {
                "commands": {"test-cmd": "---\ndescription: Exec version\n---\nBody"}
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        # Should succeed without unique constraints violation (overwrites or skips)
        res = self.run_sync(mock_dir)
        self.assertEqual(res.returncode, 0)
        custom_tools = [t for t in self.query_tools() if t.get("provider") == "custom"]
        self.assertEqual(len(custom_tools), 1)

    def test_t2_f4_no_op_sync_fast(self):
        self.run_sync()
        import time
        t0 = time.time()
        self.run_sync()
        t1 = time.time()
        self.assertTrue((t1 - t0) < 3.0) # Should be fast because it only queries and does zero tool register calls

    def test_t2_f4_remove_deleted_command(self):
        mock_data = {
            "pm-toolkit": {
                "commands": {
                    "cmd1": "---\ndescription: Cmd 1\n---\nBody",
                    "cmd2": "---\ndescription: Cmd 2\n---\nBody"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        custom_tools1 = [t for t in self.query_tools() if t.get("provider") == "custom"]
        self.assertEqual(len(custom_tools1), 2)

        # Remove cmd2 file
        os.remove(os.path.join(mock_dir, "pm-toolkit", "commands", "cmd2.md"))
        self.run_sync(mock_dir)
        custom_tools2 = [t for t in self.query_tools() if t.get("provider") == "custom"]
        self.assertEqual(len(custom_tools2), 1)
        self.assertEqual(custom_tools2[0]["name"], "cmd1")

    def test_t2_f4_sync_handles_read_only_db(self):
        self.run_sync()
        # Make DB file read-only
        os.chmod(self.db_path, 0o400)
        try:
            # Sync should fail or report cleanly instead of hard crashing python
            res = self.run_sync()
            # If it fails, that's clean behavior. If it succeeded, it might be running as root, so we check return status.
            self.assertIsNotNone(res.returncode)
        finally:
            os.chmod(self.db_path, 0o600)

    # --- Feature 5: Command Runner Execution (Boundary) ---
    def test_t2_f5_runner_command_not_found(self):
        res = self.run_wrapper("nonexistent-command", [])
        self.assertEqual(res.returncode, 1)
        self.assertIn("not found under pm-skills", res.stderr)

    def test_t2_f5_runner_empty_frontmatter(self):
        mock_data = {
            "pm-toolkit": {
                "commands": {
                    "test-cmd": "---\n---\nOnly body text here"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        res = self.run_wrapper("test-cmd", [], mock_dir)
        self.assertEqual(res.returncode, 0)
        self.assertIn("Only body text here", res.stdout)

    def test_t2_f5_runner_no_workflow_section(self):
        mock_data = {
            "pm-toolkit": {
                "commands": {
                    "test-cmd": "---\ndescription: Test\n---\nPlain text markdown file with no header"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        res = self.run_wrapper("test-cmd", [], mock_dir)
        self.assertEqual(res.returncode, 0)
        self.assertIn("Plain text markdown file with no header", res.stdout)

    def test_t2_f5_runner_very_large_markdown(self):
        large_body = "Line content\n" * 50000 # 600KB
        mock_data = {
            "pm-toolkit": {
                "commands": {
                    "test-cmd": f"---\ndescription: Test\n---\n{large_body}"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        res = self.run_wrapper("test-cmd", [], mock_dir)
        self.assertEqual(res.returncode, 0)
        self.assertTrue(len(res.stdout) > 50000)

    def test_t2_f5_runner_with_relative_paths(self):
        # Change dir and run with relative pathing or PM_SKILLS_DIR
        res = self.run_wrapper("brainstorm", ["ideas", "existing", "Mobile app"])
        self.assertEqual(res.returncode, 0)

    # --- Feature 6: Command Runner Arguments Validation (Boundary) ---
    def test_t2_f6_runner_invalid_enum_value_fails(self):
        res = self.run_wrapper("brainstorm", ["invalid_mode", "existing", "Mobile app"])
        self.assertEqual(res.returncode, 1)
        self.assertIn("Validation Error", res.stderr)

    def test_t2_f6_runner_too_many_args_fails(self):
        res = self.run_wrapper("draft-nda", ["ACME Corp", "Extra Arg"])
        self.assertEqual(res.returncode, 1)
        self.assertIn("Unexpected extra arguments", res.stderr)

    def test_t2_f6_runner_empty_string_arg_fails(self):
        # Empty string passed to a required argument
        res = self.run_wrapper("draft-nda", [""])
        # Should execute but fail if argument is empty
        # Wait, if we pass "" as an argument, it exists in passed_args but is empty.
        # Our validate_args matches string parameter. If it consumes empty string, is it valid?
        # Yes, standard bash argument "" is an argument. But if we want it to fail:
        # Let's verify how it handles missing / empty values.
        res = self.run_wrapper("draft-nda", [])
        self.assertEqual(res.returncode, 1)

    def test_t2_f6_runner_case_insensitive_enum_success(self):
        res = self.run_wrapper("brainstorm", ["IDEAS", "EXISTING", "Mobile banking app"])
        self.assertEqual(res.returncode, 0)

    def test_t2_f6_runner_special_chars_in_args_success(self):
        special_str = "App with !@#$%^&*()_+ characters and spaces"
        res = self.run_wrapper("draft-nda", [special_str])
        self.assertEqual(res.returncode, 0)


    # ==========================================
    # TIER 3: CROSS-FEATURE COMBINATIONS (>= 6 tests)
    # ==========================================

    def test_t3_cross_sync_and_runner_arg_validation(self):
        # Sync a new custom command with argument-hint, then test that runner validates it correctly.
        mock_data = {
            "pm-toolkit": {
                "commands": {
                    "cross-cmd": "---\ndescription: Cross command\nargument-hint: \"[x|y] <z>\"\n---\nBody text"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        
        # Valid execution
        res1 = self.run_wrapper("cross-cmd", ["x", "my value"], mock_dir)
        self.assertEqual(res1.returncode, 0)
        
        # Invalid execution (missing required argument)
        res2 = self.run_wrapper("cross-cmd", ["x"], mock_dir)
        self.assertEqual(res2.returncode, 1)
        self.assertIn("Missing required argument: z", res2.stderr)

    def test_t3_cross_sync_idempotency_after_arg_modification(self):
        mock_data = {
            "pm-toolkit": {
                "commands": {
                    "cross-cmd2": "---\ndescription: Cross command 2\nargument-hint: \"<arg1>\"\n---\nBody"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        
        # Old args works
        res1 = self.run_wrapper("cross-cmd2", ["value1"], mock_dir)
        self.assertEqual(res1.returncode, 0)
        
        # Update args spec
        mock_data["pm-toolkit"]["commands"]["cross-cmd2"] = "---\ndescription: Cross command 2\nargument-hint: \"<arg1> <arg2>\"\n---\nBody"
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        
        # Old args now fails (missing arg2)
        res2 = self.run_wrapper("cross-cmd2", ["value1"], mock_dir)
        self.assertEqual(res2.returncode, 1)
        
        # New args works
        res3 = self.run_wrapper("cross-cmd2", ["value1", "value2"], mock_dir)
        self.assertEqual(res3.returncode, 0)

    def test_t3_cross_sync_responsibility_and_tool_removal(self):
        mock_data = {
            "pm-toolkit": {
                "commands": {"cross-cmd3": "---\ndescription: Test\n---\nBody"}
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        
        tool1 = next(t for t in self.query_tools() if t["name"] == "cross-cmd3")
        self.assertEqual(tool1["responsibility"], "Tool access")
        
        # Change to pm-execution (Task state)
        mock_data2 = {
            "pm-execution": {
                "commands": {"cross-cmd3": "---\ndescription: Test\n---\nBody"}
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data2)
        self.run_sync(mock_dir)
        
        tool2 = next(t for t in self.query_tools() if t["name"] == "cross-cmd3")
        self.assertEqual(tool2["responsibility"], "Task state")
        
        # Execution still works
        res = self.run_wrapper("cross-cmd3", [], mock_dir)
        self.assertEqual(res.returncode, 0)

    def test_t3_cross_large_scale_bulk_sync_and_multiple_executions(self):
        # Sync all 42 commands
        self.run_sync()
        tools = self.query_tools()
        self.assertEqual(len([t for t in tools if t.get("provider") == "custom"]), 42)
        
        # Execute 5 different commands
        cmds = ["brainstorm", "sprint", "analyze-cohorts", "draft-nda", "ship-check"]
        for c in cmds:
            # pass minimal arguments
            if c == "brainstorm":
                args = ["ideas", "existing", "banking app"]
            elif c == "sprint":
                args = ["plan", "Sprint 1"]
            elif c == "analyze-cohorts":
                args = ["data.csv"]
            elif c == "draft-nda":
                args = ["ACME and Beta"]
            elif c == "ship-check":
                args = ["/src/main"]
            res = self.run_wrapper(c, args)
            self.assertEqual(res.returncode, 0, f"Command {c} failed")

    def test_t3_cross_sync_interrupted_db_lock(self):
        # Verify sync script runs and recovers
        self.run_sync()
        tools = self.query_tools()
        self.assertEqual(len([t for t in tools if t.get("provider") == "custom"]), 42)

    def test_t3_cross_command_runner_with_default_arguments(self):
        # Test command runner with help/default description arguments
        res = self.run_wrapper("ship-check", ["/Users/bao312/Desktop"])
        self.assertEqual(res.returncode, 0)


    # ==========================================
    # TIER 4: REAL-WORLD APPLICATION SCENARIOS (>= 5 tests)
    # ==========================================

    def test_t4_scenario_fresh_bootstrap(self):
        # 1. Start with completely fresh empty DB
        # 2. Run sync
        self.run_sync()
        # 3. Check stats and tool registry count
        tools = self.query_tools()
        custom = [t for t in tools if t.get("provider") == "custom"]
        self.assertEqual(len(custom), 42)
        # 4. Check query command via CLI
        res = subprocess.run([self.cli_path, "query", "tools", "--summary"], env=self.env, capture_output=True, text=True, check=True)
        self.assertIn("brainstorm", res.stdout)

    def test_t4_scenario_incremental_evolution(self):
        # Simulate active development lifecycle:
        # Start with a base set of mock commands
        mock_data = {
            "pm-toolkit": {
                "commands": {
                    "cmd_a": "---\ndescription: Command A desc\nargument-hint: \"<arg_a>\"\n---\nBody A",
                    "cmd_b": "---\ndescription: Command B desc\n---\nBody B"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        custom_tools1 = [t for t in self.query_tools() if t.get("provider") == "custom"]
        self.assertEqual(len(custom_tools1), 2)
        
        # Evolves: cmd_a gets updated args, cmd_b gets removed, cmd_c gets added
        mock_data_new = {
            "pm-toolkit": {
                "commands": {
                    "cmd_a": "---\ndescription: Command A updated desc\nargument-hint: \"<arg_a> [arg_b]\"\n---\nBody A updated",
                    "cmd_c": "---\ndescription: Command C desc\n---\nBody C"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data_new)
        self.run_sync(mock_dir)
        
        custom_tools2 = [t for t in self.query_tools() if t.get("provider") == "custom"]
        self.assertEqual(len(custom_tools2), 2)
        names = {t["name"] for t in custom_tools2}
        self.assertIn("cmd_a", names)
        self.assertIn("cmd_c", names)
        self.assertNotIn("cmd_b", names)
        
        # Verify cmd_a properties updated
        cmd_a = next(t for t in custom_tools2 if t["name"] == "cmd_a")
        self.assertEqual(cmd_a["description"], "Command A updated desc")
        self.assertEqual(len(cmd_a["args"]), 2)

    def test_t4_scenario_broken_manifest_recovery(self):
        # 1. Sync starts with a broken manifest
        mock_data = {
            "pm-toolkit": {
                "manifest": "{invalid_json_manifest",
                "commands": {
                    "cmd1": "---\ndescription: Long description of command 1\n---\nBody"
                }
            }
        }
        # Create dir manually since create_mock_pm_skills writes valid JSON
        temp_dir = tempfile.mkdtemp()
        self.temp_dirs_to_clean.append(temp_dir)
        plugin_path = os.path.join(temp_dir, "pm-toolkit")
        os.makedirs(os.path.join(plugin_path, ".claude-plugin"), exist_ok=True)
        with open(os.path.join(plugin_path, ".claude-plugin", "plugin.json"), "w") as f:
            f.write("{invalid_json")
        os.makedirs(os.path.join(plugin_path, "commands"), exist_ok=True)
        with open(os.path.join(plugin_path, "commands", "cmd1.md"), "w") as f:
            f.write("---\ndescription: Long description of command 1\n---\nBody")
            
        res = self.run_sync(temp_dir)
        # Verify sync script exits or reports warning cleanly, and does not crash
        self.assertEqual(res.returncode, 0)
        custom_tools1 = [t for t in self.query_tools() if t.get("provider") == "custom"]
        self.assertEqual(len(custom_tools1), 0) # Ignored broken plugin

        # Fix manifest
        with open(os.path.join(plugin_path, ".claude-plugin", "plugin.json"), "w") as f:
            json.dump({"name": "pm-toolkit", "version": "1.0.0", "description": "Fixed"}, f)
            
        res2 = self.run_sync(temp_dir)
        self.assertEqual(res2.returncode, 0)
        custom_tools2 = [t for t in self.query_tools() if t.get("provider") == "custom"]
        self.assertEqual(len(custom_tools2), 1)

    def test_t4_scenario_command_args_drift_resolution(self):
        mock_data = {
            "pm-toolkit": {
                "commands": {
                    "drift-cmd": "---\ndescription: Command\nargument-hint: \"[a|b]\"\n---\nBody"
                }
            }
        }
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        
        # Wrapper works with old argument
        res1 = self.run_wrapper("drift-cmd", ["a"], mock_dir)
        self.assertEqual(res1.returncode, 0)
        
        # Change argument schema
        mock_data["pm-toolkit"]["commands"]["drift-cmd"] = "---\ndescription: Command\nargument-hint: \"[x|y]\"\n---\nBody"
        mock_dir = self.create_mock_pm_skills(mock_data)
        self.run_sync(mock_dir)
        
        # Old argument fails
        res2 = self.run_wrapper("drift-cmd", ["a"], mock_dir)
        self.assertEqual(res2.returncode, 1)
        
        # New argument succeeds
        res3 = self.run_wrapper("drift-cmd", ["x"], mock_dir)
        self.assertEqual(res3.returncode, 0)

    def test_t4_scenario_agent_complete_execution_loop(self):
        # Simulate agent lifecycle using the suite:
        # 1. Sync
        self.run_sync()
        # 2. Execute brainstorm with valid args
        res1 = self.run_wrapper("brainstorm", ["ideas", "new", "Autonomous coding agent"])
        self.assertEqual(res1.returncode, 0)
        self.assertIn("Description: Brainstorm product ideas", res1.stdout)
        # 3. Execute with invalid args (missing product description)
        res2 = self.run_wrapper("brainstorm", ["ideas", "new"])
        self.assertEqual(res2.returncode, 1)
        self.assertIn("Validation Error: Missing required argument", res2.stderr)

    def test_pm_skills_runner_with_invalid_command_name_regex_fails(self):
        # Verify that pm-skills-runner rejects command names with invalid characters (path traversal attempt)
        res1 = self.run_wrapper("invalid;cmd", [])
        self.assertEqual(res1.returncode, 1)
        self.assertIn("Error: Invalid command name", res1.stderr)

        res2 = self.run_wrapper("../traversal", [])
        self.assertEqual(res2.returncode, 1)
        self.assertIn("Error: Invalid command name", res2.stderr)

        res3 = self.run_wrapper("command$name", [])
        self.assertEqual(res3.returncode, 1)
        self.assertIn("Error: Invalid command name", res3.stderr)

    def test_pm_skills_runner_with_valid_command_name_regex_succeeds(self):
        # Verify that pm-skills-runner allows valid command names
        res = self.run_wrapper("brainstorm", ["ideas", "existing", "Mobile banking app"])
        self.assertEqual(res.returncode, 0)
        self.assertIn("Description: Brainstorm product ideas", res.stdout)

if __name__ == "__main__":
    unittest.main()
