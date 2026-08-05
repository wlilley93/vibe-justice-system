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

use std::path::Path;

use vjs_core::report::Finding;
use vjs_core::types::Severity;

pub fn store_register_findings(repo: &Path, findings: &mut Vec<Finding>) {
    let reg_path = repo.join(".vjs/store-register.yaml");
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
    let empty = Vec::new();
    let stores: Vec<String> = parsed["stores"]
        .as_sequence()
        .unwrap_or(&empty)
        .iter()
        .filter_map(|s| s["path"].as_str().map(|p| p.to_string()))
        .collect();
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
    for s in &stores {
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
}
