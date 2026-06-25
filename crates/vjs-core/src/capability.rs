//! The unified capability primitive (global invariants K-4..K-11).
//!
//! One authority record, generic over an OPAQUE subscriber-supplied resource vocabulary, so
//! canon never learns what a `kind` means (the canon-boundary stays intact - PC-15 EntityScope
//! discipline). Deterministic, model-free, network-free (REG-KERNEL-001). A VJS permit and an
//! Acmeco bearer-grant both become profiles of this record. Synthesized from Agent kernelB's
//! capability engine; granted by the PC-19 keystone reference (K2) on the non-narrowing
//! conditions that it stay generic and that the entrenched assent floor dominate every
//! capability decision (a capability governs PROSPECTIVE authorisation only; it may never void
//! or downgrade an assented record).
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

use std::collections::{BTreeMap, BTreeSet};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    Allow,
    Deny,
    Ask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Active,
    Revoked,
    Disabled,
}

/// A typed, canonical resource `kind:body`. As a capability PATTERN, `body` may be `*`
/// (whole kind) or end `/*` (a subtree); a concrete REQUEST is always exact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedResource {
    pub kind: String,
    pub body: String,
}

impl TypedResource {
    pub fn parse(s: &str) -> Result<Self, CapError> {
        if s == "*" {
            return Err(CapError::BareWildcard); // K-9: bare global wildcard rejected
        }
        let (kind, body) = s.split_once(':').ok_or(CapError::Untyped)?;
        if kind.is_empty() || body.is_empty() {
            return Err(CapError::Untyped);
        }
        // K-9: wildcards are terminal-only.
        if body.contains('*') && body != "*" && !body.ends_with("/*") {
            return Err(CapError::NonTerminalWildcard);
        }
        Ok(Self {
            kind: kind.to_string(),
            body: body.to_string(),
        })
    }

    /// Does this pattern COVER a concrete requested resource? Prefix-collision-safe:
    /// `k:src/*` covers `k:src/main` but NOT `k:src2/main`.
    pub fn covers(&self, req: &TypedResource) -> bool {
        if self.kind != req.kind {
            return false;
        }
        if self.body == "*" {
            return true;
        }
        if let Some(prefix) = self.body.strip_suffix("/*") {
            return req.body == prefix || req.body.starts_with(&format!("{prefix}/"));
        }
        self.body == req.body
    }

