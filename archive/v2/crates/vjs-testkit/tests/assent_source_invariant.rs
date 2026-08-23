//! Proof tests for the fail-closed Assent-Source Invariant (CASE-LAW s. 23(5);
//! [2026] REALM-SC 10). A record that claims binding runtime force is admitted
//! ONLY if it declares an `assent_source` resolving to an allowed form; a missing
//! field, an empty value, an unrecognised form, or an unresolved trace is rejected.
//!
//! This is the affirmative ALLOW-LIST the Supreme Court mandated. The court held
//! the not-equal-to-self_authorised DENY-LIST form void as fail-open, because it
//! passes a record that merely omits the assent source. These tests prove the
//! mechanism fails CLOSED: omission and an unresolved trace are both rejected,
//! which is exactly the proof the court required as a condition of commencement.

use std::collections::HashMap;
use std::path::PathBuf;
use vjs_core::spec::{Invariant, LawpackFacts, RepoState, evaluate_invariants};
use vjs_core::types::*;

fn state_with(path: &str, content: &str) -> RepoState {
    let mut file_contents = HashMap::new();
    file_contents.insert(PathBuf::from(path), content.to_string());
    RepoState {
        root: PathBuf::from("."),
        head_sha: None,
        changed_paths: vec![PathBuf::from(path)],
        added_files: vec![PathBuf::from(path)],
        modified_files: Vec::new(),
        deleted_files: Vec::new(),
        file_contents,
        dependency_changes: Vec::new(),
        permits: Vec::new(),
        proofs: Vec::new(),
        logs: Vec::new(),
        orders: Vec::new(),
        boundary_findings: Vec::new(),
    }
}

fn assent_invariant() -> Invariant {
    Invariant {
        id: InvariantId("INV-ASSENT-SOURCE-001".into()),
        title: "Assent-Source Invariant (fail-closed allow-list)".into(),
        basis: vec![AuthorityId("ACT-COMPUTER-FIRST-REALM:s23".into())],
        scope: None,
        rule: PredicateExpr::AssentSourceValid {
            allowed: vec![
                "sovereign_assent".into(),
                "standing_bounded_assent".into(),
            ],
        },
        severity: Severity::Fatal,
        remedy: "Declare a valid assent_source (sovereign_assent or standing_bounded_assent) traceable to specific Sovereign assent.".into(),
    }
}

fn passes(state: &RepoState) -> bool {
    let inv = assent_invariant();
    let report = evaluate_invariants(state, std::slice::from_ref(&inv), &LawpackFacts::default())
        .expect("eval");
    report.findings[0].passed
}

// --- REJECTIONS (fail-closed) -------------------------------------------------

#[test]
fn rejects_missing_assent_source() {
    // The exact hazard the deny-list form missed: a runtime-force record that
    // simply omits assent_source. The allow-list rejects it.
    let st = state_with(
        "lawpack/v2/statutes/99-test.yaml",
        "id: TEST-STAT-001\ntitle: Test\nstatus: binding\n",
    );
    assert!(
        !passes(&st),
        "a record that omits assent_source must be REJECTED"
    );
}

#[test]
fn rejects_unresolved_trace() {
    let st = state_with(
        "lawpack/v2/statutes/99-test.yaml",
        "id: TEST-STAT-001\nassent_source: pending_v1_constitutional_route\nstatus: binding\n",
    );
    assert!(
        !passes(&st),
        "an unresolved assent_source trace must be REJECTED"
    );
}

#[test]
fn rejects_self_authorised() {
    let st = state_with(
        "lawpack/v2/regulations/REG-test.yaml",
        "id: REG-TEST\nassent_source: self_authorised\nstatus: binding\n",
    );
    assert!(!passes(&st), "self_authorised must be REJECTED");
}

#[test]
fn rejects_empty_value() {
    let st = state_with(
        "lawpack/v2/rules/RULE-test.yaml",
        "id: RULE-TEST\nassent_source:\nstatus: binding\n",
    );
    assert!(!passes(&st), "an empty assent_source must be REJECTED");
}

