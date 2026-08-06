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
    // THE CANON'S OWN LICENCE CONDITION ([2026] VJS-PC 11 D2). Entrenched on the day it
    // was built, because the defect it exists to catch is EXACTLY a silent edit: the
    // canon's licence was replaced in an anonymising history squash on 2026-07-11 with
    // no adoption record and no mention in the commit message, and nothing noticed for
    // close to a month. A gate whose whole subject matter is an unannounced change must
    // not itself be changeable without announcement. Note especially the severity
    // hinge - the Warning/Fatal split that turns on whether the conflict is recorded in
    // `.vjs/logs/breaches` - which is precisely the line an author in a hurry would move
    // to make a launch go green.
    // admitted-by: crates/vjs-testkit/tests/canon_licence.rs::a_confined_edit_to_the_severity_hinge_flips_a_bright_line
    "crates/vjs-engine/src/canon_licence.rs",
];

const LOCK_PATH: &str = ".vjs/enforcement-surface.lock";

const LOCK_HEADER: &str = "\
# VJS entrenched-enforcement-surface pin (PC-16 D4). A drift from these digests is a loud,
# blocking finding. Re-pin with `vjs enforcement-lock --authority '<order or decision-log id>'`
# ONLY after a deliberate, recorded gate change.
#
# EVERY ENTRY CARRIES THE AUTHORITY UNDER WHICH ITS DIGEST MOVED ([2026] VJS-CC-VJS 18 C7,
# applying CC-VJS 12(d) to the second lock). `--authority` is required and empty is refused,
# because a field that defaults to a constant, or accepts empty, checks nothing.
#
# The authority is stamped ONLY on entries whose digest actually moved. An entry that did not
# move keeps the authority it already carried - re-stamping an unchanged digest with an
# unrelated citation would manufacture a false provenance record, which is the defect this
# field exists to prevent.
#
# ON THE FORMAT MIGRATION (2026-08-02). This file was flat `path sha256:...` lines with no
# authority at all until CC-VJS 18 C7. Every entry therefore acquired its authority in that
# one write. That is a true statement about THIS RECORD, not a claim about which ruling
# originally entrenched each file: that history is in the ENFORCEMENT_SURFACE const's own
# comments and is deliberately NOT reconstructed here, because a reconstructed citation is
# indistinguishable from a recorded one once written down.
#
# Generated. Re-pin rather than hand-edit: an entry with an empty or missing authority is
# ENFORCEMENT_LOCK_UNREADABLE, which is Fatal, not silence.
";

fn digest_of(path: &Path) -> Option<String> {
    use sha2::Digest;
    let bytes = std::fs::read(path).ok()?;
    let mut h = sha2::Sha256::new();
    h.update(&bytes);
    Some(format!("sha256:{}", hex::encode(h.finalize())))
}

/// One pinned file: its digest, and the authority under which THAT DIGEST MOVED.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PinnedEntry {
    pub path: String,
    pub digest: String,
    /// An order citation (`[2026] VJS-CC-VJS 18`) or a decision-log id. Never empty:
    /// `write_lock` refuses to write without one and `read_lock` refuses to trust a lock
    /// that carries one.
    pub authority: String,
    /// When this digest was pinned. A verdict with no timestamp cannot be told from a stale
    /// one. Carried forward unchanged while the digest does not move, so the file does not
    /// churn on an unrelated re-pin.
    pub pinned_at: String,
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
struct LockFile {
    #[serde(default)]
    entry: Vec<PinnedEntry>,
}

