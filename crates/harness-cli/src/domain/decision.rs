use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::types::{InputType, Priority, RiskLane, WorkItemType};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SystemOneQuestion {
    #[serde(rename = "choice")]
    Choice {
        instructions: String,
        criteria: HashMap<String, String>,
    },
    #[serde(rename = "noul")]
    Noul {
        instructions: String,
    },
    #[serde(rename = "score")]
    Score {
        instructions: String,
        criteria: Vec<String>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemOneRequest {
    pub model: String,
    pub state: String,
    pub questions: HashMap<String, SystemOneQuestion>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemOneAnswer {
    #[serde(rename = "type")]
    pub answer_type: String,
    pub choice: Option<String>,
    pub confidence: Option<f64>,
    pub probabilities: Option<HashMap<String, f64>>,
    pub noul: Option<f64>,
    pub score: Option<f64>,
    pub legend: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemOneResponse {
    pub model: String,
    pub answers: HashMap<String, SystemOneAnswer>,
    pub usage: Option<serde_json::Value>,
    pub cost: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IntakeDecisionResult {
    pub input_type: InputType,
    pub predicted_lane: RiskLane,
    pub priority: Priority,
    pub work_item_type: Option<WorkItemType>,
    pub is_urgent: bool,
    pub confidence: f64,
    pub probabilities: HashMap<String, f64>,
    pub summary_reason: String,
}

pub trait DecisionEngine: Send + Sync {
    fn evaluate_intake(&self, spec_text: &str) -> Result<IntakeDecisionResult, String>;
}
