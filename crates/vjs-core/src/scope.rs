//! The generic entity-scope, the cascade algebra, and the anti-relaxation rule
//! ([2026] VJS-PC 15 D1/D2/D3). Substrate machinery under ACT-CONSOLIDATION-FRAMEWORK:s7
//! giving load-time teeth to ACT-007:s3/s4 and REG-FEDERATION-COORDINATION-001 - it
//! amends no statute, bench, tier, jurisdiction, or assent rule.
//!
//! Canon ships the EMPTY FRAME and the cascade algebra; a subscriber supplies its own
//! dimension NAMES, ordering, floors, rules, and runtime-act verbs in its Tier-2
//! lawpack. Canon hard-codes NO subscriber vocabulary. PC-15 D3 named one subscriber's
//! business hierarchy (org/ws/matter/flow/step) ONLY to EXCLUDE it; it appears nowhere
//! here, nor in any type, field, variant, or fixture, as a scope level.

use serde::{Deserialize, Serialize};

/// A vocabulary-free entity scope: an ORDERED list of `(dimension, value)` pairs,
/// broad -> narrow. `dims[0]` is the least specific (nearest the apex, where the canon
/// floors bind universally); `dims.last()` is the most specific. The dimension names
/// are opaque, subscriber-supplied strings - canon never reads or hard-codes one.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityScope {
    #[serde(default)]
    pub dims: Vec<(String, String)>,
}

impl EntityScope {
    pub fn new(dims: Vec<(String, String)>) -> Self {
        Self { dims }
    }

    /// The apex scope (no dimensions): where the canon floors sit and bind everything.
    pub fn root() -> Self {
        Self { dims: Vec::new() }
    }

    /// Specificity = depth down the cascade path. The apex is 0; each named dimension
    /// narrows by one. The cascade resolves most-specific-first, so a larger value is
    /// nearer the leaf.
    pub fn specificity(&self) -> usize {
        self.dims.len()
    }

    /// `self` COVERS `target` when `self` is a PREFIX of `target` on the cascade path
    /// (same dimension names AND values, in order) - i.e. `self` sits at or above
    /// `target`. The apex scope (no dims) covers everything: that is where the canon
    /// floors bind universally. Coverage is the cascade relation the loader and the
    /// resolver both key on.
    pub fn covers(&self, target: &EntityScope) -> bool {
        self.dims.len() <= target.dims.len()
            && self
                .dims
                .iter()
                .zip(&target.dims)
                .all(|(a, b)| a == b)
    }
}

/// A canon Tier-1 FLOOR: at `scope` (and everything below it on the cascade), every
/// verb in `forbids` is forbidden. Floors are read-only and always win
/// (canon-precedence, ACT-007:s3). Canon ships zero concrete floors in the frame; a
/// floor is enacted only by canon's own lawmaking, never by a subscriber.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Floor {
    pub id: String,
    #[serde(default)]
    pub scope: EntityScope,
    #[serde(default)]
    pub forbids: Vec<String>,
}

/// The ACT-007:s3 authority a Tier-2 divergence may DECLARE to authorise an override
/// of a canon floor: a Privy Council order, or a valid Principal assent_source. The
/// loader keys on the PRESENCE of a well-formed authority and never re-adjudicates it
/// (whether the order or assent event truly exists is the separate provenance question
/// reserved to the assent-resolution court).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverrideAuthority {
    /// A Privy Council order citation authorising the divergence.
    PrivyOrder(String),
    /// A declared Principal assent_source value (validated against the allow-list).
    AssentSource(String),
}

impl OverrideAuthority {
    /// A well-formed authority: a non-empty order citation, or an allow-listed assent
    /// value. `valid_assent` is injected (front_door::is_valid_assent_value) so this
    /// crate stays a leaf.
    pub fn is_present(&self, valid_assent: &impl Fn(&str) -> bool) -> bool {
        match self {
            OverrideAuthority::PrivyOrder(c) => !c.trim().is_empty(),
            OverrideAuthority::AssentSource(v) => valid_assent(v),
        }
    }
}

/// A subscriber Tier-2 LOCAL RULE at `scope`. It may ADD restriction freely
/// (`adds_forbids`); it may GRANT verbs (`grants`), but a grant of a verb a covering
/// canon floor forbids is a RELAXATION - void at load unless `authority` declares an
/// ACT-007:s3 override, or the rule itself bears a valid `assent_source` (then routed,
/// never voided, per VJS-ACT 10).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalRule {
    pub id: String,
    #[serde(default)]
    pub scope: EntityScope,
    #[serde(default)]
    pub adds_forbids: Vec<String>,
    #[serde(default)]
    pub grants: Vec<String>,
    #[serde(default)]
    pub authority: Option<OverrideAuthority>,
    #[serde(default)]
    pub assent_source: Option<String>,
}

