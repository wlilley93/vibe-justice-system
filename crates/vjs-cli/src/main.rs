use chrono::Datelike;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use vjs_core::*;
use vjs_git::*;
use vjs_lawpack::*;
use vjs_redact::*;
use vjs_store::*;

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
        /// Read a Claude Code hook payload from stdin and take the written
        /// file path from it (tool_input.file_path / notebook_path).
        #[arg(long)]
        stdin_json: bool,
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
    /// (Re)write the atomic install manifest .vjs/install.lock over the current
    /// surface (PC-13 D5). Run after a deliberate surface change to re-lock it.
    InstallLock,
    /// Full-spectrum conformance audit (PC-13 D11): enumerate every kernel_effect
    /// duty in canon and report, deterministically, which are bound to a kernel
    /// gate. Writes the conformance map (the D12 predicate).
    Audit {
        /// Write the map here (default docs/conformance-map.md).
        #[arg(long)]
        out: Option<PathBuf>,
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
    /// The court's auditable surface: list the docket, or record a convening.
    Court {
        #[command(subcommand)]
        subcmd: CourtCommands,
    },
    /// Verify a deployment bundle.lock against REG-BUNDLE-001 (fail-closed).
    Bundle {
        #[command(subcommand)]
        subcmd: BundleCommands,
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
    Validate { path: PathBuf },
    Apply { path: PathBuf },
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
enum CourtCommands {
    /// List the docket: filed submissions and issued orders, grouped by issue.
    Docket,
    /// Record a convening: pin the sha256 of a filed submission (the symmetric
    /// case file) and the bench that decided it.
    Record {
        #[arg(long)]
        court: String,
        /// The filed submission id whose bytes are the case file.
        #[arg(long)]
        submission: String,
        /// A deciding seat (repeat for each bench member).
        #[arg(long = "seat")]
        bench: Vec<String>,
        #[arg(long)]
        issue: Option<String>,
    },
}

#[derive(Subcommand)]
enum BundleCommands {
    /// Verify a bundle.lock: schema completeness, sha256 well-formedness, and the
    /// AGPL/MIT licence firewall. Fails closed on the first violation.
    Verify {
        /// Path to the bundle.lock manifest.
        path: PathBuf,
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
            eprintln!(
                "Error: cannot determine current directory: {} (pass --repo)",
                e
            );
            std::process::exit(1);
        })
    });
    let json = cli.json;

    let result = match cli.command {
        Commands::Init { lawpack } => cmd_init(&repo, lawpack),
        Commands::Route {
            kind,
            issue,
            risk,
            intent,
            public,
            external,
            irreversible,
            paths,
            gate,
        } => cmd_route(
            &repo,
            kind,
            issue,
            risk,
            intent,
            public,
            external,
            irreversible,
            paths,
            gate,
            json,
        ),
        Commands::Hook {
            event,
            paths,
            tool,
            stdin_json,
        } => cmd_hook(&repo, event, paths, tool, stdin_json, json),
        Commands::Invoke {
            jurisdiction,
            principal,
            lawpack,
            install_hooks,
        } => cmd_invoke(&repo, jurisdiction, principal, lawpack, install_hooks, json),
        Commands::Lookup { issue, limit } => cmd_lookup(&repo, issue, limit, json),
        Commands::Log { subcmd } => cmd_log(&repo, subcmd, json),
        Commands::Proof { subcmd } => cmd_proof(&repo, subcmd, json),
        Commands::Validate {
            staged,
            external,
            scope,
        } => cmd_validate(&repo, staged, external, scope, json),
        Commands::LocalCi => cmd_local_ci(&repo, json),
        Commands::Order { subcmd } => cmd_order(&repo, subcmd, json),
        Commands::Court { subcmd } => cmd_court(&repo, subcmd, json),
        Commands::Bundle { subcmd } => cmd_bundle(&repo, subcmd, json),
        Commands::File {
            court,
            question,
            facts_file,
        } => cmd_file(&repo, court, question, facts_file, json),
        Commands::Status => cmd_status(&repo, json),
        Commands::NextCitation { series, year } => cmd_next_citation(&repo, series, year, json),
        Commands::InstallLock => cmd_install_lock(&repo, json),
        Commands::Audit { out } => cmd_audit(&repo, out, json),
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
        std::fs::write(&agents_md, content).map_err(|e| KernelError::Io(e.to_string()))?;
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
    let path_globs: Vec<String> = paths
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();

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
            route_id: RouteId(format!(
                "ROUTE-{}",
                chrono::Utc::now().format("%Y%m%d-%H%M%S")
            )),
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
                eprintln!(
                    "BLOCK: court required - convene on own motion; do not route the fork to the Principal."
                );
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
    mut paths: Vec<PathBuf>,
    tool: Option<String>,
    stdin_json: bool,
    json: bool,
) -> Result<(), KernelError> {
    let hook_event = vjs_core::hook::parse_event(&event)
        .ok_or_else(|| KernelError::InvalidInput(format!("unknown hook event: {}", event)))?;
    if stdin_json {
        // The agent-harness payload: extract the written path, relativise it
        // to this jurisdiction; payloads without one (or outside it) pass.
        let mut buf = String::new();
        use std::io::Read;
        let _ = std::io::stdin().read_to_string(&mut buf);
        let payload: serde_json::Value = serde_json::from_str(&buf).unwrap_or_default();
        let file = payload["tool_input"]["file_path"]
            .as_str()
            .or_else(|| payload["tool_input"]["notebook_path"].as_str())
            .map(PathBuf::from);
        match file {
            Some(f) => {
                let abs = if f.is_absolute() { f } else { repo.join(f) };
                match abs.strip_prefix(repo) {
                    Ok(rel) => paths.push(rel.to_path_buf()),
                    Err(_) => return Ok(()), // outside this jurisdiction
                }
            }
            None => return Ok(()),
        }
    }
    let input = vjs_core::hook::HookInput {
        event: hook_event,
        repo_root: repo.to_path_buf(),
        actor: "lexby".into(),
        paths,
        tool,
    };
    let ctx = build_kernel_context(repo)?;
    // Permit-aware: a governed write under an active in-scope permit passes;
    // an unpermitted one fails closed with the route as the remedy.
    let permits = Store::read_permits(repo).unwrap_or_default();
    let cfg = Store::read_repo_config(repo).ok().flatten();
    let jurisdiction_id = cfg
        .as_ref()
        .map(|c| c.jurisdiction_id.clone())
        .unwrap_or_default();
    let (required, exempt) = cfg
        .as_ref()
        .and_then(|c| c.governance.clone())
        .map(|g| (g.permit_required, g.permit_exempt))
        .unwrap_or_default();
    // REG-FEDERATION-COORDINATION-001 kernel-checkable bright-line (giving effect to [2026] VJS-SC 1): a
    // subscribing jurisdiction may not assert an apex/final-court function, i.e. may not record a supreme/privy
    // court order; it must refer up. The apex seat is the canonical VJS jurisdiction.
    const APEX_SEAT: &str = "vjs";
    let canon_repo_code = cfg
        .as_ref()
        .and_then(|c| c.repo_code.clone())
        .unwrap_or_else(|| jurisdiction_id.to_uppercase());
    // Canon-write gate ([2026] VJS-PC 13 D1), pre_write half: best-effort on content
    // (the file may not be on disk yet; the authoritative bite is validate --staged).
    let canon_block = |paths: &[PathBuf]| -> Option<vjs_core::hook::HookDecision> {
        let cf = RedactScanner::scan_canon_writes(repo, paths, &canon_repo_code);
        let first = cf
            .into_iter()
            .find(|x| matches!(x.severity, Severity::Error | Severity::Fatal))?;
        let where_ = first
            .path
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_default();
        Some(vjs_core::hook::HookDecision::Block(
            vjs_core::hook::Finding {
                code: "CANON_BOUNDARY_VIOLATION".into(),
                message: format!(
                    "{where_} carries subscriber-scoped content (private repo path or repo_code). \
                 Canon holds system data only; file it in the subscriber's own .justice/."
                ),
                next: Some("move to subscriber .justice/".into()),
            },
        ))
    };
    // PC-13 D4, pre_write half: fail closed on an incomplete install surface before
    // any governed write proceeds (REG-INVOCATION-001).
    let install_block = || -> Option<vjs_core::hook::HookDecision> {
        let d = vjs_core::install::verify_surface(repo).into_iter().next()?;
        Some(vjs_core::hook::HookDecision::Block(
            vjs_core::hook::Finding {
                code: d.code().into(),
                message: "Install incomplete: governance is only partly active. Run vjs invoke \
                      --install-hooks then vjs install-lock before governed writes."
                    .into(),
                next: Some("vjs invoke --install-hooks".into()),
            },
        ))
    };
    let decision = vjs_core::hook::apex_routing_decision(&input, &jurisdiction_id, APEX_SEAT)
        .or_else(install_block)
        .or_else(|| canon_block(&input.paths))
        .unwrap_or_else(|| {
            vjs_core::hook::evaluate_governed(&input, &ctx, &permits, &required, &exempt)
        });

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
        jur = jurisdiction,
        code = repo_code,
        lp = lawpack,
        prin = principal,
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

    // 2. lawpack.lock - pin the lawpack digest. Canonical fields matching the
    // one LawpackLock serde model (Bug A: the writer and reader no longer drift),
    // including schema_version for the load-time version handshake (Bug C).
    let lock = format!(
        "lawpack_id = \"{lp}\"\nlawpack_version = \"0.1.0\"\ndigest = \"{dig}\"\nschema_version = {sv}\ngenerated_at = \"{ts}\"\nlocked_by = \"{prin}\"\n",
        lp = lawpack,
        dig = digest,
        sv = vjs_store::LOCK_SCHEMA_VERSION,
        ts = now_rfc,
        prin = principal,
    );
    std::fs::write(vjs_dir.join("lawpack.lock"), lock).map_err(io)?;

    // 3. the local sovereign invocation record (the constitutional act).
    let inv = format!(
        "id: INVOCATION-{stamp}\nkind: local_sovereign_invocation\nstatus: in_force\njurisdiction:\n  id: {jur}\n  repo_root: \".\"\n  repo_code: {code}\nprincipal:\n  name: \"{prin}\"\n  capacity: local_sovereign\nsubscription:\n  lawpack: {lp}\n  lawpack_lock: .vjs/lawpack.lock\n  mode: subscribed\n  v1_archive_import: none_unless_expressly_incorporated\nassent:\n  given: true\n  form: local_sovereign_act\n  statement: >\n    The Principal invokes this repository as a VJS V2 local jurisdiction,\n    subscribes it to the stated lawpack, and authorises the kernel, hooks,\n    permits, proofs, logs, and court route to govern repo work.\neffect:\n  - creates_local_jurisdiction\n  - creates_county_court_for_repo\n  - binds_agents_to_kernel_route\n  - requires_permits_for_governed_writes\n  - requires_logs_for_material_decisions\n  - installs_validation_hooks\n",
        stamp = stamp,
        jur = jurisdiction,
        code = repo_code,
        prin = principal,
        lp = lawpack,
    );
    let inv_path = vjs_dir
        .join("invocation")
        .join(format!("{}-local-sovereign-invocation.yaml", stamp));
    std::fs::write(&inv_path, inv).map_err(io)?;

    // 4. install enforcement hooks (the activation): git core.hooksPath + tiny
    // extensionless wrappers git will run, made executable.
    let mut hooks_installed = false;
    if install_hooks {
        let hooks_dir = vjs_dir.join("hooks");
        std::fs::create_dir_all(&hooks_dir).map_err(io)?;
        // The bypass-proof wall (PC-13 D6, PC-14 REG-FRONT-DOOR-001): resolve the
        // kernel binary from the REPO ROOT so the gate survives a repo move and works
        // whether the binary was built by cargo (target/) or exported from the
        // server_of_law Docker image (bin/, REG-FRONT-DOOR-DELIVERY-001), falling back
        // to PATH. It NEVER depends on the MCP container being up - the container is
        // the door, this is the wall.
        let resolver = "root=\"$(git rev-parse --show-toplevel)\"\nbin=\"\"\nfor c in bin/vjs target/release/vjs target/debug/vjs; do\n  [ -x \"$root/$c\" ] && bin=\"$root/$c\" && break\ndone\n[ -n \"$bin\" ] || bin=vjs";
        std::fs::write(
            hooks_dir.join("pre-commit"),
            format!("#!/usr/bin/env bash\n{resolver}\nexec \"$bin\" validate --staged\n"),
        )
        .map_err(io)?;
        std::fs::write(
            hooks_dir.join("pre-push"),
            format!("#!/usr/bin/env bash\n{resolver}\nexec \"$bin\" local-ci\n"),
        )
        .map_err(io)?;
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
            .args([
                "-C",
                &repo.to_string_lossy(),
                "config",
                "core.hooksPath",
                ".vjs/hooks",
            ])
            .output();
        hooks_installed = out.map(|o| o.status.success()).unwrap_or(false);

        // PC-13 D8: emit the thin agent-runtime adapters (pre_write / session_start
        // / post_action) for every supported runtime. Each only calls the kernel
        // (REG-HOOKS-001 thin-adapter rule); the logic stays in hook.rs.
        let _ = vjs_core::install::generate_adapters(repo);
    }

    // PC-13 D5: atomically lock the surface into .vjs/install.lock. Best-effort -
    // if hooks were not installed this run, the surface is incomplete and the lock
    // is deferred to a later `vjs install-lock`; the completeness invariant (D4)
    // fails closed until it exists.
    let manifest_locked = vjs_core::install::build_manifest(repo, now_rfc.clone())
        .and_then(|m| {
            let body = toml::to_string(&m).ok()?;
            let header = "# VJS install manifest (REG-INSTALL-MANIFEST-001).\n";
            std::fs::write(
                repo.join(vjs_core::install::MANIFEST_FILE),
                format!("{header}{body}"),
            )
            .ok()
        })
        .is_some();

    if json {
        println!(
            "{}",
            serde_json::json!({
                "jurisdiction": jurisdiction,
                "repo_code": repo_code,
                "lawpack": lawpack,
                "lawpack_digest": digest,
                "invocation": inv_path.to_string_lossy(),
                "config_written": config_written,
                "hooks_installed": hooks_installed,
                "manifest_locked": manifest_locked,
            })
        );
    } else {
        println!(
            "Invoked '{}' as a VJS jurisdiction (repo_code {}).",
            jurisdiction, repo_code
        );
        println!(
            "  lawpack: {} ({}...)",
            lawpack,
            &digest[..digest.len().min(23)]
        );
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

/// A decision-log id that does not collide with an existing log. The base is
/// second-precision; on collision (two logs written in the same second, as a single
/// command can) a -2/-3 suffix is appended, so a record is never silently
/// overwritten by a sibling sharing its timestamp.
fn unique_log_id(repo: &Path) -> String {
    let base = format!("LOG-{}", chrono::Utc::now().format("%Y-%m-%d-%H%M%S"));
    let dir = repo.join(".vjs/logs/decisions");
    let mut id = base.clone();
    let mut n = 2;
    while dir.join(format!("{id}.yaml")).exists() {
        id = format!("{base}-{n}");
        n += 1;
    }
    id
}

fn cmd_log(repo: &Path, subcmd: LogCommands, json: bool) -> Result<(), KernelError> {
    match subcmd {
        LogCommands::Decision {
            kind,
            issue,
            decision,
            basis,
            risk,
            why,
        } => {
            let log = DecisionLog {
                id: unique_log_id(repo),
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
        LogCommands::FromPermit {
            permit_id,
            decision,
            why,
        } => {
            let permits = Store::read_permits(repo)?;
            let permit = permits
                .into_iter()
                .find(|p| p.id.0 == permit_id)
                .ok_or_else(|| KernelError::PermitNotFound(permit_id.clone()))?;

            // A log may only be written from a LIVE permit: active and
            // unexpired, failing closed on an unparseable expiry (the gate's
            // rule). Defect class this closes: a route that returns
            // court_required issues no permit, and a caller that then grabs
            // the newest permit file silently links the log to a stale,
            // unrelated permit (LOG-2026-06-11-111946 records the instance).
            if !matches!(permit.status, PermitStatus::Active) {
                return Err(KernelError::InvalidInput(format!(
                    "permit {} is {:?}, not active; a decision log may only be written from a live permit",
                    permit_id, permit.status
                )));
            }
            let expired = match chrono::DateTime::parse_from_rfc3339(&permit.expires_at) {
                Ok(expiry) => chrono::Utc::now() >= expiry.with_timezone(&chrono::Utc),
                Err(_) => true,
            };
            if expired {
                return Err(KernelError::InvalidInput(format!(
                    "permit {} is expired (expires_at {}); route again for a live permit",
                    permit_id, permit.expires_at
                )));
            }

            let log = DecisionLog {
                id: unique_log_id(repo),
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
                println!(
                    "Decision log written: {} (from permit {})",
                    log.id, permit_id
                );
            }
            Ok(())
        }
    }
}

fn cmd_proof(repo: &Path, subcmd: ProofCommands, json: bool) -> Result<(), KernelError> {
    match subcmd {
        ProofCommands::Add {
            permit_id,
            kind,
            status,
        } => {
            let proof_id = ProofId(format!(
                "PROOF-{}",
                chrono::Utc::now().format("%Y%m%d-%H%M%S")
            ));
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

        // ACT-004:s8 (PC-13 D2): citation uniqueness, collisions fatal. The
        // reconciliation-at-write half - no two canon records may claim the same
        // citation, closing the hand-asserted-citation vector.
        let citation_findings = LawpackValidator::check_citation_uniqueness(&lawpack_dir)?;
        if citation_findings
            .iter()
            .any(|f| matches!(f.severity, Severity::Fatal | Severity::Error))
        {
            ok = false;
        }
        findings.extend(citation_findings.into_iter().map(|f| ValidationFinding {
            severity: f.severity,
            code: f.code,
            path: f.path,
            message: f.message,
            suggested_fix: f.suggested_fix,
        }));
    }

    // PC-14 D3 assent floor: the set of staged governed records that DECLARE a valid
    // assent_source. A defect on one of these may never block - it is degraded to
    // route-for-correction (ACT-ASSENTED-RECORD-PROTECTION), applied as a final pass.
    let mut assented_record_paths: std::collections::HashSet<String> =
        std::collections::HashSet::new();
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
            for rel in &changed {
                if vjs_core::front_door::is_governed_record(rel)
                    && let Ok(content) = std::fs::read_to_string(repo.join(rel))
                    && vjs_core::front_door::declares_valid_assent(&content)
                {
                    assented_record_paths.insert(rel.clone());
                }
            }
            findings.push(ValidationFinding {
                severity: Severity::Info,
                code: "STAGED_FILES".into(),
                path: None,
                message: format!("{} staged files", changed.len()),
                suggested_fix: None,
            });
            // REG-FEDERATION-COORDINATION-001 kernel-checkable bright-line (the same decision the hook uses,
            // here on the commit gate both repos run): a subscribing jurisdiction may not record an apex
            // (supreme/privy) court order; it must refer up. Fails the commit closed. ([2026] VJS-SC 4 D3.)
            let jurisdiction_id = Store::read_repo_config(repo)
                .ok()
                .flatten()
                .map(|c| c.jurisdiction_id)
                .unwrap_or_default();
            let apex_input = vjs_core::hook::HookInput {
                event: vjs_core::hook::HookEvent::PreCommit,
                repo_root: repo.to_path_buf(),
                actor: "lexby".into(),
                paths: changed.iter().map(PathBuf::from).collect(),
                tool: None,
            };
            if let Some(vjs_core::hook::HookDecision::Block(f)) =
                vjs_core::hook::apex_routing_decision(&apex_input, &jurisdiction_id, "vjs")
            {
                ok = false;
                findings.push(ValidationFinding {
                    severity: Severity::Fatal,
                    code: f.code,
                    path: None,
                    message: f.message,
                    suggested_fix: f.next,
                });
            }
            // Build repo state and evaluate invariants
            let repo_state = RepoScanner::build_repo_state(repo)?;
            let facts = vjs_lawpack::lawpack_facts(repo, &lawpack);
            let invariant_report = evaluate_invariants(&repo_state, &lawpack.invariants, &facts)?;
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
                    message: format!(
                        "{} invariants evaluated, all passed",
                        invariant_report.findings.len()
                    ),
                    suggested_fix: None,
                });
            }

            // Permit gate: governed staged paths require valid permit
            let config = Store::read_repo_config(repo)?;
            let (permit_required, permit_exempt) = if let Some(ref cfg) = config {
                let req = cfg
                    .governance
                    .as_ref()
                    .map(|g| g.permit_required.clone())
                    .unwrap_or_default();
                let ex = cfg
                    .governance
                    .as_ref()
                    .map(|g| g.permit_exempt.clone())
                    .unwrap_or_default();
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

            // Canon-write gate ([2026] VJS-PC 13 D1): the deterministic boundary
            // scanner extended to fire on lawpack/v2 records and inspect structured
            // fields, blocking subscriber-scoped content from entering canon
            // (ACT-005:s1/s5, ACT-007:s4). This is the commit-time half of the same
            // gate the pre_write hook runs; it closes the vector by which eleven
            // DEC-OPBOX/INV-OPBOX/SPEC-OPBOX files self-asserted into the public lawpack.
            let canon_repo_code = config
                .as_ref()
                .and_then(|c| c.repo_code.clone())
                .or_else(|| config.as_ref().map(|c| c.jurisdiction_id.to_uppercase()))
                .unwrap_or_else(|| "VJS".into());
            let canon_findings =
                RedactScanner::scan_canon_writes(repo, &staged_paths, &canon_repo_code);
            if !RedactScanner::check_public_safe(&canon_findings) {
                ok = false;
                for f in canon_findings {
                    findings.push(ValidationFinding {
                        severity: f.severity,
                        code: "CANON_BOUNDARY_VIOLATION".into(),
                        path: f.path,
                        message: f.message,
                        suggested_fix: Some(format!("{:?}", f.suggested_route)),
                    });
                }
            }

            // D3: the thin working-root jurisdiction check. A permit whose scope
            // reaches outside the working root is a true cross-repo permit, lawful
            // under ACT-007:s3 only by privy order / principal assent. No such
            // authority field exists in the repo-local model, so fail closed.
            for (permit_id, glob) in PermitGate::cross_repo_reaches(&permits) {
                ok = false;
                findings.push(ValidationFinding {
                    severity: Severity::Fatal,
                    code: "CROSS_REPO_PERMIT".into(),
                    path: None,
                    message: format!(
                        "Permit {permit_id} scopes '{glob}', which reaches outside the working \
                         root. A cross-repo reach into another repo's law is lawful only by a \
                         Privy Council order or Principal assent (ACT-007:s3). Failing closed."
                    ),
                    suggested_fix: Some(
                        "Re-scope the permit to in-root paths, or seek privy/principal authority"
                            .into(),
                    ),
                });
            }

            // PC-13 D10 (bench-integrity) + D7 (structural tier-floor): verify each
            // staged order against the constitution [2026] VJS-SC 2, read BY
            // REFERENCE (never hard-coded - that would amend the constitution).
            // Bifurcated by assent under ACT-ASSENTED-RECORD-PROTECTION: a
            // non-assented defective order hard-blocks (Fatal); a record declaring a
            // valid assent_source is only ever routed-for-correction (Warning), never
            // voided. Scoped to staged orders so historical records are not
            // retroactively re-judged.
            if let Some(constitution) = lawpack
                .orders
                .iter()
                .find(|o| o.id == "2026-VJS-COURTS-CONSTITUTION-001")
            {
                for rel in changed
                    .iter()
                    .filter(|p| p.starts_with("lawpack/v2/orders/") && p.ends_with(".yaml"))
                {
                    let Ok(content) = std::fs::read_to_string(repo.join(rel)) else {
                        continue;
                    };
                    let Ok(order) = serde_yaml::from_str::<vjs_core::types::Order>(&content) else {
                        continue;
                    };
                    let opinion_text = order
                        .source_opinion
                        .as_ref()
                        .and_then(|p| std::fs::read_to_string(repo.join(p)).ok());
                    let defects = vjs_core::bench::verify_bench(
                        &order,
                        constitution,
                        opinion_text.as_deref(),
                    );
                    if defects.is_empty() {
                        continue;
                    }
                    let assented = order
                        .assent_source
                        .as_ref()
                        .map(|s| !s.trim().is_empty())
                        .unwrap_or(false);
                    for d in defects {
                        let (severity, fix) = if assented {
                            (
                                Severity::Warning,
                                "Assented record: route for correction (never void). Correct the \
                                 bench size / opinions and re-record.",
                            )
                        } else {
                            ok = false;
                            (
                                Severity::Fatal,
                                "Constitute the bench correctly (constituted odd size + a non-empty \
                                 opinion per seat) before recording.",
                            )
                        };
                        findings.push(ValidationFinding {
                            severity,
                            code: d.code().into(),
                            path: Some(PathBuf::from(rel)),
                            message: d.message(),
                            suggested_fix: Some(fix.into()),
                        });
                    }
                }
            }
        }
    }

    if external && GitIntegration::is_public_remote(repo)? {
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

    // PC-13 D4 + D5: install-completeness invariant + atomic-manifest re-verify.
    // Fail closed unless the REG-INVOCATION-001 surface is present and active AND the
    // manifest (REG-INSTALL-MANIFEST-001) is in sync. Exempt for a non-jurisdiction
    // directory (no .vjs/). A half-installed jurisdiction is an agent operating only
    // some of the system's mechanisms - the disease PC-13 names.
    {
        let mut install_defects = vjs_core::install::verify_surface(repo);
        install_defects.extend(vjs_core::install::verify_manifest(repo));
        if !install_defects.is_empty() {
            ok = false;
            for d in install_defects {
                findings.push(ValidationFinding {
                    severity: Severity::Fatal,
                    code: d.code().into(),
                    path: None,
                    message: d.message(),
                    suggested_fix: Some(
                        "Run vjs invoke --install-hooks, then vjs install-lock".into(),
                    ),
                });
            }
        }
    }

    // PC-14 D3 assent floor (the consolidating limb's other edge): a record DECLARING
    // a valid assent_source may NEVER be voided or blocked - the instant assent
    // attaches, every block on that record DEGRADES to route-for-correction
    // (ACT-ASSENTED-RECORD-PROTECTION; entrenched). Surfaced and flagged, never
    // silently passed. The non-assented off-door record stays blocked; this only ever
    // softens, never hardens, and keys solely on the record declaring valid assent.
    if !assented_record_paths.is_empty() {
        for f in &mut findings {
            if matches!(f.severity, Severity::Fatal | Severity::Error)
                && let Some(p) = &f.path
                && assented_record_paths.contains(&p.to_string_lossy().to_string())
            {
                f.severity = Severity::Warning;
                f.message = format!(
                    "[{}: assented record routed for correction, never blocked - ACT-ASSENTED-RECORD-PROTECTION] {}",
                    vjs_core::front_door::ROUTE_FOR_CORRECTION_CODE,
                    f.message
                );
            }
        }
        // After the floor, ok reflects only genuinely blocking findings, so the
        // downgrade actually unblocks the assented record (every gate that set
        // ok=false also pushed a Fatal/Error finding).
        ok = !findings
            .iter()
            .any(|f| matches!(f.severity, Severity::Fatal | Severity::Error));
    }

    let result = ValidationResult { ok, findings };

    if json {
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else {
        println!("Validation: {}", if ok { "OK" } else { "FAILED" });
        for finding in &result.findings {
            println!(
                "  [{:?}] {}: {}",
                finding.severity, finding.code, finding.message
            );
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
        message: if report.ok {
            "Lawpack valid".into()
        } else {
            "Lawpack invalid".into()
        },
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
        message: if citation_ok {
            "No citation collisions".into()
        } else {
            "Citation collision detected".into()
        },
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
        message: if boundary_ok {
            "Boundary scan passed".into()
        } else {
            "Boundary scan failed".into()
        },
    });
    if !boundary_ok {
        ok = false;
    }

    // Step 4: Order validation
    let mut order_ok = true;
    for order in &lawpack.orders {
        if order.holding.is_empty()
            || order.directives.is_empty()
            || order.runtime_summary.is_empty()
        {
            order_ok = false;
        }
    }
    steps.push(CiStep {
        name: "order_validate".into(),
        passed: order_ok,
        message: if order_ok {
            "Orders valid".into()
        } else {
            "Invalid orders found".into()
        },
    });
    if !order_ok {
        ok = false;
    }

    // Step 5: Invariant evaluation
    let repo_state = RepoScanner::build_repo_state(repo)?;
    let facts = vjs_lawpack::lawpack_facts(repo, &lawpack);
    let invariant_report = evaluate_invariants(&repo_state, &lawpack.invariants, &facts)?;
    let invariant_ok = invariant_report.findings.iter().all(|f| f.passed);
    steps.push(CiStep {
        name: "invariant_eval".into(),
        passed: invariant_ok,
        message: if invariant_ok {
            format!("{} invariants passed", invariant_report.findings.len())
        } else {
            let failures: Vec<_> = invariant_report
                .findings
                .iter()
                .filter(|f| !f.passed)
                .map(|f| f.invariant_id.0.clone())
                .collect();
            format!("Invariant failures: {}", failures.join(", "))
        },
    });
    if !invariant_ok {
        ok = false;
    }

    // Step 6: Permit gate
    let config = Store::read_repo_config(repo)?;
    let (permit_required, permit_exempt) = if let Some(ref cfg) = config {
        let req = cfg
            .governance
            .as_ref()
            .map(|g| g.permit_required.clone())
            .unwrap_or_default();
        let ex = cfg
            .governance
            .as_ref()
            .map(|g| g.permit_exempt.clone())
            .unwrap_or_default();
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
            let failures: Vec<_> = gate_result
                .findings
                .iter()
                .filter(|f| matches!(f.severity, Severity::Fatal | Severity::Error))
                .map(|f| f.code.clone())
                .collect();
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
            println!(
                "  {}: {}",
                step.name,
                if step.passed { "PASS" } else { "FAIL" }
            );
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
            let content =
                std::fs::read_to_string(&path).map_err(|e| KernelError::Io(e.to_string()))?;
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
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "ok": ok,
                        "findings": findings
                    }))
                    .unwrap()
                );
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
            let content =
                std::fs::read_to_string(&path).map_err(|e| KernelError::Io(e.to_string()))?;
            let order: Order = serde_yaml::from_str(&content)
                .map_err(|e| KernelError::Serialization(e.to_string()))?;

            Store::write_order(repo, &order)?;

            println!("Order applied: {}", order.id);
        }
    }

    Ok(())
}

