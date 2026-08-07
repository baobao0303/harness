use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseHarnessValueError {
    #[error("unknown intake type '{0}'. Use: new spec, spec slice, change request, new initiative, maintenance request, or harness improvement")]
    InputType(String),
    #[error("unknown lane '{0}'. Use: tiny, normal, or high-risk. Use tiny instead of low.")]
    RiskLane(String),
    #[error("{0} must be an integer")]
    Integer(String),
    #[error("{0} must be 0 or 1. Example: --unit 1 --integration 1 --e2e 0 --platform 0")]
    BoolFlag(String),
    #[error("unknown priority '{0}'. Use: p0, p1, p2, or p3.")]
    Priority(String),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ToolValidationError {
    #[error("--description must be 10-200 characters")]
    DescriptionLength,
    #[error("unknown responsibility '{0}'. Use: {1}")]
    Responsibility(String, String),
    #[error("invalid --args spec '{0}'. Use name:type:required or name:type:required:help")]
    ArgSpec(String),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseWorkItemError {
    #[error("unknown work item type '{0}'. Valid types: Epic, Feature, User Story, Technical Story, Task, Bug, Testcase")]
    UnknownType(String),
    #[error("unknown state '{0}'. Valid states: New, Accepted, Active, Resolved, Closed, Blocked, Removed")]
    UnknownState(String),
    #[error("unknown severity '{0}'. Valid severities: Low, Medium, High, Critical")]
    UnknownSeverity(String),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum StateMachineError {
    #[error("invalid state transition for {work_type} from '{current}' to '{target}'. Allowed next states: {allowed}")]
    InvalidTransition {
        work_type: String,
        current: String,
        target: String,
        allowed: String,
    },
    #[error("role permission denied: role '{role}' is not authorized to transition {work_type} from '{current}' to '{target}'. Authorized roles: {authorized}")]
    UnauthorizedRole {
        role: String,
        work_type: String,
        current: String,
        target: String,
        authorized: String,
    },
    #[error("transition precondition failed: {reason}")]
    PreconditionFailed { reason: String },
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SyntaxValidationError {
    #[error("invalid Epic title '{0}'. Expected format: '[Giá trị nghiệp vụ] – [Tác động chính đến người dùng / doanh nghiệp]'")]
    InvalidEpicTitle(String),
    #[error("invalid Feature title '{0}'. Expected format: '[Khả năng cụ thể] – [Module/Tính năng chính]'")]
    InvalidFeatureTitle(String),
    #[error("invalid User Story title '{0}'. Expected format: '[Module/Feature] – [Vai trò] có thể [hành động/mục tiêu]'")]
    InvalidUserStoryTitle(String),
    #[error("invalid User Story description. Must start with or contain 'As a ... I want ... So that ...' format")]
    InvalidUserStoryDescription,
    #[error("invalid Task title '{0}'. Expected format: '[Module hoặc tính năng] - [Động từ + hành động cụ thể]'")]
    InvalidTaskTitle(String),
    #[error("invalid Testcase title '{0}'. Expected format: '[Module]: [Result expect] when [how to...]'")]
    InvalidTestcaseTitle(String),
    #[error("invalid Bug title '{0}'. Expected format: '[Module]: [Error message] when [why]'")]
    InvalidBugTitle(String),
}

