//! PC-16 D4: the entrenched-enforcement-surface digest pin.
//!
//! Moves the integrity WITNESS outside the mutable in-kernel surface - the pin lives in a
//! data file (.vjs/enforcement-surface.lock), not in the code being witnessed - and makes
//! a weakening edit to a gate NON-SILENT: any change to a pinned gate-source file bumps
//! its digest and trips a loud, blocking finding, rather than passing under its own
//! (possibly weakened) logic. A deliberate, recorded gate change clears it by re-pinning
//! (`vjs enforcement-lock`), which is itself the visible acknowledgment.
//!
//! The irreducible remainder is recorded candidly (PC-16 D4): an author with full write
//! access who edits a gate AND re-locks is beyond any in-binary check; no binary can bind
//! the hand that compiles it. The ultimate backstops are non-machine - the Sovereign's
//! gate (ACT-COMPUTER-FIRST-REALM s.14) and the continuing duty of reasonable skill and
//! care (ACT-003 s.4-s.8, breach self-fileable). This is not represented as more.

use crate::report::Finding;
use crate::types::Severity;
use std::collections::BTreeMap;
use std::path::Path;

/// The entrenched-enforcement code path: the focused, rarely-churning gates whose
/// weakening would let a record escape the assent-RESOLUTION floor, the assent allow-list,
/// or bench-integrity. Pinned so an edit is visible and recorded (PC-16 D4). The
/// apex-singleness and citation gates are additionally protected by the constitutive-code
/// mechanism ([2026] VJS-PC 16), which no assent claim can soften.
pub const ENFORCEMENT_SURFACE: &[&str] = &[
    "crates/vjs-engine/src/assent.rs", // the resolution check + the constitutive codes
    "crates/vjs-core/src/front_door.rs", // the assent allow-list + governed-record kinds
    "crates/vjs-core/src/bench.rs",    // bench-integrity (verify_bench, constituted_sizes)
    "crates/vjs-core/src/hook.rs",     // the apex/federation bright-line (apex_routing_decision)
    "crates/vjs-core/src/governance/permit_gate.rs", // the pre-write permit authorization gate
    "crates/vjs-redact/src/lib.rs",    // the canon-write boundary + secret/identity scanner
    "crates/vjs-redact/src/tests.rs", // the scanner's binding proofs (split out of lib.rs; still witnessed)
    // The staged-commit pipeline fires more bright-line gates than the assent/bench/apex set:
    // these three were unpinned until the 2026-06-26 goal-completion audit found them.
    "crates/vjs-engine/src/staged.rs", // the staged-commit gate set (BOUNDARY_MEDIA_IN_CANON, destructive-delete, cross-repo reach)
    "crates/vjs-lawpack/src/validator.rs", // citation uniqueness (CITATION_COLLISION) + duplicate-id + grounding checks
    "crates/vjs-lawpack/src/refs.rs",      // the citation-grounding teeth (ground_operative)
    "crates/vjs-core/src/enforcement.rs",  // this witness itself
];

const LOCK_PATH: &str = ".vjs/enforcement-surface.lock";

fn digest_of(path: &Path) -> Option<String> {
    use sha2::Digest;
    let bytes = std::fs::read(path).ok()?;
    let mut h = sha2::Sha256::new();
    h.update(&bytes);
    Some(format!("sha256:{}", hex::encode(h.finalize())))
}

/// The current (path, digest) of every surface file present, sorted.
pub fn surface_digests(repo: &Path) -> Vec<(String, String)> {
    ENFORCEMENT_SURFACE
        .iter()
        .filter_map(|rel| digest_of(&repo.join(rel)).map(|d| (rel.to_string(), d)))
        .collect()
}

/// Write the pinned manifest over the current surface - the deliberate, recorded
/// acknowledgment after an intended gate change.
pub fn write_lock(repo: &Path) -> std::io::Result<()> {
    let mut out = String::from(
        "# VJS entrenched-enforcement-surface pin (PC-16 D4). A drift from these digests\n\
         # is a loud, blocking finding. Re-lock with `vjs enforcement-lock` ONLY after a\n\
         # deliberate, recorded gate change (and self-file the rationale).\n",
    );
    for (rel, d) in surface_digests(repo) {
        out.push_str(&format!("{rel} {d}\n"));
    }
    std::fs::write(repo.join(LOCK_PATH), out)
}

fn read_lock(repo: &Path) -> Option<BTreeMap<String, String>> {
    let content = std::fs::read_to_string(repo.join(LOCK_PATH)).ok()?;
    let mut map = BTreeMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((rel, d)) = line.rsplit_once(' ') {
            map.insert(rel.trim().to_string(), d.trim().to_string());
        }
    }
    Some(map)
}