// --- ADMISSIONS (valid assent source) ----------------------------------------

#[test]
fn accepts_specific_sovereign_assent() {
    let st = state_with(
        "lawpack/v2/statutes/99-test.yaml",
        "id: TEST-STAT-001\nassent_source: sovereign_assent\nstatus: binding\n",
    );
    assert!(
        passes(&st),
        "a valid sovereign_assent record must be ADMITTED"
    );
}

#[test]
fn accepts_standing_bounded_assent() {
    let st = state_with(
        "lawpack/v2/regulations/REG-test.yaml",
        "id: REG-TEST\nassent_source: standing_bounded_assent\nstatus: binding\n",
    );
    assert!(
        passes(&st),
        "a valid standing_bounded_assent record must be ADMITTED"
    );
}

// --- SCOPE -------------------------------------------------------------------

#[test]
fn ignores_non_runtime_records() {
    // A provenance / doc / decision file does not claim runtime force and is out
    // of scope: it passes without an assent_source.
    let st = state_with(
        "lawpack/v2/provenance/founding/NOTE.md",
        "some note without an assent_source\n",
    );
    assert!(passes(&st), "non-runtime records are out of scope and pass");
}

// --- INJECTION GOLDEN VECTORS (executed against the binary 2026-06-12) --------
// A raw-line scanner reads the document as a bag of lines, so any line anywhere
// that trims to `assent_source: <allowed>` pardons the whole record. These cases
// were confirmed admitted by the compiled kernel; the operative-field reader must
// REJECT every one, because none declares assent_source as the top-level field.

#[test]
fn rejects_assent_source_only_in_text_block() {
    // No top-level assent_source; the only occurrence is a folded-scalar line
    // inside a section. A YAML parser reads it as prose, not the field.
    let st = state_with(
        "lawpack/v2/statutes/99-attack.yaml",
        "id: ATK-1\ntitle: forged\nstatus: binding\nsections:\n  - id: ATK-1:s1\n    text: >\n      assent_source: sovereign_assent\n",
    );
    assert!(
        !passes(&st),
        "a buried (non-operative) assent line must be REJECTED"
    );
}

#[test]
fn rejects_forbidden_field_with_buried_valid_line() {
    // The operative field declares the explicitly-void self_authorised; a buried
    // valid line must not pardon it. The operative field is what governs.
    let st = state_with(
        "lawpack/v2/statutes/99-attack.yaml",
        "id: ATK-2\nassent_source: self_authorised\ntitle: forged\nstatus: binding\nsections:\n  - id: ATK-2:s1\n    text: >\n      assent_source: sovereign_assent\n",
    );
    assert!(
        !passes(&st),
        "a forbidden operative field must be REJECTED despite a buried valid line"
    );
}

#[test]
fn rejects_buried_quoted_value() {
    let st = state_with(
        "lawpack/v2/statutes/99-attack.yaml",
        "id: ATK-3\ntitle: forged\nstatus: binding\nsections:\n  - id: ATK-3:s1\n    text: >\n      assent_source: \"sovereign_assent\"\n",
    );
    assert!(!passes(&st), "a buried quoted assent line must be REJECTED");
}

#[test]
fn admits_real_operative_field_amid_sections() {
    // Regression guard: a genuine top-level assent_source must still be ADMITTED
    // even when the record carries sections, citations, and other prose.
    let st = state_with(
        "lawpack/v2/statutes/99-good.yaml",
        "id: GOOD-1\nassent_source: sovereign_assent\ncitation: \"[2026] VJS-ACT 50\"\ntitle: genuine\nstatus: binding\nsections:\n  - id: GOOD-1:s1\n    title: s\n    text: >\n      ordinary prose about the law\n",
    );
    assert!(
        passes(&st),
        "a genuine top-level assent_source amid sections must be ADMITTED"
    );
}
