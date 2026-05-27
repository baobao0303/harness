use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

use rusqlite::{params, types::ValueRef, Connection, OptionalExtension};
use thiserror::Error;

use crate::application::{
    BacklogAddInput, BacklogCloseInput, BrownfieldImportResult, DecisionAddInput,
    DecisionVerifyResult, HarnessContext, InitResult, IntakeInput, MigrateResult, QueryTable,
    StoryAddInput, StoryUpdateInput, StoryVerifyResult, TraceInput,
};
use crate::domain::{
    normalize_token, yes_no, BacklogRecord, DecisionRecord, FrictionRecord, HarnessStats,
    IntakeRecord, RiskLane, SkillInfo, SkillResult, StoryMatrixRecord, TraceRecord,
};

pub type Result<T> = std::result::Result<T, HarnessInfraError>;

#[derive(Debug, Error)]
pub enum HarnessInfraError {
    #[error("database not found at {0}. Run: harness init")]
    MissingDatabase(String),
    #[error("brownfield import: missing {0}")]
    MissingBrownfieldPath(String),
    #[error("decision {0} has no verify_command")]
    MissingDecisionVerifyCommand(String),
    #[error("story update: story '{0}' not found")]
    StoryNotFound(String),
    #[error("backlog close: backlog item '{0}' not found")]
    BacklogNotFound(i64),
    #[error("story update: nothing to update")]
    EmptyStoryUpdate,
    #[error("story {0} has no test_skill defined")]
    MissingStoryTestSkill(String),
    #[error("verification script not found for skill {0} under {1}")]
    MissingSkillVerifyScript(String, String),
    #[error("failed to parse verification JSON output from skill {0}: {1}")]
    ParseVerificationOutput(String, String),
    #[error("Skill {0} not found under {1}")]
    SkillNotFound(String, String),
    #[error("Skill {0} has no run.sh wrapper at {1}")]
    MissingSkillWrapper(String, String),
    #[error("failed to parse JSON result from skill {0}: {1}")]
    ParseSkillOutput(String, String),
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub trait HarnessRepository {
    fn init(&self) -> Result<InitResult>;
    fn migrate(&self) -> Result<MigrateResult>;
    fn import_brownfield(&self) -> Result<BrownfieldImportResult>;
    fn record_intake(&self, input: IntakeInput) -> Result<i64>;
    fn add_story(&self, input: StoryAddInput) -> Result<()>;
    fn update_story(&self, input: StoryUpdateInput) -> Result<()>;
    fn story_verify(&self, id: &str) -> Result<StoryVerifyResult>;
    fn add_decision(&self, input: DecisionAddInput) -> Result<()>;
    fn verify_decision(&self, id: &str) -> Result<DecisionVerifyResult>;
    fn add_backlog(&self, input: BacklogAddInput) -> Result<i64>;
    fn close_backlog(&self, input: BacklogCloseInput) -> Result<()>;
    fn record_trace(&self, input: TraceInput) -> Result<i64>;
    fn query_matrix(&self) -> Result<Vec<StoryMatrixRecord>>;
    fn query_backlog(&self) -> Result<Vec<BacklogRecord>>;
    fn query_decisions(&self) -> Result<Vec<DecisionRecord>>;
    fn query_intakes(&self) -> Result<Vec<IntakeRecord>>;
    fn query_traces(&self) -> Result<Vec<TraceRecord>>;
    fn query_friction(&self) -> Result<Vec<FrictionRecord>>;
    fn query_stats(&self) -> Result<HarnessStats>;
    fn query_sql(&self, sql: &str) -> Result<QueryTable>;
    fn db_export(&self, out_path: &str) -> Result<()>;
    fn db_import(&self, file_path: &str) -> Result<()>;
    fn list_skills(&self) -> Result<Vec<SkillInfo>>;
    fn invoke_skill(&self, name: &str, story_id: Option<&str>) -> Result<SkillResult>;
}

const EMBEDDED_SCHEMA_V1: &str = include_str!("../scripts/schema/001-init.sql");

#[derive(Debug)]
pub struct SqliteHarnessRepository {
    repo_root: PathBuf,
    db_path: PathBuf,
    schema_dir: PathBuf,
}

impl SqliteHarnessRepository {
    pub fn new(repo_root: PathBuf, db_path: PathBuf, schema_dir: PathBuf) -> Self {
        Self {
            repo_root,
            db_path,
            schema_dir,
        }
    }

    fn open_existing(&self) -> Result<Connection> {
        if !self.db_path.exists() {
            return Err(HarnessInfraError::MissingDatabase(
                self.db_path.display().to_string(),
            ));
        }

        let connection = Connection::open(&self.db_path)?;
        connection.pragma_update(None, "foreign_keys", "ON")?;
        Ok(connection)
    }

    fn open_or_create(&self) -> Result<Connection> {
        let connection = Connection::open(&self.db_path)?;
        connection.pragma_update(None, "foreign_keys", "ON")?;
        Ok(connection)
    }

    fn schema_version(connection: &Connection) -> Result<i64> {
        let version = connection
            .query_row(
                "SELECT COALESCE(MAX(version),0) FROM schema_version;",
                [],
                |row| row.get::<_, i64>(0),
            )
            .optional()?
            .unwrap_or(0);
        Ok(version)
    }

    fn apply_schema_v1(&self, connection: &Connection) -> Result<()> {
        let schema_path = self.schema_dir.join("001-init.sql");
        let schema = if schema_path.exists() {
            fs::read_to_string(schema_path)?
        } else {
            EMBEDDED_SCHEMA_V1.to_string()
        };

        connection.execute_batch(&schema)?;
        Ok(())
    }

    fn migration_files(&self) -> Result<Vec<(i64, PathBuf)>> {
        let mut files = Vec::new();
        if !self.schema_dir.exists() {
            return Ok(files);
        }
        for entry in fs::read_dir(&self.schema_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|value| value.to_str()) != Some("sql") {
                continue;
            }
            let Some(file_name) = path.file_name().and_then(|value| value.to_str()) else {
                continue;
            };
            let Some(prefix) = file_name.split('-').next() else {
                continue;
            };
            let Ok(version) = prefix.trim_start_matches('0').parse::<i64>() else {
                continue;
            };
            files.push((version, path));
        }
        files.sort_by_key(|(version, _)| *version);
        Ok(files)
    }

