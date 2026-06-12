use std::fmt;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseHarnessValueError {
    #[error("unknown intake type '{0}'. Use: new spec, spec slice, change request, new initiative, maintenance request, or harness improvement")]
    InputType(String),
    #[error("unknown lane '{0}'. Use: tiny, normal, or high-risk. Use tiny instead of low.")]
    RiskLane(String),
    #[error("{0} must be an integer")]
    Integer(String),
    #[error("{0} must be 0 or 1. Example: --unit 1 --integration 1 --e2e 0 --platform 0")]
    BoolFlag(String),
    #[error("unknown priority '{0}'. Use: p0, p1, p2, or p3.")]
    Priority(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputType {
    NewSpec,
    SpecSlice,
    ChangeRequest,
    NewInitiative,
    Maintenance,
    HarnessImprovement,
}

impl InputType {
    pub fn as_db_value(&self) -> &'static str {
        match self {
            Self::NewSpec => "new_spec",
            Self::SpecSlice => "spec_slice",
            Self::ChangeRequest => "change_request",
            Self::NewInitiative => "new_initiative",
            Self::Maintenance => "maintenance",
            Self::HarnessImprovement => "harness_improvement",
        }
    }
}

impl FromStr for InputType {
    type Err = ParseHarnessValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let normalized = normalize_token(value);
        match normalized.as_str() {
            "new_spec" => Ok(Self::NewSpec),
            "spec_slice" => Ok(Self::SpecSlice),
            "change_request" => Ok(Self::ChangeRequest),
            "new_initiative" => Ok(Self::NewInitiative),
            "maintenance" | "maintenance_request" => Ok(Self::Maintenance),
            "harness_improvement" => Ok(Self::HarnessImprovement),
            _ => Err(ParseHarnessValueError::InputType(value.to_owned())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RiskLane {
    Tiny,
    Normal,
    HighRisk,
}

impl RiskLane {
    pub fn as_db_value(&self) -> &'static str {
        match self {
            Self::Tiny => "tiny",
            Self::Normal => "normal",
            Self::HighRisk => "high_risk",
        }
    }
}

impl FromStr for RiskLane {
    type Err = ParseHarnessValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let normalized = normalize_token(value);
        match normalized.as_str() {
            "tiny" => Ok(Self::Tiny),
            "normal" => Ok(Self::Normal),
            "high_risk" => Ok(Self::HighRisk),
            _ => Err(ParseHarnessValueError::RiskLane(value.to_owned())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Priority {
    P0,
    P1,
    P2,
    P3,
}

impl Priority {
    pub fn as_db_value(&self) -> &'static str {
        match self {
            Self::P0 => "P0",
            Self::P1 => "P1",
            Self::P2 => "P2",
            Self::P3 => "P3",
        }
    }
}

impl FromStr for Priority {
    type Err = ParseHarnessValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let normalized = normalize_token(value);
        match normalized.as_str() {
            "p0" => Ok(Self::P0),
            "p1" => Ok(Self::P1),
            "p2" => Ok(Self::P2),
            "p3" => Ok(Self::P3),
            _ => Err(ParseHarnessValueError::Priority(value.to_owned())),
        }
    }
}

pub const RISK_LANE_HELP: &str =
    "Accepted lanes: tiny, normal, high-risk. Use tiny instead of low.";

pub const RESPONSIBILITIES: &[&str] = &[
    "Task specification",
    "Context selection",
    "Tool access",
    "Project memory",
    "Task state",
    "Observability",
    "Failure attribution",
    "Verification",
    "Permissions",
    "Entropy auditing",
    "Intervention recording",
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToolArgSpec {
    pub name: String,
    pub arg_type: String,
    pub required: bool,
    pub help: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToolEntry {
    pub provider: String,
    pub name: String,
    pub command: String,
    pub description: String,
    pub args: Vec<ToolArgSpec>,
    pub responsibility: String,
    pub source: String,
    pub since: String,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ToolValidationError {
    #[error("--description must be 10-200 characters")]
    DescriptionLength,
    #[error("unknown responsibility '{0}'. Use: {1}")]
    Responsibility(String, String),
    #[error("invalid --args spec '{0}'. Use name:type:required or name:type:required:help")]
    ArgSpec(String),
}

pub fn parse_tool_args(value: Option<String>) -> Result<Vec<ToolArgSpec>, ToolValidationError> {
    let Some(value) = value else {
        return Ok(Vec::new());
    };
    if value.trim().is_empty() {
        return Ok(Vec::new());
    }

    value
        .split(',')
        .map(|raw| {
            let parts = raw.splitn(4, ':').map(str::trim).collect::<Vec<_>>();
            if parts.len() < 3
                || parts[0].is_empty()
                || parts[1].is_empty()
                || !matches!(parts[2], "required" | "optional")
            {
                return Err(ToolValidationError::ArgSpec(raw.to_owned()));
            }
            Ok(ToolArgSpec {
                name: parts[0].to_owned(),
                arg_type: parts[1].to_owned(),
                required: parts[2] == "required",
                help: parts
                    .get(3)
                    .filter(|value| !value.is_empty())
                    .map(|value| value.to_string()),
            })
        })
        .collect()
}

pub fn validate_tool_description(description: &str) -> Result<(), ToolValidationError> {
    let length = description.trim().chars().count();
    if !(10..=200).contains(&length) {
        return Err(ToolValidationError::DescriptionLength);
    }
    Ok(())
}

pub fn validate_responsibility(value: &str) -> Result<String, ToolValidationError> {
    RESPONSIBILITIES
        .iter()
        .find(|item| normalize_token(item) == normalize_token(value))
        .map(|item| (*item).to_owned())
        .ok_or_else(|| {
            ToolValidationError::Responsibility(value.to_owned(), RESPONSIBILITIES.join(", "))
        })
}

pub fn compiled_tool_registry() -> Vec<ToolEntry> {
    vec![
        tool(
            "harness-cli",
            "init",
            "init",
            "Create the harness database.",
            &[],
            "Task state",
            "0.1.0",
        ),
        tool(
            "harness-cli",
            "migrate",
            "migrate",
            "Apply pending schema migrations.",
            &[],
            "Task state",
            "0.1.0",
        ),
        tool(
            "harness-cli",
            "import brownfield",
            "import brownfield",
            "Seed durable records from markdown state.",
            &[],
            "Project memory",
            "0.1.0",
        ),
        tool(
            "harness-cli",
            "intake",
            "intake",
            "Record a feature intake classification.",
            &[
                ("type", "string", true),
                ("summary", "string", true),
                ("lane", "enum", true),
            ],
            "Task specification",
            "0.1.0",
        ),
        tool(
            "harness-cli",
            "story add",
            "story add",
            "Create a durable story record.",
            &[
                ("id", "string", true),
                ("title", "string", true),
                ("lane", "enum", true),
            ],
            "Task state",
            "0.1.0",
        ),
        tool(
            "harness-cli",
            "story update",
            "story update",
            "Update story status, proof flags, or verification command.",
            &[("id", "string", true)],
            "Task state",
            "0.1.0",
        ),
        tool(
            "harness-cli",
            "story verify",
            "story verify",
            "Run one story verify_command and record pass or fail.",
            &[("id", "string", true)],
            "Verification",
            "0.1.6",
        ),
        tool(
            "harness-cli",
            "story verify-all",
            "story verify-all",
            "Run every configured story verification command.",
            &[],
            "Verification",
            "0.1.8",
        ),
        tool(
            "harness-cli",
            "decision add",
            "decision add",
            "Create a durable decision record.",
            &[("id", "string", true), ("title", "string", true)],
            "Project memory",
            "0.1.0",
        ),
        tool(
            "harness-cli",
            "decision verify",
            "decision verify",
            "Run one decision verification command.",
            &[("id", "string", true)],
            "Verification",
            "0.1.6",
        ),
        tool(
            "harness-cli",
            "backlog add",
            "backlog add",
            "Record a harness improvement proposal.",
            &[("title", "string", true)],
            "Entropy auditing",
            "0.1.0",
        ),
        tool(
            "harness-cli",
            "backlog close",
            "backlog close",
            "Close a backlog item with outcome evidence.",
            &[("id", "integer", true)],
            "Entropy auditing",
            "0.1.0",
        ),
        tool(
            "harness-cli",
            "trace",
            "trace",
            "Record an agent execution trace.",
            &[("summary", "string", true)],
            "Observability",
            "0.1.0",
        ),
        tool(
            "harness-cli",
            "score-trace",
            "score-trace",
            "Score trace detail against lane requirements.",
            &[("id", "integer", false)],
            "Observability",
            "0.1.4",
        ),
        tool(
            "harness-cli",
            "score-context",
            "score-context",
            "Score trace context reads against context rules.",
            &[("trace-id", "integer", true)],
            "Context selection",
            "0.1.8",
        ),
        tool(
            "harness-cli",
            "audit",
            "audit",
            "Run drift checks and compute entropy score.",
            &[],
            "Entropy auditing",
            "0.1.8",
        ),
        tool(
            "harness-cli",
            "propose",
            "propose",
            "Generate harness improvement proposals from observed patterns.",
            &[("commit", "flag", false)],
            "Entropy auditing",
            "0.1.8",
        ),
        tool(
            "harness-cli",
            "query matrix",
            "query matrix",
            "Show durable story proof matrix.",
            &[],
            "Task state",
            "0.1.0",
        ),
        tool(
            "harness-cli",
            "query backlog",
            "query backlog",
            "Show harness improvement backlog.",
            &[],
            "Entropy auditing",
            "0.1.0",
        ),
        tool(
            "harness-cli",
            "query decisions",
            "query decisions",
            "Show durable decision records.",
            &[],
            "Project memory",
            "0.1.0",
        ),
        tool(
            "harness-cli",
            "query intakes",
            "query intakes",
            "Show recent intake records.",
            &[],
            "Task specification",
            "0.1.0",
        ),
        tool(
            "harness-cli",
            "query traces",
            "query traces",
            "Show recent trace records.",
            &[],
            "Observability",
            "0.1.0",
        ),
        tool(
            "harness-cli",
            "query friction",
            "query friction",
            "Show traces that recorded harness friction.",
            &[],
            "Failure attribution",
            "0.1.4",
        ),
        tool(
            "harness-cli",
            "query interventions",
            "query interventions",
            "Show human or review intervention records.",
            &[],
            "Intervention recording",
            "0.1.8",
        ),
        tool(
            "harness-cli",
            "query stats",
            "query stats",
            "Show durable record counts.",
            &[],
            "Task state",
            "0.1.0",
        ),
        tool(
            "harness-cli",
            "query tools",
            "query tools",
            "Show compiled and registered tool manifest entries.",
            &[],
            "Tool access",
            "0.1.8",
        ),
        tool(
            "harness-cli",
            "query sql",
            "query sql",
            "Run arbitrary SQL against harness.db.",
            &[("query", "string", true)],
            "Tool access",
            "0.1.0",
        ),
        tool(
            "harness-cli",
            "tool register",
            "tool register",
            "Register an external project tool.",
            &[("name", "string", true), ("command", "string", true)],
            "Tool access",
            "0.1.8",
        ),
        tool(
            "harness-cli",
            "tool remove",
            "tool remove",
            "Remove a registered external tool.",
            &[("name", "string", true)],
            "Tool access",
            "0.1.8",
        ),
        tool(
            "harness-cli",
            "intervention add",
            "intervention add",
            "Record a human or review intervention.",
            &[
                ("type", "enum", true),
                ("description", "string", true),
                ("source", "enum", true),
            ],
            "Intervention recording",
            "0.1.8",
        ),
    ]
}

fn tool(
    provider: &str,
    name: &str,
    command: &str,
    description: &str,
    args: &[(&str, &str, bool)],
    responsibility: &str,
    since: &str,
) -> ToolEntry {
    ToolEntry {
        provider: provider.to_owned(),
        name: name.to_owned(),
        command: command.to_owned(),
        description: description.to_owned(),
        args: args
            .iter()
            .map(|(name, arg_type, required)| ToolArgSpec {
                name: (*name).to_owned(),
                arg_type: (*arg_type).to_owned(),
                required: *required,
                help: None,
            })
            .collect(),
        responsibility: responsibility.to_owned(),
        source: "compiled".to_owned(),
        since: since.to_owned(),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct IntakeRecord {
    pub id: i64,
    pub created_at: String,
    pub input_type: String,
    pub risk_lane: String,
    pub summary: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StoryMatrixRecord {
    pub id: String,
    pub title: String,
    pub status: String,
    pub unit: i64,
    pub integration: i64,
    pub e2e: i64,
    pub platform: i64,
    pub evidence: Option<String>,
    pub priority: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StoryVerifyStatus {
    pub id: String,
    pub verify_command: Option<String>,
    pub last_verified_result: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StoryVerifyAllItem {
    pub id: String,
    pub title: String,
    pub command: Option<String>,
    pub result: String,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StoryVerifyAllResult {
    pub items: Vec<StoryVerifyAllItem>,
}

impl StoryVerifyAllResult {
    pub fn passed(&self) -> usize {
        self.items
            .iter()
            .filter(|item| item.result == "pass")
            .count()
    }

    pub fn failed(&self) -> usize {
        self.items
            .iter()
            .filter(|item| item.result == "fail")
            .count()
    }

    pub fn skipped(&self) -> usize {
        self.items
            .iter()
            .filter(|item| item.result == "skipped")
            .count()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BacklogRecord {
    pub id: i64,
    pub title: String,
    pub status: String,
    pub risk: Option<String>,
    pub predicted_impact: Option<String>,
    pub actual_outcome: Option<String>,
    pub priority: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BacklogFilter {
    All,
    Open,
    Closed,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DecisionRecord {
    pub id: String,
    pub title: String,
    pub status: String,
    pub last_verified_at: Option<String>,
    pub last_verified_result: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TraceRecord {
    pub id: i64,
    pub created_at: String,
    pub outcome: Option<String>,
    pub task_summary: String,
    pub harness_friction: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TraceQualityTier {
    Incomplete = 0,
    Minimal = 1,
    Standard = 2,
    Detailed = 3,
}

impl TraceQualityTier {
    pub fn label(self) -> &'static str {
        match self {
            Self::Incomplete => "incomplete",
            Self::Minimal => "minimal",
            Self::Standard => "standard",
            Self::Detailed => "detailed",
        }
    }

    pub fn score(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TraceScoreSource {
    pub id: i64,
    pub task_summary: String,
    pub intake_id: Option<i64>,
    pub risk_lane: Option<String>,
    pub agent: Option<String>,
    pub actions_taken: Option<String>,
    pub files_read: Option<String>,
    pub files_changed: Option<String>,
    pub decisions_made: Option<String>,
    pub errors: Option<String>,
    pub outcome: Option<String>,
    pub duration_seconds: Option<i64>,
    pub token_estimate: Option<i64>,
    pub harness_friction: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TraceScoreResult {
    pub trace_id: i64,
    pub achieved: TraceQualityTier,
    pub risk_lane: Option<String>,
    pub required: Option<TraceQualityTier>,
    pub meets_requirement: bool,
    pub missing_minimal: Vec<String>,
    pub missing_standard: Vec<String>,
    pub missing_detailed: Vec<String>,
}

pub fn required_trace_tier_for_lane(risk_lane: &str) -> Option<TraceQualityTier> {
    match risk_lane {
        "tiny" => Some(TraceQualityTier::Minimal),
        "normal" => Some(TraceQualityTier::Standard),
        "high_risk" => Some(TraceQualityTier::Detailed),
        _ => None,
    }
}

pub fn score_trace(source: TraceScoreSource) -> TraceScoreResult {
    let missing_minimal = missing_minimal_fields(&source);
    let missing_standard = if missing_minimal.is_empty() {
        missing_standard_fields(&source)
    } else {
        Vec::new()
    };
    let missing_detailed = if missing_minimal.is_empty() && missing_standard.is_empty() {
        missing_detailed_fields(&source)
    } else {
        Vec::new()
    };

    let achieved = if !missing_minimal.is_empty() {
        TraceQualityTier::Incomplete
    } else if !missing_standard.is_empty() {
        TraceQualityTier::Minimal
    } else if !missing_detailed.is_empty() {
        TraceQualityTier::Standard
    } else {
        TraceQualityTier::Detailed
    };
    let required = source
        .risk_lane
        .as_deref()
        .and_then(required_trace_tier_for_lane);
    let meets_requirement = required.is_none_or(|tier| achieved >= tier);

    TraceScoreResult {
        trace_id: source.id,
        achieved,
        risk_lane: source.risk_lane,
        required,
        meets_requirement,
        missing_minimal,
        missing_standard,
        missing_detailed,
    }
}

pub fn score_context(source: ContextScoreSource) -> ContextScoreResult {
    let lane = source
        .risk_lane
        .clone()
        .unwrap_or_else(|| "unknown".to_owned());
    let phase = infer_context_phase(&source);
    let read = jsonish_list(source.files_read.as_deref());
    let changed = jsonish_list(source.files_changed.as_deref());

    let mut must = Vec::new();
    let mut should = Vec::new();
    let mut skipped = Vec::new();

    add_base_context_rules(&lane, &phase, &mut must, &mut should, &mut skipped);
    if changed
        .iter()
        .any(|path| path.starts_with("scripts/schema/"))
    {
        must.push((
            "SQLite durable layer decision",
            "docs/decisions/0004-sqlite-durable-layer.md",
        ));
    }
    if changed
        .iter()
        .any(|path| path.starts_with("crates/harness-cli/") || path.starts_with("scripts/bin/"))
    {
        must.push((
            "Prebuilt CLI decision",
            "docs/decisions/0005-prebuilt-rust-harness-cli.md",
        ));
    }

    let must = must
        .into_iter()
        .map(|(label, target)| ContextRequirementResult {
            label: label.to_owned(),
            target: target.to_owned(),
            met: path_read(&read, target, &changed),
        })
        .collect::<Vec<_>>();
    let should = should
        .into_iter()
        .map(|(label, target)| ContextRequirementResult {
            label: label.to_owned(),
            target: target.to_owned(),
            met: path_read(&read, target, &changed),
        })
        .collect::<Vec<_>>();
    let over_read = read
        .into_iter()
        .filter(|path| skipped.iter().any(|skip| path_matches(path, skip)))
        .collect::<Vec<_>>();

    ContextScoreResult {
        trace_id: source.id,
        lane,
        phase,
        must,
        should,
        over_read,
    }
}

fn infer_context_phase(source: &ContextScoreSource) -> String {
    let changed = source.files_changed.as_deref().unwrap_or("").trim();
    if source.outcome.as_deref() == Some("completed") {
        "trace".to_owned()
    } else if source.story_id.is_some() && !changed.is_empty() && changed != "[]" {
        "implementation".to_owned()
    } else if source.risk_lane.is_some() {
        "planning".to_owned()
    } else {
        "intake".to_owned()
    }
}

fn add_base_context_rules<'a>(
    lane: &str,
    phase: &str,
    must: &mut Vec<(&'a str, &'a str)>,
    should: &mut Vec<(&'a str, &'a str)>,
    skipped: &mut Vec<&'a str>,
) {
    match phase {
        "trace" => {
            must.push(("Trace specification", "docs/TRACE_SPEC.md"));
            must.push(("Changed-file list", "git status --short"));
            if lane == "normal" || lane == "high_risk" {
                must.push(("Durable matrix", "scripts/bin/harness-cli query matrix"));
            } else {
                should.push(("Durable matrix", "scripts/bin/harness-cli query matrix"));
            }
        }
        "implementation" => {
            must.push(("Files being changed", "<changed-files>"));
            if lane == "normal" || lane == "high_risk" {
                must.push(("Relevant story packet", "docs/stories/"));
                should.push(("Architecture rules", "docs/ARCHITECTURE.md"));
            }
            if lane == "high_risk" {
                must.push(("Architecture rules", "docs/ARCHITECTURE.md"));
                must.push((
                    "High-risk story template",
                    "docs/templates/high-risk-story/",
                ));
            }
        }
        "planning" => {
            must.push(("Files to edit", "<changed-files>"));
            if lane == "normal" || lane == "high_risk" {
                must.push(("Story template", "docs/templates/story.md"));
                must.push(("Test matrix", "docs/TEST_MATRIX.md"));
            }
            if lane == "high_risk" {
                must.push((
                    "High-risk story template",
                    "docs/templates/high-risk-story/",
                ));
                must.push(("Harness maturity", "docs/HARNESS_MATURITY.md"));
            }
        }
        _ => {
            must.push(("Agent entrypoint", "AGENTS.md"));
            must.push(("Feature intake", "docs/FEATURE_INTAKE.md"));
            must.push(("Durable matrix", "scripts/bin/harness-cli query matrix"));
            if lane == "tiny" {
                skipped.push("docs/ARCHITECTURE.md");
            } else {
                must.push(("README", "README.md"));
                must.push(("Harness operating model", "docs/HARNESS.md"));
            }
        }
    }
}

fn path_read(read: &[String], target: &str, changed: &[String]) -> bool {
    if target == "<changed-files>" {
        return !changed.is_empty();
    }
    read.iter().any(|path| path_matches(path, target))
}

fn path_matches(path: &str, target: &str) -> bool {
    if target.ends_with('/') {
        path.starts_with(target)
    } else {
        path == target || path.contains(target)
    }
}

pub fn jsonish_list(value: Option<&str>) -> Vec<String> {
    let Some(value) = value else {
        return Vec::new();
    };
    value
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.trim().trim_matches('"').to_owned())
        .filter(|item| !item.is_empty() && item != "null")
        .collect()
}

fn missing_minimal_fields(source: &TraceScoreSource) -> Vec<String> {
    let mut missing = Vec::new();
    if source.task_summary.trim().len() < 10 {
        missing.push("task_summary: missing or shorter than 10 characters".to_owned());
    }
    if blank(&source.outcome) {
        missing.push("outcome: null".to_owned());
    }
    missing
}

fn missing_standard_fields(source: &TraceScoreSource) -> Vec<String> {
    let mut missing = Vec::new();
    if blank(&source.agent) {
        missing.push("agent: empty".to_owned());
    }
    if short_json_list(&source.actions_taken) {
        missing.push("actions_taken: empty".to_owned());
    }
    if short_json_list(&source.files_read) {
        missing.push("files_read: empty".to_owned());
    }
    if source.files_changed.is_none() {
        missing.push("files_changed: null".to_owned());
    }
    if source.errors.is_none() && source.harness_friction.is_none() {
        missing.push("errors or harness_friction: both null".to_owned());
    }
    missing
}

fn missing_detailed_fields(source: &TraceScoreSource) -> Vec<String> {
    let mut missing = Vec::new();
    if short_json_list(&source.decisions_made) {
        missing.push("decisions_made: empty".to_owned());
    }
    if source.errors.is_none() {
        missing.push("errors: null".to_owned());
    }
    if source.harness_friction.is_none() {
        missing.push("harness_friction: null".to_owned());
    }
    if source.duration_seconds.is_none() && !notes_explain_missing(&source.notes, "duration") {
        missing.push("duration_seconds: null (no explanation in notes)".to_owned());
    }
    if source.token_estimate.is_none() && !notes_explain_missing(&source.notes, "token") {
        missing.push("token_estimate: null (no explanation in notes)".to_owned());
    }
    missing
}

fn blank(value: &Option<String>) -> bool {
    value.as_deref().map(str::trim).unwrap_or("").is_empty()
}

fn short_json_list(value: &Option<String>) -> bool {
    value.as_deref().map(str::trim).unwrap_or("").len() <= 2
}

fn notes_explain_missing(notes: &Option<String>, field: &str) -> bool {
    let Some(notes) = notes.as_deref() else {
        return false;
    };
    let lower = notes.to_ascii_lowercase();
    lower.contains(field)
        && (lower.contains("unavailable")
            || lower.contains("not available")
            || lower.contains("unknown"))
}

#[derive(Debug, PartialEq, Eq)]
pub struct FrictionRecord {
    pub id: i64,
    pub created_at: String,
    pub risk_lane: Option<String>,
    pub input_type: Option<String>,
    pub task_summary: String,
    pub harness_friction: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InterventionRecord {
    pub id: i64,
    pub created_at: String,
    pub trace_id: Option<i64>,
    pub story_id: Option<String>,
    pub intervention_type: String,
    pub description: String,
    pub source: String,
    pub impact: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ContextScoreSource {
    pub id: i64,
    pub risk_lane: Option<String>,
    pub story_id: Option<String>,
    pub files_read: Option<String>,
    pub files_changed: Option<String>,
    pub outcome: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ContextRequirementResult {
    pub label: String,
    pub target: String,
    pub met: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ContextScoreResult {
    pub trace_id: i64,
    pub lane: String,
    pub phase: String,
    pub must: Vec<ContextRequirementResult>,
    pub should: Vec<ContextRequirementResult>,
    pub over_read: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AuditFinding {
    pub id: String,
    pub title: String,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct AuditResult {
    pub orphaned_stories: Vec<AuditFinding>,
    pub unverified_stories: Vec<AuditFinding>,
    pub unverified_decisions: Vec<AuditFinding>,
    pub backlog_without_outcomes: Vec<AuditFinding>,
    pub stale_stories: Vec<AuditFinding>,
    pub broken_tools: Vec<AuditFinding>,
}

impl AuditResult {
    pub fn entropy_score(&self) -> i64 {
        let raw = (self.orphaned_stories.len() as i64 * 10)
            + (self.unverified_stories.len() as i64 * 5)
            + (self.unverified_decisions.len() as i64 * 5)
            + (self.backlog_without_outcomes.len() as i64 * 2)
            + (self.stale_stories.len() as i64 * 3)
            + (self.broken_tools.len() as i64 * 8);
        raw.min(100)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ImprovementProposal {
    pub title: String,
    pub component: String,
    pub evidence: String,
    pub predicted_impact: String,
    pub risk: String,
    pub suggested_action: String,
    pub validation_plan: String,
    pub confidence: String,
    pub committed_backlog_id: Option<i64>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HarnessStats {
    pub intakes: i64,
    pub stories: i64,
    pub decisions: i64,
    pub backlog_items: i64,
    pub traces: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsvList(pub Option<String>);

impl CsvList {
    pub fn from_optional(value: Option<String>) -> Self {
        Self(value.filter(|item| !item.is_empty()))
    }

    pub fn as_json_text(&self) -> Option<String> {
        self.0.as_ref().map(|value| {
            let escaped_items = value
                .split(',')
                .map(|item| format!("\"{}\"", escape_json_string(item.trim())))
                .collect::<Vec<_>>()
                .join(",");
            format!("[{escaped_items}]")
        })
    }

    pub fn as_json_text_or_null_literal(&self) -> String {
        self.as_json_text().unwrap_or_else(|| "null".to_owned())
    }
}

impl fmt::Display for CsvList {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.as_json_text_or_null_literal())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoolFlag(pub i64);

impl BoolFlag {
    pub fn parse(label: &str, value: &str) -> Result<Self, ParseHarnessValueError> {
        match value {
            "0" => Ok(Self(0)),
            "1" => Ok(Self(1)),
            _ => Err(ParseHarnessValueError::BoolFlag(label.to_owned())),
        }
    }
}

pub fn parse_optional_integer(
    label: &str,
    value: Option<String>,
) -> Result<Option<i64>, ParseHarnessValueError> {
    value
        .map(|inner| {
            inner
                .parse::<i64>()
                .map_err(|_| ParseHarnessValueError::Integer(label.to_owned()))
        })
        .transpose()
}

fn escape_json_string(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

pub fn normalize_token(value: &str) -> String {
    let mut normalized = String::new();
    let mut last_was_separator = false;

    for character in value.trim().chars().flat_map(char::to_lowercase) {
        if character.is_ascii_alphanumeric() {
            normalized.push(character);
            last_was_separator = false;
        } else if !last_was_separator && !normalized.is_empty() {
            normalized.push('_');
            last_was_separator = true;
        }
    }

    while normalized.ends_with('_') {
        normalized.pop();
    }

    normalized
}

pub fn yes_no(value: i64) -> String {
    if value == 1 {
        "yes".to_owned()
    } else {
        "no".to_owned()
    }
}

pub fn proof_display(value: i64, numeric: bool) -> String {
    if numeric {
        value.to_string()
    } else {
        yes_no(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_input_type_aliases() {
        assert_eq!("new_spec".parse::<InputType>().unwrap(), InputType::NewSpec);
        assert_eq!(
            "maintenance request".parse::<InputType>().unwrap(),
            InputType::Maintenance
        );
        assert_eq!(
            "Harness improvement".parse::<InputType>().unwrap(),
            InputType::HarnessImprovement
        );
    }

    #[test]
    fn parses_high_risk_lane_alias() {
        assert_eq!("high-risk".parse::<RiskLane>().unwrap(), RiskLane::HighRisk);
    }

    #[test]
    fn parses_priority_alias() {
        assert_eq!("p0".parse::<Priority>().unwrap(), Priority::P0);
        assert_eq!("P1".parse::<Priority>().unwrap(), Priority::P1);
        assert_eq!("p2".parse::<Priority>().unwrap(), Priority::P2);
        assert_eq!("P3".parse::<Priority>().unwrap(), Priority::P3);
        assert!("p4".parse::<Priority>().is_err());
    }

    #[test]
    fn renders_csv_as_json_text() {
        assert_eq!(
            CsvList::from_optional(Some("auth, data model".to_owned()))
                .as_json_text_or_null_literal(),
            "[\"auth\",\"data model\"]"
        );
        assert_eq!(
            CsvList::from_optional(None).as_json_text_or_null_literal(),
            "null"
        );
    }

    #[test]
    fn parses_bool_flags() {
        assert_eq!(BoolFlag::parse("--unit", "1").unwrap(), BoolFlag(1));
        assert!(BoolFlag::parse("--unit", "yes").is_err());
    }

    fn trace_source() -> TraceScoreSource {
        TraceScoreSource {
            id: 7,
            task_summary: "Completed a useful task".to_owned(),
            intake_id: None,
            risk_lane: None,
            agent: None,
            actions_taken: None,
            files_read: None,
            files_changed: None,
            decisions_made: None,
            errors: None,
            outcome: Some("completed".to_owned()),
            duration_seconds: None,
            token_estimate: None,
            harness_friction: None,
            notes: None,
        }
    }

    #[test]
    fn scores_minimal_standard_and_detailed_traces() {
        let minimal = score_trace(trace_source());
        assert_eq!(minimal.achieved, TraceQualityTier::Minimal);

        let mut standard_source = trace_source();
        standard_source.agent = Some("codex".to_owned());
        standard_source.actions_taken = Some("[\"read\",\"patched\"]".to_owned());
        standard_source.files_read = Some("[\"PHASE3.md\"]".to_owned());
        standard_source.files_changed = Some("[\"docs/TRACE_SPEC.md\"]".to_owned());
        standard_source.harness_friction = Some("none".to_owned());
        let standard = score_trace(standard_source);
        assert_eq!(standard.achieved, TraceQualityTier::Standard);

        let mut detailed_source = trace_source();
        detailed_source.agent = Some("codex".to_owned());
        detailed_source.actions_taken = Some("[\"read\",\"patched\"]".to_owned());
        detailed_source.files_read = Some("[\"PHASE3.md\"]".to_owned());
        detailed_source.files_changed = Some("[\"docs/TRACE_SPEC.md\"]".to_owned());
        detailed_source.decisions_made = Some("[\"kept schema unchanged\"]".to_owned());
        detailed_source.errors = Some("[\"none\"]".to_owned());
        detailed_source.harness_friction = Some("none".to_owned());
        detailed_source.duration_seconds = Some(120);
        detailed_source.token_estimate = Some(2000);
        let detailed = score_trace(detailed_source);
        assert_eq!(detailed.achieved, TraceQualityTier::Detailed);
    }

    #[test]
    fn compares_trace_score_to_lane_requirement() {
        let mut source = trace_source();
        source.risk_lane = Some("high_risk".to_owned());
        source.agent = Some("codex".to_owned());
        source.actions_taken = Some("[\"read\",\"patched\"]".to_owned());
        source.files_read = Some("[\"PHASE3.md\"]".to_owned());
        source.files_changed = Some("[\"docs/TRACE_SPEC.md\"]".to_owned());
        source.harness_friction = Some("none".to_owned());

        let result = score_trace(source);

        assert_eq!(result.achieved, TraceQualityTier::Standard);
        assert_eq!(result.required, Some(TraceQualityTier::Detailed));
        assert!(!result.meets_requirement);
        assert!(result
            .missing_detailed
            .iter()
            .any(|field| field.starts_with("decisions_made")));
    }

    #[test]
    fn context_score_applies_lane_and_retrieval_triggers() {
        let result = score_context(ContextScoreSource {
            id: 42,
            risk_lane: Some("normal".to_owned()),
            story_id: Some("US-019".to_owned()),
            files_read: Some(
                "[\"docs/stories/epics/E03-phase-5-evolution-infrastructure/US-019-tool-registry.md\",\"docs/decisions/0005-prebuilt-rust-harness-cli.md\"]".to_owned(),
            ),
            files_changed: Some("[\"crates/harness-cli/src/interface.rs\"]".to_owned()),
            outcome: None,
        });

        assert_eq!(result.phase, "implementation");
        assert!(result
            .must
            .iter()
            .any(|item| item.target == "docs/stories/" && item.met));
        assert!(result.must.iter().any(|item| item.target
            == "docs/decisions/0005-prebuilt-rust-harness-cli.md"
            && item.met));
    }
}
