//! The orchestration kernel: the validate pipeline (and, later, record-creation),
//! lifted out of the CLI so the CLI, the MCP server, and CI all call ONE
//! implementation. REG-KERNEL-001's "only smart point", in one place.
//!
//! Every gate returns `vjs_core::report::Finding`; `validate` composes them and
//! applies the PC-14 D3 assent floor. Behaviour is identical to the former 592-line
//! `cmd_validate` - this is a move, not a redesign; the test suite is the net.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use vjs_core::report::{Finding, Report};
use vjs_core::types::Severity;
use vjs_core::{KernelError, RepoScanner, evaluate_invariants};
use vjs_git::GitIntegration;
use vjs_lawpack::{Lawpack, LawpackLoader, LawpackValidator, lawpack_facts};
use vjs_redact::RedactScanner;
use vjs_store::Store;

pub mod assent;
pub mod runtime;

/// Options for a validate run.
#[derive(Clone, Debug, Default)]
pub struct ValidateOpts {
    /// Run the commit-gate (staged) checks: permit, bench/order, canon-write, apex,
    /// media, destructive-delete, install surface, and the assent floor.
    pub staged: bool,
    /// Warn if the repo has a public remote (release-warrant reminder).
    pub external: bool,
}

fn f(severity: Severity, code: &str, message: String) -> Finding {
    Finding {
        severity,
        code: code.into(),
        path: None,
        message,
        citation: None,
        suggested_fix: None,
    }
}

