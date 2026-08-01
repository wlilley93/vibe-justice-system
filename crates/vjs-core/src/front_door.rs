//! The single governed-record front door (PC-14, [2026] VJS-PC 14; given form by
//! REG-FRONT-DOOR-001).
//!
//! The unifying principle, declared correct by PC-13 and enacted by PC-14: no
//! governed record (law, ruling, citation, order, permit) may come into being
//! except through the kernel record-creation path - route -> permit, convene ->
//! court, allocate -> citation, record -> order. The existing deterministic gates
//! (the permit gate, the D1 canon-write gate, the D2 citation gate, the D7 tier
//! gate, the D10 bench gate) ARE that front door at the write/route chokepoint.
//!
//! What this module adds is the UNIFORM ASSENT FLOOR (PC-14 D3, giving teeth to the
//! entrenched ACT-ASSENTED-RECORD-PROTECTION): the limb bites HARD on a non-assented
//! off-door record (it is refused at the write, never brought into being), but the
//! instant a record DECLARES a valid assent_source the disposition DEGRADES to
//! route-for-correction - surfaced and flagged, never voided or blocked. The limb
//! keys ONLY on the KIND of record and whether it declares valid assent; it never
//! reaches the duty surface at large and never keys on the conformance-map counts.

/// The fail-closed allow-list of valid assent_source values (INV-ASSENT-SOURCE-001 /
/// INV-ASSENT-DRAFT-001). A record carrying any other value is NOT an assented
/// record and is not protected by the floor.
pub const VALID_ASSENT_SOURCES: &[&str] = &["sovereign_assent", "standing_bounded_assent"];

/// The code a downgraded (route-for-correction) finding carries, so the assent-floor
/// discipline is auditable and never reads as a clean pass. Registered in the
/// kernel route-for-correction set; never an Error/Fatal/Block.
pub const ROUTE_FOR_CORRECTION_CODE: &str = "ASSENTED_ROUTE_FOR_CORRECTION";

/// True when `value` is a valid assent_source per the INV-ASSENT-SOURCE-001 allow-list
/// (fail-closed). Use this on an order's `assent_source` FIELD - the bench gate and
/// the MCP record verb formerly accepted any non-empty string, which let a junk value
/// like `assent_source: made_it_up` soften a bench defect (bug fixed by routing both
/// through here).
pub fn is_valid_assent_value(value: &str) -> bool {
    let v = value.trim().trim_matches('"').trim_matches('\'').trim();
    VALID_ASSENT_SOURCES.contains(&v)
}

/// True when `content` declares a top-level `assent_source:` whose value is on the
/// allow-list. Deterministic, no LLM. Only the record's OWN top-level field counts
/// (column zero), so an assent_source mentioned inside prose does not protect it.
pub fn declares_valid_assent(content: &str) -> bool {
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("assent_source:") {
            let v = rest.trim().trim_matches('"').trim_matches('\'').trim();
            return VALID_ASSENT_SOURCES.contains(&v);
        }
    }
    false
}

/// One root of the governed-record body: where it lives, and whether only YAML under it
/// counts as a record (the canon tree carries manifests, provenance and prose alongside
/// its instruments; the court registers under `.vjs/` carry records only).
pub struct GovernedRoot {
    pub path: &'static str,
    pub yaml_only: bool,
}

/// THE declaration of what a governed record is. One list, read by the predicate
/// (`is_governed_record`) and by the scan (`governed_record_roots`) alike.
///
/// Anything that has to reason over the WHOLE body of governed records must read all
/// three roots, or it reasons over a fraction and reports the answer with full confidence.
/// That is not hypothetical: `live_citation_max` read `lawpack/v2` alone, which holds
/// 86 defining citations and NOT ONE of them County, so `vjs next-citation CC 2026`
/// returned `1` unconditionally while the series stood at 8. The canon PC series was
/// mis-allocating by the same mechanism, offering `[2026] VJS-PC 20` while that
/// citation was held.
///
/// Declared here rather than read from `.vjs/config.toml`: the config declares an
/// `orders` key but has no key for the court register, `PathsConfig` is non-Option, and
/// config.toml is itself a permit-required path.
///
/// It was TWO lists until [2026] VJS-CC-VJS 15. The doc comment on the scan said it was
/// "derived from `is_governed_record`"; it was a second hand-written copy of the same
/// three roots, sitting twenty lines below the first, and nothing made them agree. The
/// prose asserted the very property the code did not have.
///
/// LAWPACK-LITERAL: referent=local-records; status=local; authority=[2026] VJS-CC-VJS 15.
/// The referent is THIS repository's own records on its own disk, not the canon it reads
/// its law from, so this literal must NOT be re-pointed at the resolver: a subscriber's
/// own County orders and court register are its own, wherever its lawpack lives.
pub const GOVERNED_RECORD_ROOTS: &[GovernedRoot] = &[
    GovernedRoot {
        path: "lawpack/v2",
        yaml_only: true,
    },
    GovernedRoot {
        path: ".vjs/orders",
        yaml_only: false,
    },
    GovernedRoot {
        path: ".vjs/court",
        yaml_only: false,
    },
];

