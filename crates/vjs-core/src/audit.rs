//! K-19 - immutable, tamper-evident, hash-chained audit over governance transitions.
//!
//! Adopts the Acmeco kernel's append-only chained-event discipline into VJS. Every recorded
//! transition links to the previous by hash, so an edit, a reorder, or a drop anywhere in the
//! chain is detectable by recomputation. Deterministic and model-free (REG-KERNEL-001): pure
//! sha256 over the canonical entry bytes, with no clock and no network at the hashing point - a
//! caller supplies any time field as opaque content, so the same inputs always produce the same
//! chain.
//!
//! Two strengths, both bound to tests below:
//!   - a plain sha256 chain is tamper-EVIDENT: anyone who recomputes detects a broken link, an
//!     edited entry, or a reorder/drop;
//!   - a keyed HMAC-SHA256 chain is tamper-RESISTANT: an editor with full write access to the
//!     records but without the key cannot forge a self-consistent chain, because it cannot
//!     recompute any link.
//!
//! Honest limit (K-30, no silent gap): tail TRUNCATION - dropping the last k entries - leaves a
//! shorter but internally consistent chain. It is undetectable from the chain alone; detecting it
//! needs an externally pinned head (`head()`), exactly as the Acmeco kernel pins its chain head.
//! This is stated, not hidden.

use crate::types::DecisionLog;