/// The three states of the lock, which the old `Option` could not tell apart.
///
/// `Absent` and `Unreadable` used to be the SAME value (`None`), and both returned no
/// finding. So a lock that existed but could not be parsed reported exactly what an
/// un-pinned repository reports: nothing. Introducing a stricter parse without splitting
/// these would have been a silent disarm on the day it landed.
enum Lock {
    /// No lock file. The pin is opt-in, so this is not a finding.
    Absent,
    /// A lock file exists and cannot be trusted. This is Fatal, never silence.
    Unreadable(String),
    Pinned(BTreeMap<String, PinnedEntry>),
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
///
/// REFUSES WITHOUT AN AUTHORITY ([2026] VJS-CC-VJS 18 C7). `authority` is the order citation
/// or decision-log id under which the digests moved. An empty or whitespace-only value is
/// refused and NOTHING IS WRITTEN - the refusal must not leave a half-written lock, because a
/// truncated lock is `ENFORCEMENT_LOCK_UNREADABLE` and the operator would be left worse off
/// than before they ran the command.
pub fn write_lock(repo: &Path, authority: &str) -> std::io::Result<()> {
    let authority = authority.trim();
    if authority.is_empty() {
        // Refuse BEFORE any filesystem write. There is no default: a field that defaults to
        // a constant checks nothing, which is the vacuity C7 names.
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "an enforcement-surface re-pin requires --authority: the order citation \
             (e.g. '[2026] VJS-CC-VJS 18') or decision-log id under which the digest moved. \
             Nothing was written. A re-pin carries a reason a reader can answer \
             (CC-VJS 12(d), applied to this lock by CC-VJS 18 C7).",
        ));
    }
    // An unreadable prior lock cannot be used to carry authorities forward. That is not a
    // reason to refuse the re-pin - re-pinning is exactly how an operator FIXES an unreadable
    // lock - so every entry is stamped with the given authority, which is then the truth.
    let prior = match read_lock(repo) {
        Lock::Pinned(m) => m,
        Lock::Absent | Lock::Unreadable(_) => BTreeMap::new(),
    };
    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let mut file = LockFile::default();
    for (path, digest) in surface_digests(repo) {
        match prior.get(&path) {
            // Unmoved: keep the entry verbatim, authority and timestamp included.
            Some(p) if p.digest == digest => file.entry.push(p.clone()),
            // New or moved: THIS is the digest that moved, so this is what the authority
            // is about.
            _ => file.entry.push(PinnedEntry {
                path,
                digest,
                authority: authority.to_string(),
                pinned_at: now.clone(),
            }),
        }
    }
    let body = toml::to_string_pretty(&file)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
    std::fs::write(repo.join(LOCK_PATH), format!("{LOCK_HEADER}{body}"))
}

fn read_lock(repo: &Path) -> Lock {
    let content = match std::fs::read_to_string(repo.join(LOCK_PATH)) {
        Ok(c) => c,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Lock::Absent,
        // A lock that is present but unreadable (permissions, a directory, bad UTF-8) is NOT
        // an un-pinned repo.
        Err(e) => return Lock::Unreadable(format!("cannot be read ({e})")),
    };
    let parsed: LockFile = match toml::from_str(&content) {
        Ok(v) => v,
        Err(e) => return Lock::Unreadable(format!("is not valid TOML ({e})")),
    };
    let mut map: BTreeMap<String, PinnedEntry> = BTreeMap::new();
    for e in parsed.entry {
        if e.path.trim().is_empty() || e.digest.trim().is_empty() {
            return Lock::Unreadable("carries an entry with an empty path or digest".into());
        }
        if e.authority.trim().is_empty() {
            return Lock::Unreadable(format!(
                "entry '{}' carries an EMPTY authority; every pinned digest must record the \
                 authority under which it moved (CC-VJS 18 C7)",
                e.path
            ));
        }
        if map.insert(e.path.clone(), e.clone()).is_some() {
            return Lock::Unreadable(format!(
                "names '{}' twice, so which digest is pinned is ambiguous",
                e.path
            ));
        }
    }
    Lock::Pinned(map)
}