/// PC-16 D4: surface a loud, blocking finding when a pinned gate-source file drifts from
/// the lock (a possibly-weakening edit), naming the entrenched instruments and routing to
/// the breach duty. No finding when the lock is absent (an un-pinned repo) - the pin is
/// opt-in via `vjs enforcement-lock`. Deterministic: sha256 over the gate bytes.
pub fn check_drift(repo: &Path) -> Vec<Finding> {
    let Some(pinned) = read_lock(repo) else {
        return Vec::new();
    };
    let current: BTreeMap<String, String> = surface_digests(repo).into_iter().collect();
    let mut findings = Vec::new();
    for rel in ENFORCEMENT_SURFACE {
        let want = pinned.get(*rel);
        let got = current.get(*rel);
        if want != got {
            findings.push(
                Finding::new(
                    Severity::Fatal,
                    "ENFORCEMENT_SURFACE_DRIFT",
                    format!(
                        "The entrenched-enforcement gate '{rel}' changed from its pinned digest \
                         (pinned={want:?} now={got:?}). A weakening edit to a gate must be \
                         non-silent (ACT-COMPUTER-FIRST-REALM s.14/s.15; VJS-ACT 10 s.2)."
                    ),
                )
                .citing("ACT-COMPUTER-FIRST-REALM:s14")
                .fix(
                    "If this is a deliberate, recorded gate change, re-pin with \
                     `vjs enforcement-lock` and self-file the rationale; otherwise restore the \
                     gate. This witness cannot bind an author with full write access who re-locks \
                     - the Sovereign's gate and the duty of care are the backstop (PC-16 D4).",
                ),
            );
        }
    }
    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_lock_means_no_finding() {
        // An un-pinned repo (no lock file) surfaces nothing - the pin is opt-in.
        let tmp = std::env::temp_dir().join("vjs-enf-test-nolock");
        let _ = std::fs::create_dir_all(&tmp);
        assert!(check_drift(&tmp).is_empty());
    }

    #[test]
    fn surface_lists_the_focused_gates() {
        // Every bright-line gate fired by the commit pipeline must be pinned so a weakening
        // edit is non-silent. This is the AUDITED gate set: the commit pipeline's gates are
        // inline Rust, not a registry, so the list is curated, but it must stay complete.
        // (Audit 2026-06-26: hook/permit_gate/redact were unpinned in the first pass; the
        // goal-completion audit then found staged.rs/validator.rs/refs.rs also unpinned.)
        for gate in [
            "crates/vjs-engine/src/assent.rs",
            "crates/vjs-core/src/front_door.rs",
            "crates/vjs-core/src/bench.rs",
            "crates/vjs-core/src/hook.rs",
            "crates/vjs-core/src/governance/permit_gate.rs",
            "crates/vjs-redact/src/lib.rs",
            // the redact tests were split out of lib.rs but remain a binding proof, so
            // they stay pinned - weakening the scanner's coverage must be non-silent.
            "crates/vjs-redact/src/tests.rs",
            // the staged-commit pipeline's own gate set + the lawpack validation/grounding
            // gates it relies on (the goal-completion audit found these three unpinned).
            "crates/vjs-engine/src/staged.rs",
            "crates/vjs-lawpack/src/validator.rs",
            "crates/vjs-lawpack/src/refs.rs",
        ] {
            assert!(ENFORCEMENT_SURFACE.contains(&gate), "unpinned gate: {gate}");
        }
    }

    #[test]
    fn check_drift_flags_an_edited_gate() {
        // K-25's LOAD-BEARING direction: an edit to a pinned gate is non-silent. The earlier
        // tests only proved the no-drift happy path; the goal-completion audit (2026-06-26)
        // found the positive direction (want != got -> Fatal) was asserted nowhere. Pin a
        // digest for a gate file that does not exist at this repo root, so the current digest
        // (None) cannot match the pin -> exactly the drift a weakening edit would produce.
        let tmp = std::env::temp_dir().join(format!("vjs-enf-drift-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".vjs")).unwrap();
        std::fs::write(
            tmp.join(LOCK_PATH),
            "# test pin\ncrates/vjs-core/src/bench.rs sha256:0000000000000000000000000000000000000000000000000000000000000000\n",
        )
        .unwrap();
        let findings = check_drift(&tmp);
        assert!(
            findings
                .iter()
                .any(|f| f.code == "ENFORCEMENT_SURFACE_DRIFT"),
            "an edited/absent pinned gate must produce a Fatal ENFORCEMENT_SURFACE_DRIFT finding, got: {:?}",
            findings.iter().map(|f| &f.code).collect::<Vec<_>>()
        );
        assert!(
            findings
                .iter()
                .any(|f| matches!(f.severity, Severity::Fatal)),
            "the drift finding must be Fatal"
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }
}