/// The fate of a Tier-2 local rule once the canon floors are visible at load
/// (PC-15 D2). Only the offending rule is ever void; the floor stands, canon is never
/// touched, and a rule bearing valid assent is never void.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuleFate {
    /// Kept with force: it only adds restriction, or its relaxation is authorised by a
    /// present ACT-007:s3 override.
    Kept,
    /// Void at load: it relaxes a covering canon floor with no s3 authority and no
    /// assent. Only THIS rule is denied force. `floor`/`verb` name the instrument and
    /// the offending grant, for the named defect (never silently swallowed).
    Void { floor: String, verb: String },
    /// Routed for correction: it relaxes a floor BUT itself bears a valid assent_source
    /// - never void or block (VJS-ACT 10); surfaced and routed.
    RouteForCorrection { floor: String, verb: String },
}

/// Decide a local rule's fate against the canon floors (PC-15 D2). The first covering
/// floor that forbids a granted verb is a relaxation; its disposition then turns on the
/// s3 authority exception and the assent floor. Pure and deterministic.
pub fn rule_fate(
    rule: &LocalRule,
    floors: &[Floor],
    valid_assent: impl Fn(&str) -> bool,
) -> RuleFate {
    for verb in &rule.grants {
        for floor in floors {
            if floor.scope.covers(&rule.scope) && floor.forbids.iter().any(|v| v == verb) {
                // ACT-007:s3 authority exception: a present, well-formed override
                // authorises the divergence. Keyed on presence, never re-adjudicated.
                if rule
                    .authority
                    .as_ref()
                    .map(|a| a.is_present(&valid_assent))
                    .unwrap_or(false)
                {
                    return RuleFate::Kept;
                }
                // VJS-ACT 10 floor: a rule itself bearing a valid assent_source is
                // never void or blocked, only routed for correction.
                if rule
                    .assent_source
                    .as_deref()
                    .map(&valid_assent)
                    .unwrap_or(false)
                {
                    return RuleFate::RouteForCorrection {
                        floor: floor.id.clone(),
                        verb: verb.clone(),
                    };
                }
                return RuleFate::Void {
                    floor: floor.id.clone(),
                    verb: verb.clone(),
                };
            }
        }
    }
    RuleFate::Kept
}

/// The outcome of resolving a `(scope, verb)` decision against the loaded overlay
/// (PC-15 D1: cascade most-specific-first to the canon floors at the apex, canon
/// always winning).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Resolution {
    /// No covering floor or local restriction forbids the verb. `law_source` names
    /// every covering instrument consulted (the affirmative twin of "every denial
    /// names its instrument").
    Permitted { law_source: Vec<String> },
    /// A covering instrument forbids the verb. `instrument` names it.
    Forbidden { instrument: String },
}

