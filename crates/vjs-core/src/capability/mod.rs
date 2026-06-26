//! The unified capability primitive (global invariants K-4..K-11).
//!
//! One authority record, generic over an OPAQUE subscriber-supplied resource vocabulary (the
//! `Resource` trait), so canon never learns what a resource MEANS (the canon-boundary stays intact,
//! per PC-15 EntityScope discipline and PC-19 K2). Deterministic, model-free, network-free
//! (REG-KERNEL-001). The built-in `TypedResource` is one vocabulary; a caller supplies its own (the
//! VJS permit gate supplies a path-glob `PathScope` in governance.rs, so a permit BECOMES a profile
//! of this record, the primitive being the live pre-write authorization engine, not shelf-ware).
//!
//! The primitive is split into two cohesive files: `resource` (the opaque vocabulary - the `Resource`
//! trait + the built-in `TypedResource`) and `store` (the capability record + the `CapabilityStore`
//! authorization engine and its verbs). This module root holds the shared `CapError`, re-exports the
//! public surface unchanged, and carries the per-property test suite.
//!
//! Properties (each bound to a test below):
//!  - K-4  the record shape; `*` as a right is rejected; rights are explicit.
//!  - K-5  deny-dominance: an active matching Deny dominates all Allows; no hidden precedence.
//!  - K-6  one-shot consumed exactly once: reserve-before-effect, refund-on-failure, auto-revoke.
//!  - K-7  attenuating-only delegation; finite-use cannot be delegated; parent revoke kills child.
//!  - K-8  revocation-on-next-check: re-evaluated against live state every authorize.
//!  - K-9  typed resources, terminal wildcards, prefix-collision rejection.
//!  - K-10 names are not capabilities: knowing a cap_id/resource string grants nothing.
//!  - K-11 grant is transfer, not minting: transfer only rights you hold; never mint deny/ask.

mod resource;
mod store;

