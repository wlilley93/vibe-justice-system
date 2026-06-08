use clap::{Parser, Subcommand};
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use vjs_core::*;
use vjs_core::types::*;
use vjs_core::error::*;
use vjs_core::route::*;
use vjs_core::spec::*;
use vjs_core::citation::*;
use vjs_lawpack::*;
use vjs_store::*;
use vjs_git::*;
use vjs_redact::*;

#[derive(Parser)]
#[command(name = "vjs")]
#[command(about = "VJS V2 - Deterministic authority resolver and route clerk")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, global = true)]
    json: bool,

    #[arg(long, global = true)]
    repo: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(long)]
        lawpack: Option<String>,
    },
    Route {
        #[arg(long)]
        kind: String,
        #[arg(long)]
        issue: Option<String>,
        #[arg(long)]
        risk: Option<String>,
        #[arg(long)]
        intent: String,
        #[arg(long)]
        public: bool,
        #[arg(long)]
        external: bool,
        #[arg(long)]
        irreversible: bool,
    },
    Lookup {
        #[arg(long)]
        issue: Option<String>,
        #[arg(long)]
        limit: Option<usize>,
    },
    Log {
        #[arg(long)]
        kind: String,
        #[arg(long)]
        issue: String,
        #[arg(long)]
        decision: String,
        #[arg(long)]
        basis: Vec<String>,
        #[arg(long)]
        risk: String,
        #[arg(long)]
        why: String,
    },
    Validate {
        #[arg(long)]
        staged: bool,
        #[arg(long)]
        external: bool,
        #[arg(long)]
        scope: Option<String>,
    },
    LocalCi,
    Order {
        #[command(subcommand)]
        subcmd: OrderCommands,
    },
    File {
        #[arg(long)]
        court: String,
        #[arg(long)]
        question: String,
        #[arg(long)]
        facts_file: Option<PathBuf>,
    },
    Status,
    NextCitation {
        series: String,
        year: Option<i32>,
    },
    MigrateV1 {
        #[arg(long)]
        v1_path: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Permit {
        #[command(subcommand)]
        subcmd: PermitCommands,
    },
}

#[derive(Subcommand)]
enum OrderCommands {
    Validate {
        path: PathBuf,
    },
    Apply {
        path: PathBuf,
    },
}