    fn import_matrix(&self, connection: &Connection) -> Result<usize> {
        let matrix_path = self.repo_root.join("docs/TEST_MATRIX.md");
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

            let status =
                normalize_story_status(&field_at(&fields, columns.status).unwrap_or_default());
            let unit = proof_from_cell(&field_at(&fields, columns.unit).unwrap_or_default());
            let integration =
                proof_from_cell(&field_at(&fields, columns.integration).unwrap_or_default());
            let e2e = proof_from_cell(&field_at(&fields, columns.e2e).unwrap_or_default());
            let platform =
                proof_from_cell(&field_at(&fields, columns.platform).unwrap_or_default());
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

    fn import_decisions(&self, connection: &Connection) -> Result<usize> {
        let decisions_dir = self.repo_root.join("docs/decisions");
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
            let status =
                normalize_decision_status(&markdown_section_first_value(&content, "Status"));
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

    fn import_backlog(&self, connection: &Connection) -> Result<usize> {
        let backlog_path = self.repo_root.join("docs/HARNESS_BACKLOG.md");
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
}

impl HarnessRepository for SqliteHarnessRepository {
    fn init(&self) -> Result<InitResult> {
        if self.db_path.exists() {
            let connection = self.open_existing()?;
            let current = Self::schema_version(&connection).unwrap_or(0);
            if current == 0 {
                self.apply_schema_v1(&connection)?;
                return Ok(InitResult::MigratedExisting {
                    db_path: self.db_path.clone(),
                });
            }

            return Ok(InitResult::Existing {
                db_path: self.db_path.clone(),
                version: current,
            });
        }

        let connection = self.open_or_create()?;
        self.apply_schema_v1(&connection)?;
        Ok(InitResult::Created {
            db_path: self.db_path.clone(),
        })
    }

    fn migrate(&self) -> Result<MigrateResult> {
        let connection = self.open_existing()?;
        let current_version = Self::schema_version(&connection).unwrap_or(0);
        let mut applied = Vec::new();

        for (version, path) in self.migration_files()? {
            if version > current_version {
                let sql = fs::read_to_string(path)?;
                connection.execute_batch(&sql)?;
                applied.push(version);
            }
        }

        Ok(MigrateResult {
            current_version,
            applied,
        })
    }

    fn import_brownfield(&self) -> Result<BrownfieldImportResult> {
        let connection = self.open_existing()?;
        let stories = self.import_matrix(&connection)?;
        let decisions = self.import_decisions(&connection)?;
        let backlog_items = self.import_backlog(&connection)?;

        Ok(BrownfieldImportResult {
            stories,
            decisions,
            backlog_items,
        })
    }

    fn record_intake(&self, input: IntakeInput) -> Result<i64> {
        let connection = self.open_existing()?;
        connection.execute(
            "INSERT INTO intake (
                input_type, summary, risk_lane, risk_flags, affected_docs, story_id, notes
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);",
            params![
                input.input_type.as_db_value(),
                input.summary,
                input.risk_lane.as_db_value(),
                input.risk_flags.as_json_text(),
                input.affected_docs.as_json_text(),
                input.story_id,
                input.notes,
            ],
        )?;

        Ok(connection.last_insert_rowid())
    }

    fn add_story(&self, input: StoryAddInput) -> Result<()> {
        let connection = self.open_existing()?;
        connection.execute(
            "INSERT INTO story (id, title, risk_lane, contract_doc, test_skill, notes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
            params![
                input.id,
                input.title,
                input.risk_lane.as_db_value(),
                input.contract_doc,
                input.test_skill,
                input.notes,
            ],
        )?;
        Ok(())
    }

    fn update_story(&self, input: StoryUpdateInput) -> Result<()> {
        if input.status.is_none()
            && input.evidence.is_none()
            && input.unit.is_none()
            && input.integration.is_none()
            && input.e2e.is_none()
            && input.platform.is_none()
        {
            return Err(HarnessInfraError::EmptyStoryUpdate);
        }

        let connection = self.open_existing()?;
        connection.execute(
            "UPDATE story SET
                status=COALESCE(?1, status),
                evidence=COALESCE(?2, evidence),
                unit_proof=COALESCE(?3, unit_proof),
                integration_proof=COALESCE(?4, integration_proof),
                e2e_proof=COALESCE(?5, e2e_proof),
                platform_proof=COALESCE(?6, platform_proof)
             WHERE id=?7;",
            params![
                input.status,
                input.evidence,
                input.unit.map(|value| value.0),
                input.integration.map(|value| value.0),
                input.e2e.map(|value| value.0),
                input.platform.map(|value| value.0),
                input.id,
            ],
        )?;

        if connection.changes() == 0 {
            return Err(HarnessInfraError::StoryNotFound(input.id));
        }
        Ok(())
    }

    fn add_decision(&self, input: DecisionAddInput) -> Result<()> {
        let connection = self.open_existing()?;
        connection.execute(
            "INSERT INTO decision (id, title, status, doc_path, verify_command, predicted_impact, notes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);",
            params![
                input.id,
                input.title,
                input.status,
                input.doc_path,
                input.verify_command,
                input.predicted_impact,
                input.notes,
            ],
        )?;
        Ok(())
    }

    fn verify_decision(&self, id: &str) -> Result<DecisionVerifyResult> {
        let connection = self.open_existing()?;
        let verify_command = connection
            .query_row(
                "SELECT verify_command FROM decision WHERE id=?1;",
                params![id],
                |row| row.get::<_, Option<String>>(0),
            )
            .optional()?
            .flatten()
            .filter(|value| !value.is_empty())
            .ok_or_else(|| HarnessInfraError::MissingDecisionVerifyCommand(id.to_owned()))?;

        let status = Command::new("sh")
            .arg("-c")
            .arg(&verify_command)
            .current_dir(&self.repo_root)
            .status()?;
        let result = if status.success() { "pass" } else { "fail" }.to_owned();
        connection.execute(
            "UPDATE decision
             SET last_verified_at=datetime('now'), last_verified_result=?1
             WHERE id=?2;",
            params![result, id],
        )?;

        Ok(DecisionVerifyResult {
            command: verify_command,
            result,
        })
    }

    fn story_verify(&self, id: &str) -> Result<StoryVerifyResult> {
        let connection = self.open_existing()?;

        let test_skill = connection
            .query_row(
                "SELECT test_skill FROM story WHERE id=?1;",
                params![id],
                |row| row.get::<_, Option<String>>(0),
            )
            .optional()?
            .flatten()
            .filter(|value| !value.is_empty())
            .ok_or_else(|| HarnessInfraError::MissingStoryTestSkill(id.to_owned()))?;

        let skill_dir = self.repo_root.join(".agents/skills").join(&test_skill);
        let verify_py = skill_dir.join("verify.py");
        let verify_sh = skill_dir.join("verify.sh");

        let mut command = if verify_py.exists() {
            let mut cmd = Command::new("python3");
            cmd.arg("verify.py");
            cmd
        } else if verify_sh.exists() {
            let mut cmd = Command::new("sh");
            cmd.arg("verify.sh");
            cmd
        } else {
            return Err(HarnessInfraError::MissingSkillVerifyScript(
                test_skill,
                skill_dir.display().to_string(),
            ));
        };

        let output = command.current_dir(&skill_dir).output()?;

        let stdout_str = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr_str = String::from_utf8_lossy(&output.stderr).into_owned();

        #[derive(serde::Deserialize)]
        struct SkillVerifyOutput {
            unit: bool,
            integration: bool,
            e2e: bool,
            platform: bool,
            evidence: String,
        }

        let parsed: SkillVerifyOutput = serde_json::from_str(&stdout_str).map_err(|err| {
            HarnessInfraError::ParseVerificationOutput(
                test_skill.clone(),
                format!("{}, stdout: {}, stderr: {}", err, stdout_str, stderr_str),
            )
        })?;

        // Update the story's proof flags and evidence in the database
        // Also update status to 'implemented' since it is now successfully verified!
        connection.execute(
            "UPDATE story SET
                unit_proof = ?1,
                integration_proof = ?2,
                e2e_proof = ?3,
                platform_proof = ?4,
                evidence = ?5,
                status = 'implemented'
             WHERE id = ?6;",
            params![
                if parsed.unit { 1 } else { 0 },
                if parsed.integration { 1 } else { 0 },
                if parsed.e2e { 1 } else { 0 },
                if parsed.platform { 1 } else { 0 },
                parsed.evidence,
                id,
            ],
        )?;

        Ok(StoryVerifyResult {
            skill_name: test_skill,
            stdout: stdout_str,
            unit: parsed.unit,
            integration: parsed.integration,
            e2e: parsed.e2e,
            platform: parsed.platform,
            evidence: parsed.evidence,
        })
    }

    fn add_backlog(&self, input: BacklogAddInput) -> Result<i64> {
        let connection = self.open_existing()?;
        connection.execute(
            "INSERT INTO backlog (
                title, discovered_while, current_pain, suggested_improvement,
                risk, predicted_impact, notes
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);",
            params![
                input.title,
                input.discovered_while,
                input.current_pain,
                input.suggestion,
                input.risk.map(|value| value.as_db_value().to_owned()),
                input.predicted_impact,
                input.notes,
            ],
        )?;
        Ok(connection.last_insert_rowid())
    }

    fn close_backlog(&self, input: BacklogCloseInput) -> Result<()> {
        let connection = self.open_existing()?;
        connection.execute(
            "UPDATE backlog
             SET status=?1, actual_outcome=?2, implemented_at=datetime('now')
             WHERE id=?3;",
            params![input.status, input.actual_outcome, input.id],
        )?;

        if connection.changes() == 0 {
            return Err(HarnessInfraError::BacklogNotFound(input.id));
        }
        Ok(())
    }

    fn record_trace(&self, input: TraceInput) -> Result<i64> {
        let connection = self.open_existing()?;
        connection.execute(
            "INSERT INTO trace (
                task_summary, intake_id, story_id, agent,
                actions_taken, files_read, files_changed, decisions_made, errors,
                outcome, duration_seconds, token_estimate, harness_friction, notes
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14);",
            params![
                input.task_summary,
                input.intake_id,
                input.story_id,
                input.agent,
                input.actions.as_json_text(),
                input.files_read.as_json_text(),
                input.files_changed.as_json_text(),
                input.decisions.as_json_text(),
                input.errors.as_json_text(),
                input.outcome,
                input.duration_seconds,
                input.token_estimate,
                input.friction,
                input.notes,
            ],
        )?;
        Ok(connection.last_insert_rowid())
    }

