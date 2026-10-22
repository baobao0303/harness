use serde::{Deserialize, Serialize};

use super::decision::IntakeDecisionResult;
use super::types::RiskLane;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LambdaRoute {
    #[serde(rename = "fast_path_lambda")]
    FastPathLambda,
    #[serde(rename = "slow_path_escalation")]
    SlowPathEscalation,
}

impl LambdaRoute {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FastPathLambda => "fast_path_lambda",
            Self::SlowPathEscalation => "slow_path_escalation",
        }
    }

    pub fn is_fast_path(&self) -> bool {
        matches!(self, Self::FastPathLambda)
    }

    pub fn is_escalated(&self) -> bool {
        matches!(self, Self::SlowPathEscalation)
    }
}

impl std::fmt::Display for LambdaRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EscalationTarget {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "supervisor_agent")]
    SupervisorAgent,
    #[serde(rename = "qc_quality_gate")]
    QcQualityGate,
    #[serde(rename = "human_in_the_loop")]
    HumanInTheLoop,
}

impl EscalationTarget {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::SupervisorAgent => "supervisor_agent",
            Self::QcQualityGate => "qc_quality_gate",
            Self::HumanInTheLoop => "human_in_the_loop",
        }
    }

    pub fn requires_human(&self) -> bool {
        matches!(self, Self::HumanInTheLoop)
    }
}

impl std::fmt::Display for EscalationTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LambdaRoutingPolicy {
    pub confidence_threshold: f64,
    pub max_fast_path_latency_ms: u64,
    pub require_actionable: bool,
}

impl Default for LambdaRoutingPolicy {
    fn default() -> Self {
        Self::default_policy()
    }
}

impl LambdaRoutingPolicy {
    pub const DEFAULT_CONFIDENCE_THRESHOLD: f64 = 0.85;
    pub const DEFAULT_FAST_PATH_LATENCY_MS: u64 = 50;

    pub fn default_policy() -> Self {
        Self {
            confidence_threshold: Self::DEFAULT_CONFIDENCE_THRESHOLD,
            max_fast_path_latency_ms: Self::DEFAULT_FAST_PATH_LATENCY_MS,
            require_actionable: true,
        }
    }

    pub fn new(confidence_threshold: f64, max_fast_path_latency_ms: u64, require_actionable: bool) -> Self {
        Self {
            confidence_threshold,
            max_fast_path_latency_ms,
            require_actionable,
        }
    }

    pub fn with_confidence_threshold(mut self, threshold: f64) -> Self {
        self.confidence_threshold = threshold;
        self
    }

    pub fn with_max_fast_path_latency_ms(mut self, latency_ms: u64) -> Self {
        self.max_fast_path_latency_ms = latency_ms;
        self
    }

    pub fn with_require_actionable(mut self, required: bool) -> Self {
        self.require_actionable = required;
        self
    }

