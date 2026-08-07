use super::entities::ToolArgSpec;
use super::errors::{
    ParseHarnessValueError, StateMachineError, SyntaxValidationError, ToolValidationError,
};
use super::registry::RESPONSIBILITIES;
use super::types::{WorkItemState, WorkItemType};

pub fn allowed_next_states(
    work_type: WorkItemType,
    current: WorkItemState,
) -> &'static [WorkItemState] {
    match work_type {
        WorkItemType::UserStory => match current {
            WorkItemState::New => &[WorkItemState::Accepted, WorkItemState::Removed],
            WorkItemState::Accepted => &[WorkItemState::Active, WorkItemState::Removed],
            WorkItemState::Active => &[
                WorkItemState::Resolved,
                WorkItemState::Blocked,
                WorkItemState::Removed,
            ],
            WorkItemState::Blocked => &[WorkItemState::Active],
            WorkItemState::Resolved => &[WorkItemState::Closed, WorkItemState::Accepted],
            WorkItemState::Closed | WorkItemState::Removed => &[],
        },
        WorkItemType::Task => match current {
            WorkItemState::New => &[WorkItemState::Active, WorkItemState::Removed],
            WorkItemState::Active => &[
                WorkItemState::Resolved,
                WorkItemState::Blocked,
                WorkItemState::Removed,
            ],
            WorkItemState::Blocked => &[WorkItemState::Active],
            WorkItemState::Resolved => &[WorkItemState::Closed, WorkItemState::Active],
            WorkItemState::Closed | WorkItemState::Removed | WorkItemState::Accepted => &[],
        },
        WorkItemType::Bug => match current {
            WorkItemState::New => &[WorkItemState::Active, WorkItemState::Removed],
            WorkItemState::Active => &[WorkItemState::Resolved, WorkItemState::Removed],
            WorkItemState::Resolved => &[WorkItemState::Closed, WorkItemState::New],
            WorkItemState::Closed
            | WorkItemState::Removed
            | WorkItemState::Accepted
            | WorkItemState::Blocked => &[],
        },
        WorkItemType::Epic => match current {
            WorkItemState::New => &[WorkItemState::Active],
            WorkItemState::Active => &[WorkItemState::Resolved, WorkItemState::Blocked],
            WorkItemState::Blocked => &[WorkItemState::Active],
            WorkItemState::Resolved => &[WorkItemState::Closed],
            WorkItemState::Closed | WorkItemState::Accepted | WorkItemState::Removed => &[],
        },
        WorkItemType::Feature => match current {
            WorkItemState::New => &[WorkItemState::Active],
            WorkItemState::Active => &[
                WorkItemState::Resolved,
                WorkItemState::Blocked,
                WorkItemState::Removed,
            ],
            WorkItemState::Blocked => &[WorkItemState::Active],
            WorkItemState::Resolved => &[WorkItemState::Closed],
            WorkItemState::Closed | WorkItemState::Removed | WorkItemState::Accepted => &[],
        },
        WorkItemType::TechnicalStory => match current {
            WorkItemState::New => &[WorkItemState::Active],
            WorkItemState::Active => &[WorkItemState::Resolved, WorkItemState::Blocked],
            WorkItemState::Blocked => &[WorkItemState::Active],
            WorkItemState::Resolved => &[WorkItemState::Closed],
            WorkItemState::Closed | WorkItemState::Accepted | WorkItemState::Removed => &[],
        },
        WorkItemType::Testcase => match current {
            WorkItemState::New => &[WorkItemState::Active],
            WorkItemState::Active => &[WorkItemState::Resolved],
            WorkItemState::Resolved => &[WorkItemState::Closed],
            WorkItemState::Closed
            | WorkItemState::Removed
            | WorkItemState::Accepted
            | WorkItemState::Blocked => &[],
        },
    }
}

