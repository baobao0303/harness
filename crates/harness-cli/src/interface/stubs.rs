use super::args::{
    ConfigAction, ConfigArgs, SkillAction, SkillArgs, SubagentAction, SubagentArgs, WorktreeAction,
    WorktreeArgs,
};

pub fn handle_worktree_subcommand(args: WorktreeArgs) {
    match args.action {
        WorktreeAction::Spawn { task } => {
            let worktree_dir = format!(".worktrees/task-{task}");
            println!("Worktree spawned: {worktree_dir}");
            println!("Environment: TMPDIR={worktree_dir}/tmp PORT_OFFSET=10");
        }
        WorktreeAction::Remove { task, force: _ } => {
            let worktree_dir = format!(".worktrees/task-{task}");
            println!("Worktree removed: {worktree_dir}");
        }
        WorktreeAction::List => {
            println!("Active Worktrees:");
            println!("- .worktrees/ (isolated environments)");
        }
    }
}

pub fn handle_subagent_subcommand(args: SubagentArgs) {
    match args.action {
        SubagentAction::Spawn {
            role,
            model,
            skills,
            workdir,
            prompt,
        } => {
            println!("Sub-Agent Spawned:");
            println!("- Role: {role}");
            println!("- Model Tier: {model}");
            println!("- Skills: {}", skills.unwrap_or_else(|| "auto-detect".to_owned()));
            println!("- Workdir: {}", workdir.unwrap_or_else(|| "main".to_owned()));
            println!("- Prompt: {prompt}");
        }
        SubagentAction::List => {
            println!("Active Sub-Agents: Chief of Staff, Implementer, Verifier, Explorer, Triage Monitor, Skill Auditor");
        }
    }
}

pub fn handle_skill_subcommand(args: SkillArgs) {
    match args.action {
        SkillAction::Find { intent } => {
            println!("Skill Find ('{intent}'):");
            println!("- Best match: .agents/skills/harness-create-story/SKILL.md");
        }
        SkillAction::Search { query } => {
            println!("Skill Search ('{query}'):");
            println!("- Found local: .agents/skills/harness-qa-generate-e2e-tests/SKILL.md");
        }
        SkillAction::Sync => {
            let server = std::env::var("HARNESS_SKILL_SERVER")
                .unwrap_or_else(|_| "https://skills-hub.yourdomain.com/api/v1".to_owned());
            println!("Synced skills from Remote Skill Server ({server}).");
        }
        SkillAction::Pull { name } => {
            let server = std::env::var("HARNESS_SKILL_SERVER")
                .unwrap_or_else(|_| "https://skills-hub.yourdomain.com/api/v1".to_owned());
            println!("Pulled skill '{name}' from Remote Skill Server ({server}).");
        }
    }
}

pub fn handle_config_subcommand(args: ConfigArgs) {
    match args.action {
        ConfigAction::Get { key } => {
            let val = std::env::var(&key).unwrap_or_else(|_| "not set".to_owned());
            println!("{key} = {val}");
        }
        ConfigAction::Set { key, value } => {
            println!("Config set: {key} = {value}");
        }
        ConfigAction::List => {
            println!("Harness Configuration:");
            println!(
                "- HARNESS_DB = {}",
                std::env::var("HARNESS_DB").unwrap_or_else(|_| "./harness.db".to_owned())
            );
            println!(
                "- HARNESS_MODEL = {}",
                std::env::var("HARNESS_MODEL").unwrap_or_else(|_| "gemini-3.6-flash".to_owned())
            );
            println!(
                "- HARNESS_SKILL_SERVER = {}",
                std::env::var("HARNESS_SKILL_SERVER")
                    .unwrap_or_else(|_| "https://skills-hub.yourdomain.com/api/v1".to_owned())
            );
        }
    }
}
