use clap::{Args, Parser, Subcommand};

use crate::domain::RISK_LANE_HELP;

#[derive(Parser, Debug)]
#[command(name = "harness-cli")]
#[command(about = "durable layer for the project harness", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Create the harness database if it does not already exist.
    Init,
    /// Apply schema migrations.
    Migrate,
    /// Seed or refresh the database from existing markdown state.
    Import(ImportArgs),
    /// Record a feature intake classification.
    Intake(IntakeArgs),
    /// Add or update a story.
    Story(StoryArgs),
    /// Manage Company Work Items (Epic, Feature, User Story, Technical Story, Task, Bug, Testcase).
    #[command(name = "work-item")]
    WorkItem(WorkItemArgs),
    /// Add a decision or run its verification.
    Decision(DecisionArgs),
    /// Add or close a backlog item.
    Backlog(BacklogArgs),
    /// Register or remove external tools.
    Tool(ToolArgs),
    /// Record a human, review, CI, or agent intervention.
    Intervention(InterventionArgs),
    /// Record an agent execution trace.
    Trace(TraceArgs),
    /// Score a trace against the trace quality tiers.
    ScoreTrace(ScoreTraceArgs),
    /// Score trace context reads against CONTEXT_RULES.md.
    ScoreContext { trace_id: String },
    /// Run drift audit and entropy score.
    Audit,
    /// Generate improvement proposals from observed patterns.
    Propose(ProposeArgs),
    /// Query harness data.
    Query(QueryArgs),
    /// Export execution trace graph (tldraw or mermaid format).
    ExportTrace(ExportTraceArgs),
    /// Manage Git Worktrees for isolated task execution.
    Worktree(WorktreeArgs),
    /// Spawn and manage sub-agents.
    Subagent(SubagentArgs),
    /// Discover and sync skills.
    Skill(SkillArgs),
    /// Inspect or set harness configuration.
    Config(ConfigArgs),
}

#[derive(Args, Debug)]
#[command(after_help = RISK_LANE_HELP)]
pub struct IntakeArgs {
    #[arg(long = "type")]
    pub input_type: String,
    #[arg(long)]
    pub summary: String,
    #[arg(long, value_name = "tiny|normal|high-risk")]
    pub lane: String,
    #[arg(long)]
    pub flags: Option<String>,
    #[arg(long)]
    pub docs: Option<String>,
    #[arg(long)]
    pub story: Option<String>,
    #[arg(long)]
    pub notes: Option<String>,
}

#[derive(Args, Debug)]
pub struct ImportArgs {
    #[command(subcommand)]
    pub source: ImportSource,
}

#[derive(Subcommand, Debug)]
pub enum ImportSource {
    /// Import TEST_MATRIX, decisions, and backlog markdown.
    Brownfield,
}

#[derive(Args, Debug)]
pub struct StoryArgs {
    #[command(subcommand)]
    pub action: StoryAction,
}

#[derive(Subcommand, Debug)]
pub enum StoryAction {
    #[command(after_help = RISK_LANE_HELP)]
    Add(StoryAddArgs),
    #[command(
        after_help = "Proof flags use numeric booleans: --unit 1 --integration 1 --e2e 0 --platform 0. Do not use yes/no."
    )]
    Update(StoryUpdateArgs),
    #[command(
        after_help = "story verify only accepts the story id. Configure proof with story add/update --verify, then record proof flags with story update."
    )]
    Verify {
        /// Story id to verify.
        id: String,
    },
    /// Verify every story, skipping stories without verify_command.
    VerifyAll,
}

#[derive(Args, Debug)]
pub struct StoryAddArgs {
    #[arg(long)]
    pub id: String,
    #[arg(long)]
    pub title: String,
    #[arg(long, value_name = "tiny|normal|high-risk")]
    pub lane: String,
    #[arg(long)]
    pub contract: Option<String>,
    #[arg(long)]
    pub verify: Option<String>,
    #[arg(long)]
    pub notes: Option<String>,
    #[arg(long, default_value = "p2", value_name = "p0|p1|p2|p3")]
    pub priority: String,
}

