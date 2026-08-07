use std::fs;
use std::path::Path;
use std::str::FromStr;

use rusqlite::{params, Connection};

use super::errors::{HarnessInfraError, Result};
use crate::domain::{normalize_token, RiskLane};

#[derive(Debug)]
pub struct MatrixColumns {
    pub story: Option<usize>,
    pub contract: Option<usize>,
    pub unit: Option<usize>,
    pub integration: Option<usize>,
    pub e2e: Option<usize>,
    pub platform: Option<usize>,
    pub status: Option<usize>,
    pub evidence: Option<usize>,
}

impl MatrixColumns {
    pub fn from_header(fields: &[String]) -> Self {
        let mut columns = Self {
            story: None,
            contract: None,
            unit: None,
            integration: None,
            e2e: None,
            platform: None,
            status: None,
            evidence: None,
        };

        for (index, field) in fields.iter().enumerate() {
            match normalize_token(field).as_str() {
                "story" => columns.story = Some(index),
                "contract" => columns.contract = Some(index),
                "unit" => columns.unit = Some(index),
                "integration" => columns.integration = Some(index),
                "e2e" => columns.e2e = Some(index),
                "platform" => columns.platform = Some(index),
                "status" => columns.status = Some(index),
                "evidence" => columns.evidence = Some(index),
                _ => {}
            }
        }

        columns
    }
}

#[derive(Debug, Default)]
pub struct BacklogMarkdownItem {
    pub title: String,
    pub discovered_while: String,
    pub current_pain: String,
    pub suggested_improvement: String,
    pub risk: String,
    pub status: String,
}

pub fn import_matrix(repo_root: &Path, connection: &Connection) -> Result<usize> {
    let matrix_path = repo_root.join("docs/TEST_MATRIX.md");
    if !matrix_path.exists() {
        return Err(HarnessInfraError::MissingBrownfieldPath(
            matrix_path.display().to_string(),
        ));
    }

    let content = fs::read_to_string(matrix_path)?;
    let mut story_count = 0;
    let mut columns: Option<MatrixColumns> = None;

    for line in content.lines() {
        if !line.trim_start().starts_with('|') {
            continue;
        }

        let fields = markdown_table_fields(line);
        if fields.len() < 2 {
            continue;
        }

        if columns.is_none() {
            let candidate = MatrixColumns::from_header(&fields);
            if candidate.story.is_some() && candidate.status.is_some() {
                columns = Some(candidate);
            }
            continue;
        }

        let columns = columns.as_ref().expect("matrix columns discovered");
        let id = field_at(&fields, columns.story).unwrap_or_default();
        let token = normalize_token(&id);
        if matches!(
            token.as_str(),
            "" | "story" | "tbd" | "todo" | "example" | "examples"
        ) || id.chars().all(|character| character == '-')
        {
            continue;
        }

        let mut title = field_at(&fields, columns.contract).unwrap_or_else(|| id.clone());
        if title.is_empty() {
            title = id.clone();
        }

        let status = normalize_story_status(&field_at(&fields, columns.status).unwrap_or_default());
        let unit = proof_from_cell(&field_at(&fields, columns.unit).unwrap_or_default());
        let integration =
            proof_from_cell(&field_at(&fields, columns.integration).unwrap_or_default());
        let e2e = proof_from_cell(&field_at(&fields, columns.e2e).unwrap_or_default());
        let platform = proof_from_cell(&field_at(&fields, columns.platform).unwrap_or_default());
        let evidence = columns
            .evidence
            .and_then(|index| evidence_from_fields(&fields, index));

        connection.execute(
            "INSERT INTO story (
                id, title, risk_lane, contract_doc, status,
                unit_proof, integration_proof, e2e_proof, platform_proof,
                evidence, notes
             ) VALUES (?1, ?2, 'high_risk', ?3, ?4, ?5, ?6, ?7, ?8, ?9,
                'Imported from docs/TEST_MATRIX.md by harness import brownfield.'
             )
             ON CONFLICT(id) DO UPDATE SET
                title=excluded.title,
                contract_doc=excluded.contract_doc,
                status=excluded.status,
                unit_proof=excluded.unit_proof,
                integration_proof=excluded.integration_proof,
                e2e_proof=excluded.e2e_proof,
                platform_proof=excluded.platform_proof,
                evidence=excluded.evidence,
                notes=excluded.notes;",
            params![
                id,
                title,
                field_at(&fields, columns.contract),
                status,
                unit,
                integration,
                e2e,
                platform,
                evidence,
            ],
        )?;
        story_count += 1;
    }

    Ok(story_count)
}

