//! The matter-governance lifecycle commands: log, proof, validate, submit-decision, order,
//! court, file, and status.

use super::*;

pub(crate) fn cmd_log(repo: &Path, subcmd: LogCommands, json: bool) -> Result<(), KernelError> {
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

pub(crate) fn cmd_proof(repo: &Path, subcmd: ProofCommands, json: bool) -> Result<(), KernelError> {
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

pub(crate) fn cmd_validate(
    repo: &Path,
    staged: bool,
    external: bool,
    _scope: Option<String>,
    json: bool,
) -> Result<(), KernelError> {
    // The validate pipeline now lives in the kernel (vjs-engine), called identically
    // by the CLI, the MCP server, and CI. This adapter only prints and sets the exit.
    let report = vjs_engine::validate(repo, &vjs_engine::ValidateOpts { staged, external })?;
    if json {
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
    } else {
        println!("Validation: {}", if report.ok { "OK" } else { "FAILED" });
        for fd in &report.findings {
            println!("  [{:?}] {}: {}", fd.severity, fd.code, fd.message);
        }
    }
    if !report.ok {
        std::process::exit(1);
    }
    Ok(())
}

/// PC-15 D5: a thin transport over the kernel's deterministic runtime clerk. Loads the
/// two-tier overlay, builds the envelope, calls vjs_engine::runtime::submit_decision,
/// and prints the disposition. No checking logic lives here.
pub(crate) fn cmd_submit_decision(
    repo: &Path,
    scope: String,
    verb: String,
    assent_source: Option<String>,
    floors: Option<PathBuf>,
    local: Option<PathBuf>,
    json: bool,
) -> Result<(), KernelError> {
    use vjs_core::scope::EntityScope;
    use vjs_engine::runtime::{DecisionEnvelope, submit_decision};
    use vjs_lawpack::overlay::OverlayLoader;

    let floors_dir = floors.unwrap_or_else(|| repo.join("lawpack/v2/overlay-floors"));
    let local_dir = local.unwrap_or_else(|| repo.join(".vjs/local-lawpack/rules"));
    let (overlay, load_findings) = OverlayLoader::load(&floors_dir, &local_dir)?;

    // Parse "k=v,k=v" into ordered dims (an empty scope is the apex).
    let dims: Vec<(String, String)> = scope
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .filter_map(|s| {
            s.split_once('=')
                .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        })
        .collect();
    let env = DecisionEnvelope {
        scope: EntityScope::new(dims),
        verb,
        assent_source,
    };
    let result = submit_decision(&overlay, &env);

    if json {
        let out = serde_json::json!({
            "disposition": result.disposition.as_str(),
            "law_source": result.law_source,
            "instrument": result.instrument,
            "load_findings": load_findings.len(),
        });
        println!("{}", serde_json::to_string_pretty(&out).unwrap());
    } else {
        println!("Decision: {}", result.disposition.as_str());
        if !result.law_source.is_empty() {
            println!("  law_source: {}", result.law_source.join(", "));
        }
        if let Some(i) = &result.instrument {
            println!("  instrument: {i}");
        }
        for fd in &load_findings {
            println!("  [{:?}] {}: {}", fd.severity, fd.code, fd.message);
        }
    }
    Ok(())
}

pub(crate) fn cmd_order(repo: &Path, subcmd: OrderCommands, json: bool) -> Result<(), KernelError> {
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

            // [2026] VJS-CC-VJS 11 D1. Until this ran, `order validate` checked three
            // emptiness conditions and then printed "Order validation: OK", which reads as
            // a full validation and is not one. It missed BENCH_OPINION_MISSING, a
            // CONSTITUTIVE code (assent.rs) that `validate --staged` treats as Fatal, so
            // the same bytes returned OK here and Fatal there. Proven by seeded case: an
            // order declaring a bench with its source_opinion removed passed this command.
            // The bench check is the one that decides whether the thing IS an order, so it
            // is precisely the check a command called `order validate` must not omit.
            if !order.bench.is_empty() {
                match load_lawpack(repo)
                    .ok()
                    .and_then(|l| l.orders.into_iter().find(|o| o.id == vjs_core::bench::COURTS_CONSTITUTION_ID))
                {
                    Some(constitution) => {
                        let opinion_text = order
                            .source_opinion
                            .as_ref()
                            .and_then(|p| std::fs::read_to_string(repo.join(p)).ok());
                        for d in vjs_core::bench::verify_bench(
                            &order,
                            &constitution,
                            opinion_text.as_deref(),
                        ) {
                            ok = false;
                            findings.push(format!("{}: {}", d.code(), d.message()));
                        }
                    }
                    // D4: say what could not be computed rather than pass silently. A
                    // bench that could not be checked must never read as a bench that was.
                    None => {
                        ok = false;
                        findings.push(
                            "BENCH_UNCHECKED: the courts-constitution order could not be loaded, \
                             so the declared bench was not verified. Not a pass."
                                .into(),
                        );
                    }
                }
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

pub(crate) fn cmd_court(repo: &Path, subcmd: CourtCommands, json: bool) -> Result<(), KernelError> {
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
            if let Ok(lawpack) = load_lawpack(repo)
                && let Some(constitution) = lawpack
                    .orders
                    .iter()
                    .find(|o| o.id == "2026-VJS-COURTS-CONSTITUTION-001")
                && let Err(msg) =
                    vjs_core::bench::convening_bench_check(constitution, &court, bench.len())
            {
                return Err(KernelError::InvalidInput(format!(
                    "{msg}. A court may not convene under-strength."
                )));
            }
            let subs = Store::read_submissions(repo)?;
            let _sub = subs.iter().find(|s| s.id == submission).ok_or_else(|| {
                KernelError::InvalidInput(format!("no filed submission {}", submission))
            })?;
            // The case-file digest pins exactly what was before the court. #3 (the
            // digest seam both the PC-13 and PC-14 benches flagged): pin the RAW
            // submission file bytes the bench reads, NOT a re-serialized struct that
            // silently drops hand-authored fields (e.g. `positions`).
            let sub_path = repo
                .join(".vjs/submissions/filed")
                .join(format!("{submission}.yaml"));
            let bytes = std::fs::read(&sub_path).map_err(|e| KernelError::Io(e.to_string()))?;
            use sha2::Digest;
            let digest = format!("sha256:{}", hex::encode(sha2::Sha256::digest(&bytes)));
            let convened_at = chrono::Utc::now().to_rfc3339();
            let rec = vjs_store::ConveningRecord {
                id: unique_convening_id(repo, &court),
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

pub(crate) fn cmd_file(
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

pub(crate) fn cmd_status(repo: &Path, json: bool) -> Result<(), KernelError> {
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
