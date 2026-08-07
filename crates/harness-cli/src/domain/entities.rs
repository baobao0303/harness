use super::types::{Priority, Severity, TraceQualityTier, WorkItemState, WorkItemType};

#[derive(Clone, Debug, PartialEq)]
pub struct WorkItem {
    pub id: i64,
    pub work_type: WorkItemType,
    pub title: String,
    pub description: Option<String>,
    pub state: WorkItemState,
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
    pub created_at: String,
    pub updated_at: String,
}


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
