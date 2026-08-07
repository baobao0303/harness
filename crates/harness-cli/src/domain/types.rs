use std::fmt;
use std::str::FromStr;

use super::errors::{ParseHarnessValueError, ParseWorkItemError};
use super::validation::{escape_json_string, normalize_token};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WorkItemType {
    Epic,
    Feature,
    UserStory,
    TechnicalStory,
    Task,
    Bug,
    Testcase,
}

impl WorkItemType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Epic => "Epic",
            Self::Feature => "Feature",
            Self::UserStory => "User Story",
            Self::TechnicalStory => "Technical Story",
            Self::Task => "Task",
            Self::Bug => "Bug",
            Self::Testcase => "Testcase",
        }
    }

    pub fn as_db_value(&self) -> &'static str {
        self.as_str()
    }
}

impl FromStr for WorkItemType {
    type Err = ParseWorkItemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim().to_lowercase().replace('_', " ").replace('-', " ");
        match normalized.as_str() {
            "epic" => Ok(Self::Epic),
            "feature" => Ok(Self::Feature),
            "user story" | "userstory" | "story" | "pbi" => Ok(Self::UserStory),
            "technical story" | "technicalstory" | "tech story" => Ok(Self::TechnicalStory),
            "task" => Ok(Self::Task),
            "bug" | "defect" => Ok(Self::Bug),
            "testcase" | "test case" | "tc" => Ok(Self::Testcase),
            _ => Err(ParseWorkItemError::UnknownType(s.to_owned())),
        }
    }
}

impl fmt::Display for WorkItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WorkItemState {
    New,
    Accepted,
    Active,
    Resolved,
    Closed,
    Blocked,
    Removed,
}

impl WorkItemState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::New => "New",
            Self::Accepted => "Accepted",
            Self::Active => "Active",
            Self::Resolved => "Resolved",
            Self::Closed => "Closed",
            Self::Blocked => "Blocked",
            Self::Removed => "Removed",
        }
    }

    pub fn as_db_value(&self) -> &'static str {
        self.as_str()
    }
}

impl FromStr for WorkItemState {
    type Err = ParseWorkItemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim().to_lowercase().replace('_', " ").replace('-', " ");
        match normalized.as_str() {
            "new" => Ok(Self::New),
            "accepted" => Ok(Self::Accepted),
            "active" | "in progress" => Ok(Self::Active),
            "resolved" => Ok(Self::Resolved),
            "closed" | "done" => Ok(Self::Closed),
            "blocked" => Ok(Self::Blocked),
            "removed" | "cancelled" => Ok(Self::Removed),
            _ => Err(ParseWorkItemError::UnknownState(s.to_owned())),
        }
    }
}

impl fmt::Display for WorkItemState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Critical => "Critical",
        }
    }

    pub fn as_db_value(&self) -> &'static str {
        self.as_str()
    }
}

impl FromStr for Severity {
    type Err = ParseWorkItemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "low" => Ok(Self::Low),
            "medium" | "med" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "critical" | "crit" | "blocker" => Ok(Self::Critical),
            _ => Err(ParseWorkItemError::UnknownSeverity(s.to_owned())),
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputType {
    NewSpec,
    SpecSlice,
    ChangeRequest,
    NewInitiative,
    Maintenance,
    HarnessImprovement,
}

impl InputType {
    pub fn as_db_value(&self) -> &'static str {
        match self {
            Self::NewSpec => "new_spec",
            Self::SpecSlice => "spec_slice",
            Self::ChangeRequest => "change_request",
            Self::NewInitiative => "new_initiative",
            Self::Maintenance => "maintenance",
            Self::HarnessImprovement => "harness_improvement",
        }
    }
}

impl FromStr for InputType {
    type Err = ParseHarnessValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let normalized = normalize_token(value);
        match normalized.as_str() {
            "new_spec" => Ok(Self::NewSpec),
            "spec_slice" => Ok(Self::SpecSlice),
            "change_request" => Ok(Self::ChangeRequest),
            "new_initiative" => Ok(Self::NewInitiative),
            "maintenance" | "maintenance_request" => Ok(Self::Maintenance),
            "harness_improvement" => Ok(Self::HarnessImprovement),
            _ => Err(ParseHarnessValueError::InputType(value.to_owned())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RiskLane {
    Tiny,
    Normal,
    HighRisk,
}

impl RiskLane {
    pub fn as_db_value(&self) -> &'static str {
        match self {
            Self::Tiny => "tiny",
            Self::Normal => "normal",
            Self::HighRisk => "high_risk",
        }
    }
}

impl FromStr for RiskLane {
    type Err = ParseHarnessValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let normalized = normalize_token(value);
        match normalized.as_str() {
            "tiny" => Ok(Self::Tiny),
            "normal" => Ok(Self::Normal),
            "high_risk" => Ok(Self::HighRisk),
            _ => Err(ParseHarnessValueError::RiskLane(value.to_owned())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Priority {
    P0,
    P1,
    P2,
    P3,
}

impl Priority {
    pub fn as_db_value(&self) -> &'static str {
        match self {
            Self::P0 => "P0",
            Self::P1 => "P1",
            Self::P2 => "P2",
            Self::P3 => "P3",
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_db_value())
    }
}


impl FromStr for Priority {
    type Err = ParseHarnessValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let normalized = normalize_token(value);
        match normalized.as_str() {
            "p0" => Ok(Self::P0),
            "p1" => Ok(Self::P1),
            "p2" => Ok(Self::P2),
            "p3" => Ok(Self::P3),
            _ => Err(ParseHarnessValueError::Priority(value.to_owned())),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BacklogFilter {
    All,
    Open,
    Closed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TraceQualityTier {
    Incomplete = 0,
    Minimal = 1,
    Standard = 2,
    Detailed = 3,
}

impl TraceQualityTier {
    pub fn label(self) -> &'static str {
        match self {
            Self::Incomplete => "incomplete",
            Self::Minimal => "minimal",
            Self::Standard => "standard",
            Self::Detailed => "detailed",
        }
    }

    pub fn score(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoolFlag(pub i64);

impl BoolFlag {
    pub fn parse(label: &str, value: &str) -> Result<Self, ParseHarnessValueError> {
        match value {
            "0" => Ok(Self(0)),
            "1" => Ok(Self(1)),
            _ => Err(ParseHarnessValueError::BoolFlag(label.to_owned())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsvList(pub Option<String>);

impl CsvList {
    pub fn from_optional(value: Option<String>) -> Self {
        Self(value.filter(|item| !item.is_empty()))
    }

    pub fn as_json_text(&self) -> Option<String> {
        self.0.as_ref().map(|value| {
            let escaped_items = value
                .split(',')
                .map(|item| format!("\"{}\"", escape_json_string(item.trim())))
                .collect::<Vec<_>>()
                .join(",");
            format!("[{escaped_items}]")
        })
    }

    pub fn as_json_text_or_null_literal(&self) -> String {
        self.as_json_text().unwrap_or_else(|| "null".to_owned())
    }
}

impl fmt::Display for CsvList {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.as_json_text_or_null_literal())
    }
}