pub use resource::{Resource, TypedResource};
pub use store::{Capability, CapabilityStore, Decision, DenyReason, Effect, Reservation, Status};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapError {
    BareWildcard,
    Untyped,
    NonTerminalWildcard,
    WildcardRight,
    NoRights,
    NotADelegableAllow,
    DepthExceeded,
    FiniteCannotDelegate,
    NotAttenuating,
    TransferExceedsHeld,
    CannotMintRestrictive,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a TypedResource pattern/request for the typed-vocabulary tests.
    fn tr(s: &str) -> TypedResource {
        TypedResource::parse(s).unwrap()
    }

    fn allowed(d: &Decision) -> bool {
        matches!(d, Decision::Allowed { .. })
    }

    #[test]
    fn k4_record_rejects_wildcard_right_and_requires_explicit_rights() {
        let mut s = CapabilityStore::new();
        assert_eq!(
            s.issue_root("a", tr("fs:src/main"), &["*"], Effect::Allow, None),
            Err(CapError::WildcardRight)
        );
        assert_eq!(
            s.issue_root("a", tr("fs:src/main"), &[], Effect::Allow, None),
            Err(CapError::NoRights)
        );
        assert!(s
            .issue_root("a", tr("fs:src/main"), &["read"], Effect::Allow, None)
            .is_ok());
    }

    #[test]
    fn k5_deny_dominates_overlapping_allow() {
        let mut s = CapabilityStore::new();
        s.issue_root("a", tr("fs:*"), &["read"], Effect::Allow, None)
            .unwrap();
        s.issue_root("a", tr("fs:secret"), &["read"], Effect::Deny, None)
            .unwrap();
        assert!(allowed(&s.authorize("a", tr("fs:public"), "read")));
        assert!(matches!(
            s.authorize("a", tr("fs:secret"), "read"),
            Decision::Denied {
                reason: DenyReason::ExplicitDeny
            }
        ));
    }

    #[test]
    fn k6_one_shot_is_consumed_exactly_once_and_refunds() {
        let mut s = CapabilityStore::new();
        let _ = s
            .issue_root("a", tr("fs:f"), &["write"], Effect::Allow, Some(1))
            .unwrap();
        // reserve-before-effect: first authorize consumes and auto-revokes
        let d1 = s.authorize("a", tr("fs:f"), "write");
        assert!(allowed(&d1));
        // a second concurrent attempt cannot cross again
        assert!(matches!(
            s.authorize("a", tr("fs:f"), "write"),
            Decision::Denied {
                reason: DenyReason::Revoked | DenyReason::NoCapability | DenyReason::Exhausted
            }
        ));
        // refund on provider failure restores the one-shot
        if let Decision::Allowed {
            reservation: Some(r),
        } = d1
        {
            s.refund(r);
        }
        assert!(allowed(&s.authorize("a", tr("fs:f"), "write")));
    }

    #[test]
    fn k7_delegation_only_attenuates_and_finite_cannot_delegate() {
        let mut s = CapabilityStore::new();
        let parent = s
            .issue_root("a", tr("fs:src/*"), &["read", "write"], Effect::Allow, None)
            .unwrap();
        // widening rights or resource is refused
        assert_eq!(
            s.delegate(&parent, "b", tr("fs:src/*"), &["read", "write", "delete"]),
            Err(CapError::NotAttenuating)
        );
        assert_eq!(
            s.delegate(&parent, "b", tr("fs:*"), &["read"]),
            Err(CapError::NotAttenuating)
        );
        // a faithful attenuation is allowed
        s.delegate(&parent, "b", tr("fs:src/lib"), &["read"])
            .unwrap();
        assert!(allowed(&s.authorize("b", tr("fs:src/lib"), "read")));
        // parent revocation transitively kills the child
        s.revoke(&parent);
        assert!(matches!(
            s.authorize("b", tr("fs:src/lib"), "read"),
            Decision::Denied { .. }
        ));
        // a finite-use capability cannot be delegated onward
        let finite = s
            .issue_root("a", tr("fs:g"), &["read"], Effect::Allow, Some(2))
            .unwrap();
        assert_eq!(
            s.delegate(&finite, "b", tr("fs:g"), &["read"]),
            Err(CapError::FiniteCannotDelegate)
        );
    }

    #[test]
    fn k8_revocation_takes_effect_on_the_next_authorize() {
        let mut s = CapabilityStore::new();
        let c = s
            .issue_root("a", tr("fs:f"), &["read"], Effect::Allow, None)
            .unwrap();
        assert!(allowed(&s.authorize("a", tr("fs:f"), "read")));
        s.revoke(&c);
        assert!(matches!(
            s.authorize("a", tr("fs:f"), "read"),
            Decision::Denied {
                reason: DenyReason::NoCapability
            }
        ));
    }

    #[test]
    fn k9_typed_resource_prefix_collision_and_terminal_wildcard() {
        assert_eq!(TypedResource::parse("*"), Err(CapError::BareWildcard));
        assert_eq!(TypedResource::parse("untyped"), Err(CapError::Untyped));
        assert_eq!(
            TypedResource::parse("fs:sr*c"),
            Err(CapError::NonTerminalWildcard)
        );
        let pat = TypedResource::parse("fs:src/*").unwrap();
        assert!(pat.covers(&TypedResource::parse("fs:src/main").unwrap()));
        assert!(!pat.covers(&TypedResource::parse("fs:src2/main").unwrap())); // prefix-collision
        assert!(!pat.covers(&TypedResource::parse("net:src/main").unwrap())); // kind mismatch
    }

    #[test]
    fn k10_names_are_not_capabilities() {
        let mut s = CapabilityStore::new();
        // 'a' holds nothing; merely naming a real-looking resource/id grants nothing.
        assert!(matches!(
            s.authorize("a", tr("fs:src/main"), "read"),
            Decision::Denied {
                reason: DenyReason::NoCapability
            }
        ));
        // even after a capability exists for ANOTHER subject, the name doesn't carry to 'a'.
        s.issue_root("owner", tr("fs:src/main"), &["read"], Effect::Allow, None)
            .unwrap();
        assert!(matches!(
            s.authorize("a", tr("fs:src/main"), "read"),
            Decision::Denied {
                reason: DenyReason::NoCapability
            }
        ));
    }

    #[test]
    fn k11_grant_is_transfer_not_minting() {
        let mut s = CapabilityStore::new();
        s.issue_root("a", tr("fs:src/*"), &["read"], Effect::Allow, None)
            .unwrap();
        // transfer a right the actor holds: ok
        assert!(s
            .grant_by_transfer("a", "b", tr("fs:src/lib"), &["read"], Effect::Allow)
            .is_ok());
        // transfer a right the actor does NOT hold: refused
        assert_eq!(
            s.grant_by_transfer("a", "b", tr("fs:src/lib"), &["write"], Effect::Allow),
            Err(CapError::TransferExceedsHeld)
        );
        // minting a restrictive (deny/ask) record via grant: refused
        assert_eq!(
            s.grant_by_transfer("a", "b", tr("fs:src/lib"), &["read"], Effect::Deny),
            Err(CapError::CannotMintRestrictive)
        );
    }
}
