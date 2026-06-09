//! Referential integrity of the lawpack: every cited law-object id resolves,
//! or the validator says so. The known drift (specs citing never-enacted
//! decisions, invariants, and obligations) surfaces as warnings; a negated
//! mention ("no DEC-X") is a statement, not a reference.

use std::path::PathBuf;
use vjs_core::types::Severity;
use vjs_lawpack::{LawpackLoader, LawpackValidator};

#[test]
fn dangling_law_citations_are_reported_as_warnings() {
    // Anchor on the crate manifest: cargo runs integration tests from the
    // package dir, where a relative "lawpack/v2" resolves to nothing and the
    // whole test would pass (or fail) vacuously.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../lawpack/v2");
    let lawpack = LawpackLoader::load(&dir).unwrap();
    let findings = LawpackValidator::check_referential_integrity(&dir, &lawpack).unwrap();

    assert!(
        findings.iter().all(|f| matches!(f.severity, Severity::Warning)),
        "drift is a lawmaking remedy, not a blocked commit"
    );

    // Known drift from the 2026-06-09 audit: a statute section cited but never
    // enacted, and a spec citing a never-enacted decision.
    for known in ["ACT-COMPUTER-FIRST-REALM:s29", "DEC-KERNEL-001"] {
        assert!(
            findings.iter().any(|f| f.message.contains(known)),
            "expected '{}' to be reported as dangling",
            known
        );
    }

    // PC-005 records "no DEC-INSTITUTIONS-001" deliberately; a negated mention
    // must not be read as a citation.
    assert!(
        !findings.iter().any(|f| f.message.contains("DEC-INSTITUTIONS-001")),
        "a negated mention is not a reference"
    );

    // Defined objects never self-report.
    assert!(!findings.iter().any(|f| f.message.starts_with("'DEC-001'")));
}
