//! The deterministic submit-decision permit clerk for runtime acts ([2026] VJS-PC 15
//! D5/D6). REG-KERNEL-001's clerk pointed at a RUNTIME act: deterministic, model-free,
//! network-free, the GRANT carrying `law_source[]` (the affirmative twin of "every
//! denial names its instrument"). The assent bifurcation (PC-14 D3 / VJS-ACT 10) is
//! hard-wired: an act declaring a valid assent_source is never hard-DENIED, only routed
//! for correction.
//!
//! The check lives HERE, in the kernel (the only smart point). A transport (CLI verb,
//! or a subscriber's own integration) is a thin shell that builds the envelope and
//! prints the result - no checking logic in the adapter.

use vjs_core::front_door::is_valid_assent_value;
use vjs_core::scope::{EntityScope, Resolution, resolve};
use vjs_lawpack::overlay::Overlay;

/// The envelope a subscriber submits for a runtime decision. The scope is the generic
/// entity-scope (subscriber-named dimensions); the verb is the subscriber's runtime act
/// (never canon-enumerated). `assent_source`, present and allow-listed, triggers the
/// VJS-ACT 10 floor.
#[derive(Clone, Debug)]
pub struct DecisionEnvelope {
    pub scope: EntityScope,
    pub verb: String,
    pub assent_source: Option<String>,
}

/// The terminal disposition of a submit-decision. For an ASSENTED act the only outcomes
/// are GRANT or ROUTE_FOR_CORRECTION (it may never be hard-DENIED, voided, or blocked);
/// an UN-assented act that breaches a covering instrument is DENIED with a named
/// instrument - the ordinary gate doing its work, not a void of an assented record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disposition {
    Grant,
    Deny,
    RouteForCorrection,
}

impl Disposition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Disposition::Grant => "GRANT",
            Disposition::Deny => "DENY",
            Disposition::RouteForCorrection => "ROUTE_FOR_CORRECTION",
        }
    }
}

/// The decision result. `law_source` is populated on GRANT (the instruments that
/// permitted it); `instrument` names the forbidding instrument on DENY / ROUTE.
#[derive(Clone, Debug)]
pub struct DecisionResult {
    pub disposition: Disposition,
    pub law_source: Vec<String>,
    pub instrument: Option<String>,
}

/// Resolve a runtime decision against the loaded overlay (canon-precedence, cascade
/// most-specific-first) and apply the assent bifurcation. Pure, deterministic, no model
/// and no I/O - the clerk. A GRANT always carries its `law_source[]`.
pub fn submit_decision(overlay: &Overlay, env: &DecisionEnvelope) -> DecisionResult {
    match resolve(&env.scope, &env.verb, &overlay.floors, &overlay.local) {
        Resolution::Permitted { law_source } => DecisionResult {
            disposition: Disposition::Grant,
            law_source,
            instrument: None,
        },
        Resolution::Forbidden { instrument } => {
            // VJS-ACT 10 floor: an act declaring a valid assent_source is never
            // hard-DENIED - its sole disposition is ROUTE_FOR_CORRECTION.
            let assented = env
                .assent_source
                .as_deref()
                .map(is_valid_assent_value)
                .unwrap_or(false);
            DecisionResult {
                disposition: if assented {
                    Disposition::RouteForCorrection
                } else {
                    Disposition::Deny
                },
                law_source: Vec::new(),
                instrument: Some(instrument),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vjs_core::scope::{Floor, LocalRule};

    fn scope(pairs: &[(&str, &str)]) -> EntityScope {
        EntityScope::new(
            pairs
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        )
    }

    fn overlay() -> Overlay {
        Overlay {
            floors: vec![Floor {
                id: "FLOOR-1".into(),
                scope: EntityScope::root(),
                forbids: vec!["charge".into()],
            }],
            local: vec![LocalRule {
                id: "LOCAL-1".into(),
                scope: scope(&[("a", "1")]),
                adds_forbids: vec!["refund".into()],
                grants: vec![],
                authority: None,
                assent_source: None,
            }],
        }
    }

    #[test]
    fn a_permitted_verb_grants_and_carries_law_source() {
        let r = submit_decision(
            &overlay(),
            &DecisionEnvelope {
                scope: scope(&[("a", "1")]),
                verb: "advance".into(),
                assent_source: None,
            },
        );
        assert_eq!(r.disposition, Disposition::Grant);
        assert!(r.law_source.contains(&"FLOOR-1".to_string()));
        assert!(r.law_source.contains(&"LOCAL-1".to_string()));
    }

    #[test]
    fn an_un_assented_breach_is_denied_with_a_named_instrument() {
        let r = submit_decision(
            &overlay(),
            &DecisionEnvelope {
                scope: scope(&[("a", "1")]),
                verb: "charge".into(),
                assent_source: None,
            },
        );
        assert_eq!(r.disposition, Disposition::Deny);
        assert_eq!(r.instrument.as_deref(), Some("FLOOR-1"));
    }

    #[test]
    fn an_assented_breach_is_routed_never_denied() {
        let r = submit_decision(
            &overlay(),
            &DecisionEnvelope {
                scope: scope(&[("a", "1")]),
                verb: "charge".into(),
                assent_source: Some("sovereign_assent".into()),
            },
        );
        // VJS-ACT 10: never hard-DENIED; routed for correction, instrument still named.
        assert_eq!(r.disposition, Disposition::RouteForCorrection);
        assert_eq!(r.instrument.as_deref(), Some("FLOOR-1"));
    }

    #[test]
    fn a_junk_assent_value_does_not_buy_the_floor() {
        let r = submit_decision(
            &overlay(),
            &DecisionEnvelope {
                scope: scope(&[("a", "1")]),
                verb: "charge".into(),
                assent_source: Some("made_it_up".into()),
            },
        );
        // Not on the allow-list -> no laundering -> denied like any un-assented breach.
        assert_eq!(r.disposition, Disposition::Deny);
    }
}
