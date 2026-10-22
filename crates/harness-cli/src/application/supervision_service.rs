use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::domain::{DecisionEngine, RiskLane, SystemOneQuestion, SystemOneRequest};
use crate::infrastructure::jev::JevClient;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupervisionDecision {
    Green,
    Yellow,
    Red,
}

impl SupervisionDecision {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupervisionEvaluation {
    pub decision: SupervisionDecision,
    pub risk_lane: String,
    pub confidence: f64,
    pub requires_human_review: bool,
    pub suggested_route: String,
    pub explanation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency_score: Option<f64>,
}

pub struct SupervisionService {
    jev: JevClient,
}

impl Default for SupervisionService {
    fn default() -> Self {
        Self::new()
    }
}

impl SupervisionService {
    pub fn new() -> Self {
        Self {
            jev: JevClient::from_env(),
        }
    }

    pub fn with_client(jev: JevClient) -> Self {
        Self { jev }
    }

    pub fn evaluate(&self, prompt_or_spec: &str) -> Result<SupervisionEvaluation, String> {
        // Build question for supervision / traffic light evaluation
        let mut questions = HashMap::new();
        let mut criteria = HashMap::new();
        criteria.insert(
            "green".to_string(),
            "Documentation fix, typos, trivial comments, zero code change risk, or safe read-only operations".to_string(),
        );
        criteria.insert(
            "yellow".to_string(),
            "Feature development, bug fixes, refactoring within existing modules, or moderate changes".to_string(),
        );
        criteria.insert(
            "red".to_string(),
            "Database drop, breaking API changes, security bypass, authentication alteration, high risk".to_string(),
        );

        questions.insert(
            "supervision_decision".to_string(),
            SystemOneQuestion::Choice {
                instructions: "Classify the risk level of this software development task (green, yellow, or red)."
                    .to_string(),
                criteria,
            },
        );
        questions.insert(
            "requires_human".to_string(),
            SystemOneQuestion::Noul {
                instructions: "Does this task require explicit human approval before execution?"
                    .to_string(),
            },
        );
        questions.insert(
            "urgency_rating".to_string(),
            SystemOneQuestion::Score {
                instructions: "How urgent is this supervision request?".to_string(),
                criteria: vec![
                    "Low: can wait days".to_string(),
                    "Medium: normal queue within 24h".to_string(),
                    "High: needs attention today".to_string(),
                    "Critical: immediate action required".to_string(),
                ],
            },
        );

        let req = SystemOneRequest {
            model: self.jev.model.clone(),
            state: prompt_or_spec.to_string(),
            questions,
        };

        let response = match self.jev.call_request(&req) {
            Ok(res) => res,
            Err(_) => {
                // If remote JEV is not reachable, fallback to deterministic heuristic
                return Ok(Self::heuristic_evaluation(prompt_or_spec));
            }
        };

        let mut decision = SupervisionDecision::Green;
        let mut confidence = 0.8;
        let mut requires_human = false;
        let mut urgency_score = None;

        if let Some(ans) = response.answers.get("supervision_decision") {
            if let Some(ref choice) = ans.choice {
                match choice.to_lowercase().as_str() {
                    "red" => decision = SupervisionDecision::Red,
                    "yellow" => decision = SupervisionDecision::Yellow,
                    _ => decision = SupervisionDecision::Green,
                }
            }
            if let Some(conf) = ans.confidence {
                confidence = conf;
            }
        } else if let Ok(intake_res) = self.jev.evaluate_intake(prompt_or_spec) {
            // Map IntakeDecisionResult to Green/Yellow/Red
            match intake_res.predicted_lane {
                RiskLane::HighRisk | RiskLane::Red => decision = SupervisionDecision::Red,
                RiskLane::Normal | RiskLane::Yellow => decision = SupervisionDecision::Yellow,
                RiskLane::Tiny | RiskLane::Green => decision = SupervisionDecision::Green,
            }
            confidence = intake_res.confidence;
        } else {
            return Ok(Self::heuristic_evaluation(prompt_or_spec));
        }

        if let Some(ans) = response.answers.get("requires_human") {
            if let Some(noul_val) = ans.noul {
                requires_human = noul_val >= 0.5;
            }
        }
        if decision == SupervisionDecision::Red {
            requires_human = true;
        }

        if let Some(ans) = response.answers.get("urgency_rating") {
            if let Some(score) = ans.score {
                urgency_score = Some(score);
            }
        }

        let route = match decision {
            SupervisionDecision::Green => "automated_worker".to_string(),
            SupervisionDecision::Yellow => "peer_review_queue".to_string(),
            SupervisionDecision::Red => "human_supervisor_approval".to_string(),
        };

        let explanation = format!(
            "Supervision classified as {:?} with confidence {:.2}% via model {}",
            decision,
            confidence * 100.0,
            response.model
        );

        Ok(SupervisionEvaluation {
            decision,
            risk_lane: match decision {
                SupervisionDecision::Green => "tiny".to_string(),
                SupervisionDecision::Yellow => "normal".to_string(),
                SupervisionDecision::Red => "high_risk".to_string(),
            },
            confidence,
            requires_human_review: requires_human,
            suggested_route: route,
            explanation,
            urgency_score,
        })
    }

    pub fn route(&self, prompt_or_spec: &str) -> Result<String, String> {
        let eval = self.evaluate(prompt_or_spec)?;
        Ok(eval.suggested_route)
    }

    pub fn heuristic_evaluation(prompt_or_spec: &str) -> SupervisionEvaluation {
        let lower = prompt_or_spec.to_lowercase();
        let (decision, requires_human, risk_lane) = if lower.contains("drop table")
            || lower.contains("delete from")
            || lower.contains("auth")
            || lower.contains("security")
            || lower.contains("secret")
            || lower.contains("jwt")
            || lower.contains("migration")
            || lower.contains("permission")
        {
            (SupervisionDecision::Red, true, "high_risk")
        } else if lower.contains("refactor") || lower.contains("feature") || lower.contains("api") {
            (SupervisionDecision::Yellow, false, "normal")
        } else {
            (SupervisionDecision::Green, false, "tiny")
        };

        let route = match decision {
            SupervisionDecision::Green => "automated_worker",
            SupervisionDecision::Yellow => "peer_review_queue",
            SupervisionDecision::Red => "human_supervisor_approval",
        };

        SupervisionEvaluation {
            decision,
            risk_lane: risk_lane.to_string(),
            confidence: 0.95,
            requires_human_review: requires_human,
            suggested_route: route.to_string(),
            explanation: format!(
                "Deterministic rule-based classification based on keywords: {:?}",
                decision
            ),
            urgency_score: None,
        }
    }
}