#[derive(Subcommand)]
enum PermitCommands {
    List,
    Close {
        #[arg(long)]
        id: String,
        #[arg(long)]
        proof: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let repo = cli.repo.unwrap_or_else(|| std::env::current_dir().unwrap());
    let json = cli.json;

    let result = match cli.command {
        Commands::Init { lawpack } => cmd_init(&repo, lawpack),
        Commands::Route { kind, issue, risk, intent, public, external, irreversible } => {
            cmd_route(&repo, kind, issue, risk, intent, public, external, irreversible, json)
        }
        Commands::Lookup { issue, limit } => cmd_lookup(&repo, issue, limit, json),
        Commands::Log { kind, issue, decision, basis, risk, why } => {
            cmd_log(&repo, kind, issue, decision, basis, risk, why, json)
        }
        Commands::Validate { staged, external, scope } => {
            cmd_validate(&repo, staged, external, scope, json)
        }
        Commands::LocalCi => cmd_local_ci(&repo, json),
        Commands::Order { subcmd } => cmd_order(&repo, subcmd, json),
        Commands::File { court, question, facts_file } => {
            cmd_file(&repo, court, question, facts_file, json)
        }
        Commands::Status => cmd_status(&repo, json),
        Commands::NextCitation { series, year } => cmd_next_citation(series, year, json),
        Commands::MigrateV1 { v1_path, out } => cmd_migrate_v1(&v1_path, out, json),
        Commands::Permit { subcmd } => cmd_permit(&repo, subcmd, json),
    };

    if let Err(e) = result {
        if json {
            let err = serde_json::json!({ "error": format!("{}", e) });
            println!("{}", serde_json::to_string_pretty(&err).unwrap());
        } else {
            eprintln!("Error: {}", e);
        }
        std::process::exit(1);
    }
}

fn cmd_init(repo: &PathBuf, lawpack: Option<String>) -> Result<(), KernelError> {
    let git_root = GitIntegration::find_repo_root(repo)?;
    let target = git_root.as_ref().unwrap_or(repo);

    Store::init_repo(target)?;
    GitIntegration::install_hooks(target)?;

    // Write AGENTS.md block
    let agents_md = target.join("AGENTS.md");
    let content = "# VJS V2 Agent Contract\n\nThis repo is governed by VJS V2.\n\nBefore governed load-bearing work, call `vjs.route`.\nIf the route is settled, follow the returned orders/rules.\nIf `court_required=true`, file a short submission.\nAfter material implementation decisions, write a decision log.\nDo not place private repo facts in public records.\nThe kernel answer is the runtime authority surface.\n";
    if !agents_md.exists() {
        std::fs::write(&agents_md, content)
            .map_err(|e| KernelError::Io(e.to_string()))?;
    }

    let lawpack_id = lawpack.as_deref().unwrap_or("vjs-v2");
    Store::write_lawpack_lock(target, lawpack_id, "0.1.0", "sha256:placeholder")?;

    println!("VJS V2 initialized in {}", target.display());
    Ok(())
}

fn cmd_route(
    repo: &PathBuf,
    kind: String,
    issue: Option<String>,
    risk: Option<String>,
    intent: String,
    public: bool,
    external: bool,
    irreversible: bool,
    json: bool,
) -> Result<(), KernelError> {
    let action_kind = parse_action_kind(&kind);
    let risk_level = parse_risk_level(risk.as_deref().unwrap_or("low"));
    let issue_tags = issue.map(|i| vec![IssueTag(i)]).unwrap_or_default();

    let input = RouteInput {
        repo_root: Some(repo.clone()),
        jurisdiction: Some(JurisdictionId("default".into())),
        actor: "lexby".into(),
        action_kind,
        issue_tags,
        intent,
        affected_paths: Vec::new(),
        risk: risk_level,
        public_target: public,
        external_target: external,
        irreversible,
        user_instruction: None,
    };

    let ctx = build_kernel_context(repo)?;
    let decision = route(input, &ctx)?;

    // Save permit if one was created
    if let Some(ref permit_id) = decision.permit_id {
        let permit = Permit {
            id: permit_id.clone(),
            route_id: RouteId(format!("ROUTE-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S"))),
            actor: "lexby".into(),
            scope: None,
            obligations: decision.obligations.clone(),
            expires_at: (chrono::Utc::now() + chrono::Duration::hours(24)).to_rfc3339(),
            status: PermitStatus::Active,
        };
        Store::write_permit(repo, &permit)?;
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&decision).unwrap());
    } else {
        println!("Decision: {:?}", decision.decision);
        println!("Court required: {}", decision.court_required);
        println!("Log required: {}", decision.log_required);
        if !decision.binding.is_empty() {
            println!("Binding authorities:");
            for auth in &decision.binding {
                println!("  - {} ({})", auth.id.0, auth.summary);
            }
        }
        if !decision.must_do.is_empty() {
            println!("Must do:");
            for item in &decision.must_do {
                println!("  - {}", item);
            }
        }
        if !decision.must_not_do.is_empty() {
            println!("Must not do:");
            for item in &decision.must_not_do {
                println!("  - {}", item);
            }
        }
        if let Some(ref permit_id) = decision.permit_id {
            println!("Permit: {}", permit_id.0);
        }
    }

    Ok(())
}

fn cmd_lookup(
    repo: &PathBuf,
    issue: Option<String>,
    limit: Option<usize>,
    json: bool,
) -> Result<(), KernelError> {
    let ctx = build_kernel_context(repo)?;
    let issue_tags = issue.map(|i| vec![IssueTag(i)]).unwrap_or_default();

    let input = RouteInput {
        repo_root: Some(repo.clone()),
        jurisdiction: Some(JurisdictionId("default".into())),
        actor: "lexby".into(),
        action_kind: ActionKind::ImplementationDecision,
        issue_tags,
        intent: "lookup".into(),
        affected_paths: Vec::new(),
        risk: RiskLevel::Low,
        public_target: false,
        external_target: false,
        irreversible: false,
        user_instruction: None,
    };

    let authorities = resolve_authority(&input, &ctx.authority_graph)?;
    let mut authorities = authorities;
    let lim = limit.unwrap_or(5);
    authorities.authorities.truncate(lim);

    if json {
        println!("{}", serde_json::to_string_pretty(&authorities).unwrap());
    } else {
        for auth in &authorities.authorities {
            println!("{} ({:?}): {}", auth.id.0, auth.rank, auth.summary);
        }
    }

    Ok(())
}

