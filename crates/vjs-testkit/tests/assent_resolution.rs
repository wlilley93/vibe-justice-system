//! [2026] VJS-PC 16 (The Assent-Resolution Floor) coverage, measured against real canon.
//!
//! D1: "valid assent_source" requires RESOLUTION to real Sovereign authority, not bare
//! allow-list membership. D2: a genuinely-assented record still resolves (the floor is
//! not narrowed); a fresh forgery does not. The constitutive-validity codes are never
//! softened by any assent claim ("void ab initio on both grounds").

use std::path::PathBuf;
use vjs_engine::assent::{assent_resolves, is_constitutive};

fn workspace_root() -> PathBuf {
    // The root these tests want is the LAWPACK'S HOME (the real records under
    // lawpack/v2 and .vjs), FOUND by walking up rather than counting levels: in a
    // vendored tree the crates sit one level deeper than the law, and the counted
    // form broke three of these tests there at the 2026-08-06 re-pull.
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        if d.join("lawpack/v2/manifest.toml").is_file() {
            return d;
        }
        assert!(
            d.pop(),
            "no lawpack/v2 above CARGO_MANIFEST_DIR: these tests need one"
        );
    }
}

// `established_at_head` is now a PURE FACT the caller supplies (the engine computes it once
// via GitIntegration::tracked_at_head). These tests pass it explicitly, so they are hermetic
// and deterministic: no per-record git subprocess, no flake. The value encodes each case's
// intent - a forgery / not-at-HEAD record is `false`; a committed record is `true`.
const ESTABLISHED: bool = true;
const FRESH: bool = false;

#[test]
fn a_fresh_forged_sovereign_assent_does_not_resolve() {
    // The audit's exploit: a never-seen record types sovereign_assent, pointing at no
    // assent event. It is named by no provenance record and is not established in canon,
    // so it does NOT resolve and stays outside the floor.
    let forged = "id: \"2026-VJS-SC-999\"\nassent_source: sovereign_assent\ncitation: \"[2026] VJS-SC 999\"\n";
    assert!(!assent_resolves(
        &workspace_root(),
        "lawpack/v2/orders/FRESH-FORGERY-never-committed.yaml",
        forged,
        FRESH,
    ));
}

#[test]
fn a_named_sovereign_assent_record_resolves() {
    // ACT-ASSENTED-RECORD-PROTECTION is lodged by a real Sovereign-assent event
    // (provenance/assent/2026-06-12-...), so it resolves.
    let real = "id: ACT-ASSENTED-RECORD-PROTECTION\nassent_source: sovereign_assent\ncitation: \"[2026] VJS-ACT 10\"\n";
    // Resolves by NAMING in the provenance corpus - independent of HEAD, so FRESH proves the
    // naming route stands on its own.
    assert!(assent_resolves(
        &workspace_root(),
        "lawpack/v2/statutes/10-assented-record-protection.yaml",
        real,
        FRESH,
    ));
}

#[test]
fn an_established_standing_bounded_record_resolves() {
    // A real Privy order, established in committed canon and tracing to the Realm's
    // Sovereign foundation, resolves - so its OTHER (correctable) defects would still
    // route for correction (D2). The path exists at HEAD.
    let order = "id: \"2026-VJS-PC-015\"\nassent_source: standing_bounded_assent\ncitation: \"[2026] VJS-PC 15\"\n";
    // Established in committed canon -> limb 1 short-circuit (D2's load-bearing carve-out).
    assert!(assent_resolves(
        &workspace_root(),
        "lawpack/v2/orders/2026-VJS-PC-015.yaml",
        order,
        ESTABLISHED,
    ));
}

#[test]
fn a_junk_or_absent_assent_value_does_not_resolve() {
    let junk = "id: x\nassent_source: i_made_this_up\n";
    let absent = "id: x\nstatus: binding\n";
    let root = workspace_root();
    assert!(!assent_resolves(
        &root,
        "lawpack/v2/orders/x.yaml",
        junk,
        FRESH
    ));
    assert!(!assent_resolves(
        &root,
        "lawpack/v2/orders/x.yaml",
        absent,
        FRESH
    ));
}