#[derive(Args, Debug)]
pub struct StoryUpdateArgs {
    #[arg(long)]
    pub id: String,
    #[arg(long)]
    pub status: Option<String>,
    #[arg(long)]
    pub evidence: Option<String>,
    #[arg(long, value_name = "0|1")]
    pub unit: Option<String>,
    #[arg(long, value_name = "0|1")]
    pub integration: Option<String>,
    #[arg(long, value_name = "0|1")]
    pub e2e: Option<String>,
    #[arg(long, value_name = "0|1")]
    pub platform: Option<String>,
    #[arg(long)]
    pub verify: Option<String>,
    #[arg(long, value_name = "p0|p1|p2|p3")]
    pub priority: Option<String>,
}

#[derive(Args, Debug)]
pub struct DecisionArgs {
    #[command(subcommand)]
    pub action: DecisionAction,
}

#[derive(Subcommand, Debug)]
pub enum DecisionAction {
    Add(DecisionAddArgs),
    Verify { id: String },
}

#[derive(Args, Debug)]
pub struct DecisionAddArgs {
    #[arg(long)]
    pub id: String,
    #[arg(long)]
    pub title: String,
    #[arg(long, default_value = "accepted")]
    pub status: String,
    #[arg(long)]
    pub doc: Option<String>,
    #[arg(long)]
    pub verify: Option<String>,
    #[arg(long)]
    pub predicted: Option<String>,
    #[arg(long)]
    pub notes: Option<String>,
}

#[derive(Args, Debug)]
pub struct BacklogArgs {
    #[command(subcommand)]
    pub action: BacklogAction,
}

#[derive(Subcommand, Debug)]
pub enum BacklogAction {
    #[command(after_help = RISK_LANE_HELP)]
    Add(BacklogAddArgs),
    Close(BacklogCloseArgs),
}

#[derive(Args, Debug)]
pub struct BacklogAddArgs {
    #[arg(long)]
    pub title: String,
    #[arg(long = "while")]
    pub discovered_while: Option<String>,
    #[arg(long)]
    pub pain: Option<String>,
    #[arg(long)]
    pub suggestion: Option<String>,
    #[arg(long, value_name = "tiny|normal|high-risk")]
    pub risk: Option<String>,
    #[arg(long)]
    pub predicted: Option<String>,
    #[arg(long)]
    pub notes: Option<String>,
    #[arg(long, default_value = "p2", value_name = "p0|p1|p2|p3")]
    pub priority: String,
}

#[derive(Args, Debug)]
pub struct BacklogCloseArgs {
    #[arg(long)]
    pub id: String,
    #[arg(long, default_value = "implemented")]
    pub status: String,
    #[arg(long)]
    pub outcome: Option<String>,
}

#[derive(Args, Debug)]
pub struct ToolArgs {
    #[command(subcommand)]
    pub action: ToolAction,
}

#[derive(Subcommand, Debug)]
pub enum ToolAction {
    Register(ToolRegisterArgs),
    Remove {
        #[arg(long)]
        name: String,
    },
}

#[derive(Args, Debug)]
pub struct ToolRegisterArgs {
    #[arg(long)]
    pub name: String,
    #[arg(long)]
    pub command: String,
    #[arg(long)]
    pub description: String,
    #[arg(long)]
    pub responsibility: String,
    #[arg(long)]
    pub args: Option<String>,
    #[arg(long)]
    pub force: bool,
}

#[derive(Args, Debug)]
pub struct InterventionArgs {
    #[command(subcommand)]
    pub action: InterventionAction,
}

#[derive(Subcommand, Debug)]
pub enum InterventionAction {
    Add(InterventionAddArgs),
}

#[derive(Args, Debug)]
pub struct InterventionAddArgs {
    #[arg(long)]
    pub trace: Option<String>,
    #[arg(long)]
    pub story: Option<String>,
    #[arg(long = "type")]
    pub intervention_type: String,
    #[arg(long)]
    pub description: String,
    #[arg(long)]
    pub source: String,
    #[arg(long)]
    pub impact: Option<String>,
}

