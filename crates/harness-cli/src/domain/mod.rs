pub mod entities;
pub mod errors;
pub mod registry;
pub mod scoring;
pub mod types;
pub mod validation;

pub use entities::*;
pub use errors::*;
pub use registry::*;
pub use scoring::*;
pub use types::*;
pub use validation::*;

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

    #[test]
    fn parses_work_item_type_aliases() {
        assert_eq!("epic".parse::<WorkItemType>().unwrap(), WorkItemType::Epic);
        assert_eq!(
            "feature".parse::<WorkItemType>().unwrap(),
            WorkItemType::Feature
        );
        assert_eq!(
            "story".parse::<WorkItemType>().unwrap(),
            WorkItemType::UserStory
        );
        assert_eq!(
            "user_story".parse::<WorkItemType>().unwrap(),
            WorkItemType::UserStory
        );
        assert_eq!(
            "tech story".parse::<WorkItemType>().unwrap(),
            WorkItemType::TechnicalStory
        );
        assert_eq!("task".parse::<WorkItemType>().unwrap(), WorkItemType::Task);
        assert_eq!("bug".parse::<WorkItemType>().unwrap(), WorkItemType::Bug);
        assert_eq!("defect".parse::<WorkItemType>().unwrap(), WorkItemType::Bug);
        assert_eq!(
            "tc".parse::<WorkItemType>().unwrap(),
            WorkItemType::Testcase
        );
        assert!("invalid".parse::<WorkItemType>().is_err());
    }

    #[test]
    fn parses_work_item_state_aliases() {
        assert_eq!("new".parse::<WorkItemState>().unwrap(), WorkItemState::New);
        assert_eq!(
            "accepted".parse::<WorkItemState>().unwrap(),
            WorkItemState::Accepted
        );
        assert_eq!(
            "in progress".parse::<WorkItemState>().unwrap(),
            WorkItemState::Active
        );
        assert_eq!(
            "resolved".parse::<WorkItemState>().unwrap(),
            WorkItemState::Resolved
        );
        assert_eq!(
            "done".parse::<WorkItemState>().unwrap(),
            WorkItemState::Closed
        );
        assert_eq!(
            "blocked".parse::<WorkItemState>().unwrap(),
            WorkItemState::Blocked
        );
        assert_eq!(
            "cancelled".parse::<WorkItemState>().unwrap(),
            WorkItemState::Removed
        );
        assert!("invalid".parse::<WorkItemState>().is_err());
    }

    #[test]
    fn parses_severity_aliases() {
        assert_eq!("low".parse::<Severity>().unwrap(), Severity::Low);
        assert_eq!("med".parse::<Severity>().unwrap(), Severity::Medium);
        assert_eq!("high".parse::<Severity>().unwrap(), Severity::High);
        assert_eq!("blocker".parse::<Severity>().unwrap(), Severity::Critical);
        assert!("invalid".parse::<Severity>().is_err());
    }

    #[test]
    fn validates_work_item_titles() {
        assert!(
            validate_work_item_title(WorkItemType::Epic, "Company CRM - Tang doanh thu").is_ok()
        );
        assert!(validate_work_item_title(WorkItemType::Epic, "Company CRM").is_err());

        assert!(
            validate_work_item_title(WorkItemType::Feature, "Quan ly don hang - Sales").is_ok()
        );
        assert!(validate_work_item_title(WorkItemType::Feature, "Quan ly don hang").is_err());

        assert!(validate_work_item_title(
            WorkItemType::UserStory,
            "CRM - Khach hang co the tao don hang"
        )
        .is_ok());
        assert!(validate_work_item_title(
            WorkItemType::UserStory,
            "CRM - Sales user can create invoice"
        )
        .is_ok());
        assert!(
            validate_work_item_title(WorkItemType::UserStory, "CRM - Khach hang tao don hang")
                .is_err()
        );

        assert!(validate_work_item_title(WorkItemType::Task, "DB - Migration schema").is_ok());
        assert!(validate_work_item_title(WorkItemType::Task, "Migration schema").is_err());

        assert!(validate_work_item_title(
            WorkItemType::Testcase,
            "[Auth]: Login successfully when correct credentials"
        )
        .is_ok());
        assert!(validate_work_item_title(
            WorkItemType::Testcase,
            "[Auth]: Login successfully khi dung mat khau"
        )
        .is_ok());
        assert!(validate_work_item_title(
            WorkItemType::Testcase,
            "Login successfully when correct credentials"
        )
        .is_err());

        assert!(validate_work_item_title(
            WorkItemType::Bug,
            "[API]: 500 Server Error when fetching user list"
        )
        .is_ok());
        assert!(validate_work_item_title(
            WorkItemType::Bug,
            "[API]: 500 Server Error khi lay danh sach"
        )
        .is_ok());
        assert!(
            validate_work_item_title(WorkItemType::Bug, "500 Server Error when fetching").is_err()
        );
    }

    #[test]
    fn validates_user_story_description() {
        let valid_desc = "As a manager, I want to see reports, So that I can make decisions.";
        assert!(validate_work_item_description(WorkItemType::UserStory, Some(valid_desc)).is_ok());

        let invalid_desc = "Just a general description without scrum formula";
        assert!(
            validate_work_item_description(WorkItemType::UserStory, Some(invalid_desc)).is_err()
        );
        assert!(validate_work_item_description(WorkItemType::UserStory, None).is_err());

        assert!(validate_work_item_description(WorkItemType::Task, None).is_ok());
    }

    #[test]
    fn validates_state_machine_transitions() {
        assert!(validate_state_transition(
            WorkItemType::UserStory,
            WorkItemState::New,
            WorkItemState::Accepted
        )
        .is_ok());
        assert!(validate_state_transition(
            WorkItemType::UserStory,
            WorkItemState::Accepted,
            WorkItemState::Active
        )
        .is_ok());
        assert!(validate_state_transition(
            WorkItemType::UserStory,
            WorkItemState::Active,
            WorkItemState::Resolved
        )
        .is_ok());
        assert!(validate_state_transition(
            WorkItemType::UserStory,
            WorkItemState::Resolved,
            WorkItemState::Closed
        )
        .is_ok());
        assert!(validate_state_transition(
            WorkItemType::UserStory,
            WorkItemState::Resolved,
            WorkItemState::Accepted
        )
        .is_ok());
        assert!(validate_state_transition(
            WorkItemType::UserStory,
            WorkItemState::New,
            WorkItemState::Closed
        )
        .is_err());

        assert!(validate_state_transition(
            WorkItemType::Bug,
            WorkItemState::Resolved,
            WorkItemState::New
        )
        .is_ok());
        assert!(validate_state_transition(
            WorkItemType::Task,
            WorkItemState::Resolved,
            WorkItemState::Active
        )
        .is_ok());
    }
}