    /// Is `self` (a child pattern) within `parent` (no widening)? Used for delegation attenuation.
    pub fn within(&self, parent: &TypedResource) -> bool {
        if self.kind != parent.kind {
            return false;
        }
        match (parent.body.as_str(), self.body.strip_suffix("/*")) {
            ("*", _) => true,
            (_, Some(child_prefix)) => parent.covers(&TypedResource {
                kind: self.kind.clone(),
                body: child_prefix.to_string(),
            }),
            (_, None) => parent.covers(self),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Capability {
    pub cap_id: String,
    pub subject: String,
    pub resource: TypedResource,
    pub rights: BTreeSet<String>,
    pub effect: Effect,
    pub issuer: String,
    pub parent_cap_id: Option<String>,
    pub delegation_depth: u32,
    pub max_delegation_depth: u32,
    /// None = unlimited; Some(n) = finite (one-shot when n == 1).
    pub uses_remaining: Option<u32>,
    pub status: Status,
    pub constraints: BTreeMap<String, String>,
}

impl Capability {
    fn is_finite(&self) -> bool {
        self.uses_remaining.is_some()
    }
}

fn checked_rights(rights: &[&str]) -> Result<BTreeSet<String>, CapError> {
    if rights.is_empty() {
        return Err(CapError::NoRights);
    }
    if rights.contains(&"*") {
        return Err(CapError::WildcardRight); // K-4: no all-rights wildcard
    }
    Ok(rights.iter().map(|r| r.to_string()).collect())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DenyReason {
    ExplicitDeny,
    NoCapability,
    Revoked,
    Exhausted,
}

#[derive(Debug)]
pub enum Decision {
    Allowed { reservation: Option<Reservation> },
    Denied { reason: DenyReason },
}

/// A reservation against a finite capability: the use was decremented before the effect; the
/// caller commits (keep) or refunds (restore) it (reserve-before-effect, K-6).
#[derive(Debug, Clone)]
pub struct Reservation {
    cap_id: String,
    prev_uses: Option<u32>,
    revoked_by_this: bool,
}

#[derive(Default)]
pub struct CapabilityStore {
    caps: Vec<Capability>,
    next: u64,
}

impl CapabilityStore {
    pub fn new() -> Self {
        Self::default()
    }

    fn mint_id(&mut self) -> String {
        self.next += 1;
        format!("cap-{}", self.next)
    }

    fn get(&self, id: &str) -> Option<&Capability> {
        self.caps.iter().find(|c| c.cap_id == id)
    }
    fn get_mut(&mut self, id: &str) -> Option<&mut Capability> {
        self.caps.iter_mut().find(|c| c.cap_id == id)
    }

    /// A capability and its whole ancestor chain are Active (parent revoke transitively kills).
    fn chain_active(&self, cap: &Capability) -> bool {
        if cap.status != Status::Active {
            return false;
        }
        let mut seen = BTreeSet::new();
        let mut cur = cap.parent_cap_id.clone();
        while let Some(pid) = cur {
            if !seen.insert(pid.clone()) {
                return false; // cycle guard
            }
            match self.get(&pid) {
                Some(p) if p.status == Status::Active => cur = p.parent_cap_id.clone(),
                _ => return false,
            }
        }
        true
    }

    /// Issue a root capability directly (trusted bootstrap path).
    pub fn issue_root(
        &mut self,
        subject: &str,
        resource: &str,
        rights: &[&str],
        effect: Effect,
        uses_remaining: Option<u32>,
    ) -> Result<String, CapError> {
        let res = TypedResource::parse(resource)?;
        let rights = checked_rights(rights)?;
        let id = self.mint_id();
        self.caps.push(Capability {
            cap_id: id.clone(),
            subject: subject.to_string(),
            resource: res,
            rights,
            effect,
            issuer: "root".to_string(),
            parent_cap_id: None,
            delegation_depth: 0,
            max_delegation_depth: 8,
            uses_remaining,
            status: Status::Active,
            constraints: BTreeMap::new(),
        });
        Ok(id)
    }

    /// K-11: grant by TRANSFER. The actor may transfer only rights it already HOLDS over a
    /// covering resource, and only as an Allow (never mint a Deny/Ask, never a finite-use onward).
    pub fn grant_by_transfer(
        &mut self,
        actor: &str,
        subject: &str,
        resource: &str,
        rights: &[&str],
        effect: Effect,
    ) -> Result<String, CapError> {
        if effect != Effect::Allow {
            return Err(CapError::CannotMintRestrictive);
        }
        let res = TypedResource::parse(resource)?;
        let wanted = checked_rights(rights)?;
        // the actor must hold a covering active allow for every right being transferred
        for right in &wanted {
            let held = self.caps.iter().any(|c| {
                c.subject == actor
                    && c.effect == Effect::Allow
                    && self.chain_active(c)
                    && c.resource.covers(&res)
                    && c.rights.contains(right)
                    && !c.is_finite()
            });
            if !held {
                return Err(CapError::TransferExceedsHeld);
            }
        }
        let id = self.mint_id();
        self.caps.push(Capability {
            cap_id: id.clone(),
            subject: subject.to_string(),
            resource: res,
            rights: wanted,
            effect: Effect::Allow,
            issuer: actor.to_string(),
            parent_cap_id: None,
            delegation_depth: 0,
            max_delegation_depth: 8,
            uses_remaining: None,
            status: Status::Active,
            constraints: BTreeMap::new(),
        });
        Ok(id)
    }

    /// K-7: delegate, attenuating only. Child resource within parent, child rights subset of
    /// parent, depth bounded. A FINITE-use parent cannot be delegated.
    pub fn delegate(
        &mut self,
        parent_id: &str,
        child_subject: &str,
        resource: &str,
        rights: &[&str],
    ) -> Result<String, CapError> {
        let parent = self.get(parent_id).ok_or(CapError::NotADelegableAllow)?.clone();
        if parent.effect != Effect::Allow || !self.chain_active(&parent) {
            return Err(CapError::NotADelegableAllow);
        }
        if parent.is_finite() {
            return Err(CapError::FiniteCannotDelegate);
        }
        let res = TypedResource::parse(resource)?;
        let want = checked_rights(rights)?;
        if !res.within(&parent.resource) {
            return Err(CapError::NotAttenuating);
        }
        if !want.is_subset(&parent.rights) {
            return Err(CapError::NotAttenuating);
        }
        let depth = parent.delegation_depth + 1;
        if depth > parent.max_delegation_depth {
            return Err(CapError::DepthExceeded);
        }
        let id = self.mint_id();
        self.caps.push(Capability {
            cap_id: id.clone(),
            subject: child_subject.to_string(),
            resource: res,
            rights: want,
            effect: Effect::Allow,
            issuer: parent.subject.clone(),
            parent_cap_id: Some(parent_id.to_string()),
            delegation_depth: depth,
            max_delegation_depth: parent.max_delegation_depth,
            uses_remaining: None,
            status: Status::Active,
            constraints: parent.constraints.clone(),
        });
        Ok(id)
    }

    /// K-8: revoke. Takes effect on the next authorize (re-evaluation), and transitively kills
    /// descendants via the parent chain.
    pub fn revoke(&mut self, cap_id: &str) {
        if let Some(c) = self.get_mut(cap_id) {
            c.status = Status::Revoked;
        }
    }

    /// Authorize a concrete (subject, resource, right). Deny-dominance (K-5); reserve-before-
    /// effect on a finite Allow (K-6); re-evaluated against live state (K-8). A bare id/name
    /// confers nothing - only a matching capability does (K-10).
    pub fn authorize(&mut self, subject: &str, resource: &str, right: &str) -> Decision {
        let Ok(req) = TypedResource::parse(resource) else {
            return Decision::Denied {
                reason: DenyReason::NoCapability,
            };
        };
        let matches = |c: &Capability| {
            c.subject == subject
                && c.resource.covers(&req)
                && c.rights.contains(right)
                && self.chain_active(c)
        };
        // K-5: an active matching Deny dominates everything.
        if self
            .caps
            .iter()
            .any(|c| c.effect == Effect::Deny && matches(c))
        {
            return Decision::Denied {
                reason: DenyReason::ExplicitDeny,
            };
        }
        // pick a matching Allow (prefer a non-finite one so a one-shot isn't burned needlessly)
        let chosen = self
            .caps
            .iter()
            .filter(|c| c.effect == Effect::Allow && matches(c))
            .min_by_key(|c| if c.is_finite() { 1 } else { 0 })
            .map(|c| c.cap_id.clone());
        let Some(id) = chosen else {
            return Decision::Denied {
                reason: DenyReason::NoCapability,
            };
        };
        let cap = self.get_mut(&id).unwrap();
        match cap.uses_remaining {
            None => Decision::Allowed { reservation: None },
            Some(0) => Decision::Denied {
                reason: DenyReason::Exhausted,
            },
            Some(n) => {
                let prev = Some(n);
                cap.uses_remaining = Some(n - 1);
                let revoked_by_this = n - 1 == 0;
                if revoked_by_this {
                    cap.status = Status::Revoked; // auto-revoke at zero
                }
                Decision::Allowed {
                    reservation: Some(Reservation {
                        cap_id: id,
                        prev_uses: prev,
                        revoked_by_this,
                    }),
                }
            }
        }
    }

    /// Commit a reservation (keep the consumption). No-op; present for symmetry/clarity.
    pub fn commit(&mut self, _r: Reservation) {}

    /// K-6: refund a reservation on provider failure - restore the use and un-revoke if this
    /// reservation was what zeroed it.
    pub fn refund(&mut self, r: Reservation) {
        if let Some(c) = self.get_mut(&r.cap_id) {
            c.uses_remaining = r.prev_uses;
            if r.revoked_by_this {
                c.status = Status::Active;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn allowed(d: &Decision) -> bool {
        matches!(d, Decision::Allowed { .. })
    }

    #[test]
    fn k4_record_rejects_wildcard_right_and_requires_explicit_rights() {
        let mut s = CapabilityStore::new();
        assert_eq!(
            s.issue_root("a", "fs:src/main", &["*"], Effect::Allow, None),
            Err(CapError::WildcardRight)
        );
        assert_eq!(
            s.issue_root("a", "fs:src/main", &[], Effect::Allow, None),
            Err(CapError::NoRights)
        );
        assert!(s.issue_root("a", "fs:src/main", &["read"], Effect::Allow, None).is_ok());
    }

    #[test]
    fn k5_deny_dominates_overlapping_allow() {
        let mut s = CapabilityStore::new();
        s.issue_root("a", "fs:*", &["read"], Effect::Allow, None).unwrap();
        s.issue_root("a", "fs:secret", &["read"], Effect::Deny, None).unwrap();
        assert!(allowed(&s.authorize("a", "fs:public", "read")));
        assert!(matches!(
            s.authorize("a", "fs:secret", "read"),
            Decision::Denied { reason: DenyReason::ExplicitDeny }
        ));
    }

    #[test]
    fn k6_one_shot_is_consumed_exactly_once_and_refunds() {
        let mut s = CapabilityStore::new();
        let _ = s.issue_root("a", "fs:f", &["write"], Effect::Allow, Some(1)).unwrap();
        // reserve-before-effect: first authorize consumes and auto-revokes
        let d1 = s.authorize("a", "fs:f", "write");
        assert!(allowed(&d1));
        // a second concurrent attempt cannot cross again
        assert!(matches!(
            s.authorize("a", "fs:f", "write"),
            Decision::Denied { reason: DenyReason::Revoked | DenyReason::NoCapability | DenyReason::Exhausted }
        ));
        // refund on provider failure restores the one-shot
        if let Decision::Allowed { reservation: Some(r) } = d1 {
            s.refund(r);
        }
        assert!(allowed(&s.authorize("a", "fs:f", "write")));
    }

    #[test]
    fn k7_delegation_only_attenuates_and_finite_cannot_delegate() {
        let mut s = CapabilityStore::new();
        let parent = s.issue_root("a", "fs:src/*", &["read", "write"], Effect::Allow, None).unwrap();
        // widening rights or resource is refused
        assert_eq!(
            s.delegate(&parent, "b", "fs:src/*", &["read", "write", "delete"]),
            Err(CapError::NotAttenuating)
        );
        assert_eq!(
            s.delegate(&parent, "b", "fs:*", &["read"]),
            Err(CapError::NotAttenuating)
        );
        // a faithful attenuation is allowed
        s.delegate(&parent, "b", "fs:src/lib", &["read"]).unwrap();
        assert!(allowed(&s.authorize("b", "fs:src/lib", "read")));
        // parent revocation transitively kills the child
        s.revoke(&parent);
        assert!(matches!(
            s.authorize("b", "fs:src/lib", "read"),
            Decision::Denied { .. }
        ));
        // a finite-use capability cannot be delegated onward
        let finite = s.issue_root("a", "fs:g", &["read"], Effect::Allow, Some(2)).unwrap();
        assert_eq!(
            s.delegate(&finite, "b", "fs:g", &["read"]),
            Err(CapError::FiniteCannotDelegate)
        );
    }

    #[test]
    fn k8_revocation_takes_effect_on_the_next_authorize() {
        let mut s = CapabilityStore::new();
        let c = s.issue_root("a", "fs:f", &["read"], Effect::Allow, None).unwrap();
        assert!(allowed(&s.authorize("a", "fs:f", "read")));
        s.revoke(&c);
        assert!(matches!(
            s.authorize("a", "fs:f", "read"),
            Decision::Denied { reason: DenyReason::NoCapability }
        ));
    }

    #[test]
    fn k9_typed_resource_prefix_collision_and_terminal_wildcard() {
        assert_eq!(TypedResource::parse("*"), Err(CapError::BareWildcard));
        assert_eq!(TypedResource::parse("untyped"), Err(CapError::Untyped));
        assert_eq!(TypedResource::parse("fs:sr*c"), Err(CapError::NonTerminalWildcard));
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
            s.authorize("a", "fs:src/main", "read"),
            Decision::Denied { reason: DenyReason::NoCapability }
        ));
        // even after a capability exists for ANOTHER subject, the name doesn't carry to 'a'.
        s.issue_root("owner", "fs:src/main", &["read"], Effect::Allow, None).unwrap();
        assert!(matches!(
            s.authorize("a", "fs:src/main", "read"),
            Decision::Denied { reason: DenyReason::NoCapability }
        ));
    }

    #[test]
    fn k11_grant_is_transfer_not_minting() {
        let mut s = CapabilityStore::new();
        s.issue_root("a", "fs:src/*", &["read"], Effect::Allow, None).unwrap();
        // transfer a right the actor holds: ok
        assert!(s.grant_by_transfer("a", "b", "fs:src/lib", &["read"], Effect::Allow).is_ok());
        // transfer a right the actor does NOT hold: refused
        assert_eq!(
            s.grant_by_transfer("a", "b", "fs:src/lib", &["write"], Effect::Allow),
            Err(CapError::TransferExceedsHeld)
        );
        // minting a restrictive (deny/ask) record via grant: refused
        assert_eq!(
            s.grant_by_transfer("a", "b", "fs:src/lib", &["read"], Effect::Deny),
            Err(CapError::CannotMintRestrictive)
        );
    }
}
