use crate::application::{BrownfieldImportResult, InitResult, MigrateResult, QueryTable};
use crate::domain::{
    escape_json_string, proof_display, AuditFinding, AuditResult, BacklogRecord,
    ContextScoreResult, DecisionRecord, FrictionRecord, HarnessStats, ImprovementProposal,
    IntakeRecord, InterventionRecord, StoryMatrixRecord, StoryVerifyAllResult, ToolEntry,
    TraceQualityTier, TraceRecord, TraceScoreResult, WorkItem,
};

pub fn export_trace_tldraw(traces: &[TraceRecord]) -> String {
    let mut records = Vec::new();

    records.push(
        r#"{"id":"document:document","typeName":"document","title":"Harness Trace Diagram"}"#
            .to_owned(),
    );
    records.push(r#"{"id":"page:page","typeName":"page","name":"Page 1","index":"a1"}"#.to_owned());

    records.push(format!(
        r#"{{"id":"shape:title","typeName":"shape","type":"geo","parentId":"page:page","index":"a1","x":100,"y":40,"props":{{"geo":"rectangle","w":650,"h":60,"color":"blue","labelColor":"black","fill":"semi","dash":"draw","size":"m","font":"draw","text":"🚀 HARNESS EXECUTION TRACE GRAPH","align":"middle","verticalAlign":"middle","growY":0}}}}"#
    ));

    for (i, trace) in traces.iter().enumerate() {
        let y_pos = 120 + (i * 110);
        let summary = trace.task_summary.replace('"', "\\\"").replace('\n', "\\n");
        let outcome = trace.outcome.as_deref().unwrap_or("unknown");
        let color = match outcome {
            "completed" | "pass" => "green",
            "failed" | "fail" => "red",
            _ => "yellow",
        };

        let label = format!("Trace #{}: {}\nOutcome: {}", trace.id, summary, outcome);
        let escaped_label = label.replace('"', "\\\"").replace('\n', "\\n");

        records.push(format!(
            r#"{{"id":"shape:trace_{}","typeName":"shape","type":"geo","parentId":"page:page","index":"a{}","x":100,"y":{},"props":{{"geo":"rectangle","w":650,"h":90,"color":"{}","labelColor":"black","fill":"semi","dash":"draw","size":"s","font":"draw","text":"{}","align":"start","verticalAlign":"middle","growY":0}}}}"#,
            trace.id, i + 2, y_pos, color, escaped_label
        ));
    }

    format!(
        r#"{{"tldrawFileFormatVersion":1,"schema":{{"schemaVersion":2,"sequences":{{"com.tldraw.store":4,"com.tldraw.asset":1,"com.tldraw.camera":1,"com.tldraw.document":2,"com.tldraw.instance":25,"com.tldraw.instance_page_state":5,"com.tldraw.page":1,"com.tldraw.pointer":1,"com.tldraw.shape":4,"com.tldraw.shape.geo":9}}}},"records":[{}]}}"#,
        records.join(",")
    )
}

pub fn export_trace_mermaid(traces: &[TraceRecord]) -> String {
    let mut lines = Vec::new();
    lines.push("```mermaid".to_owned());
    lines.push("graph TD;".to_owned());
    lines.push("    Title[\"🚀 HARNESS EXECUTION TRACE\"]".to_owned());

    for (i, trace) in traces.iter().enumerate() {
        let summary = trace.task_summary.replace('"', "'");
        let node_id = format!("T{}", trace.id);
        lines.push(format!(
            "    {node_id}[\"Trace #{}: {}\"]",
            trace.id, summary
        ));
        if i == 0 {
            lines.push(format!("    Title --> {node_id}"));
        } else {
            let prev_id = format!("T{}", traces[i - 1].id);
            lines.push(format!("    {prev_id} --> {node_id}"));
        }
    }

    lines.push("```".to_owned());
    lines.join("\n")
}

pub fn print_trace_score(result: &TraceScoreResult, latest: bool) {
    if latest {
        println!("Trace #{} (latest):", result.trace_id);
    } else {
        println!("Trace #{}:", result.trace_id);
    }
    println!(
        "  Tier achieved: {} ({}/3)",
        result.achieved.label(),
        result.achieved.score()
    );

    match (&result.risk_lane, result.required) {
        (Some(lane), Some(required)) => {
            println!(
                "  Lane: {} -> required tier: {} ({}/3)",
                lane,
                required.label(),
                required.score()
            );
            if result.meets_requirement {
                println!("  MEETS REQUIREMENT");
            } else {
                println!("  BELOW REQUIREMENT");
            }
        }
        _ => {
            println!("  Lane: unknown (no linked intake)");
        }
    }

    print_missing_fields(
        "minimal",
        TraceQualityTier::Minimal,
        &result.missing_minimal,
    );
    print_missing_fields(
        "standard",
        TraceQualityTier::Standard,
        &result.missing_standard,
    );
    print_missing_fields(
        "detailed",
        TraceQualityTier::Detailed,
        &result.missing_detailed,
    );
}

pub fn print_story_verify_all(result: &StoryVerifyAllResult) {
    for item in &result.items {
        match item.result.as_str() {
            "skipped" => println!("Story {}: skipped (no verify_command)", item.id),
            status => {
                println!("Story {}: {status}", item.id);
                if !item.stdout.is_empty() {
                    print!("{}", item.stdout);
                }
                if !item.stderr.is_empty() {
                    print!("{}", item.stderr);
                }
            }
        }
    }
    println!(
        "{} stories verified: {} passed, {} failed, {} skipped (no verify_command)",
        result.items.len(),
        result.passed(),
        result.failed(),
        result.skipped()
    );
}

pub fn print_context_score(result: &ContextScoreResult) {
    println!(
        "Trace #{} | Lane: {} | Phase: {}",
        result.trace_id, result.lane, result.phase
    );
    println!();
    let must_met = result.must.iter().filter(|item| item.met).count();
    println!("Must-read compliance: {must_met}/{}", result.must.len());
    for item in &result.must {
        println!(
            "  {} {} ({})",
            if item.met { "OK" } else { "MISSING" },
            item.label,
            item.target
        );
    }
    let should_met = result.should.iter().filter(|item| item.met).count();
    println!(
        "Should-read compliance: {should_met}/{}",
        result.should.len()
    );
    for item in &result.should {
        println!(
            "  {} {} ({})",
            if item.met { "OK" } else { "MISSING" },
            item.label,
            item.target
        );
    }
    println!("Over-reading: {} item(s)", result.over_read.len());
    for item in &result.over_read {
        println!("  - {item}");
    }
}

pub fn print_audit(result: &AuditResult) {
    println!("=== Harness Drift Audit ===");
    print_audit_category(
        "Orphaned stories (planned/in-progress, no traces)",
        &result.orphaned_stories,
    );
    print_audit_category("Unverified stories", &result.unverified_stories);
    print_audit_category("Unverified decisions", &result.unverified_decisions);
    print_audit_category(
        "Open backlog without outcomes",
        &result.backlog_without_outcomes,
    );
    print_audit_category("Stale stories", &result.stale_stories);
    print_audit_category("Broken tools", &result.broken_tools);
    println!(
        "Entropy score: {}/100 (lower is better)",
        result.entropy_score()
    );
}

pub fn print_audit_category(label: &str, findings: &[AuditFinding]) {
    println!();
    println!("{label}: {}", findings.len());
    for finding in findings {
        println!("  - {}: {}", finding.id, finding.title);
    }
}

pub fn print_proposals(proposals: &[ImprovementProposal]) {
    println!("=== Improvement Proposals ===");
    if proposals.is_empty() {
        println!("No proposals generated.");
        return;
    }
    for (index, proposal) in proposals.iter().enumerate() {
        println!();
        println!(
            "Proposal {} ({} confidence):",
            index + 1,
            proposal.confidence
        );
        println!("  Title: {}", proposal.title);
        println!("  Component: {}", proposal.component);
        println!("  Evidence: {}", proposal.evidence);
        println!("  Predicted impact: {}", proposal.predicted_impact);
        println!("  Risk: {}", proposal.risk);
        println!("  Suggested action: {}", proposal.suggested_action);
        println!("  Validation: {}", proposal.validation_plan);
        if let Some(id) = proposal.committed_backlog_id {
            println!("  Created backlog item #{id}");
        }
    }
    println!();
    println!(
        "{} proposals generated. Use --commit to create backlog items.",
        proposals.len()
    );
}

pub fn print_missing_fields(label: &str, tier: TraceQualityTier, fields: &[String]) {
    if fields.is_empty() {
        return;
    }
    println!();
    println!("  Missing for {label}:");
    for field in fields {
        println!("    - {field}");
    }
    if tier == TraceQualityTier::Detailed {
        println!();
    }
}

pub fn print_brownfield_import_result(result: BrownfieldImportResult) {
    println!("Brownfield import complete.");
    println!("Stories imported or updated: {}", result.stories);
    println!("Decisions imported or updated: {}", result.decisions);
    println!("Backlog items discovered: {}", result.backlog_items);
}

pub fn print_init_result(result: InitResult) {
    match result {
        InitResult::Created { db_path } => {
            println!("Creating harness database at {}", db_path.display());
            println!("Schema applied.");
        }
        InitResult::Existing { db_path, version } => {
            println!("Database already exists at {}", db_path.display());
            println!("Current schema version: {version}");
        }
        InitResult::MigratedExisting { db_path } => {
            println!("Database already exists at {}", db_path.display());
            println!("No schema version found. Applying schema.");
            println!("Schema applied.");
        }
    }
}

pub fn print_migrate_result(result: MigrateResult) {
    println!("Current schema version: {}", result.current_version);
    if result.applied.is_empty() {
        println!("Already up to date.");
    } else {
        for version in &result.applied {
            println!("Applying migration {version}...");
        }
        println!("Applied {} migration(s).", result.applied.len());
    }
}

pub fn print_matrix(records: &[StoryMatrixRecord], numeric: bool) {
    let rows = records
        .iter()
        .map(|record| {
            vec![
                record.id.clone(),
                record.title.clone(),
                record.status.clone(),
                record.priority.clone(),
                proof_display(record.unit, numeric),
                proof_display(record.integration, numeric),
                proof_display(record.e2e, numeric),
                proof_display(record.platform, numeric),
                record.evidence.clone().unwrap_or_default(),
            ]
        })
        .collect::<Vec<_>>();
    print_table(
        &[
            "id", "title", "status", "priority", "unit", "integ", "e2e", "plat", "evidence",
        ],
        &rows,
    );
}

pub fn print_backlog(records: &[BacklogRecord]) {
    let rows = records
        .iter()
        .map(|record| {
            vec![
                record.id.to_string(),
                record.title.clone(),
                record.status.clone(),
                record.priority.clone(),
                record.risk.clone().unwrap_or_default(),
                record.predicted_impact.clone().unwrap_or_default(),
                record.actual_outcome.clone().unwrap_or_default(),
            ]
        })
        .collect::<Vec<_>>();
    print_table(
        &[
            "id",
            "title",
            "status",
            "priority",
            "risk",
            "predicted_impact",
            "actual_outcome",
        ],
        &rows,
    );
}

pub fn print_decisions(records: &[DecisionRecord]) {
    let rows = records
        .iter()
        .map(|record| {
            vec![
                record.id.clone(),
                record.title.clone(),
                record.status.clone(),
                record.last_verified_at.clone().unwrap_or_default(),
                record.last_verified_result.clone().unwrap_or_default(),
            ]
        })
        .collect::<Vec<_>>();
    print_table(
        &[
            "id",
            "title",
            "status",
            "last_verified_at",
            "last_verified_result",
        ],
        &rows,
    );
}

pub fn print_intakes(records: &[IntakeRecord]) {
    let rows = records
        .iter()
        .map(|record| {
            vec![
                record.id.to_string(),
                record.created_at.clone(),
                record.input_type.clone(),
                record.risk_lane.clone(),
                record.summary.clone(),
            ]
        })
        .collect::<Vec<_>>();

    print_table(
        &["id", "created_at", "input_type", "risk_lane", "summary"],
        &rows,
    );
}

pub fn print_traces(records: &[TraceRecord]) {
    let rows = records
        .iter()
        .map(|record| {
            vec![
                record.id.to_string(),
                record.created_at.clone(),
                record.outcome.clone().unwrap_or_default(),
                record.task_summary.clone(),
                record.harness_friction.clone().unwrap_or_default(),
            ]
        })
        .collect::<Vec<_>>();
    print_table(
        &[
            "id",
            "created_at",
            "outcome",
            "task_summary",
            "harness_friction",
        ],
        &rows,
    );
}

pub fn print_friction(records: &[FrictionRecord]) {
    let rows = records
        .iter()
        .map(|record| {
            vec![
                record.id.to_string(),
                record.created_at.clone(),
                record.risk_lane.clone().unwrap_or_else(|| "-".to_owned()),
                record.input_type.clone().unwrap_or_else(|| "-".to_owned()),
                record.task_summary.clone(),
                record.harness_friction.clone(),
            ]
        })
        .collect::<Vec<_>>();
    print_table(
        &[
            "id",
            "created_at",
            "risk_lane",
            "input_type",
            "task_summary",
            "harness_friction",
        ],
        &rows,
    );
}

pub fn print_tools_summary(records: &[ToolEntry]) {
    let rows = records
        .iter()
        .map(|record| {
            vec![
                record.command.clone(),
                record.responsibility.clone(),
                record.source.clone(),
                record.description.clone(),
            ]
        })
        .collect::<Vec<_>>();
    print_table(
        &["command", "responsibility", "source", "description"],
        &rows,
    );
}

pub fn print_tools_json(records: &[ToolEntry]) {
    println!("[");
    for (index, record) in records.iter().enumerate() {
        let comma = if index + 1 == records.len() { "" } else { "," };
        println!("  {{");
        println!(
            "    \"provider\": \"{}\",",
            escape_json_string(&record.provider)
        );
        println!("    \"name\": \"{}\",", escape_json_string(&record.name));
        println!(
            "    \"command\": \"{}\",",
            escape_json_string(&record.command)
        );
        println!(
            "    \"description\": \"{}\",",
            escape_json_string(&record.description)
        );
        println!("    \"args\": [");
        for (arg_index, arg) in record.args.iter().enumerate() {
            let arg_comma = if arg_index + 1 == record.args.len() {
                ""
            } else {
                ","
            };
            println!(
                "      {{\"name\":\"{}\",\"type\":\"{}\",\"required\":{},\"help\":\"{}\"}}{}",
                escape_json_string(&arg.name),
                escape_json_string(&arg.arg_type),
                arg.required,
                escape_json_string(arg.help.as_deref().unwrap_or("")),
                arg_comma
            );
        }
        println!("    ],");
        println!(
            "    \"responsibility\": \"{}\",",
            escape_json_string(&record.responsibility)
        );
        println!(
            "    \"source\": \"{}\",",
            escape_json_string(&record.source)
        );
        println!("    \"since\": \"{}\"", escape_json_string(&record.since));
        println!("  }}{comma}");
    }
    println!("]");
}

pub fn print_interventions(records: &[InterventionRecord]) {
    let rows = records
        .iter()
        .map(|record| {
            vec![
                record.id.to_string(),
                record.created_at.clone(),
                record
                    .trace_id
                    .map(|value| value.to_string())
                    .unwrap_or_default(),
                record.story_id.clone().unwrap_or_default(),
                record.intervention_type.clone(),
                record.source.clone(),
                record.description.clone(),
                record.impact.clone().unwrap_or_default(),
            ]
        })
        .collect::<Vec<_>>();
    print_table(
        &[
            "id",
            "created_at",
            "trace",
            "story",
            "type",
            "source",
            "description",
            "impact",
        ],
        &rows,
    );
}

pub fn print_stats(stats: &HarnessStats) {
    println!("=== Harness Stats ===");
    print_table(
        &["intakes", "stories", "decisions", "backlog_items", "traces"],
        &[vec![
            stats.intakes.to_string(),
            stats.stories.to_string(),
            stats.decisions.to_string(),
            stats.backlog_items.to_string(),
            stats.traces.to_string(),
        ]],
    );
}

pub fn print_query_table(table: &QueryTable) {
    let headers = table.headers.iter().map(String::as_str).collect::<Vec<_>>();
    print_table(&headers, &table.rows);
}

pub fn print_work_items(records: &[WorkItem]) {
    let rows = records
        .iter()
        .map(|record| {
            vec![
                record.id.to_string(),
                record.work_type.as_str().to_string(),
                record.title.clone(),
                record.state.as_str().to_string(),
                record.priority.as_db_value().to_string(),
                record.assigned_to.clone().unwrap_or_default(),
            ]
        })
        .collect::<Vec<_>>();
    print_table(
        &["id", "type", "title", "state", "priority", "assigned_to"],
        &rows,
    );
}

pub fn print_work_item_detail(item: &WorkItem) {
    println!("WorkItem #{}", item.id);
    println!("  Type:                {}", item.work_type);
    println!("  Title:               {}", item.title);
    println!("  State:               {}", item.state);
    println!("  Priority:            {}", item.priority);
    if let Some(ref desc) = item.description {
        println!("  Description:         {}", desc);
    }
    if let Some(ref assigned) = item.assigned_to {
        println!("  Assigned To:         {}", assigned);
    }
    if let Some(sp) = item.story_points {
        println!("  Story Points:        {}", sp);
    }
    if let Some(rw) = item.remaining_work {
        println!("  Remaining Work:      {}", rw);
    }
    if let Some(sev) = item.severity {
        println!("  Severity:            {}", sev);
    }
    if let Some(parent) = item.parent_id {
        println!("  Parent ID:           {}", parent);
    }
    if let Some(ref area) = item.area_path {
        println!("  Area Path:           {}", area);
    }
    if let Some(ref iter) = item.iteration_path {
        println!("  Iteration Path:      {}", iter);
    }
    if !item.tags.is_empty() {
        println!("  Tags:                {}", item.tags.join(", "));
    }
    if let Some(ref ac) = item.acceptance_criteria {
        println!("  Acceptance Criteria: {}", ac);
    }
    if let Some(ref repro) = item.repro_steps {
        println!("  Repro Steps:         {}", repro);
    }
    if let Some(ref actual) = item.actual_result {
        println!("  Actual Result:       {}", actual);
    }
    if let Some(ref expected) = item.expected_result {
        println!("  Expected Result:     {}", expected);
    }
    if let Some(ref steps) = item.steps {
        println!("  Steps:               {}", steps);
    }
    println!("  Created At:          {}", item.created_at);
    println!("  Updated At:          {}", item.updated_at);
}

pub fn print_table(headers: &[&str], rows: &[Vec<String>]) {
    let widths = headers
        .iter()
        .enumerate()
        .map(|(index, header)| {
            rows.iter()
                .filter_map(|row| row.get(index))
                .map(String::len)
                .chain(std::iter::once(header.len()))
                .max()
                .unwrap_or(header.len())
        })
        .collect::<Vec<_>>();

    print_row(
        &headers
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>(),
        &widths,
    );
    print_row(
        &widths
            .iter()
            .map(|width| "-".repeat(*width))
            .collect::<Vec<_>>(),
        &widths,
    );
    for row in rows {
        print_row(row, &widths);
    }
}

pub fn print_row(values: &[String], widths: &[usize]) {
    for (index, width) in widths.iter().enumerate() {
        if index > 0 {
            print!("  ");
        }
        let value = values.get(index).map(String::as_str).unwrap_or("");
        print!("{value:<width$}");
    }
    println!();
}