// ---- vjs bundle verify: the fail-closed deployment-bundle check (REG-BUNDLE-001) ----

#[derive(Deserialize)]
struct BundleManifest {
    schema_version: Option<u64>,
    bundle: Option<String>,
    distribution_licence: Option<String>,
    #[serde(default)]
    component: Vec<BundleComponent>,
}

#[derive(Deserialize)]
struct BundleComponent {
    id: Option<String>,
    repo: Option<String>,
    digest: Option<String>,
    source_commit: Option<String>,
    licence: Option<String>,
    adoption_mode: Option<String>,
}

const BUNDLE_COPYLEFT: &[&str] = &[
    "AGPL-3.0-only",
    "AGPL-3.0",
    "GPL-3.0-only",
    "GPL-3.0",
    "LGPL-3.0",
];
const BUNDLE_PERMISSIVE: &[&str] = &["MIT", "Apache-2.0", "BSD-3-Clause", "BSD-2-Clause", "ISC"];

fn is_sha256_digest(s: &str) -> bool {
    match s.strip_prefix("sha256:") {
        Some(h) => {
            h.len() == 64
                && h.bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        }
        None => false,
    }
}

/// Validate a parsed bundle manifest per REG-BUNDLE-001: every component carries
/// every prescribed field, every digest is a well-formed sha256, and the AGPL/MIT
/// licence firewall holds (an AGPL component into a permissive distribution
/// boundary only as a vendored, re-stamped, pinned copy). Pure (no I/O), so it is
/// unit-tested directly. Returns the pass summary, or the first violation.
fn verify_bundle_manifest(m: &BundleManifest) -> Result<String, String> {
    let present = |o: &Option<String>| o.as_deref().map(|s| !s.trim().is_empty()).unwrap_or(false);
    if m.schema_version.is_none() {
        return Err("manifest is missing the prescribed field 'schema_version'".into());
    }
    if !present(&m.bundle) {
        return Err("manifest is missing the prescribed field 'bundle'".into());
    }
    if !present(&m.distribution_licence) {
        return Err("manifest is missing the prescribed field 'distribution_licence'".into());
    }
    if m.component.is_empty() {
        return Err("manifest declares no components".into());
    }
    let dist = m.distribution_licence.as_deref().unwrap();
    let mut seen = std::collections::HashSet::new();
    for c in &m.component {
        let cid = c.id.as_deref().unwrap_or("<unnamed>");
        for (name, val) in [
            ("id", &c.id),
            ("repo", &c.repo),
            ("digest", &c.digest),
            ("source_commit", &c.source_commit),
            ("licence", &c.licence),
            ("adoption_mode", &c.adoption_mode),
        ] {
            if !present(val) {
                return Err(format!(
                    "component '{cid}' is missing the prescribed field '{name}'"
                ));
            }
        }
        if !seen.insert(cid.to_string()) {
            return Err(format!("duplicate component id '{cid}'"));
        }
        let digest = c.digest.as_deref().unwrap();
        if !is_sha256_digest(digest) {
            return Err(format!(
                "component '{cid}' digest is not a well-formed sha256: {digest}"
            ));
        }
        let licence = c.licence.as_deref().unwrap();
        let adoption = c.adoption_mode.as_deref().unwrap();
        if BUNDLE_PERMISSIVE.contains(&dist)
            && BUNDLE_COPYLEFT.contains(&licence)
            && adoption != "vendored-restamped-readonly"
        {
            return Err(format!(
                "licence firewall: copyleft component '{cid}' ({licence}) is consumed into a \
                 {dist} distribution boundary with adoption_mode '{adoption}'; AGPL is permitted \
                 only as vendored-restamped-readonly"
            ));
        }
    }
    Ok(format!(
        "bundle '{}' verified - {} components, distribution {}, licence firewall holds.",
        m.bundle.as_deref().unwrap(),
        m.component.len(),
        dist
    ))
}

