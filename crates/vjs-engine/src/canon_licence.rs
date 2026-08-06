//! THE CANON'S OWN LICENCE, checked against the order that binds it.
//!
//! [2026] VJS-PC 11 D2 is a binding directive and `lexby` is the named actor:
//! "keep the canon a separate PUBLIC AGPL repo at the root with zero inbound
//! dependency". The holding puts it in terms - the canon "must REMAIN a separate,
//! PUBLIC, AGPL-3.0 repository" - and the opinions call the AGPL stamp
//! "non-negotiable", because it is what keeps the AGPL/MIT firewall of
//! REG-BUNDLE-001 standing by construction rather than by anyone's care.
//!
//! Nothing checked it. On 2026-07-11 an anonymising history squash replaced
//! `LICENSE` with PolyForm Noncommercial 1.0.0, and for close to a month the canon
//! shipped a licence its own binding law forbids while `Cargo.toml` and `NOTICE.md`
//! went on reciting AGPL-3.0. Three files, two answers, no gate, and the
//! contradiction was found by reading rather than by running. It very nearly went
//! the other way: the fix in flight when this was written was to update the two
//! reciting files to match `LICENSE`, which would have silenced the last signal
//! that anything was wrong and left the breach machine-invisible for good.
//!
//! So there are two duties here and they are deliberately separable:
//!
//! 1. CONSISTENCY. Whatever the canon's licence is, every place that states it must
//!    state the same one. This duty needs no view about which licence is right, and
//!    on its own it would have caught 2026-07-11 on the day.
//! 2. CONFORMITY. The canon's licence must be the one PC 11 D2 requires. This duty
//!    is the order's, not this file's.
//!
//! The gate reports them as different codes on purpose: a drift is a mistake, a
//! non-conforming licence is a breach of a binding order, and a cure for one is not
//! a cure for the other.
//!
//! WHAT THIS GATE DOES NOT DO: it does not change a licence, and no gate may. A
//! licence is a grant by the copyright holder with effect in real-world law, which
//! ACT-001:s3 places above everything in this corpus. The gate's whole job is to
//! make sure the question reaches a person instead of rotting in a diff.

use std::path::Path;

use vjs_core::report::Finding;
use vjs_core::types::Severity;

/// The licence PC 11 D2 requires of the canon. ONE copy, here, with its authority
/// named beside it. It is not derived from the order file's `must:` slug because
/// slug-scraping a licence identifier out of prose is a reader that fails open the
/// day someone rewords a directive.
const CANON_REQUIRED_LICENCE: &str = "AGPL-3.0-only";
const CANON_REQUIRED_LICENCE_ALT: &str = "AGPL-3.0";
const CANON_LICENCE_AUTHORITY: &str = "[2026] VJS-PC 11 D2";

/// True where THIS repository is the canon itself, rather than a subscriber holding
/// a vendored copy of its lawpack.
///
/// A subscriber pins the canon's lawpack, so `canonical = true` in the manifest is
/// NOT on its own a statement that the reader is standing in the canon. The kernel
/// SOURCE is: the canon is the repository the crates are written in, and a
/// subscriber consumes them as a dependency. Where a tree carries both, the AGPL
/// condition genuinely does reach it, so the conjunction is not merely a convenient
/// discriminator - it is the right one.
fn is_the_canon(repo: &Path) -> bool {
    if !repo.join("crates/vjs-core/src/lib.rs").is_file() {
        return false;
    }
    // LAWPACK-LITERAL: referent=local-records; status=local; authority=[2026] VJS-CC-VJS 21.
    // Deliberately NOT the resolver. The question here is "is THIS TREE the canon", and
    // the resolver answers "where does this jurisdiction read its law from" - which for a
    // subscriber points AT the canon and would make every subscriber answer yes. Reading
    // the in-tree manifest is the whole point: a vendored or out-of-tree lawpack must not
    // satisfy it.
    std::fs::read_to_string(repo.join("lawpack/v2/manifest.toml"))
        .map(|m| {
            m.lines()
                .any(|l| l.split('#').next().unwrap_or("").replace(' ', "") == "canonical=true")
        })
        .unwrap_or(false)
}

/// The `license = "..."` of `[workspace.package]`, read as a VALUE and not as a
/// substring: a file that merely mentions AGPL somewhere in a comment has not
/// declared it.
fn cargo_licence(repo: &Path) -> Option<String> {
    let text = std::fs::read_to_string(repo.join("Cargo.toml")).ok()?;
    text.lines()
        .map(|l| l.split('#').next().unwrap_or("").trim())
        .find_map(|l| {
            let rest = l.strip_prefix("license")?.trim_start();
            let rest = rest.strip_prefix('=')?.trim();
            Some(rest.trim_matches('"').trim_matches('\'').to_string())
        })
}

/// The licence `LICENSE` actually carries, identified from its own text.
///
/// Returns the SPDX-ish identifier where the text is recognised, and otherwise the
/// file's first non-empty line, so an unrecognised licence is reported BY NAME. A
/// gate that can only say "not the expected one" sends the reader back to the file
/// to find out what they have, and this one should not need to.
fn license_file_identity(repo: &Path) -> Option<(String, bool)> {
    let text = std::fs::read_to_string(repo.join("LICENSE")).ok()?;
    let head: String = text.chars().take(4000).collect::<String>().to_uppercase();
    if head.contains("GNU AFFERO GENERAL PUBLIC LICENSE") {
        return Some((CANON_REQUIRED_LICENCE.to_string(), true));
    }
    let first = text
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .unwrap_or("(empty)")
        .to_string();
    Some((first, false))
}

