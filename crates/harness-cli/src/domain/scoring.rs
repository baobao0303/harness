use super::entities::{
    ContextRequirementResult, ContextScoreResult, ContextScoreSource, TraceScoreResult,
    TraceScoreSource,
};
use super::types::TraceQualityTier;

pub fn required_trace_tier_for_lane(risk_lane: &str) -> Option<TraceQualityTier> {
    match risk_lane {
        "tiny" => Some(TraceQualityTier::Minimal),
        "normal" => Some(TraceQualityTier::Standard),
        "high_risk" => Some(TraceQualityTier::Detailed),
        _ => None,
    }
}

pub fn score_trace(source: TraceScoreSource) -> TraceScoreResult {
    let missing_minimal = missing_minimal_fields(&source);
    let missing_standard = if missing_minimal.is_empty() {
        missing_standard_fields(&source)
    } else {
        Vec::new()
    };
    let missing_detailed = if missing_minimal.is_empty() && missing_standard.is_empty() {
        missing_detailed_fields(&source)
    } else {
        Vec::new()
    };

    let achieved = if !missing_minimal.is_empty() {
        TraceQualityTier::Incomplete
    } else if !missing_standard.is_empty() {
        TraceQualityTier::Minimal
    } else if !missing_detailed.is_empty() {
        TraceQualityTier::Standard
    } else {
        TraceQualityTier::Detailed
    };
    let required = source
        .risk_lane
        .as_deref()
        .and_then(required_trace_tier_for_lane);
    let meets_requirement = required.is_none_or(|tier| achieved >= tier);

    TraceScoreResult {
        trace_id: source.id,
        achieved,
        risk_lane: source.risk_lane,
        required,
        meets_requirement,
        missing_minimal,
        missing_standard,
        missing_detailed,
    }
}

pub fn score_context(source: ContextScoreSource) -> ContextScoreResult {
    let lane = source
        .risk_lane
        .clone()
        .unwrap_or_else(|| "unknown".to_owned());
    let phase = infer_context_phase(&source);
    let read = jsonish_list(source.files_read.as_deref());
    let changed = jsonish_list(source.files_changed.as_deref());

    let mut must = Vec::new();
    let mut should = Vec::new();
    let mut skipped = Vec::new();

    add_base_context_rules(&lane, &phase, &mut must, &mut should, &mut skipped);
    if changed
        .iter()
        .any(|path| path.starts_with("scripts/schema/"))
    {
        must.push((
            "SQLite durable layer decision",
            "docs/decisions/0004-sqlite-durable-layer.md",
        ));
    }
    if changed
        .iter()
        .any(|path| path.starts_with("crates/harness-cli/") || path.starts_with("scripts/bin/"))
    {
        must.push((
            "Prebuilt CLI decision",
            "docs/decisions/0005-prebuilt-rust-harness-cli.md",
        ));
    }

    let must = must
        .into_iter()
        .map(|(label, target)| ContextRequirementResult {
            label: label.to_owned(),
            target: target.to_owned(),
            met: path_read(&read, target, &changed),
        })
        .collect::<Vec<_>>();
    let should = should
        .into_iter()
        .map(|(label, target)| ContextRequirementResult {
            label: label.to_owned(),
            target: target.to_owned(),
            met: path_read(&read, target, &changed),
        })
        .collect::<Vec<_>>();
    let over_read = read
        .into_iter()
        .filter(|path| skipped.iter().any(|skip| path_matches(path, skip)))
        .collect::<Vec<_>>();

    ContextScoreResult {
        trace_id: source.id,
        lane,
        phase,
        must,
        should,
        over_read,
    }
}

fn infer_context_phase(source: &ContextScoreSource) -> String {
    let changed = source.files_changed.as_deref().unwrap_or("").trim();
    if source.outcome.as_deref() == Some("completed") {
        "trace".to_owned()
    } else if source.story_id.is_some() && !changed.is_empty() && changed != "[]" {
        "implementation".to_owned()
    } else if source.risk_lane.is_some() {
        "planning".to_owned()
    } else {
        "intake".to_owned()
    }
}