/// PC-16 D4: surface a loud, blocking finding when a pinned gate-source file drifts from
/// the lock (a possibly-weakening edit), naming the entrenched instruments and routing to
/// the breach duty. No finding when the lock is absent (an un-pinned repo) - the pin is
/// opt-in via `vjs enforcement-lock`. Deterministic: sha256 over the gate bytes.
pub fn check_drift(repo: &Path) -> Vec<Finding> {
    let pinned = match read_lock(repo) {
        Lock::Absent => return Vec::new(),
        // THE SILENT DISARM THIS EXISTS TO PREVENT (CC-VJS 18 C7). Before the authority
        // field there was one parse and it could barely fail; the stricter parse creates
        // real ways to fail, and the previous code reported every one of them as "no lock,
        // no finding" - identical to an un-pinned repo. A lock that exists and cannot be
        // trusted is the LOUDEST case, not the quietest: it is exactly the state a weakening
        // edit would leave behind if it could corrupt the witness instead of tripping it.
        Lock::Unreadable(why) => {
            return vec![
                Finding::new(
                    Severity::Fatal,
                    "ENFORCEMENT_LOCK_UNREADABLE",
                    format!(
                        "The entrenched-enforcement lock ({LOCK_PATH}) exists but {why}. \
                         NOTHING WAS CHECKED: no gate digest was compared, so this is \
                         unverified, not verified-good."
                    ),
                )
                .citing("ACT-COMPUTER-FIRST-REALM:s14")
                .fix(
                    "Re-pin with `vjs enforcement-lock --authority '<order citation or \
                     decision-log id>'`, which regenerates the file in full. Do not hand-edit \
                     it back to green: the parse is what stands between a pinned surface and \
                     an unwitnessed one.",
                ),
            ];
        }
        Lock::Pinned(m) => m,
    };
    let current: BTreeMap<String, String> = surface_digests(repo).into_iter().collect();
    let mut findings = Vec::new();
    for rel in ENFORCEMENT_SURFACE {
        let want = pinned.get(*rel).map(|e| e.digest.clone());
        let got = current.get(*rel).cloned();
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
            "[[entry]]\npath = \"crates/vjs-core/src/bench.rs\"\n\
             digest = \"sha256:0000000000000000000000000000000000000000000000000000000000000000\"\n\
             authority = \"[2026] VJS-CC-VJS 18 C7 (test fixture)\"\npinned_at = \"2026-08-02T00:00:00Z\"\n",
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

    // ---------------------------------------------------------------------- //
    // [2026] VJS-CC-VJS 18 C7 - per-entry authority, and the parse that can now fail
    // ---------------------------------------------------------------------- //

    fn scratch(name: &str) -> std::path::PathBuf {
        let tmp = std::env::temp_dir().join(format!("vjs-enf-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".vjs")).unwrap();
        std::fs::create_dir_all(tmp.join("crates/vjs-core/src")).unwrap();
        tmp
    }

    fn authority_of(repo: &Path, rel: &str) -> String {
        match read_lock(repo) {
            Lock::Pinned(m) => m.get(rel).expect("entry is pinned").authority.clone(),
            Lock::Absent => panic!("the lock is absent"),
            Lock::Unreadable(w) => panic!("the lock is unreadable: {w}"),
        }
    }

    #[test]
    fn write_lock_refuses_without_an_authority_and_writes_nothing() {
        // C7's own "proof it can fail": the write is REFUSED and no file is written. The
        // no-file half is the load-bearing one - a refusal that still truncated the lock
        // would leave the operator with ENFORCEMENT_LOCK_UNREADABLE and no pin at all.
        let tmp = scratch("refuse");
        std::fs::write(tmp.join("crates/vjs-core/src/bench.rs"), "gate v1").unwrap();

        for empty in ["", "   ", "\t\n"] {
            let err = write_lock(&tmp, empty).expect_err("an empty authority must be refused");
            assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
            assert!(
                err.to_string().contains("--authority"),
                "the refusal must name the cure, got: {err}"
            );
            assert!(
                !tmp.join(LOCK_PATH).exists(),
                "REFUSED BUT WROTE ANYWAY on input {empty:?} - a partial lock is worse than none"
            );
        }

        // The control: the same call with an authority DOES write, so the refusals above are
        // not passing because write_lock is broken for every input.
        write_lock(&tmp, "[2026] VJS-CC-VJS 18").expect("a stamped re-pin is written");
        assert!(tmp.join(LOCK_PATH).exists());

        // And a refusal after a good lock exists leaves that lock untouched.
        let before = std::fs::read(tmp.join(LOCK_PATH)).unwrap();
        assert!(write_lock(&tmp, "  ").is_err());
        assert_eq!(
            before,
            std::fs::read(tmp.join(LOCK_PATH)).unwrap(),
            "a refused re-pin must not disturb the lock already on disk"
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn an_unmoved_digest_keeps_the_authority_it_already_carried() {
        // The false-provenance guard. C7 asks for "the authority under which the DIGEST
        // MOVED". Re-stamping every entry on every re-pin would record that an unrelated
        // ruling moved a file it never touched, which is a lie the file would then carry
        // indefinitely.
        let tmp = scratch("carry");
        let gate = tmp.join("crates/vjs-core/src/bench.rs");
        std::fs::write(&gate, "gate v1").unwrap();

        write_lock(&tmp, "[2026] FIRST").unwrap();
        assert_eq!(
            authority_of(&tmp, "crates/vjs-core/src/bench.rs"),
            "[2026] FIRST"
        );

        // A re-pin under a different authority, with this gate UNCHANGED.
        write_lock(&tmp, "[2026] SECOND").unwrap();
        assert_eq!(
            authority_of(&tmp, "crates/vjs-core/src/bench.rs"),
            "[2026] FIRST",
            "an unmoved digest was re-stamped with an authority that did not move it"
        );

        // Now MOVE it. This digest did change under SECOND's successor, so it takes it.
        std::fs::write(&gate, "gate v2 - a deliberate gate change").unwrap();
        write_lock(&tmp, "[2026] THIRD").unwrap();
        assert_eq!(
            authority_of(&tmp, "crates/vjs-core/src/bench.rs"),
            "[2026] THIRD",
            "a MOVED digest must take the authority of the re-pin that moved it"
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn a_lock_that_exists_and_cannot_be_trusted_is_fatal_not_silence() {
        // THE SILENT DISARM C7 WOULD OTHERWISE HAVE INTRODUCED. Every one of these used to
        // read as `None` - identical to an un-pinned repository - so a stricter parse would
        // have turned each into "no lock, no finding" on the day it landed.
        let cases: [(&str, &str); 4] = [
            (
                "flat",
                // the PRE-C7 format, which every existing lock is written in
                "# VJS pin\ncrates/vjs-core/src/bench.rs sha256:00\n",
            ),
            (
                "empty-authority",
                "[[entry]]\npath = \"a.rs\"\ndigest = \"sha256:00\"\nauthority = \"  \"\npinned_at = \"t\"\n",
            ),
            (
                "duplicate-path",
                "[[entry]]\npath = \"a.rs\"\ndigest = \"sha256:00\"\nauthority = \"x\"\npinned_at = \"t\"\n\
                 [[entry]]\npath = \"a.rs\"\ndigest = \"sha256:11\"\nauthority = \"x\"\npinned_at = \"t\"\n",
            ),
            (
                "empty-digest",
                "[[entry]]\npath = \"a.rs\"\ndigest = \"\"\nauthority = \"x\"\npinned_at = \"t\"\n",
            ),
        ];
        for (name, body) in cases {
            let tmp = scratch(name);
            std::fs::write(tmp.join(LOCK_PATH), body).unwrap();
            let findings = check_drift(&tmp);
            assert!(
                findings
                    .iter()
                    .any(|f| f.code == "ENFORCEMENT_LOCK_UNREADABLE"
                        && matches!(f.severity, Severity::Fatal)),
                "case {name:?}: an untrustworthy lock must be a Fatal \
                 ENFORCEMENT_LOCK_UNREADABLE, got {:?}",
                findings.iter().map(|f| &f.code).collect::<Vec<_>>()
            );
            let _ = std::fs::remove_dir_all(&tmp);
        }

        // THE NEGATIVE CONTROL, without which the above passes for the wrong reason: an
        // ABSENT lock must still be silence. The pin is opt-in, and a gate that fires on
        // every un-pinned repository would be switched off within a day.
        let bare = scratch("absent");
        assert!(
            check_drift(&bare).is_empty(),
            "an un-pinned repo must produce no finding - the pin is opt-in"
        );
        // ... and a WELL-FORMED lock must be silence too, or the code above is just
        // "always Fatal".
        std::fs::write(bare.join("crates/vjs-core/src/bench.rs"), "gate").unwrap();
        write_lock(&bare, "[2026] VJS-CC-VJS 18").unwrap();
        assert!(
            !check_drift(&bare)
                .iter()
                .any(|f| f.code == "ENFORCEMENT_LOCK_UNREADABLE"),
            "a lock this crate just wrote must be readable by this crate"
        );
        let _ = std::fs::remove_dir_all(&bare);
    }
}
