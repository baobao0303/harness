pub mod queries;
pub mod schema;

use std::path::PathBuf;
use std::process::Command;

use rusqlite::{params, Connection, OptionalExtension};

pub use queries::*;
pub use schema::*;

use crate::application::{
    BacklogAddInput, BacklogCloseInput, BrownfieldImportResult, DecisionAddInput,
    DecisionVerifyResult, HarnessContext, InitResult, IntakeInput, InterventionAddInput,
    InterventionFilter, MigrateResult, QueryTable, StoryAddInput, StoryUpdateInput,
    StoryVerifyResult, ToolRegisterInput, TraceInput, WorkItemAddInput, WorkItemUpdateInput,
};
use crate::domain::{
    compiled_tool_registry, normalize_token, score_context, score_trace, tags_to_json,
    validate_state_transition, validate_tool_description, validate_work_item_description,
    validate_work_item_title, AuditFinding, AuditResult, BacklogFilter, BacklogRecord,
    ContextScoreResult, ContextScoreSource, DecisionRecord, FrictionRecord, HarnessStats,
    ImprovementProposal, IntakeRecord, InterventionRecord, StoryMatrixRecord,
    StoryVerifyAllItem, StoryVerifyAllResult, StoryVerifyStatus, ToolEntry, TraceRecord,
    TraceScoreResult, WorkItem, WorkItemState, WorkItemType,
};



use crate::infrastructure::brownfield::{import_backlog, import_decisions, import_matrix};
use crate::infrastructure::errors::{HarnessInfraError, Result};
use crate::infrastructure::process::{command_available, verifier_shell};
use crate::infrastructure::HarnessRepository;

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

    pub fn open_existing(&self) -> Result<Connection> {
        if !self.db_path.exists() {
            return Err(HarnessInfraError::MissingDatabase(
                self.db_path.display().to_string(),
            ));
        }

        let connection = Connection::open(&self.db_path)?;
        connection.pragma_update(None, "foreign_keys", "ON")?;
        Ok(connection)
    }

    pub fn open_or_create(&self) -> Result<Connection> {
        let connection = Connection::open(&self.db_path)?;
        connection.pragma_update(None, "foreign_keys", "ON")?;
        Ok(connection)
    }
}