    fn query_matrix(&self) -> Result<Vec<StoryMatrixRecord>> {
        let connection = self.open_existing()?;
        let mut statement = connection.prepare(
            "SELECT id, title, status, unit_proof, integration_proof, e2e_proof, platform_proof, evidence, test_skill
             FROM story ORDER BY id;",
        )?;

        let rows = statement.query_map([], |row| {
            Ok(StoryMatrixRecord {
                id: row.get(0)?,
                title: row.get(1)?,
                status: row.get(2)?,
                unit: yes_no(row.get::<_, i64>(3)?),
                integration: yes_no(row.get::<_, i64>(4)?),
                e2e: yes_no(row.get::<_, i64>(5)?),
                platform: yes_no(row.get::<_, i64>(6)?),
                evidence: row.get(7)?,
                test_skill: row.get(8)?,
            })
        })?;

        collect_rows(rows)
    }

    fn query_backlog(&self) -> Result<Vec<BacklogRecord>> {
        let connection = self.open_existing()?;
        let mut statement = connection.prepare(
            "SELECT id, title, status, risk, predicted_impact, actual_outcome
             FROM backlog ORDER BY status, id;",
        )?;

        let rows = statement.query_map([], |row| {
            Ok(BacklogRecord {
                id: row.get(0)?,
                title: row.get(1)?,
                status: row.get(2)?,
                risk: row.get(3)?,
                predicted_impact: row.get(4)?,
                actual_outcome: row.get(5)?,
            })
        })?;

        collect_rows(rows)
    }

    fn query_decisions(&self) -> Result<Vec<DecisionRecord>> {
        let connection = self.open_existing()?;
        let mut statement = connection.prepare(
            "SELECT id, title, status, last_verified_at, last_verified_result
             FROM decision ORDER BY id;",
        )?;

        let rows = statement.query_map([], |row| {
            Ok(DecisionRecord {
                id: row.get(0)?,
                title: row.get(1)?,
                status: row.get(2)?,
                last_verified_at: row.get(3)?,
                last_verified_result: row.get(4)?,
            })
        })?;

        collect_rows(rows)
    }

    fn query_intakes(&self) -> Result<Vec<IntakeRecord>> {
        let connection = self.open_existing()?;
        let mut statement = connection.prepare(
            "SELECT id, created_at, input_type, risk_lane, summary
             FROM intake ORDER BY id DESC LIMIT 20;",
        )?;

        let rows = statement.query_map([], |row| {
            Ok(IntakeRecord {
                id: row.get(0)?,
                created_at: row.get(1)?,
                input_type: row.get(2)?,
                risk_lane: row.get(3)?,
                summary: row.get(4)?,
            })
        })?;

        collect_rows(rows)
    }

    fn query_traces(&self) -> Result<Vec<TraceRecord>> {
        let connection = self.open_existing()?;
        let mut statement = connection.prepare(
            "SELECT id, created_at, outcome, task_summary, harness_friction
             FROM trace ORDER BY id DESC LIMIT 20;",
        )?;

        let rows = statement.query_map([], |row| {
            Ok(TraceRecord {
                id: row.get(0)?,
                created_at: row.get(1)?,
                outcome: row.get(2)?,
                task_summary: row.get(3)?,
                harness_friction: row.get(4)?,
            })
        })?;

        collect_rows(rows)
    }

    fn query_friction(&self) -> Result<Vec<FrictionRecord>> {
        let connection = self.open_existing()?;
        let mut statement = connection.prepare(
            "SELECT id, created_at, task_summary, harness_friction
             FROM trace WHERE harness_friction IS NOT NULL
             ORDER BY id DESC;",
        )?;

        let rows = statement.query_map([], |row| {
            Ok(FrictionRecord {
                id: row.get(0)?,
                created_at: row.get(1)?,
                task_summary: row.get(2)?,
                harness_friction: row.get(3)?,
            })
        })?;

        collect_rows(rows)
    }

    fn query_stats(&self) -> Result<HarnessStats> {
        let connection = self.open_existing()?;
        connection
            .query_row(
                "SELECT
                    (SELECT COUNT(*) FROM intake) AS intakes,
                    (SELECT COUNT(*) FROM story) AS stories,
                    (SELECT COUNT(*) FROM decision) AS decisions,
                    (SELECT COUNT(*) FROM backlog) AS backlog_items,
                    (SELECT COUNT(*) FROM trace) AS traces;",
                [],
                |row| {
                    Ok(HarnessStats {
                        intakes: row.get(0)?,
                        stories: row.get(1)?,
                        decisions: row.get(2)?,
                        backlog_items: row.get(3)?,
                        traces: row.get(4)?,
                    })
                },
            )
            .map_err(HarnessInfraError::from)
    }

    fn query_sql(&self, sql: &str) -> Result<QueryTable> {
        let connection = self.open_existing()?;
        let mut statement = connection.prepare(sql)?;
        let headers = statement
            .column_names()
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>();
        let column_count = statement.column_count();
        let rows = statement.query_map([], |row| {
            let mut values = Vec::new();
            for index in 0..column_count {
                values.push(sql_value_to_string(row.get_ref(index)?));
            }
            Ok(values)
        })?;

        Ok(QueryTable {
            headers,
            rows: collect_rows(rows)?,
        })
    }

    fn db_export(&self, out_path: &str) -> Result<()> {
        let connection = self.open_existing()?;
        let mut script = String::new();

        script.push_str("PRAGMA foreign_keys = OFF;\n\n");
        script.push_str("DELETE FROM trace;\n");
        script.push_str("DELETE FROM backlog;\n");
        script.push_str("DELETE FROM decision;\n");
        script.push_str("DELETE FROM story;\n");
        script.push_str("DELETE FROM intake;\n");
        script.push_str("DELETE FROM schema_version;\n\n");

        let tables = vec![
            ("schema_version", vec!["version", "applied_at"]),
            (
                "intake",
                vec![
                    "id",
                    "created_at",
                    "input_type",
                    "summary",
                    "risk_lane",
                    "risk_flags",
                    "affected_docs",
                    "story_id",
                    "notes",
                ],
            ),
            (
                "story",
                vec![
                    "id",
                    "title",
                    "created_at",
                    "risk_lane",
                    "contract_doc",
                    "status",
                    "unit_proof",
                    "integration_proof",
                    "e2e_proof",
                    "platform_proof",
                    "evidence",
                    "test_skill",
                    "notes",
                ],
            ),
            (
                "decision",
                vec![
                    "id",
                    "title",
                    "created_at",
                    "status",
                    "doc_path",
                    "verify_command",
                    "last_verified_at",
                    "last_verified_result",
                    "predicted_impact",
                    "actual_outcome",
                    "notes",
                ],
            ),
            (
                "backlog",
                vec![
                    "id",
                    "created_at",
                    "title",
                    "discovered_while",
                    "current_pain",
                    "suggested_improvement",
                    "risk",
                    "status",
                    "predicted_impact",
                    "actual_outcome",
                    "implemented_at",
                    "notes",
                ],
            ),
            (
                "trace",
                vec![
                    "id",
                    "created_at",
                    "task_summary",
                    "intake_id",
                    "story_id",
                    "agent",
                    "actions_taken",
                    "files_read",
                    "files_changed",
                    "decisions_made",
                    "errors",
                    "outcome",
                    "duration_seconds",
                    "token_estimate",
                    "harness_friction",
                    "notes",
                ],
            ),
        ];

        for (table_name, columns) in tables {
            let cols_joined = columns.join(", ");
            let sql = format!("SELECT {} FROM {};", cols_joined, table_name);
            let mut statement = connection.prepare(&sql)?;
            let col_count = columns.len();
            let mut rows = statement.query([])?;

            while let Some(row) = rows.next()? {
                let mut values = Vec::new();
                for i in 0..col_count {
                    let val_ref = row.get_ref(i)?;
                    let literal = match val_ref {
                        rusqlite::types::ValueRef::Null => "NULL".to_owned(),
                        rusqlite::types::ValueRef::Integer(n) => n.to_string(),
                        rusqlite::types::ValueRef::Real(f) => f.to_string(),
                        rusqlite::types::ValueRef::Text(t) => {
                            let text = String::from_utf8_lossy(t).into_owned();
                            format!("'{}'", text.replace('\'', "''"))
                        }
                        rusqlite::types::ValueRef::Blob(b) => {
                            let text = String::from_utf8_lossy(b).into_owned();
                            format!("'{}'", text.replace('\'', "''"))
                        }
                    };
                    values.push(literal);
                }
                script.push_str(&format!(
                    "INSERT INTO {} ({}) VALUES ({});\n",
                    table_name,
                    cols_joined,
                    values.join(", ")
                ));
            }
            script.push_str("\n");
        }

        script.push_str("PRAGMA foreign_keys = ON;\n");

        fs::write(out_path, script)?;
        Ok(())
    }

