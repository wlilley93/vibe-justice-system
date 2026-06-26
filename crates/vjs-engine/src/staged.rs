//! The staged (commit-gate) checks: the gates that run only over a non-empty staged set -
//! apex-routing, invariants, the permit gate, the canon-write/media/destructive-delete gates, the
//! cross-repo reach, and the bench/tier/order-grounding checks over staged orders. Lifted out of
//! `validate` verbatim; the test suite is the net.

use std::path::{Path, PathBuf};

use vjs_core::report::Finding;
use vjs_core::types::Severity;
use vjs_core::{KernelError, RepoScanner, evaluate_invariants};
use vjs_git::GitIntegration;
use vjs_lawpack::{Lawpack, lawpack_facts};
use vjs_redact::RedactScanner;
use vjs_store::Store;

use super::f;

/// The gates that run only over a non-empty staged set.
pub(crate) fn staged_gates(
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

    // #7 media-file-in-canon (ACT-005:s1). Extracted to media_in_canon_findings so this
    // Fatal gate has a direct behavioral test (it is a pinned enforcement-surface gate).
    findings.extend(media_in_canon_findings(changed));

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
                vjs_core::types::Court::CourtOfAppeal
                    | vjs_core::types::Court::PrivyCouncil
                    | vjs_core::types::Court::SupremeCourt
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

/// ACT-005:s1: a screenshot / log / media file staged into a public record path must not be
/// published; that is a Fatal canon-boundary violation. Pure over the changed-path list, so it
/// is directly testable (the goal-completion audit, 2026-06-26, found this inline gate had no
/// behavioral test while living in an unpinned file).
pub(crate) fn media_in_canon_findings(changed: &[String]) -> Vec<Finding> {
    let mut out = Vec::new();
    for rel in changed {
        let low = rel.to_ascii_lowercase();
        let in_public = low.starts_with("lawpack/v2/") || low.starts_with("public/");
        let is_media = [
            ".png", ".jpg", ".jpeg", ".gif", ".bmp", ".webp", ".log", ".mp4", ".mov",
        ]
        .iter()
        .any(|e| low.ends_with(e));
        if in_public && is_media {
            out.push(
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
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boundary_media_in_canon_fires_fatal() {
        // A screenshot/log/media file in a public canon path is a Fatal BOUNDARY_MEDIA_IN_CANON.
        let flagged = media_in_canon_findings(&[
            "lawpack/v2/orders/2026-VJS-PC-099-screenshot.png".to_string(),
            "public/evidence.log".to_string(),
        ]);
        assert_eq!(
            flagged.len(),
            2,
            "both media-in-canon paths must be flagged"
        );
        assert!(
            flagged
                .iter()
                .all(|fnd| fnd.code == "BOUNDARY_MEDIA_IN_CANON"
                    && matches!(fnd.severity, Severity::Fatal)),
            "each must be a Fatal BOUNDARY_MEDIA_IN_CANON: {:?}",
            flagged.iter().map(|fnd| &fnd.code).collect::<Vec<_>>()
        );
        // A yaml order is not media; a media file OUTSIDE the public canon path is not flagged.
        let clean = media_in_canon_findings(&[
            "lawpack/v2/orders/2026-VJS-PC-099.yaml".to_string(),
            ".vjs/private/screenshot.png".to_string(),
        ]);
        assert!(
            clean.is_empty(),
            "a yaml canon record and a .vjs/private media file must not be flagged: {:?}",
            clean.iter().map(|fnd| &fnd.code).collect::<Vec<_>>()
        );
    }
}
