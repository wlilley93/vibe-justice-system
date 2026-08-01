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

/// The entrenched-enforcement code path.
///
/// THE TEST FOR MEMBERSHIP ([2026] VJS-CC-VJS 18, D1/D2). A file belongs here if, and only
/// if, an edit CONFINED TO THAT FILE can by itself change whether a bright-line finding is
/// emitted, what severity it carries, or whether the check that produces it runs at all.
/// Call position is not the test; DISPOSITIVE POWER is. No file joins on the theory that it
/// is upstream of a gate, however close it sits.
///
/// THE LIST IS CURATED AND IS KNOWN TO BE UNDER-INCLUSIVE. This replaces an earlier claim
/// that the list "must stay complete" - prose asserting a property nothing checks, which
/// [2026] VJS-CC-VJS 15 holds is not enforcement. The commit pipeline's gates are inline
/// Rust and not a registry, so nothing derives this list and nothing can prove it total. A
/// list known to be short but presented as audited misleads worse than an honest gap, so the
/// gap is stated: four files were identified by measurement in CC-VJS 18 s.3 and are
/// UNDECIDED - neither admitted nor rejected, because no counterexample has been committed
/// for any of them:
///   - `crates/vjs-core/src/install.rs`  - emits Fatals through validate and blocks the
///     pre-write door; unpinned (CC-VJS 18 obiter (iii)).
///   - `crates/vjs-cli/src/front.rs`     - the pre-write hook dispatch.
///   - `crates/vjs-cli/src/local_ci.rs`  - the pre-push door; it calls neither `check_drift`
///     nor the lawpack-lock check, and holds a second, weaker citation check.
///   - `crates/vjs-mcp/src/lib.rs`       - the fourth door; calls `verify_bench` itself.
///
/// THE BOUND ON GROWTH (CC-VJS 18 C6, D3). No path is added to this list except on a
/// COMMITTED TEST showing a confined edit to that file flipping a bright-line outcome while
/// this lock stays green - the discipline CC-VJS 15 used when it proved its marker gate red
/// by a seeded counterexample. Every entry BEYOND THE TWELVE below must carry, in its own
/// comment, `admitted-by: <test file>::<test fn>` naming that test, machine-checked per
/// addition by `crates/vjs-testkit/tests/enforcement_surface_admission.rs`. The surface
/// therefore cannot grow by argument, and nothing that has demonstrably disarmed a gate can
/// be kept off.
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
    // THE GATE DISPATCHER, entrenched WHOLE by [2026] VJS-CC-VJS 18 (Option A, varied: no
    // prior split). Three disarm sites, named by WHAT THEY DO rather than by line number,
    // because line numbers rot and behaviour does not:
    //   (1) THE TWO REFERENT-KEYED CONDITIONS that decide whether the lawpack-TREE check and
    //       the lawpack-LOCK-DIGEST check run at all. One wrong guard - a single condition
    //       keyed to a VENDORED copy - made a Fatal unreachable for every out-of-tree
    //       jurisdiction, and validate reported OK, exit 0, against a falsified digest
    //       ([2026] VJS-CC-VJS 14).
    //   (2) THE ONLY PRODUCTION CALL OF THE ENTRENCHMENT WITNESS IN THE WORKSPACE
    //       (`vjs_core::enforcement::check_drift`). `enforcement.rs` above is pinned as
    //       "this witness itself" and, until this entry, its sole invocation sat in an
    //       unpinned file: a lock bolted to a frame anyone may unscrew. Note candidly what
    //       the pin does NOT buy: entrenchment cannot protect that call from its own
    //       DELETION. Delete it and the digest still moves but no reporter is left inside
    //       the binary. The surviving witness is out of band - the workspace test suite,
    //       specifically kernel_invariant_bindings.rs::
    //       validate_reports_enforcement_surface_drift_through_the_real_pipeline.
    //   (3) THE ASSENT FLOOR - the severity mutation that rewrites a blocking finding on an
    //       assent-resolving record into the route-for-correction form - INCLUDING the
    //       `assent::is_constitutive` carve-out. Delete the carve-out and a forged record
    //       launders bench-integrity, apex-singleness and citation collisions: the exact
    //       forgery PC-16 was convened over. PC-16 D4 already directed the pinning of "the
    //       assent-validity AND FLOOR-ENFORCEMENT code path"; the resolution half went to
    //       `assent.rs`, and this is the floor half completing it.
    // CEILING (CC-VJS 18 C2, D4). This file stands well inside the 600-line ceiling
    // machine-checked by `crates/vjs-testkit/tests/structural_ceiling.rs` only because
    // [2026] VJS-CC-VJS 16 lifted the resolver out of it into `resolver.rs`; before that it
    // was at 591 of 600. When the ceiling next forces a split, THE DISARM SITES CARRY THEIR
    // ENTRENCHMENT WITH THEM IN THE SAME CHANGE: whichever file receives site (1), (2) or
    // (3) joins this list in the same commit that creates it, on the pattern of
    // `crates/vjs-redact/src/tests.rs` above, which was split out of `redact/lib.rs` and
    // stayed pinned. Entrenchment follows the code across a split; it never waits for one,
    // and a split is never a de-entrenchment event.
    "crates/vjs-engine/src/lib.rs",
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
        // THIS IS A SPELLING CHECK, NOT A PROOF ([2026] VJS-CC-VJS 18 C1, forbidden clause:
        // "offer_a_membership_spelling_check_as_proof_that_a_gate_fires"). All it establishes
        // is that a string with these bytes appears in the const. It does NOT establish that
        // the file is hashed, that the digest is compared, that a drift produces a finding,
        // that the finding is Fatal, or that `validate` ever asks. It MAY NOT be offered in
        // satisfaction of C3; the proof at the governed boundary is
        // kernel_invariant_bindings.rs::
        // validate_reports_enforcement_surface_drift_through_the_real_pipeline, which runs
        // the real pipeline and requires the Fatal back out of the Report.
        //
        // The list is CURATED and is known to be UNDER-INCLUSIVE - see the const's own
        // comment for the four undecided candidates. This test cannot detect the gap it is
        // most likely to have; it can only detect a typo in what is already there.
        // (Audit 2026-06-26: hook/permit_gate/redact were unpinned in the first pass; the
        // goal-completion audit then found staged.rs/validator.rs/refs.rs also unpinned.
        // 2026-08-01: vjs-engine/src/lib.rs, the DISPATCHER, was found unpinned by
        // [2026] VJS-CC-VJS 18.)
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
            // the gate DISPATCHER (CC-VJS 18): the referent-keyed guards, the sole
            // production call of check_drift, and the assent-floor severity mutation.
            "crates/vjs-engine/src/lib.rs",
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