    fn db_import(&self, file_path: &str) -> Result<()> {
        let connection = self.open_existing()?;
        let path = std::path::Path::new(file_path);
        if !path.exists() {
            return Err(HarnessInfraError::MissingBrownfieldPath(
                file_path.to_owned(),
            ));
        }
        let sql = fs::read_to_string(path)?;
        connection.execute_batch(&sql)?;
        Ok(())
    }

    fn list_skills(&self) -> Result<Vec<SkillInfo>> {
        let skills_dir = self.repo_root.join(".agents/skills");
        if !skills_dir.exists() || !skills_dir.is_dir() {
            return Ok(Vec::new());
        }

        let mut skills = Vec::new();
        for entry in fs::read_dir(&skills_dir)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };

            let run_sh_path = path.join("run.sh");
            let has_wrapper = run_sh_path.exists() && run_sh_path.is_file();

            let mut description = String::new();
            let skill_md_path = path.join("SKILL.md");
            if skill_md_path.exists() && skill_md_path.is_file() {
                if let Ok(content) = fs::read_to_string(&skill_md_path) {
                    let mut inside_frontmatter = false;
                    for line in content.lines() {
                        if line.trim() == "---" {
                            if inside_frontmatter {
                                break;
                            } else {
                                inside_frontmatter = true;
                                continue;
                            }
                        }
                        if inside_frontmatter {
                            if let Some(desc) = line.strip_prefix("description:") {
                                let trimmed = desc.trim();
                                let stripped = trimmed
                                    .strip_prefix('"')
                                    .and_then(|s| s.strip_suffix('"'))
                                    .or_else(|| {
                                        trimmed
                                            .strip_prefix('\'')
                                            .and_then(|s| s.strip_suffix('\''))
                                    })
                                    .unwrap_or(trimmed);
                                description = stripped.to_owned();
                            }
                        }
                    }
                }
            }

            if description.is_empty() {
                description = format!("Workflow checklist and guidelines for {}.", name);
            }

            skills.push(SkillInfo {
                name: name.to_owned(),
                description,
                path: path.display().to_string(),
                has_wrapper,
            });
        }

        skills.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(skills)
    }

    fn invoke_skill(&self, name: &str, story_id: Option<&str>) -> Result<SkillResult> {
        let skill_dir = self.repo_root.join(".agents/skills").join(name);
        if !skill_dir.exists() || !skill_dir.is_dir() {
            return Err(HarnessInfraError::SkillNotFound(
                name.to_owned(),
                skill_dir.display().to_string(),
            ));
        }

        let run_sh_path = skill_dir.join("run.sh");
        if !run_sh_path.exists() || !run_sh_path.is_file() {
            return Err(HarnessInfraError::MissingSkillWrapper(
                name.to_owned(),
                run_sh_path.display().to_string(),
            ));
        }

        let mut cmd = Command::new("sh");
        cmd.arg("run.sh");
        if let Some(sid) = story_id {
            cmd.arg(sid);
        }

        let output = cmd.current_dir(&skill_dir).output()?;
        let stdout_str = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr_str = String::from_utf8_lossy(&output.stderr).into_owned();

        if !output.status.success() {
            return Err(HarnessInfraError::ParseSkillOutput(
                name.to_owned(),
                format!(
                    "Command exited with status {}.\nStdout: {}\nStderr: {}",
                    output.status, stdout_str, stderr_str
                ),
            ));
        }

        let parsed: SkillResult = serde_json::from_str(&stdout_str).map_err(|err| {
            HarnessInfraError::ParseSkillOutput(
                name.to_owned(),
                format!("{}, stdout: {}, stderr: {}", err, stdout_str, stderr_str),
            )
        })?;

        Ok(parsed)
    }
}

impl From<HarnessContext> for SqliteHarnessRepository {
    fn from(context: HarnessContext) -> Self {
        Self::new(context.repo_root, context.db_path, context.schema_dir)
    }
}

#[derive(Debug)]
struct MatrixColumns {
    story: Option<usize>,
    contract: Option<usize>,
    unit: Option<usize>,
    integration: Option<usize>,
    e2e: Option<usize>,
    platform: Option<usize>,
    status: Option<usize>,
    evidence: Option<usize>,
}

#[derive(Debug, Default)]
struct BacklogMarkdownItem {
    title: String,
    discovered_while: String,
    current_pain: String,
    suggested_improvement: String,
    risk: String,
    status: String,
}