#[derive(Args, Debug)]
pub struct TraceArgs {
    #[arg(long)]
    pub summary: String,
    #[arg(long)]
    pub intake: Option<String>,
    #[arg(long)]
    pub story: Option<String>,
    #[arg(long)]
    pub agent: Option<String>,
    #[arg(long)]
    pub outcome: Option<String>,
    #[arg(long)]
    pub duration: Option<String>,
    #[arg(long)]
    pub tokens: Option<String>,
    #[arg(long)]
    pub friction: Option<String>,
    #[arg(long)]
    pub actions: Option<String>,
    #[arg(long = "read")]
    pub files_read: Option<String>,
    #[arg(long = "changed")]
    pub files_changed: Option<String>,
    #[arg(long)]
    pub decisions: Option<String>,
    #[arg(long)]
    pub errors: Option<String>,
    #[arg(long)]
    pub notes: Option<String>,
}

#[derive(Args, Debug)]
pub struct ScoreTraceArgs {
    /// Score a specific trace id. Defaults to the latest trace.
    #[arg(long)]
    pub id: Option<String>,
}

#[derive(Args, Debug)]
pub struct ProposeArgs {
    #[arg(long)]
    pub commit: bool,
}

#[derive(Args, Debug)]
pub struct QueryArgs {
    #[command(subcommand)]
    pub view: QueryView,
}

#[derive(Args, Debug)]
pub struct MatrixQueryArgs {
    /// Render proof flags as CLI input values, 1 and 0, instead of yes and no.
    #[arg(long)]
    pub numeric: bool,
}

#[derive(Args, Debug)]
pub struct BacklogQueryArgs {
    /// Show only proposed and accepted backlog items.
    #[arg(long, conflicts_with = "closed")]
    pub open: bool,
    /// Show only implemented and rejected backlog items.
    #[arg(long)]
    pub closed: bool,
}

#[derive(Subcommand, Debug)]
pub enum QueryView {
    /// Test matrix.
    Matrix(MatrixQueryArgs),
    /// Harness improvement proposals.
    Backlog(BacklogQueryArgs),
    /// Decision records.
    Decisions,
    /// Recent intake classifications.
    Intakes,
    /// Recent traces.
    Traces,
    /// Traces with harness friction.
    Friction,
    /// Machine-readable and registered tool manifest.
    Tools(ToolsQueryArgs),
    /// Intervention records.
    Interventions(InterventionsQueryArgs),
    /// Summary counts.
    Stats,
    /// Run arbitrary SQL.
    Sql { query: Vec<String> },
}

#[derive(Args, Debug)]
pub struct ToolsQueryArgs {
    #[arg(long)]
    pub json: bool,
    #[arg(long)]
    pub summary: bool,
    #[arg(long)]
    pub responsibility: Option<String>,
}

#[derive(Args, Debug)]
pub struct InterventionsQueryArgs {
    #[arg(long)]
    pub trace: Option<String>,
    #[arg(long)]
    pub story: Option<String>,
    #[arg(long = "type")]
    pub intervention_type: Option<String>,
}

#[derive(Args, Debug)]
pub struct ExportTraceArgs {
    #[arg(long, default_value = "tldraw", value_name = "tldraw|mermaid")]
    pub format: String,
    #[arg(long)]
    pub out: Option<String>,
}

#[derive(Args, Debug)]
pub struct WorktreeArgs {
    #[command(subcommand)]
    pub action: WorktreeAction,
}

#[derive(Subcommand, Debug)]
pub enum WorktreeAction {
    /// Spawn a new Git Worktree for a task.
    Spawn {
        #[arg(long)]
        task: String,
    },
    /// Remove an existing Git Worktree.
    Remove {
        #[arg(long)]
        task: String,
        #[arg(long, default_value_t = true)]
        force: bool,
    },
    /// List active Worktrees.
    List,
}

#[derive(Args, Debug)]
pub struct SubagentArgs {
    #[command(subcommand)]
    pub action: SubagentAction,
}

