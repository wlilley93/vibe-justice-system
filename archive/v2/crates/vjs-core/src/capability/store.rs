//! The capability record and the store that authorizes against it (K-4..K-8, K-11).
//!
//! `CapabilityStore` is the live authorization engine: deny-dominance (K-5), reserve-before-effect on
//! a finite Allow (K-6), attenuating-only delegation (K-7), revocation-on-next-check (K-8), and
//! grant-by-transfer (K-11). It is deterministic, model-free, network-free (REG-KERNEL-001) and
//! generic over the opaque `Resource` vocabulary.

use std::collections::{BTreeMap, BTreeSet};

use super::{CapError, Resource};

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

#[derive(Debug, Clone)]
pub struct Capability<R: Resource> {
    pub cap_id: String,
    pub subject: String,
    pub resource: R,
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

impl<R: Resource> Capability<R> {
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

pub struct CapabilityStore<R: Resource> {
    caps: Vec<Capability<R>>,
    next: u64,
}

impl<R: Resource> Default for CapabilityStore<R> {
    fn default() -> Self {
        Self {
            caps: Vec::new(),
            next: 0,
        }
    }
}

impl<R: Resource> CapabilityStore<R> {
    pub fn new() -> Self {
        Self::default()
    }

    fn mint_id(&mut self) -> String {
        self.next += 1;
        format!("cap-{}", self.next)
    }

    fn get(&self, id: &str) -> Option<&Capability<R>> {
        self.caps.iter().find(|c| c.cap_id == id)
    }
    fn get_mut(&mut self, id: &str) -> Option<&mut Capability<R>> {
        self.caps.iter_mut().find(|c| c.cap_id == id)
    }

    /// A capability and its whole ancestor chain are Active (parent revoke transitively kills).
    fn chain_active(&self, cap: &Capability<R>) -> bool {
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
        resource: R,
        rights: &[&str],
        effect: Effect,
        uses_remaining: Option<u32>,
    ) -> Result<String, CapError> {
        let rights = checked_rights(rights)?;
        let id = self.mint_id();
        self.caps.push(Capability {
            cap_id: id.clone(),
            subject: subject.to_string(),
            resource,
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
        resource: R,
        rights: &[&str],
        effect: Effect,
    ) -> Result<String, CapError> {
        if effect != Effect::Allow {
            return Err(CapError::CannotMintRestrictive);
        }
        let wanted = checked_rights(rights)?;
        // the actor must hold a covering active allow for every right being transferred
        for right in &wanted {
            let held = self.caps.iter().any(|c| {
                c.subject == actor
                    && c.effect == Effect::Allow
                    && self.chain_active(c)
                    && c.resource.covers(&resource)
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
            resource,
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
        resource: R,
        rights: &[&str],
    ) -> Result<String, CapError> {
        let parent = self
            .get(parent_id)
            .ok_or(CapError::NotADelegableAllow)?
            .clone();
        if parent.effect != Effect::Allow || !self.chain_active(&parent) {
            return Err(CapError::NotADelegableAllow);
        }
        if parent.is_finite() {
            return Err(CapError::FiniteCannotDelegate);
        }
        let want = checked_rights(rights)?;
        if !resource.within(&parent.resource) {
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
            resource,
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
    pub fn authorize(&mut self, subject: &str, resource: R, right: &str) -> Decision {
        let req = resource;
        let matches = |c: &Capability<R>| {
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
