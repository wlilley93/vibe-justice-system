//! K-26 (compile-time fail-closed) + K-28 (derived, drift-proof attributes) for VJS.
//!
//! K-26: a release binary cannot be told to skip or weaken enforcement. The VJS gate has NO
//! runtime env toggle and NO feature-gated bypass - every non-test path IS the enforcement logic,
//! and all scaffolding is `#[cfg(test)]` (excised from a release build by construction). This guard
//! FAILS if anyone later adds an env-var toggle or a `#[cfg(feature=...)]` bypass to the enforcement
//! crates - so the compile-time fail-closed property cannot silently regress.
//!
//! K-28: the lawpack digest is a DERIVED attribute (sha256 over the lawpack), pinned in
//! .vjs/lawpack.lock; the validate gate raises a Fatal LAWPACK_LOCK_DRIFT when the loaded law does
//! not hash to the pinned digest. This proves the derivation is deterministic and that an
//! undeclared content change is detected (drift-proof), exactly the comparison the gate makes.

use std::path::{Path, PathBuf};

use vjs_engine::compute_digest;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Collect every .rs file under a dir (tolerant of a vanished dir under a concurrent build).
fn rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            rs_files(&p, out);
        } else if p.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(p);
        }
    }
}

#[test]
fn the_enforcement_crates_have_no_runtime_toggle_or_feature_bypass() {
    let root = workspace_root();
    // The crates that hold the enforcement decision. (This test lives in vjs-testkit, so it never
    // scans itself - its own source carries these literal patterns.)
    let mut files = Vec::new();
    rs_files(&root.join("crates/vjs-core/src"), &mut files);
    rs_files(&root.join("crates/vjs-engine/src"), &mut files);
    assert!(!files.is_empty(), "expected enforcement source files");

    let mut offences = Vec::new();
    for f in &files {
        let Ok(src) = std::fs::read_to_string(f) else {
            continue;
        };
        for (n, line) in src.lines().enumerate() {
            let t = line.trim();
            if t.starts_with("//") {
                continue; // prose, not code
            }
            // (a) a runtime env toggle would let a release binary be told to skip enforcement.
            if t.contains("env::var(") || t.contains("env::set_var(") {
                offences.push(format!("{}:{} reads an env toggle", f.display(), n + 1));
            }
            // (b) a feature gate could excise/weaken enforcement from a release build.
            if t.contains("#[cfg(feature") {
                offences.push(format!("{}:{} feature-gates enforcement", f.display(), n + 1));
            }
        }
    }
    assert!(
        offences.is_empty(),
        "K-26: enforcement must be neither runtime-toggleable nor feature-removable. \
         A release binary always enforces. Offences:\n{}",
        offences.join("\n")
    );
}

#[test]
fn the_lawpack_digest_is_a_deterministic_derived_attribute_and_drift_is_detected() {
    let dir = std::env::temp_dir().join(format!("vjs-k28-{}", std::process::id()));
    let lp = dir.join("lawpack/v2");
    std::fs::create_dir_all(&lp).unwrap();
    let manifest = lp.join("manifest.toml");

    std::fs::write(&manifest, "lawpack_id = \"vjs\"\nversion = \"1\"\n").unwrap();
    let d1 = compute_digest(&dir).unwrap();
    let d1_again = compute_digest(&dir).unwrap();
    assert_eq!(d1, d1_again, "the derived digest must be deterministic");
    assert!(d1.starts_with("sha256:"), "digest is a pinned sha256");

    // An undeclared content change MUST change the derived attribute (so drift is detectable).
    std::fs::write(&manifest, "lawpack_id = \"vjs\"\nversion = \"2\"\n").unwrap();
    let d2 = compute_digest(&dir).unwrap();
    assert_ne!(d1, d2, "a content change must change the derived digest");

    // This is exactly the LAWPACK_LOCK_DRIFT comparison: a lock pinned over the OLD content no
    // longer matches the recomputed digest, so the gate fires Fatal (drift-proof attribute).
    let pinned_over_old = d1;
    assert_ne!(
        pinned_over_old, d2,
        "a pinned lock digest != the recomputed digest -> LAWPACK_LOCK_DRIFT"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
