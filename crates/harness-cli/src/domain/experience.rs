use serde::{Deserialize, Serialize};

use super::entities::WorkItem;
use super::types::WorkItemState;
use super::validation::validate_state_transition;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperienceRecord {
    pub id: i64,
    pub domain: String,
    pub work_item_id: Option<i64>,
    pub cue: String,
    pub lesson: String,
    pub evidence: String,
    pub confidence: String,
    pub harness_patch: Option<String>,
    pub created_at: String,
    #[serde(default)]
    pub pattern_hash: Option<String>,
    #[serde(default)]
    pub successful_harness: Option<String>,
    #[serde(default)]
    pub failure_trajectories: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdaptiveRule {
    pub domain: String,
    pub pattern_hash: String,
    pub cue: String,
    pub rule: String,
    pub priority: i32,
    pub active: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdaptiveTransitionAdvice {
    pub current_state: WorkItemState,
    pub target_state: WorkItemState,
    pub can_transition: bool,
    pub applicable_lessons: Vec<String>,
    pub warnings: Vec<String>,
    pub transition_notes: String,
}

pub fn evaluate_adaptive_transition(
    work_item: &WorkItem,
    target_state: WorkItemState,
    experiences: &[ExperienceRecord],
) -> AdaptiveTransitionAdvice {
    let mut warnings = Vec::new();
    let mut applicable_lessons = Vec::new();

    let can_transition =
        validate_state_transition(work_item.work_type, work_item.state, target_state).is_ok();
    if !can_transition {
        warnings.push(format!(
            "Invalid state transition from {:?} to {:?}",
            work_item.state, target_state
        ));
    }

    let search_blob = format!(
        "{} {} {} {}",
        work_item.title.to_lowercase(),
        work_item
            .description
            .as_deref()
            .unwrap_or("")
            .to_lowercase(),
        work_item.tags.join(" ").to_lowercase(),
        work_item.area_path.as_deref().unwrap_or("").to_lowercase()
    );

    for exp in experiences {
        let cue_lower = exp.cue.to_lowercase();
        let matches_cue = cue_lower
            .split_whitespace()
            .any(|w| w.len() > 3 && search_blob.contains(w));

        let matches_domain = work_item
            .area_path
            .as_deref()
            .map(|a| a.to_lowercase().contains(&exp.domain.to_lowercase()))
            .unwrap_or(false);

        if matches_cue || matches_domain {
            applicable_lessons.push(format!(
                "[{}] (Confidence: {}) - Cue: {} | Lesson: {}",
                exp.domain, exp.confidence, exp.cue, exp.lesson
            ));
            if exp.confidence.eq_ignore_ascii_case("high")
                && target_state == WorkItemState::Resolved
            {
                warnings.push(format!(
                    "High-confidence lesson exists for '{}': verify '{}' before resolving.",
                    exp.domain, exp.lesson
                ));
            }
        }
    }

    let transition_notes = if applicable_lessons.is_empty() {
        format!(
            "Transition {:?} -> {:?} evaluated with 0 prior domain experiences.",
            work_item.state, target_state
        )
    } else {
        format!(
            "Transition {:?} -> {:?} informed by {} relevant experiences.",
            work_item.state,
            target_state,
            applicable_lessons.len()
        )
    };

    AdaptiveTransitionAdvice {
        current_state: work_item.state,
        target_state,
        can_transition,
        applicable_lessons,
        warnings,
        transition_notes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Priority, WorkItemType};

    fn sample_work_item(state: WorkItemState) -> WorkItem {
        WorkItem {
            id: 101,
            work_type: WorkItemType::Task,
            title: "Implement database ledger migration".to_string(),
            description: Some(
                "Ensure ledger transactions balance and handle negative credits".to_string(),
            ),
            state,
            assigned_to: Some("peer".to_string()),
            story_points: Some(3),
            remaining_work: Some(0.0),
            priority: Priority::P1,
            severity: None,
            parent_id: None,
            area_path: Some("accounting/ledger".to_string()),
            iteration_path: None,
            tags: vec!["ledger".to_string(), "migration".to_string()],
            acceptance_criteria: None,
            repro_steps: None,
            actual_result: None,
            expected_result: None,
            steps: None,
            created_at: "2026-10-22 09:00:00".to_string(),
            updated_at: "2026-10-22 09:00:00".to_string(),
        }
    }

    #[test]
    fn evaluates_transition_with_relevant_experiences() {
        let item = sample_work_item(WorkItemState::Active);
        let exps = vec![ExperienceRecord {
            id: 1,
            domain: "accounting".to_string(),
            work_item_id: Some(99),
            cue: "When ledger includes credits and debits".to_string(),
            lesson: "Include both signs when summing amounts in ledger summary".to_string(),
            evidence: "Probe total omitted negative credits".to_string(),
            confidence: "high".to_string(),
            harness_patch: Some("{\"scripts\":[\"summarize.py\"]}".to_string()),
            created_at: "2026-10-22 08:30:00".to_string(),
            pattern_hash: Some("hash123".to_string()),
            successful_harness: Some("harness_v1".to_string()),
            failure_trajectories: None,
        }];

        let advice = evaluate_adaptive_transition(&item, WorkItemState::Resolved, &exps);
        assert!(advice.can_transition);
        assert_eq!(advice.applicable_lessons.len(), 1);
        assert_eq!(advice.warnings.len(), 1);
        assert!(advice.warnings[0].contains("High-confidence lesson"));
    }

    #[test]
    fn prevents_invalid_transition() {
        let item = sample_work_item(WorkItemState::New);
        let advice = evaluate_adaptive_transition(&item, WorkItemState::Closed, &[]);
        assert!(!advice.can_transition);
        assert!(advice
            .warnings
            .iter()
            .any(|w| w.contains("Invalid state transition")));
    }

    #[test]
    fn serializes_and_deserializes_adaptive_rule() {
        let rule = AdaptiveRule {
            domain: "accounting".to_string(),
            pattern_hash: "hash_ledger_01".to_string(),
            cue: "When balancing credits".to_string(),
            rule: "Enforce zero-sum constraint".to_string(),
            priority: 1,
            active: true,
        };
        let serialized = serde_json::to_string(&rule).unwrap();
        let deserialized: AdaptiveRule = serde_json::from_str(&serialized).unwrap();
        assert_eq!(rule, deserialized);
    }
}
