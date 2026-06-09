//! Referential integrity of the lawpack: every cited law-object id resolves,
//! or the validator says so. Detection is proven on a fixture; the real
//! lawpack is asserted clean (the 2026-06-09 omnibus, assented the same day,
//! cured the 20 dangling citations the review found).

use std::path::PathBuf;
use vjs_core::types::Severity;
use vjs_lawpack::{LawpackLoader, LawpackValidator};

fn repo_root() -> PathBuf {
    // Anchor on the crate manifest: cargo runs integration tests from the
    // package dir, where a relative "lawpack/v2" resolves to nothing and the
    // whole test would pass (or fail) vacuously.
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn the_real_lawpack_has_no_dangling_citations() {
    let dir = repo_root().join("lawpack/v2");
    let lawpack = LawpackLoader::load(&dir).unwrap();
    let findings = LawpackValidator::check_referential_integrity(&dir, &lawpack).unwrap();
    assert!(
        findings.is_empty(),
        "the canon must cite only law that exists; dangling: {:?}",
        findings.iter().map(|f| &f.message).collect::<Vec<_>>()
    );
}

#[test]
fn a_dangling_citation_is_detected_and_a_negated_mention_is_not() {
    let dir = std::env::temp_dir().join(format!("vjs-lawpack-fixture-{}", std::process::id()));
    std::fs::create_dir_all(dir.join("statutes")).unwrap();
    std::fs::write(
        dir.join("statutes/01-fixture.yaml"),
        "id: ACT-FIXTURE\ntitle: Fixture Act\nstatus: binding\nsections: []\ncommentary: >\n  This act relies on DEC-DOES-NOT-EXIST-001 for its premise, and records\n  that there is no DEC-NEGATED-001 (settled without a separate decision).\n",
    )
    .unwrap();

    let lawpack = LawpackLoader::load(&dir).unwrap();
    let findings = LawpackValidator::check_referential_integrity(&dir, &lawpack).unwrap();

    assert!(
        findings.iter().any(|f| f.message.contains("DEC-DOES-NOT-EXIST-001")),
        "a citation of an undefined object must be reported"
    );
    assert!(
        !findings.iter().any(|f| f.message.contains("DEC-NEGATED-001")),
        "a negated mention is a statement, not a reference"
    );
    assert!(
        findings.iter().all(|f| matches!(f.severity, Severity::Warning)),
        "drift is a lawmaking remedy, not a blocked commit"
    );
    // The fixture act itself is defined, so it never self-reports.
    assert!(!findings.iter().any(|f| f.message.starts_with("'ACT-FIXTURE'")));

    std::fs::remove_dir_all(&dir).ok();
}
