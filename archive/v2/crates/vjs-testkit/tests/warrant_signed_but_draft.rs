//! [2026] VJS-CC-VJS 20 D7, as amended by D22: a warrant that has been SIGNED and still
//! reads `draft`.
//!
//! The failure is quiet and total. The warrant reading rule counts only GOVERNING
//! warrants, and a warrant reading `draft` is not counted - so a warrant the Principal
//! has actually signed governs nothing, escapes the concurrency cap, and is skipped by
//! the supersession chain. The register would be internally consistent and simply wrong
//! about who holds a commission.
//!
//! D6 already makes it a duty to record a signature in the same act as the instrument it
//! issues. This is that duty made checkable.

use std::path::{Path, PathBuf};

fn findings(lawpack: &Path) -> Vec<(String, String)> {
    let mut out = Vec::new();
    vjs_engine::warrants::signed_but_draft_findings(lawpack, &mut out);
    out.into_iter().map(|f| (f.code, f.message)).collect()
}

fn lawpack(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("vjs-warrant-sbd-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("provenance/warrants")).unwrap();
    std::fs::create_dir_all(dir.join("provenance/assent")).unwrap();
    dir
}

fn warrant(dir: &Path, file: &str, inner_id: &str, status: &str) {
    std::fs::write(
        dir.join(format!("provenance/warrants/{file}.yaml")),
        format!("id: {inner_id}\nkind: commission_warrant\nstatus: {status}\n"),
    )
    .unwrap();
}

fn signature(dir: &Path, sig_id: &str, names: &str) {
    std::fs::write(
        dir.join(format!("provenance/assent/{sig_id}.yaml")),
        format!(
            "id: {sig_id}\nkind: sovereign_assent_event\n\
             instrument: Commission Warrant {names} under the Rectification Commission Act\n"
        ),
    )
    .unwrap();
}

#[test]
fn a_signed_warrant_still_reading_draft_is_fatal() {
    // THE RED SEED. The signature exists; the register never caught up. Before D7
    // nothing compared the two records, which were written at two different moments by
    // two different acts.
    let dir = lawpack("signed-draft");
    warrant(&dir, "WARRANT-CANON-002", "WARRANT-CANON-002", "draft");
    signature(&dir, "SIGNATURE-CANON-002", "WARRANT-CANON-002");
    let f = findings(&dir);
    assert!(
        f.iter().any(|(c, _)| c == "WARRANT-SIGNED-BUT-DRAFT"),
        "{f:?}"
    );
}

#[test]
fn an_unsigned_draft_is_silent() {
    // The control, and the ordinary case: a warrant prepared and awaiting the
    // Principal's signature is exactly what `draft` is FOR. Both live 002 warrants are
    // in this state right now, held open by D5, and must raise nothing.
    let dir = lawpack("unsigned");
    warrant(&dir, "WARRANT-CANON-002", "WARRANT-CANON-002", "draft");
    assert!(findings(&dir).is_empty(), "{:?}", findings(&dir));
}

#[test]
fn an_in_force_warrant_with_a_signature_is_silent() {
    let dir = lawpack("in-force");
    warrant(&dir, "WARRANT-CANON-001", "WARRANT-CANON-001", "in_force");
    signature(&dir, "SIGNATURE-CANON-001", "WARRANT-CANON-001");
    assert!(findings(&dir).is_empty(), "{:?}", findings(&dir));
}

#[test]
fn the_join_survives_a_signature_that_names_an_older_rendering() {
    // D22's occasion. The signature records were written before the pseudonymity Acts
    // and name their warrant by an older rendering; the register file carries the
    // ACCESSIONED one. A join keyed only on the filename would miss the signature
    // entirely - the gate would go quiet on precisely the warrant whose paperwork is
    // most tangled.
    let dir = lawpack("old-rendering");
    warrant(&dir, "WARRANT-SUB1-002", "WARRANT-FORMER-002", "draft");
    signature(&dir, "SIGNATURE-FORMER-002", "WARRANT-FORMER-002");
    let f = findings(&dir);
    assert!(
        f.iter().any(|(c, _)| c == "WARRANT-SIGNED-BUT-DRAFT"),
        "a signature is a signature whatever it called its subject: {f:?}"
    );
}

#[test]
fn the_finding_speaks_only_the_accessioned_rendering() {
    // D22 IN TERMS, and the half that matters more than the join. A gate that repeats an
    // old rendering in order to report a defect has published it. The message names the
    // warrant from its FILENAME, never from the `id:` inside the signed record - which
    // cannot be edited and may spell it any way it likes.
    let dir = lawpack("rendering");
    warrant(&dir, "WARRANT-SUB1-002", "WARRANT-FORMER-002", "draft");
    signature(&dir, "SIGNATURE-FORMER-002", "WARRANT-FORMER-002");
    let f = findings(&dir);
    let (_, msg) = f
        .iter()
        .find(|(c, _)| c == "WARRANT-SIGNED-BUT-DRAFT")
        .expect("the finding fires");
    assert!(
        msg.contains("WARRANT-SUB1-002"),
        "the finding names the accessioned rendering: {msg}"
    );
    assert!(
        !msg.contains("WARRANT-FORMER-002"),
        "and never the rendering the signed record happens to use: {msg}"
    );
}

#[test]
fn a_register_with_no_signatures_at_all_is_silent() {
    // No signature records means nothing to compare against, which is a true statement
    // about a fresh estate and not a finding about its warrants.
    let dir = lawpack("no-sigs");
    warrant(&dir, "WARRANT-CANON-002", "WARRANT-CANON-002", "draft");
    let _ = std::fs::remove_dir_all(dir.join("provenance/assent"));
    assert!(findings(&dir).is_empty(), "{:?}", findings(&dir));
}
