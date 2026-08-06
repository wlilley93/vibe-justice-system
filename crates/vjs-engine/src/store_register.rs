//! THE STORE REGISTER (ACT-PROCEEDINGS-DISCIPLINE s13): every store capable of
//! holding a governed record or a citation is REGISTERED, and the register is
//! checked, not recited. "An enumeration that omits a store is not an audit of the
//! jurisdiction" - and a search over unregistered stores is a bounded search nobody
//! bounded on purpose.
//!
//! The register lives at `.vjs/store-register.yaml`: one entry per store with its
//! path, kind and registration note. The gate holds three duties the audit carried
//! as unwired since enactment: the governed record roots must EQUAL the register's
//! coverage (a root missing from the register is Fatal - the register is the map,
//! and a map missing a continent is not a map), a registered store that does not
//! exist is a Warning (ghost entries rot the register's authority), and an ABSENT
//! register is an Info disclosure, never a silent pass - a fresh jurisdiction arms
//! it by writing the file, exactly the ratchet's own arming pattern.
//!
//! A FOURTH duty, added under [2026] VJS-CC-VJS 20 D4: an entry that was on the
//! register at HEAD and is not on it now must SAY SO. "Deregistration is lawful;
//! silent deregistration is not." The occasion was a live one - on 2026-08-06 a
//! YAML-aware repair reserialised this very file and silently dropped its header
//! and two entries. The three duties above did not catch it as a lost entry; the
//! completeness duty happened to fire because the dropped stores were also governed
//! roots, which is luck, not a gate. A store that is registered but NOT a governed
//! root (the `.justice` citator, an archive, a subscriber's own store) would have
//! vanished in total silence, and the register would have kept reporting itself run.
//! The witness compares the register against its own committed self and refuses the
//! difference unless the register names it in `deregistered:` with a reason and an
//! authority. That keeps the lawful act one line of YAML away and makes the silent
//! one impossible.

use std::collections::HashSet;
use std::path::Path;

use vjs_core::report::Finding;
use vjs_core::types::Severity;
use vjs_git::GitIntegration;

const REGISTER_REL: &str = ".vjs/store-register.yaml";

/// `stores[].path`, normalised the way every comparison in this module wants them.
fn store_paths(parsed: &serde_yaml::Value) -> Vec<String> {
    let empty = Vec::new();
    parsed["stores"]
        .as_sequence()
        .unwrap_or(&empty)
        .iter()
        .filter_map(|s| s["path"].as_str())
        .map(|p| p.trim_end_matches('/').to_string())
        .collect()
}

pub fn store_register_findings(repo: &Path, findings: &mut Vec<Finding>) {
    let reg_path = repo.join(REGISTER_REL);
    let Ok(text) = std::fs::read_to_string(&reg_path) else {
        findings.push(crate::f(
            Severity::Info,
            "STORE-REGISTER-UNTRACKED",
            format!(
                "the store register DID NOT RUN - no register at {}. This is a disclosure, \
                 not a pass; write the register to arm the gate (ACT-PROCEEDINGS-DISCIPLINE \
                 s13).",
                reg_path.display()
            ),
        ));
        return;
    };
    let parsed: serde_yaml::Value = match serde_yaml::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            findings.push(crate::f(
                Severity::Fatal,
                "STORE-REGISTER-GARBLED",
                format!(
                    "the store register at {} does not parse ({e}) - an unreadable register \
                     registers nothing.",
                    reg_path.display()
                ),
            ));
            return;
        }
    };
    let stores = store_paths(&parsed);
    if stores.is_empty() {
        findings.push(crate::f(
            Severity::Fatal,
            "STORE-REGISTER-GARBLED",
            "the store register carries no `stores:` entries with `path:` - it would \
             certify completeness over nothing while reporting itself as run."
                .into(),
        ));
        return;
    }
    // Every governed record root is registered, by the SAME derivation the front door
    // uses - so the register cannot silently drift from what the kernel governs.
    for root in vjs_core::front_door::governed_record_roots(repo) {
        let rel = root
            .strip_prefix(repo)
            .unwrap_or(&root)
            .to_string_lossy()
            .to_string();
        if !stores.iter().any(|s| s.trim_end_matches('/') == rel) {
            findings.push(crate::f(
                Severity::Fatal,
                "STORE-UNREGISTERED",
                format!(
                    "governed record root '{rel}' is not in the store register - an \
                     enumeration that omits a store is not an audit of the jurisdiction \
                     (ACT-PROCEEDINGS-DISCIPLINE s13). Register it in .vjs/store-register.yaml."
                ),
            ));
        }
    }
    // The continuity citator is a CITATION-BEARING store by definition (s13 reaches
    // every store capable of holding a governed record OR a citation), and it is
    // nameable, so it is enforced by name: a `.justice` tree present on disk but
    // absent from the register is exactly the omission the Act prosecutes. Found by
    // live probe 2026-08-05: the first version only enforced the governed roots, so
    // deleting the .justice entry passed silently while the registry row claimed
    // otherwise - an overclaim, cured here by making the claim true.
    if repo.join(".justice").is_dir()
        && !stores.iter().any(|s| s.trim_end_matches('/') == ".justice")
    {
        findings.push(crate::f(
            Severity::Fatal,
            "STORE-UNREGISTERED",
            "the continuity citator .justice exists in this tree and is not in the store \
             register - a citation-bearing store the audit would sweep past \
             (ACT-PROCEEDINGS-DISCIPLINE s13; the ACT 13 s4 respelled duty). Register it."
                .into(),
        ));
    }

    // Every registered store exists; a ghost entry rots the register's authority.
    //
    // EXCEPT where the register itself said the store would not be here. Once
    // [2026] VJS-CC-VJS 20 D2 untracked six roots, a fresh clone legitimately carries
    // none of them, and every subscriber's first `vjs validate` opened with six
    // warnings about stores the canon had deliberately stopped publishing. A ghost is
    // a store the register FORGOT was gone; an entry marked `published: false` is a
    // store the register predicted would be absent, and predicting it is the opposite
    // of forgetting it. The two must not read the same.
    //
    // This does not weaken the duty on the author's own disk: `published: false` is a
    // claim about what travels, not about what exists here, so a store that is missing
    // WHERE IT LIVES is still a ghost and still warns.
    let unpublished: HashSet<String> = parsed["stores"]
        .as_sequence()
        .unwrap_or(&Vec::new())
        .iter()
        .filter(|s| s["published"].as_bool() == Some(false))
        .filter_map(|s| s["path"].as_str())
        .map(|p| p.trim_end_matches('/').to_string())
        .collect();
    for s in &stores {
        if unpublished.contains(s) {
            continue;
        }
        if !repo.join(s).exists() {
            findings.push(crate::f(
                Severity::Warning,
                "STORE-REGISTER-GHOST",
                format!(
                    "the store register names '{s}', which does not exist in this tree - \
                     remove the entry or restore the store; a register with ghosts cannot \
                     be the map the audit sweeps by."
                ),
            ));
        }
    }

    lost_entry_witness(repo, &parsed, &stores, findings);
}