fn add_base_context_rules<'a>(
    lane: &str,
    phase: &str,
    must: &mut Vec<(&'a str, &'a str)>,
    should: &mut Vec<(&'a str, &'a str)>,
    skipped: &mut Vec<&'a str>,
) {
    match phase {
        "trace" => {
            must.push(("Trace specification", "docs/TRACE_SPEC.md"));
            must.push(("Changed-file list", "git status --short"));
            if lane == "normal" || lane == "high_risk" {
                must.push(("Durable matrix", "scripts/bin/harness-cli query matrix"));
            } else {
                should.push(("Durable matrix", "scripts/bin/harness-cli query matrix"));
            }
        }
        "implementation" => {
            must.push(("Files being changed", "<changed-files>"));
            if lane == "normal" || lane == "high_risk" {
                must.push(("Relevant story packet", "docs/stories/"));
                should.push(("Architecture rules", "docs/ARCHITECTURE.md"));
            }
            if lane == "high_risk" {
                must.push(("Architecture rules", "docs/ARCHITECTURE.md"));
                must.push((
                    "High-risk story template",
                    "docs/templates/high-risk-story/",
                ));
            }
        }
        "planning" => {
            must.push(("Files to edit", "<changed-files>"));
            if lane == "normal" || lane == "high_risk" {
                must.push(("Story template", "docs/templates/story.md"));
                must.push(("Test matrix", "docs/TEST_MATRIX.md"));
            }
            if lane == "high_risk" {
                must.push((
                    "High-risk story template",
                    "docs/templates/high-risk-story/",
                ));
                must.push(("Harness maturity", "docs/HARNESS_MATURITY.md"));
            }
        }
        _ => {
            must.push(("Agent entrypoint", "AGENTS.md"));
            must.push(("Feature intake", "docs/FEATURE_INTAKE.md"));
            must.push(("Durable matrix", "scripts/bin/harness-cli query matrix"));
            if lane == "tiny" {
                skipped.push("docs/ARCHITECTURE.md");
            } else {
                must.push(("README", "README.md"));
                must.push(("Harness operating model", "docs/HARNESS.md"));
            }
        }
    }
}

fn path_read(read: &[String], target: &str, changed: &[String]) -> bool {
    if target == "<changed-files>" {
        return !changed.is_empty();
    }
    read.iter().any(|path| path_matches(path, target))
}

fn path_matches(path: &str, target: &str) -> bool {
    if target.ends_with('/') {
        path.starts_with(target)
    } else {
        path == target || path.contains(target)
    }
}

pub fn jsonish_list(value: Option<&str>) -> Vec<String> {
    let Some(value) = value else {
        return Vec::new();
    };
    value
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.trim().trim_matches('"').to_owned())
        .filter(|item| !item.is_empty() && item != "null")
        .collect()
}

fn missing_minimal_fields(source: &TraceScoreSource) -> Vec<String> {
    let mut missing = Vec::new();
    if source.task_summary.trim().len() < 10 {
        missing.push("task_summary: missing or shorter than 10 characters".to_owned());
    }
    if blank(&source.outcome) {
        missing.push("outcome: null".to_owned());
    }
    missing
}

fn missing_standard_fields(source: &TraceScoreSource) -> Vec<String> {
    let mut missing = Vec::new();
    if blank(&source.agent) {
        missing.push("agent: empty".to_owned());
    }
    if short_json_list(&source.actions_taken) {
        missing.push("actions_taken: empty".to_owned());
    }
    if short_json_list(&source.files_read) {
        missing.push("files_read: empty".to_owned());
    }
    if source.files_changed.is_none() {
        missing.push("files_changed: null".to_owned());
    }
    if source.errors.is_none() && source.harness_friction.is_none() {
        missing.push("errors or harness_friction: both null".to_owned());
    }
    missing
}

fn missing_detailed_fields(source: &TraceScoreSource) -> Vec<String> {
    let mut missing = Vec::new();
    if short_json_list(&source.decisions_made) {
        missing.push("decisions_made: empty".to_owned());
    }
    if source.errors.is_none() {
        missing.push("errors: null".to_owned());
    }
    if source.harness_friction.is_none() {
        missing.push("harness_friction: null".to_owned());
    }
    if source.duration_seconds.is_none() && !notes_explain_missing(&source.notes, "duration") {
        missing.push("duration_seconds: null (no explanation in notes)".to_owned());
    }
    if source.token_estimate.is_none() && !notes_explain_missing(&source.notes, "token") {
        missing.push("token_estimate: null (no explanation in notes)".to_owned());
    }
    missing
}

fn blank(value: &Option<String>) -> bool {
    value.as_deref().map(str::trim).unwrap_or("").is_empty()
}

fn short_json_list(value: &Option<String>) -> bool {
    value.as_deref().map(str::trim).unwrap_or("").len() <= 2
}

fn notes_explain_missing(notes: &Option<String>, field: &str) -> bool {
    let Some(notes) = notes.as_deref() else {
        return false;
    };
    let lower = notes.to_ascii_lowercase();
    lower.contains(field)
        && (lower.contains("unavailable")
            || lower.contains("not available")
            || lower.contains("unknown"))
}