impl HarnessRepository for SqliteHarnessRepository {
    fn init(&self) -> Result<InitResult> {
        if self.db_path.exists() {
            let connection = self.open_existing()?;
            let current = schema_version(&connection).unwrap_or(0);
            if current == 0 {
                apply_schema_v1(&self.schema_dir, &connection)?;
                apply_pending_migrations(&self.schema_dir, &connection, 1)?;
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
        apply_schema_v1(&self.schema_dir, &connection)?;
        apply_pending_migrations(&self.schema_dir, &connection, 1)?;
        Ok(InitResult::Created {
            db_path: self.db_path.clone(),
        })
    }

    fn migrate(&self) -> Result<MigrateResult> {
        let connection = self.open_existing()?;
        let current_version = schema_version(&connection).unwrap_or(0);
        let applied = apply_pending_migrations(&self.schema_dir, &connection, current_version)?;

        Ok(MigrateResult {
            current_version,
            applied,
        })
    }

    fn import_brownfield(&self) -> Result<BrownfieldImportResult> {
        let connection = self.open_existing()?;
        let stories = import_matrix(&self.repo_root, &connection)?;
        let decisions = import_decisions(&self.repo_root, &connection)?;
        let backlog_items = import_backlog(&self.repo_root, &connection)?;

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

    fn add_work_item(&self, input: WorkItemAddInput) -> Result<i64> {
        validate_work_item_title(input.work_type, &input.title)?;
        validate_work_item_description(input.work_type, input.description.as_deref())?;

        let connection = self.open_existing()?;
        connection.execute(
            "INSERT INTO work_item (
                type, title, description, state, assigned_to, story_points, remaining_work,
                priority, severity, parent_id, area_path, iteration_path, tags,
                acceptance_criteria, repro_steps, actual_result, expected_result, steps
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18);",
            params![
                input.work_type.as_db_value(),
                input.title,
                input.description,
                WorkItemState::New.as_db_value(),
                input.assigned_to,
                input.story_points,
                input.remaining_work,
                input.priority.as_db_value(),
                input.severity.as_ref().map(|s| s.as_db_value()),
                input.parent_id,
                input.area_path,
                input.iteration_path,
                tags_to_json(&input.tags),
                input.acceptance_criteria,
                input.repro_steps,
                input.actual_result,
                input.expected_result,
                input.steps,
            ],
        )?;

        Ok(connection.last_insert_rowid())
    }

    fn update_work_item(&self, input: WorkItemUpdateInput) -> Result<()> {
        let existing = self
            .get_work_item(input.id)?
            .ok_or(HarnessInfraError::WorkItemNotFound(input.id))?;

        if input.title.is_none()
            && input.description.is_none()
            && input.state.is_none()
            && input.assigned_to.is_none()
            && input.story_points.is_none()
            && input.remaining_work.is_none()
            && input.priority.is_none()
            && input.severity.is_none()
            && input.parent_id.is_none()
            && input.area_path.is_none()
            && input.iteration_path.is_none()
            && input.tags.is_none()
            && input.acceptance_criteria.is_none()
            && input.repro_steps.is_none()
            && input.actual_result.is_none()
            && input.expected_result.is_none()
            && input.steps.is_none()
        {
            return Err(HarnessInfraError::EmptyWorkItemUpdate);
        }

        if let Some(ref new_title) = input.title {
            validate_work_item_title(existing.work_type, new_title)?;
        }

        if let Some(ref new_desc) = input.description {
            validate_work_item_description(existing.work_type, Some(new_desc))?;
        }

        if let Some(target_state) = input.state {
            validate_state_transition(existing.work_type, existing.state, target_state)?;
        }

        let connection = self.open_existing()?;
        connection.execute(
            "UPDATE work_item SET
                title = COALESCE(?1, title),
                description = COALESCE(?2, description),
                state = COALESCE(?3, state),
                assigned_to = COALESCE(?4, assigned_to),
                story_points = COALESCE(?5, story_points),
                remaining_work = COALESCE(?6, remaining_work),
                priority = COALESCE(?7, priority),
                severity = COALESCE(?8, severity),
                parent_id = COALESCE(?9, parent_id),
                area_path = COALESCE(?10, area_path),
                iteration_path = COALESCE(?11, iteration_path),
                tags = COALESCE(?12, tags),
                acceptance_criteria = COALESCE(?13, acceptance_criteria),
                repro_steps = COALESCE(?14, repro_steps),
                actual_result = COALESCE(?15, actual_result),
                expected_result = COALESCE(?16, expected_result),
                steps = COALESCE(?17, steps),
                updated_at = datetime('now')
             WHERE id = ?18;",
            params![
                input.title,
                input.description,
                input.state.as_ref().map(|s| s.as_db_value()),
                input.assigned_to,
                input.story_points,
                input.remaining_work,
                input.priority.as_ref().map(|p| p.as_db_value()),
                input.severity.as_ref().map(|s| s.as_db_value()),
                input.parent_id,
                input.area_path,
                input.iteration_path,
                input.tags.as_ref().map(|t| tags_to_json(t)),
                input.acceptance_criteria,
                input.repro_steps,
                input.actual_result,
                input.expected_result,
                input.steps,
                input.id,
            ],
        )?;

        Ok(())
    }

    fn get_work_item(&self, id: i64) -> Result<Option<WorkItem>> {
        let connection = self.open_existing()?;
        let work_item = connection
            .query_row(
                "SELECT
                    id, type, title, description, state, assigned_to, story_points, remaining_work,
                    priority, severity, parent_id, area_path, iteration_path, tags,
                    acceptance_criteria, repro_steps, actual_result, expected_result, steps,
                    created_at, updated_at
                 FROM work_item
                 WHERE id = ?1;",
                params![id],
                work_item_from_row,
            )
            .optional()?;
        Ok(work_item)
    }

    fn query_work_items(
        &self,
        work_type: Option<WorkItemType>,
        state: Option<WorkItemState>,
    ) -> Result<Vec<WorkItem>> {
        let connection = self.open_existing()?;
        let mut statement = connection.prepare(
            "SELECT
                id, type, title, description, state, assigned_to, story_points, remaining_work,
                priority, severity, parent_id, area_path, iteration_path, tags,
                acceptance_criteria, repro_steps, actual_result, expected_result, steps,
                created_at, updated_at
             FROM work_item
             WHERE (?1 IS NULL OR type = ?1)
               AND (?2 IS NULL OR state = ?2)
             ORDER BY id;",
        )?;

        let rows = statement.query_map(
            params![
                work_type.map(|t| t.as_db_value()),
                state.map(|s| s.as_db_value())
            ],
            work_item_from_row,
        )?;

        collect_rows(rows)
    }

    fn add_story(&self, input: StoryAddInput) -> Result<()> {
        let connection = self.open_existing()?;
        connection.execute(
            "INSERT INTO story (id, title, risk_lane, contract_doc, verify_command, notes, priority)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);",
            params![
                input.id,
                input.title,
                input.risk_lane.as_db_value(),
                input.contract_doc,
                input.verify_command,
                input.notes,
                input.priority.as_db_value(),
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
            && input.verify_command.is_none()
            && input.priority.is_none()
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
                platform_proof=COALESCE(?6, platform_proof),
                verify_command=COALESCE(?7, verify_command),
                priority=COALESCE(?8, priority)
             WHERE id=?9;",
            params![
                input.status,
                input.evidence,
                input.unit.map(|value| value.0),
                input.integration.map(|value| value.0),
                input.e2e.map(|value| value.0),
                input.platform.map(|value| value.0),
                input.verify_command,
                input.priority.as_ref().map(|p| p.as_db_value()),
                input.id,
            ],
        )?;

        if connection.changes() == 0 {
            return Err(HarnessInfraError::StoryNotFound(input.id));
        }
        Ok(())
    }

    fn verify_story(&self, id: &str) -> Result<StoryVerifyResult> {
        let connection = self.open_existing()?;
        let verify_command = connection
            .query_row(
                "SELECT verify_command FROM story WHERE id=?1;",
                params![id],
                |row| row.get::<_, Option<String>>(0),
            )
            .optional()?
            .flatten()
            .filter(|value| !value.is_empty())
            .ok_or_else(|| HarnessInfraError::MissingStoryVerifyCommand(id.to_owned()))?;

        let (shell, flag) = verifier_shell();
        let output = Command::new(shell)
            .arg(flag)
            .arg(&verify_command)
            .current_dir(&self.repo_root)
            .output()?;
        let result = if output.status.success() {
            "pass"
        } else {
            "fail"
        }
        .to_owned();
        connection.execute(
            "UPDATE story
             SET last_verified_at=datetime('now'), last_verified_result=?1
             WHERE id=?2;",
            params![result, id],
        )?;

        Ok(StoryVerifyResult {
            command: verify_command,
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            result,
        })
    }

    fn verify_all_stories(&self) -> Result<StoryVerifyAllResult> {
        let connection = self.open_existing()?;
        let mut statement =
            connection.prepare("SELECT id, title, verify_command FROM story ORDER BY id;")?;
        let story_rows = statement.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
            ))
        })?;
        let stories = collect_rows(story_rows)?;
        let mut items = Vec::new();