#[derive(Subcommand, Debug)]
pub enum SubagentAction {
    /// Spawn a sub-agent with role and model allocation.
    Spawn {
        #[arg(long)]
        role: String,
        #[arg(long, default_value = "flash", value_name = "flash|pro|inherit")]
        model: String,
        #[arg(long)]
        skills: Option<String>,
        #[arg(long)]
        workdir: Option<String>,
        #[arg(long)]
        prompt: String,
    },
    /// List sub-agents.
    List,
}

#[derive(Args, Debug)]
pub struct SkillArgs {
    #[command(subcommand)]
    pub action: SkillAction,
}

#[derive(Subcommand, Debug)]
pub enum SkillAction {
    /// Find best matching skill for an intent.
    Find { intent: String },
    /// Search skills by query.
    Search { query: String },
    /// Sync skills from Remote Skill Server.
    Sync,
    /// Pull a specific skill from Remote Skill Server.
    Pull { name: String },
}

#[derive(Args, Debug)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub action: ConfigAction,
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Get a configuration value.
    Get { key: String },
    /// Set a configuration value.
    Set { key: String, value: String },
    /// List configuration settings.
    List,
}

#[derive(Args, Debug)]
pub struct WorkItemArgs {
    #[command(subcommand)]
    pub action: WorkItemAction,
}

#[derive(Subcommand, Debug)]
pub enum WorkItemAction {
    /// Create a new work item.
    Add(WorkItemAddArgs),
    /// Update an existing work item.
    Update(WorkItemUpdateArgs),
    /// List work items with optional filtering by type and state.
    List(WorkItemListArgs),
    /// Show details of a work item by ID.
    Show {
        /// Work item ID.
        #[arg(long)]
        id: i64,
    },
}

#[derive(Args, Debug)]
pub struct WorkItemAddArgs {
    #[arg(long = "type")]
    pub work_type: String,
    #[arg(long)]
    pub title: String,
    #[arg(long)]
    pub description: Option<String>,
    #[arg(long)]
    pub assigned_to: Option<String>,
    #[arg(long)]
    pub story_points: Option<i64>,
    #[arg(long)]
    pub remaining_work: Option<f64>,
    #[arg(long, default_value = "p2", value_name = "p0|p1|p2|p3")]
    pub priority: String,
    #[arg(long)]
    pub severity: Option<String>,
    #[arg(long)]
    pub parent_id: Option<i64>,
    #[arg(long)]
    pub area_path: Option<String>,
    #[arg(long)]
    pub iteration_path: Option<String>,
    #[arg(long)]
    pub tags: Option<String>,
    #[arg(long)]
    pub acceptance_criteria: Option<String>,
    #[arg(long)]
    pub repro_steps: Option<String>,
    #[arg(long)]
    pub actual_result: Option<String>,
    #[arg(long)]
    pub expected_result: Option<String>,
    #[arg(long)]
    pub steps: Option<String>,
}

#[derive(Args, Debug)]
pub struct WorkItemUpdateArgs {
    #[arg(long)]
    pub id: i64,
    #[arg(long)]
    pub state: Option<String>,
    #[arg(long)]
    pub title: Option<String>,
    #[arg(long)]
    pub description: Option<String>,
    #[arg(long)]
    pub assigned_to: Option<String>,
    #[arg(long)]
    pub story_points: Option<i64>,
    #[arg(long)]
    pub remaining_work: Option<f64>,
    #[arg(long)]
    pub priority: Option<String>,
    #[arg(long)]
    pub severity: Option<String>,
    #[arg(long)]
    pub parent_id: Option<i64>,
    #[arg(long)]
    pub area_path: Option<String>,
    #[arg(long)]
    pub iteration_path: Option<String>,
    #[arg(long)]
    pub tags: Option<String>,
    #[arg(long)]
    pub acceptance_criteria: Option<String>,
    #[arg(long)]
    pub repro_steps: Option<String>,
    #[arg(long)]
    pub actual_result: Option<String>,
    #[arg(long)]
    pub expected_result: Option<String>,
    #[arg(long)]
    pub steps: Option<String>,
}

#[derive(Args, Debug)]
pub struct WorkItemListArgs {
    #[arg(long = "type")]
    pub work_type: Option<String>,
    #[arg(long)]
    pub state: Option<String>,
}