pub fn import_decisions(repo_root: &Path, connection: &Connection) -> Result<usize> {
    let decisions_dir = repo_root.join("docs/decisions");
    if !decisions_dir.is_dir() {
        return Err(HarnessInfraError::MissingBrownfieldPath(
            decisions_dir.display().to_string(),
        ));
    }

    let mut files = Vec::new();
    for entry in fs::read_dir(&decisions_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("md") {
            continue;
        }
        let Some(file_name) = path.file_name().and_then(|value| value.to_str()) else {
            continue;
        };
        if is_decision_file_name(file_name) {
            files.push(path);
        }
    }
    files.sort();

    let mut decision_count = 0;
    for path in files {
        let content = fs::read_to_string(&path)?;
        let stem = path
            .file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_owned();
        let title = content
            .lines()
            .next()
            .and_then(|line| line.strip_prefix("# "))
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or(&stem)
            .to_owned();
        let status = normalize_decision_status(&markdown_section_first_value(&content, "Status"));
        let doc_path = format!(
            "docs/decisions/{}",
            path.file_name()
                .and_then(|value| value.to_str())
                .unwrap_or_default()
        );

        connection.execute(
            "INSERT INTO decision (id, title, status, doc_path, notes)
             VALUES (?1, ?2, ?3, ?4,
                'Imported from docs/decisions by harness import brownfield.'
             )
             ON CONFLICT(id) DO UPDATE SET
                title=excluded.title,
                status=excluded.status,
                doc_path=excluded.doc_path,
                notes=excluded.notes;",
            params![stem, title, status, doc_path],
        )?;
        decision_count += 1;
    }

    Ok(decision_count)
}

pub fn import_backlog(repo_root: &Path, connection: &Connection) -> Result<usize> {
    let backlog_path = repo_root.join("docs/HARNESS_BACKLOG.md");
    if !backlog_path.exists() {
        return Ok(0);
    }

    let content = fs::read_to_string(backlog_path)?;
    let items = backlog_items(&content);
    let mut imported = 0;
    for item in items {
        if item.title.is_empty() || item.title == "Short name." {
            continue;
        }

        let risk = if item.risk.is_empty() {
            None
        } else {
            RiskLane::from_str(&item.risk)
                .ok()
                .map(|value| value.as_db_value().to_owned())
        };
        let status = normalize_backlog_status(&item.status);
        let discovered = empty_to_none(item.discovered_while);
        let pain = empty_to_none(item.current_pain);
        let suggestion = empty_to_none(item.suggested_improvement);

        connection.execute(
            "INSERT INTO backlog (
                title, discovered_while, current_pain, suggested_improvement,
                risk, status, notes
             )
             SELECT ?1, ?2, ?3, ?4, ?5, ?6,
                'Imported from docs/HARNESS_BACKLOG.md by harness import brownfield.'
             WHERE NOT EXISTS (
                SELECT 1 FROM backlog WHERE title=?1
             );",
            params![item.title, discovered, pain, suggestion, risk, status],
        )?;
        imported += 1;
    }

    Ok(imported)
}