/// Resolve a decision `(scope, verb)` against the canon floors (Tier-1) and the kept
/// local rules (Tier-2), canon-precedence. A covering FLOOR forbidding the verb wins
/// outright (canon always wins); else a covering local restriction forbids it; else
/// permitted, carrying every covering instrument as `law_source`. Pure and
/// deterministic - no model, no I/O.
pub fn resolve(scope: &EntityScope, verb: &str, floors: &[Floor], local: &[LocalRule]) -> Resolution {
    // Canon floors first: canon always wins.
    for floor in floors {
        if floor.scope.covers(scope) && floor.forbids.iter().any(|v| v == verb) {
            return Resolution::Forbidden {
                instrument: floor.id.clone(),
            };
        }
    }
    // Local additional restrictions (local law may only ADD restriction).
    for rule in local {
        if rule.scope.covers(scope) && rule.adds_forbids.iter().any(|v| v == verb) {
            return Resolution::Forbidden {
                instrument: rule.id.clone(),
            };
        }
    }
    let mut law_source: Vec<String> = floors
        .iter()
        .filter(|f| f.scope.covers(scope))
        .map(|f| f.id.clone())
        .collect();
    law_source.extend(
        local
            .iter()
            .filter(|r| r.scope.covers(scope))
            .map(|r| r.id.clone()),
    );
    Resolution::Permitted { law_source }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn allow(v: &str) -> bool {
        v == "sovereign_assent" || v == "standing_bounded_assent"
    }

    fn scope(pairs: &[(&str, &str)]) -> EntityScope {
        EntityScope::new(pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect())
    }

    #[test]
    fn apex_covers_everything_and_prefix_is_the_cascade() {
        assert!(EntityScope::root().covers(&scope(&[("a", "1"), ("b", "2")])));
        assert!(scope(&[("a", "1")]).covers(&scope(&[("a", "1"), ("b", "2")])));
        // Not a prefix: different value, or longer than the target.
        assert!(!scope(&[("a", "9")]).covers(&scope(&[("a", "1"), ("b", "2")])));
        assert!(!scope(&[("a", "1"), ("b", "2")]).covers(&scope(&[("a", "1")])));
    }

    #[test]
    fn a_local_grant_relaxing_a_covering_floor_is_void() {
        let floors = vec![Floor {
            id: "FLOOR-1".into(),
            scope: EntityScope::root(),
            forbids: vec!["charge".into()],
        }];
        let rule = LocalRule {
            id: "LOCAL-1".into(),
            scope: scope(&[("a", "1")]),
            adds_forbids: vec![],
            grants: vec!["charge".into()],
            authority: None,
            assent_source: None,
        };
        assert_eq!(
            rule_fate(&rule, &floors, allow),
            RuleFate::Void {
                floor: "FLOOR-1".into(),
                verb: "charge".into()
            }
        );
    }

    #[test]
    fn an_authorised_override_is_kept_and_a_pure_restriction_is_kept() {
        let floors = vec![Floor {
            id: "FLOOR-1".into(),
            scope: EntityScope::root(),
            forbids: vec!["charge".into()],
        }];
        // Authorised by a Privy order: kept (keyed on presence, not re-adjudicated).
        let authorised = LocalRule {
            id: "LOCAL-2".into(),
            scope: EntityScope::root(),
            adds_forbids: vec![],
            grants: vec!["charge".into()],
            authority: Some(OverrideAuthority::PrivyOrder("[2026] VJS-PC 99".into())),
            assent_source: None,
        };
        assert_eq!(rule_fate(&authorised, &floors, allow), RuleFate::Kept);
        // Pure restriction (adds_forbids only): always kept - local may add restriction.
        let restrict = LocalRule {
            id: "LOCAL-3".into(),
            scope: EntityScope::root(),
            adds_forbids: vec!["refund".into()],
            grants: vec![],
            authority: None,
            assent_source: None,
        };
        assert_eq!(rule_fate(&restrict, &floors, allow), RuleFate::Kept);
    }

    #[test]
    fn a_relaxing_rule_bearing_valid_assent_is_routed_not_void() {
        let floors = vec![Floor {
            id: "FLOOR-1".into(),
            scope: EntityScope::root(),
            forbids: vec!["charge".into()],
        }];
        let assented = LocalRule {
            id: "LOCAL-4".into(),
            scope: EntityScope::root(),
            adds_forbids: vec![],
            grants: vec!["charge".into()],
            authority: None,
            assent_source: Some("sovereign_assent".into()),
        };
        assert_eq!(
            rule_fate(&assented, &floors, allow),
            RuleFate::RouteForCorrection {
                floor: "FLOOR-1".into(),
                verb: "charge".into()
            }
        );
        // A junk assent value does NOT route - it is not on the allow-list (no
        // laundering); it is void like any other un-authorised relaxation.
        let junk = LocalRule {
            assent_source: Some("made_it_up".into()),
            ..assented
        };
        assert!(matches!(rule_fate(&junk, &floors, allow), RuleFate::Void { .. }));
    }

    #[test]
    fn resolve_is_canon_precedence_most_specific_first() {
        let floors = vec![Floor {
            id: "FLOOR-1".into(),
            scope: EntityScope::root(),
            forbids: vec!["charge".into()],
        }];
        let local = vec![LocalRule {
            id: "LOCAL-1".into(),
            scope: scope(&[("a", "1")]),
            adds_forbids: vec!["refund".into()],
            grants: vec![],
            authority: None,
            assent_source: None,
        }];
        // Canon floor forbids 'charge' everywhere.
        assert_eq!(
            resolve(&scope(&[("a", "1")]), "charge", &floors, &local),
            Resolution::Forbidden { instrument: "FLOOR-1".into() }
        );
        // Local restriction forbids 'refund' under a=1.
        assert_eq!(
            resolve(&scope(&[("a", "1")]), "refund", &floors, &local),
            Resolution::Forbidden { instrument: "LOCAL-1".into() }
        );
        // 'advance' is permitted and carries the covering instruments as law_source.
        match resolve(&scope(&[("a", "1")]), "advance", &floors, &local) {
            Resolution::Permitted { law_source } => {
                assert!(law_source.contains(&"FLOOR-1".to_string()));
                assert!(law_source.contains(&"LOCAL-1".to_string()));
            }
            other => panic!("expected permitted, got {other:?}"),
        }
        // Outside the local rule's scope, the local restriction does not bind.
        assert!(matches!(
            resolve(&scope(&[("a", "9")]), "refund", &floors, &local),
            Resolution::Permitted { .. }
        ));
    }
}