fn cmd_log(
    repo: &PathBuf,
    kind: String,
    issue: String,
    decision: String,
    basis: Vec<String>,
    risk: String,
    why: String,
    json: bool,
) -> Result<(), KernelError> {
    let log = DecisionLog {
        id: format!("LOG-{}", chrono::Utc::now().format("%Y-%m-%d-%H%M%S")),
        time: chrono::Utc::now().to_rfc3339(),
        actor: "lexby".into(),
        kind,
        issue,
        decision,
        basis,
        risk: parse_risk_level(&risk),
        reversibility: "easy".into(),
        court_required: false,
        why,
    };

    // Validate word count
    let word_count = log.why.split_whitespace().count();
    if word_count > 150 {
        return Err(KernelError::WordLimitExceeded {
            actual: word_count,
            limit: 150,
        });
    }

    Store::write_log(repo, &log)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&log).unwrap());
    } else {
        println!("Decision log written: {}", log.id);
    }

    Ok(())
}

fn cmd_validate(
    repo: &PathBuf,
    staged: bool,
    external: bool,
    _scope: Option<String>,
    json: bool,
) -> Result<(), KernelError> {
    let mut findings = Vec::new();
    let mut ok = true;

    // Load lawpack
    let lawpack = load_lawpack(repo)?;
    let report = LawpackValidator::validate(&lawpack)?;
    if !report.ok {
        ok = false;
    }
    findings.extend(report.findings.into_iter().map(|f| ValidationFinding {
        severity: f.severity,
        code: f.code,
        path: f.path,
        message: f.message,
        suggested_fix: f.suggested_fix,
    }));

    if staged {
        let changed = GitIntegration::read_staged_files(repo)?;
        if changed.is_empty() {
            findings.push(ValidationFinding {
                severity: Severity::Info,
                code: "NO_STAGED_FILES".into(),
                path: None,
                message: "No staged files to validate".into(),
                suggested_fix: None,
            });
        } else {
            findings.push(ValidationFinding {
                severity: Severity::Info,
                code: "STAGED_FILES".into(),
                path: None,
                message: format!("{} staged files", changed.len()),
                suggested_fix: None,
            });
            // Build repo state and evaluate invariants
            let repo_state = RepoScanner::build_repo_state(repo)?;
            let invariant_report = evaluate_invariants(&repo_state, &lawpack.invariants)?;
            let mut invariant_failures = false;
            for finding in &invariant_report.findings {
                if !finding.passed {
                    invariant_failures = true;
                    findings.push(ValidationFinding {
                        severity: finding.severity.clone(),
                        code: finding.invariant_id.0.clone(),
                        path: None,
                        message: finding.message.clone(),
                        suggested_fix: Some(finding.remedy.clone()),
                    });
                }
            }
            if invariant_failures {
                ok = false;
            } else {
                findings.push(ValidationFinding {
                    severity: Severity::Info,
                    code: "INVARIANTS_PASS".into(),
                    path: None,
                    message: format!("{} invariants evaluated, all passed", invariant_report.findings.len()),
                    suggested_fix: None,
                });
            }
        }
    }

    if external {
        if GitIntegration::is_public_remote(repo)? {
            findings.push(ValidationFinding {
                severity: Severity::Warning,
                code: "PUBLIC_REMOTE".into(),
                path: None,
                message: "Repository has a public remote. Release warrant may be required.".into(),
                suggested_fix: Some("Run vjs release-warrant check".into()),
            });
        }
    }

    // Boundary scan
    let vjs_dir = repo.join(".vjs");
    if vjs_dir.exists() {
        let boundary_findings = RedactScanner::scan_directory(&vjs_dir)?;
        if !RedactScanner::check_public_safe(&boundary_findings) {
            ok = false;
            for f in boundary_findings {
                if matches!(f.severity, Severity::Fatal | Severity::Error) {
                    findings.push(ValidationFinding {
                        severity: f.severity,
                        code: "BOUNDARY_VIOLATION".into(),
                        path: f.path,
                        message: f.message,
                        suggested_fix: Some(format!("{:?}", f.suggested_route)),
                    });
                }
            }
        }
    }

    let result = ValidationResult { ok, findings };

    if json {
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else {
        println!("Validation: {}", if ok { "OK" } else { "FAILED" });
        for finding in &result.findings {
            println!("  [{:?}] {}: {}", finding.severity, finding.code, finding.message);
        }
    }

    if !ok {
        std::process::exit(1);
    }

    Ok(())
}

fn cmd_local_ci(repo: &PathBuf, json: bool) -> Result<(), KernelError> {
    let mut ok = true;
    let mut steps = Vec::new();

    // Step 1: Lawpack validation
    let lawpack = load_lawpack(repo)?;
    let report = LawpackValidator::validate(&lawpack)?;
    steps.push(CiStep {
        name: "lawpack_validate".into(),
        passed: report.ok,
        message: if report.ok { "Lawpack valid".into() } else { "Lawpack invalid".into() },
    });
    if !report.ok {
        ok = false;
    }

    // Step 2: Citation check
    let mut citation_ok = true;
    let mut seen = std::collections::HashSet::new();
    for order in &lawpack.orders {
        if !seen.insert(order.id.clone()) {
            citation_ok = false;
        }
    }
    steps.push(CiStep {
        name: "citation_check".into(),
        passed: citation_ok,
        message: if citation_ok { "No citation collisions".into() } else { "Citation collision detected".into() },
    });
    if !citation_ok {
        ok = false;
    }

    // Step 3: Boundary scan
    let vjs_dir = repo.join(".vjs");
    let boundary_ok = if vjs_dir.exists() {
        let findings = RedactScanner::scan_directory(&vjs_dir)?;
        RedactScanner::check_public_safe(&findings)
    } else {
        true
    };
    steps.push(CiStep {
        name: "boundary_scan".into(),
        passed: boundary_ok,
        message: if boundary_ok { "Boundary scan passed".into() } else { "Boundary scan failed".into() },
    });
    if !boundary_ok {
        ok = false;
    }

    // Step 4: Order validation
    let mut order_ok = true;
    for order in &lawpack.orders {
        if order.holding.is_empty() || order.directives.is_empty() || order.runtime_summary.is_empty() {
            order_ok = false;
        }
    }
    steps.push(CiStep {
        name: "order_validate".into(),
        passed: order_ok,
        message: if order_ok { "Orders valid".into() } else { "Invalid orders found".into() },
    });
    if !order_ok {
        ok = false;
    }

    // Step 5: Invariant evaluation
    let repo_state = RepoScanner::build_repo_state(repo)?;
    let invariant_report = evaluate_invariants(&repo_state, &lawpack.invariants)?;
    let invariant_ok = invariant_report.findings.iter().all(|f| f.passed);
    steps.push(CiStep {
        name: "invariant_eval".into(),
        passed: invariant_ok,
        message: if invariant_ok {
            format!("{} invariants passed", invariant_report.findings.len())
        } else {
            let failures: Vec<_> = invariant_report.findings.iter().filter(|f| !f.passed).map(|f| f.invariant_id.0.clone()).collect();
            format!("Invariant failures: {}", failures.join(", "))
        },
    });
    if !invariant_ok {
        ok = false;
    }

    let result = CiResult { ok, steps };

    if json {
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else {
        println!("Local CI: {}", if ok { "PASS" } else { "FAIL" });
        for step in &result.steps {
            println!("  {}: {}", step.name, if step.passed { "PASS" } else { "FAIL" });
        }
    }

    if !ok {
        std::process::exit(1);
    }

    Ok(())
}

fn cmd_order(repo: &PathBuf, subcmd: OrderCommands, json: bool) -> Result<(), KernelError> {
    match subcmd {
        OrderCommands::Validate { path } => {
            let content = std::fs::read_to_string(&path)
                .map_err(|e| KernelError::Io(e.to_string()))?;
            let order: Order = serde_yaml::from_str(&content)
                .map_err(|e| KernelError::Serialization(e.to_string()))?;

            let mut ok = true;
            let mut findings: Vec<String> = Vec::new();

            if order.holding.is_empty() {
                ok = false;
                findings.push("holding is required".into());
            }
            if order.directives.is_empty() {
                ok = false;
                findings.push("directives are required".into());
            }
            if order.runtime_summary.is_empty() {
                ok = false;
                findings.push("runtime_summary is required".into());
            }

            if json {
                println!("{}", serde_json::to_string_pretty(&serde_json::json!({
                    "ok": ok,
                    "findings": findings
                })).unwrap());
            } else {
                println!("Order validation: {}", if ok { "OK" } else { "FAILED" });
                for f in &findings {
                    println!("  - {}", f);
                }
            }

            if !ok {
                std::process::exit(1);
            }
        }
        OrderCommands::Apply { path } => {
            let content = std::fs::read_to_string(&path)
                .map_err(|e| KernelError::Io(e.to_string()))?;
            let order: Order = serde_yaml::from_str(&content)
                .map_err(|e| KernelError::Serialization(e.to_string()))?;

            Store::write_order(repo, &order)?;

            println!("Order applied: {}", order.id);
        }
    }

    Ok(())
}

fn cmd_file(
    repo: &PathBuf,
    court: String,
    question: String,
    facts_file: Option<PathBuf>,
    json: bool,
) -> Result<(), KernelError> {
    let facts = if let Some(path) = facts_file {
        std::fs::read_to_string(&path)
            .map_err(|e| KernelError::Io(e.to_string()))?
    } else {
        String::new()
    };

    let word_count = question.split_whitespace().count() + facts.split_whitespace().count();
    if word_count > 500 {
        return Err(KernelError::WordLimitExceeded {
            actual: word_count,
            limit: 500,
        });
    }

    let submission = Submission {
        id: format!("SUBMISSION-{}", chrono::Utc::now().format("%Y-%m-%d-%H%M%S")),
        court_requested: court,
        jurisdiction: "default".into(),
        question,
        facts,
        requested_order: "TBD".into(),
        private_boundary: "local".into(),
        word_count,
    };

    Store::write_submission(repo, &submission)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&submission).unwrap());
    } else {
        println!("Submission filed: {}", submission.id);
    }

    Ok(())
}

