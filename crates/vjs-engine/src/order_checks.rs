//! The per-order checks, applied to ONE governed record: well-formedness (ACT-002:s10),
//! a non-empty issue, citation grounding (PC-17 D1-D5), the subject-tier advisory, the
//! apex bench requirement, and bench-integrity (PC-16, constitutive).
//!
//! Lifted out of `staged.rs` under [2026] VJS-CC-VJS 20 D13, which requires these to run
//! AT REST and not only over a staged set. The court sequenced D13 last and expressly
//! after D10, because until BENCH_OPINION_MISSING stopped being constitutive a sweep at
//! rest would have turned thirteen orders PC-21 holds are in force into Fatals.
//!
//! ONE body, two callers. The commit gate passes the staged paths; the at-rest sweep
//! passes every governed record on disk. Two copies of a rule is one copy and a
//! disagreement ([2026] VJS-CC-VJS 12), and a gate that judges a record differently
//! depending on which door it came through is exactly that.

use std::path::{Path, PathBuf};

use vjs_core::report::Finding;
use vjs_core::types::Severity;

use super::f;
use crate::grounding::GroundingCorpus;

/// Every finding this record earns. Returns empty for a path that is not a readable,
/// parseable order - a convening, a non-order instrument, or an unreadable file - so
/// the caller never has to know which governed records are orders.
pub(crate) fn order_findings(
    repo: &Path,
    rel: &str,
    constitution: &vjs_core::types::Order,
    corpus: &GroundingCorpus,
) -> Vec<Finding> {
    let mut out = Vec::new();
    let GroundingCorpus {
        defined: defined_ids,
        citations: defined_citations,
        in_force,
    } = corpus;
    let Ok(content) = std::fs::read_to_string(repo.join(rel)) else {
        return out;
    };
    let Ok(order) = serde_yaml::from_str::<vjs_core::types::Order>(&content) else {
        return out;
    };
    // #5 ACT-002:s10 well-formedness.
    for (field, is_empty) in [
        ("holding", order.holding.is_empty()),
        ("directives", order.directives.is_empty()),
        ("runtime_summary", order.runtime_summary.is_empty()),
    ] {
        if is_empty {
            out.push(
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
    // ACT-PROCEEDINGS-DISCIPLINE:s3: an order with an empty issue is unroutable -
    // the one-live-order-per-issue discipline cannot even ask its question of it.
    // (An order OMITTING the field entirely fails the parse above and rides the
    // existing skip; this refuses the present-but-empty form that parsed fine.)
    if order.issue.0.trim().is_empty() {
        out.push(
            f(
                Severity::Fatal,
                "ORDER_MALFORMED",
                "Order carries an empty 'issue' (ACT-PROCEEDINGS-DISCIPLINE:s3: an \
                 order binds on an issue, and the one-live-order-per-issue \
                 discipline cannot rank an order that names none)."
                    .into(),
            )
            .at(PathBuf::from(rel))
            .fix("Set the order's lower_snake issue tag before recording."),
        );
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
                vjs_lawpack::refs::Grounding::Unresolved => out.push(
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
                vjs_lawpack::refs::Grounding::NotInForce => out.push(
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
    if let Some(msg) = vjs_core::bench::subject_tier_advisory(&order.issue.0, &order.court) {
        out.push(
            f(Severity::Warning, "TIER_ADVISORY", msg)
                .at(PathBuf::from(rel))
                .fix("Confirm the court tier, or re-file the matter at the indicated tier."),
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
        out.push(
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
    let defects = vjs_core::bench::verify_bench(&order, constitution, opinion_text.as_deref());
    for d in defects {
        out.push(
            f(Severity::Fatal, d.code(), d.message())
                .at(PathBuf::from(rel))
                .fix(
                    "Constitute the bench correctly (constituted odd size + a non-empty \
                     opinion per seat) before recording.",
                ),
        );
    }
    out
}

/// THE ORDER GATES AT REST ([2026] VJS-CC-VJS 20 D13).
///
/// The checks above ran only over a STAGED set, so a record that was committed once
/// was never looked at again. A defect introduced before the gate existed, or by a
/// commit that touched a different path, stayed invisible forever. "An unrun gate
/// banks debt": the corpus was accumulating exactly the defects these checks describe
/// and reporting itself clean, because nothing ever asked the question of a record at
/// rest.
///
/// The court sequenced this LAST and expressly after D10 ("D13 does not land before
/// D10. If it does, plain `vjs validate` turns Fatal on thirteen orders PC-21 holds
/// are in force"), which is why it lands now and not on 2026-08-06.
///
/// SEVERITY IS DELIBERATELY NOT THE STAGED SEVERITY. A record being WRITTEN makes a
/// claim about itself and a Fatal is the right answer to a false one. A record AT REST
/// is already in force and relied upon; its defects are the correction register's
/// business (PC 21 D2/D3), not a reason to refuse every future commit in the
/// jurisdiction until somebody repairs history. So every finding here is reported at
/// Warning and named `AT_REST_`, and the register is what carries the obligation.
///
/// That is not a softening of the staged gate, which is untouched. It is the
/// difference between "you may not write this" and "you have this", and conflating
/// them is how a gate gets switched off by the first person who cannot land a commit.
pub fn at_rest_order_findings(repo: &Path, lawpack: &vjs_lawpack::Lawpack) -> Vec<Finding> {
    at_rest_order_findings_raw(repo, lawpack)
        .into_iter()
        .map(|mut f| {
            f.severity = Severity::Warning;
            f
        })
        .collect()
}

/// The same sweep with the STAGED severities intact.
///
/// Two views of one walk, because two callers need different halves of the same fact
/// and neither may re-derive it. `validate` needs the downgrade: a record at rest is in
/// force and its defects must not refuse every future commit. The correction register
/// needs the original, because an OBLIGATION is precisely a finding that would refuse
/// the record if you wrote it today - Fatal at the staged door - and an advisory is not
/// one. TIER_ADVISORY says "possibly under-tiered" and ORDER_CITATION_NOT_IN_FORCE is
/// held by PC-17 D3 to be "advisory only, never blocks"; a register carrying those
/// becomes a list of opinions rather than a ledger of what is owed.
///
/// Deriving the split from severity rather than from a list of codes is deliberate: a
/// future check joins the right side of the line by being written correctly, and no
/// enumeration goes stale behind it.
pub fn at_rest_order_findings_raw(repo: &Path, lawpack: &vjs_lawpack::Lawpack) -> Vec<Finding> {
    let Some(constitution) = lawpack
        .orders
        .iter()
        .find(|o| o.id == vjs_core::bench::COURTS_CONSTITUTION_ID)
    else {
        // No constitution means no bench can be verified against anything. Disclose it
        // rather than sweep and report a clean result the sweep did not earn.
        return vec![f(
            Severity::Info,
            "AT_REST_ORDERS_UNCHECKED",
            "the at-rest order sweep DID NOT RUN - this lawpack carries no courts \
             constitution to verify a bench against. A statement about this estate, \
             never a finding that its orders are sound ([2026] VJS-CC-VJS 20 D13)."
                .into(),
        )];
    };
    let corpus = crate::grounding::grounding_corpus(repo, lawpack);

    // TWO FILES SHARING A RECORD ID ARE ONE RECORD
    // ([2026] VJS-CC-RECORD-PROJECTION-009 D2/D4, applied by CC-VJS 20: "collapse two
    // files sharing a record id into one record before counting any per-record duty").
    //
    // This sweep walks FILES, and the corpus deliberately keeps projections - the same
    // order filed under `.vjs/court/orders` and overlaid from `.vjs/orders`. The first
    // version of this function reported per file and so counted two records twice: 15
    // findings over 13 records, against a correction register that correctly holds 13
    // rows. A register and a gate that disagree by two is a register nobody can reconcile,
    // and the disagreement would have been read as two unrecorded obligations rather than
    // as double vision.
    let mut seen: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();
    let mut out = Vec::new();
    for root in vjs_core::front_door::governed_record_roots(repo) {
        for entry in walkdir::WalkDir::new(&root).into_iter().flatten() {
            let path = entry.path();
            match path.extension().and_then(|e| e.to_str()) {
                Some("yaml") | Some("yml") => {}
                _ => continue,
            }
            let rel = path
                .strip_prefix(repo)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();
            if !vjs_core::front_door::is_governed_record(&rel) {
                continue;
            }
            let record_id = std::fs::read_to_string(path)
                .ok()
                .and_then(|t| {
                    t.lines().find_map(|l| {
                        let v = l.strip_prefix("id:")?.trim();
                        let v = v.trim_matches('"').trim_matches('\'').trim();
                        (!v.is_empty()).then(|| v.to_string())
                    })
                })
                // A record with no readable id cannot be collapsed with anything, so it
                // keys on its own path and is reported once, which is correct.
                .unwrap_or_else(|| rel.clone());
            for mut fnd in order_findings(repo, &rel, constitution, &corpus) {
                fnd.code = format!("AT_REST_{}", fnd.code);
                if !seen.insert((record_id.clone(), fnd.message.clone())) {
                    continue;
                }
                out.push(fnd);
            }
        }
    }
    out
}