        for (id, title, verify_command) in stories {
            let Some(command) = verify_command.filter(|value| !value.trim().is_empty()) else {
                items.push(StoryVerifyAllItem {
                    id,
                    title,
                    command: None,
                    result: "skipped".to_owned(),
                    stdout: String::new(),
                    stderr: String::new(),
                });
                continue;
            };

            let (shell, flag) = verifier_shell();
            let output = Command::new(shell)
                .arg(flag)
                .arg(&command)
                .current_dir(&self.repo_root)
                .output()?;
            let result = if output.status.success() {
                "pass"
            } else {
                "fail"
            }
            .to_owned();
            connection.execute(
                "UPDATE story
                 SET last_verified_at=datetime('now'), last_verified_result=?1
                 WHERE id=?2;",
                params![result, id],
            )?;
            items.push(StoryVerifyAllItem {
                id,
                title,
                command: Some(command),
                result,
                stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            });
        }

        Ok(StoryVerifyAllResult { items })
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

        let (shell, flag) = verifier_shell();
        let status = Command::new(shell)
            .arg(flag)
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

    fn add_backlog(&self, input: BacklogAddInput) -> Result<i64> {
        let connection = self.open_existing()?;
        connection.execute(
            "INSERT INTO backlog (
                title, discovered_while, current_pain, suggested_improvement,
                risk, predicted_impact, notes, priority
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);",
            params![
                input.title,
                input.discovered_while,
                input.current_pain,
                input.suggestion,
                input.risk.map(|value| value.as_db_value().to_owned()),
                input.predicted_impact,
                input.notes,
                input.priority.as_db_value(),
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

    fn register_tool(&self, input: ToolRegisterInput) -> Result<()> {
        validate_tool_description(&input.description)?;
        if !input.force && !command_available(&self.repo_root, &input.command) {
            return Err(HarnessInfraError::ToolCommandNotFound(input.command));
        }

        let connection = self.open_existing()?;
        let existing = connection
            .query_row(
                "SELECT command FROM tool WHERE name=?1;",
                params![input.name],
                |row| row.get::<_, String>(0),
            )
            .optional()?;
        if let Some(command) = existing {
            return Err(HarnessInfraError::ToolAlreadyExists(input.name, command));
        }

        connection.execute(
            "INSERT INTO tool (name, provider, command, description, args, responsibility, since)
             VALUES (?1, 'custom', ?2, ?3, ?4, ?5, 'registered');",
            params![
                input.name,
                input.command,
                input.description,
                tool_args_json(&input.args),
                input.responsibility,
            ],
        )?;
        Ok(())
    }

    fn remove_tool(&self, name: &str) -> Result<()> {
        let connection = self.open_existing()?;
        connection.execute("DELETE FROM tool WHERE name=?1;", params![name])?;
        if connection.changes() == 0 {
            return Err(HarnessInfraError::ToolNotFound(name.to_owned()));
        }
        Ok(())
    }

    fn add_intervention(&self, input: InterventionAddInput) -> Result<i64> {
        let connection = self.open_existing()?;
        connection.execute(
            "INSERT INTO intervention (trace_id, story_id, type, description, source, impact)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
            params![
                input.trace_id,
                input.story_id,
                input.intervention_type,
                input.description,
                input.source,
                input.impact,
            ],
        )?;
        Ok(connection.last_insert_rowid())
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

    fn score_trace(&self, id: Option<i64>) -> Result<TraceScoreResult> {
        let connection = self.open_existing()?;
        let sql = match id {
            Some(_) => {
                "SELECT
                    trace.id,
                    trace.task_summary,
                    trace.intake_id,
                    intake.risk_lane,
                    trace.agent,
                    trace.actions_taken,
                    trace.files_read,
                    trace.files_changed,
                    trace.decisions_made,
                    trace.errors,
                    trace.outcome,
                    trace.duration_seconds,
                    trace.token_estimate,
                    trace.harness_friction,
                    trace.notes
                 FROM trace
                 LEFT JOIN intake ON intake.id = trace.intake_id
                 WHERE trace.id = ?1"
            }
            None => {
                "SELECT
                    trace.id,
                    trace.task_summary,
                    trace.intake_id,
                    intake.risk_lane,
                    trace.agent,
                    trace.actions_taken,
                    trace.files_read,
                    trace.files_changed,
                    trace.decisions_made,
                    trace.errors,
                    trace.outcome,
                    trace.duration_seconds,
                    trace.token_estimate,
                    trace.harness_friction,
                    trace.notes
                 FROM trace
                 LEFT JOIN intake ON intake.id = trace.intake_id
                 ORDER BY trace.id DESC
                 LIMIT 1"
            }
        };

        let source = if let Some(id) = id {
            connection
                .query_row(sql, params![id], trace_score_source_from_row)
                .optional()?
                .ok_or(HarnessInfraError::TraceNotFound(id))?
        } else {
            connection
                .query_row(sql, [], trace_score_source_from_row)
                .optional()?
                .ok_or(HarnessInfraError::NoTraces)?
        };

        Ok(score_trace(source))
    }

    fn score_context(&self, id: i64) -> Result<ContextScoreResult> {
        let connection = self.open_existing()?;
        let source = connection
            .query_row(
                "SELECT
                    trace.id,
                    intake.risk_lane,
                    trace.story_id,
                    trace.files_read,
                    trace.files_changed,
                    trace.outcome
                 FROM trace
                 LEFT JOIN intake ON intake.id = trace.intake_id
                 WHERE trace.id=?1;",
                params![id],
                |row| {
                    Ok(ContextScoreSource {
                        id: row.get(0)?,
                        risk_lane: row.get(1)?,
                        story_id: row.get(2)?,
                        files_read: row.get(3)?,
                        files_changed: row.get(4)?,
                        outcome: row.get(5)?,
                    })
                },
            )
            .optional()?
            .ok_or(HarnessInfraError::TraceNotFound(id))?;

        Ok(score_context(source))
    }

    fn story_verify_status(&self, id: &str) -> Result<StoryVerifyStatus> {
        let connection = self.open_existing()?;
        connection
            .query_row(
                "SELECT id, verify_command, last_verified_result FROM story WHERE id=?1;",
                params![id],
                |row| {
                    Ok(StoryVerifyStatus {
                        id: row.get(0)?,
                        verify_command: row.get(1)?,
                        last_verified_result: row.get(2)?,
                    })
                },
            )
            .optional()?
            .ok_or_else(|| HarnessInfraError::StoryNotFound(id.to_owned()))
    }

    fn query_matrix(&self) -> Result<Vec<StoryMatrixRecord>> {
        let connection = self.open_existing()?;
        let mut statement = connection.prepare(
            "SELECT id, title, status, unit_proof, integration_proof, e2e_proof, platform_proof, evidence, priority
             FROM story ORDER BY id;",
        )?;

        let rows = statement.query_map([], |row| {
            Ok(StoryMatrixRecord {
                id: row.get(0)?,
                title: row.get(1)?,
                status: row.get(2)?,
                unit: row.get(3)?,
                integration: row.get(4)?,
                e2e: row.get(5)?,
                platform: row.get(6)?,
                evidence: row.get(7)?,
                priority: row.get(8)?,
            })
        })?;

        collect_rows(rows)
    }

    fn query_backlog(&self, filter: BacklogFilter) -> Result<Vec<BacklogRecord>> {
        let connection = self.open_existing()?;
        let where_clause = match filter {
            BacklogFilter::All => "",
            BacklogFilter::Open => "WHERE status IN ('proposed', 'accepted')",
            BacklogFilter::Closed => "WHERE status IN ('implemented', 'rejected')",
        };
        let sql = format!(
            "SELECT id, title, status, risk, predicted_impact, actual_outcome, priority
             FROM backlog {where_clause} ORDER BY status, id;"
        );
        let mut statement = connection.prepare(&sql)?;

        let rows = statement.query_map([], |row| {
            Ok(BacklogRecord {
                id: row.get(0)?,
                title: row.get(1)?,
                status: row.get(2)?,
                risk: row.get(3)?,
                predicted_impact: row.get(4)?,
                actual_outcome: row.get(5)?,
                priority: row.get(6)?,
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
            "SELECT
                trace.id,
                trace.created_at,
                intake.risk_lane,
                intake.input_type,
                trace.task_summary,
                trace.harness_friction
             FROM trace
             LEFT JOIN intake ON intake.id = trace.intake_id
             WHERE trace.harness_friction IS NOT NULL
             ORDER BY trace.id DESC;",
        )?;

        let rows = statement.query_map([], |row| {
            Ok(FrictionRecord {
                id: row.get(0)?,
                created_at: row.get(1)?,
                risk_lane: row.get(2)?,
                input_type: row.get(3)?,
                task_summary: row.get(4)?,
                harness_friction: row.get(5)?,
            })
        })?;

        collect_rows(rows)
    }

    fn query_tools(&self, responsibility: Option<String>) -> Result<Vec<ToolEntry>> {
        let connection = self.open_existing()?;
        let mut tools = compiled_tool_registry();
        let mut statement = connection.prepare(
            "SELECT provider, name, command, description, args, responsibility, since
             FROM tool ORDER BY name;",
        )?;
        let rows = statement.query_map([], |row| {
            Ok(ToolEntry {
                provider: row.get(0)?,
                name: row.get(1)?,
                command: row.get(2)?,
                description: row.get(3)?,
                args: parse_stored_tool_args(row.get::<_, Option<String>>(4)?.as_deref()),
                responsibility: row.get(5)?,
                source: "registered".to_owned(),
                since: row.get(6)?,
            })
        })?;
        tools.extend(collect_rows(rows)?);
        if let Some(responsibility) = responsibility {
            let normalized = normalize_token(&responsibility);
            tools.retain(|tool| normalize_token(&tool.responsibility) == normalized);
        }
        Ok(tools)
    }

    fn query_interventions(&self, filter: InterventionFilter) -> Result<Vec<InterventionRecord>> {
        let connection = self.open_existing()?;
        let mut statement = connection.prepare(
            "SELECT id, created_at, trace_id, story_id, type, description, source, impact
             FROM intervention
             WHERE (?1 IS NULL OR trace_id = ?1)
               AND (?2 IS NULL OR story_id = ?2)
               AND (?3 IS NULL OR type = ?3)
             ORDER BY id DESC;",
        )?;
        let rows = statement.query_map(
            params![filter.trace_id, filter.story_id, filter.intervention_type],
            |row| {
                Ok(InterventionRecord {
                    id: row.get(0)?,
                    created_at: row.get(1)?,
                    trace_id: row.get(2)?,
                    story_id: row.get(3)?,
                    intervention_type: row.get(4)?,
                    description: row.get(5)?,
                    source: row.get(6)?,
                    impact: row.get(7)?,
                })
            },
        )?;
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

    fn audit(&self) -> Result<AuditResult> {
        let connection = self.open_existing()?;
        let mut result = AuditResult {
            orphaned_stories: audit_findings(
                &connection,
                "SELECT story.id, story.title
                 FROM story
                 LEFT JOIN trace ON trace.story_id = story.id
                 WHERE story.status IN ('planned','in_progress') AND trace.id IS NULL
                 ORDER BY story.id;",
            )?,
            unverified_stories: audit_findings(
                &connection,
                "SELECT id, title FROM story
                 WHERE verify_command IS NOT NULL
                   AND TRIM(verify_command) <> ''
                   AND last_verified_result IS NULL
                 ORDER BY id;",
            )?,
            unverified_decisions: audit_findings(
                &connection,
                "SELECT id, title FROM decision
                 WHERE verify_command IS NOT NULL
                   AND TRIM(verify_command) <> ''
                   AND last_verified_result IS NULL
                 ORDER BY id;",
            )?,
            backlog_without_outcomes: audit_findings(
                &connection,
                "SELECT CAST(id AS TEXT), title FROM backlog
                 WHERE predicted_impact IS NOT NULL
                   AND actual_outcome IS NULL
                   AND status='implemented'
                 ORDER BY id;",
            )?,
            stale_stories: audit_findings(
                &connection,
                "SELECT story.id, story.title
                 FROM story
                 JOIN trace ON trace.story_id = story.id
                 WHERE story.status <> 'implemented'
                 GROUP BY story.id, story.title
                 HAVING julianday('now') - julianday(MAX(trace.created_at)) > 30
                 ORDER BY story.id;",
            )?,
            broken_tools: Vec::new(),
        };

        let mut statement = connection.prepare("SELECT name, command FROM tool ORDER BY name;")?;
        let rows = statement.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        for row in collect_rows(rows)? {
            if !command_available(&self.repo_root, &row.1) {
                result.broken_tools.push(AuditFinding {
                    id: row.0,
                    title: row.1,
                });
            }
        }
        Ok(result)
    }

    fn propose(&self, commit: bool) -> Result<Vec<ImprovementProposal>> {
        let connection = self.open_existing()?;
        let audit = self.audit()?;
        let mut proposals = Vec::new();

        for (text, count) in repeated_friction(&connection)? {
            proposals.push(ImprovementProposal {
                title: format!("Reduce repeated friction: {}", short_title(&text)),
                component: "Failure attribution".to_owned(),
                evidence: format!("{count} traces recorded similar friction: {text}"),
                predicted_impact: "Fewer repeated harness friction entries for similar tasks.".to_owned(),
                risk: "normal".to_owned(),
                suggested_action: "Update the relevant Harness docs, templates, or CLI guidance for this friction pattern.".to_owned(),
                validation_plan: "Review the next five related traces and compare friction frequency.".to_owned(),
                confidence: confidence_for_count(count),
                committed_backlog_id: None,
            });
        }

        for (key, count) in repeated_interventions(&connection)? {
            proposals.push(ImprovementProposal {
                title: format!("Address repeated intervention: {}", short_title(&key)),
                component: "Intervention recording".to_owned(),
                evidence: format!("{count} interventions share the pattern: {key}"),
                predicted_impact: "Fewer repeated human or review interventions for the same issue.".to_owned(),
                risk: "normal".to_owned(),
                suggested_action: "Clarify the relevant operating rule or validation gate that would have caught this earlier.".to_owned(),
                validation_plan: "Future interventions of this type should decrease after the rule change.".to_owned(),
                confidence: confidence_for_count(count),
                committed_backlog_id: None,
            });
        }

        for (category, count) in [
            (
                "orphaned planned or in-progress stories",
                audit.orphaned_stories.len(),
            ),
            ("unverified story commands", audit.unverified_stories.len()),
            (
                "unverified decision commands",
                audit.unverified_decisions.len(),
            ),
            (
                "implemented backlog items without outcomes",
                audit.backlog_without_outcomes.len(),
            ),
            ("stale unfinished stories", audit.stale_stories.len()),
            ("broken registered tools", audit.broken_tools.len()),
        ] {
            if count > 0 {
                proposals.push(ImprovementProposal {
                    title: format!("Clean up {category}"),
                    component: "Entropy auditing".to_owned(),
                    evidence: format!("Audit found {count} {category}."),
                    predicted_impact: "Lower entropy score and stronger completion evidence.".to_owned(),
                    risk: "tiny".to_owned(),
                    suggested_action: "Resolve the listed audit findings or record why they are intentionally retained.".to_owned(),
                    validation_plan: "Run harness-cli audit and confirm the category count decreases.".to_owned(),
                    confidence: "low".to_owned(),
                    committed_backlog_id: None,
                });
            }
        }

        if commit {
            for proposal in &mut proposals {
                connection.execute(
                    "INSERT INTO backlog (
                        title, discovered_while, current_pain, suggested_improvement,
                        risk, predicted_impact, notes
                     ) VALUES (?1, 'harness-cli propose', ?2, ?3, ?4, ?5, ?6);",
                    params![
                        proposal.title,
                        proposal.evidence,
                        proposal.suggested_action,
                        normalize_token(&proposal.risk),
                        proposal.predicted_impact,
                        format!(
                            "component: {}; confidence: {}; validation: {}",
                            proposal.component, proposal.confidence, proposal.validation_plan
                        ),
                    ],
                )?;
                proposal.committed_backlog_id = Some(connection.last_insert_rowid());
            }
        }

        Ok(proposals)
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
}

impl From<HarnessContext> for SqliteHarnessRepository {
    fn from(context: HarnessContext) -> Self {
        Self::new(context.repo_root, context.db_path, context.schema_dir)
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;
    use crate::application::{
        BacklogAddInput, BacklogCloseInput, DecisionAddInput, IntakeInput, InterventionAddInput,
        InterventionFilter, StoryAddInput, StoryUpdateInput, ToolRegisterInput, TraceInput,
    };
    use crate::domain::{
        BacklogFilter, BoolFlag, CsvList, InputType, Priority, RiskLane, ToolArgSpec, TraceQualityTier,
    };

    fn test_repository() -> (TempDir, SqliteHarnessRepository) {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .unwrap()
            .to_path_buf();
        let repository = SqliteHarnessRepository::new(
            repo_root.clone(),
            temp_dir.path().join("harness.db"),
            repo_root.join("scripts/schema"),
        );
        (temp_dir, repository)
    }

    fn story_columns(connection: &Connection) -> Vec<String> {
        let mut statement = connection.prepare("PRAGMA table_info(story);").unwrap();
        let rows = statement
            .query_map([], |row| row.get::<_, String>(1))
            .unwrap();
        rows.collect::<std::result::Result<Vec<_>, _>>().unwrap()
    }

    #[test]
    fn init_creates_database_and_schema() {
        let (_temp_dir, repository) = test_repository();

        let result = repository.init().unwrap();

        assert!(matches!(result, InitResult::Created { .. }));
        assert_eq!(repository.query_stats().unwrap().intakes, 0);
        let connection = repository.open_existing().unwrap();
        let schema_version = schema_version(&connection).unwrap();
        assert_eq!(schema_version, 6);
        let story_columns = story_columns(&connection);
        assert!(story_columns.contains(&"verify_command".to_owned()));
        assert!(story_columns.contains(&"last_verified_at".to_owned()));
        assert!(story_columns.contains(&"last_verified_result".to_owned()));
        assert!(story_columns.contains(&"priority".to_owned()));
    }


    #[test]
    fn records_intake_and_queries_it_back() {
        let (_temp_dir, repository) = test_repository();
        repository.init().unwrap();

        let id = repository
            .record_intake(IntakeInput {
                input_type: InputType::NewSpec,
                summary: "Add SQLite durable database layer".to_owned(),
                risk_lane: RiskLane::Normal,
                risk_flags: CsvList::from_optional(Some("database".to_owned())),
                affected_docs: CsvList::from_optional(Some("docs/HARNESS.md".to_owned())),
                story_id: Some("US-001".to_owned()),
                notes: Some("Initial spike complete".to_owned()),
            })
            .unwrap();

        assert_eq!(id, 1);
        let records = repository.query_intakes().unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].summary, "Add SQLite durable database layer");
        assert_eq!(records[0].input_type, "new_spec");
        assert_eq!(records[0].risk_lane, "normal");

        let stats = repository.query_stats().unwrap();
        assert_eq!(stats.intakes, 1);
    }

    #[test]
    fn manages_story_records() {
        let (_temp_dir, repository) = test_repository();
        repository.init().unwrap();

        repository
            .add_story(StoryAddInput {
                id: "US-001".to_owned(),
                title: "SQLite durable layer".to_owned(),
                risk_lane: RiskLane::HighRisk,
                contract_doc: Some("docs/stories/US-001.md".to_owned()),
                verify_command: Some("cargo test --package harness-cli".to_owned()),
                notes: None,
                priority: Priority::P0,
            })
            .unwrap();

        repository
            .update_story(StoryUpdateInput {
                id: "US-001".to_owned(),
                status: Some("implemented".to_owned()),
                evidence: Some("tests pass".to_owned()),
                unit: Some(BoolFlag(1)),
                integration: Some(BoolFlag(1)),
                e2e: Some(BoolFlag(0)),
                platform: Some(BoolFlag(0)),
                verify_command: None,
                priority: Some(Priority::P1),
            })
            .unwrap();

        let matrix = repository.query_matrix().unwrap();
        assert_eq!(matrix.len(), 1);
        assert_eq!(matrix[0].id, "US-001");
        assert_eq!(matrix[0].status, "implemented");
        assert_eq!(matrix[0].unit, 1);
        assert_eq!(matrix[0].priority, "P1");

        let status = repository.story_verify_status("US-001").unwrap();
        assert_eq!(
            status.verify_command.as_deref(),
            Some("cargo test --package harness-cli")
        );
        assert_eq!(status.last_verified_result, None);
    }

    #[test]
    fn manages_decisions_and_backlog() {
        let (_temp_dir, repository) = test_repository();
        repository.init().unwrap();

        repository
            .add_decision(DecisionAddInput {
                id: "0004-sqlite-layer".to_owned(),
                title: "Use SQLite for durable state".to_owned(),
                status: "accepted".to_owned(),
                doc_path: Some("docs/decisions/0004-sqlite-layer.md".to_owned()),
                verify_command: Some("cargo check".to_owned()),
                predicted_impact: Some("High".to_owned()),
                notes: None,
            })
            .unwrap();

        let backlog_id = repository
            .add_backlog(BacklogAddInput {
                title: "Add prompt leverage skill".to_owned(),
                discovered_while: Some("milestone review".to_owned()),
                current_pain: Some("manual prompt tuning".to_owned()),
                suggestion: Some("import skill".to_owned()),
                risk: Some(RiskLane::Normal),
                predicted_impact: Some("Faster setup".to_owned()),
                notes: None,
                priority: Priority::P2,
            })
            .unwrap();

        repository
            .close_backlog(BacklogCloseInput {
                id: backlog_id,
                status: "implemented".to_owned(),
                actual_outcome: Some("Prompt leverage skill imported".to_owned()),
            })
            .unwrap();

        assert_eq!(repository.query_decisions().unwrap().len(), 1);

        let open_backlog = repository.query_backlog(BacklogFilter::Open).unwrap();
        let closed_backlog = repository.query_backlog(BacklogFilter::Closed).unwrap();
        assert_eq!(open_backlog.len(), 0);
        assert_eq!(closed_backlog.len(), 1);
        assert_eq!(closed_backlog[0].priority, "P2");
    }

    #[test]
    fn registers_and_queries_tools() {
        let (_temp_dir, repository) = test_repository();
        repository.init().unwrap();

        repository
            .register_tool(ToolRegisterInput {
                name: "test-tool".to_owned(),
                command: "cargo check".to_owned(),
                description: "Run cargo check for local package compilation verification.".to_owned(),
                responsibility: "Verification".to_owned(),
                args: vec![ToolArgSpec {
                    name: "package".to_owned(),
                    arg_type: "string".to_owned(),
                    required: true,
                    help: Some("Package name to compile check".to_owned()),
                }],
                force: true,
            })
            .unwrap();

        let tools = repository.query_tools(Some("verification".to_owned())).unwrap();
        assert!(tools.iter().any(|tool| tool.name == "test-tool"));
        let tool = tools.iter().find(|tool| tool.name == "test-tool").unwrap();
        assert_eq!(tool.source, "registered");
        assert_eq!(tool.args.len(), 1);
        assert_eq!(tool.args[0].name, "package");
    }

    #[test]
    fn records_and_scores_traces() {
        let (_temp_dir, repository) = test_repository();
        repository.init().unwrap();

        let intake_id = repository
            .record_intake(IntakeInput {
                input_type: InputType::NewSpec,
                summary: "Add trace scoring command".to_owned(),
                risk_lane: RiskLane::Normal,
                risk_flags: CsvList::from_optional(None),
                affected_docs: CsvList::from_optional(None),
                story_id: None,
                notes: None,
            })
            .unwrap();

        let trace_id = repository
            .record_trace(TraceInput {
                task_summary: "Implemented trace quality scoring".to_owned(),
                intake_id: Some(intake_id),
                story_id: None,
                agent: Some("codex".to_owned()),
                outcome: Some("completed".to_owned()),
                duration_seconds: Some(120),
                token_estimate: Some(1500),
                friction: Some("minor schema alignment retry".to_owned()),
                notes: None,
                actions: CsvList::from_optional(Some("read,patched,tested".to_owned())),
                files_read: CsvList::from_optional(Some("crates/harness-cli/src/domain.rs".to_owned())),
                files_changed: CsvList::from_optional(Some("crates/harness-cli/src/domain.rs".to_owned())),
                decisions: CsvList::from_optional(Some("kept scoring inside domain module".to_owned())),
                errors: CsvList::from_optional(Some("none".to_owned())),
            })
            .unwrap();

        repository
            .add_intervention(InterventionAddInput {
                trace_id: Some(trace_id),
                story_id: None,
                intervention_type: "correction".to_owned(),
                description: "Fixed missing error handling during review".to_owned(),
                source: "reviewer".to_owned(),
                impact: Some("prevented runtime panic".to_owned()),
            })
            .unwrap();

        let trace_score = repository.score_trace(Some(trace_id)).unwrap();
        assert_eq!(trace_score.achieved, TraceQualityTier::Detailed);
        assert_eq!(trace_score.required, Some(TraceQualityTier::Standard));
        assert!(trace_score.meets_requirement);

        let context_score = repository.score_context(trace_id).unwrap();
        assert_eq!(context_score.trace_id, trace_id);
        assert_eq!(context_score.lane, "normal");

        let friction = repository.query_friction().unwrap();
        assert_eq!(friction.len(), 1);
        assert_eq!(
            friction[0].harness_friction,
            "minor schema alignment retry"
        );

        let interventions = repository
            .query_interventions(InterventionFilter {
                trace_id: Some(trace_id),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(interventions.len(), 1);
        assert_eq!(interventions[0].source, "reviewer");
    }

    #[test]
    fn audit_and_propose_generate_actionable_improvements() {
        let (_temp_dir, repository) = test_repository();
        repository.init().unwrap();

        let trace_input = TraceInput {
            task_summary: "Task with friction".to_owned(),
            intake_id: None,
            story_id: None,
            agent: Some("codex".to_owned()),
            outcome: Some("completed".to_owned()),
            duration_seconds: None,
            token_estimate: None,
            friction: Some("ambiguous CLI help output".to_owned()),
            notes: None,
            actions: CsvList::from_optional(None),
            files_read: CsvList::from_optional(None),
            files_changed: CsvList::from_optional(None),
            decisions: CsvList::from_optional(None),
            errors: CsvList::from_optional(None),
        };
        repository.record_trace(trace_input.clone()).unwrap();
        repository.record_trace(trace_input).unwrap();

        let proposals = repository.propose(true).unwrap();
        assert!(!proposals.is_empty());
        assert!(proposals
            .iter()
            .any(|proposal| proposal.title.contains("ambiguous CLI help output")));
        assert!(proposals
            .iter()
            .all(|proposal| proposal.committed_backlog_id.is_some()));

        let audit = repository.audit().unwrap();
        assert_eq!(audit.entropy_score(), 0);
    }

    #[test]
    fn manages_work_item_records() {
        let (_temp_dir, repository) = test_repository();
        repository.init().unwrap();

        let id = repository
            .add_work_item(WorkItemAddInput {
                work_type: WorkItemType::UserStory,
                title: "CRM - Khach hang co the tao don hang".to_owned(),
                description: Some("As a customer, I want to create orders, So that I get items.".to_owned()),
                assigned_to: Some("alice".to_owned()),
                story_points: Some(5),
                remaining_work: None,
                priority: Priority::P1,
                severity: None,
                parent_id: None,
                area_path: Some("CRM/Sales".to_owned()),
                iteration_path: Some("Sprint 1".to_owned()),
                tags: vec!["FE".to_owned(), "API".to_owned()],
                acceptance_criteria: Some("AC 1: Pass payment".to_owned()),
                repro_steps: None,
                actual_result: None,
                expected_result: None,
                steps: None,
            })
            .unwrap();

        assert_eq!(id, 1);

        let item = repository.get_work_item(1).unwrap().expect("item found");
        assert_eq!(item.work_type, WorkItemType::UserStory);
        assert_eq!(item.state, WorkItemState::New);
        assert_eq!(item.priority, Priority::P1);
        assert_eq!(item.tags, vec!["FE", "API"]);

        repository
            .update_work_item(WorkItemUpdateInput {
                id: 1,
                title: None,
                description: None,
                state: Some(WorkItemState::Accepted),
                assigned_to: None,
                story_points: None,
                remaining_work: None,
                priority: None,
                severity: None,
                parent_id: None,
                area_path: None,
                iteration_path: None,
                tags: None,
                acceptance_criteria: None,
                repro_steps: None,
                actual_result: None,
                expected_result: None,
                steps: None,
            })
            .unwrap();

        let updated = repository.get_work_item(1).unwrap().unwrap();
        assert_eq!(updated.state, WorkItemState::Accepted);

        let err = repository.update_work_item(WorkItemUpdateInput {
            id: 1,
            title: None,
            description: None,
            state: Some(WorkItemState::Closed),
            assigned_to: None,
            story_points: None,
            remaining_work: None,
            priority: None,
            severity: None,
            parent_id: None,
            area_path: None,
            iteration_path: None,
            tags: None,
            acceptance_criteria: None,
            repro_steps: None,
            actual_result: None,
            expected_result: None,
            steps: None,
        });
        assert!(err.is_err());

        let list = repository.query_work_items(Some(WorkItemType::UserStory), None).unwrap();
        assert_eq!(list.len(), 1);
        let empty_list = repository.query_work_items(Some(WorkItemType::Bug), None).unwrap();
        assert_eq!(empty_list.len(), 0);
    }
}