fn cmd_status(repo: &PathBuf, json: bool) -> Result<(), KernelError> {
    let git_root = GitIntegration::find_repo_root(repo)?;
    let is_git = git_root.is_some();
    let is_public = if is_git {
        GitIntegration::is_public_remote(&git_root.as_ref().unwrap_or(repo)).unwrap_or(false)
    } else {
        false
    };

    let vjs_dir = repo.join(".vjs");
    let vjs_installed = vjs_dir.exists();

    let lock = Store::read_lawpack_lock(repo)?;
    let lawpack_info = lock.map(|l| format!("{}@{}", l.lawpack_id, l.lawpack_version));

    let logs = if vjs_installed {
        Store::read_logs(repo)?.len()
    } else {
        0
    };

    let orders = if vjs_installed {
        Store::read_orders(repo)?.len()
    } else {
        0
    };

    let status = StatusInfo {
        repo: repo.display().to_string(),
        git_repo: is_git,
        public_remote: is_public,
        vjs_installed,
        lawpack: lawpack_info,
        logs_count: logs,
        orders_count: orders,
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&status).unwrap());
    } else {
        println!("Repo: {}", status.repo);
        println!("Git: {}", status.git_repo);
        println!("Public remote: {}", status.public_remote);
        println!("VJS installed: {}", status.vjs_installed);
        if let Some(ref lp) = status.lawpack {
            println!("Lawpack: {}", lp);
        }
        println!("Logs: {}", status.logs_count);
        println!("Orders: {}", status.orders_count);
    }

    Ok(())
}

