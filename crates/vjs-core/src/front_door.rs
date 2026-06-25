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

/// A governed record is a lawpack/v2 instrument or a court record under .vjs that
/// carries legal force (it is the KIND of thing the front door governs the creation
/// of). The floor applies to a finding ABOUT such a staged record.
pub fn is_governed_record(rel_path: &str) -> bool {
    let p = rel_path.replace('\\', "/");
    (p.starts_with("lawpack/v2/") && (p.ends_with(".yaml") || p.ends_with(".yml")))
        || p.starts_with(".vjs/orders/")
        || p.starts_with(".vjs/court/")
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

    #[test]
    fn governed_record_kinds() {
        assert!(is_governed_record("lawpack/v2/orders/2026-VJS-PC-014.yaml"));
        assert!(is_governed_record("lawpack/v2/decisions/DEC-001.yaml"));
        assert!(is_governed_record(".vjs/orders/x.yaml"));
        assert!(!is_governed_record("crates/vjs-core/src/lib.rs"));
        assert!(!is_governed_record("README.md"));
    }
}