    pub fn evaluate(&self, risk_lane: &RiskLane, confidence: f64, is_actionable: bool) -> SupervisionDecision {
        if self.require_actionable && !is_actionable {
            return SupervisionDecision {
                route: LambdaRoute::SlowPathEscalation,
                risk_lane: *risk_lane,
                confidence,
                is_actionable: false,
                latency_budget_ms: 3000,
                escalation_target: EscalationTarget::SupervisorAgent,
                requires_human_approval: true,
                requires_qc_gate: false,
                rationale: "Slow-path escalation: Task is missing required context or is non-actionable. Forwarding to Supervisor intake gate.".to_string(),
            };
        }

        if risk_lane.is_red() {
            return SupervisionDecision {
                route: LambdaRoute::SlowPathEscalation,
                risk_lane: *risk_lane,
                confidence,
                is_actionable,
                latency_budget_ms: 3000,
                escalation_target: EscalationTarget::HumanInTheLoop,
                requires_human_approval: true,
                requires_qc_gate: true,
                rationale: format!(
                    "Slow-path escalation: Red risk lane poses high systemic risk; requires Supervisor review and Human-in-the-loop approval (confidence: {:.4}).",
                    confidence
                ),
            };
        }

        if risk_lane.is_yellow() {
            return SupervisionDecision {
                route: LambdaRoute::SlowPathEscalation,
                risk_lane: *risk_lane,
                confidence,
                is_actionable,
                latency_budget_ms: 1500,
                escalation_target: EscalationTarget::QcQualityGate,
                requires_human_approval: false,
                requires_qc_gate: true,
                rationale: format!(
                    "Slow-path escalation: Yellow risk lane requires independent QC quality gate verification before release (confidence: {:.4}).",
                    confidence
                ),
            };
        }

        if confidence < self.confidence_threshold {
            return SupervisionDecision {
                route: LambdaRoute::SlowPathEscalation,
                risk_lane: *risk_lane,
                confidence,
                is_actionable,
                latency_budget_ms: 3000,
                escalation_target: EscalationTarget::SupervisorAgent,
                requires_human_approval: true,
                requires_qc_gate: true,
                rationale: format!(
                    "Slow-path escalation: Calibrated confidence {:.4} is below threshold {:.2}. Escalating to Supervisor LLM for thorough review.",
                    confidence, self.confidence_threshold
                ),
            };
        }

        SupervisionDecision {
            route: LambdaRoute::FastPathLambda,
            risk_lane: *risk_lane,
            confidence,
            is_actionable,
            latency_budget_ms: self.max_fast_path_latency_ms,
            escalation_target: EscalationTarget::None,
            requires_human_approval: false,
            requires_qc_gate: false,
            rationale: format!(
                "Fast-path Lambda execution granted: Green risk lane with calibrated confidence {:.4} >= {:.2} (sub-{}ms budget).",
                confidence, self.confidence_threshold, self.max_fast_path_latency_ms
            ),
        }
    }