#[test]
fn every_committed_standing_regulation_resolves_keeping_its_floor() {
    // [2026] VJS-SC 5 D2 (unanimous the carve-out is load-bearing): the established-canon
    // carve-out shelters EVERY regulation committed at HEAD - not one in-force regulation
    // loses its floor. The complete, zero-narrowing migration.
    let root = workspace_root();
    let dir = root.join("lawpack/v2/regulations");
    let mut checked = 0;
    for entry in std::fs::read_dir(&dir).unwrap().flatten() {
        let p = entry.path();
        if p.extension().and_then(|s| s.to_str()) != Some("yaml") {
            continue;
        }
        let content = std::fs::read_to_string(&p).unwrap();
        if !content.contains("assent_source: standing_bounded_assent") {
            continue;
        }
        let rel = format!(
            "lawpack/v2/regulations/{}",
            p.file_name().unwrap().to_str().unwrap()
        );
        assert!(
            assent_resolves(&root, &rel, &content, ESTABLISHED),
            "committed regulation {rel} must resolve via limb 1 (established-at-HEAD)"
        );
        checked += 1;
    }
    assert!(
        checked >= 10,
        "expected many standing regulations, got {checked}"
    );
}

#[test]
fn a_fresh_regulation_with_a_real_parent_resolves_without_per_instrument_paper() {
    // SC-5 D3 limb 2: a fresh regulation (a path NOT at HEAD) declaring a real parent
    // authority resolves by class-route, carrying NO per-instrument provenance.
    let reg = "id: REG-FRESH-001\nassent_source: standing_bounded_assent\ncitation: \"[2026] VJS-REG 99\"\nauthority: ACT-CONSOLIDATION-FRAMEWORK:s7\n";
    // FRESH (not at HEAD) -> must resolve by limb 2 (real parent), with no per-instrument paper.
    assert!(assent_resolves(
        &workspace_root(),
        "lawpack/v2/regulations/REG-FRESH-NOT-AT-HEAD.yaml",
        reg,
        FRESH,
    ));
}

#[test]
fn a_fresh_record_on_no_recognised_route_does_not_resolve() {
    // SC-5: a fresh standing_bounded_assent record not established, naming no real parent,
    // and not an order with a bench, has an unresolved trace and does not resolve.
    let root = workspace_root();
    let junk = "id: REG-FORGED-001\nassent_source: standing_bounded_assent\ncitation: \"[2026] VJS-REG 98\"\nauthority: ACT-DOES-NOT-EXIST:s1\n";
    assert!(!assent_resolves(
        &root,
        "lawpack/v2/regulations/REG-FORGED-NOT-AT-HEAD.yaml",
        junk,
        FRESH,
    ));
}

#[test]
fn a_fresh_order_with_a_constituted_bench_resolves_limb3() {
    // SC-5 D4 limb 3: a genuine new order (not at HEAD) with a non-empty bench resolves,
    // so its correctable defects route for correction. A bench-less order is barred
    // independently by the constitutive codes (proven in the e2e harness).
    let root = workspace_root();
    let with_bench = "id: \"2026-VJS-PC-099\"\nassent_source: standing_bounded_assent\ncourt: privy_council\nbench:\n  - A\n  - B\n  - C\n";
    // FRESH (not at HEAD) -> resolves only via limb 3 (a constituted bench).
    assert!(assent_resolves(
        &root,
        "lawpack/v2/orders/2026-VJS-PC-099-NOT-AT-HEAD.yaml",
        with_bench,
        FRESH,
    ));
    let bench_less = "id: \"2026-VJS-PC-098\"\nassent_source: standing_bounded_assent\ncourt: privy_council\nbench: []\n";
    assert!(!assent_resolves(
        &root,
        "lawpack/v2/orders/2026-VJS-PC-098-NOT-AT-HEAD.yaml",
        bench_less,
        FRESH,
    ));
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
    // [2026] VJS-PC 17 D2: an unresolved operative citation is Fatal but CORRECTABLE - it
    // must NOT be constitutive, so a resolving order's citation defect routes for
    // correction rather than voiding the order (per-incuriam voidness is for a court).
    assert!(!is_constitutive("ORDER_CITATION_UNRESOLVED"));
}
