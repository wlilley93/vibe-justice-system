//! The s.5(a) kernel-effect teeth-gate, authorized as machinery with conditions
//! by [2026] VJS-CC 15 (Marrowby CCJ). A runtime-force record whose declared
//! kernel_effect binds to no recognized operation is inert ceremony, ROUTED FOR
//! CORRECTION (a non-blocking Warning), never voided (D2). It is structural only
//! (D4). The critical safety property: it must produce ZERO false positives on
//! the real lawpack - a populated `defines` (e.g. s.2's force_source) is real
//! kernel effect and must not be flagged - so the assent floor is never engaged.

use std::path::PathBuf;
use vjs_lawpack::{is_inert_kernel_effect, KernelEffect, LawpackLoader, LawpackValidator};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn empty_ke() -> KernelEffect {
    KernelEffect {
        when: None,
        must: None,
        may: None,
        must_not: None,
        exceptions: None,
        proof: None,
        defines: None,
        prohibits: None,
        status: None,
    }
}

#[test]
fn an_all_empty_kernel_effect_is_inert() {
    assert!(is_inert_kernel_effect(&empty_ke()));
}

#[test]
fn a_populated_defines_is_not_inert() {
    // The s.2 shape: force_source lives INSIDE a populated defines. Real effect.
    let ke = KernelEffect {
        defines: Some(serde_json::json!({ "force_source": "organ_constitutive_act" })),
        ..empty_ke()
    };
    assert!(
        !is_inert_kernel_effect(&ke),
        "a populated defines is real kernel effect and must not be flagged"
    );
}

#[test]
fn a_lone_prohibits_is_not_inert() {
    let ke = KernelEffect {
        prohibits: Some(vec!["legislature_self_extension".into()]),
        ..empty_ke()
    };
    assert!(!is_inert_kernel_effect(&ke));
}

#[test]
fn a_guard_only_kernel_effect_is_inert() {
    // `when` is a guard, not an effect: a block with only `when` binds to nothing.
    let ke = KernelEffect {
        when: Some(vjs_lawpack::Condition { any: None, all: Some(vec!["x == true".into()]) }),
        ..empty_ke()
    };
    assert!(is_inert_kernel_effect(&ke));
}

#[test]
fn the_real_lawpack_has_no_inert_kernel_effects() {
    // SAFETY: no legitimate, assented record may be wrongly flagged inert. If this
    // fails, the gate is over-broad and would engage the assent floor the court
    // forbade ([2026] VJS-CC 15 D2/D3).
    let lawpack = LawpackLoader::load(&repo_root().join("lawpack/v2")).unwrap();
    let report = LawpackValidator::validate(&lawpack).unwrap();
    let inert: Vec<&String> = report
        .findings
        .iter()
        .filter(|f| f.code == "S5_INERT_KERNEL_EFFECT")
        .map(|f| &f.message)
        .collect();
    assert!(inert.is_empty(), "false positives on the real lawpack: {:?}", inert);
}

#[test]
fn a_present_but_inert_kernel_effect_is_routed_for_correction() {
    // A kernel_effect carrying ONLY an unrecognized key (force_source at the block
    // top level, which serde drops) parses to an all-empty KernelEffect - inert
    // ceremony that the raw-line / id-resolves witnesses never caught. It must be
    // flagged (Warning, route for correction), and it must NOT block (never void).
    let dir = std::env::temp_dir().join(format!("vjs-ke-fixture-{}", std::process::id()));
    std::fs::create_dir_all(dir.join("statutes")).unwrap();
    std::fs::write(
        dir.join("statutes/01-fixture.yaml"),
        "id: ACT-FIX\ntitle: Fix\nstatus: binding\nsections:\n  - id: ACT-FIX:s1\n    title: s\n    text: t\n    kernel_effect:\n      force_source: organ_constitutive_act\n",
    )
    .unwrap();

    let lawpack = LawpackLoader::load(&dir).unwrap();
    let report = LawpackValidator::validate(&lawpack).unwrap();
    std::fs::remove_dir_all(&dir).ok();

    let finding = report
        .findings
        .iter()
        .find(|f| f.code == "S5_INERT_KERNEL_EFFECT");
    assert!(
        finding.is_some(),
        "a kernel_effect with only an unrecognized key is inert and must be routed for correction"
    );
    assert_eq!(
        finding.unwrap().severity,
        vjs_core::types::Severity::Warning,
        "the disposition is route-for-correction (Warning), never a blocking void (D2)"
    );
}

#[test]
fn the_never_void_disposition_is_entrenched_warning_only() {
    // [2026] VJS-PC 12 D3 (3-0, on appeal): the gate may never void or block an
    // assented record; its severity is ENTRENCHED as Warning, amendable only by
    // Sovereign-assented primary law expressly citing s.5 (s.14/s.23 are
    // non-derogable by construction). Every inert finding - section AND regulation -
    // must be Warning, never Error/Fatal. A change that blocks/voids breaks this
    // test BY DESIGN: it is a constitutional act, not a refactor.
    let dir = std::env::temp_dir().join(format!("vjs-entrench-{}", std::process::id()));
    std::fs::create_dir_all(dir.join("statutes")).unwrap();
    std::fs::create_dir_all(dir.join("regulations")).unwrap();
    std::fs::write(
        dir.join("statutes/01-fix.yaml"),
        "id: ACT-FIX\ntitle: Fix\nstatus: binding\nsections:\n  - id: ACT-FIX:s1\n    title: s\n    text: t\n    kernel_effect:\n      force_source: organ_constitutive_act\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("regulations/REG-FIX.yaml"),
        "id: REG-FIX\ntitle: Fix Reg\nauthority: ACT-FIX:s1\nstatus: binding\ntext: t\nkernel_effect:\n  force_source: organ_constitutive_act\n",
    )
    .unwrap();

    let lawpack = LawpackLoader::load(&dir).unwrap();
    let report = LawpackValidator::validate(&lawpack).unwrap();
    std::fs::remove_dir_all(&dir).ok();

    let inert: Vec<_> = report
        .findings
        .iter()
        .filter(|f| f.code == "S5_INERT_KERNEL_EFFECT")
        .collect();
    assert!(
        inert.len() >= 2,
        "expected the inert section AND regulation both flagged, got {}",
        inert.len()
    );
    for f in &inert {
        assert_eq!(
            f.severity,
            vjs_core::types::Severity::Warning,
            "ENTRENCHED [2026] VJS-PC 12 D3: the gate may never block or void; severity must stay Warning"
        );
    }
}
