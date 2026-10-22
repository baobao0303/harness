use std::path::PathBuf;

use super::dto::*;
use crate::domain::{
    AuditResult, BacklogFilter, BacklogRecord, ContextScoreResult, CsvList, DecisionEngine,
    DecisionRecord, FrictionRecord, HarnessStats, ImprovementProposal, IntakeDecisionResult,
    IntakeRecord, InterventionRecord, RiskLane, StoryMatrixRecord, StoryVerifyAllResult,
    StoryVerifyStatus, ToolEntry, TraceRecord, TraceScoreResult,
};
use crate::infrastructure::{
    HarnessInfraError, HarnessRepository, JevClient, SqliteHarnessRepository,
};

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

    pub fn record_experience(
        &self,
        input: ExperienceAddInput,
    ) -> crate::infrastructure::Result<i64> {
        self.repository.record_experience(input)
    }

    pub fn query_experience(
        &self,
        domain: Option<&str>,
        work_item_id: Option<i64>,
    ) -> crate::infrastructure::Result<Vec<crate::domain::ExperienceRecord>> {
        self.repository.query_experience(domain, work_item_id)
    }

    pub fn adaptive_transition(
        &self,
        work_item_id: i64,
        target_state: crate::domain::WorkItemState,
    ) -> crate::infrastructure::Result<crate::domain::AdaptiveTransitionAdvice> {
        self.repository
            .adaptive_transition(work_item_id, target_state)
    }

    pub fn record_task_experience(
        &self,
        input: ExperienceAddInput,
    ) -> crate::infrastructure::Result<i64> {
        self.repository.record_experience(input)
    }

    pub fn query_adaptive_guidance(
        &self,
        work_item_id: i64,
        target_state: crate::domain::WorkItemState,
    ) -> crate::infrastructure::Result<crate::domain::AdaptiveTransitionAdvice> {
        self.repository
            .adaptive_transition(work_item_id, target_state)
    }

    pub fn apply_adaptive_transition(
        &self,
        work_item_id: i64,
        target_state: crate::domain::WorkItemState,
    ) -> crate::infrastructure::Result<crate::domain::AdaptiveTransitionAdvice> {
        let advice = self
            .repository
            .adaptive_transition(work_item_id, target_state)?;
        if advice.can_transition {
            self.repository.update_work_item(WorkItemUpdateInput {
                id: work_item_id,
                title: None,
                description: None,
                state: Some(target_state),
                assigned_to: None,
                story_points: None,
                remaining_work: None,
                priority: None,
                severity: None,
                parent_id: None,
                area_path: None,
                iteration_path: None,
                tags: None,
                acceptance_criteria: None,
                repro_steps: None,
                actual_result: None,
                expected_result: None,
                steps: None,
            })?;
        }
        Ok(advice)
    }

    pub fn learning_status(
        &self,
        domain: Option<&str>,
    ) -> crate::infrastructure::Result<LearningStatus> {
        let all_exps = self.repository.query_experience(domain, None)?;
        let total_experiences = all_exps.len();
        let high_confidence_count = all_exps
            .iter()
            .filter(|e| e.confidence.eq_ignore_ascii_case("high"))
            .count();
        let medium_confidence_count = all_exps
            .iter()
            .filter(|e| e.confidence.eq_ignore_ascii_case("medium"))
            .count();
        let low_confidence_count = all_exps
            .iter()
            .filter(|e| e.confidence.eq_ignore_ascii_case("low"))
            .count();

        let mut counts = std::collections::BTreeMap::new();
        for exp in &all_exps {
            *counts.entry(exp.domain.clone()).or_insert(0usize) += 1;
        }
        let domain_counts = counts.into_iter().collect();

        let recent_lessons: Vec<crate::domain::ExperienceRecord> =
            all_exps.into_iter().take(5).collect();
        let recent_experiences = recent_lessons.clone();

        Ok(LearningStatus {
            domain: domain.map(|d| d.to_string()),
            total_experiences,
            high_confidence_count,
            medium_confidence_count,
            low_confidence_count,
            domain_counts,
            recent_lessons,
            recent_experiences,
        })
    }

    pub fn get_learning_status(
        &self,
        domain: Option<&str>,
    ) -> crate::infrastructure::Result<LearningStatus> {
        self.learning_status(domain)
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

    pub fn auto_intake(
        &self,
        spec_text: &str,
    ) -> crate::infrastructure::Result<(i64, IntakeDecisionResult)> {
        let jev = JevClient::from_env();
        let mut decision = jev
            .evaluate_intake(spec_text)
            .map_err(|e| HarnessInfraError::Jev(e))?;

        // Hard policy check: If spec explicitly touches Auth or DB migration, enforce HighRisk
        let lower = spec_text.to_lowercase();
        let mut flags = Vec::new();
        if lower.contains("auth")
            || lower.contains("login")
            || lower.contains("jwt")
            || lower.contains("permission")
        {
            flags.push("Auth / Permission".to_string());
            decision.predicted_lane = RiskLane::HighRisk;
        }
        if lower.contains("migration") || lower.contains("drop table") || lower.contains("schema") {
            flags.push("Data Model Migration".to_string());
            decision.predicted_lane = RiskLane::HighRisk;
        }

        let summary = if spec_text.lines().count() > 0 {
            spec_text
                .lines()
                .next()
                .unwrap_or("Automated Intake")
                .trim()
                .chars()
                .take(120)
                .collect::<String>()
        } else {
            "Automated Intake".to_string()
        };

        let intake_id = self.record_intake(IntakeInput {
            input_type: decision.input_type.clone(),
            summary,
            risk_lane: decision.predicted_lane.clone(),
            risk_flags: CsvList::from_optional(if flags.is_empty() {
                None
            } else {
                Some(flags.join(", "))
            }),
            affected_docs: CsvList::from_optional(None),
            story_id: None,
            notes: Some(decision.summary_reason.clone()),
        })?;

        Ok((intake_id, decision))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Priority, WorkItemState, WorkItemType};
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn test_service() -> (TempDir, HarnessService) {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .unwrap()
            .to_path_buf();
        let context = HarnessContext {
            repo_root: repo_root.clone(),
            db_path: temp_dir.path().join("harness.db"),
            schema_dir: repo_root.join("scripts/schema"),
        };
        let service = HarnessService::new(context);
        service.init().unwrap();
        (temp_dir, service)
    }

    #[test]
    fn test_adaptive_transition_service_flow() {
        let (_tmp, service) = test_service();

        let item_id = service
            .add_work_item(WorkItemAddInput {
                work_type: WorkItemType::Task,
                title: "[Task] - Refactor auth handler".to_string(),
                description: Some("Handle expired JWT token".to_string()),
                assigned_to: Some("peer".to_string()),
                story_points: Some(2),
                remaining_work: Some(0.0),
                priority: Priority::P1,
                severity: None,
                parent_id: None,
                area_path: Some("security/auth".to_string()),
                iteration_path: None,
                tags: vec!["auth".to_string(), "jwt".to_string()],
                acceptance_criteria: None,
                repro_steps: None,
                actual_result: None,
                expected_result: None,
                steps: None,
            })
            .unwrap();

        // Record experience
        let exp_id = service
            .record_task_experience(ExperienceAddInput {
                domain: "security".to_string(),
                work_item_id: Some(item_id),
                cue: "When handling jwt tokens in auth handler".to_string(),
                lesson: "Verify signature before claims".to_string(),
                evidence: "Expired token check skipped signature".to_string(),
                confidence: "high".to_string(),
                harness_patch: None,
                pattern_hash: Some("hash_sec_jwt".to_string()),
                successful_harness: None,
                failure_trajectories: None,
            })
            .unwrap();
        assert!(exp_id > 0);

        // Check learning status
        let status = service.learning_status(Some("security")).unwrap();
        assert_eq!(status.total_experiences, 1);
        assert_eq!(status.high_confidence_count, 1);
        assert_eq!(status.recent_lessons.len(), 1);
        assert_eq!(status.recent_lessons[0].domain, "security");

        // Transition: New -> Active
        let advice = service
            .apply_adaptive_transition(item_id, WorkItemState::Active)
            .unwrap();
        assert!(advice.can_transition);
        let updated = service.get_work_item(item_id).unwrap().unwrap();
        assert_eq!(updated.state, WorkItemState::Active);

        // Transition: Active -> Resolved (has advice with high confidence lesson)
        let guidance = service
            .query_adaptive_guidance(item_id, WorkItemState::Resolved)
            .unwrap();
        assert!(guidance.can_transition);
        assert_eq!(guidance.applicable_lessons.len(), 1);
        assert!(!guidance.warnings.is_empty());

        let advice = service
            .apply_adaptive_transition(item_id, WorkItemState::Resolved)
            .unwrap();
        assert!(advice.can_transition);
        let resolved = service.get_work_item(item_id).unwrap().unwrap();
        assert_eq!(resolved.state, WorkItemState::Resolved);

        // Invalid transition: Resolved -> New (should fail / not transition)
        let invalid_advice = service
            .apply_adaptive_transition(item_id, WorkItemState::New)
            .unwrap();
        assert!(!invalid_advice.can_transition);
        let unchanged = service.get_work_item(item_id).unwrap().unwrap();
        assert_eq!(unchanged.state, WorkItemState::Resolved);
    }
}
