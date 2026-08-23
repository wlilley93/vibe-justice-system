//! The permit gate fails closed. These tests pin the enforcement paths the
//! gate actually blocks on: status (expired / revoked / closed), the expiry
//! timestamp (including the unparseable case), before-commit obligations
//! (decision log, proof), and the path-boundary semantics of glob matching
//! (a literal scope must not cover sibling files via substring prefix).

use std::path::PathBuf;
use vjs_core::governance::{PathClassifier, PermitGate};
use vjs_core::spec::{Permit, Proof};
use vjs_core::types::*;

fn permit(scope_paths: Vec<&str>) -> Permit {
    Permit {
        id: PermitId("PERMIT-TEST".into()),
        route_id: RouteId("ROUTE-TEST".into()),
        actor: "lexby".into(),
        scope: Some(Scope {
            paths: Some(scope_paths.into_iter().map(|s| s.to_string()).collect()),
            jurisdictions: None,
            action_kinds: None,
            issue_tags: None,
            records: None,
        }),
        obligations: Vec::new(),
        expires_at: "2099-01-01T00:00:00+00:00".into(),
        status: PermitStatus::Active,
        self_issued: true,
        meaning: None,
        intent_digest: None,
        law_source: Vec::new(),
    }
}

fn obligation(kind: ObligationKind) -> Obligation {
    Obligation {
        id: ObligationId("OBL-TEST".into()),
        kind,
        required: true,
        due: ObligationDue::BeforeCommit,
        description: "test obligation".into(),
    }
}

fn gate(
    staged: &[&str],
    permits: &[Permit],
    logs: &[DecisionLog],
    proofs: &[Proof],
) -> vjs_core::governance::PermitGateResult {
    let staged: Vec<PathBuf> = staged.iter().map(PathBuf::from).collect();
    let required = vec![
        "lawpack/**".to_string(),
        "crates/**".to_string(),
        "Cargo.toml".to_string(),
    ];
    PermitGate::evaluate(&staged, permits, logs, proofs, &required, &[])
}

fn has_finding(result: &vjs_core::governance::PermitGateResult, code: &str) -> bool {
    result.findings.iter().any(|f| f.code == code)
}

// --- status enforcement -----------------------------------------------------

#[test]
fn an_expired_status_permit_is_refused_with_the_precise_finding() {
    let mut p = permit(vec!["crates/**"]);
    p.status = PermitStatus::Expired;
    let r = gate(&["crates/vjs-core/src/route.rs"], &[p], &[], &[]);
    assert!(!r.ok);
    assert!(
        has_finding(&r, "PERMIT-EXPIRED"),
        "expired permit must surface PERMIT-EXPIRED, not PERMIT-MISSING"
    );
}

#[test]
fn a_revoked_permit_is_refused_with_the_precise_finding() {
    let mut p = permit(vec!["crates/**"]);
    p.status = PermitStatus::Revoked;
    let r = gate(&["crates/vjs-core/src/route.rs"], &[p], &[], &[]);
    assert!(!r.ok);
    assert!(has_finding(&r, "PERMIT-REVOKED"));
}

#[test]
fn a_closed_permit_does_not_excuse_new_staged_changes() {
    let mut p = permit(vec!["crates/**"]);
    p.status = PermitStatus::Closed;
    let r = gate(&["crates/vjs-core/src/route.rs"], &[p], &[], &[]);
    assert!(
        !r.ok,
        "a closed permit covering new work would also skip its obligations - a bypass"
    );
    assert!(has_finding(&r, "PERMIT-CLOSED"));
}

#[test]
fn an_active_unexpired_permit_is_preferred_over_a_dead_one_covering_the_same_path() {
    let mut dead = permit(vec!["crates/**"]);
    dead.status = PermitStatus::Closed;
    let live = permit(vec!["crates/**"]);
    let r = gate(&["crates/vjs-core/src/route.rs"], &[dead, live], &[], &[]);
    assert!(
        r.ok,
        "a live permit must win even when a dead permit also covers the path"
    );
}

// --- expiry timestamp enforcement (fail closed) ------------------------------

#[test]
fn a_timestamp_expired_active_permit_is_refused() {
    let mut p = permit(vec!["crates/**"]);
    p.expires_at = "2001-01-01T00:00:00+00:00".into();
    let r = gate(&["crates/vjs-core/src/route.rs"], &[p], &[], &[]);
    assert!(!r.ok);
    assert!(has_finding(&r, "PERMIT-EXPIRED"));
}

#[test]
fn an_unparseable_expiry_never_excuses_a_write() {
    let mut p = permit(vec!["crates/**"]);
    p.expires_at = "not-a-timestamp".into();
    let r = gate(&["crates/vjs-core/src/route.rs"], &[p], &[], &[]);
    assert!(
        !r.ok,
        "fail closed: a corrupt expiry must not grant indefinite validity"
    );
    assert!(has_finding(&r, "PERMIT-EXPIRED"));
}