fn cmd_next_citation(series: String, year: Option<i32>, json: bool) -> Result<(), KernelError> {
    let y = year.unwrap_or_else(|| chrono::Utc::now().year());
    let citation_series = match series.as_str() {
        "cc" => CitationSeries::Cc("REPO".into()),
        "pc" => CitationSeries::Pc,
        "sc" => CitationSeries::Sc,
        "reg" => CitationSeries::Reg,
        "act" => CitationSeries::Act,
        _ => CitationSeries::Cc("REPO".into()),
    };

    let registry = CitationRegistry::new();
    let next = registry.next_citation(citation_series, y);

    let citation_str = format!("[{}] VJS-{:?} {}", next.year, next.series, next.n);

    if json {
        println!("{}", serde_json::to_string_pretty(&serde_json::json!({
            "year": next.year,
            "series": series,
            "n": next.n,
            "citation": citation_str
        })).unwrap());
    } else {
        println!("Next citation: {}", citation_str);
    }

    Ok(())
}

fn cmd_migrate_v1(_v1_path: &PathBuf, out: Option<PathBuf>, json: bool) -> Result<(), KernelError> {
    let output = out.unwrap_or_else(|| PathBuf::from("migration/draft-ledger.yaml"));
    std::fs::create_dir_all(output.parent().unwrap_or(&PathBuf::from(".")))
        .map_err(|e| KernelError::Io(e.to_string()))?;

    let ledger = MigrationLedger {
        version: "v2-draft".into(),
        entries: vec![
            MigrationEntry {
                id: "L-001".into(),
                title: "Memory is not authority".into(),
                status: MigrationStatus::Migrated,
                v1_sources: vec![
                    V1Source { file: "Constitution/AGENTS.md".into(), reference: "retrieval-first".into() },
                    V1Source { file: "AGENTS.md".into(), reference: "cdd-cli-spine".into() },
                ],
                v2_destination: V2Destination {
                    statute: "ACT-AGENT-DUTIES".into(),
                    rule: "AGENT-LOOKUP-001".into(),
                    invariant: "INV-PERMIT-REQUIRED".into(),
                },
                runtime_effect: vec![
                    "route lookup required before governed work".into(),
                    "missing permit blocks governed write".into(),
                ],
            },
            MigrationEntry {
                id: "L-002".into(),
                title: "Five court triggers".into(),
                status: MigrationStatus::Migrated,
                v1_sources: vec![
                    V1Source { file: "README.md".into(), reference: "five-triggers".into() },
                ],
                v2_destination: V2Destination {
                    statute: "ACT-COURTS-ORDERS".into(),
                    rule: "RULE-COURT-TRIGGER".into(),
                    invariant: "INV-COURT-REQUIRED".into(),
                },
                runtime_effect: vec![
                    "court only convenes for defined triggers".into(),
                    "everything else is citation fast-path".into(),
                ],
            },
            MigrationEntry {
                id: "L-003".into(),
                title: "Public/private boundary".into(),
                status: MigrationStatus::Migrated,
                v1_sources: vec![
                    V1Source { file: "README.md".into(), reference: "public-private".into() },
                    V1Source { file: ".gitignore".into(), reference: "private-exclusions".into() },
                ],
                v2_destination: V2Destination {
                    statute: "ACT-PUBLIC-PRIVATE".into(),
                    rule: "DATA-PRIVATE-001".into(),
                    invariant: "INV-PUBLIC-NO-PRIVATE-FACTS".into(),
                },
                runtime_effect: vec![
                    "block unredacted private evidence from public record".into(),
                    "route local facts to .vjs/private".into(),
                ],
            },
        ],
    };

    let content = serde_yaml::to_string(&ledger)
        .map_err(|e| KernelError::Serialization(e.to_string()))?;
    std::fs::write(&output, content)
        .map_err(|e| KernelError::Io(e.to_string()))?;

    if json {
        println!("{}", serde_json::to_string_pretty(&ledger).unwrap());
    } else {
        println!("Migration ledger written to {}", output.display());
        println!("Entries: {}", ledger.entries.len());
    }

    Ok(())
}

