//! [2026] VJS-PC 15 (the runtime-overlay keystone) coverage.
//!
//! D7 (the canon-clean scan): a deterministic check that no subscriber dimension
//! VOCABULARY is hard-coded into canon as a scope key - the substrate stays generic,
//! the tenant's business hierarchy lives only in its Tier-2 (ACT-007:s4). Plus the two
//! behaviours the keystone turns on, measured through the public API rather than by
//! review-by-vibes: the loader voids a relaxing local rule (D1/D2), and submit-decision
//! honours the assent bifurcation (D5/D6, VJS-ACT 10).

use std::path::{Path, PathBuf};

use vjs_core::scope::{EntityScope, Floor};
use vjs_engine::runtime::{DecisionEnvelope, Disposition, submit_decision};
use vjs_lawpack::overlay::{Overlay, OverlayLoader};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The five names PC-15 named ONLY to exclude. Canon must hard-code none as a scope
/// key. They are checked as QUOTED strings (the form a hard-coded dimension key takes);
/// ordinary prose that names them to exclude them is unquoted and does not trip this.
/// Built from an unquoted, slash-joined source so this scanner's own file carries no
/// quoted token to match itself on.
fn excluded_dimension_keys() -> Vec<String> {
    "org/ws/matter/flow/step"
        .split('/')
        .map(|n| format!("\"{n}\""))
        .collect()
}

fn walk_collect(dir: &Path, exts: &[&str], out: &mut Vec<PathBuf>) {
    if !dir.exists() {
        return;
    }
    for entry in std::fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            // skip build artefacts
            if path.file_name().and_then(|s| s.to_str()) == Some("target") {
                continue;
            }
            walk_collect(&path, exts, out);
        } else if path
            .extension()
            .and_then(|s| s.to_str())
            .map(|ext| exts.contains(&ext))
            .unwrap_or(false)
        {
            out.push(path);
        }
    }
}

#[test]
fn canon_hard_codes_no_subscriber_dimension_vocabulary() {
    let root = workspace_root();
    let mut files = Vec::new();
    walk_collect(&root.join("crates"), &["rs"], &mut files);
    walk_collect(&root.join("lawpack"), &["yaml"], &mut files);

    let keys = excluded_dimension_keys();
    let mut offences = Vec::new();
    for f in &files {
        let content = std::fs::read_to_string(f).unwrap_or_default();
        for key in &keys {
            if content.contains(key.as_str()) {
                offences.push(format!("{} hard-codes {} as a scope key", f.display(), key));
            }
        }
    }
    assert!(
        offences.is_empty(),
        "PC-15 D7: canon must hard-code no subscriber dimension vocabulary. Offences:\n{}",
        offences.join("\n")
    );
}

#[test]
fn entity_scope_is_a_generic_named_dimension_frame() {
    // The substrate ships the empty frame: an ordered map of opaque (dimension, value)
    // pairs, cascade by prefix. No business noun is a field. (If a future edit added a
    // business-named field this test would not compile against the literal below.)
    let s = EntityScope::new(vec![("a".into(), "1".into()), ("b".into(), "2".into())]);
    assert_eq!(s.specificity(), 2);
    assert!(EntityScope::root().covers(&s));
}

#[test]
fn overlay_loader_voids_a_relaxing_local_rule_with_a_named_defect() {
    // A canon floor forbids 'charge'; a Tier-2 rule that grants it (no authority) is
    // void at load - only that rule, with a named OVERLAY_RELAXATION_VOID defect.
    let dir = workspace_root().join("crates/vjs-testkit/tests/_tmp_overlay");
    let floors = dir.join("floors");
    let local = dir.join("local");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&floors).unwrap();
    std::fs::create_dir_all(&local).unwrap();
    std::fs::write(
        floors.join("f.yaml"),
        "id: FLOOR-1\nscope:\n  dims: []\nforbids:\n  - charge\n",
    )
    .unwrap();
    std::fs::write(
        local.join("r.yaml"),
        "id: LOCAL-RELAX\nscope:\n  dims:\n    - [\"a\", \"1\"]\ngrants:\n  - charge\n",
    )
    .unwrap();

    let (overlay, findings) = OverlayLoader::load(&floors, &local).unwrap();
    // The relaxing rule is not kept.
    assert!(overlay.local.is_empty(), "the relaxing rule must be voided, not loaded");
    assert!(
        findings.iter().any(|f| f.code == "OVERLAY_RELAXATION_VOID"
            && f.citation.as_deref() == Some("ACT-007:s3")),
        "the void must be a named defect citing ACT-007:s3"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn submit_decision_never_hard_denies_an_assented_act() {
    let overlay = Overlay {
        floors: vec![Floor {
            id: "FLOOR-1".into(),
            scope: EntityScope::root(),
            forbids: vec!["charge".into()],
        }],
        local: vec![],
    };
    // Un-assented breach: DENY with the named instrument.
    let denied = submit_decision(
        &overlay,
        &DecisionEnvelope {
            scope: EntityScope::root(),
            verb: "charge".into(),
            assent_source: None,
        },
    );
    assert_eq!(denied.disposition, Disposition::Deny);
    assert_eq!(denied.instrument.as_deref(), Some("FLOOR-1"));
    // Assented breach: never hard-DENIED - routed for correction (VJS-ACT 10).
    let routed = submit_decision(
        &overlay,
        &DecisionEnvelope {
            scope: EntityScope::root(),
            verb: "charge".into(),
            assent_source: Some("sovereign_assent".into()),
        },
    );
    assert_eq!(routed.disposition, Disposition::RouteForCorrection);
}
