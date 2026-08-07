use std::path::PathBuf;

use super::dto::*;
use crate::domain::{
    AuditResult, BacklogFilter, BacklogRecord, ContextScoreResult, DecisionRecord, FrictionRecord,
    HarnessStats, ImprovementProposal, IntakeRecord, InterventionRecord, StoryMatrixRecord,
    StoryVerifyAllResult, StoryVerifyStatus, ToolEntry, TraceRecord, TraceScoreResult,
};
use crate::infrastructure::{HarnessRepository, SqliteHarnessRepository};

#[derive(Debug)]
pub struct HarnessContext {
    pub repo_root: PathBuf,
    pub db_path: PathBuf,
    pub schema_dir: PathBuf,
}

pub struct HarnessService {
    repository: SqliteHarnessRepository,
}

impl HarnessService {
    pub fn new(context: HarnessContext) -> Self {
        Self {
            repository: SqliteHarnessRepository::new(
                context.repo_root,
                context.db_path,
                context.schema_dir,
            ),
        }
    }

    pub fn init(&self) -> crate::infrastructure::Result<InitResult> {
        self.repository.init()
    }

    pub fn migrate(&self) -> crate::infrastructure::Result<MigrateResult> {
        self.repository.migrate()
    }

    pub fn import_brownfield(&self) -> crate::infrastructure::Result<BrownfieldImportResult> {
        self.repository.import_brownfield()
    }

    pub fn add_work_item(&self, input: WorkItemAddInput) -> crate::infrastructure::Result<i64> {
        self.repository.add_work_item(input)
    }

    pub fn update_work_item(
        &self,
        input: WorkItemUpdateInput,
    ) -> crate::infrastructure::Result<()> {
        self.repository.update_work_item(input)
    }

    pub fn get_work_item(
        &self,
        id: i64,
    ) -> crate::infrastructure::Result<Option<crate::domain::WorkItem>> {
        self.repository.get_work_item(id)
    }

    pub fn query_work_items(
        &self,
        work_type: Option<crate::domain::WorkItemType>,
        state: Option<crate::domain::WorkItemState>,
    ) -> crate::infrastructure::Result<Vec<crate::domain::WorkItem>> {
        self.repository.query_work_items(work_type, state)
    }

    pub fn record_intake(&self, input: IntakeInput) -> crate::infrastructure::Result<i64> {

        self.repository.record_intake(input)
    }

    pub fn add_story(&self, input: StoryAddInput) -> crate::infrastructure::Result<()> {
        self.repository.add_story(input)
    }

    pub fn update_story(&self, input: StoryUpdateInput) -> crate::infrastructure::Result<()> {
        self.repository.update_story(input)
    }

    pub fn verify_story(&self, id: &str) -> crate::infrastructure::Result<StoryVerifyResult> {
        self.repository.verify_story(id)
    }

    pub fn verify_all_stories(&self) -> crate::infrastructure::Result<StoryVerifyAllResult> {
        self.repository.verify_all_stories()
    }

    pub fn add_decision(&self, input: DecisionAddInput) -> crate::infrastructure::Result<()> {
        self.repository.add_decision(input)
    }

    pub fn verify_decision(&self, id: &str) -> crate::infrastructure::Result<DecisionVerifyResult> {
        self.repository.verify_decision(id)
    }

    pub fn add_backlog(&self, input: BacklogAddInput) -> crate::infrastructure::Result<i64> {
        self.repository.add_backlog(input)
    }

    pub fn close_backlog(&self, input: BacklogCloseInput) -> crate::infrastructure::Result<()> {
        self.repository.close_backlog(input)
    }

    pub fn register_tool(&self, input: ToolRegisterInput) -> crate::infrastructure::Result<()> {
        self.repository.register_tool(input)
    }

    pub fn remove_tool(&self, name: &str) -> crate::infrastructure::Result<()> {
        self.repository.remove_tool(name)
    }

    pub fn add_intervention(
        &self,
        input: InterventionAddInput,
    ) -> crate::infrastructure::Result<i64> {
        self.repository.add_intervention(input)
    }

    pub fn record_trace(&self, input: TraceInput) -> crate::infrastructure::Result<i64> {
        self.repository.record_trace(input)
    }

    pub fn score_trace(&self, id: Option<i64>) -> crate::infrastructure::Result<TraceScoreResult> {
        self.repository.score_trace(id)
    }

    pub fn score_context(&self, id: i64) -> crate::infrastructure::Result<ContextScoreResult> {
        self.repository.score_context(id)
    }

    pub fn story_verify_status(
        &self,
        id: &str,
    ) -> crate::infrastructure::Result<StoryVerifyStatus> {
        self.repository.story_verify_status(id)
    }

    pub fn query_matrix(&self) -> crate::infrastructure::Result<Vec<StoryMatrixRecord>> {
        self.repository.query_matrix()
    }

    pub fn query_backlog(
        &self,
        filter: BacklogFilter,
    ) -> crate::infrastructure::Result<Vec<BacklogRecord>> {
        self.repository.query_backlog(filter)
    }

    pub fn query_decisions(&self) -> crate::infrastructure::Result<Vec<DecisionRecord>> {
        self.repository.query_decisions()
    }

    pub fn query_intakes(&self) -> crate::infrastructure::Result<Vec<IntakeRecord>> {
        self.repository.query_intakes()
    }

    pub fn query_traces(&self) -> crate::infrastructure::Result<Vec<TraceRecord>> {
        self.repository.query_traces()
    }

    pub fn query_friction(&self) -> crate::infrastructure::Result<Vec<FrictionRecord>> {
        self.repository.query_friction()
    }

    pub fn query_tools(
        &self,
        responsibility: Option<String>,
    ) -> crate::infrastructure::Result<Vec<ToolEntry>> {
        self.repository.query_tools(responsibility)
    }

    pub fn query_interventions(
        &self,
        filter: InterventionFilter,
    ) -> crate::infrastructure::Result<Vec<InterventionRecord>> {
        self.repository.query_interventions(filter)
    }

    pub fn query_stats(&self) -> crate::infrastructure::Result<HarnessStats> {
        self.repository.query_stats()
    }

    pub fn audit(&self) -> crate::infrastructure::Result<AuditResult> {
        self.repository.audit()
    }

    pub fn propose(&self, commit: bool) -> crate::infrastructure::Result<Vec<ImprovementProposal>> {
        self.repository.propose(commit)
    }

    pub fn query_sql(&self, sql: &str) -> crate::infrastructure::Result<QueryTable> {
        self.repository.query_sql(sql)
    }
}