// Helpers

fn parse_action_kind(s: &str) -> ActionKind {
    match s {
        "implementation-decision" => ActionKind::ImplementationDecision,
        "public-record-change" => ActionKind::PublicRecordChange,
        "private-record-change" => ActionKind::PrivateRecordChange,
        "external-act" => ActionKind::ExternalAct,
        "release-or-push" => ActionKind::ReleaseOrPush,
        "security-sensitive-act" => ActionKind::SecuritySensitiveAct,
        "data-boundary-decision" => ActionKind::DataBoundaryDecision,
        "court-filing" => ActionKind::CourtFiling,
        "legislative-draft" => ActionKind::LegislativeDraft,
        "refactor" => ActionKind::Refactor,
        "trivial-preference" => ActionKind::TrivialPreference,
        "dependency-change" => ActionKind::DependencyChange,
        "schema-change" => ActionKind::SchemaChange,
        _ => ActionKind::ImplementationDecision,
    }
}

fn parse_risk_level(s: &str) -> RiskLevel {
    match s {
        "low" => RiskLevel::Low,
        "medium" => RiskLevel::Medium,
        "high" => RiskLevel::High,
        "critical" => RiskLevel::Critical,
        _ => RiskLevel::Low,
    }
}

fn build_kernel_context(repo: &PathBuf) -> Result<KernelContext, KernelError> {
    let lawpack = load_lawpack(repo)?;
    let graph = lawpack.build_authority_graph()?;
    let digest = compute_digest(repo)?;

    Ok(KernelContext {
        authority_graph: graph,
        limits: ContextLimits::default(),
        lawpack_digest: digest,
    })
}