/// [2026] VJS-CC-VJS 20 D4. An entry on the register at HEAD and absent from the
/// register on disk is a DEREGISTRATION, and deregistration must be said out loud:
/// name the path under `deregistered:` with a `reason:` and an `authority:`, and the
/// witness is satisfied. Say nothing and it is Fatal.
///
/// The witness declines to run rather than guess in exactly one case, and discloses
/// when it does: the register is committed at HEAD but git will not hand over the
/// blob. It stays SILENT where the register is simply not committed yet, because
/// that is not a bounded search - a register with no committed self has provably
/// lost nothing, and a fresh subscriber must not be greeted by a finding about a
/// history they do not have.
fn lost_entry_witness(
    repo: &Path,
    parsed: &serde_yaml::Value,
    on_disk: &[String],
    findings: &mut Vec<Finding>,
) {
    let tracked = match GitIntegration::tracked_at_head(repo) {
        Ok(t) => t,
        Err(_) => return,
    };
    if !tracked.contains(REGISTER_REL) {
        return;
    }
    let head_text = match GitIntegration::read_blob_at_head(repo, REGISTER_REL) {
        Ok(Some(t)) => t,
        _ => {
            findings.push(crate::f(
                Severity::Info,
                "STORE-REGISTER-WITNESS-UNRUN",
                format!(
                    "the lost-entry witness DID NOT RUN: {REGISTER_REL} is committed at HEAD \
                     but its blob could not be read. This is a statement about this checkout, \
                     never a finding that nothing was lost ([2026] VJS-CC-VJS 20 D4)."
                ),
            ));
            return;
        }
    };
    let Ok(head_parsed) = serde_yaml::from_str::<serde_yaml::Value>(&head_text) else {
        // A garbled committed register is already the GARBLED fatal's business the
        // next time it is read; here it simply means there is no prior set to
        // compare against, and inventing one would invent the loss too.
        return;
    };
    let now: HashSet<&str> = on_disk.iter().map(|s| s.as_str()).collect();
    let empty = Vec::new();
    let declared: HashSet<String> = parsed["deregistered"]
        .as_sequence()
        .unwrap_or(&empty)
        .iter()
        .filter_map(|d| d["path"].as_str())
        .map(|p| p.trim_end_matches('/').to_string())
        .collect();

    for was in store_paths(&head_parsed) {
        if now.contains(was.as_str()) || declared.contains(&was) {
            continue;
        }
        findings.push(crate::f(
            Severity::Fatal,
            "STORE-REGISTER-ENTRY-LOST",
            format!(
                "'{was}' was on the store register at HEAD and is on it no longer, and \
                 nothing in this tree says so. Deregistration is lawful; silent \
                 deregistration is not ([2026] VJS-CC-VJS 20 D4). Either restore the \
                 entry, or record it under `deregistered:` in {REGISTER_REL} with a \
                 `reason:` and the `authority:` that permitted it."
            ),
        ));
    }
}
