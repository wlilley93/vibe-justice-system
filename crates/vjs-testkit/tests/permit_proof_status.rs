//! The permit/proof invariants must witness STATUS, not mere presence. Before
//! the witness-soundness sweep, `GovernedWritesRequirePermit` and
//! `ProofsExistBeforeClose` were `!is_empty()` - so a Closed/Revoked permit or a
//! Pending/Failed proof satisfied them. These tests pin the real checks: only an
//! Active permit, and only a Passed proof, count.

use std::path::PathBuf;
use vjs_core::spec::{Invariant, LawpackFacts, Permit, Proof, RepoState, evaluate_invariants};
use vjs_core::types::*;

fn empty_state() -> RepoState {
    RepoState {
        root: PathBuf::from("."),
        head_sha: None,
        changed_paths: Vec::new(),
        added_files: Vec::new(),
        modified_files: Vec::new(),
        deleted_files: Vec::new(),
        file_contents: Default::default(),
        dependency_changes: Vec::new(),
        permits: Vec::new(),
        proofs: Vec::new(),
        logs: Vec::new(),
        orders: Vec::new(),
        boundary_findings: Vec::new(),
    }
}

fn inv(rule: PredicateExpr) -> Invariant {
    Invariant {
        id: InvariantId("INV-FIXTURE".into()),
        title: "fixture".into(),
        basis: vec![],
        scope: None,
        rule,
        severity: Severity::Fatal,
        remedy: "fixture".into(),
    }
}

fn passes(state: &RepoState, rule: PredicateExpr) -> bool {
    let i = inv(rule);
    evaluate_invariants(state, std::slice::from_ref(&i), &LawpackFacts::default())
        .unwrap()
        .findings[0]
        .passed
}

fn permit(status: PermitStatus) -> Permit {
    Permit {
        id: PermitId("PERMIT-FIX".into()),
        route_id: RouteId("ROUTE-FIX".into()),
        actor: "lexby".into(),
        scope: Some(Scope {
            paths: Some(vec!["crates/vjs-core/**".into()]),
            jurisdictions: None,
            action_kinds: None,
            issue_tags: None,
            records: None,
        }),
        obligations: Vec::new(),
        expires_at: "2099-01-01T00:00:00+00:00".into(),
        status,
        self_issued: true,
        meaning: None,
        intent_digest: None,
    }
}

fn proof(status: ProofStatus) -> Proof {
    Proof {
        id: ProofId("PROOF-FIX".into()),
        permit_id: PermitId("PERMIT-FIX".into()),
        kind: ProofKind::TestResult,
        status,
        digest: None,
        captured_at: "2026-06-12T00:00:00+00:00".into(),
    }
}

#[test]
fn active_permit_satisfies_governed_writes() {
    let mut st = empty_state();
    st.permits.push(permit(PermitStatus::Active));
    assert!(passes(&st, PredicateExpr::GovernedWritesRequirePermit));
}

#[test]
fn closed_permit_does_not_satisfy_governed_writes() {
    let mut st = empty_state();
    st.permits.push(permit(PermitStatus::Closed));
    assert!(
        !passes(&st, PredicateExpr::GovernedWritesRequirePermit),
        "a Closed permit must not excuse a governed write"
    );
}

#[test]
fn revoked_permit_does_not_satisfy_governed_writes() {
    let mut st = empty_state();
    st.permits.push(permit(PermitStatus::Revoked));
    assert!(
        !passes(&st, PredicateExpr::GovernedWritesRequirePermit),
        "a Revoked permit must not excuse a governed write"
    );
}

#[test]
fn passed_proof_satisfies_close() {
    let mut st = empty_state();
    st.proofs.push(proof(ProofStatus::Passed));
    assert!(passes(&st, PredicateExpr::ProofsExistBeforeClose));
}

#[test]
fn pending_proof_does_not_satisfy_close() {
    let mut st = empty_state();
    st.proofs.push(proof(ProofStatus::Pending));
    assert!(
        !passes(&st, PredicateExpr::ProofsExistBeforeClose),
        "a Pending proof must not discharge the close obligation"
    );
}

#[test]
fn failed_proof_does_not_satisfy_close() {
    let mut st = empty_state();
    st.proofs.push(proof(ProofStatus::Failed));
    assert!(
        !passes(&st, PredicateExpr::ProofsExistBeforeClose),
        "a Failed proof must not discharge the close obligation"
    );
}