/// A governed record is a lawpack/v2 instrument or a court record under .vjs that
/// carries legal force (it is the KIND of thing the front door governs the creation
/// of). The floor applies to a finding ABOUT such a staged record.
pub fn is_governed_record(rel_path: &str) -> bool {
    let p = rel_path.replace('\\', "/");
    GOVERNED_RECORD_ROOTS.iter().any(|root| {
        p.starts_with(&format!("{}/", root.path))
            && (!root.yaml_only || p.ends_with(".yaml") || p.ends_with(".yml"))
    })
}

/// True when a repo-relative path lies inside the CANON tree (as opposed to the court
/// registers under `.vjs/`). Derived from the one declaration above, so a gate that has to
/// ask "is this path canon?" does not restate the root and drift from it - which is exactly
/// what `media_in_canon_findings` and the lawpack validator were both doing.
pub fn is_in_canon_tree(rel_path: &str) -> bool {
    let p = rel_path.replace('\\', "/");
    GOVERNED_RECORD_ROOTS
        .iter()
        .any(|r| r.yaml_only && p.starts_with(&format!("{}/", r.path)))
}

/// The roots `is_governed_record` recognises, as directories to scan.
pub fn governed_record_roots(repo: &std::path::Path) -> Vec<std::path::PathBuf> {
    GOVERNED_RECORD_ROOTS
        .iter()
        .map(|r| repo.join(r.path))
        .filter(|p| p.exists())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognises_valid_assent_only() {
        assert!(declares_valid_assent(
            "id: x\nassent_source: sovereign_assent\n"
        ));
        assert!(declares_valid_assent(
            "assent_source: standing_bounded_assent\n"
        ));
        assert!(!declares_valid_assent("id: x\nassent_source: made_it_up\n"));
        assert!(!declares_valid_assent(
            "id: x\n# assent_source: sovereign_assent\n"
        ));
        assert!(!declares_valid_assent("id: x\nstatus: binding\n"));
    }

    /// [2026] VJS-CC-VJS 15 C5: the predicate and the scan are two halves of ONE
    /// declaration and cannot drift. Before the collapse they were two hand-written
    /// lists; adding a root to either alone compiled, passed, and left the other half
    /// silently reasoning over a fraction of the record body. This drives BOTH halves
    /// over EVERY entry, so a root added to the const without the predicate agreeing
    /// (or vice versa) is a red test, not a quiet disagreement.
    #[test]
    fn the_predicate_and_the_scan_agree_on_every_root() {
        let repo = std::env::temp_dir().join(format!(
            "vjs-front-door-roots-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&repo);
        for root in GOVERNED_RECORD_ROOTS {
            std::fs::create_dir_all(repo.join(root.path)).unwrap();
        }

        let scanned = governed_record_roots(&repo);
        assert_eq!(
            scanned.len(),
            GOVERNED_RECORD_ROOTS.len(),
            "the scan must yield one directory per declared root"
        );
        for root in GOVERNED_RECORD_ROOTS {
            assert!(
                scanned.contains(&repo.join(root.path)),
                "the scan omits declared root {}",
                root.path
            );
            // The predicate must recognise a record under every root the scan walks,
            // or the scan reads files the floor does not treat as governed.
            let yaml = format!("{}/x.yaml", root.path);
            assert!(
                is_governed_record(&yaml),
                "the scan walks {} but the predicate rejects {yaml}",
                root.path
            );
            // And the yaml_only limb is the declaration's, not a second opinion.
            let other = format!("{}/x.md", root.path);
            assert_eq!(
                is_governed_record(&other),
                !root.yaml_only,
                "the predicate's extension rule must follow the declared yaml_only for {}",
                root.path
            );
        }
        // A path under no declared root is not a governed record.
        assert!(!is_governed_record("crates/x.yaml"));
        let _ = std::fs::remove_dir_all(&repo);
    }

    #[test]
    fn governed_record_kinds() {
        assert!(is_governed_record("lawpack/v2/orders/2026-VJS-PC-014.yaml"));
        assert!(is_governed_record("lawpack/v2/decisions/DEC-001.yaml"));
        assert!(is_governed_record(".vjs/orders/x.yaml"));
        assert!(!is_governed_record("crates/vjs-core/src/lib.rs"));
        assert!(!is_governed_record("README.md"));
    }
}
