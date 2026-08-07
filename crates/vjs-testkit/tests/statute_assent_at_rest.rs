//! [2026] VJS-CC-VJS 20 D9: the assent question, asked of the STATUTE BOOK at rest.
//!
//! `assent_resolves` is asked at the floor-attachment site - the moment a STAGED record
//! claims the assent floor's shelter. That is the right place for it. But it means the
//! question is only ever asked of a record somebody is touching, and a statute is the
//! least-touched record in the corpus: lodged once, then relied on for months. The one
//! class whose assent matters most was the one class nobody re-examined.

use std::path::{Path, PathBuf};

fn findings(repo: &Path) -> Vec<(String, String)> {
    vjs_engine::assent::at_rest_statute_assent_findings(repo)
        .into_iter()
        .map(|f| (format!("{:?}", f.severity), f.code))
        .collect()
}

fn estate(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("vjs-statute-assent-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("lawpack/v2/statutes")).unwrap();
    std::fs::create_dir_all(dir.join("lawpack/v2/provenance/assent")).unwrap();
    dir
}

/// A statute whose assent resolves: it declares `sovereign_assent` and its id is NAMED
/// in the provenance corpus, which is one of the three limbs `assent_resolves` accepts.
fn lodge_assented(dir: &Path, id: &str) {
    std::fs::write(
        dir.join(format!("lawpack/v2/statutes/{id}.yaml")),
        format!("id: {id}\nstatus: in_force\nassent_source: sovereign_assent\ntitle: A statute\n"),
    )
    .unwrap();
    std::fs::write(
        dir.join(format!("lawpack/v2/provenance/assent/{id}-assent.yaml")),
        format!("id: ASSENT-{id}\nkind: sovereign_assent_event\ninstrument_id: {id}\n"),
    )
    .unwrap();
}

#[test]
fn a_statute_whose_assent_resolves_earns_silence() {
    let dir = estate("resolves");
    lodge_assented(&dir, "ACT-SOUND");
    assert!(findings(&dir).is_empty(), "{:?}", findings(&dir));
}

#[test]
fn a_statute_pointing_at_a_digest_nobody_pinned_is_caught() {
    // THE RED SEED, and it is the shape s.23 names in terms: an UNRESOLVED TRACE. The
    // record types an allowed form and points at an assent event that does not exist.
    // Nothing at rest asked this question before D9.
    let dir = estate("unpinned");
    lodge_assented(&dir, "ACT-SOUND");
    std::fs::write(
        dir.join("lawpack/v2/statutes/ACT-FORGED.yaml"),
        "id: ACT-FORGED\nstatus: in_force\nassent_source: sovereign_assent\n\
         assent_instrument_digest: \"sha256:0000000000000000000000000000000000000000000000000000000000000000\"\n",
    )
    .unwrap();
    let f = findings(&dir);
    assert!(
        f.contains(&("Warning".into(), "AT_REST_STATUTE_ASSENT_UNRESOLVED".into())),
        "{f:?}"
    );
}

#[test]
fn a_statute_declaring_no_assent_at_all_is_caught() {
    let dir = estate("bare");
    std::fs::write(
        dir.join("lawpack/v2/statutes/ACT-BARE.yaml"),
        "id: ACT-BARE\nstatus: in_force\ntitle: claims force, declares no assent\n",
    )
    .unwrap();
    assert!(
        findings(&dir)
            .iter()
            .any(|(_, c)| c == "AT_REST_STATUTE_ASSENT_UNRESOLVED"),
        "{:?}",
        findings(&dir)
    );
}

#[test]
fn a_draft_statute_is_not_asked_the_question() {
    // A draft has not claimed force, so it owes no assent yet. Asking anyway would make
    // every work-in-progress a finding and teach people to ignore the code.
    let dir = estate("draft");
    std::fs::write(
        dir.join("lawpack/v2/statutes/ACT-DRAFT.yaml"),
        "id: ACT-DRAFT\nstatus: draft\ntitle: not yet enacted\n",
    )
    .unwrap();
    assert!(findings(&dir).is_empty(), "{:?}", findings(&dir));
}

#[test]
fn the_finding_is_a_warning_and_never_a_fatal() {
    // A statute at rest is IN FORCE and relied upon. ACT-ASSENTED-RECORD-PROTECTION
    // s.1/s.2 routes its defects for correction rather than voiding or blocking, and a
    // Fatal here would refuse every future commit in the jurisdiction over a defect in
    // history. Same reasoning D13 settled for the order sweep hours earlier.
    let dir = estate("severity");
    std::fs::write(
        dir.join("lawpack/v2/statutes/ACT-BARE.yaml"),
        "id: ACT-BARE\nstatus: in_force\ntitle: claims force, declares no assent\n",
    )
    .unwrap();
    let f = findings(&dir);
    assert!(!f.is_empty(), "the defect must still be reported");
    assert!(
        f.iter().all(|(s, _)| s == "Warning"),
        "in force and relied upon: routed for correction, never blocked: {f:?}"
    );
}

#[test]
fn an_absent_statute_book_discloses_rather_than_passing() {
    // Zero findings and zero checkers are not one number. An estate with no statutes has
    // not been audited clean; it has not been audited.
    let dir = std::env::temp_dir().join(format!("vjs-statute-none-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let f = findings(&dir);
    assert!(
        f.contains(&("Info".into(), "AT_REST_STATUTES_UNCHECKED".into())),
        "{f:?}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}