fn load_lawpack(repo: &PathBuf) -> Result<Lawpack, KernelError> {
    let lawpack_dir = repo.join("lawpack/v2");
    if lawpack_dir.exists() {
        LawpackLoader::load(&lawpack_dir)
    } else {
        // Fallback: use empty lawpack
        Ok(Lawpack {
            statutes: Vec::new(),
            regulations: Vec::new(),
            rules: Vec::new(),
            orders: Vec::new(),
            specs: Vec::new(),
            invariants: Vec::new(),
            decisions: Vec::new(),
        })
    }
}

fn compute_digest(repo: &PathBuf) -> Result<String, KernelError> {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    let manifest = repo.join("lawpack/v2/manifest.toml");
    if manifest.exists() {
        let content = std::fs::read(&manifest)
            .map_err(|e| KernelError::Io(e.to_string()))?;
        hasher.update(&content);
    }
    Ok(format!("sha256:{}", hex::encode(hasher.finalize())))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ValidationResult {
    ok: bool,
    findings: Vec<ValidationFinding>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ValidationFinding {
    severity: Severity,
    code: String,
    path: Option<PathBuf>,
    message: String,
    suggested_fix: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CiResult {
    ok: bool,
    steps: Vec<CiStep>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CiStep {
    name: String,
    passed: bool,
    message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StatusInfo {
    repo: String,
    git_repo: bool,
    public_remote: bool,
    vjs_installed: bool,
    lawpack: Option<String>,
    logs_count: usize,
    orders_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MigrationLedger {
    version: String,
    entries: Vec<MigrationEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MigrationEntry {
    id: String,
    title: String,
    status: MigrationStatus,
    v1_sources: Vec<V1Source>,
    v2_destination: V2Destination,
    runtime_effect: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct V1Source {
    file: String,
    reference: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct V2Destination {
    statute: String,
    rule: String,
    invariant: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum MigrationStatus {
    Migrated,
    ArchiveOnly,
    Rejected,
    Deferred,
    Duplicate,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct LawpackLock {
    lawpack_id: String,
    lawpack_version: String,
    digest: String,
    generated_at: String,
}

fn cmd_permit(repo: &PathBuf, subcmd: PermitCommands, json: bool) -> Result<(), KernelError> {
    match subcmd {
        PermitCommands::List => {
            let permits = Store::read_permits(repo)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&permits).unwrap());
            } else {
                if permits.is_empty() {
                    println!("No active permits");
                } else {
                    for permit in &permits {
                        println!("{} ({:?}): {} obligations", permit.id.0, permit.status, permit.obligations.len());
                    }
                }
            }
            Ok(())
        }
        PermitCommands::Close { id, proof } => {
            let mut permits = Store::read_permits(repo)?;
            let mut found = false;
            for permit in &mut permits {
                if permit.id.0 == id {
                    permit.status = PermitStatus::Closed;
                    found = true;
                    if let Some(proof_content) = proof {
                        let proof_id = ProofId(format!("PROOF-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S")));
                        let proof = Proof {
                            id: proof_id,
                            permit_id: permit.id.clone(),
                            kind: ProofKind::DecisionLog,
                            status: ProofStatus::Passed,
                            digest: None,
                            captured_at: chrono::Utc::now().to_rfc3339(),
                        };
                        // Store proof separately? For now, just print it
                        if json {
                            println!("{}", serde_json::to_string_pretty(&proof).unwrap());
                        }
                    }
                    break;
                }
            }
            if !found {
                return Err(KernelError::PermitNotFound(id));
            }
            // Write back all permits
            for permit in permits {
                Store::write_permit(repo, &permit)?;
            }
            if json {
                println!("{{ \"ok\": true, \"permit_id\": \"{}\" }}", id);
            } else {
                println!("Permit {} closed", id);
            }
            Ok(())
        }
    }
}