fn cmd_bundle(repo: &Path, subcmd: BundleCommands, json: bool) -> Result<(), KernelError> {
    match subcmd {
        BundleCommands::Verify { path } => {
            let p = if path.is_absolute() {
                path.clone()
            } else {
                repo.join(&path)
            };
            let content = std::fs::read_to_string(&p).map_err(|e| {
                KernelError::InvalidInput(format!("cannot read {}: {e}", p.display()))
            })?;
            let manifest: BundleManifest = toml::from_str(&content)
                .map_err(|e| KernelError::InvalidInput(format!("bundle.lock parse error: {e}")))?;
            match verify_bundle_manifest(&manifest) {
                Ok(summary) => {
                    if json {
                        println!("{}", serde_json::json!({ "ok": true, "summary": summary }));
                    } else {
                        println!("OK: {summary}");
                        for c in &manifest.component {
                            println!(
                                "   {:8} {:16} {:30} {}",
                                c.id.as_deref().unwrap_or(""),
                                c.licence.as_deref().unwrap_or(""),
                                c.adoption_mode.as_deref().unwrap_or(""),
                                c.digest.as_deref().unwrap_or("")
                            );
                        }
                    }
                    Ok(())
                }
                Err(msg) => Err(KernelError::InvalidInput(format!("FAIL: {msg}"))),
            }
        }
    }
}

