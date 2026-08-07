pub mod brownfield;
pub mod db;
pub mod errors;
pub mod process;

pub use brownfield::*;
pub use db::*;
pub use errors::*;
pub use process::*;

use crate::application::{
    BacklogAddInput, BacklogCloseInput, BrownfieldImportResult, DecisionAddInput,
    DecisionVerifyResult, InitResult, IntakeInput, InterventionAddInput, InterventionFilter,
    MigrateResult, QueryTable, StoryAddInput, StoryUpdateInput, StoryVerifyResult,
    ToolRegisterInput, TraceInput, WorkItemAddInput, WorkItemUpdateInput,
};
use crate::domain::{
    AuditResult, BacklogFilter, BacklogRecord, ContextScoreResult, DecisionRecord, FrictionRecord,
    HarnessStats, ImprovementProposal, IntakeRecord, InterventionRecord, StoryMatrixRecord,
    StoryVerifyAllResult, StoryVerifyStatus, ToolEntry, TraceRecord, TraceScoreResult, WorkItem,
    WorkItemState, WorkItemType,
};


pub trait HarnessRepository {
    fn init(&self) -> Result<InitResult>;
    fn migrate(&self) -> Result<MigrateResult>;
    fn import_brownfield(&self) -> Result<BrownfieldImportResult>;
    fn record_intake(&self, input: IntakeInput) -> Result<i64>;
    fn add_work_item(&self, input: WorkItemAddInput) -> Result<i64>;
    fn update_work_item(&self, input: WorkItemUpdateInput) -> Result<()>;
    fn get_work_item(&self, id: i64) -> Result<Option<WorkItem>>;
    fn query_work_items(
        &self,
        work_type: Option<WorkItemType>,
        state: Option<WorkItemState>,
    ) -> Result<Vec<WorkItem>>;
    fn add_story(&self, input: StoryAddInput) -> Result<()>;

    fn update_story(&self, input: StoryUpdateInput) -> Result<()>;
    fn verify_story(&self, id: &str) -> Result<StoryVerifyResult>;
    fn verify_all_stories(&self) -> Result<StoryVerifyAllResult>;
    fn add_decision(&self, input: DecisionAddInput) -> Result<()>;
    fn verify_decision(&self, id: &str) -> Result<DecisionVerifyResult>;
    fn add_backlog(&self, input: BacklogAddInput) -> Result<i64>;
    fn close_backlog(&self, input: BacklogCloseInput) -> Result<()>;
    fn register_tool(&self, input: ToolRegisterInput) -> Result<()>;
    fn remove_tool(&self, name: &str) -> Result<()>;
    fn add_intervention(&self, input: InterventionAddInput) -> Result<i64>;
    fn record_trace(&self, input: TraceInput) -> Result<i64>;
    fn score_trace(&self, id: Option<i64>) -> Result<TraceScoreResult>;
    fn score_context(&self, id: i64) -> Result<ContextScoreResult>;
    fn story_verify_status(&self, id: &str) -> Result<StoryVerifyStatus>;
    fn query_matrix(&self) -> Result<Vec<StoryMatrixRecord>>;
    fn query_backlog(&self, filter: BacklogFilter) -> Result<Vec<BacklogRecord>>;
    fn query_decisions(&self) -> Result<Vec<DecisionRecord>>;
    fn query_intakes(&self) -> Result<Vec<IntakeRecord>>;
    fn query_traces(&self) -> Result<Vec<TraceRecord>>;
    fn query_friction(&self) -> Result<Vec<FrictionRecord>>;
    fn query_tools(&self, responsibility: Option<String>) -> Result<Vec<ToolEntry>>;
    fn query_interventions(&self, filter: InterventionFilter) -> Result<Vec<InterventionRecord>>;
    fn query_stats(&self) -> Result<HarnessStats>;
    fn audit(&self) -> Result<AuditResult>;
    fn propose(&self, commit: bool) -> Result<Vec<ImprovementProposal>>;
    fn query_sql(&self, sql: &str) -> Result<QueryTable>;
}
