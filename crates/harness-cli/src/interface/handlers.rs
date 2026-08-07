use std::env;
use std::path::PathBuf;
use std::str::FromStr;

use super::args::*;
use super::errors::InterfaceError;
use super::formatters::*;
use super::stubs::*;
use crate::application::{
    BacklogAddInput, BacklogCloseInput, DecisionAddInput, HarnessContext, HarnessService,
    IntakeInput, InterventionAddInput, InterventionFilter, StoryAddInput, StoryUpdateInput,
    ToolRegisterInput, TraceInput, WorkItemAddInput, WorkItemUpdateInput,
};
use crate::domain::{
    parse_optional_integer, parse_tags_string, parse_tool_args, validate_responsibility,
    BacklogFilter, BoolFlag, CsvList, InputType, Priority, RiskLane, Severity, WorkItemState,
    WorkItemType,
};

pub fn run(cli: Cli) -> Result<(), InterfaceError> {
    let service = HarnessService::new(resolve_context()?);

    match cli.command {
        Command::Init => print_init_result(service.init()?),
        Command::Migrate => print_migrate_result(service.migrate()?),
        Command::Import(args) => match args.source {
            ImportSource::Brownfield => {
                print_brownfield_import_result(service.import_brownfield()?)
            }
        },
        Command::Intake(args) => {
            let id = service.record_intake(IntakeInput {
                input_type: InputType::from_str(&args.input_type)?,
                summary: args.summary,
                risk_lane: RiskLane::from_str(&args.lane)?,
                risk_flags: CsvList::from_optional(args.flags),
                affected_docs: CsvList::from_optional(args.docs),
                story_id: args.story,
                notes: args.notes,
            })?;
            println!("Intake #{id} recorded.");
        }
        Command::WorkItem(args) => match args.action {
            WorkItemAction::Add(args) => {
                let work_type = WorkItemType::from_str(&args.work_type)?;
                let priority = Priority::from_str(&args.priority)?;
                let severity = args
                    .severity
                    .map(|value| Severity::from_str(&value))
                    .transpose()?;
                let tags = parse_tags_string(args.tags.as_deref());

                let id = service.add_work_item(WorkItemAddInput {
                    work_type,
                    title: args.title,
                    description: args.description,
                    assigned_to: args.assigned_to,
                    story_points: args.story_points,
                    remaining_work: args.remaining_work,
                    priority,
                    severity,
                    parent_id: args.parent_id,
                    area_path: args.area_path,
                    iteration_path: args.iteration_path,
                    tags,
                    acceptance_criteria: args.acceptance_criteria,
                    repro_steps: args.repro_steps,
                    actual_result: args.actual_result,
                    expected_result: args.expected_result,
                    steps: args.steps,
                })?;
                println!("WorkItem #{id} ({}) created.", work_type.as_str());
            }
            WorkItemAction::Update(args) => {
                let state = args
                    .state
                    .map(|value| WorkItemState::from_str(&value))
                    .transpose()?;
                let priority = args
                    .priority
                    .map(|value| Priority::from_str(&value))
                    .transpose()?;
                let severity = args
                    .severity
                    .map(|value| Severity::from_str(&value))
                    .transpose()?;
                let tags = args.tags.map(|value| parse_tags_string(Some(&value)));

                service.update_work_item(WorkItemUpdateInput {
                    id: args.id,
                    title: args.title,
                    description: args.description,
                    state,
                    assigned_to: args.assigned_to,
                    story_points: args.story_points,
                    remaining_work: args.remaining_work,
                    priority,
                    severity,
                    parent_id: args.parent_id,
                    area_path: args.area_path,
                    iteration_path: args.iteration_path,
                    tags,
                    acceptance_criteria: args.acceptance_criteria,
                    repro_steps: args.repro_steps,
                    actual_result: args.actual_result,
                    expected_result: args.expected_result,
                    steps: args.steps,
                })?;
                println!("WorkItem #{} updated.", args.id);
            }
            WorkItemAction::List(args) => {
                let work_type = args
                    .work_type
                    .map(|value| WorkItemType::from_str(&value))
                    .transpose()?;
                let state = args
                    .state
                    .map(|value| WorkItemState::from_str(&value))
                    .transpose()?;
                let items = service.query_work_items(work_type, state)?;
                print_work_items(&items);
            }
            WorkItemAction::Show { id } => {
                let item = service
                    .get_work_item(id)?
                    .ok_or(crate::infrastructure::HarnessInfraError::WorkItemNotFound(id))?;
                print_work_item_detail(&item);
            }
        },
        Command::Story(args) => match args.action {

            StoryAction::Add(args) => {
                service.add_story(StoryAddInput {
                    id: args.id.clone(),
                    title: args.title,
                    risk_lane: RiskLane::from_str(&args.lane)?,
                    contract_doc: args.contract,
                    verify_command: args.verify,
                    notes: args.notes,
                    priority: Priority::from_str(&args.priority)?,
                })?;
                println!("Story {} added.", args.id);
            }
            StoryAction::Update(args) => {
                service.update_story(StoryUpdateInput {
                    id: args.id.clone(),
                    status: args.status,
                    evidence: args.evidence,
                    unit: parse_optional_bool("story update: --unit", args.unit)?,
                    integration: parse_optional_bool(
                        "story update: --integration",
                        args.integration,
                    )?,
                    e2e: parse_optional_bool("story update: --e2e", args.e2e)?,
                    platform: parse_optional_bool("story update: --platform", args.platform)?,
                    verify_command: args.verify,
                    priority: args.priority.map(|p| Priority::from_str(&p)).transpose()?,
                })?;
                println!("Story {} updated.", args.id);
            }
            StoryAction::Verify { id } => {
                let result = service.verify_story(&id)?;
                println!("Running: {}", result.command);
                print!("{}", result.stdout);
                print!("{}", result.stderr);
                println!("Story {id} verification: {}", result.result);
                if result.result == "fail" {
                    std::process::exit(1);
                }
            }
            StoryAction::VerifyAll => {
                let result = service.verify_all_stories()?;
                print_story_verify_all(&result);
                if result.failed() > 0 {
                    std::process::exit(1);
                }
            }
        },
        Command::Decision(args) => match args.action {
            DecisionAction::Add(args) => {
                service.add_decision(DecisionAddInput {
                    id: args.id.clone(),
                    title: args.title,
                    status: args.status,
                    doc_path: args.doc,
                    verify_command: args.verify,
                    predicted_impact: args.predicted,
                    notes: args.notes,
                })?;
                println!("Decision {} added.", args.id);
            }
            DecisionAction::Verify { id } => {
                let result = service.verify_decision(&id)?;
                println!("Running: {}", result.command);
                println!("Decision {id} verification: {}", result.result);
                if result.result == "fail" {
                    std::process::exit(1);
                }
            }
        },
        Command::Backlog(args) => match args.action {
            BacklogAction::Add(args) => {
                let id = service.add_backlog(BacklogAddInput {
                    title: args.title,
                    discovered_while: args.discovered_while,
                    current_pain: args.pain,
                    suggestion: args.suggestion,
                    risk: args
                        .risk
                        .map(|value| RiskLane::from_str(&value))
                        .transpose()?,
                    predicted_impact: args.predicted,
                    notes: args.notes,
                    priority: Priority::from_str(&args.priority)?,
                })?;
                println!("Backlog #{id} added.");
            }
            BacklogAction::Close(args) => {
                let id = parse_optional_integer("backlog close: --id", Some(args.id))?
                    .expect("value provided");
                let status = args.status;
                service.close_backlog(BacklogCloseInput {
                    id,
                    status: status.clone(),
                    actual_outcome: args.outcome,
                })?;
                println!("Backlog #{id} closed as {status}.");
            }
        },
        Command::Tool(args) => match args.action {
            ToolAction::Register(args) => {
                service.register_tool(ToolRegisterInput {
                    name: args.name.clone(),
                    command: args.command,
                    description: args.description,
                    responsibility: validate_responsibility(&args.responsibility)?,
                    args: parse_tool_args(args.args)?,
                    force: args.force,
                })?;
                println!("Tool {} registered.", args.name);
            }
            ToolAction::Remove { name } => {
                service.remove_tool(&name)?;
                println!("Tool {name} removed.");
            }
        },
        Command::Intervention(args) => match args.action {
            InterventionAction::Add(args) => {
                let id = service.add_intervention(InterventionAddInput {
                    trace_id: parse_optional_integer("intervention add: --trace", args.trace)?,
                    story_id: args.story,
                    intervention_type: args.intervention_type,
                    description: args.description,
                    source: args.source,
                    impact: args.impact,
                })?;
                println!("Intervention #{id} recorded.");
            }
        },
        Command::Trace(args) => {
            let story_id = args.story.clone();
            let id = service.record_trace(TraceInput {
                task_summary: args.summary,
                intake_id: parse_optional_integer("trace: --intake", args.intake)?,
                story_id: args.story,
                agent: args.agent,
                outcome: args.outcome,
                duration_seconds: parse_optional_integer("trace: --duration", args.duration)?,
                token_estimate: parse_optional_integer("trace: --tokens", args.tokens)?,
                friction: args.friction,
                notes: args.notes,
                actions: CsvList::from_optional(args.actions),
                files_read: CsvList::from_optional(args.files_read),
                files_changed: CsvList::from_optional(args.files_changed),
                decisions: CsvList::from_optional(args.decisions),
                errors: CsvList::from_optional(args.errors),
            })?;
            println!("Trace #{id} recorded.");
            let result = service.score_trace(Some(id))?;
            print_trace_score(&result, false);
            println!("Reminder: Record any human corrections with: harness-cli intervention add");
            if let Some(story_id) = story_id {
                print_story_verify_warning(&service, &story_id)?;
            }
        }
        Command::ScoreTrace(args) => {
            let id = parse_optional_integer("score-trace: --id", args.id)?;
            let result = service.score_trace(id)?;
            print_trace_score(&result, id.is_none());
            if !result.meets_requirement {
                std::process::exit(1);
            }
        }
        Command::ScoreContext { trace_id } => {
            let id = parse_optional_integer("score-context: trace-id", Some(trace_id))?
                .expect("value provided");
            print_context_score(&service.score_context(id)?);
        }
        Command::Audit => print_audit(&service.audit()?),
        Command::Propose(args) => print_proposals(&service.propose(args.commit)?),
        Command::Query(args) => match args.view {
            QueryView::Matrix(args) => print_matrix(&service.query_matrix()?, args.numeric),
            QueryView::Backlog(args) => {
                print_backlog(&service.query_backlog(backlog_filter(&args))?)
            }
            QueryView::Decisions => print_decisions(&service.query_decisions()?),
            QueryView::Intakes => print_intakes(&service.query_intakes()?),
            QueryView::Traces => print_traces(&service.query_traces()?),
            QueryView::Friction => print_friction(&service.query_friction()?),
            QueryView::Tools(args) => {
                let responsibility = args
                    .responsibility
                    .map(|value| validate_responsibility(&value))
                    .transpose()?;
                let tools = service.query_tools(responsibility)?;
                if args.json {
                    print_tools_json(&tools);
                } else {
                    print_tools_summary(&tools);
                }
            }
            QueryView::Interventions(args) => {
                let trace_id = parse_optional_integer("query interventions: --trace", args.trace)?;
                print_interventions(&service.query_interventions(InterventionFilter {
                    trace_id,
                    story_id: args.story,
                    intervention_type: args.intervention_type,
                })?);
            }
            QueryView::Stats => print_stats(&service.query_stats()?),
            QueryView::Sql { query } => {
                if query.is_empty() {
                    return Err(InterfaceError::EmptySql);
                }
                print_query_table(&service.query_sql(&query.join(" "))?);
            }
        },
        Command::ExportTrace(args) => {
            let traces = service.query_traces()?;
            let content = if args.format == "mermaid" {
                export_trace_mermaid(&traces)
            } else {
                export_trace_tldraw(&traces)
            };
            if let Some(out_path) = args.out {
                std::fs::write(&out_path, &content)
                    .map_err(crate::infrastructure::HarnessInfraError::Io)?;
                println!("Exported trace diagram ({}) to {out_path}", args.format);
            } else {
                println!("{content}");
            }
        }
        Command::Worktree(args) => handle_worktree_subcommand(args),
        Command::Subagent(args) => handle_subagent_subcommand(args),
        Command::Skill(args) => handle_skill_subcommand(args),
        Command::Config(args) => handle_config_subcommand(args),
    }

    Ok(())
}