/// Load the canon lawpack (empty when there is none).
pub fn load_lawpack(repo: &Path) -> Result<Lawpack, KernelError> {
    let dir = repo.join("lawpack/v2");
    if dir.exists() {
        LawpackLoader::load(&dir)
    } else {
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

/// The lawpack digest the lock pins: the sha256 of lawpack/v2/manifest.toml.
pub fn compute_digest(repo: &Path) -> Result<String, KernelError> {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    let manifest = repo.join("lawpack/v2/manifest.toml");
    if manifest.exists() {
        let content = std::fs::read(&manifest).map_err(|e| KernelError::Io(e.to_string()))?;
        hasher.update(&content);
    }
    Ok(format!("sha256:{}", hex::encode(hasher.finalize())))
}

/// Run the full validate pipeline and return the report. Pure of I/O effects beyond
/// reading the repo; the caller prints / exits.
pub fn validate(repo: &Path, opts: &ValidateOpts) -> Result<Report, KernelError> {
    let mut findings: Vec<Finding> = Vec::new();

    // --- Lawpack-wide checks (always) ---
    let lawpack = load_lawpack(repo)?;
    let report = LawpackValidator::validate(&lawpack)?;
    for ff in report.findings {
        findings.push(Finding {
            severity: ff.severity,
            code: ff.code,
            path: ff.path,
            message: ff.message,
            citation: None,
            suggested_fix: ff.suggested_fix,
        });
    }

    let lawpack_dir = repo.join("lawpack/v2");
    if lawpack_dir.exists() {
        for ff in LawpackValidator::check_referential_integrity(&lawpack_dir, &lawpack)? {
            findings.push(Finding {
                severity: ff.severity,
                code: ff.code,
                path: ff.path,
                message: ff.message,
                citation: None,
                suggested_fix: ff.suggested_fix,
            });
        }
        // ACT-004:s8 (D2): citation uniqueness, collisions fatal.
        for ff in LawpackValidator::check_citation_uniqueness(&lawpack_dir)? {
            findings.push(Finding {
                severity: ff.severity,
                code: ff.code,
                path: ff.path,
                message: ff.message,
                citation: None,
                suggested_fix: ff.suggested_fix,
            });
        }
        // ACT-007:s7 (#2): the loaded law must hash to the pinned lock digest.
        if let Ok(Some(lock)) = Store::read_lawpack_lock(repo)
            && let Ok(computed) = compute_digest(repo)
            && lock.digest != computed
        {
            findings.push(
                f(
                    Severity::Fatal,
                    "LAWPACK_LOCK_DRIFT",
                    format!(
                        "Loaded lawpack does not hash to the pinned lock digest (ACT-007:s7). \
                         lock={} computed={}.",
                        lock.digest, computed
                    ),
                )
                .fix(
                    "Re-pin the lock (vjs invoke regenerates .vjs/lawpack.lock) only after \
                     confirming the lawpack change is intended.",
                ),
            );
        }
    }

    // --- Commit-gate (staged) checks ---
    let mut assented_record_paths: HashSet<String> = HashSet::new();
    if opts.staged {
        let changed = GitIntegration::read_staged_files(repo)?;
        if changed.is_empty() {
            findings.push(f(
                Severity::Info,
                "NO_STAGED_FILES",
                "No staged files to validate".into(),
            ));
        } else {
            // ONE deterministic git read of the committed tree, not a per-record subprocess:
            // whether a governed record is established at HEAD (an edit vs a fresh insertion)
            // is now a pure fact handed to the resolver. A genuine git failure surfaces here
            // as a loud error, never a silent per-record floor-strip (ACT-010 / REG-KERNEL-001).
            let head_set = GitIntegration::tracked_at_head(repo)?;
            for rel in &changed {
                // PC-16 D1: the floor shelters a staged record only if its declared
                // assent_source RESOLVES to real Sovereign authority - not merely names
                // an allow-listed form. A forged record that types the words but resolves
                // to nothing is left at its native severity.
                if vjs_core::front_door::is_governed_record(rel)
                    && let Ok(content) = std::fs::read_to_string(repo.join(rel))
                    && crate::assent::assent_resolves(repo, rel, &content, head_set.contains(rel))
                {
                    assented_record_paths.insert(rel.clone());
                }
            }
            findings.push(f(
                Severity::Info,
                "STAGED_FILES",
                format!("{} staged files", changed.len()),
            ));

            staged_gates(repo, &lawpack, &changed, &mut findings)?;
        }
    }

    if opts.external && GitIntegration::is_public_remote(repo)? {
        findings.push(
            f(
                Severity::Warning,
                "PUBLIC_REMOTE",
                "Repository has a public remote. Release warrant may be required.".into(),
            )
            .fix("Run vjs release-warrant check"),
        );
    }

    // Boundary scan over .vjs (always).
    let vjs_dir = repo.join(".vjs");
    if vjs_dir.exists() {
        let boundary = RedactScanner::scan_directory(&vjs_dir)?;
        if !RedactScanner::check_public_safe(&boundary) {
            for bf in boundary {
                if matches!(bf.severity, Severity::Fatal | Severity::Error) {
                    findings.push(Finding {
                        severity: bf.severity,
                        code: "BOUNDARY_VIOLATION".into(),
                        path: bf.path,
                        message: bf.message,
                        citation: None,
                        suggested_fix: Some(format!("{:?}", bf.suggested_route)),
                    });
                }
            }
        }
    }

    // PC-16 D4: the entrenched-enforcement-surface pin. A drift of a pinned gate-source
    // file from its lock is a loud, blocking finding - a weakening edit is never silent.
    findings.extend(vjs_core::enforcement::check_drift(repo));

    // Install-completeness + atomic manifest, scoped to --staged (the commit gate).
    if opts.staged {
        let mut defects = vjs_core::install::verify_surface(repo);
        defects.extend(vjs_core::install::verify_manifest(repo));
        for d in defects {
            findings.push(
                f(Severity::Fatal, d.code(), d.message())
                    .fix("Run vjs invoke --install-hooks, then vjs install-lock"),
            );
        }
        for hook in vjs_core::install::hook_tamper(repo) {
            findings.push(
                f(
                    Severity::Warning,
                    "HOOK_TAMPERED",
                    format!(
                        "Hook '.vjs/hooks/{hook}' does not match its pinned digest \
                         (REG-INSTALL-MANIFEST-001) - possible tamper or post-template-change drift."
                    ),
                )
                .fix(
                    "If the change is intended, re-lock with vjs install-lock; otherwise restore \
                     the hook (vjs invoke --install-hooks).",
                ),
            );
        }
    }

    // PC-14 D3 assent floor: every block on a staged record whose assent RESOLVES
    // (PC-16 D1) degrades to route-for-correction (never void/block,
    // ACT-ASSENTED-RECORD-PROTECTION) - EXCEPT a constitutive-validity finding, which
    // goes to whether the record IS a valid record of its kind and is never softened by
    // any assent claim (PC-16: "void ab initio on both grounds").
    if !assented_record_paths.is_empty() {
        for fd in &mut findings {
            if fd.is_blocking()
                && !crate::assent::is_constitutive(&fd.code)
                && let Some(p) = &fd.path
                && assented_record_paths.contains(&p.to_string_lossy().to_string())
            {
                fd.severity = Severity::Warning;
                fd.message = format!(
                    "[{}: assented record routed for correction, never blocked - ACT-ASSENTED-RECORD-PROTECTION] {}",
                    vjs_core::front_door::ROUTE_FOR_CORRECTION_CODE,
                    fd.message
                );
            }
        }
    }

    Ok(Report::from_findings(findings))
}

/// The gates that run only over a non-empty staged set.
fn staged_gates(
    repo: &Path,
    lawpack: &Lawpack,
    changed: &[String],
    findings: &mut Vec<Finding>,
) -> Result<(), KernelError> {
    use vjs_core::governance::PermitGate;

    // Apex-routing bright-line (REG-FEDERATION-COORDINATION-001).
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
    if let Some(vjs_core::hook::HookDecision::Block(bf)) =
        vjs_core::hook::apex_routing_decision(&apex_input, &jurisdiction_id, "vjs")
    {
        let mut fnd = f(Severity::Fatal, &bf.code, bf.message);
        fnd.suggested_fix = bf.next;
        findings.push(fnd);
    }

    // Invariants.
    let repo_state = RepoScanner::build_repo_state(repo)?;
    let facts = lawpack_facts(repo, lawpack);
    let invariant_report = evaluate_invariants(&repo_state, &lawpack.invariants, &facts)?;
    let mut any_invariant_failure = false;
    for finding in &invariant_report.findings {
        if !finding.passed {
            any_invariant_failure = true;
            findings.push(Finding {
                severity: finding.severity.clone(),
                code: finding.invariant_id.0.clone(),
                path: None,
                message: finding.message.clone(),
                citation: None,
                suggested_fix: Some(finding.remedy.clone()),
            });
        }
    }
    if !any_invariant_failure {
        findings.push(f(
            Severity::Info,
            "INVARIANTS_PASS",
            format!(
                "{} invariants evaluated, all passed",
                invariant_report.findings.len()
            ),
        ));
    }

    // Permit gate.
    let config = Store::read_repo_config(repo)?;
    let (permit_required, permit_exempt) = config
        .as_ref()
        .and_then(|c| c.governance.clone())
        .map(|g| (g.permit_required, g.permit_exempt))
        .unwrap_or_default();
    let staged_paths: Vec<PathBuf> = changed.iter().map(PathBuf::from).collect();
    let permits = Store::read_permits(repo)?;
    let logs = Store::read_logs(repo)?;
    let proofs = Store::read_proofs(repo)?;
    let gate = PermitGate::evaluate(
        &staged_paths,
        &permits,
        &logs,
        &proofs,
        &permit_required,
        &permit_exempt,
    );
    for pf in &gate.findings {
        findings.push(Finding {
            severity: pf.severity.clone(),
            code: pf.code.clone(),
            path: pf.path.clone(),
            message: pf.message.clone(),
            citation: None,
            suggested_fix: Some(pf.remedy.clone()),
        });
    }

    // Canon-write gate (D1).
    let canon_repo_code = config
        .as_ref()
        .and_then(|c| c.repo_code.clone())
        .or_else(|| config.as_ref().map(|c| c.jurisdiction_id.to_uppercase()))
        .unwrap_or_else(|| "VJS".into());
    let canon = RedactScanner::scan_canon_writes(repo, &staged_paths, &canon_repo_code);
    if !RedactScanner::check_public_safe(&canon) {
        for bf in canon {
            findings.push(Finding {
                severity: bf.severity,
                code: "CANON_BOUNDARY_VIOLATION".into(),
                path: bf.path,
                message: bf.message,
                citation: None,
                suggested_fix: Some(format!("{:?}", bf.suggested_route)),
            });
        }
    }

    // #7 media-file-in-canon (ACT-005:s1).
    for rel in changed {
        let low = rel.to_ascii_lowercase();
        let in_public = low.starts_with("lawpack/v2/") || low.starts_with("public/");
        let is_media = [
            ".png", ".jpg", ".jpeg", ".gif", ".bmp", ".webp", ".log", ".mp4", ".mov",
        ]
        .iter()
        .any(|e| low.ends_with(e));
        if in_public && is_media {
            findings.push(
                f(
                    Severity::Fatal,
                    "BOUNDARY_MEDIA_IN_CANON",
                    format!(
                        "'{rel}' is a screenshot/log/media file in a public record path \
                         (ACT-005:s1 must_not publish_screenshots / publish_logs)."
                    ),
                )
                .at(PathBuf::from(rel))
                .fix("Keep operational evidence in .vjs/private and reference it by pointer (ACT-005:s4)."),
            );
        }
    }

    // #6 destructive governed-record deletion (ACT-006:s4 / ACT-004:s9).
    if let Ok(deletions) = GitIntegration::read_staged_deletions(repo) {
        for del in deletions
            .iter()
            .filter(|d| vjs_core::front_door::is_governed_record(d))
        {
            findings.push(
                f(
                    Severity::Warning,
                    "DESTRUCTIVE_RECORD_DELETE",
                    format!(
                        "'{del}' is a governed record being DELETED - a destructive act \
                         (ACT-006:s4; ACT-004:s9). Confirm it is human-approved and authorised."
                    ),
                )
                .at(PathBuf::from(del))
                .fix("Route with --irreversible and record the authority before deleting a governed record."),
            );
        }
    }

    // D3 cross-repo permit reach.
    for (permit_id, glob) in PermitGate::cross_repo_reaches(&permits) {
        findings.push(
            f(
                Severity::Fatal,
                "CROSS_REPO_PERMIT",
                format!(
                    "Permit {permit_id} scopes '{glob}', which reaches outside the working root. \
                     A cross-repo reach into another repo's law is lawful only by a Privy Council \
                     order or Principal assent (ACT-007:s3). Failing closed."
                ),
            )
            .fix("Re-scope the permit to in-root paths, or seek privy/principal authority"),
        );
    }

    // D10 bench-integrity + D7 tier-floor + #5/#9/#10 order checks (staged orders).
    if let Some(constitution) = lawpack
        .orders
        .iter()
        .find(|o| o.id == "2026-VJS-COURTS-CONSTITUTION-001")
    {
        // PC-17 D1 corpus: every defined id (incl. section ids), every defined citation,
        // and the in-force subset (defined minus superseded). Computed once.
        let defined_ids = vjs_lawpack::defined_ids(lawpack);
        let defined_citations = vjs_lawpack::defined_citations(lawpack);
        let superseded = vjs_lawpack::superseded_ids(lawpack);
        let mut in_force: std::collections::HashSet<String> =
            defined_ids.difference(&superseded).cloned().collect();
        // A CITATION is in force when its owning record is in force (status binding/
        // in_force, not superseded). Without this a citation token - never an id - would
        // always read NOT_IN_FORCE, falsely flagging a reference to a binding order.
        let norm = |c: &str| c.split_whitespace().collect::<Vec<_>>().join(" ");
        let live = |st: &vjs_core::types::AuthorityStatus| {
            matches!(
                st,
                vjs_core::types::AuthorityStatus::Binding
                    | vjs_core::types::AuthorityStatus::InForce
            )
        };
        for o in &lawpack.orders {
            if let Some(c) = &o.citation
                && live(&o.status)
                && !superseded.contains(&o.id)
            {
                in_force.insert(norm(c));
            }
        }
        for s in &lawpack.statutes {
            if let Some(c) = &s.citation
                && live(&s.status)
            {
                in_force.insert(norm(c));
            }
        }
        for r in &lawpack.regulations {
            if let Some(c) = &r.citation
                && live(&r.status)
            {
                in_force.insert(norm(c));
            }
        }

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
            // #5 ACT-002:s10 well-formedness.
            for (field, is_empty) in [
                ("holding", order.holding.is_empty()),
                ("directives", order.directives.is_empty()),
                ("runtime_summary", order.runtime_summary.is_empty()),
            ] {
                if is_empty {
                    findings.push(
                        f(
                            Severity::Fatal,
                            "ORDER_MALFORMED",
                            format!(
                                "Order is missing required '{field}' (ACT-002:s10: orders bind, \
                                 and must state their holding, directives, and runtime summary)."
                            ),
                        )
                        .at(PathBuf::from(rel))
                        .fix("Add the missing field before recording the order."),
                    );
                }
            }
            // PC-17 D1-D5: citation-grounding over the order's OPERATIVE parts (holding +
            // each directive's must + each forbidden clause). Existence-only; never reads
            // what the cited authority says (D7/D8). Self-reference (D4(c)): seed the
            // order's own id + citation so a forward self-reference resolves to itself.
            {
                let mut operative = order.holding.clone();
                for d in &order.directives {
                    operative.push('\n');
                    operative.push_str(&d.must);
                }
                if let Some(forbidden) = &order.forbidden {
                    for clause in forbidden {
                        operative.push('\n');
                        operative.push_str(clause);
                    }
                }
                // D7: explicitly-listed operative authorities (the machine-resolvable
                // mechanism that extends the teeth past lossy snake_case directive bodies).
                if let Some(cites) = &order.cites_authorities {
                    for a in cites {
                        operative.push('\n');
                        operative.push_str(a);
                    }
                }
                let mut defined_self = defined_ids.clone();
                defined_self.insert(order.id.clone());
                let mut cites_self = defined_citations.clone();
                let mut in_force_self = in_force.clone();
                in_force_self.insert(order.id.clone());
                if let Some(c) = &order.citation {
                    let norm = c.split_whitespace().collect::<Vec<_>>().join(" ");
                    cites_self.insert(norm.clone());
                    in_force_self.insert(norm);
                }
                for (tok, grounding) in vjs_lawpack::refs::ground_operative(
                    &operative,
                    &defined_self,
                    &cites_self,
                    &in_force_self,
                ) {
                    match grounding {
                        // D2: Fatal but CORRECTABLE (not constitutive) - the existing
                        // assent floor routes it for correction on a resolving order and
                        // leaves it Fatal otherwise. The finding states the existence
                        // fact; it never brands the order void (per-incuriam voidness is
                        // for a court on appeal, REG-KERNEL-001 clerk-not-court).
                        vjs_lawpack::refs::Grounding::Unresolved => findings.push(
                            f(
                                Severity::Fatal,
                                "ORDER_CITATION_UNRESOLVED",
                                format!(
                                    "Operative citation '{tok}' resolves to no defined authority \
                                     (per-incuriam existence limb; ACT-002:s7, REG-KERNEL-001). \
                                     Routed for correction, not voided."
                                ),
                            )
                            .at(PathBuf::from(rel))
                            .citing("ACT-002:s7")
                            .fix("Correct the citation to a defined authority, or remove it."),
                        ),
                        // D3: defined-but-not-in-force is advisory only, never blocks.
                        vjs_lawpack::refs::Grounding::NotInForce => findings.push(
                            f(
                                Severity::Warning,
                                "ORDER_CITATION_NOT_IN_FORCE",
                                format!(
                                    "Operative citation '{tok}' resolves to a defined but \
                                     superseded/spent authority (existence satisfied; aptness of \
                                     relying on historical law is out of scope, PC-17 D3)."
                                ),
                            )
                            .at(PathBuf::from(rel)),
                        ),
                        vjs_lawpack::refs::Grounding::Resolved => {}
                    }
                }
            }
            // #9 subject-tier advisory.
            if let Some(msg) = vjs_core::bench::subject_tier_advisory(&order.issue.0, &order.court)
            {
                findings.push(
                    f(Severity::Warning, "TIER_ADVISORY", msg)
                        .at(PathBuf::from(rel))
                        .fix(
                            "Confirm the court tier, or re-file the matter at the indicated tier.",
                        ),
                );
            }
            // #10 apex order must declare its bench.
            if matches!(
                order.court,
                vjs_core::types::Court::PrivyCouncil | vjs_core::types::Court::SupremeCourt
            ) && order.bench.is_empty()
            {
                findings.push(
                    f(
                        Severity::Fatal,
                        "BENCH_REQUIRED",
                        format!(
                            "A {:?} order must declare its constituted bench ([2026] VJS-SC 2: no \
                             court may issue an order until constituted).",
                            order.court
                        ),
                    )
                    .at(PathBuf::from(rel))
                    .fix("Add the bench (the seats that decided) before recording."),
                );
            }
            // D10/D7 bench-integrity. PC-16: bench-integrity is CONSTITUTIVE - whether an
            // order issues from a constituted bench goes to whether it IS an order at all
            // ("void ab initio on both grounds"), and is NEVER softened by an assent
            // claim. The former code downgraded these on mere allow-list membership, the
            // very laundering [2026] VJS-PC 16 closed. A genuinely-constituted order has
            // no such defect, so always-Fatal narrows no real order.
            let opinion_text = order
                .source_opinion
                .as_ref()
                .and_then(|p| std::fs::read_to_string(repo.join(p)).ok());
            let defects =
                vjs_core::bench::verify_bench(&order, constitution, opinion_text.as_deref());
            for d in defects {
                findings.push(
                    f(Severity::Fatal, d.code(), d.message())
                        .at(PathBuf::from(rel))
                        .fix(
                            "Constitute the bench correctly (constituted odd size + a non-empty \
                             opinion per seat) before recording.",
                        ),
                );
            }
        }
    }

    Ok(())
}