/// The fixed genesis link every chain starts from (so an empty chain still has a well-defined
/// `prev_hash` for entry 0, and a forged "entry 0" with a different prev is caught).
pub const GENESIS: &str = "vjs-audit-genesis-v1";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuditEntry {
    pub seq: u64,
    pub actor: String,
    pub kind: String,
    /// sha256 of the opaque payload bytes - the content this transition records.
    pub payload_digest: String,
    pub prev_hash: String,
    pub hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuditFault {
    /// `seq` did not match its position - an entry was reordered or dropped (a gap).
    OutOfOrder { expected: u64, found: u64 },
    /// the entry's `prev_hash` does not match the running chain head.
    BrokenLink { seq: u64 },
    /// the entry's stored `hash` does not recompute from its own fields - it was edited.
    Tampered { seq: u64 },
}

#[derive(Clone, Debug, Default)]
pub struct AuditChain {
    entries: Vec<AuditEntry>,
    /// `None` = plain sha256 (tamper-evident); `Some(key)` = HMAC-SHA256 (tamper-resistant).
    key: Option<Vec<u8>>,
}

impl AuditChain {
    /// A plain, tamper-evident chain.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            key: None,
        }
    }

    /// A keyed, tamper-resistant chain. The key never enters an entry; it only gates recomputation.
    pub fn keyed(key: &[u8]) -> Self {
        Self {
            entries: Vec::new(),
            key: Some(key.to_vec()),
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn entries(&self) -> &[AuditEntry] {
        &self.entries
    }

    /// The current chain head (the externally pinnable value that also detects tail truncation).
    pub fn head(&self) -> String {
        self.entries
            .last()
            .map(|e| e.hash.clone())
            .unwrap_or_else(|| GENESIS.to_string())
    }

    /// Append a transition. The hash is computed deterministically from (seq, actor, kind,
    /// payload_digest, prev_hash); the payload itself is digested, never stored raw (K-20).
    pub fn append(&mut self, actor: &str, kind: &str, payload: &str) -> &AuditEntry {
        let seq = self.entries.len() as u64;
        let prev_hash = self.head();
        let payload_digest = sha256_hex(payload.as_bytes());
        let hash = link_hash(
            self.key.as_deref(),
            seq,
            actor,
            kind,
            &payload_digest,
            &prev_hash,
        );
        self.entries.push(AuditEntry {
            seq,
            actor: actor.to_string(),
            kind: kind.to_string(),
            payload_digest,
            prev_hash,
            hash,
        });
        self.entries.last().unwrap()
    }

    /// Re-derive the whole chain and fail at the FIRST inconsistency, naming the precise seq. A
    /// reorder/drop trips `OutOfOrder`; a relinked break trips `BrokenLink`; an in-place edit (or
    /// an HMAC chain re-forged without the key) trips `Tampered`.
    pub fn verify(&self) -> Result<(), AuditFault> {
        let mut prev = GENESIS.to_string();
        for (i, e) in self.entries.iter().enumerate() {
            let expected = i as u64;
            if e.seq != expected {
                return Err(AuditFault::OutOfOrder {
                    expected,
                    found: e.seq,
                });
            }
            if e.prev_hash != prev {
                return Err(AuditFault::BrokenLink { seq: e.seq });
            }
            let recomputed = link_hash(
                self.key.as_deref(),
                e.seq,
                &e.actor,
                &e.kind,
                &e.payload_digest,
                &e.prev_hash,
            );
            if recomputed != e.hash {
                return Err(AuditFault::Tampered { seq: e.seq });
            }
            prev = e.hash.clone();
        }
        Ok(())
    }

    /// Test/forensics hook: borrow the entries mutably to simulate a tamper.
    #[cfg(test)]
    fn entries_mut(&mut self) -> &mut Vec<AuditEntry> {
        &mut self.entries
    }
}

/// Fold a sequence of committed decision logs into a verifiable chain over their CANONICAL bytes,
/// so the chain is anchored to real kernel records rather than free-floating. Deterministic: the
/// log's own `time` field is folded in as opaque content; no clock is read here.
pub fn chain_decision_logs(logs: &[DecisionLog], key: Option<&[u8]>) -> AuditChain {
    let mut chain = match key {
        Some(k) => AuditChain::keyed(k),
        None => AuditChain::new(),
    };
    for log in logs {
        chain.append(&log.actor, "decision_log", &canonical_log(log));
    }
    chain
}

/// A stable, order-fixed serialization of a decision log's operative fields. Field order is fixed
/// so the digest is reproducible; any change to any operative field changes the digest.
fn canonical_log(log: &DecisionLog) -> String {
    format!(
        "id={}\ntime={}\nactor={}\nkind={}\nissue={}\ndecision={}\nbasis={}\nrisk={:?}\nreversibility={}\ncourt_required={}\nwhy={}",
        log.id,
        log.time,
        log.actor,
        log.kind,
        log.issue,
        log.decision,
        log.basis.join("|"),
        log.risk,
        log.reversibility,
        log.court_required,
        log.why,
    )
}

fn sha256_hex(msg: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    hex::encode(Sha256::digest(msg))
}

/// The per-link hash. Plain sha256 when unkeyed; HMAC-SHA256 when keyed.
fn link_hash(
    key: Option<&[u8]>,
    seq: u64,
    actor: &str,
    kind: &str,
    payload_digest: &str,
    prev_hash: &str,
) -> String {
    let canon = format!("{seq}\n{actor}\n{kind}\n{payload_digest}\n{prev_hash}");
    match key {
        Some(k) => hex::encode(hmac_sha256(k, canon.as_bytes())),
        None => sha256_hex(canon.as_bytes()),
    }
}

/// HMAC-SHA256 (RFC 2104), hand-rolled on sha2 to avoid pulling another crate into the kernel
/// closure (the K-12 net/model fence cares about the dep graph; a hand-rolled MAC keeps it small).
fn hmac_sha256(key: &[u8], msg: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    const BLOCK: usize = 64;
    let mut k = [0u8; BLOCK];
    if key.len() > BLOCK {
        k[..32].copy_from_slice(&Sha256::digest(key));
    } else {
        k[..key.len()].copy_from_slice(key);
    }
    let mut ipad = [0x36u8; BLOCK];
    let mut opad = [0x5cu8; BLOCK];
    for i in 0..BLOCK {
        ipad[i] ^= k[i];
        opad[i] ^= k[i];
    }
    let inner = {
        let mut h = Sha256::new();
        h.update(ipad);
        h.update(msg);
        h.finalize()
    };
    let mut h = Sha256::new();
    h.update(opad);
    h.update(inner);
    let mut out = [0u8; 32];
    out.copy_from_slice(&h.finalize());
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::RiskLevel;

    fn log(id: &str, decision: &str) -> DecisionLog {
        DecisionLog {
            id: id.into(),
            time: "2026-06-26T00:00:00Z".into(),
            actor: "lexby".into(),
            kind: "implementation_decision".into(),
            issue: "K-19".into(),
            decision: decision.into(),
            basis: vec!["REG-KERNEL-001".into()],
            risk: RiskLevel::Low,
            reversibility: "rollbackable".into(),
            court_required: false,
            why: "binding the audit invariant".into(),
        }
    }

    #[test]
    fn k19_clean_chain_verifies() {
        let mut c = AuditChain::new();
        c.append("lexby", "route", "open route r1");
        c.append("lexby", "permit", "open permit p1");
        c.append("lexby", "log", "decision d1");
        assert_eq!(c.verify(), Ok(()));
        // genesis is honoured: entry 0 links to GENESIS, not to nothing.
        assert_eq!(c.entries()[0].prev_hash, GENESIS);
    }

    #[test]
    fn k19_an_edited_entry_is_detected() {
        let mut c = AuditChain::new();
        c.append("lexby", "permit", "covers crates/**");
        c.append("lexby", "permit", "covers docs/**");
        // a forensic edit: change a recorded payload digest without re-hashing.
        c.entries_mut()[0].payload_digest = sha256_hex(b"covers /etc/**");
        assert_eq!(c.verify(), Err(AuditFault::Tampered { seq: 0 }));
    }

    #[test]
    fn k19_a_reordered_or_dropped_entry_is_detected() {
        let mut c = AuditChain::new();
        c.append("lexby", "a", "1");
        c.append("lexby", "b", "2");
        c.append("lexby", "c", "3");

        let mut reordered = c.clone();
        reordered.entries_mut().swap(1, 2);
        assert!(matches!(
            reordered.verify(),
            Err(AuditFault::OutOfOrder { .. })
        ));

        let mut dropped = c.clone();
        dropped.entries_mut().remove(1); // a mid-chain drop shows as a seq gap
        assert!(matches!(
            dropped.verify(),
            Err(AuditFault::OutOfOrder { expected: 1, found: 2 })
        ));
    }

    #[test]
    fn k19_hmac_chain_cannot_be_reforged_without_the_key() {
        let key = b"floor-signing-key";
        let mut c = AuditChain::keyed(key);
        c.append("lexby", "permit", "covers crates/**");
        c.append("lexby", "permit", "covers docs/**");
        assert_eq!(c.verify(), Ok(()));

        // An attacker with full write access edits entry 0 and, lacking the key, recomputes the
        // link with a PLAIN sha256 (the best they can do) and re-forges every forward link the
        // same way. The keyed verify still rejects it: a plain link is not the HMAC link.
        let forged_payload = sha256_hex(b"covers /etc/**");
        {
            let es = c.entries_mut();
            es[0].payload_digest = forged_payload;
            let mut prev = GENESIS.to_string();
            for e in es.iter_mut() {
                e.prev_hash = prev.clone();
                e.hash = link_hash(None, e.seq, &e.actor, &e.kind, &e.payload_digest, &e.prev_hash);
                prev = e.hash.clone();
            }
        }
        assert_eq!(c.verify(), Err(AuditFault::Tampered { seq: 0 }));
    }

    #[test]
    fn k19_decision_logs_fold_into_a_verifiable_chain() {
        let logs = vec![log("LOG-1", "ship K-19"), log("LOG-2", "ship K-23")];
        let chain = chain_decision_logs(&logs, Some(b"k"));
        assert_eq!(chain.len(), 2);
        assert_eq!(chain.verify(), Ok(()));

        // tampering with a committed log's operative field changes its digest, so a chain rebuilt
        // over the tampered logs no longer matches the head that was pinned over the originals.
        let mut tampered = logs.clone();
        tampered[0].decision = "ship something else".into();
        let rebuilt = chain_decision_logs(&tampered, Some(b"k"));
        assert_ne!(rebuilt.head(), chain.head());
    }
}
