//! The PC-17 grounding corpus: every defined id, every defined citation, and the
//! in-force subset - built ONCE and shared by every door that grounds citations.
//!
//! Extracted verbatim from the staged commit gate (Operation Watertight WS1) so
//! `vjs draft check` grounds a draft's citations against the SAME sets the commit
//! gate uses. Two copies of a resolver are one copy and one silent disagreement
//! ([2026] VJS-CC-VJS 12); this module exists so the draft clerk and the commit
//! gate can never drift apart.

use std::collections::HashSet;
use std::path::Path;

use vjs_lawpack::Lawpack;

/// The three sets `vjs_lawpack::refs::ground` takes, computed over the lawpack
/// PLUS every governed record root (CC-VJS 9: an instrument reasoning about the
/// citation register reads the REGISTER, not one root of it).
pub struct GroundingCorpus {
    pub defined: HashSet<String>,
    pub citations: HashSet<String>,
    pub in_force: HashSet<String>,
}

pub fn grounding_corpus(repo: &Path, lawpack: &Lawpack) -> GroundingCorpus {
    // PC-17 D1 corpus: every defined id (incl. section ids), every defined citation,
    // and the in-force subset (defined minus superseded). Computed once.
    let defined_ids = vjs_lawpack::defined_ids(lawpack);
    let mut defined_citations = vjs_lawpack::defined_citations(lawpack);
    // [2026] VJS-CC-VJS 9: an instrument that reasons about the citation register must
    // read the REGISTER, not one root of it. `defined_citations` walks the lawpack
    // alone, so a County order citing another County order always resolved to "no
    // defined authority" and was reported per incuriam, although the cited order
    // exists, is binding, and is what the allocator itself counts. Union in every
    // governed record's own top-level citation, from the same `governed_record_roots`
    // the allocator uses.
    //
    // Widening a DEFINEDNESS set is monotone: it can only make more citations resolve,
    // never fewer, so it cannot introduce a finding.
    // Collected once: (citation, is_live). The status matters as much as the
    // existence, because a defined-but-not-in-force citation is reported as
    // "superseded/spent". Reading the register for existence and NOT for status would
    // have the gate announce that a binding County order is spent, which is a false
    // statement by an instrument whose whole job is to be believed.
    let mut governed_citations: Vec<(String, bool)> = Vec::new();
    // RECURSE: `governed_record_roots` yields `.vjs/court`, not `.vjs/court/orders`,
    // so a flat read_dir sees only subdirectories and no record.
    for root in vjs_core::front_door::governed_record_roots(repo) {
        for entry in walkdir::WalkDir::new(&root)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path().to_path_buf();
            if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&path) else {
                continue;
            };
            let val = |key: &str| {
                text.lines().find_map(|l| {
                    l.strip_prefix(key).map(|r| {
                        r.trim()
                            .trim_matches('"')
                            .trim_matches('\'')
                            .trim()
                            .to_string()
                    })
                })
            };
            if let Some(c) = val("citation:").filter(|c| !c.is_empty()) {
                let live = matches!(
                    val("status:").as_deref(),
                    Some("binding") | Some("in_force")
                );
                governed_citations.push((c.split_whitespace().collect::<Vec<_>>().join(" "), live));
            }
        }
    }
    for (c, _) in &governed_citations {
        defined_citations.insert(c.clone());
    }
    let defined_citations = defined_citations;
    let superseded = vjs_lawpack::superseded_ids(lawpack);
    let mut in_force: HashSet<String> = defined_ids.difference(&superseded).cloned().collect();
    // A CITATION is in force when its owning record is in force (status binding/
    // in_force, not superseded). Without this a citation token - never an id - would
    // always read NOT_IN_FORCE, falsely flagging a reference to a binding order.
    let norm = |c: &str| c.split_whitespace().collect::<Vec<_>>().join(" ");
    let live = |st: &vjs_core::types::AuthorityStatus| {
        matches!(
            st,
            vjs_core::types::AuthorityStatus::Binding | vjs_core::types::AuthorityStatus::InForce
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
    // Same register, same reason (CC-VJS 9). Monotone: adding in-force citations can
    // only downgrade a warning.
    for (c, is_live) in &governed_citations {
        if *is_live {
            in_force.insert(c.clone());
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
    GroundingCorpus {
        defined: defined_ids,
        citations: defined_citations,
        in_force,
    }
}