#[cfg(test)]
mod bundle_tests {
    use super::*;
    fn comp(id: &str, licence: &str, mode: &str) -> BundleComponent {
        BundleComponent {
            id: Some(id.into()),
            repo: Some("wlilley93/x".into()),
            digest: Some(format!("sha256:{}", "a".repeat(64))),
            source_commit: Some("abc1234".into()),
            licence: Some(licence.into()),
            adoption_mode: Some(mode.into()),
        }
    }
    fn manifest(dist: &str, comps: Vec<BundleComponent>) -> BundleManifest {
        BundleManifest {
            schema_version: Some(1),
            bundle: Some("house".into()),
            distribution_licence: Some(dist.into()),
            component: comps,
        }
    }
    #[test]
    fn passes_a_well_formed_bundle() {
        let m = manifest(
            "MIT",
            vec![
                comp("canon", "AGPL-3.0-only", "vendored-restamped-readonly"),
                comp("engine", "MIT", "vendored-readonly"),
            ],
        );
        assert!(verify_bundle_manifest(&m).is_ok());
    }
    #[test]
    fn fails_closed_on_licence_firewall_breach() {
        let m = manifest(
            "MIT",
            vec![comp("canon", "AGPL-3.0-only", "monorepo-package")],
        );
        let e = verify_bundle_manifest(&m).unwrap_err();
        assert!(e.contains("licence firewall"), "{e}");
    }
    #[test]
    fn fails_closed_on_missing_field() {
        let mut c = comp("canon", "MIT", "vendored-readonly");
        c.source_commit = None;
        let m = manifest("MIT", vec![c]);
        assert!(
            verify_bundle_manifest(&m)
                .unwrap_err()
                .contains("source_commit")
        );
    }
    #[test]
    fn fails_closed_on_bad_digest() {
        let mut c = comp("canon", "MIT", "vendored-readonly");
        c.digest = Some("sha256:notahex".into());
        let m = manifest("MIT", vec![c]);
        assert!(
            verify_bundle_manifest(&m)
                .unwrap_err()
                .contains("well-formed sha256")
        );
    }
}

fn cmd_court(repo: &Path, subcmd: CourtCommands, json: bool) -> Result<(), KernelError> {
    match subcmd {
        CourtCommands::Docket => {
            let submissions = Store::read_submissions(repo)?;
            // The docket is the canon orders (lawpack/v2/orders), not the
            // runtime .vjs/orders working copy.
            let orders = load_lawpack(repo).map(|l| l.orders).unwrap_or_default();
            let convenings = Store::read_convenings(repo)?;
            if json {
                println!(
                    "{}",
                    serde_json::json!({
                        "pending_submissions": submissions.iter().map(|s| serde_json::json!({
                            "id": s.id, "court": s.court_requested, "question": s.question,
                        })).collect::<Vec<_>>(),
                        "convenings": convenings.iter().map(|c| serde_json::json!({
                            "id": c.id, "court": c.court, "submission": c.submission_id,
                            "case_file_digest": c.case_file_digest, "bench": c.bench,
                        })).collect::<Vec<_>>(),
                        "orders": orders.iter().map(|o| serde_json::json!({
                            "id": o.id, "citation": o.citation, "court": o.court,
                            "issue": o.issue.0, "bench": o.bench, "case_file_digest": o.case_file_digest,
                        })).collect::<Vec<_>>(),
                    })
                );
            } else {
                println!("The docket\n");
                println!("Pending submissions ({}):", submissions.len());
                for s in &submissions {
                    println!("  {} -> {} : {}", s.id, s.court_requested, s.question);
                }
                println!("\nConvenings ({}):", convenings.len());
                for c in &convenings {
                    println!(
                        "  {} [{}] case-file {} bench {:?}",
                        c.id, c.court, c.case_file_digest, c.bench
                    );
                }
                println!("\nOrders ({}):", orders.len());
                for o in &orders {
                    let cite = o.citation.clone().unwrap_or_else(|| o.id.clone());
                    let recorded = if o.case_file_digest.is_some() {
                        "recorded"
                    } else {
                        "no convening record"
                    };
                    println!("  {} ({}) [{}]", cite, o.issue.0, recorded);
                }
            }
        }
        CourtCommands::Record {
            court,
            submission,
            bench,
            issue,
        } => {
            if bench.is_empty() {
                return Err(KernelError::InvalidInput(
                    "a convening records at least one --seat".into(),
                ));
            }
            // PC-13 D10 (court-record half): a convening is not validly constituted
            // unless its bench size equals the constituted odd size for the tier, read
            // BY REFERENCE from [2026] VJS-SC 2. The silent-seat half runs at validate
            // --staged once opinions exist. This refuses a malformed convening (e.g. a
            // privy bench of 2); it does not void any assented record.
            let court_tier = if court.contains("county") {
                Some(vjs_core::types::Court::County)
            } else if court.contains("privy") {
                Some(vjs_core::types::Court::PrivyCouncil)
            } else if court.contains("supreme") {
                Some(vjs_core::types::Court::SupremeCourt)
            } else {
                None
            };
            if let Some(tier) = court_tier
                && let Ok(lawpack) = load_lawpack(repo)
                && let Some(constitution) = lawpack
                    .orders
                    .iter()
                    .find(|o| o.id == "2026-VJS-COURTS-CONSTITUTION-001")
                && let Some(allowed) = vjs_core::bench::constituted_sizes(constitution, &tier)
                && !allowed.contains(&bench.len())
            {
                return Err(KernelError::InvalidInput(format!(
                    "bench of {} is not the constituted odd size {:?} for '{}' ([2026] VJS-SC 2). \
                     A court may not convene under-strength.",
                    bench.len(),
                    allowed,
                    court
                )));
            }
            let subs = Store::read_submissions(repo)?;
            let sub = subs.iter().find(|s| s.id == submission).ok_or_else(|| {
                KernelError::InvalidInput(format!("no filed submission {}", submission))
            })?;
            // The case-file digest pins exactly what was before the court.
            let bytes = serde_yaml::to_string(sub)
                .map_err(|e| KernelError::Serialization(e.to_string()))?;
            use sha2::Digest;
            let digest = format!(
                "sha256:{}",
                hex::encode(sha2::Sha256::digest(bytes.as_bytes()))
            );
            let convened_at = chrono::Utc::now().to_rfc3339();
            let rec = vjs_store::ConveningRecord {
                id: format!(
                    "CONVENING-{}-{}",
                    court,
                    chrono::Utc::now().format("%Y-%m-%d-%H%M%S")
                ),
                court,
                submission_id: submission,
                issue,
                case_file_digest: digest.clone(),
                bench,
                convened_at,
            };
            Store::write_convening(repo, &rec)?;
            if json {
                println!(
                    "{}",
                    serde_json::json!({ "convening": rec.id, "case_file_digest": digest })
                );
            } else {
                println!("Convening recorded: {} (case file {})", rec.id, digest);
            }
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
        std::fs::read_to_string(&path).map_err(|e| KernelError::Io(e.to_string()))?
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
        id: format!(
            "SUBMISSION-{}",
            chrono::Utc::now().format("%Y-%m-%d-%H%M%S")
        ),
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

    let active_permits = permits
        .iter()
        .filter(|p| matches!(p.status, PermitStatus::Active))
        .count();
    let closed_permits = permits
        .iter()
        .filter(|p| matches!(p.status, PermitStatus::Closed))
        .count();

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
        println!(
            "Permits: {} total, {} active, {} closed",
            status.permits_count, status.active_permits_count, status.closed_permits_count
        );
        println!("Proofs: {}", status.proofs_count);
    }

    Ok(())
}

/// Resolve the repo CODE for a `--repo` value: prefer the repo's declared `repo_code` from
/// `.vjs/config.toml`, falling back to the path's final component uppercased. This makes both
/// `--repo .` (a repo dir whose config declares OPBOX) and `--repo OPBOX` (a bare code) resolve a
/// sensible Cc-series code, instead of the hardcoded "REPO" placeholder.
fn resolve_repo_code(repo: &Path) -> String {
    if let Ok(txt) = std::fs::read_to_string(repo.join(".vjs/config.toml")) {
        for line in txt.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("repo_code") {
                let val = rest
                    .trim_start_matches([' ', '='])
                    .trim()
                    .trim_matches('"')
                    .trim();
                if !val.is_empty() {
                    return val.to_uppercase();
                }
            }
        }
    }
    repo.file_name()
        .map(|s| s.to_string_lossy().to_uppercase())
        .filter(|s| !s.is_empty() && s != ".")
        .unwrap_or_else(|| "REPO".into())
}

