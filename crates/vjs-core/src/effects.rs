//! K-23 / K-24 - effect reversibility classification + the durable, decided-once approval queue.
//!
//! Harvested from Agent kernelB's exemplar (the reference for irreversible-effect + human
//! authority) and made a VJS profile. Two invariants:
//!
//!  - K-23: every outward effect carries a reversibility class (`irreversible | rollbackable |
//!    none`). The PROVIDER classifies its own effect; an unclassifiable or unknown class is
//!    REFUSED, never silently allowed (fail closed).
//!  - K-24: an irreversible OUTWARD action blocks on a durable approval queue. The human grant is
//!    decided EXACTLY ONCE and consumed EXACTLY ONCE - a decided request cannot be re-decided, and
//!    a granted request authorises one resume, not a standing licence. Every decision is recorded.
//!
//! Deterministic and model-free (REG-KERNEL-001): classification is a pure lookup over a declared
//! class; the queue is pure state with no clock or network. This governs PROSPECTIVE authorisation
//! only and never voids or downgrades an entrenched assented record (K-15/K-16 dominate).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectClass {
    /// Cannot be undone once it lands (a sent email, a published artifact, a spent payment).
    Irreversible,
    /// Has an outward effect that can be rolled back (a staged write, a reversible txn).
    Rollbackable,
    /// No outward effect at all (a pure read/compute).
    NoEffect,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EffectError {
    /// The provider declared no class, or one the kernel does not recognise. Fail closed.
    Unclassifiable,
}

/// K-23: a provider classifies its own effect. A missing or unknown declaration is REFUSED
/// (fail closed) - the kernel never guesses a class, and an unclassifiable effect cannot proceed.
pub fn classify(declared: Option<&str>) -> Result<EffectClass, EffectError> {
    match declared.map(str::trim) {
        Some("irreversible") => Ok(EffectClass::Irreversible),
        Some("rollbackable") => Ok(EffectClass::Rollbackable),
        Some("none") => Ok(EffectClass::NoEffect),
        _ => Err(EffectError::Unclassifiable),
    }
}

