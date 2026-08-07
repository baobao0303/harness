use thiserror::Error;

#[derive(Debug, Error)]
pub enum InterfaceError {
    #[error("{0}")]
    ParseHarnessValue(#[from] crate::domain::ParseHarnessValueError),
    #[error("{0}")]
    ParseWorkItem(#[from] crate::domain::ParseWorkItemError),
    #[error("{0}")]
    SyntaxValidation(#[from] crate::domain::SyntaxValidationError),
    #[error("{0}")]
    StateMachine(#[from] crate::domain::StateMachineError),
    #[error("{0}")]
    ToolValidation(#[from] crate::domain::ToolValidationError),
    #[error("{0}")]
    Infrastructure(#[from] crate::infrastructure::HarnessInfraError),
    #[error("could not determine current directory: {0}")]
    CurrentDir(std::io::Error),
    #[error("query sql requires a SQL statement")]
    EmptySql,
}