pub fn validate_state_transition(
    work_type: WorkItemType,
    current: WorkItemState,
    target: WorkItemState,
) -> Result<(), StateMachineError> {
    if current == target {
        return Ok(());
    }

    let allowed = allowed_next_states(work_type, current);
    if allowed.contains(&target) {
        Ok(())
    } else {
        let allowed_str = if allowed.is_empty() {
            "None (terminal state)".to_owned()
        } else {
            allowed
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        };
        Err(StateMachineError::InvalidTransition {
            work_type: work_type.as_str().to_string(),
            current: current.as_str().to_string(),
            target: target.as_str().to_string(),
            allowed: allowed_str,
        })
    }
}

fn has_dash_separator(s: &str) -> bool {
    let parts: Vec<&str> = s.split(|c| c == '-' || c == '–' || c == '—').collect();
    parts.len() >= 2 && !parts[0].trim().is_empty() && !parts[1..].join("").trim().is_empty()
}

pub fn validate_work_item_title(
    work_type: WorkItemType,
    title: &str,
) -> Result<(), SyntaxValidationError> {
    let trimmed = title.trim();
    if trimmed.is_empty() {
        return Err(match work_type {
            WorkItemType::Epic => SyntaxValidationError::InvalidEpicTitle(title.to_owned()),
            WorkItemType::Feature => SyntaxValidationError::InvalidFeatureTitle(title.to_owned()),
            WorkItemType::UserStory => {
                SyntaxValidationError::InvalidUserStoryTitle(title.to_owned())
            }
            WorkItemType::Task | WorkItemType::TechnicalStory => {
                SyntaxValidationError::InvalidTaskTitle(title.to_owned())
            }
            WorkItemType::Testcase => SyntaxValidationError::InvalidTestcaseTitle(title.to_owned()),
            WorkItemType::Bug => SyntaxValidationError::InvalidBugTitle(title.to_owned()),
        });
    }

    match work_type {
        WorkItemType::Epic => {
            if !has_dash_separator(trimmed) {
                return Err(SyntaxValidationError::InvalidEpicTitle(title.to_owned()));
            }
        }
        WorkItemType::Feature => {
            if !has_dash_separator(trimmed) {
                return Err(SyntaxValidationError::InvalidFeatureTitle(title.to_owned()));
            }
        }
        WorkItemType::UserStory => {
            let lower = trimmed.to_lowercase();
            if !has_dash_separator(trimmed)
                || (!lower.contains("có thể")
                    && !lower.contains("co the")
                    && !lower.contains("can"))
            {
                return Err(SyntaxValidationError::InvalidUserStoryTitle(
                    title.to_owned(),
                ));
            }
        }

        WorkItemType::Task | WorkItemType::TechnicalStory => {
            if !has_dash_separator(trimmed) {
                return Err(SyntaxValidationError::InvalidTaskTitle(title.to_owned()));
            }
        }
        WorkItemType::Testcase => {
            let lower = trimmed.to_lowercase();
            if !trimmed.starts_with('[')
                || !trimmed.contains(']')
                || !trimmed.contains(':')
                || (!lower.contains("when") && !lower.contains("khi"))
            {
                return Err(SyntaxValidationError::InvalidTestcaseTitle(
                    title.to_owned(),
                ));
            }
        }
        WorkItemType::Bug => {
            let lower = trimmed.to_lowercase();
            if !trimmed.starts_with('[')
                || !trimmed.contains(']')
                || !trimmed.contains(':')
                || (!lower.contains("when") && !lower.contains("khi"))
            {
                return Err(SyntaxValidationError::InvalidBugTitle(title.to_owned()));
            }
        }
    }

    Ok(())
}

pub fn validate_work_item_description(
    work_type: WorkItemType,
    description: Option<&str>,
) -> Result<(), SyntaxValidationError> {
    if work_type == WorkItemType::UserStory {
        let Some(desc) = description else {
            return Err(SyntaxValidationError::InvalidUserStoryDescription);
        };
        let lower = desc.to_lowercase();
        let as_a_pos = lower.find("as a");
        let i_want_pos = lower.find("i want");
        let so_that_pos = lower.find("so that");

        match (as_a_pos, i_want_pos, so_that_pos) {
            (Some(a), Some(b), Some(c)) if a < b && b < c => Ok(()),
            _ => Err(SyntaxValidationError::InvalidUserStoryDescription),
        }
    } else {
        Ok(())
    }
}