/// What the effect gate decides for a classified action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Disposition {
    /// Proceed now (no outward effect, or a reversible one).
    Proceed,
    /// Block until a human grants approval for this enqueued request.
    RequireApproval { request_id: u64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestState {
    Pending,
    Granted,
    Denied,
    /// A granted request whose single authorisation has been spent on one resume.
    Consumed,
}

#[derive(Debug, Clone)]
pub struct ApprovalRequest {
    pub id: u64,
    pub subject: String,
    pub action: String,
    /// A digest of the concrete effect, so the human decides on a specific action and a granted
    /// request cannot be silently rebound to a different one (binds the decision to the deed).
    pub effect_digest: String,
    pub state: RequestState,
    /// Who decided (recorded for audit); None while Pending.
    pub decided_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApprovalError {
    UnknownRequest,
    /// K-24: a request may be decided exactly once.
    AlreadyDecided,
    /// Resuming requires a granted request that has not already been consumed.
    NotGranted,
    AlreadyConsumed,
}

/// A durable, decided-once approval queue (K-24). Pure state: a host persists it; the kernel only
/// transitions it deterministically. Append-only on decisions so the trail is auditable.
#[derive(Debug, Default)]
pub struct ApprovalQueue {
    requests: Vec<ApprovalRequest>,
    next: u64,
}

impl ApprovalQueue {
    pub fn new() -> Self {
        Self::default()
    }

    /// The effect gate (K-23 + K-24): a classified action either proceeds or blocks on approval.
    /// Only an irreversible AND outward action blocks; a reversible or no-effect action proceeds.
    /// An irreversible but NON-outward effect (a local, contained irreversible change) proceeds -
    /// the durable-queue requirement is for irreversible OUTWARD actions.
    pub fn gate(
        &mut self,
        class: EffectClass,
        outward: bool,
        subject: &str,
        action: &str,
        effect_digest: &str,
    ) -> Disposition {
        if class == EffectClass::Irreversible && outward {
            let id = self.enqueue(subject, action, effect_digest);
            Disposition::RequireApproval { request_id: id }
        } else {
            Disposition::Proceed
        }
    }

    fn enqueue(&mut self, subject: &str, action: &str, effect_digest: &str) -> u64 {
        self.next += 1;
        let id = self.next;
        self.requests.push(ApprovalRequest {
            id,
            subject: subject.to_string(),
            action: action.to_string(),
            effect_digest: effect_digest.to_string(),
            state: RequestState::Pending,
            decided_by: None,
        });
        id
    }

    fn get_mut(&mut self, id: u64) -> Option<&mut ApprovalRequest> {
        self.requests.iter_mut().find(|r| r.id == id)
    }

    pub fn get(&self, id: u64) -> Option<&ApprovalRequest> {
        self.requests.iter().find(|r| r.id == id)
    }

    /// K-24: a one-shot human decision, made EXACTLY ONCE. A second decision on the same request -
    /// grant or deny - is refused; the first decision stands and is recorded.
    pub fn decide(
        &mut self,
        id: u64,
        granted: bool,
        decider: &str,
    ) -> Result<RequestState, ApprovalError> {
        let req = self.get_mut(id).ok_or(ApprovalError::UnknownRequest)?;
        if req.state != RequestState::Pending {
            return Err(ApprovalError::AlreadyDecided);
        }
        req.state = if granted {
            RequestState::Granted
        } else {
            RequestState::Denied
        };
        req.decided_by = Some(decider.to_string());
        Ok(req.state)
    }

    /// K-24: spend the one-shot grant on a single resume. A granted request authorises EXACTLY ONE
    /// action; once consumed it cannot authorise a second (no standing licence). Denied/pending
    /// requests never authorise.
    pub fn consume(&mut self, id: u64) -> Result<(), ApprovalError> {
        let req = self.get_mut(id).ok_or(ApprovalError::UnknownRequest)?;
        match req.state {
            RequestState::Granted => {
                req.state = RequestState::Consumed;
                Ok(())
            }
            RequestState::Consumed => Err(ApprovalError::AlreadyConsumed),
            _ => Err(ApprovalError::NotGranted),
        }
    }

    pub fn pending(&self) -> impl Iterator<Item = &ApprovalRequest> {
        self.requests
            .iter()
            .filter(|r| r.state == RequestState::Pending)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn k23_classifies_the_three_classes() {
        assert_eq!(classify(Some("irreversible")), Ok(EffectClass::Irreversible));
        assert_eq!(classify(Some("rollbackable")), Ok(EffectClass::Rollbackable));
        assert_eq!(classify(Some("none")), Ok(EffectClass::NoEffect));
    }

    #[test]
    fn k23_an_unknown_or_absent_class_is_refused_fail_closed() {
        assert_eq!(classify(None), Err(EffectError::Unclassifiable));
        assert_eq!(classify(Some("")), Err(EffectError::Unclassifiable));
        assert_eq!(classify(Some("maybe")), Err(EffectError::Unclassifiable));
        // case sensitivity: the kernel does not guess a near-miss.
        assert_eq!(classify(Some("Irreversible")), Err(EffectError::Unclassifiable));
    }

    #[test]
    fn k24_an_irreversible_outward_action_blocks_until_granted() {
        let mut q = ApprovalQueue::new();
        let d = q.gate(EffectClass::Irreversible, true, "agent", "send_email", "sha256:abc");
        let Disposition::RequireApproval { request_id } = d else {
            panic!("irreversible outward must block");
        };
        // blocked: cannot consume before a grant.
        assert_eq!(q.consume(request_id), Err(ApprovalError::NotGranted));
        // a human grants -> now one resume is authorised.
        assert_eq!(q.decide(request_id, true, "will"), Ok(RequestState::Granted));
        assert_eq!(q.get(request_id).unwrap().decided_by.as_deref(), Some("will"));
        assert_eq!(q.consume(request_id), Ok(()));
    }

    #[test]
    fn k24_a_rollbackable_or_no_effect_action_does_not_block() {
        let mut q = ApprovalQueue::new();
        assert_eq!(
            q.gate(EffectClass::Rollbackable, true, "a", "stage_write", "d"),
            Disposition::Proceed
        );
        assert_eq!(
            q.gate(EffectClass::NoEffect, true, "a", "read", "d"),
            Disposition::Proceed
        );
        // an irreversible but NON-outward effect proceeds (the queue is for outward actions).
        assert_eq!(
            q.gate(EffectClass::Irreversible, false, "a", "local_delete", "d"),
            Disposition::Proceed
        );
        assert!(q.pending().next().is_none(), "nothing should have queued");
    }

    #[test]
    fn k24_a_grant_is_decided_once_and_consumed_once() {
        let mut q = ApprovalQueue::new();
        let Disposition::RequireApproval { request_id } =
            q.gate(EffectClass::Irreversible, true, "a", "publish", "d")
        else {
            panic!("must block");
        };
        assert_eq!(q.decide(request_id, true, "will"), Ok(RequestState::Granted));
        // decided exactly once: a second decision is refused, the first stands.
        assert_eq!(q.decide(request_id, false, "will"), Err(ApprovalError::AlreadyDecided));
        // consumed exactly once: a granted request authorises one resume, not a standing licence.
        assert_eq!(q.consume(request_id), Ok(()));
        assert_eq!(q.consume(request_id), Err(ApprovalError::AlreadyConsumed));
    }

    #[test]
    fn k24_a_denied_request_never_authorises() {
        let mut q = ApprovalQueue::new();
        let Disposition::RequireApproval { request_id } =
            q.gate(EffectClass::Irreversible, true, "a", "wire_funds", "d")
        else {
            panic!("must block");
        };
        assert_eq!(q.decide(request_id, false, "will"), Ok(RequestState::Denied));
        assert_eq!(q.consume(request_id), Err(ApprovalError::NotGranted));
        assert_eq!(q.decide(request_id, true, "will"), Err(ApprovalError::AlreadyDecided));
    }
}
