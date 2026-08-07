use std::path::PathBuf;

use crate::domain::{
    BoolFlag, CsvList, InputType, Priority, RiskLane, Severity, ToolArgSpec, WorkItemState,
    WorkItemType,
};

#[derive(Clone, Debug)]
pub struct WorkItemAddInput {
    pub work_type: WorkItemType,
    pub title: String,
    pub description: Option<String>,
    pub assigned_to: Option<String>,
    pub story_points: Option<i64>,
    pub remaining_work: Option<f64>,
    pub priority: Priority,
    pub severity: Option<Severity>,
    pub parent_id: Option<i64>,
    pub area_path: Option<String>,
    pub iteration_path: Option<String>,
    pub tags: Vec<String>,
    pub acceptance_criteria: Option<String>,
    pub repro_steps: Option<String>,
    pub actual_result: Option<String>,
    pub expected_result: Option<String>,
    pub steps: Option<String>,
}

#[derive(Clone, Debug)]
pub struct WorkItemUpdateInput {
    pub id: i64,
    pub title: Option<String>,
    pub description: Option<String>,
    pub state: Option<WorkItemState>,
    pub assigned_to: Option<String>,
    pub story_points: Option<i64>,
    pub remaining_work: Option<f64>,
    pub priority: Option<Priority>,
    pub severity: Option<Severity>,
    pub parent_id: Option<i64>,
    pub area_path: Option<String>,
    pub iteration_path: Option<String>,
    pub tags: Option<Vec<String>>,
    pub acceptance_criteria: Option<String>,
    pub repro_steps: Option<String>,
    pub actual_result: Option<String>,
    pub expected_result: Option<String>,
    pub steps: Option<String>,
}

#[derive(Clone, Debug)]
pub struct IntakeInput {
    pub input_type: InputType,
    pub summary: String,
    pub risk_lane: RiskLane,
    pub risk_flags: CsvList,
    pub affected_docs: CsvList,
    pub story_id: Option<String>,
    pub notes: Option<String>,
}

#[derive(Clone, Debug)]
pub struct StoryAddInput {
    pub id: String,
    pub title: String,
    pub risk_lane: RiskLane,
    pub contract_doc: Option<String>,
    pub verify_command: Option<String>,
    pub notes: Option<String>,
    pub priority: Priority,
}

#[derive(Clone, Debug)]
pub struct StoryUpdateInput {
    pub id: String,
    pub status: Option<String>,
    pub evidence: Option<String>,
    pub unit: Option<BoolFlag>,
    pub integration: Option<BoolFlag>,
    pub e2e: Option<BoolFlag>,
    pub platform: Option<BoolFlag>,
    pub verify_command: Option<String>,
    pub priority: Option<Priority>,
}

#[derive(Clone, Debug)]
pub struct DecisionAddInput {
    pub id: String,
    pub title: String,
    pub status: String,
    pub doc_path: Option<String>,
    pub verify_command: Option<String>,
    pub predicted_impact: Option<String>,
    pub notes: Option<String>,
}

#[derive(Clone, Debug)]
pub struct BacklogAddInput {
    pub title: String,
    pub discovered_while: Option<String>,
    pub current_pain: Option<String>,
    pub suggestion: Option<String>,
    pub risk: Option<RiskLane>,
    pub predicted_impact: Option<String>,
    pub notes: Option<String>,
    pub priority: Priority,
}

#[derive(Clone, Debug)]
pub struct ToolRegisterInput {
    pub name: String,
    pub command: String,
    pub description: String,
    pub responsibility: String,
    pub args: Vec<ToolArgSpec>,
    pub force: bool,
}

#[derive(Clone, Debug)]
pub struct InterventionAddInput {
    pub trace_id: Option<i64>,
    pub story_id: Option<String>,
    pub intervention_type: String,
    pub description: String,
    pub source: String,
    pub impact: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct InterventionFilter {
    pub trace_id: Option<i64>,
    pub story_id: Option<String>,
    pub intervention_type: Option<String>,
}

#[derive(Clone, Debug)]
pub struct BacklogCloseInput {
    pub id: i64,
    pub status: String,
    pub actual_outcome: Option<String>,
}

#[derive(Clone, Debug)]
pub struct TraceInput {
    pub task_summary: String,
    pub intake_id: Option<i64>,
    pub story_id: Option<String>,
    pub agent: Option<String>,
    pub outcome: Option<String>,
    pub duration_seconds: Option<i64>,
    pub token_estimate: Option<i64>,
    pub friction: Option<String>,
    pub notes: Option<String>,
    pub actions: CsvList,
    pub files_read: CsvList,
    pub files_changed: CsvList,
    pub decisions: CsvList,
    pub errors: CsvList,
}

#[derive(Debug, PartialEq, Eq)]
pub enum InitResult {
    Created { db_path: PathBuf },
    Existing { db_path: PathBuf, version: i64 },
    MigratedExisting { db_path: PathBuf },
}

#[derive(Debug, PartialEq, Eq)]
pub struct MigrateResult {
    pub current_version: i64,
    pub applied: Vec<i64>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BrownfieldImportResult {
    pub stories: usize,
    pub decisions: usize,
    pub backlog_items: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DecisionVerifyResult {
    pub command: String,
    pub result: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StoryVerifyResult {
    pub command: String,
    pub stdout: String,
    pub stderr: String,
    pub result: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct QueryTable {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}