fn is_required(id: &str) -> bool {
    id == CANON_REQUIRED_LICENCE || id == CANON_REQUIRED_LICENCE_ALT
}

/// Is the conflict ON THE RECORD and before the person who can decide it?
///
/// Straight Fatal was the first shape and it was wrong in a way worth naming: only
/// the copyright holder can cure a licence, so a Fatal here refuses every commit
/// until they act - INCLUDING the self-file and the case papers that put the
/// question to them. A gate that blocks the route to its own cure does not protect
/// anything; it gets switched off by the first person in a hurry, which is how the
/// canon ends up with no licence gate at all.
///
/// So the rule is the one D4 settled for the store register three hours earlier, and
/// it is the same rule because it is the same problem: the unresolved state is
/// LAWFUL while it is recorded and awaiting its decision-maker, and unlawful the
/// moment it is silent. A self-filed breach under `.vjs/logs/breaches/` that names
/// the ACTUAL non-conforming identifier holds the finding at Warning.
///
/// Naming the identifier is what stops this being a box to tick: a filing cannot be
/// written in advance, cannot be generic, and goes stale by itself the moment the
/// licence changes again - at which point the Fatal returns, which is correct,
/// because a new licence is a new question.
///
/// This does NOT license publication. Publishing under a licence a binding order
/// forbids is the outward-facing act, and no record of the conflict makes it lawful.
fn conflict_is_on_the_record(repo: &Path, operative: &str) -> bool {
    let dir = repo.join(".vjs/logs/breaches");
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return false;
    };
    entries.flatten().any(|e| {
        std::fs::read_to_string(e.path())
            .map(|t| t.contains(operative) && t.contains("CANON-LICENCE"))
            .unwrap_or(false)
    })
}

pub fn canon_licence_findings(repo: &Path, findings: &mut Vec<Finding>) {
    if !is_the_canon(repo) {
        return;
    }
    let declared = cargo_licence(repo);
    let carried = license_file_identity(repo);

    match (&declared, &carried) {
        (None, _) => findings.push(crate::f(
            Severity::Fatal,
            "CANON-LICENCE-UNDECLARED",
            format!(
                "the canon's Cargo.toml declares no `license`, so nothing states the licence \
                 {CANON_LICENCE_AUTHORITY} binds it to. Declare it as \
                 `{CANON_REQUIRED_LICENCE}`."
            ),
        )),
        (_, None) => findings.push(crate::f(
            Severity::Fatal,
            "CANON-LICENCE-UNDECLARED",
            format!(
                "the canon carries no LICENSE file. {CANON_LICENCE_AUTHORITY} requires a \
                 public {CANON_REQUIRED_LICENCE} repository, and a licence nobody can read \
                 is not one."
            ),
        )),
        (Some(declared), Some((operative, recognised))) => {
            let recorded = conflict_is_on_the_record(repo, operative);
            let severity = if recorded {
                Severity::Warning
            } else {
                Severity::Fatal
            };
            let posture = if recorded {
                "The conflict is ON THE RECORD in .vjs/logs/breaches and held open for the \
                 copyright holder, which is why this is a Warning and not a Fatal. It does \
                 NOT authorise publication."
            } else {
                "Nothing in this tree records the conflict, which is what makes it Fatal."
            };

            // Duty 1: CONSISTENCY. Needs no view about which licence is right, and on
            // its own would have caught 2026-07-11 on the day.
            //
            // Asked on the ONE axis the gate can actually prove: is-it-AGPL. A raw
            // string comparison would set the file's first line - a TITLE - against an
            // SPDX identifier and report a difference in SPELLING as a difference in
            // grant, which is a check that cries wolf.
            //
            // Both directions here are proofs. LICENSE identified as AGPL while
            // Cargo.toml declares something else: they disagree. Cargo.toml declaring
            // AGPL while LICENSE is provably not AGPL: they disagree, and this is the
            // 2026-07-11 state exactly. Only the remaining case is unknowable - two
            // different non-AGPL spellings, which may or may not be one licence - and
            // there the gate says nothing about drift, because the conformity duty
            // below already reports that state in terms.
            let drifted = is_required(declared) != *recognised;
            if drifted {
                findings.push(crate::f(
                    severity.clone(),
                    "CANON-LICENCE-DRIFT",
                    format!(
                        "the canon states two different licences: Cargo.toml declares \
                         `{declared}` and LICENSE carries `{operative}`. A consumer reads \
                         LICENSE and a build tool reads Cargo.toml, so one repository grants \
                         two different sets of rights. Make them agree; where that means \
                         changing the grant, only the copyright holder can \
                         ({CANON_LICENCE_AUTHORITY}). {posture}"
                    ),
                ));
            }
            // Duty 2: CONFORMITY to the order. A separate code, because a drift is a
            // mistake and this is a breach of a binding directive, and a cure for one
            // is not a cure for the other.
            if !is_required(operative) {
                findings.push(crate::f(
                    severity,
                    "CANON-LICENCE-NOT-CONFORMING",
                    format!(
                        "the canon's LICENSE carries `{operative}`, and \
                         {CANON_LICENCE_AUTHORITY} binds the canon to remain a separate, \
                         PUBLIC, {CANON_REQUIRED_LICENCE} repository - the AGPL stamp is what \
                         holds the REG-BUNDLE-001 firewall up by construction. This gate \
                         cannot cure it: a licence is a grant by the copyright holder with \
                         effect in real-world law (ACT-001:s3), so the routes are the holder \
                         restoring the required licence, or the Court moving D2 on a fresh \
                         record. Publishing under a licence a binding order forbids is not \
                         one of them. {posture}"
                    ),
                ));
            }
        }
    }
}