fn cmd_next_citation(
    repo: &Path,
    series: String,
    year: Option<i32>,
    json: bool,
) -> Result<(), KernelError> {
    let y = year.unwrap_or_else(|| chrono::Utc::now().year());
    let repo_code = resolve_repo_code(repo);
    let s = series.to_ascii_uppercase();

    // PC-13 D2: allocate from the LIVE persisted register (the citator index), not
    // an empty in-memory registry. The Cc series is bound to THIS repo's code; canon
    // series (PC/SC/REG/ACT/DEC/SPEC/INV/COA/...) carry no repo segment. The next
    // number is one past the current max, so a hand-asserted number cannot mint a
    // citation - validate --staged reconciles and fails closed on any collision.
    let lawpack_dir = repo.join("lawpack/v2");
    let (repo_for_lookup, repo_segment): (Option<&str>, String) = if s == "CC" {
        (Some(repo_code.as_str()), format!("-{}", repo_code))
    } else {
        (None, String::new())
    };
    let max = if lawpack_dir.exists() {
        LawpackValidator::live_citation_max(&lawpack_dir, &s, repo_for_lookup, y)?
    } else {
        0
    };
    let n = max + 1;
    let citation_str = format!("[{}] VJS-{}{} {}", y, s, repo_segment, n);

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "year": y,
                "series": s,
                "repoCode": repo_code,
                "n": n,
                "citation": citation_str
            }))
            .unwrap()
        );
    } else {
        println!("Next citation: {}", citation_str);
    }

    Ok(())
}

fn cmd_audit(repo: &Path, out: Option<PathBuf>, json: bool) -> Result<(), KernelError> {
    let lawpack = load_lawpack(repo)?;
    let report = vjs_lawpack::conformance_audit(&lawpack);

    if json {
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
        return Ok(());
    }

    // Render the conformance map (the D12 factual predicate).
    let mut md = String::new();
    md.push_str("# VJS Conformance Map (PC-13 D11)\n\n");
    md.push_str(
        "Produced THROUGH the kernel by `vjs audit`. Every kernel_effect duty \
         (must / must_not / prohibits) in every in-force statute and regulation, with \
         whether it is bound to a deterministic kernel gate. The UNWIRED list is the \
         factual predicate for the reserved D12 single-front-door instrument.\n\n",
    );
    md.push_str(&format!(
        "- total duties: {}\n- wired: {}\n- unwired: {}\n\n",
        report.total, report.wired, report.unwired
    ));
    md.push_str(
        "> Triage note: UNWIRED does not mean \"must be gated\". Many unwired duties are \
         declarative (`defines`-adjacent), one-time/transition acts, or agent-duties that \
         a deterministic gate cannot or need not enforce. The conservative registry marks a \
         duty WIRED only when a named, deterministic gate can be pointed at it, so the map \
         never overstates coverage. D12 triages this list to decide which unwired duties the \
         single-front-door instrument must bite on.\n\n",
    );

    md.push_str("## Unwired duties (the side doors)\n\n");
    md.push_str("| instrument | kind | duty |\n|---|---|---|\n");
    for d in report.duties.iter().filter(|d| d.gate.is_none()) {
        md.push_str(&format!(
            "| {} | {} | {} |\n",
            d.instrument, d.kind, d.token
        ));
    }
    md.push_str("\n## Wired duties\n\n");
    md.push_str("| instrument | kind | duty | gate |\n|---|---|---|---|\n");
    for d in report.duties.iter().filter(|d| d.gate.is_some()) {
        md.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            d.instrument,
            d.kind,
            d.token,
            d.gate.as_deref().unwrap_or("")
        ));
    }

    let out_path = out.unwrap_or_else(|| PathBuf::from("docs/conformance-map.md"));
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| KernelError::Io(e.to_string()))?;
    }
    std::fs::write(&out_path, &md).map_err(|e| KernelError::Io(e.to_string()))?;

    println!(
        "Conformance audit: {} duties, {} wired, {} unwired -> {}",
        report.total,
        report.wired,
        report.unwired,
        out_path.display()
    );
    Ok(())
}