impl MatrixColumns {
    fn from_header(fields: &[String]) -> Self {
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

fn collect_rows<T>(
    rows: rusqlite::MappedRows<'_, impl FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>>,
) -> Result<Vec<T>> {
    rows.collect::<std::result::Result<Vec<_>, _>>()
        .map_err(HarnessInfraError::from)
}

fn markdown_table_fields(line: &str) -> Vec<String> {
    let trimmed = line.trim();
    let trimmed = trimmed.strip_prefix('|').unwrap_or(trimmed);
    let trimmed = trimmed.strip_suffix('|').unwrap_or(trimmed);
    trimmed
        .split('|')
        .map(|field| field.trim().to_owned())
        .collect()
}

fn field_at(fields: &[String], index: Option<usize>) -> Option<String> {
    index
        .and_then(|value| fields.get(value))
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn evidence_from_fields(fields: &[String], start_index: usize) -> Option<String> {
    fields
        .get(start_index..)
        .map(|values| values.join(" | "))
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn proof_from_cell(value: &str) -> i64 {
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

fn normalize_story_status(value: &str) -> String {
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

fn normalize_decision_status(value: &str) -> String {
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

fn normalize_backlog_status(value: &str) -> String {
    match normalize_token(value).as_str() {
        "proposed" => "proposed",
        "accepted" => "accepted",
        "implemented" => "implemented",
        "rejected" => "rejected",
        _ => "proposed",
    }
    .to_owned()
}

fn markdown_section_first_value(content: &str, heading: &str) -> String {
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

fn backlog_items(content: &str) -> Vec<BacklogMarkdownItem> {
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

fn empty_to_none(value: String) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn is_decision_file_name(file_name: &str) -> bool {
    let Some((prefix, _)) = file_name.split_once('-') else {
        return false;
    };
    prefix.len() == 4 && prefix.chars().all(|character| character.is_ascii_digit())
}

fn sql_value_to_string(value: ValueRef<'_>) -> String {
    match value {
        ValueRef::Null => String::new(),
        ValueRef::Integer(value) => value.to_string(),
        ValueRef::Real(value) => value.to_string(),
        ValueRef::Text(value) => String::from_utf8_lossy(value).into_owned(),
        ValueRef::Blob(value) => format!("<{} bytes>", value.len()),
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;
    use crate::application::{
        BacklogAddInput, BacklogCloseInput, DecisionAddInput, IntakeInput, StoryAddInput,
        StoryUpdateInput, TraceInput,
    };
    use crate::domain::{BoolFlag, CsvList, InputType, RiskLane};

    fn test_repository() -> (TempDir, SqliteHarnessRepository) {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let repository = SqliteHarnessRepository::new(
            repo_root.clone(),
            temp_dir.path().join("harness.db"),
            repo_root.join("scripts/schema"),
        );
        (temp_dir, repository)
    }

    #[test]
    fn init_creates_database_and_schema() {
        let (_temp_dir, repository) = test_repository();

        let result = repository.init().unwrap();

        assert!(matches!(result, InitResult::Created { .. }));
        assert_eq!(repository.query_stats().unwrap().intakes, 0);
    }

    #[test]
    fn records_and_queries_intake() {
        let (_temp_dir, repository) = test_repository();
        repository.init().unwrap();

        let id = repository
            .record_intake(IntakeInput {
                input_type: InputType::HarnessImprovement,
                summary: "Port one CLI slice".to_owned(),
                risk_lane: RiskLane::HighRisk,
                risk_flags: CsvList::from_optional(Some("public contracts".to_owned())),
                affected_docs: CsvList::from_optional(None),
                story_id: Some("US-002".to_owned()),
                notes: None,
            })
            .unwrap();

        let intakes = repository.query_intakes().unwrap();
        assert_eq!(id, 1);
        assert_eq!(intakes[0].summary, "Port one CLI slice");
        assert_eq!(intakes[0].input_type, "harness_improvement");
        assert_eq!(intakes[0].risk_lane, "high_risk");

        let connection = repository.open_existing().unwrap();
        let missing_lists_are_null: (bool, bool) = connection
            .query_row(
                "SELECT risk_flags IS NULL, affected_docs IS NULL FROM intake WHERE id=?1;",
                params![id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(missing_lists_are_null, (false, true));
    }

    #[test]
    fn decision_verify_runs_from_repo_root() {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_root = temp_dir.path().join("repo");
        fs::create_dir_all(&repo_root).unwrap();
        let schema_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scripts/schema");
        let repository = SqliteHarnessRepository::new(
            repo_root.clone(),
            temp_dir.path().join("harness.db"),
            schema_root,
        );
        repository.init().unwrap();

        let pwd_output = temp_dir.path().join("verify-pwd.txt");
        repository
            .add_decision(DecisionAddInput {
                id: "0001-test".to_owned(),
                title: "Verify from root".to_owned(),
                status: "accepted".to_owned(),
                doc_path: None,
                verify_command: Some(format!("pwd > {}", pwd_output.display())),
                predicted_impact: None,
                notes: None,
            })
            .unwrap();

        let result = repository.verify_decision("0001-test").unwrap();

        assert_eq!(result.result, "pass");
        assert_eq!(
            fs::canonicalize(fs::read_to_string(pwd_output).unwrap().trim()).unwrap(),
            fs::canonicalize(repo_root).unwrap()
        );
    }

    #[test]
    fn story_backlog_trace_and_queries_work() {
        let (_temp_dir, repository) = test_repository();
        repository.init().unwrap();
        repository.migrate().unwrap();

        repository
            .add_story(StoryAddInput {
                id: "US-T".to_owned(),
                title: "Test story".to_owned(),
                risk_lane: RiskLane::Normal,
                contract_doc: None,
                test_skill: None,
                notes: None,
            })
            .unwrap();
        repository
            .update_story(StoryUpdateInput {
                id: "US-T".to_owned(),
                status: Some("implemented".to_owned()),
                evidence: Some("unit test".to_owned()),
                unit: Some(BoolFlag(1)),
                integration: None,
                e2e: None,
                platform: None,
            })
            .unwrap();
        assert_eq!(repository.query_matrix().unwrap()[0].unit, "yes");

        let backlog_id = repository
            .add_backlog(BacklogAddInput {
                title: "Improve CLI".to_owned(),
                discovered_while: None,
                current_pain: Some("manual SQL".to_owned()),
                suggestion: None,
                risk: Some(RiskLane::HighRisk),
                predicted_impact: None,
                notes: None,
            })
            .unwrap();
        repository
            .close_backlog(BacklogCloseInput {
                id: backlog_id,
                status: "implemented".to_owned(),
                actual_outcome: Some("done".to_owned()),
            })
            .unwrap();
        assert_eq!(
            repository.query_backlog().unwrap()[0]
                .actual_outcome
                .as_deref(),
            Some("done")
        );

        let trace_id = repository
            .record_trace(TraceInput {
                task_summary: "Test trace".to_owned(),
                intake_id: None,
                story_id: Some("US-T".to_owned()),
                agent: Some("test".to_owned()),
                outcome: Some("completed".to_owned()),
                duration_seconds: None,
                token_estimate: None,
                friction: Some("none".to_owned()),
                notes: None,
                actions: CsvList::from_optional(Some("one,two".to_owned())),
                files_read: CsvList::from_optional(None),
                files_changed: CsvList::from_optional(None),
                decisions: CsvList::from_optional(None),
                errors: CsvList::from_optional(None),
            })
            .unwrap();
        assert_eq!(trace_id, 1);
        assert_eq!(
            repository.query_traces().unwrap()[0].task_summary,
            "Test trace"
        );
        assert_eq!(
            repository.query_friction().unwrap()[0].harness_friction,
            "none"
        );
    }

    #[test]
    fn import_brownfield_seeds_markdown_state_idempotently() {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_root = temp_dir.path().join("repo");
        fs::create_dir_all(repo_root.join("docs/decisions")).unwrap();
        fs::write(
            repo_root.join("docs/TEST_MATRIX.md"),
            r#"# Test Matrix

| Story | Contract | Unit | Integration | E2E | Platform | Status | Evidence |
| --- | --- | --- | --- | --- | --- | --- | --- |
| US-010 | docs/product/tasks.md | yes | pending | no | mac smoke | implemented | cargo test |
"#,
        )
        .unwrap();
        fs::write(
            repo_root.join("docs/decisions/0007-test-decision.md"),
            r#"# Test Decision

## Status

Accepted
"#,
        )
        .unwrap();
        fs::write(
            repo_root.join("docs/HARNESS_BACKLOG.md"),
            r#"# Harness Backlog

## Items

### Title

Import existing docs

### Discovered While

Testing brownfield import

### Current Pain

Existing Harness v0 repos have markdown truth.

### Suggested Improvement

Seed the durable database.

### Risk

normal

### Status

accepted

### Title

Keep installer checksum

### Discovered While

Testing release install

### Current Pain

Downloads need verification.

### Suggested Improvement

Verify sha256 files.

### Risk

high-risk

### Status

implemented
"#,
        )
        .unwrap();

        let source_repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let repository = SqliteHarnessRepository::new(
            repo_root.clone(),
            temp_dir.path().join("harness.db"),
            source_repo_root.join("scripts/schema"),
        );
        repository.init().unwrap();
        repository.migrate().unwrap();

        let first = repository.import_brownfield().unwrap();
        let second = repository.import_brownfield().unwrap();

        assert_eq!(
            first,
            BrownfieldImportResult {
                stories: 1,
                decisions: 1,
                backlog_items: 2,
            }
        );
        assert_eq!(second.backlog_items, 2);

        let matrix = repository.query_matrix().unwrap();
        assert_eq!(matrix[0].id, "US-010");
        assert_eq!(matrix[0].title, "docs/product/tasks.md");
        assert_eq!(matrix[0].status, "implemented");
        assert_eq!(matrix[0].unit, "yes");
        assert_eq!(matrix[0].integration, "no");
        assert_eq!(matrix[0].platform, "yes");

        let decisions = repository.query_decisions().unwrap();
        assert_eq!(decisions[0].id, "0007-test-decision");
        assert_eq!(decisions[0].status, "accepted");

        let backlog = repository.query_backlog().unwrap();
        assert_eq!(backlog.len(), 2);
        assert!(backlog
            .iter()
            .any(|item| item.title == "Import existing docs"
                && item.status == "accepted"
                && item.risk.as_deref() == Some("normal")));
        assert!(backlog
            .iter()
            .any(|item| item.title == "Keep installer checksum"
                && item.status == "implemented"
                && item.risk.as_deref() == Some("high_risk")));
    }

    #[test]
    fn generate_ide_comparison_dashboard_svg() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let svg_path = repo_root.join("docs/assets/ide-comparison-dashboard.svg");

        let svg_content = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1000 800" width="100%" height="100%">
  <!-- Definitions for gradients and shadows -->
  <defs>
    <linearGradient id="bg-grad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#0d0f14"/>
      <stop offset="100%" stop-color="#161922"/>
    </linearGradient>
    <linearGradient id="glow-indigo" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="#4f46e5"/>
      <stop offset="100%" stop-color="#818cf8"/>
    </linearGradient>
    <linearGradient id="glow-cyan" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="#0891b2"/>
      <stop offset="100%" stop-color="#22d3ee"/>
    </linearGradient>
    <linearGradient id="glow-emerald" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="#059669"/>
      <stop offset="100%" stop-color="#34d399"/>
    </linearGradient>
    <linearGradient id="glow-rose" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#fb7185"/>
    </linearGradient>
    <filter id="shadow" x="-10%" y="-10%" width="120%" height="120%">
      <feDropShadow dx="0" dy="4" stdDeviation="6" flood-color="#000000" flood-opacity="0.5"/>
    </filter>
  </defs>

  <style>
    .title { font-family: 'Inter', system-ui, -apple-system, sans-serif; font-size: 28px; font-weight: 800; fill: #ffffff; letter-spacing: -0.5px; }
    .subtitle { font-family: 'Inter', system-ui, -apple-system, sans-serif; font-size: 14px; font-weight: 500; fill: #818cf8; letter-spacing: 0.5px; }
    .col-header { font-family: 'Inter', system-ui, -apple-system, sans-serif; font-size: 13px; font-weight: 700; fill: #94a3b8; text-anchor: middle; }
    .row-header-title { font-family: 'Inter', system-ui, -apple-system, sans-serif; font-size: 14px; font-weight: 700; fill: #e2e8f0; }
    .row-header-desc { font-family: 'Inter', system-ui, -apple-system, sans-serif; font-size: 11px; font-weight: 400; fill: #64748b; }
    .stat-text { font-family: 'Inter', system-ui, -apple-system, sans-serif; font-size: 12px; font-weight: 700; fill: #ffffff; text-anchor: middle; }
    .metric-value { font-family: 'Inter', system-ui, -apple-system, sans-serif; font-size: 12px; font-weight: 600; fill: #cbd5e1; }
    .metric-label { font-family: 'Inter', system-ui, -apple-system, sans-serif; font-size: 10px; font-weight: 500; fill: #64748b; }
  </style>

  <!-- Background Canvas -->
  <rect width="1000" height="800" rx="16" fill="url(#bg-grad)" stroke="#1e293b" stroke-width="1"/>
  
  <!-- Header -->
  <text x="40" y="55" class="title">HARNESS <tspan fill="#6366f1">|</tspan> AI Coding Agent Matrix</text>
  <text x="40" y="80" class="subtitle">AUTOMATED TESTING &amp; CAPABILITY BENCHMARK</text>

  <!-- Headers Background -->
  <rect x="40" y="110" width="920" height="60" rx="8" fill="#111827" opacity="0.6" stroke="#1e293b" stroke-width="1"/>
  <text x="140" y="145" class="row-header-title" fill="#64748b">Comparing Agents</text>

  <!-- Columns Headers -->
  <!-- Claude Code -->
  <rect x="300" y="120" width="95" height="40" rx="6" fill="#1e1b4b" stroke="#312e81" stroke-width="1"/>
  <text x="348" y="144" class="col-header" fill="#a5b4fc">Claude Code</text>
  
  <!-- Cursor -->
  <rect x="410" y="120" width="95" height="40" rx="6" fill="#082f49" stroke="#075985" stroke-width="1"/>
  <text x="458" y="144" class="col-header" fill="#7dd3fc">Cursor</text>

  <!-- Windsurf -->
  <rect x="520" y="120" width="95" height="40" rx="6" fill="#112240" stroke="#0284c7" stroke-width="1"/>
  <text x="568" y="144" class="col-header" fill="#38bdf8">Windsurf</text>

  <!-- GitHub Copilot -->
  <rect x="630" y="120" width="95" height="40" rx="6" fill="#1e1e38" stroke="#3b3b7a" stroke-width="1"/>
  <text x="678" y="144" class="col-header" fill="#a5b4fc">Copilot</text>

  <!-- Antigravity -->
  <rect x="740" y="120" width="95" height="40" rx="6" fill="#2e1065" stroke="#581c87" stroke-width="1"/>
  <text x="788" y="144" class="col-header" fill="#d8b4fe">Antigravity</text>

  <!-- Codex -->
  <rect x="850" y="120" width="95" height="40" rx="6" fill="#022c22" stroke="#065f46" stroke-width="1"/>
  <text x="898" y="144" class="col-header" fill="#6ee7b7">Codex</text>

  <!-- ================= ROW 1: Context Compaction ================= -->
  <g transform="translate(0, 190)">
    <rect x="40" y="0" width="920" height="100" rx="8" fill="#111827" opacity="0.4" stroke="#1e293b" stroke-width="1"/>
    
    <!-- Row Title -->
    <text x="60" y="38" class="row-header-title">Context Compaction</text>
    <text x="60" y="58" class="row-header-desc">Automatic compression of chat</text>
    <text x="60" y="72" class="row-header-desc">history when memory is full.</text>

    <!-- Circles -->
    <!-- Claude Code: 90% -->
    <circle cx="348" cy="50" r="28" fill="none" stroke="#1e293b" stroke-width="6"/>
    <circle cx="348" cy="50" r="28" fill="none" stroke="url(#glow-indigo)" stroke-width="6" stroke-dasharray="176" stroke-dashoffset="17"/>
    <text x="348" y="54" class="stat-text">90%</text>

    <!-- Cursor: 85% -->
    <circle cx="458" cy="50" r="28" fill="none" stroke="#1e293b" stroke-width="6"/>
    <circle cx="458" cy="50" r="28" fill="none" stroke="url(#glow-cyan)" stroke-width="6" stroke-dasharray="176" stroke-dashoffset="26"/>
    <text x="458" y="54" class="stat-text">85%</text>

    <!-- Windsurf: 75% -->
    <circle cx="568" cy="50" r="28" fill="none" stroke="#1e293b" stroke-width="6"/>
    <circle cx="568" cy="50" r="28" fill="none" stroke="url(#glow-cyan)" stroke-width="6" stroke-dasharray="176" stroke-dashoffset="44"/>
    <text x="568" y="54" class="stat-text">75%</text>

    <!-- Copilot: 60% -->
    <circle cx="678" cy="50" r="28" fill="none" stroke="#1e293b" stroke-width="6"/>
    <circle cx="678" cy="50" r="28" fill="none" stroke="url(#glow-indigo)" stroke-width="6" stroke-dasharray="176" stroke-dashoffset="70"/>
    <text x="678" y="54" class="stat-text">60%</text>

    <!-- Antigravity: 95% -->
    <circle cx="788" cy="50" r="28" fill="none" stroke="#1e293b" stroke-width="6"/>
    <circle cx="788" cy="50" r="28" fill="none" stroke="url(#glow-indigo)" stroke-width="6" stroke-dasharray="176" stroke-dashoffset="8"/>
    <text x="788" y="54" class="stat-text">95%</text>

    <!-- Codex: 75% -->
    <circle cx="898" cy="50" r="28" fill="none" stroke="#1e293b" stroke-width="6"/>
    <circle cx="898" cy="50" r="28" fill="none" stroke="url(#glow-emerald)" stroke-width="6" stroke-dasharray="176" stroke-dashoffset="44"/>
    <text x="898" y="54" class="stat-text">75%</text>
  </g>

  <!-- ================= ROW 2: Autonomy &amp; Execution ================= -->
  <g transform="translate(0, 310)">
    <rect x="40" y="0" width="920" height="100" rx="8" fill="#111827" opacity="0.4" stroke="#1e293b" stroke-width="1"/>
    
    <!-- Row Title -->
    <text x="60" y="38" class="row-header-title">Autonomy &amp; Execution</text>
    <text x="60" y="58" class="row-header-desc">Durable task planning and execution</text>
    <text x="60" y="72" class="row-header-desc">without human intervention.</text>

    <!-- Mini Progress Bars -->
    <!-- Claude Code -->
    <text x="310" y="40" class="metric-label">SPEED</text><text x="380" y="40" class="metric-value" text-anchor="end">20ms</text>
    <rect x="310" y="48" width="75" height="4" rx="2" fill="#1e293b"/>
    <rect x="310" y="48" width="60" height="4" rx="2" fill="#818cf8"/>
    <text x="310" y="70" class="metric-label">TASK</text><text x="380" y="70" class="metric-value" text-anchor="end">80%</text>
    <rect x="310" y="75" width="75" height="4" rx="2" fill="#1e293b"/>
    <rect x="310" y="75" width="60" height="4" rx="2" fill="#818cf8"/>

    <!-- Cursor -->
    <text x="420" y="40" class="metric-label">SPEED</text><text x="490" y="40" class="metric-value" text-anchor="end">100ms</text>
    <rect x="420" y="48" width="75" height="4" rx="2" fill="#1e293b"/>
    <rect x="420" y="48" width="20" height="4" rx="2" fill="#22d3ee"/>
    <text x="420" y="70" class="metric-label">TASK</text><text x="490" y="70" class="metric-value" text-anchor="end">30%</text>
    <rect x="420" y="75" width="75" height="4" rx="2" fill="#1e293b"/>
    <rect x="420" y="75" width="25" height="4" rx="2" fill="#22d3ee"/>

    <!-- Windsurf -->
    <text x="530" y="40" class="metric-label">SPEED</text><text x="600" y="40" class="metric-value" text-anchor="end">30ms</text>
    <rect x="530" y="48" width="75" height="4" rx="2" fill="#1e293b"/>
    <rect x="530" y="48" width="55" height="4" rx="2" fill="#22d3ee"/>
    <text x="530" y="70" class="metric-label">TASK</text><text x="600" y="70" class="metric-value" text-anchor="end">80%</text>
    <rect x="530" y="75" width="75" height="4" rx="2" fill="#1e293b"/>
    <rect x="530" y="75" width="60" height="4" rx="2" fill="#22d3ee"/>

    <!-- Copilot -->
    <text x="640" y="40" class="metric-label">SPEED</text><text x="710" y="40" class="metric-value" text-anchor="end">30ms</text>
    <rect x="640" y="48" width="75" height="4" rx="2" fill="#1e293b"/>
    <rect x="640" y="48" width="50" height="4" rx="2" fill="#818cf8"/>
    <text x="640" y="70" class="metric-label">TASK</text><text x="710" y="70" class="metric-value" text-anchor="end">90%</text>
    <rect x="640" y="75" width="75" height="4" rx="2" fill="#1e293b"/>
    <rect x="640" y="75" width="68" height="4" rx="2" fill="#818cf8"/>

    <!-- Antigravity -->
    <text x="750" y="40" class="metric-label">SPEED</text><text x="820" y="40" class="metric-value" text-anchor="end">15ms</text>
    <rect x="750" y="48" width="75" height="4" rx="2" fill="#1e293b"/>
    <rect x="750" y="48" width="70" height="4" rx="2" fill="#a855f7"/>
    <text x="750" y="70" class="metric-label">TASK</text><text x="820" y="70" class="metric-value" text-anchor="end">95%</text>
    <rect x="750" y="75" width="75" height="4" rx="2" fill="#1e293b"/>
    <rect x="750" y="75" width="72" height="4" rx="2" fill="#a855f7"/>

    <!-- Codex -->
    <text x="860" y="40" class="metric-label">SPEED</text><text x="930" y="40" class="metric-value" text-anchor="end">40ms</text>
    <rect x="860" y="48" width="75" height="4" rx="2" fill="#1e293b"/>
    <rect x="860" y="48" width="45" height="4" rx="2" fill="#34d399"/>
    <text x="860" y="70" class="metric-label">TASK</text><text x="930" y="70" class="metric-value" text-anchor="end">90%</text>
    <rect x="860" y="75" width="75" height="4" rx="2" fill="#1e293b"/>
    <rect x="860" y="75" width="68" height="4" rx="2" fill="#34d399"/>
  </g>

  <!-- ================= ROW 3: Sub-agent Isolation ================= -->
  <g transform="translate(0, 430)">
    <rect x="40" y="0" width="920" height="100" rx="8" fill="#111827" opacity="0.4" stroke="#1e293b" stroke-width="1"/>
    
    <!-- Row Title -->
    <text x="60" y="38" class="row-header-title">Sub-agent Isolation</text>
    <text x="60" y="58" class="row-header-desc">Supports launching sandboxed</text>
    <text x="60" y="72" class="row-header-desc">sub-agents for task breakdown.</text>

    <!-- Visual status -->
    <!-- Claude Code: Double Shield -->
    <circle cx="348" cy="50" r="16" fill="#1e1b4b" stroke="#4f46e5" stroke-width="2"/>
    <path d="M343 45l5 2 5-2v3c0 3-2 5-5 6-3-1-5-3-5-6v-3z" fill="none" stroke="#818cf8" stroke-width="2"/>
    
    <!-- Cursor: Checkmark -->
    <circle cx="458" cy="50" r="16" fill="#064e3b" stroke="#059669" stroke-width="2"/>
    <path d="M452 50l4 4 6-8" fill="none" stroke="#34d399" stroke-width="2" stroke-linecap="round"/>

    <!-- Windsurf: Checkmark -->
    <circle cx="568" cy="50" r="16" fill="#064e3b" stroke="#059669" stroke-width="2"/>
    <path d="M562 50l4 4 6-8" fill="none" stroke="#34d399" stroke-width="2" stroke-linecap="round"/>

    <!-- Copilot: Checkmark -->
    <circle cx="678" cy="50" r="16" fill="#064e3b" stroke="#059669" stroke-width="2"/>
    <path d="M672 50l4 4 6-8" fill="none" stroke="#34d399" stroke-width="2" stroke-linecap="round"/>

    <!-- Antigravity: Multi Sandbox (Double Shield Glowing) -->
    <circle cx="788" cy="50" r="18" fill="#3b0764" stroke="#a855f7" stroke-width="2"/>
    <path d="M783 45l5 2 5-2v3c0 3-2 5-5 6-3-1-5-3-5-6v-3z" fill="none" stroke="#c084fc" stroke-width="2"/>

    <!-- Codex: Cross -->
    <circle cx="898" cy="50" r="16" fill="#4c0519" stroke="#be123c" stroke-width="2"/>
    <path d="M893 45l10 10M903 45l-10 10" fill="none" stroke="#fb7185" stroke-width="2" stroke-linecap="round"/>
  </g>

  <!-- ================= ROW 4: Test Verification Integrity ================= -->
  <g transform="translate(0, 550)">
    <rect x="40" y="0" width="920" height="100" rx="8" fill="#111827" opacity="0.4" stroke="#1e293b" stroke-width="1"/>
    
    <!-- Row Title -->
    <text x="60" y="38" class="row-header-title">Test Verification Integrity</text>
    <text x="60" y="58" class="row-header-desc">Automatic validation loop and</text>
    <text x="60" y="72" class="row-header-desc">robust verification gates.</text>

    <!-- Mini chart representations -->
    <!-- Claude Code -->
    <path d="M315 75 q15 -25 35 -15 t35 -35" fill="none" stroke="#818cf8" stroke-width="3" stroke-linecap="round"/>
    <circle cx="385" cy="25" r="4" fill="#818cf8"/>

    <!-- Cursor -->
    <path d="M425 75 q15 -10 35 -15 t35 -20" fill="none" stroke="#22d3ee" stroke-width="3" stroke-linecap="round"/>
    <circle cx="495" cy="40" r="4" fill="#22d3ee"/>

    <!-- Windsurf -->
    <path d="M535 75 q15 -15 35 -5 t35 -25" fill="none" stroke="#22d3ee" stroke-width="3" stroke-linecap="round"/>
    <circle cx="605" cy="45" r="4" fill="#22d3ee"/>

    <!-- Copilot -->
    <path d="M645 75 q15 -20 35 -10 t35 -30" fill="none" stroke="#818cf8" stroke-width="3" stroke-linecap="round"/>
    <circle cx="715" cy="35" r="4" fill="#818cf8"/>

    <!-- Antigravity: Hyper-growth Chart -->
    <path d="M755 75 q15 -35 35 -20 t35 -45" fill="none" stroke="#c084fc" stroke-width="4" stroke-linecap="round"/>
    <circle cx="825" cy="10" r="5" fill="#e9d5ff"/>

    <!-- Codex -->
    <path d="M865 75 q15 -15 35 -15 t35 -15" fill="none" stroke="#34d399" stroke-width="3" stroke-linecap="round"/>
    <circle cx="935" cy="45" r="4" fill="#34d399"/>
  </g>

  <!-- ================= Footer Stats ================= -->
  <text x="40" y="765" font-family="'Inter', system-ui, -apple-system, sans-serif" font-size="11" fill="#475569">Status: Operational • Platform: macos-arm64 • Harness DB: Verified</text>
  <text x="960" y="765" font-family="'Inter', system-ui, -apple-system, sans-serif" font-size="11" fill="#475569" text-anchor="end">Last Updated: 2026-05-24 13:45 AM</text>
</svg>"##;

        fs::write(svg_path, svg_content).unwrap();
    }

    #[test]
    fn db_export_and_import_restores_data() {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let db_path_src = temp_dir.path().join("harness_src.db");
        let db_path_dst = temp_dir.path().join("harness_dst.db");
        let schema_root = repo_root.join("scripts/schema");

        let repo_src = SqliteHarnessRepository::new(
            repo_root.clone(),
            db_path_src.clone(),
            schema_root.clone(),
        );
        repo_src.init().unwrap();
        repo_src.migrate().unwrap();

        // 1. Add some data
        repo_src
            .record_intake(IntakeInput {
                input_type: InputType::NewSpec,
                summary: "Testing DB export".to_owned(),
                risk_lane: RiskLane::Normal,
                risk_flags: CsvList::from_optional(None),
                affected_docs: CsvList::from_optional(None),
                story_id: None,
                notes: None,
            })
            .unwrap();

        repo_src
            .add_story(StoryAddInput {
                id: "US-EXPORT".to_owned(),
                title: "Export Story".to_owned(),
                risk_lane: RiskLane::Normal,
                contract_doc: None,
                test_skill: None,
                notes: None,
            })
            .unwrap();

        // 2. Export to file
        let export_path = temp_dir.path().join("backup.sql");
        repo_src
            .db_export(&export_path.display().to_string())
            .unwrap();

        // 3. Create destination database and import
        let repo_dst = SqliteHarnessRepository::new(repo_root, db_path_dst, schema_root);
        repo_dst.init().unwrap();
        repo_dst.migrate().unwrap();
        repo_dst
            .db_import(&export_path.display().to_string())
            .unwrap();

        // 4. Verify data in destination database
        let intakes = repo_dst.query_intakes().unwrap();
        assert_eq!(intakes.len(), 1);
        assert_eq!(intakes[0].summary, "Testing DB export");

        let stories = repo_dst.query_matrix().unwrap();
        assert_eq!(stories.len(), 1);
        assert_eq!(stories[0].id, "US-EXPORT");
    }

    #[test]
    fn test_story_verify_invokes_skill_subprocess() {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_root = temp_dir.path().to_path_buf();
        let db_path = temp_dir.path().join("harness.db");
        let schema_root = repo_root.join("scripts/schema");
        fs::create_dir_all(&schema_root).unwrap();

        // Write v1 and v2 migrations
        fs::write(
            schema_root.join("001-init.sql"),
            include_str!("../scripts/schema/001-init.sql"),
        )
        .unwrap();
        fs::write(
            schema_root.join("002-add-story-test-skill.sql"),
            include_str!("../scripts/schema/002-add-story-test-skill.sql"),
        )
        .unwrap();

        let repository = SqliteHarnessRepository::new(repo_root.clone(), db_path, schema_root);
        repository.init().unwrap();
        repository.migrate().unwrap();

        // Create skill folder and mock verify.py
        let skill_dir = repo_root.join(".agents/skills/harness-qa-generate-e2e-tests");
        fs::create_dir_all(&skill_dir).unwrap();

        let verify_script = r#"
import json
print(json.dumps({
    "unit": True,
    "integration": True,
    "e2e": True,
    "platform": False,
    "evidence": "Verification succeeded on e2e test suite."
}))
"#;
        fs::write(skill_dir.join("verify.py"), verify_script).unwrap();

        // Add story with test_skill
        repository
            .add_story(StoryAddInput {
                id: "US-SKILL-TEST".to_owned(),
                title: "Skill Test Story".to_owned(),
                risk_lane: RiskLane::Normal,
                contract_doc: None,
                test_skill: Some("harness-qa-generate-e2e-tests".to_owned()),
                notes: None,
            })
            .unwrap();

        // Perform verification
        let result = repository.story_verify("US-SKILL-TEST").unwrap();
        assert_eq!(result.skill_name, "harness-qa-generate-e2e-tests");
        assert!(result.unit);
        assert!(result.integration);
        assert!(result.e2e);
        assert!(!result.platform);
        assert_eq!(result.evidence, "Verification succeeded on e2e test suite.");

        // Query matrix and assert database has updated proof flags and status 'implemented'
        let matrix = repository.query_matrix().unwrap();
        assert_eq!(matrix.len(), 1);
        assert_eq!(matrix[0].id, "US-SKILL-TEST");
        assert_eq!(matrix[0].unit, "yes");
        assert_eq!(matrix[0].integration, "yes");
        assert_eq!(matrix[0].e2e, "yes");
        assert_eq!(matrix[0].platform, "no");
        assert_eq!(
            matrix[0].evidence,
            Some("Verification succeeded on e2e test suite.".to_owned())
        );
        assert_eq!(matrix[0].status, "implemented");
        assert_eq!(
            matrix[0].test_skill,
            Some("harness-qa-generate-e2e-tests".to_owned())
        );
    }

    #[test]
    fn test_list_skills_and_invoke_skill_subprocess() {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_root = temp_dir.path().to_path_buf();
        let db_path = temp_dir.path().join("harness.db");
        let schema_root = repo_root.join("scripts/schema");
        fs::create_dir_all(&schema_root).unwrap();

        // Write v1 migration
        fs::write(
            schema_root.join("001-init.sql"),
            include_str!("../scripts/schema/001-init.sql"),
        )
        .unwrap();

        let repository = SqliteHarnessRepository::new(repo_root.clone(), db_path, schema_root);
        repository.init().unwrap();

        // Create skill folder, SKILL.md, and run.sh
        let skill_dir = repo_root.join(".agents/skills/harness-test-skill");
        fs::create_dir_all(&skill_dir).unwrap();

        let skill_md = r#"---
name: harness-test-skill
description: 'This is a test skill'
---
# Test Skill Workflow
"#;
        fs::write(skill_dir.join("SKILL.md"), skill_md).unwrap();

        let run_sh = r#"#!/bin/sh
echo '{"unit_passed":true,"integration_passed":true,"e2e_passed":false,"platform_passed":false}'
"#;
        fs::write(skill_dir.join("run.sh"), run_sh).unwrap();

        // List skills
        let skills = repository.list_skills().unwrap();
        assert_eq!(skills.len(), 1);
        assert_eq!(skills[0].name, "harness-test-skill");
        assert_eq!(skills[0].description, "This is a test skill");
        assert!(skills[0].has_wrapper);

        // Invoke skill
        let result = repository
            .invoke_skill("harness-test-skill", Some("US-001"))
            .unwrap();
        assert!(result.unit_passed);
        assert!(result.integration_passed);
        assert!(!result.e2e_passed);
        assert!(!result.platform_passed);
    }
}