    pub fn evaluate_intake(&self, intake: &IntakeDecisionResult) -> SupervisionDecision {
        let is_actionable = !intake.summary_reason.is_empty();
        self.evaluate(&intake.predicted_lane, intake.confidence, is_actionable)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SupervisionDecision {
    pub route: LambdaRoute,
    pub risk_lane: RiskLane,
    pub confidence: f64,
    pub is_actionable: bool,
    pub latency_budget_ms: u64,
    pub escalation_target: EscalationTarget,
    pub requires_human_approval: bool,
    pub requires_qc_gate: bool,
    pub rationale: String,
}

impl SupervisionDecision {
    pub fn is_fast_path(&self) -> bool {
        self.route.is_fast_path()
    }

    pub fn is_escalated(&self) -> bool {
        self.route.is_escalated()
    }

    pub fn can_execute_sub_50ms(&self) -> bool {
        self.is_fast_path() && self.latency_budget_ms <= 50
    }

    pub fn route_label(&self) -> &'static str {
        self.route.as_str()
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::types::RiskLane;

    #[test]
    fn test_fast_path_green_high_confidence() {
        let policy = LambdaRoutingPolicy::default();
        let decision = policy.evaluate(&RiskLane::Green, 0.92, true);

        assert!(decision.is_fast_path());
        assert!(!decision.is_escalated());
        assert!(decision.can_execute_sub_50ms());
        assert_eq!(decision.route, LambdaRoute::FastPathLambda);
        assert_eq!(decision.escalation_target, EscalationTarget::None);
        assert!(!decision.requires_human_approval);
        assert!(!decision.requires_qc_gate);
        assert_eq!(decision.latency_budget_ms, 50);
    }

    #[test]
    fn test_fast_path_exact_threshold() {
        let policy = LambdaRoutingPolicy::default();
        let decision = policy.evaluate(&RiskLane::Green, 0.85, true);

        assert!(decision.is_fast_path());
        assert_eq!(decision.route, LambdaRoute::FastPathLambda);
    }

    #[test]
    fn test_fast_path_tiny_alias_high_confidence() {
        let policy = LambdaRoutingPolicy::default();
        let decision = policy.evaluate(&RiskLane::Tiny, 0.88, true);

        assert!(decision.is_fast_path());
        assert_eq!(decision.route, LambdaRoute::FastPathLambda);
        assert_eq!(decision.escalation_target, EscalationTarget::None);
    }

    #[test]
    fn test_slow_path_green_low_confidence() {
        let policy = LambdaRoutingPolicy::default();
        let decision = policy.evaluate(&RiskLane::Green, 0.8499, true);

        assert!(decision.is_escalated());
        assert!(!decision.is_fast_path());
        assert!(!decision.can_execute_sub_50ms());
        assert_eq!(decision.route, LambdaRoute::SlowPathEscalation);
        assert_eq!(decision.escalation_target, EscalationTarget::SupervisorAgent);
        assert!(decision.requires_human_approval);
        assert!(decision.requires_qc_gate);
    }

    #[test]
    fn test_slow_path_yellow_high_confidence() {
        let policy = LambdaRoutingPolicy::default();
        let decision = policy.evaluate(&RiskLane::Yellow, 0.99, true);

        assert!(decision.is_escalated());
        assert_eq!(decision.route, LambdaRoute::SlowPathEscalation);
        assert_eq!(decision.escalation_target, EscalationTarget::QcQualityGate);
        assert!(!decision.requires_human_approval);
        assert!(decision.requires_qc_gate);
    }

    #[test]
    fn test_slow_path_normal_alias_high_confidence() {
        let policy = LambdaRoutingPolicy::default();
        let decision = policy.evaluate(&RiskLane::Normal, 0.95, true);

        assert!(decision.is_escalated());
        assert_eq!(decision.route, LambdaRoute::SlowPathEscalation);
        assert_eq!(decision.escalation_target, EscalationTarget::QcQualityGate);
        assert!(decision.requires_qc_gate);
    }

    #[test]
    fn test_slow_path_red_high_confidence() {
        let policy = LambdaRoutingPolicy::default();
        let decision = policy.evaluate(&RiskLane::Red, 0.99, true);

        assert!(decision.is_escalated());
        assert_eq!(decision.route, LambdaRoute::SlowPathEscalation);
        assert_eq!(decision.escalation_target, EscalationTarget::HumanInTheLoop);
        assert!(decision.requires_human_approval);
        assert!(decision.requires_qc_gate);
        assert!(decision.escalation_target.requires_human());
    }

    #[test]
    fn test_slow_path_high_risk_alias() {
        let policy = LambdaRoutingPolicy::default();
        let decision = policy.evaluate(&RiskLane::HighRisk, 0.90, true);

        assert!(decision.is_escalated());
        assert_eq!(decision.route, LambdaRoute::SlowPathEscalation);
        assert_eq!(decision.escalation_target, EscalationTarget::HumanInTheLoop);
        assert!(decision.requires_human_approval);
    }

    #[test]
    fn test_slow_path_non_actionable() {
        let policy = LambdaRoutingPolicy::default();
        let decision = policy.evaluate(&RiskLane::Green, 0.99, false);

        assert!(decision.is_escalated());
        assert_eq!(decision.route, LambdaRoute::SlowPathEscalation);
        assert_eq!(decision.escalation_target, EscalationTarget::SupervisorAgent);
        assert!(!decision.is_actionable);
    }

    #[test]
    fn test_custom_policy_builder() {
        let policy = LambdaRoutingPolicy::new(0.90, 25, false);
        assert_eq!(policy.confidence_threshold, 0.90);
        assert_eq!(policy.max_fast_path_latency_ms, 25);
        assert!(!policy.require_actionable);

        let decision = policy.evaluate(&RiskLane::Green, 0.88, false);
        assert!(decision.is_escalated());

        let decision_high = policy.evaluate(&RiskLane::Green, 0.91, false);
        assert!(decision_high.is_fast_path());
        assert_eq!(decision_high.latency_budget_ms, 25);
    }

    #[test]
    fn test_json_serialization_round_trip() {
        let policy = LambdaRoutingPolicy::default();
        let original = policy.evaluate(&RiskLane::Green, 0.91, true);

        let json = original.to_json().expect("Serialization failed");
        let deserialized = SupervisionDecision::from_json(&json).expect("Deserialization failed");

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_evaluate_intake_result() {
        use std::collections::HashMap;
        use crate::domain::types::{InputType, Priority};

        let policy = LambdaRoutingPolicy::default();
        let intake = IntakeDecisionResult {
            input_type: InputType::SpecSlice,
            predicted_lane: RiskLane::Green,
            priority: Priority::P1,
            work_item_type: None,
            is_urgent: false,
            confidence: 0.93,
            probabilities: HashMap::new(),
            summary_reason: "Small safe slice".to_string(),
        };

        let decision = policy.evaluate_intake(&intake);
        assert!(decision.is_fast_path());
        assert_eq!(decision.latency_budget_ms, 50);
    }
}
