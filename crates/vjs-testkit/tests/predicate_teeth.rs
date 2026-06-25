//! The armed predicates must have teeth: a fixture that SHOULD fail does fail,
//! and the same predicate passes on good input. Before this, these arms were
//! placeholders returning `true`, so the gate could not catch the violations
//! they name. Each test builds a minimal RepoState + LawpackFacts and evaluates
//! one real invariant from the lawpack.

use std::collections::HashSet;
use std::path::PathBuf;

use vjs_core::spec::{Invariant, LawpackFacts, RepoState, evaluate_invariants};
use vjs_lawpack::LawpackLoader;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn invariant(id: &str) -> Invariant {
    let lawpack = LawpackLoader::load(&repo_root().join("lawpack/v2")).unwrap();
    lawpack
        .invariants
        .into_iter()
        .find(|i| i.id.0 == id)
        .unwrap_or_else(|| panic!("invariant {} is in the lawpack", id))
}

fn empty_state() -> RepoState {
    RepoState {
        root: repo_root(),
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

fn passes(inv: &Invariant, state: &RepoState, facts: &LawpackFacts) -> bool {
    evaluate_invariants(state, std::slice::from_ref(inv), facts)
        .unwrap()
        .findings
        .first()
        .map(|f| f.passed)
        .unwrap_or(false)
}

/// A staged file under a runtime-authority path must satisfy the scoped change
/// gate; INV-001/002/003 then read the whole-lawpack facts.
fn staged_runtime_state(content: &str) -> RepoState {
    let mut st = empty_state();
    let p = PathBuf::from("lawpack/v2/regulations/REG-FIXTURE.yaml");
    st.changed_paths.push(p.clone());
    st.file_contents.insert(p, content.to_string());
    st
}

#[test]
fn no_duplicate_ids_has_teeth() {
    let inv = invariant("INV-002");
    let state = staged_runtime_state("id: REG-FIXTURE\nstatus: in_force\n");
    let good = LawpackFacts {
        duplicate_ids: false,
        ..Default::default()
    };
    let bad = LawpackFacts {
        duplicate_ids: true,
        ..Default::default()
    };
    assert!(
        passes(&inv, &state, &good),
        "INV-002 passes when ids are unique"
    );
    assert!(
        !passes(&inv, &state, &bad),
        "INV-002 must fail on a duplicate id"
    );
}

#[test]
fn no_duplicate_citations_has_teeth() {
    let inv = invariant("INV-003");
    let state = staged_runtime_state("id: REG-FIXTURE\nstatus: in_force\n");
    let good = LawpackFacts {
        duplicate_citations: false,
        ..Default::default()
    };
    let bad = LawpackFacts {
        duplicate_citations: true,
        ..Default::default()
    };
    assert!(
        passes(&inv, &state, &good),
        "INV-003 passes when citations are unique"
    );
    assert!(
        !passes(&inv, &state, &bad),
        "INV-003 must fail on a citation collision"
    );
}

#[test]
fn lawpack_validates_has_teeth() {
    let inv = invariant("INV-001");
    let state = staged_runtime_state("id: REG-FIXTURE\nstatus: in_force\n");
    let good = LawpackFacts {
        validates: true,
        ..Default::default()
    };
    let bad = LawpackFacts {
        validates: false,
        ..Default::default()
    };
    assert!(
        passes(&inv, &state, &good),
        "INV-001 passes when the lawpack validates"
    );
    assert!(
        !passes(&inv, &state, &bad),
        "INV-001 must fail when validation fails"
    );
}

#[test]
fn required_fields_has_teeth() {
    let inv = invariant("INV-LAWMAKING-001");
    let facts = LawpackFacts::default();
    // A new regulation missing kernel_effect should fail; with it, pass.
    let bad = staged_runtime_state("id: REG-FIXTURE\nauthority: ACT-003\nstatus: in_force\n");
    let good = staged_runtime_state(
        "id: REG-FIXTURE\nauthority: ACT-003\nstatus: in_force\nkernel_effect:\n  must: [x]\n",
    );
    // The invariant requires its declared fields; at minimum a record missing a
    // required field fails while a complete one passes.
    let bad_passes = passes(&inv, &bad, &facts);
    let good_passes = passes(&inv, &good, &facts);
    assert!(good_passes, "INV-LAWMAKING-001 passes a complete record");
    assert!(
        !bad_passes,
        "INV-LAWMAKING-001 must fail a record missing a required field"
    );
}

#[test]
fn draft_law_is_not_binding_has_teeth() {
    let inv = invariant("INV-LAWMAKING-002");
    // A staged record whose id is in the graph but marked draft must fail.
    let mut all_ids = HashSet::new();
    all_ids.insert("REG-FIXTURE".to_string());
    let facts = LawpackFacts {
        all_ids,
        ..Default::default()
    };
    let draft = staged_runtime_state("id: REG-FIXTURE\nstatus: draft\n");
    let in_force = staged_runtime_state("id: REG-FIXTURE\nstatus: in_force\n");
    assert!(
        !passes(&inv, &draft, &facts),
        "INV-LAWMAKING-002 must fail a draft in the runtime graph"
    );
    assert!(
        passes(&inv, &in_force, &facts),
        "INV-LAWMAKING-002 passes an in-force record"
    );
}

#[test]
fn directory_roles_resolve_has_teeth() {
    let inv = invariant("INV-012");
    let state = empty_state();
    let good = LawpackFacts {
        directory_roles_resolve: true,
        ..Default::default()
    };
    let bad = LawpackFacts {
        directory_roles_resolve: false,
        ..Default::default()
    };
    assert!(
        passes(&inv, &state, &good),
        "INV-012 passes when roles resolve in-repo"
    );
    assert!(
        !passes(&inv, &state, &bad),
        "INV-012 must fail when a role path escapes the repo"
    );
}

#[test]
fn mcp_local_first_has_teeth() {
    let inv = invariant("INV-011");
    let state = empty_state();
    let good = LawpackFacts {
        mcp_local_first: true,
        ..Default::default()
    };
    let bad = LawpackFacts {
        mcp_local_first: false,
        ..Default::default()
    };
    assert!(
        passes(&inv, &state, &good),
        "INV-011 passes when MCP is local-first"
    );
    assert!(
        !passes(&inv, &state, &bad),
        "INV-011 must fail on a public MCP bind"
    );
}