// --- before-commit obligations -----------------------------------------------

#[test]
fn a_decision_log_obligation_blocks_commit_until_a_log_exists() {
    let mut p = permit(vec!["crates/**"]);
    p.obligations.push(obligation(ObligationKind::DecisionLog));

    let r = gate(
        &["crates/vjs-core/src/route.rs"],
        std::slice::from_ref(&p),
        &[],
        &[],
    );
    assert!(!r.ok);
    assert!(has_finding(&r, "PERMIT-OBLIGATION-MISSING"));

    let log = DecisionLog {
        id: format!("LOG-{}", p.id.0),
        time: "2026-06-09T00:00:00+00:00".into(),
        actor: "lexby".into(),
        kind: "decision".into(),
        issue: "test".into(),
        decision: "do the work".into(),
        basis: vec![p.id.0.clone()],
        risk: RiskLevel::Low,
        reversibility: "reversible".into(),
        court_required: false,
        why: "test".into(),
    };
    let r = gate(&["crates/vjs-core/src/route.rs"], &[p], &[log], &[]);
    assert!(r.ok, "a log citing the permit satisfies the obligation");
}

#[test]
fn a_proof_obligation_blocks_commit_until_a_proof_exists() {
    let mut p = permit(vec!["crates/**"]);
    p.obligations.push(obligation(ObligationKind::Proof));

    let r = gate(
        &["crates/vjs-core/src/route.rs"],
        std::slice::from_ref(&p),
        &[],
        &[],
    );
    assert!(!r.ok);
    assert!(has_finding(&r, "PERMIT-PROOF-MISSING"));

    let proof = Proof {
        id: ProofId("PROOF-TEST".into()),
        permit_id: p.id.clone(),
        kind: ProofKind::TestResult,
        status: ProofStatus::Passed,
        digest: None,
        captured_at: "2026-06-09T00:00:00+00:00".into(),
    };
    let r = gate(&["crates/vjs-core/src/route.rs"], &[p], &[], &[proof]);
    assert!(
        r.ok,
        "a proof attached to the permit satisfies the obligation"
    );
}

// --- glob boundary semantics --------------------------------------------------

#[test]
fn a_literal_scope_does_not_cover_sibling_files_by_prefix() {
    let p = permit(vec!["Cargo.toml"]);
    let r = gate(&["Cargo.toml"], std::slice::from_ref(&p), &[], &[]);
    assert!(r.ok, "the named file itself is covered");
    // Cargo.toml.bak is ungoverned under the required globs, so prove the
    // bypass at the matcher itself: the literal scope must not cover it.
    assert!(!PathClassifier::glob_matches(
        "Cargo.toml",
        "Cargo.toml.bak"
    ));
    assert!(!PathClassifier::glob_matches("README.md", "README.md.old"));
}

#[test]
fn a_recursive_glob_respects_the_path_boundary() {
    assert!(PathClassifier::glob_matches(
        "crates/**",
        "crates/vjs-core/src/lib.rs"
    ));
    assert!(PathClassifier::glob_matches("crates/**", "crates"));
    assert!(!PathClassifier::glob_matches(
        "crates/**",
        "crates-evil/src/lib.rs"
    ));
    assert!(!PathClassifier::glob_matches(
        "lawpack/**",
        "lawpack2/x.yaml"
    ));
}

#[test]
fn an_infix_recursive_glob_matches_zero_or_more_dirs_on_boundaries() {
    assert!(PathClassifier::glob_matches(
        "crates/**/Cargo.toml",
        "crates/vjs-core/Cargo.toml"
    ));
    assert!(PathClassifier::glob_matches(
        "crates/**/Cargo.toml",
        "crates/Cargo.toml"
    ));
    assert!(!PathClassifier::glob_matches(
        "crates/**/Cargo.toml",
        "crates/vjs-core/NotCargo.toml"
    ));
    assert!(!PathClassifier::glob_matches(
        "crates/**/Cargo.toml",
        "crates2/x/Cargo.toml"
    ));
}

#[test]
fn a_single_star_stays_within_one_path_segment() {
    assert!(PathClassifier::glob_matches(
        "crates/*/Cargo.toml",
        "crates/vjs-core/Cargo.toml"
    ));
    assert!(!PathClassifier::glob_matches(
        "crates/*/Cargo.toml",
        "crates/a/b/Cargo.toml"
    ));
    // The old implementation rewrote ** to .* and then mangled it again via
    // the single-star pass; this pins the mixed form.
    assert!(PathClassifier::glob_matches(
        "**/*.yaml",
        "lawpack/v2/orders/x.yaml"
    ));
    assert!(!PathClassifier::glob_matches("*.yaml", "lawpack/x.yaml"));
}
