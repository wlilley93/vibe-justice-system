//! [2026] VJS-PC 16 (The Assent-Resolution Floor) coverage, measured against real canon.
//!
//! D1: "valid assent_source" requires RESOLUTION to real Sovereign authority, not bare
//! allow-list membership. D2: a genuinely-assented record still resolves (the floor is
//! not narrowed); a fresh forgery does not. The constitutive-validity codes are never
//! softened by any assent claim ("void ab initio on both grounds").

use std::path::PathBuf;
use vjs_engine::assent::{assent_resolves, is_constitutive};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn a_fresh_forged_sovereign_assent_does_not_resolve() {
    // The audit's exploit: a never-seen record types sovereign_assent, pointing at no
    // assent event. It is named by no provenance record and is not established in canon,
    // so it does NOT resolve and stays outside the floor.
    let forged = "id: \"2026-VJS-SC-999\"\nassent_source: sovereign_assent\ncitation: \"[2026] VJS-SC 999\"\n";
    assert!(!assent_resolves(
        &workspace_root(),
        "lawpack/v2/orders/FRESH-FORGERY-never-committed.yaml",
        forged
    ));
}

#[test]
fn a_named_sovereign_assent_record_resolves() {
    // ACT-ASSENTED-RECORD-PROTECTION is lodged by a real Sovereign-assent event
    // (provenance/assent/2026-06-12-...), so it resolves.
    let real = "id: ACT-ASSENTED-RECORD-PROTECTION\nassent_source: sovereign_assent\ncitation: \"[2026] VJS-ACT 10\"\n";
    assert!(assent_resolves(
        &workspace_root(),
        "lawpack/v2/statutes/10-assented-record-protection.yaml",
        real
    ));
}

#[test]
fn an_established_standing_bounded_record_resolves() {
    // A real Privy order, established in committed canon and tracing to the Realm's
    // Sovereign foundation, resolves - so its OTHER (correctable) defects would still
    // route for correction (D2). The path exists at HEAD.
    let order = "id: \"2026-VJS-PC-015\"\nassent_source: standing_bounded_assent\ncitation: \"[2026] VJS-PC 15\"\n";
    assert!(assent_resolves(
        &workspace_root(),
        "lawpack/v2/orders/2026-VJS-PC-015.yaml",
        order
    ));
}

#[test]
fn a_junk_or_absent_assent_value_does_not_resolve() {
    let junk = "id: x\nassent_source: i_made_this_up\n";
    let absent = "id: x\nstatus: binding\n";
    let root = workspace_root();
    assert!(!assent_resolves(&root, "lawpack/v2/orders/x.yaml", junk));
    assert!(!assent_resolves(&root, "lawpack/v2/orders/x.yaml", absent));
}

#[test]
fn constitutive_codes_are_the_void_ab_initio_grounds() {
    // Bench-integrity, apex-singleness, and citation collision go to whether the record
    // IS a valid record/order - never softened by assent.
    for c in [
        "BENCH_REQUIRED",
        "BENCH_SIZE_MISMATCH",
        "BENCH_SILENT_SEAT",
        "BENCH_OPINION_MISSING",
        "TIER_NOT_CONSTITUTED",
        "CITATION_COLLISION",
        "APEX_RECORD_IN_SUBSCRIBING_JURISDICTION",
    ] {
        assert!(is_constitutive(c), "{c} must be constitutive");
    }
    // Ordinary, correctable defects remain downgradeable for a resolving record (D2).
    assert!(!is_constitutive("S5_INERT_KERNEL_EFFECT"));
    assert!(!is_constitutive("ORDER_MALFORMED"));
    assert!(!is_constitutive("DANGLING_REFERENCE"));
}