fn cmd_install_lock(repo: &Path, json: bool) -> Result<(), KernelError> {
    // The surface must be complete before it can be atomically locked.
    let surface = vjs_core::install::verify_surface(repo);
    if !surface.is_empty() {
        let msgs: Vec<String> = surface.iter().map(|d| d.message()).collect();
        return Err(KernelError::InvalidInput(format!(
            "cannot lock an incomplete install: {}",
            msgs.join("; ")
        )));
    }
    // Materialise the thin agent-runtime adapters (D8) before locking, so the
    // manifest binds them (create-if-absent; never clobbers a customised adapter).
    let _ = vjs_core::install::generate_adapters(repo);
    let now = chrono::Utc::now().to_rfc3339();
    let manifest = vjs_core::install::build_manifest(repo, now).ok_or_else(|| {
        KernelError::InvalidInput("surface complete but manifest could not be built".into())
    })?;
    let body = toml::to_string(&manifest).map_err(|e| KernelError::Serialization(e.to_string()))?;
    let header = "# VJS install manifest (REG-INSTALL-MANIFEST-001). Atomic sha256 lock of the\n\
                  # REG-INVOCATION-001 surface. Re-lock with `vjs install-lock` after a surface change.\n";
    let path = repo.join(vjs_core::install::MANIFEST_FILE);
    std::fs::write(&path, format!("{header}{body}")).map_err(|e| KernelError::Io(e.to_string()))?;
    if json {
        println!(
            "{}",
            serde_json::json!({ "manifest": vjs_core::install::MANIFEST_FILE, "locked": true })
        );
    } else {
        println!(
            "Install manifest locked: {}",
            vjs_core::install::MANIFEST_FILE
        );
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
                    V1Source {
                        file: "Constitution/AGENTS.md".into(),
                        reference: "retrieval-first".into(),
                    },
                    V1Source {
                        file: "AGENTS.md".into(),
                        reference: "cdd-cli-spine".into(),
                    },
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
                v1_sources: vec![V1Source {
                    file: "README.md".into(),
                    reference: "five-triggers".into(),
                }],
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
                    V1Source {
                        file: "README.md".into(),
                        reference: "public-private".into(),
                    },
                    V1Source {
                        file: ".gitignore".into(),
                        reference: "private-exclusions".into(),
                    },
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

    let content =
        serde_yaml::to_string(&ledger).map_err(|e| KernelError::Serialization(e.to_string()))?;
    std::fs::write(&output, content).map_err(|e| KernelError::Io(e.to_string()))?;

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
        println!(
            "No eval suite matched '{}'. Try: agent-harness | prompts | route | all",
            suite
        );
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
    const SITE_BASE: &str = "https://wlilley93.github.io/vibe-justice-system/";
    const FEED_TAG: &str = "tag:wlilley93.github.io,2026-06-09:vibe-justice-system:gazette";

    let lawpack_dir = repo.join("lawpack/v2");
    // The Gazette publishes only a loadable canon.
    let _ = LawpackLoader::load(&lawpack_dir)?;

    let io = |e: std::io::Error| KernelError::Io(e.to_string());
    let ser = |e: serde_yaml::Error| KernelError::Serialization(e.to_string());

    fn s(v: &serde_yaml::Value, key: &str) -> Option<String> {
        v.get(key)
            .and_then(|x| x.as_str())
            .map(|x| x.trim().to_string())
    }
    fn str_list(v: &serde_yaml::Value, key: &str) -> Vec<String> {
        match v.get(key) {
            // A sequence: collect its string items (the canonical form).
            Some(x) if x.is_sequence() => x
                .as_sequence()
                .map(|seq| {
                    seq.iter()
                        .filter_map(|i| i.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
            // Tolerate a bare scalar string as a one-element list: varies / affirms /
            // appeal_of are written as scalars across the order record.
            Some(x) => x
                .as_str()
                .filter(|s| !s.is_empty())
                .map(|s| vec![s.to_string()])
                .unwrap_or_default(),
            None => Vec::new(),
        }
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
            if let Some(x) = v.get(k)
                && !x.is_null()
            {
                obj.insert(k.to_string(), yaml_to_json(x));
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
                let enacted: Vec<serde_json::Value> = v
                    .get("sections")
                    .and_then(|x| x.as_sequence())
                    .map(|secs| {
                        secs.iter()
                            .map(|sec| {
                                pick(sec, &["id", "title", "text", "commentary", "kernel_effect"])
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                // Contiguous numbering: every ordinal from s1 to the highest
                // enacted section appears; absent ordinals are Reserved (the
                // positive drafting convention), so the count never jumps.
                let act_id = v.get("id").and_then(|x| x.as_str()).unwrap_or_default();
                let ordinal = |sec: &serde_json::Value| -> Option<u32> {
                    sec["id"].as_str()?.rsplit(":s").next()?.parse().ok()
                };
                let max = enacted.iter().filter_map(&ordinal).max().unwrap_or(0);
                let mut by_ord: std::collections::HashMap<u32, serde_json::Value> = enacted
                    .into_iter()
                    .filter_map(|sec| ordinal(&sec).map(|n| (n, sec)))
                    .collect();
                let sections: Vec<serde_json::Value> = (1..=max)
                    .map(|n| {
                        by_ord.remove(&n).unwrap_or_else(|| {
                            serde_json::json!({
                                "id": format!("{}:s{}", act_id, n),
                                "title": "Reserved",
                                "reserved": true,
                            })
                        })
                    })
                    .collect();
                body["sections"] = serde_json::Value::Array(sections);
                body
            }
            "regulation" => pick(v, &["authority", "text", "kernel_effect"]),
            "order" => pick(
                v,
                &[
                    "question",
                    "holding",
                    "directives",
                    "forbidden",
                    "exceptions",
                    "runtime_summary",
                    "source_opinion",
                ],
            ),
            "decision" => pick(
                v,
                &[
                    "decision",
                    "reason",
                    "basis",
                    "consequences",
                    "review_triggers",
                    "scope",
                ],
            ),
            "invariant" => pick(v, &["severity", "rule", "remedy", "basis"]),
            "obligation" => pick(v, &["text", "kind", "due", "required", "basis"]),
            "spec" => pick(
                v,
                &[
                    "purpose",
                    "scope",
                    "decisions",
                    "invariants",
                    "obligations",
                    "review_triggers",
                ],
            ),
            "rule" => pick(
                v,
                &["summary", "effect", "scope", "exceptions", "rank", "source"],
            ),
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
    /// Capital Case for Act and Case names: significant words capitalised,
    /// small words lowered (never first or last), tokens already carrying
    /// capitals or digits (V1, VJS-PC, 2026, Computer-First) left alone.
    fn title_case(t: &str) -> String {
        const SMALL: [&str; 14] = [
            "a", "an", "and", "as", "at", "but", "by", "for", "in", "of", "on", "or", "the", "to",
        ];
        let words: Vec<&str> = t.split(' ').collect();
        let last = words.len().saturating_sub(1);
        words
            .iter()
            .enumerate()
            .map(|(i, w)| {
                if w.chars().skip(1).any(|c| c.is_uppercase())
                    || w.chars().any(|c| c.is_ascii_digit())
                {
                    return w.to_string();
                }
                let lower = w.to_lowercase();
                // a bracket or a dash opens a new phrase: the first word of a
                // subtitle ("... - the Founding Settlement") is capitalised
                let after_break = i > 0 && words[i - 1] == "-";
                if !after_break
                    && !w.starts_with('(')
                    && i != 0
                    && i != last
                    && SMALL.contains(&lower.trim_matches(|c: char| !c.is_alphanumeric()))
                {
                    return lower;
                }
                let mut cs = w.chars();
                match cs.next() {
                    Some(f) if f.is_alphabetic() => {
                        f.to_uppercase().collect::<String>() + cs.as_str()
                    }
                    Some('(') => {
                        // capitalise inside an opening bracket: "(the order)" -> "(The Order)"
                        let rest = cs.as_str();
                        let mut rc = rest.chars();
                        match rc.next() {
                            Some(g) => format!("({}{}", g.to_uppercase(), rc.as_str()),
                            None => w.to_string(),
                        }
                    }
                    _ => w.to_string(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
    /// "[2026] VJS-PC 5" from an id like "2026-VJS-PC-005"; None otherwise.
    fn derive_order_citation(id: &str) -> Option<String> {
        let parts: Vec<&str> = id.split('-').collect();
        if parts.len() == 4
            && parts[0].len() == 4
            && parts[0].chars().all(|c| c.is_ascii_digit())
            && parts[1] == "VJS"
            && matches!(parts[2], "SC" | "PC" | "CC" | "BOOT")
        {
            let n: u32 = parts[3].parse().ok()?;
            return Some(format!("[{}] VJS-{} {}", parts[0], parts[2], n));
        }
        None
    }

    // V1 archive: derive a REALM-form neutral citation from the id + year when
    // the curated estate left one blank (REALM-SC-1 -> [2026] REALM-SC 1;
    // SI-6 -> [2026] REALM-SI 6; BILL-1 -> [2026] REALM-BILL 1).
    fn derive_v1_citation(id: &str, date: &str) -> Option<String> {
        let year = date
            .get(0..4)
            .filter(|y| y.chars().all(|c| c.is_ascii_digit()))?;
        let (series, n) = id.rsplit_once('-')?;
        if n.is_empty() || !n.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }
        let series = match series.strip_prefix("REALM-") {
            Some(rest) => format!("REALM-{}", rest),
            None => format!("REALM-{}", series),
        };
        Some(format!("[{}] {} {}", year, series, n))
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

    // Dates from history, two single passes over git log: oldest-first with
    // --diff-filter=A gives first-enacted; newest-first gives last-amended.
    // (Orders carry created_at and prefer it for the enactment date.)
    fn git_dates(repo: &Path, extra: &[&str]) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();
        let mut args = vec![
            "-C".to_string(),
            repo.to_string_lossy().to_string(),
            "log".to_string(),
        ];
        args.extend(extra.iter().map(|a| a.to_string()));
        args.extend(["--name-only".to_string(), "--format=\u{1}%cI".to_string()]);
        if let Ok(out) = std::process::Command::new("git").args(&args).output() {
            let text = String::from_utf8_lossy(&out.stdout);
            let mut current = String::new();
            for line in text.lines() {
                if let Some(ts) = line.strip_prefix('\u{1}') {
                    // keep the full committer timestamp (%cI); the day is derived
                    // where a date-only display is wanted
                    current = ts.to_string();
                } else if !line.is_empty() && !current.is_empty() {
                    map.entry(line.to_string())
                        .or_insert_with(|| current.clone());
                }
            }
        }
        map
    }
    let added_at = git_dates(repo, &["--reverse", "--diff-filter=A"]);
    let updated_at = git_dates(repo, &[]);

    // Publication provenance: bind the artifact to the record it was
    // generated from. The on-disk lock keys (lawpack/digest/locked_at) do not
    // match Store::read_lawpack_lock's struct, so parse leniently here.
    let mut lock_meta: std::collections::HashMap<String, String> = Default::default();
    if let Ok(lock) = std::fs::read_to_string(repo.join(".vjs/lawpack.lock")) {
        for line in lock.lines() {
            if let Some((k, v)) = line.split_once('=') {
                lock_meta.insert(k.trim().to_string(), v.trim().trim_matches('"').to_string());
            }
        }
    }
    let source_commit = std::process::Command::new("git")
        .args(["-C", &repo.to_string_lossy(), "rev-parse", "HEAD"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    let mut items: Vec<serde_json::Value> = Vec::new();
    // Full-text bodies for the in-place reader, id -> kind-specific body.
    let mut texts: std::collections::BTreeMap<String, serde_json::Value> =
        std::collections::BTreeMap::new();
    // The Gazette registers legislation and case law only; the kernel
    // machinery is scheduled under REG-REALM-INVARIANTS-001 and read within
    // it (the instrument's own terms, assented per [2026] VJS-PC 7).
    const MACHINERY_INSTRUMENT: &str = "REG-REALM-INVARIANTS-001";
    const MACHINERY: [&str; 5] = ["invariant", "obligation", "rule", "spec", "decision"];
    let mut schedules: Vec<serde_json::Value> = Vec::new();

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
            // Name first, always: an untitled order is headed by the name of
            // its subject (the issue it was defined against), never its raw id.
            let title = s(&v, "title")
                .or_else(|| {
                    s(&v, "issue").map(|issue| {
                        let last = issue.rsplit('.').next().unwrap_or(&issue);
                        let stem = last.trim_start_matches("vjs-v2-").replace(['-', '_'], " ");
                        let mut cs = stem.chars();
                        match cs.next() {
                            Some(f) => format!("{}{} (the order)", f.to_uppercase(), cs.as_str()),
                            None => issue.clone(),
                        }
                    })
                })
                .filter(|t| !t.trim().is_empty())
                .unwrap_or_else(|| id.clone());
            let citation = s(&v, "citation").unwrap_or_default();
            let status = s(&v, "status")
                .or_else(|| s(&v, "severity").map(|sev| format!("severity {}", sev)))
                .unwrap_or_default();
            // The displayed court follows the CITATION series where one exists,
            // so the court label always matches the neutral citation a reader
            // sees: VJS-CC -> County Court, VJS-PC -> Privy Council, VJS-SC ->
            // Supreme Court, REALM-CA -> Court of Appeal. A county-coded order
            // with NO citation (the founding boot slate) keeps the [2026] VJS-PC
            // 6 first-instance-is-the-Privy-Council presentation.
            let court = if kind == "order" {
                let cite = s(&v, "citation").unwrap_or_default();
                let raw = s(&v, "court").unwrap_or_default();
                if cite.contains("-SC ") || cite.contains("REALM-SC") || raw == "supreme_court" {
                    "sc"
                } else if cite.contains("-CA ") || cite.contains("REALM-CA") {
                    "ca"
                } else if cite.contains("-CC") {
                    "county"
                } else if cite.contains("-PC ")
                    || cite.contains("REALM-PC")
                    || raw == "privy_council"
                {
                    "pc"
                } else if raw == "county" {
                    "pc"
                } else {
                    ""
                }
            } else {
                ""
            }
            .to_string();
            let title = if matches!(kind, "statute" | "regulation" | "rule" | "order") {
                title_case(&title)
            } else {
                title
            };

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
                        p.extend(
                            str_list(k, "must_not")
                                .iter()
                                .map(|m| format!("never {}", humanize(m))),
                        );
                        p.extend(
                            str_list(k, "prohibits")
                                .iter()
                                .map(|m| format!("prohibits {}", humanize(m))),
                        );
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
            let summary = ed
                .filter(|e| !e.summary.is_empty())
                .map(|e| e.summary.clone())
                .unwrap_or(mech_summary);
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

            let rel = format!(
                "lawpack/v2/{}/{}",
                dir,
                path.file_name().unwrap().to_string_lossy()
            );
            let day = |s: &str| {
                s.split('T')
                    .next()
                    .unwrap_or("")
                    .trim_matches('"')
                    .to_string()
            };
            // Full-precision sort key: the record's declared created_at if it has
            // one, else the git commit timestamp, else the day at midnight. The
            // register orders newest-first on this, so same-day records keep
            // their true chronological order, not a kind tiebreak.
            let ts = s(&v, "created_at")
                .filter(|c| c.contains('T'))
                .or_else(|| added_at.get(&rel).cloned())
                .unwrap_or_default();
            let date = s(&v, "created_at")
                .map(|c| day(&c))
                .filter(|c| !c.is_empty())
                .or_else(|| added_at.get(&rel).map(|t| day(t)))
                .unwrap_or_default();
            let ts = if ts.contains('T') {
                ts
            } else {
                format!("{}T00:00:00Z", date)
            };
            let citation = if citation.is_empty() && kind == "order" {
                derive_order_citation(&id).unwrap_or_default()
            } else {
                citation
            };
            if MACHINERY.contains(&kind) {
                schedules.push(serde_json::json!({
                    "id": id, "kind": kind, "title": title,
                    "status": status, "summary": summary, "points": points,
                    "path": rel,
                }));
                continue;
            }
            let mut item = serde_json::json!({
                "id": id, "title": title, "citation": citation, "kind": kind,
                "court": court, "estate": "v2", "status": status, "date": date, "ts": ts,
                "summary": summary, "points": points, "cites": cites,
                "supersedes": str_list(&v, "supersedes"),
                "varies": str_list(&v, "varies"),
                "affirms": str_list(&v, "affirms"),
                "has_text": true,
                // the question before the court, shown up top on court documents
                "question": s(&v, "question").unwrap_or_default(),
                "path": rel, "url": format!("{}{}", V2_BASE, rel),
            });
            item["doc"] = serde_json::Value::String(format!(
                "law.html#{}",
                item["id"].as_str().unwrap_or("")
            ));
            // Court orders render as PDF (the machine YAML stands alongside as
            // the secondary on the page). The PDF is a rendering of this very
            // record, carried under pdfs/orders/.
            if kind == "order" {
                let id = item["id"].as_str().unwrap_or("");
                let pdf_rel = format!("pdfs/orders/{}.pdf", id);
                if repo.join(&pdf_rel).exists() {
                    item["pdf"] = serde_json::Value::String(pdf_rel);
                }
            }
            if let Some(asrc) = s(&v, "assent_source").filter(|a| !a.is_empty()) {
                item["assent_source"] = serde_json::Value::String(asrc);
            }
            item["updated"] = serde_json::Value::String(
                updated_at
                    .get(&rel)
                    .map(|t| day(t))
                    .unwrap_or_else(|| date.clone()),
            );
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
                // the structured court record (REG-COURT-RECORD-001): the bench,
                // the vote, the pinned case-file digest - surfaced as a headnote
                let bench = str_list(&v, "bench");
                if !bench.is_empty() {
                    item["bench"] = serde_json::json!(bench);
                }
                for f in ["vote", "case_file_digest", "convened_at"] {
                    if let Some(val) = s(&v, f).filter(|x| !x.is_empty()) {
                        item[f] = serde_json::Value::String(val);
                    }
                }
            }
            items.push(item);
            texts.insert(
                items.last().unwrap()["id"].as_str().unwrap().to_string(),
                text_body(kind, &v),
            );
        }
    }

    // The consolidating instrument carries its schedules in full, grouped by
    // kind in schedule order ([2026] VJS-PC 7 D5).
    schedules.sort_by(|a, b| {
        let ord = |k: &str| MACHINERY.iter().position(|m| *m == k).unwrap_or(9);
        ord(a["kind"].as_str().unwrap_or(""))
            .cmp(&ord(b["kind"].as_str().unwrap_or("")))
            .then(a["id"].as_str().cmp(&b["id"].as_str()))
    });
    if let Some(body) = texts.get_mut(MACHINERY_INSTRUMENT) {
        let mut counts: std::collections::BTreeMap<String, usize> = Default::default();
        for sch in &schedules {
            *counts
                .entry(sch["kind"].as_str().unwrap_or("").to_string())
                .or_default() += 1;
        }
        body["schedules"] = serde_json::Value::Array(schedules.clone());
        if let Some(item) = items.iter_mut().find(|i| i["id"] == MACHINERY_INSTRUMENT) {
            let line = counts
                .iter()
                .map(|(k, n)| format!("{} {}s", n, k))
                .collect::<Vec<_>>()
                .join(", ");
            if let Some(pts) = item["points"].as_array_mut() {
                pts.push(serde_json::Value::String(format!("Schedules: {}", line)));
            }
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
                let v1_id = s(it, "id").unwrap_or_default();
                let v1_kind = s(it, "kind").unwrap_or_default();
                let v1_title = s(it, "title").unwrap_or_default();
                let v1_title = if matches!(v1_kind.as_str(), "act" | "instrument" | "judgment") {
                    title_case(&v1_title)
                } else {
                    v1_title
                };
                // A record with no native V1 PDF renders as a webpage from its
                // frozen source markdown, in the Gazette's own document style.
                let mut archive_text = false;
                if let Some(src) = s(it, "source_md").filter(|x| !x.is_empty())
                    && let Ok(md) = std::fs::read_to_string(repo.join(&src))
                {
                    texts.insert(v1_id.clone(), serde_json::json!({ "archive_md": md }));
                    archive_text = true;
                }
                let v1_id_for_doc = v1_id.clone();
                let v1_date = s(it, "date").unwrap_or_default();
                let v1_citation = {
                    let c = s(it, "citation").unwrap_or_default();
                    if c.is_empty() {
                        derive_v1_citation(&v1_id_for_doc, &v1_date).unwrap_or_default()
                    } else {
                        c
                    }
                };
                items.push(serde_json::json!({
                    "id": v1_id,
                    "title": v1_title,
                    "citation": v1_citation,
                    "kind": s(it, "kind").unwrap_or_default(),
                    "court": s(it, "court").unwrap_or_default(),
                    "estate": "v1",
                    "status": s(it, "status").unwrap_or_default(),
                    "date": s(it, "date").unwrap_or_default(),
                    "ts": format!("{}T00:00:00Z", s(it, "date").unwrap_or_default()),
                    "summary": s(it, "summary").unwrap_or_default(),
                    "points": str_list(it, "points"),
                    "cites": str_list(it, "cites"),
                    "supersedes": [],
                    "has_text": false,
                    "question": s(it, "question").unwrap_or_default(),
                    "archive_text": archive_text,
                    "updated": s(it, "date").unwrap_or_default(),
                    "pdf": s(it, "pdf").unwrap_or_default(),
                    "doc": format!("law.html#{}", v1_id_for_doc),
                    "path": path.clone(),
                    // The honoured archive is the frozen V1-lineage corpus,
                    // which spans more than the vibe-justice-system v1 branch
                    // (e.g. agent-universe). An estate item may name its own
                    // frozen github source; otherwise the v1 branch is assumed.
                    "url": s(it, "url").filter(|u| !u.is_empty())
                        .unwrap_or_else(|| format!("{}{}", V1_BASE, path)),
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
                if id == FOUNDING_ACT {
                    V1_FOUNDING_BILL
                } else {
                    FOUNDING_ACT
                }
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
    // The docket thread: orders on the same subject (the order's issue tag)
    // belong to one docket. They thread to the docket's ORIGIN - its first
    // entry - not each to its predecessor, because a docket is a hub, not a
    // sequence: the founding boot slate (BOOT-001..011) was passed together,
    // so BOOT-005 does not follow from BOOT-004; the orders are siblings of
    // one docket that opened with BOOT-001. Threading to the origin makes a
    // docket circle its first order instead of streaming off as a chain. (A
    // genuine appeal is a separate relationship, carried by appeal_of.)
    {
        // Issue tags are unique per case but carry family structure:
        // "governance.x" / "constitutional.x" are dotted dockets, and the
        // "vjs-v2-*" tags are the boot-series docket. The family is the
        // docket; the full issue stays on the item as its subject.
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
            if item["kind"] == "order"
                && let Some(subj) = item["subject"].as_str()
            {
                by_subject.entry(subject_family(subj)).or_default().push((
                    item["date"].as_str().unwrap_or_default().to_string(),
                    item["id"].as_str().unwrap_or_default().to_string(),
                    idx,
                ));
            }
        }
        for (_, mut docket) in by_subject {
            docket.sort();
            // a docket of one is no thread; otherwise every member after the
            // first threads to the origin (the hub)
            if let Some((_, origin, _)) = docket.first().cloned() {
                for (_, _id, idx) in docket.iter().skip(1) {
                    items[*idx]["thread"] = serde_json::json!([origin]);
                }
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
    // Treatment inverses: a higher court that varies/affirms a lower order is
    // recorded back on that order as varied_by/affirmed_by, so no node is a stale
    // dead-end. Mirrors superseded_by. Edges to off-gazette orders (e.g. County,
    // which live in .vjs/court/orders not lawpack/v2/orders) resolve away.
    let mut varied_by: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    let mut affirmed_by: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    for item in &mut items {
        let own_id = item["id"].as_str().unwrap_or_default().to_string();
        let estate = item["estate"].as_str().unwrap_or_default().to_string();
        let kind = item["kind"].as_str().unwrap_or_default().to_string();

        let raw_cites: Vec<String> = item["cites"]
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|c| c.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let resolved = resolve(&raw_cites, &own_id, &known);
        dropped += raw_cites.len().saturating_sub(resolved.len());
        item["cites"] = serde_json::Value::Array(
            resolved
                .iter()
                .cloned()
                .map(serde_json::Value::String)
                .collect(),
        );

        let raw_sup: Vec<String> = item["supersedes"]
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|c| c.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let sup = resolve(&raw_sup, &own_id, &known);
        for target in &sup {
            superseded_by
                .entry(target.clone())
                .or_default()
                .push(own_id.clone());
        }
        item["supersedes"] =
            serde_json::Value::Array(sup.into_iter().map(serde_json::Value::String).collect());

        for (field, map) in [("varies", &mut varied_by), ("affirms", &mut affirmed_by)] {
            let raw: Vec<String> = item[field]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|c| c.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();
            let resolved_t = resolve(&raw, &own_id, &known);
            for target in &resolved_t {
                map.entry(target.clone()).or_default().push(own_id.clone());
            }
            item[field] = serde_json::Value::Array(
                resolved_t
                    .into_iter()
                    .map(serde_json::Value::String)
                    .collect(),
            );
        }

        // A case interweaves by its SUBJECT: the legislation it construes
        // (resolved citations) or the docket thread on its issue. The
        // constitutional anchor is a last resort against orphaning only.
        let case_like = kind == "order" || kind == "judgment";
        let threaded = item["thread"]
            .as_array()
            .map(|a| !a.is_empty())
            .unwrap_or(false);
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
        let own_id = item["id"].as_str().unwrap_or_default().to_string();
        for (field, map) in [
            ("superseded_by", &superseded_by),
            ("varied_by", &varied_by),
            ("affirmed_by", &affirmed_by),
        ] {
            let mut v = map.get(&own_id).cloned().unwrap_or_default();
            v.sort();
            v.dedup();
            item[field] =
                serde_json::Value::Array(v.into_iter().map(serde_json::Value::String).collect());
        }
        // Every V1 archive node carries the uniform migration relation to the V2
        // canon, so no honoured-archive record is a navigable dead-end. It was
        // superseded as live law by the consolidation (ACT-COMPUTER-FIRST-REALM
        // s.6; preserved as archive by s.7) and its settled law was restated in
        // Schedule 1 of the framework (ACT-CONSOLIDATION-FRAMEWORK s.4), live in
        // V2 only where expressly incorporated (s.20). This is a MIGRATION edge,
        // not per-ruling court treatment: no V2 record varies/affirms/overrules an
        // individual V1 node, and V1 declares no such treatment of itself. Both
        // targets are V2 statutes in the gazette, so the edges resolve as links.
        if item["estate"] == "v1" {
            item["migration"] = serde_json::json!({
                "superseded_as_live_by": ["ACT-COMPUTER-FIRST-REALM"],
                "restated_in": ["ACT-CONSOLIDATION-FRAMEWORK"],
            });
        }
    }

    let v2_count = items.iter().filter(|i| i["estate"] == "v2").count();
    let data = serde_json::json!({
        "generated_at": chrono::Utc::now().to_rfc3339(),
        "meta": {
            "lawpack": {
                "id": lock_meta.get("lawpack"),
                "digest": lock_meta.get("digest"),
                "locked_at": lock_meta.get("locked_at"),
            },
            "source_commit": source_commit,
            "counts": { "total": items.len(), "canon": v2_count, "archive": items.len() - v2_count },
        },
        "items": items,
    });
    // A `</` inside law text would terminate the host <script> tag; emit the
    // JSON escape `<\/` instead (identical parse, inert markup).
    let guard = |s: String| s.replace("</", "<\\/");
    let out_path = out.unwrap_or_else(|| repo.join("gazette-data.js"));
    // Publication boundary gate (prevention for BREACH-2026-06-10-client-data-
    // published): the Gazette is public record, so scan everything it is about
    // to publish and FAIL CLOSED before writing. (1) RedactScanner catches
    // secrets, tokens, emails, and private-domain references. (2) A hashed
    // denylist (.vjs/publication-denylist.txt) catches private identifiers, e.g.
    // a client name on a carried external-matter artifact - which is private by
    // default. The path that carries archive PDFs and estate text now has the
    // boundary scan the governed writers always had.
    {
        let published = format!(
            "{}\n{}",
            serde_json::to_string(&data).unwrap_or_default(),
            serde_json::to_string(&texts).unwrap_or_default()
        );
        // Keep the high-confidence findings (keys, tokens, emails, passwords);
        // drop PrivateHostname, whose word.local/internal/private/lan pattern
        // false-positives on legal prose and config examples (e.g. the Boundary
        // Act's own "private_store.local" illustration).
        let findings: Vec<_> = vjs_redact::RedactScanner::scan_file(&out_path, &published)
            .into_iter()
            .filter(|f| !matches!(f.kind, BoundaryFindingKind::PrivateHostname))
            .collect();
        if !findings.is_empty() {
            let kinds: Vec<String> = findings.iter().map(|f| f.message.clone()).collect();
            return Err(KernelError::InvalidInput(format!(
                "publication boundary: the Gazette would publish private data ({}); fix the record before regenerating",
                kinds.join("; ")
            )));
        }
        if let Ok(deny) = std::fs::read_to_string(repo.join(".vjs/publication-denylist.txt")) {
            use sha2::Digest;
            let hashes: std::collections::HashSet<String> = deny
                .lines()
                .map(|l| l.trim())
                .filter(|l| !l.is_empty() && !l.starts_with('#'))
                .map(|l| l.to_string())
                .collect();
            let mut token = String::new();
            let mut hit = false;
            let check = |t: &str, hit: &mut bool| {
                if t.len() >= 3 {
                    let h = format!("{:x}", sha2::Sha256::digest(t.to_lowercase().as_bytes()));
                    if hashes.contains(&h) {
                        *hit = true;
                    }
                }
            };
            for ch in published.chars() {
                if ch.is_alphanumeric() || ch == '-' {
                    token.push(ch);
                } else {
                    check(&token, &mut hit);
                    token.clear();
                }
            }
            check(&token, &mut hit);
            if hit {
                return Err(KernelError::InvalidInput(
                    "publication boundary: the Gazette would publish a denylisted private term; a carried external-matter artifact is private by default and must be cleared before publication (BREACH-2026-06-10)".into(),
                ));
            }
        }
    }

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

    // Plain JSON for tooling, beside the JS.
    let json_path = out_path.with_extension("json");
    std::fs::write(
        &json_path,
        serde_json::to_string_pretty(&data).expect("gazette data serializes"),
    )
    .map_err(io)?;

    // The Atom feed: the Gazette as a periodical of record. Deterministic on
    // unchanged law: entry ids are the lawpack's own ids, entry dates come
    // from enactment and amendment history, and the feed's updated is the max
    // entry updated, never the generation time.
    fn xml_esc(t: &str) -> String {
        t.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }
    let mut feed_items: Vec<&serde_json::Value> = items.iter().collect();
    feed_items.sort_by(|a, b| {
        b["updated"]
            .as_str()
            .cmp(&a["updated"].as_str())
            .then(a["id"].as_str().cmp(&b["id"].as_str()))
    });
    let feed_updated = feed_items
        .iter()
        .filter_map(|i| i["updated"].as_str())
        .max()
        .unwrap_or("2026-06-09");
    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
    xml.push_str("<feed xmlns=\"http://www.w3.org/2005/Atom\">\n");
    xml.push_str("  <title>The VJS Gazette</title>\n");
    xml.push_str("  <subtitle>The record of the realm: the living canon and the honoured archive</subtitle>\n");
    xml.push_str(&format!("  <id>{}</id>\n", FEED_TAG));
    xml.push_str(&format!(
        "  <updated>{}T00:00:00Z</updated>\n",
        feed_updated
    ));
    xml.push_str(&format!(
        "  <link rel=\"self\" href=\"{}gazette.xml\"/>\n",
        SITE_BASE
    ));
    xml.push_str(&format!(
        "  <link rel=\"alternate\" href=\"{}\"/>\n",
        SITE_BASE
    ));
    xml.push_str("  <author><name>Vibe Justice System</name></author>\n");
    xml.push_str("  <rights>Publication is constitutively inert (REG-GAZETTE-CONTINUITY-001): force comes from the lawpack and the Sovereign's assent, never from publication or syndication.</rights>\n");
    for i in &feed_items {
        let id = i["id"].as_str().unwrap_or_default();
        xml.push_str("  <entry>\n");
        xml.push_str(&format!("    <id>{}:{}</id>\n", FEED_TAG, xml_esc(id)));
        xml.push_str(&format!(
            "    <title>{}</title>\n",
            xml_esc(i["title"].as_str().unwrap_or(id))
        ));
        xml.push_str(&format!(
            "    <link rel=\"alternate\" href=\"{}#{}\"/>\n",
            SITE_BASE,
            xml_esc(id)
        ));
        xml.push_str(&format!(
            "    <link rel=\"via\" href=\"{}\"/>\n",
            xml_esc(i["url"].as_str().unwrap_or(""))
        ));
        xml.push_str(&format!(
            "    <category term=\"{}\"/>\n",
            xml_esc(i["kind"].as_str().unwrap_or(""))
        ));
        xml.push_str(&format!(
            "    <category term=\"{}\"/>\n",
            if i["estate"] == "v1" {
                "archive"
            } else {
                "canon"
            }
        ));
        xml.push_str(&format!(
            "    <published>{}T00:00:00Z</published>\n",
            i["date"].as_str().unwrap_or("2026-06-09")
        ));
        xml.push_str(&format!(
            "    <updated>{}T00:00:00Z</updated>\n",
            i["updated"].as_str().unwrap_or("2026-06-09")
        ));
        xml.push_str(&format!(
            "    <summary type=\"text\">{}</summary>\n",
            xml_esc(i["summary"].as_str().unwrap_or(""))
        ));
        xml.push_str("  </entry>\n");
    }
    xml.push_str("</feed>\n");
    let feed_path = out_path.with_file_name("gazette.xml");
    std::fs::write(&feed_path, xml).map_err(io)?;

    // JSON-LD: each register item as schema.org Legislation, injected
    // idempotently between the marker tags in gazette.html. The
    // legislationLegalForce property is deliberately omitted: asserting
    // force from the publication surface would cut against inertness.
    let gazette_page = out_path.with_file_name("gazette.html");
    if let Ok(html) = std::fs::read_to_string(&gazette_page) {
        const START: &str = "<script type=\"application/ld+json\" id=\"gazette-jsonld\">";
        const END: &str = "</script>";
        if let Some(s_idx) = html.find(START)
            && let Some(e_off) = html[s_idx + START.len()..].find(END)
        {
            let mut graph = vec![serde_json::json!({
                "@type": "Periodical",
                "name": "The VJS Gazette",
                "url": SITE_BASE,
            })];
            for i in &items {
                let id = i["id"].as_str().unwrap_or_default();
                let citation = i["citation"].as_str().unwrap_or_default();
                graph.push(serde_json::json!({
                    "@type": "Legislation",
                    "name": i["title"],
                    "legislationIdentifier": if citation.is_empty() { id } else { citation },
                    "legislationDate": i["date"],
                    "legislationType": i["kind"],
                    "url": format!("{}law.html#{}", SITE_BASE, id),
                    "isPartOf": { "@type": "Periodical", "name": "The VJS Gazette" },
                }));
            }
            let ld = serde_json::json!({ "@context": "https://schema.org", "@graph": graph });
            let body = guard(serde_json::to_string(&ld).expect("jsonld serializes"));
            let new_html = format!(
                "{}{}\n{}\n{}",
                &html[..s_idx],
                START,
                body,
                &html[s_idx + START.len() + e_off..]
            );
            std::fs::write(&gazette_page, new_html).map_err(io)?;
        }
    }

    if json {
        println!(
            "{}",
            serde_json::json!({
                "out": out_path.to_string_lossy(),
                "json_out": json_path.to_string_lossy(),
                "feed_out": feed_path.to_string_lossy(),
                "text_out": text_path.to_string_lossy(),
                "text_bytes": text_body_js.len(),
                "items": known.len(),
                "edges_dropped_to_non_items": dropped,
            })
        );
    } else {
        println!(
            "Gazette data: {} items -> {}",
            known.len(),
            out_path.display()
        );
        println!(
            "  full text: {} bodies ({} KB) -> {}",
            texts.len(),
            text_body_js.len() / 1024,
            text_path.display()
        );
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
        let content = std::fs::read(&manifest).map_err(|e| KernelError::Io(e.to_string()))?;
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
                        println!(
                            "{} ({:?}): {} obligations",
                            permit.id.0,
                            permit.status,
                            permit.obligations.len()
                        );
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
                let digest = format!(
                    "sha256:{}",
                    hex::encode(sha2::Sha256::digest(proof_content.as_bytes()))
                );
                let proof = Proof {
                    id: ProofId(format!(
                        "PROOF-{}",
                        chrono::Utc::now().format("%Y%m%d-%H%M%S")
                    )),
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