pub fn markdown_table_fields(line: &str) -> Vec<String> {
    let trimmed = line.trim();
    let trimmed = trimmed.strip_prefix('|').unwrap_or(trimmed);
    let trimmed = trimmed.strip_suffix('|').unwrap_or(trimmed);
    trimmed
        .split('|')
        .map(|field| field.trim().to_owned())
        .collect()
}

pub fn field_at(fields: &[String], index: Option<usize>) -> Option<String> {
    index
        .and_then(|value| fields.get(value))
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

pub fn evidence_from_fields(fields: &[String], start_index: usize) -> Option<String> {
    fields
        .get(start_index..)
        .map(|values| values.join(" | "))
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

pub fn proof_from_cell(value: &str) -> i64 {
    match normalize_token(value).as_str() {
        ""
        | "no"
        | "none"
        | "n_a"
        | "na"
        | "planned"
        | "pending"
        | "blocked"
        | "not_attempted"
        | "not_operator_reviewed" => 0,
        token
            if token.starts_with("no_")
                || token.starts_with("pending")
                || token.starts_with("blocked")
                || token.contains("pending")
                || token.contains("blocked")
                || token.contains("not_attempted")
                || token.contains("not_operator_reviewed") =>
        {
            0
        }
        _ => 1,
    }
}

pub fn normalize_story_status(value: &str) -> String {
    match normalize_token(value).as_str() {
        "planned" => "planned",
        "in_progress" => "in_progress",
        "implemented" => "implemented",
        "changed" => "changed",
        "retired" => "retired",
        _ => "planned",
    }
    .to_owned()
}

pub fn normalize_decision_status(value: &str) -> String {
    let token = normalize_token(value);
    match token.as_str() {
        "proposed" => "proposed",
        "accepted" => "accepted",
        "superseded" => "superseded",
        "rejected" => "rejected",
        token if token.starts_with("superseded_") => "superseded",
        _ => "accepted",
    }
    .to_owned()
}

pub fn normalize_backlog_status(value: &str) -> String {
    match normalize_token(value).as_str() {
        "proposed" => "proposed",
        "accepted" => "accepted",
        "implemented" => "implemented",
        "rejected" => "rejected",
        _ => "proposed",
    }
    .to_owned()
}

pub fn markdown_section_first_value(content: &str, heading: &str) -> String {
    let target = format!("## {heading}");
    let mut found = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if found && !trimmed.is_empty() {
            return trimmed.to_owned();
        }
        if trimmed == target {
            found = true;
        }
    }
    String::new()
}

pub fn backlog_items(content: &str) -> Vec<BacklogMarkdownItem> {
    let mut in_items = false;
    let mut current_heading = String::new();
    let mut current = BacklogMarkdownItem::default();
    let mut items = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "## Items" {
            in_items = true;
            current_heading.clear();
            continue;
        }
        if !in_items {
            continue;
        }

        if let Some(heading) = trimmed.strip_prefix("### ") {
            let normalized = normalize_token(heading);
            if normalized == "title" && !current.title.is_empty() {
                items.push(current);
                current = BacklogMarkdownItem::default();
            }
            current_heading = normalized;
            continue;
        }

        if trimmed.is_empty() || current_heading.is_empty() {
            continue;
        }

        let target = match current_heading.as_str() {
            "title" => &mut current.title,
            "discovered_while" => &mut current.discovered_while,
            "current_pain" => &mut current.current_pain,
            "suggested_improvement" => &mut current.suggested_improvement,
            "risk" => &mut current.risk,
            "status" => &mut current.status,
            _ => continue,
        };
        if target.is_empty() {
            *target = trimmed.to_owned();
        }
    }

    if !current.title.is_empty() {
        items.push(current);
    }
    items
}

pub fn empty_to_none(value: String) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

pub fn is_decision_file_name(file_name: &str) -> bool {
    let Some((prefix, _)) = file_name.split_once('-') else {
        return false;
    };
    prefix.len() == 4 && prefix.chars().all(|character| character.is_ascii_digit())
}