fn resolve_context() -> Result<HarnessContext, InterfaceError> {
    let repo_root = match env::var_os("HARNESS_REPO_ROOT") {
        Some(path) => PathBuf::from(path),
        None => env::current_dir().map_err(InterfaceError::CurrentDir)?,
    };
    let db_path = env::var_os("HARNESS_DB")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root.join("harness.db"));

    let schema_dir = repo_root.join("scripts/schema");

    Ok(HarnessContext {
        repo_root,
        db_path,
        schema_dir,
    })
}

fn backlog_filter(args: &BacklogQueryArgs) -> BacklogFilter {
    if args.open {
        BacklogFilter::Open
    } else if args.closed {
        BacklogFilter::Closed
    } else {
        BacklogFilter::All
    }
}

fn parse_optional_bool(
    label: &str,
    value: Option<String>,
) -> Result<Option<BoolFlag>, InterfaceError> {
    value
        .map(|inner| BoolFlag::parse(label, &inner))
        .transpose()
        .map_err(InterfaceError::from)
}

fn print_story_verify_warning(
    service: &HarnessService,
    story_id: &str,
) -> Result<(), InterfaceError> {
    let status = service.story_verify_status(story_id)?;
    let has_command = status
        .verify_command
        .as_deref()
        .map(str::trim)
        .is_some_and(|value| !value.is_empty());
    if has_command && status.last_verified_result.as_deref() != Some("pass") {
        println!();
        println!(
            "Warning: Story {} has verify_command but verification has not passed.",
            status.id
        );
        println!("Run: harness-cli story verify {}", status.id);
    }
    Ok(())
}
