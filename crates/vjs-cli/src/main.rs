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

mod admin;
use admin::*;
mod bundle;
mod context;
pub(crate) use context::*;
mod eval;
use eval::*;
mod front;
mod permit;
use permit::*;
use front::*;
mod invoke;
mod lifecycle;
use lifecycle::*;
mod status;
use status::*;
mod local_ci;
mod gazette;

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
    /// (Re)pin the entrenched-enforcement surface .vjs/enforcement-surface.lock over the
    /// current gate-source digests (PC-16 D4). Run ONLY after a deliberate, recorded gate
    /// change - the re-pin is the visible acknowledgment that a gate moved.
    EnforcementLock,
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
    /// The deterministic runtime permit clerk ([2026] VJS-PC 15 D5): resolve a runtime
    /// act (scope + verb) against the two-tier overlay and dispose GRANT / DENY /
    /// ROUTE_FOR_CORRECTION. A thin transport; the check is in the kernel.
    SubmitDecision {
        /// The entity scope as ordered dimension=value pairs, broad->narrow,
        /// comma-separated (e.g. "a=1,b=2"). Dimension names are subscriber-supplied;
        /// an empty value is the apex scope where the canon floors bind.
        #[arg(long, default_value = "")]
        scope: String,
        /// The runtime act verb (subscriber-supplied; never canon-enumerated).
        #[arg(long)]
        verb: String,
        /// A declared assent_source (triggers the VJS-ACT 10 floor when allow-listed).
        #[arg(long)]
        assent_source: Option<String>,
        /// Canon Tier-1 floors dir (default: lawpack/v2/overlay-floors).
        #[arg(long)]
        floors: Option<PathBuf>,
        /// Subscriber Tier-2 rules dir (default: .vjs/local-lawpack/rules).
        #[arg(long)]
        local: Option<PathBuf>,
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
        } => invoke::cmd_invoke(&repo, jurisdiction, principal, lawpack, install_hooks, json),
        Commands::Lookup { issue, limit } => cmd_lookup(&repo, issue, limit, json),
        Commands::Log { subcmd } => cmd_log(&repo, subcmd, json),
        Commands::Proof { subcmd } => cmd_proof(&repo, subcmd, json),
        Commands::Validate {
            staged,
            external,
            scope,
        } => cmd_validate(&repo, staged, external, scope, json),
        Commands::LocalCi => local_ci::cmd_local_ci(&repo, json),
        Commands::Order { subcmd } => cmd_order(&repo, subcmd, json),
        Commands::Court { subcmd } => cmd_court(&repo, subcmd, json),
        Commands::Bundle { subcmd } => bundle::cmd_bundle(&repo, subcmd, json),
        Commands::File {
            court,
            question,
            facts_file,
        } => cmd_file(&repo, court, question, facts_file, json),
        Commands::Status => cmd_status(&repo, json),
        Commands::NextCitation { series, year } => cmd_next_citation(&repo, series, year, json),
        Commands::InstallLock => cmd_install_lock(&repo, json),
        Commands::EnforcementLock => match vjs_core::enforcement::write_lock(&repo) {
            Ok(()) => {
                println!(
                    "Pinned the entrenched-enforcement surface (.vjs/enforcement-surface.lock)."
                );
                Ok(())
            }
            Err(e) => Err(KernelError::Io(e.to_string())),
        },
        Commands::Audit { out } => cmd_audit(&repo, out, json),
        Commands::MigrateV1 { v1_path, out } => cmd_migrate_v1(&repo, &v1_path, out, json),
        Commands::Permit { subcmd } => cmd_permit(&repo, subcmd, json),
        Commands::Eval { suite } => cmd_eval(&repo, suite, json),
        Commands::Gazette { out } => gazette::cmd_gazette(&repo, out, json),
        Commands::SubmitDecision {
            scope,
            verb,
            assent_source,
            floors,
            local,
        } => cmd_submit_decision(&repo, scope, verb, assent_source, floors, local, json),
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

/// A convening id that does not collide with an existing record (#15): same
/// second-precision base, with a -N suffix appended on collision so a convening is
/// never silently overwritten by a sibling sharing its timestamp.
fn unique_convening_id(repo: &Path, court: &str) -> String {
    let base = format!(
        "CONVENING-{}-{}",
        court,
        chrono::Utc::now().format("%Y-%m-%d-%H%M%S")
    );
    let dir = repo.join(".vjs/court/convenings");
    let mut id = base.clone();
    let mut n = 2;
    while dir.join(format!("{id}.yaml")).exists() {
        id = format!("{base}-{n}");
        n += 1;
    }
    id
}

/// Resolve the repo CODE for a `--repo` value: prefer the repo's declared `repo_code` from
/// `.vjs/config.toml`, falling back to the path's final component uppercased. This makes both
/// `--repo .` (a repo dir whose config declares ACMECO) and `--repo ACMECO` (a bare code) resolve a
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

// the validate pipeline now lives in vjs-engine (REG-KERNEL-001: one smart point).

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