pub fn parse_tags_string(s: Option<&str>) -> Vec<String> {
    let Some(raw) = s else { return Vec::new() };
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }
    if trimmed.starts_with('[') && trimmed.ends_with(']') {
        trimmed[1..trimmed.len() - 1]
            .split(',')
            .map(|item| item.trim().trim_matches('"').trim_matches('\'').to_string())
            .filter(|item| !item.is_empty())
            .collect()
    } else {
        trimmed
            .split(',')
            .map(|item| item.trim().to_string())
            .filter(|item| !item.is_empty())
            .collect()
    }
}

pub fn tags_to_json(tags: &[String]) -> Option<String> {
    if tags.is_empty() {
        None
    } else {
        let items = tags
            .iter()
            .map(|t| format!("\"{}\"", escape_json_string(t.trim())))
            .collect::<Vec<_>>()
            .join(",");
        Some(format!("[{items}]"))
    }
}

pub fn parse_tool_args(value: Option<String>) -> Result<Vec<ToolArgSpec>, ToolValidationError> {
    let Some(value) = value else {
        return Ok(Vec::new());
    };
    if value.trim().is_empty() {
        return Ok(Vec::new());
    }

    value
        .split(',')
        .map(|raw| {
            let parts = raw.splitn(4, ':').map(str::trim).collect::<Vec<_>>();
            if parts.len() < 3
                || parts[0].is_empty()
                || parts[1].is_empty()
                || !matches!(parts[2], "required" | "optional")
            {
                return Err(ToolValidationError::ArgSpec(raw.to_owned()));
            }
            Ok(ToolArgSpec {
                name: parts[0].to_owned(),
                arg_type: parts[1].to_owned(),
                required: parts[2] == "required",
                help: parts
                    .get(3)
                    .filter(|value| !value.is_empty())
                    .map(|value| value.to_string()),
            })
        })
        .collect()
}

pub fn validate_tool_description(description: &str) -> Result<(), ToolValidationError> {
    let length = description.trim().chars().count();
    if !(10..=200).contains(&length) {
        return Err(ToolValidationError::DescriptionLength);
    }
    Ok(())
}

pub fn validate_responsibility(value: &str) -> Result<String, ToolValidationError> {
    RESPONSIBILITIES
        .iter()
        .find(|item| normalize_token(item) == normalize_token(value))
        .map(|item| (*item).to_owned())
        .ok_or_else(|| {
            ToolValidationError::Responsibility(value.to_owned(), RESPONSIBILITIES.join(", "))
        })
}

pub fn normalize_token(value: &str) -> String {
    let mut normalized = String::new();
    let mut last_was_separator = false;

    for character in value.trim().chars().flat_map(char::to_lowercase) {
        if character.is_ascii_alphanumeric() {
            normalized.push(character);
            last_was_separator = false;
        } else if !last_was_separator && !normalized.is_empty() {
            normalized.push('_');
            last_was_separator = true;
        }
    }

    while normalized.ends_with('_') {
        normalized.pop();
    }

    normalized
}

pub fn yes_no(value: i64) -> String {
    if value == 1 {
        "yes".to_owned()
    } else {
        "no".to_owned()
    }
}

pub fn proof_display(value: i64, numeric: bool) -> String {
    if numeric {
        value.to_string()
    } else {
        yes_no(value)
    }
}

pub fn parse_optional_integer(
    label: &str,
    value: Option<String>,
) -> Result<Option<i64>, ParseHarnessValueError> {
    value
        .map(|inner| {
            inner
                .parse::<i64>()
                .map_err(|_| ParseHarnessValueError::Integer(label.to_owned()))
        })
        .transpose()
}

pub fn escape_json_string(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
