use clap::{Parser, Subcommand};
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use vjs_core::*;
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
        /// The governed paths this action covers. The issued permit is scoped to
        /// exactly these paths (a permit with no paths excuses nothing).
        #[arg(long = "path")]
        paths: Vec<PathBuf>,
        /// Function-hook mode: exit non-zero (fail closed) when the kernel
        /// requires a court or blocks, so an executable hook can gate the action.
        #[arg(long)]
        gate: bool,
    },
    /// Functional hook (REG-HOOKS-001): deterministic decision over repo state.
    Hook {
        #[arg(long)]
        event: String,
        #[arg(long = "path")]
        paths: Vec<PathBuf>,
        #[arg(long)]
        tool: Option<String>,
    },
    /// Local sovereign invocation (REG-INVOCATION-001): bind this repo as a VJS
    /// jurisdiction by subscribing + locking the lawpack, recording the
    /// invocation, and (with --install-hooks) activating the enforcement hooks.
    Invoke {
        #[arg(long)]
        jurisdiction: String,
        #[arg(long)]
        principal: String,
        #[arg(long)]
        lawpack: Option<String>,
        /// Set git core.hooksPath so the permit gate fires at commit time.
        #[arg(long)]
        install_hooks: bool,
    },
    Lookup {
        #[arg(long)]
        issue: Option<String>,
        #[arg(long)]
        limit: Option<usize>,
    },
    Log {
        #[command(subcommand)]
        subcmd: LogCommands,
    },
    Proof {
        #[command(subcommand)]
        subcmd: ProofCommands,
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
    Eval {
        /// Suite to run: agent-harness | prompts | route | all
        suite: Option<String>,
    },
    /// Publish the Gazette data: both estates (the V2 canon from the lawpack,
    /// the V1 archive from provenance) as one machine-readable file consumed
    /// by gazette.html and gazette-graph.html. Publication is constitutively
    /// inert (REG-GAZETTE-CONTINUITY-001).
    Gazette {
        /// Output path (default: <repo>/gazette-data.js)
        #[arg(long)]
        out: Option<PathBuf>,
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

#[derive(Subcommand)]
enum LogCommands {
    Decision {
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
    FromPermit {
        #[arg(long)]
        permit_id: String,
        #[arg(long)]
        decision: String,
        #[arg(long)]
        why: String,
    },
}

#[derive(Subcommand)]
enum ProofCommands {
    Add {
        #[arg(long)]
        permit_id: String,
        #[arg(long)]
        kind: Option<String>,
        #[arg(long)]
        status: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let repo = cli.repo.unwrap_or_else(|| {
        std::env::current_dir().unwrap_or_else(|e| {
            eprintln!("Error: cannot determine current directory: {} (pass --repo)", e);
            std::process::exit(1);
        })
    });
    let json = cli.json;

    let result = match cli.command {
        Commands::Init { lawpack } => cmd_init(&repo, lawpack),
        Commands::Route { kind, issue, risk, intent, public, external, irreversible, paths, gate } => {
            cmd_route(&repo, kind, issue, risk, intent, public, external, irreversible, paths, gate, json)
        }
        Commands::Hook { event, paths, tool } => cmd_hook(&repo, event, paths, tool, json),
        Commands::Invoke { jurisdiction, principal, lawpack, install_hooks } => {
            cmd_invoke(&repo, jurisdiction, principal, lawpack, install_hooks, json)
        }
        Commands::Lookup { issue, limit } => cmd_lookup(&repo, issue, limit, json),
        Commands::Log { subcmd } => cmd_log(&repo, subcmd, json),
        Commands::Proof { subcmd } => cmd_proof(&repo, subcmd, json),
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
        Commands::Eval { suite } => cmd_eval(&repo, suite, json),
        Commands::Gazette { out } => cmd_gazette(&repo, out, json),
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

fn cmd_init(repo: &Path, lawpack: Option<String>) -> Result<(), KernelError> {
    let git_root = GitIntegration::find_repo_root(repo)?;
    let target = git_root.as_deref().unwrap_or(repo);

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

#[allow(clippy::too_many_arguments)]
fn cmd_route(
    repo: &Path,
    kind: String,
    issue: Option<String>,
    risk: Option<String>,
    intent: String,
    public: bool,
    external: bool,
    irreversible: bool,
    paths: Vec<PathBuf>,
    gate: bool,
    json: bool,
) -> Result<(), KernelError> {
    let action_kind = parse_action_kind(&kind);
    let risk_level = parse_risk_level(risk.as_deref().unwrap_or("low"));
    let issue_tags = issue.map(|i| vec![IssueTag(i)]).unwrap_or_default();
    let path_globs: Vec<String> = paths.iter().map(|p| p.to_string_lossy().to_string()).collect();

    let input = RouteInput {
        repo_root: Some(repo.to_path_buf()),
        jurisdiction: Some(JurisdictionId("default".into())),
        actor: "lexby".into(),
        action_kind,
        issue_tags,
        intent,
        affected_paths: paths.clone(),
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
            scope: if path_globs.is_empty() {
                None
            } else {
                Some(Scope {
                    paths: Some(path_globs.clone()),
                    jurisdictions: None,
                    action_kinds: None,
                    issue_tags: None,
                    records: None,
                })
            },
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

    // Function-hook mode: fail closed so an executable hook can gate the action.
    if gate {
        match decision.decision {
            RouteOutcome::CourtRequired => {
                eprintln!("BLOCK: court required - convene on own motion; do not route the fork to the Principal.");
                std::process::exit(2);
            }
            RouteOutcome::Blocked => {
                eprintln!("BLOCK: action blocked by the kernel.");
                std::process::exit(2);
            }
            RouteOutcome::HumanApprovalRequired
            | RouteOutcome::ReleaseWarrantRequired
            | RouteOutcome::PrivateBoundaryRequired => {
                eprintln!("BLOCK: a warrant/approval/boundary condition must be satisfied first.");
                std::process::exit(3);
            }
            _ => {}
        }
    }

    Ok(())
}

fn cmd_hook(
    repo: &Path,
    event: String,
    paths: Vec<PathBuf>,
    tool: Option<String>,
    json: bool,
) -> Result<(), KernelError> {
    let hook_event = vjs_core::hook::parse_event(&event)
        .ok_or_else(|| KernelError::InvalidInput(format!("unknown hook event: {}", event)))?;
    let input = vjs_core::hook::HookInput {
        event: hook_event,
        repo_root: repo.to_path_buf(),
        actor: "lexby".into(),
        paths,
        tool,
    };
    let ctx = build_kernel_context(repo)?;
    let decision = vjs_core::hook::evaluate(&input, &ctx);

    if json {
        println!("{}", serde_json::to_string_pretty(&decision).unwrap());
    } else {
        println!("[{}] {}", decision.code(), decision.message());
    }

    let code = decision.exit_code();
    if code != 0 {
        std::process::exit(code);
    }
    Ok(())
}

fn cmd_invoke(
    repo: &Path,
    jurisdiction: String,
    principal: String,
    lawpack: Option<String>,
    install_hooks: bool,
    json: bool,
) -> Result<(), KernelError> {
    let io = |e: std::io::Error| KernelError::InvalidInput(format!("io: {}", e));
    let lawpack = lawpack.unwrap_or_else(|| "vjs-v2@0.1.0".into());
    let repo_code = jurisdiction.to_uppercase();
    let vjs_dir = repo.join(".vjs");
    std::fs::create_dir_all(vjs_dir.join("invocation")).map_err(io)?;

    let digest = build_kernel_context(repo)?.lawpack_digest;
    let now = chrono::Utc::now();
    let stamp = now.format("%Y%m%d-%H%M%S").to_string();
    let now_rfc = now.to_rfc3339();

    // 1. config.toml - write only if absent (never clobber an existing config).
    // create_new makes the existence check and the write one atomic act, so a
    // config that appears between check and write survives untouched.
    let config_path = vjs_dir.join("config.toml");
    let config = format!(
        "version = \"2\"\njurisdiction_id = \"{jur}\"\nrepo_code = \"{code}\"\nlawpack = \"{lp}\"\nprincipal = \"{prin}\"\n\n[paths]\norders = \".vjs/orders\"\nlogs = \".vjs/logs\"\nsubmissions = \".vjs/submissions\"\nproofs = \".vjs/proofs\"\npermits = \".vjs/permits\"\nprivate = \".vjs/private\"\n\n[paths.public]\nenabled = false\n\n[governance]\npermit_required = [\"src/**\", \"crates/**\", \"lawpack/**\", \"Cargo.toml\", \"package.json\", \"AGENTS.md\", \"VJS.md\", \"README.md\"]\npermit_exempt = [\".vjs/logs/**\", \".vjs/permits/**\", \".vjs/proofs/**\", \".vjs/cache/**\", \".vjs/private/**\", \"target/**\", \"node_modules/**\"]\n",
        jur = jurisdiction, code = repo_code, lp = lawpack, prin = principal,
    );
    let config_written = match std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&config_path)
    {
        Ok(mut f) => {
            use std::io::Write;
            f.write_all(config.as_bytes()).map_err(io)?;
            true
        }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => false,
        Err(e) => return Err(io(e)),
    };

    // 2. lawpack.lock - pin the lawpack digest.
    let lock = format!(
        "lawpack = \"{lp}\"\ndigest = \"{dig}\"\nlocked_at = \"{ts}\"\nlocked_by = \"{prin}\"\n",
        lp = lawpack, dig = digest, ts = now_rfc, prin = principal,
    );
    std::fs::write(vjs_dir.join("lawpack.lock"), lock).map_err(io)?;

    // 3. the local sovereign invocation record (the constitutional act).
    let inv = format!(
        "id: INVOCATION-{stamp}\nkind: local_sovereign_invocation\nstatus: in_force\njurisdiction:\n  id: {jur}\n  repo_root: \".\"\n  repo_code: {code}\nprincipal:\n  name: \"{prin}\"\n  capacity: local_sovereign\nsubscription:\n  lawpack: {lp}\n  lawpack_lock: .vjs/lawpack.lock\n  mode: subscribed\n  v1_archive_import: none_unless_expressly_incorporated\nassent:\n  given: true\n  form: local_sovereign_act\n  statement: >\n    The Principal invokes this repository as a VJS V2 local jurisdiction,\n    subscribes it to the stated lawpack, and authorises the kernel, hooks,\n    permits, proofs, logs, and court route to govern repo work.\neffect:\n  - creates_local_jurisdiction\n  - creates_county_court_for_repo\n  - binds_agents_to_kernel_route\n  - requires_permits_for_governed_writes\n  - requires_logs_for_material_decisions\n  - installs_validation_hooks\n",
        stamp = stamp, jur = jurisdiction, code = repo_code, prin = principal, lp = lawpack,
    );
    let inv_path = vjs_dir.join("invocation").join(format!("{}-local-sovereign-invocation.yaml", stamp));
    std::fs::write(&inv_path, inv).map_err(io)?;

    // 4. install enforcement hooks (the activation): git core.hooksPath + tiny
    // extensionless wrappers git will run, made executable.
    let mut hooks_installed = false;
    if install_hooks {
        let hooks_dir = vjs_dir.join("hooks");
        std::fs::create_dir_all(&hooks_dir).map_err(io)?;
        // Call the V2 kernel by its absolute path (the binary running this
        // invoke), so the hook never depends on PATH `vjs` resolving to the
        // wrong tool (e.g. a V1 CLI).
        let exe = std::env::current_exe()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "vjs".into());
        std::fs::write(hooks_dir.join("pre-commit"),
            format!("#!/usr/bin/env bash\nexec \"{}\" validate --staged\n", exe)).map_err(io)?;
        std::fs::write(hooks_dir.join("pre-push"),
            format!("#!/usr/bin/env bash\nexec \"{}\" local-ci\n", exe)).map_err(io)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            for h in ["pre-commit", "pre-push"] {
                let p = hooks_dir.join(h);
                if let Ok(meta) = std::fs::metadata(&p) {
                    let mut perm = meta.permissions();
                    perm.set_mode(0o755);
                    let _ = std::fs::set_permissions(&p, perm);
                }
            }
        }
        let out = std::process::Command::new("git")
            .args(["-C", &repo.to_string_lossy(), "config", "core.hooksPath", ".vjs/hooks"])
            .output();
        hooks_installed = out.map(|o| o.status.success()).unwrap_or(false);
    }

    if json {
        println!("{}", serde_json::json!({
            "jurisdiction": jurisdiction,
            "repo_code": repo_code,
            "lawpack": lawpack,
            "lawpack_digest": digest,
            "invocation": inv_path.to_string_lossy(),
            "config_written": config_written,
            "hooks_installed": hooks_installed,
        }));
    } else {
        println!("Invoked '{}' as a VJS jurisdiction (repo_code {}).", jurisdiction, repo_code);
        println!("  lawpack: {} ({}...)", lawpack, &digest[..digest.len().min(23)]);
        println!("  invocation: {}", inv_path.display());
        println!("  config written: {}", config_written);
        println!("  hooks installed (core.hooksPath): {}", hooks_installed);
        if !install_hooks {
            println!("  (run with --install-hooks to activate commit-time enforcement)");
        }
    }
    Ok(())
}

fn cmd_lookup(
    repo: &Path,
    issue: Option<String>,
    limit: Option<usize>,
    json: bool,
) -> Result<(), KernelError> {
    let ctx = build_kernel_context(repo)?;
    let issue_tags = issue.map(|i| vec![IssueTag(i)]).unwrap_or_default();

    let input = RouteInput {
        repo_root: Some(repo.to_path_buf()),
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
    repo: &Path,
    subcmd: LogCommands,
    json: bool,
) -> Result<(), KernelError> {
    match subcmd {
        LogCommands::Decision { kind, issue, decision, basis, risk, why } => {
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
        LogCommands::FromPermit { permit_id, decision, why } => {
            let permits = Store::read_permits(repo)?;
            let permit = permits.into_iter().find(|p| p.id.0 == permit_id)
                .ok_or_else(|| KernelError::PermitNotFound(permit_id.clone()))?;

            let log = DecisionLog {
                id: format!("LOG-{}", chrono::Utc::now().format("%Y-%m-%d-%H%M%S")),
                time: chrono::Utc::now().to_rfc3339(),
                actor: "lexby".into(),
                kind: "decision".into(),
                issue: permit.route_id.0.clone(),
                decision,
                basis: vec![permit_id.clone()],
                risk: RiskLevel::Low,
                reversibility: "easy".into(),
                court_required: false,
                why,
            };

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
                println!("Decision log written: {} (from permit {})", log.id, permit_id);
            }
            Ok(())
        }
    }
}

fn cmd_proof(
    repo: &Path,
    subcmd: ProofCommands,
    json: bool,
) -> Result<(), KernelError> {
    match subcmd {
        ProofCommands::Add { permit_id, kind, status } => {
            let proof_id = ProofId(format!("PROOF-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S")));
            let proof_status = match status.as_deref() {
                Some("passed") => ProofStatus::Passed,
                Some("failed") => ProofStatus::Failed,
                _ => ProofStatus::Pending,
            };
            let proof_kind = match kind.as_deref() {
                Some("decision_log") => ProofKind::DecisionLog,
                Some("test_result") => ProofKind::TestResult,
                Some("command_result") => ProofKind::CommandResult,
                Some("public_private_scan") => ProofKind::PublicPrivateScan,
                Some("validation_report") => ProofKind::ValidationReport,
                _ => ProofKind::CommandResult,
            };

            let proof = Proof {
                id: proof_id,
                permit_id: PermitId(permit_id),
                kind: proof_kind,
                status: proof_status,
                digest: None,
                captured_at: chrono::Utc::now().to_rfc3339(),
            };

            Store::write_proof(repo, &proof)?;

            if json {
                println!("{}", serde_json::to_string_pretty(&proof).unwrap());
            } else {
                println!("Proof written: {}", proof.id.0);
            }
            Ok(())
        }
    }
}

fn cmd_validate(
    repo: &Path,
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

    // Referential integrity over the raw lawpack files: a citation of a law
    // object that exists nowhere is drift the citator cannot be trusted under.
    let lawpack_dir = repo.join("lawpack/v2");
    if lawpack_dir.exists() {
        findings.extend(
            LawpackValidator::check_referential_integrity(&lawpack_dir, &lawpack)?
                .into_iter()
                .map(|f| ValidationFinding {
                    severity: f.severity,
                    code: f.code,
                    path: f.path,
                    message: f.message,
                    suggested_fix: f.suggested_fix,
                }),
        );
    }

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

            // Permit gate: governed staged paths require valid permit
            let config = Store::read_repo_config(repo)?;
            let (permit_required, permit_exempt) = if let Some(ref cfg) = config {
                let req = cfg.governance.as_ref().map(|g| g.permit_required.clone()).unwrap_or_default();
                let ex = cfg.governance.as_ref().map(|g| g.permit_exempt.clone()).unwrap_or_default();
                (req, ex)
            } else {
                (Vec::new(), Vec::new())
            };

            let staged_paths: Vec<PathBuf> = changed.iter().map(PathBuf::from).collect();
            let permits = Store::read_permits(repo)?;
            let logs = Store::read_logs(repo)?;
            let proofs = Store::read_proofs(repo)?;

            let gate_result = PermitGate::evaluate(
                &staged_paths,
                &permits,
                &logs,
                &proofs,
                &permit_required,
                &permit_exempt,
            );

            if !gate_result.ok {
                ok = false;
            }

            for finding in &gate_result.findings {
                findings.push(ValidationFinding {
                    severity: finding.severity.clone(),
                    code: finding.code.clone(),
                    path: finding.path.clone(),
                    message: finding.message.clone(),
                    suggested_fix: Some(finding.remedy.clone()),
                });
            }
        }
    }

    if external
        && GitIntegration::is_public_remote(repo)? {
            findings.push(ValidationFinding {
                severity: Severity::Warning,
                code: "PUBLIC_REMOTE".into(),
                path: None,
                message: "Repository has a public remote. Release warrant may be required.".into(),
                suggested_fix: Some("Run vjs release-warrant check".into()),
            });
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

fn cmd_local_ci(repo: &Path, json: bool) -> Result<(), KernelError> {
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

    // Step 6: Permit gate
    let config = Store::read_repo_config(repo)?;
    let (permit_required, permit_exempt) = if let Some(ref cfg) = config {
        let req = cfg.governance.as_ref().map(|g| g.permit_required.clone()).unwrap_or_default();
        let ex = cfg.governance.as_ref().map(|g| g.permit_exempt.clone()).unwrap_or_default();
        (req, ex)
    } else {
        (Vec::new(), Vec::new())
    };

    let staged_files = GitIntegration::read_staged_files(repo)?;
    let staged_paths: Vec<PathBuf> = staged_files.iter().map(PathBuf::from).collect();
    let permits = Store::read_permits(repo)?;
    let logs = Store::read_logs(repo)?;
    let proofs = Store::read_proofs(repo)?;

    let gate_result = PermitGate::evaluate(
        &staged_paths,
        &permits,
        &logs,
        &proofs,
        &permit_required,
        &permit_exempt,
    );

    let permit_gate_ok = gate_result.ok;
    steps.push(CiStep {
        name: "permit_gate".into(),
        passed: permit_gate_ok,
        message: if permit_gate_ok {
            format!("{} staged paths, permit gate passed", staged_paths.len())
        } else {
            let failures: Vec<_> = gate_result.findings.iter()
                .filter(|f| matches!(f.severity, Severity::Fatal | Severity::Error))
                .map(|f| f.code.clone()).collect();
            format!("Permit gate failures: {}", failures.join(", "))
        },
    });
    if !permit_gate_ok {
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

fn cmd_order(repo: &Path, subcmd: OrderCommands, json: bool) -> Result<(), KernelError> {
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
    repo: &Path,
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

fn cmd_status(repo: &Path, json: bool) -> Result<(), KernelError> {
    let git_root = GitIntegration::find_repo_root(repo)?;
    let is_git = git_root.is_some();
    let is_public = if is_git {
        GitIntegration::is_public_remote(git_root.as_deref().unwrap_or(repo)).unwrap_or(false)
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

    let permits = if vjs_installed {
        Store::read_permits(repo)?
    } else {
        Vec::new()
    };

    let proofs = if vjs_installed {
        Store::read_proofs(repo)?.len()
    } else {
        0
    };

    let active_permits = permits.iter().filter(|p| matches!(p.status, PermitStatus::Active)).count();
    let closed_permits = permits.iter().filter(|p| matches!(p.status, PermitStatus::Closed)).count();

    let status = StatusInfo {
        repo: repo.display().to_string(),
        git_repo: is_git,
        public_remote: is_public,
        vjs_installed,
        lawpack: lawpack_info,
        logs_count: logs,
        orders_count: orders,
        permits_count: permits.len(),
        active_permits_count: active_permits,
        closed_permits_count: closed_permits,
        proofs_count: proofs,
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
        println!("Permits: {} total, {} active, {} closed", status.permits_count, status.active_permits_count, status.closed_permits_count);
        println!("Proofs: {}", status.proofs_count);
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

fn cmd_migrate_v1(_v1_path: &Path, out: Option<PathBuf>, json: bool) -> Result<(), KernelError> {
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

fn cmd_eval(repo: &Path, suite: Option<String>, json: bool) -> Result<(), KernelError> {
    let suite = suite.unwrap_or_else(|| "all".into());
    let lawpack = load_lawpack(repo)?;
    // The route suite needs a kernel context; build it best-effort.
    let ctx = build_kernel_context(repo).ok();
    let reports = vjs_core::evals::run_suite(&suite, &lawpack.invariants, ctx.as_ref(), repo);

    let total_failed: usize = reports.iter().map(|r| r.failed).sum();
    let total_passed: usize = reports.iter().map(|r| r.passed).sum();

    if json {
        println!("{}", serde_json::to_string_pretty(&reports).unwrap());
    } else if reports.is_empty() {
        println!("No eval suite matched '{}'. Try: agent-harness | prompts | route | all", suite);
    } else {
        for report in &reports {
            println!(
                "suite {}: {} passed, {} failed",
                report.suite, report.passed, report.failed
            );
            for c in &report.results {
                let mark = if c.passed { "PASS" } else { "FAIL" };
                println!("  [{}] {} - {}", mark, c.case, c.description);
                if !c.passed {
                    println!("        expected {}, got {}", c.expected, c.actual);
                    if let Some(fix) = &c.fix {
                        println!("        fix: {}", fix);
                    }
                }
            }
        }
        println!("TOTAL: {} passed, {} failed", total_passed, total_failed);
    }

    if total_failed > 0 {
        std::process::exit(1);
    }
    Ok(())
}

fn build_kernel_context(repo: &Path) -> Result<KernelContext, KernelError> {
    let lawpack = load_lawpack(repo)?;
    let graph = lawpack.build_authority_graph()?;
    let digest = compute_digest(repo)?;

    Ok(KernelContext {
        authority_graph: graph,
        limits: ContextLimits::default(),
        lawpack_digest: digest,
    })
}

/// Publish the Gazette data file: every law object of the V2 canon (walked
/// from the lawpack so nothing is hand-curated out of the record) plus the
/// curated V1 archive estate, with editorial summaries and citation edges
/// overlaid where the provenance file carries them. Edges to ids that do not
/// resolve to an item are dropped, so the graph can never link to a non-item.
fn cmd_gazette(repo: &Path, out: Option<PathBuf>, json: bool) -> Result<(), KernelError> {
    const V2_BASE: &str = "https://github.com/wlilley93/vibe-justice-system/blob/master/";
    const V1_BASE: &str = "https://github.com/wlilley93/vibe-justice-system/blob/v1/";

    let lawpack_dir = repo.join("lawpack/v2");
    // The Gazette publishes only a loadable canon.
    let _ = LawpackLoader::load(&lawpack_dir)?;

    let io = |e: std::io::Error| KernelError::Io(e.to_string());
    let ser = |e: serde_yaml::Error| KernelError::Serialization(e.to_string());

    fn s(v: &serde_yaml::Value, key: &str) -> Option<String> {
        v.get(key).and_then(|x| x.as_str()).map(|x| x.trim().to_string())
    }
    fn str_list(v: &serde_yaml::Value, key: &str) -> Vec<String> {
        v.get(key)
            .and_then(|x| x.as_sequence())
            .map(|seq| seq.iter().filter_map(|i| i.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default()
    }
    fn first_sentence(text: &str) -> String {
        let collapsed = text.split_whitespace().collect::<Vec<_>>().join(" ");
        match collapsed.find(". ") {
            Some(i) => collapsed[..i + 1].to_string(),
            None => collapsed,
        }
    }
    fn humanize(token: &str) -> String {
        token.replace('_', " ")
    }
    fn yaml_to_json(v: &serde_yaml::Value) -> serde_json::Value {
        serde_json::to_value(v).unwrap_or(serde_json::Value::Null)
    }
    /// Pick the whitelisted keys of a YAML mapping into a JSON object,
    /// verbatim (parsed scalars keep their paragraph breaks; no summarising).
    fn pick(v: &serde_yaml::Value, keys: &[&str]) -> serde_json::Value {
        let mut obj = serde_json::Map::new();
        for k in keys {
            if let Some(x) = v.get(k) {
                if !x.is_null() {
                    obj.insert(k.to_string(), yaml_to_json(x));
                }
            }
        }
        serde_json::Value::Object(obj)
    }
    /// The full-text body of a law object, by kind. This is what the in-place
    /// reader renders; only whitelisted fields leave the lawpack.
    fn text_body(kind: &str, v: &serde_yaml::Value) -> serde_json::Value {
        match kind {
            "statute" => {
                let mut body = pick(v, &["purpose"]);
                let sections: Vec<serde_json::Value> = v
                    .get("sections")
                    .and_then(|x| x.as_sequence())
                    .map(|secs| {
                        secs.iter()
                            .map(|sec| pick(sec, &["id", "title", "text", "commentary", "kernel_effect"]))
                            .collect()
                    })
                    .unwrap_or_default();
                body["sections"] = serde_json::Value::Array(sections);
                body
            }
            "regulation" => pick(v, &["authority", "text", "kernel_effect"]),
            "order" => pick(v, &[
                "holding", "directives", "forbidden", "exceptions", "runtime_summary", "source_opinion",
            ]),
            "decision" => pick(v, &["decision", "reason", "basis", "consequences", "review_triggers", "scope"]),
            "invariant" => pick(v, &["severity", "rule", "remedy", "basis"]),
            "obligation" => pick(v, &["text", "kind", "due", "required", "basis"]),
            "spec" => pick(v, &["purpose", "scope", "decisions", "invariants", "obligations", "review_triggers"]),
            "rule" => pick(v, &["summary", "effect", "scope", "exceptions", "rank", "source"]),
            _ => serde_json::Value::Null,
        }
    }
    /// Every law id and neutral citation the text of a record actually
    /// mentions: this is how a case interweaves by its subject. Negated
    /// mentions ("no DEC-X") are statements, not references (the same rule
    /// as the integrity checker).
    fn textual_refs(content: &str) -> Vec<String> {
        let mut out = Vec::new();
        let id_re = regex::Regex::new(
            r"\b((?:ACT|DEC|INV|OBL|SPEC|REG)-[A-Z0-9][A-Za-z0-9-]*[A-Za-z0-9](?::s\d+)?)",
        )
        .expect("static regex");
        let cite_re = regex::Regex::new(r"\[(\d{4})\]\s+(VJS|REALM)-([A-Z]{2})\s+(\d+)")
            .expect("static regex");
        for line in content.lines() {
            for m in id_re.find_iter(line) {
                if !line[..m.start()].trim_end().ends_with("no") {
                    out.push(m.as_str().to_string());
                }
            }
            for c in cite_re.captures_iter(line) {
                let (year, realm, court, n) = (&c[1], &c[2], &c[3], &c[4]);
                out.push(match realm {
                    // "[2026] VJS-PC 5" -> the order id "2026-VJS-PC-005"
                    "VJS" => format!("{}-VJS-{}-{:0>3}", year, court, n),
                    // "[2026] REALM-SC 10" -> the archive item id "REALM-SC-10"
                    _ => format!("REALM-{}-{}", court, n),
                });
            }
        }
        out
    }
    /// "[2026] VJS-PC 5" from an id like "2026-VJS-PC-005"; None otherwise.
    fn derive_order_citation(id: &str) -> Option<String> {
        let parts: Vec<&str> = id.split('-').collect();
        if parts.len() == 4
            && parts[0].len() == 4
            && parts[0].chars().all(|c| c.is_ascii_digit())
            && parts[1] == "VJS"
            && matches!(parts[2], "SC" | "PC" | "CC")
        {
            let n: u32 = parts[3].parse().ok()?;
            return Some(format!("[{}] VJS-{} {}", parts[0], parts[2], n));
        }
        None
    }

    // Editorial overlay: presentation copy only, never force.
    #[derive(serde::Deserialize, Default)]
    struct Editorial {
        #[serde(default)]
        summary: String,
        #[serde(default)]
        points: Vec<String>,
        #[serde(default)]
        cites: Vec<String>,
    }
    let editorial: std::collections::HashMap<String, Editorial> = {
        let p = lawpack_dir.join("provenance/gazette/editorial.yaml");
        if p.exists() {
            let v: serde_yaml::Value =
                serde_yaml::from_str(&std::fs::read_to_string(&p).map_err(io)?).map_err(ser)?;
            serde_yaml::from_value(v.get("items").cloned().unwrap_or_default()).map_err(ser)?
        } else {
            Default::default()
        }
    };

    // First-enacted dates from history: one pass over git log, oldest first,
    // so the first add wins. (Orders carry created_at and prefer it.)
    let mut added_at: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    if let Ok(out) = std::process::Command::new("git")
        .args([
            "-C",
            &repo.to_string_lossy(),
            "log",
            "--reverse",
            "--diff-filter=A",
            "--name-only",
            "--format=\u{1}%cI",
        ])
        .output()
    {
        let text = String::from_utf8_lossy(&out.stdout);
        let mut current = String::new();
        for line in text.lines() {
            if let Some(ts) = line.strip_prefix('\u{1}') {
                current = ts.split('T').next().unwrap_or("").to_string();
            } else if !line.is_empty() && !current.is_empty() {
                added_at.entry(line.to_string()).or_insert_with(|| current.clone());
            }
        }
    }

    let mut items: Vec<serde_json::Value> = Vec::new();
    // Full-text bodies for the in-place reader, id -> kind-specific body.
    let mut texts: std::collections::BTreeMap<String, serde_json::Value> =
        std::collections::BTreeMap::new();

    let kinds = [
        ("statutes", "statute"),
        ("regulations", "regulation"),
        ("rules", "rule"),
        ("orders", "order"),
        ("specs", "spec"),
        ("invariants", "invariant"),
        ("decisions", "decision"),
        ("obligations", "obligation"),
    ];
    for (dir, kind) in kinds {
        let d = lawpack_dir.join(dir);
        if !d.exists() {
            continue;
        }
        let mut entries: Vec<_> = std::fs::read_dir(&d)
            .map_err(io)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("yaml"))
            .collect();
        entries.sort();
        for path in entries {
            let content = std::fs::read_to_string(&path).map_err(io)?;
            let v: serde_yaml::Value = serde_yaml::from_str(&content).map_err(ser)?;
            let id = match s(&v, "id") {
                Some(id) => id,
                None => continue,
            };
            let title = s(&v, "title").unwrap_or_else(|| id.clone());
            let citation = s(&v, "citation").unwrap_or_default();
            let status = s(&v, "status")
                .or_else(|| s(&v, "severity").map(|sev| format!("severity {}", sev)))
                .unwrap_or_default();
            let court = match kind {
                "order" => match s(&v, "court").unwrap_or_default().as_str() {
                    "supreme_court" => "sc",
                    "privy_council" => "pc",
                    "county" => "county",
                    _ => "",
                },
                _ => "",
            }
            .to_string();

            // Mechanical fallbacks straight from the law text.
            let summary_field = match kind {
                "statute" | "spec" => "purpose",
                "regulation" | "obligation" => "text",
                "order" => "runtime_summary",
                "decision" => "decision",
                "invariant" => "remedy",
                "rule" => "summary",
                _ => "",
            };
            let mech_summary = s(&v, summary_field)
                .map(|t| first_sentence(&t))
                .unwrap_or_else(|| title.clone());
            let mech_points: Vec<String> = match kind {
                "statute" => v
                    .get("sections")
                    .and_then(|x| x.as_sequence())
                    .map(|secs| {
                        secs.iter()
                            .filter_map(|sec| {
                                let sid = sec.get("id")?.as_str()?;
                                let stitle = sec.get("title")?.as_str()?;
                                let n = sid.rsplit(':').next().unwrap_or(sid);
                                Some(format!("{} - {}", n, stitle))
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
                "order" => v
                    .get("directives")
                    .and_then(|x| x.as_sequence())
                    .map(|ds| {
                        ds.iter()
                            .filter_map(|d| d.get("must")?.as_str().map(humanize))
                            .collect()
                    })
                    .unwrap_or_default(),
                "decision" => v
                    .get("consequences")
                    .map(|c| str_list(c, "must").iter().map(|m| humanize(m)).collect())
                    .unwrap_or_default(),
                "regulation" => v
                    .get("kernel_effect")
                    .map(|k| {
                        let mut p: Vec<String> =
                            str_list(k, "must").iter().map(|m| humanize(m)).collect();
                        p.extend(str_list(k, "must_not").iter().map(|m| format!("never {}", humanize(m))));
                        p.extend(str_list(k, "prohibits").iter().map(|m| format!("prohibits {}", humanize(m))));
                        p
                    })
                    .unwrap_or_default(),
                "obligation" => {
                    let mut p = Vec::new();
                    if let Some(k) = s(&v, "kind") {
                        p.push(format!("performed by a {}", humanize(&k)));
                    }
                    if let Some(d) = s(&v, "due") {
                        p.push(format!("due {}", humanize(&d)));
                    }
                    p
                }
                "spec" => {
                    let mut p = Vec::new();
                    for key in ["decisions", "invariants", "obligations"] {
                        for r in str_list(&v, key) {
                            p.push(format!("carried by {}", r));
                        }
                    }
                    p
                }
                _ => Vec::new(),
            };

            // Mechanical citation edges: the law's own fields, plus every id
            // and neutral citation its text actually mentions (the subject
            // linkage; a case connects to the legislation it construes).
            let mut cites: Vec<String> = str_list(&v, "basis");
            cites.extend(str_list(&v, "supersedes"));
            if let Some(a) = s(&v, "authority") {
                cites.push(a);
            }
            for key in ["decisions", "invariants", "obligations"] {
                cites.extend(str_list(&v, key));
            }
            cites.extend(textual_refs(&content));

            let ed = editorial.get(&id);
            let summary = ed.filter(|e| !e.summary.is_empty()).map(|e| e.summary.clone()).unwrap_or(mech_summary);
            let points = ed
                .filter(|e| !e.points.is_empty())
                .map(|e| e.points.clone())
                .unwrap_or(mech_points);
            if let Some(e) = ed {
                cites.extend(e.cites.clone());
            }
            cites.sort();
            cites.dedup();
            cites.retain(|c| *c != id);

            let rel = format!("lawpack/v2/{}/{}", dir, path.file_name().unwrap().to_string_lossy());
            let date = s(&v, "created_at")
                .map(|c| c.split('T').next().unwrap_or("").trim_matches('"').to_string())
                .filter(|c| !c.is_empty())
                .or_else(|| added_at.get(&rel).cloned())
                .unwrap_or_default();
            let citation = if citation.is_empty() && kind == "order" {
                derive_order_citation(&id).unwrap_or_default()
            } else {
                citation
            };
            let mut item = serde_json::json!({
                "id": id, "title": title, "citation": citation, "kind": kind,
                "court": court, "estate": "v2", "status": status, "date": date,
                "summary": summary, "points": points, "cites": cites,
                "supersedes": str_list(&v, "supersedes"),
                "has_text": true,
                "path": rel, "url": format!("{}{}", V2_BASE, rel),
            });
            // A case's subject: the problem it was defined against.
            if kind == "order" {
                if let Some(issue) = s(&v, "issue").filter(|i| !i.is_empty()) {
                    item["subject"] = serde_json::Value::String(humanize(&issue));
                }
                if let Some(op) = s(&v, "source_opinion").filter(|p| !p.is_empty()) {
                    item["opinion"] = serde_json::json!({
                        "path": op,
                        "url": format!("{}{}", V2_BASE, op.trim_start_matches("./")),
                    });
                }
            }
            items.push(item);
            texts.insert(items.last().unwrap()["id"].as_str().unwrap().to_string(), text_body(kind, &v));
        }
    }

    // The V1 archive estate: curated, frozen, existence-verified provenance.
    let v1_path = lawpack_dir.join("provenance/gazette/v1-estate.yaml");
    if v1_path.exists() {
        let v: serde_yaml::Value =
            serde_yaml::from_str(&std::fs::read_to_string(&v1_path).map_err(io)?).map_err(ser)?;
        if let Some(seq) = v.get("items").and_then(|x| x.as_sequence()) {
            for it in seq {
                let path = s(it, "path").unwrap_or_default();
                items.push(serde_json::json!({
                    "id": s(it, "id").unwrap_or_default(),
                    "title": s(it, "title").unwrap_or_default(),
                    "citation": s(it, "citation").unwrap_or_default(),
                    "kind": s(it, "kind").unwrap_or_default(),
                    "court": s(it, "court").unwrap_or_default(),
                    "estate": "v1",
                    "status": s(it, "status").unwrap_or_default(),
                    "date": s(it, "date").unwrap_or_default(),
                    "summary": s(it, "summary").unwrap_or_default(),
                    "points": str_list(it, "points"),
                    "cites": str_list(it, "cites"),
                    "supersedes": [],
                    "has_text": false,
                    "path": path,
                    "url": format!("{}{}", V1_BASE, path),
                }));
            }
        }
    }

    // Authority lineage: the force chain, derived from the enacted structure
    // rather than textual citation. A record that never quotes the founding
    // act still holds its force through it; the Gazette shows that descent.
    // Per-kind anchors (skipped when the anchor is the item itself):
    //   canon:   statute -> the founding act (which itself traces to the
    //            assented Bill 32 in the archive); regulation -> its parent
    //            act (the textual `authority` already carries this); rule,
    //            decision, invariant, obligation, spec -> the Constitution
    //            and Sources of Authority Act (which constitutes those
    //            categories); order -> the courts-constitution order.
    //   archive: act -> the Acts of Union; instrument -> the SI-delegation
    //            act; judgment -> the courts/citations act.
    const FOUNDING_ACT: &str = "ACT-COMPUTER-FIRST-REALM";
    const SOURCES_ACT: &str = "ACT-001";
    const COURTS_ORDER: &str = "2026-VJS-COURTS-CONSTITUTION-001";
    const V1_UNION: &str = "BILL-1";
    const V1_SI_ACT: &str = "BILL-14";
    const V1_COURTS_ACT: &str = "BILL-16";
    const V1_FOUNDING_BILL: &str = "BILL-32";
    fn lineage_anchor(estate: &str, kind: &str, id: &str) -> Option<&'static str> {
        let anchor = match (estate, kind) {
            ("v2", "statute") => {
                if id == FOUNDING_ACT { V1_FOUNDING_BILL } else { FOUNDING_ACT }
            }
            ("v2", "regulation") => "", // its authority field already names the parent
            ("v2", "order") => COURTS_ORDER,
            ("v2", _) => SOURCES_ACT,
            ("v1", "act") => V1_UNION,
            ("v1", "instrument") => V1_SI_ACT,
            ("v1", "judgment") => V1_COURTS_ACT,
            _ => "",
        };
        (!anchor.is_empty() && anchor != id).then_some(anchor)
    }

    // An edge may only point at an item: a section cite collapses to its
    // parent act's item; anything else unresolved is dropped.
    let known: std::collections::HashSet<String> = items
        .iter()
        .map(|i| i["id"].as_str().unwrap_or_default().to_string())
        .collect();
    // The docket thread: successive cases on the same subject (the order's
    // issue tag) chain chronologically. This is how cases interweave when
    // they cite no legislation textually: by the problem they were defined
    // against.
    {
        // Issue tags are unique per case but carry family structure:
        // "governance.x" / "constitutional.x" are dotted dockets, and the
        // "vjs-v2-*" tags are the boot-series docket. The family is the
        // thread; the full issue stays on the item as its subject.
        fn subject_family(issue: &str) -> String {
            if let Some((fam, _)) = issue.split_once('.') {
                return fam.to_string();
            }
            if issue.starts_with("vjs-v2") {
                return "vjs-v2".into();
            }
            issue.to_string()
        }
        let mut by_subject: std::collections::BTreeMap<String, Vec<(String, String, usize)>> =
            std::collections::BTreeMap::new();
        for (idx, item) in items.iter().enumerate() {
            if item["kind"] == "order" {
                if let Some(subj) = item["subject"].as_str() {
                    by_subject.entry(subject_family(subj)).or_default().push((
                        item["date"].as_str().unwrap_or_default().to_string(),
                        item["id"].as_str().unwrap_or_default().to_string(),
                        idx,
                    ));
                }
            }
        }
        for (_, mut chain) in by_subject {
            chain.sort();
            for w in chain.windows(2) {
                let (prev_id, idx) = (&w[0].1, w[1].2);
                items[idx]["thread"] = serde_json::json!([prev_id]);
            }
        }
    }

    let resolve = |list: &[String], own_id: &str, known: &std::collections::HashSet<String>| {
        let mut out: Vec<String> = list
            .iter()
            .filter_map(|c| {
                if known.contains(c) {
                    Some(c.clone())
                } else {
                    let parent = c.split(':').next().unwrap_or(c);
                    known.contains(parent).then(|| parent.to_string())
                }
            })
            .filter(|c| c != own_id)
            .collect();
        out.sort();
        out.dedup();
        out
    };

    let mut dropped = 0usize;
    let mut superseded_by: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    for item in &mut items {
        let own_id = item["id"].as_str().unwrap_or_default().to_string();
        let estate = item["estate"].as_str().unwrap_or_default().to_string();
        let kind = item["kind"].as_str().unwrap_or_default().to_string();

        let raw_cites: Vec<String> = item["cites"]
            .as_array()
            .map(|a| a.iter().filter_map(|c| c.as_str().map(String::from)).collect())
            .unwrap_or_default();
        let resolved = resolve(&raw_cites, &own_id, &known);
        dropped += raw_cites.len().saturating_sub(resolved.len());
        item["cites"] = serde_json::Value::Array(
            resolved.iter().cloned().map(serde_json::Value::String).collect(),
        );

        let raw_sup: Vec<String> = item["supersedes"]
            .as_array()
            .map(|a| a.iter().filter_map(|c| c.as_str().map(String::from)).collect())
            .unwrap_or_default();
        let sup = resolve(&raw_sup, &own_id, &known);
        for target in &sup {
            superseded_by.entry(target.clone()).or_default().push(own_id.clone());
        }
        item["supersedes"] = serde_json::Value::Array(
            sup.into_iter().map(serde_json::Value::String).collect(),
        );

        // A case interweaves by its SUBJECT: the legislation it construes
        // (resolved citations) or the docket thread on its issue. The
        // constitutional anchor is a last resort against orphaning only.
        let case_like = kind == "order" || kind == "judgment";
        let threaded = item["thread"].as_array().map(|a| !a.is_empty()).unwrap_or(false);
        let anchor = if case_like && (!resolved.is_empty() || threaded) {
            None
        } else {
            lineage_anchor(&estate, &kind, &own_id).or_else(|| {
                // A regulation normally hangs off its textual `authority`; if
                // that failed to resolve it still holds force under the SI power.
                (estate == "v2" && kind == "regulation" && resolved.is_empty())
                    .then_some("ACT-CONSOLIDATION-FRAMEWORK")
            })
        };
        let lineage: Vec<serde_json::Value> = anchor
            .filter(|a| known.contains(*a) && !resolved.contains(&a.to_string()))
            .map(|a| serde_json::Value::String(a.to_string()))
            .into_iter()
            .collect();
        item["lineage"] = serde_json::Value::Array(lineage);
    }
    for item in &mut items {
        let own_id = item["id"].as_str().unwrap_or_default();
        let mut sb = superseded_by.get(own_id).cloned().unwrap_or_default();
        sb.sort();
        sb.dedup();
        item["superseded_by"] =
            serde_json::Value::Array(sb.into_iter().map(serde_json::Value::String).collect());
    }

    let data = serde_json::json!({
        "generated_at": chrono::Utc::now().to_rfc3339(),
        "items": items,
    });
    // A `</` inside law text would terminate the host <script> tag; emit the
    // JSON escape `<\/` instead (identical parse, inert markup).
    let guard = |s: String| s.replace("</", "<\\/");
    let out_path = out.unwrap_or_else(|| repo.join("gazette-data.js"));
    let body = format!(
        "// Generated by `vjs gazette`. Do not edit: regenerate from the lawpack.\nwindow.GAZETTE = {};\n",
        guard(serde_json::to_string_pretty(&data).expect("gazette data serializes"))
    );
    std::fs::write(&out_path, body).map_err(io)?;

    let text_path = out_path.with_file_name("gazette-text.js");
    let text_body_js = format!(
        "// Generated by `vjs gazette`. Full law text for the in-place reader.\nwindow.GAZETTE_TEXT = {};\n",
        guard(serde_json::to_string(&texts).expect("gazette text serializes"))
    );
    std::fs::write(&text_path, &text_body_js).map_err(io)?;

    if json {
        println!(
            "{}",
            serde_json::json!({
                "out": out_path.to_string_lossy(),
                "text_out": text_path.to_string_lossy(),
                "text_bytes": text_body_js.len(),
                "items": known.len(),
                "edges_dropped_to_non_items": dropped,
            })
        );
    } else {
        println!("Gazette data: {} items -> {}", known.len(), out_path.display());
        println!("  full text: {} bodies ({} KB) -> {}", texts.len(), text_body_js.len() / 1024, text_path.display());
        println!("  citation edges to non-items dropped: {}", dropped);
    }
    Ok(())
}

fn load_lawpack(repo: &Path) -> Result<Lawpack, KernelError> {
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
            obligations: Vec::new(),
        })
    }
}

fn compute_digest(repo: &Path) -> Result<String, KernelError> {
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
    permits_count: usize,
    active_permits_count: usize,
    closed_permits_count: usize,
    proofs_count: usize,
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

fn cmd_permit(repo: &Path, subcmd: PermitCommands, json: bool) -> Result<(), KernelError> {
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
            let permit = permits
                .iter_mut()
                .find(|p| p.id.0 == id)
                .ok_or(KernelError::PermitNotFound(id.clone()))?;
            permit.status = PermitStatus::Closed;

            if let Some(proof_content) = proof {
                use sha2::Digest;
                let digest = format!("sha256:{}", hex::encode(sha2::Sha256::digest(proof_content.as_bytes())));
                let proof = Proof {
                    id: ProofId(format!("PROOF-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S"))),
                    permit_id: permit.id.clone(),
                    kind: ProofKind::DecisionLog,
                    status: ProofStatus::Passed,
                    digest: Some(digest),
                    captured_at: chrono::Utc::now().to_rfc3339(),
                };
                Store::write_proof(repo, &proof)?;
                if json {
                    println!("{}", serde_json::to_string_pretty(&proof).unwrap());
                }
            }

            Store::write_permit(repo, permit)?;
            if json {
                println!("{{ \"ok\": true, \"permit_id\": \"{}\" }}", id);
            } else {
                println!("Permit {} closed", id);
            }
            Ok(())
        }
    }
}
