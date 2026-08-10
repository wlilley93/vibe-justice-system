//! The referral-record door: a record in the orders store that is NOT an order.
//! Adopted upstream 2026-08-06 from the First Subscriber's compelled divergence
//! ([2026] VJS-SC 4 as applied at [2026] VJS-CC-OPBOX 160 O1; offered under
//! OBL-2026-08-05-UPSTREAM-DOOR and carried here by the reconciliation re-pull).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// A record in the orders store that is NOT AN ORDER: a matter this jurisdiction referred UP to the
/// canonical Supreme Court, retained as the audit trail of a mis-filing and its correction.
///
/// `2026-VJS-SC-OPBOX-001.yaml` is the only one. It was declared unreadable because it has no
/// `holding` and no directives, and ACT-002:s10 makes both constitutive of a VALID ORDER - so the
/// parse failure looked like the statute's own consequence, and completing it looked like the
/// engineer writing a Supreme Court ratio from nothing. I declined to do that and I still would.
///
/// The premise was wrong. It is not an order lacking a holding; it is not an order. Its own
/// `correction_note` says it was recorded in error, and the apex ruling it names says so too, in
/// terms: `[2026] VJS-SC 4`'s holding RE-CHARACTERISES the repo-level record "as a referral", the
/// Supreme Court being singular and canonical so that a subscribing jurisdiction may refer a matter
/// up but may not hold its own Supreme sitting. Both halves of that (the canonical order and its
/// opinion) were read at `wlilley93/vibe-justice-system` before this type was written, because a
/// pointer that has not been followed is not evidence.
///
/// So this is [2026] VJS-CC-OPBOX 160 O1's cure applied literally - widen the reader, never edit the
/// record - and the characterisation is not mine to make or to withhold: binding apex law already
/// made it. Nothing about the record changes and nothing new binds. A referral never enters the
/// citator, never becomes an `AuthorityPointer`, and confers no force of its own. What opbox is
/// bound by is VJS-SC 4 itself, as any subscribing jurisdiction is.
///
/// THE DOOR IS DELIBERATELY NARROW, because a lax one would let a genuinely broken order be
/// reclassified into silence, which is the same fail-open shape as the 55 unreadable orders. See
/// `is_referral_not_order`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReferralRecord {
    #[serde(default)]
    pub id: String,
    /// Why the record was corrected. Required: a referral that does not say it is one is just a file.
    pub correction_note: String,
    pub referral: ReferralTarget,
    /// MUST BE ABSENT. A file carrying a holding is an ORDER that happens to mention a referral, and
    /// it stays an order - it is not admitted through this door, it is reported unreadable as before.
    #[serde(default)]
    pub holding: Option<String>,
    /// Same test as `holding`, for the same reason.
    #[serde(default)]
    pub directives: Option<Vec<serde_yaml::Value>>,
}

impl ReferralRecord {
    /// All four limbs must hold. Any one of them failing means the file is read as an unreadable
    /// ORDER, which is the safe direction: it stays counted, and the alarm stays armed.
    pub fn is_referral_not_order(&self) -> bool {
        self.holding.is_none()
            && self.directives.is_none()
            && !self.correction_note.trim().is_empty()
            && !self.referral.apex_ruling.trim().is_empty()
    }
}

/// What a file in the orders store IS, once it has failed to parse as an `Order`.
///
/// ONE statement of the rule. Three readers stated it three ways and drifted apart, which is the
/// same shape [2026] VJS-CC-OPBOX 16 C1 records for the two record-writing doors: a caller that
/// restates where records go is a second statement of the rule, and that is how the doors came
/// apart in the first place. Here `vjs-engine` announced referrals correctly, `vjs-store` hard-failed
/// the whole command on one (so `vjs status` could not run at all against the First Subscriber's
/// store), and `vjs-core::repo` silently dropped every unparseable record on the floor - the last
/// being the worst, because that reader feeds `evaluate_invariants`, so invariants were passing over
/// an order set that was quietly short.
#[derive(Clone, Debug)]
pub enum UnparsedOrder {
    /// A referral record. Announce it and leave it out of the citator; it confers no local force.
    Referral(Box<ReferralRecord>),
    /// Genuinely unreadable. It stays COUNTED and the alarm stays ARMED - never silently skipped,
    /// because a record quietly reclassified out of the count is the fail-open this whole matter
    /// exists about.
    Unreadable,
}

/// Classify a file in the orders store that did not parse as an `Order`.
///
/// The door is deliberately narrow (see [`ReferralRecord::is_referral_not_order`]): anything that
/// does not satisfy every limb comes back [`UnparsedOrder::Unreadable`], which is the safe
/// direction. Callers MUST NOT re-implement this test.
pub fn classify_unparsed_order(content: &str) -> UnparsedOrder {
    match serde_yaml::from_str::<ReferralRecord>(content) {
        Ok(r) if r.is_referral_not_order() => UnparsedOrder::Referral(Box::new(r)),
        _ => UnparsedOrder::Unreadable,
    }
}

impl ReferralRecord {
    /// The single announcement wording, so every door says the same thing about the same record.
    pub fn announcement(&self, path: &std::path::Path) -> String {
        format!(
            "note: {} is a REFERRAL record, not an order: {} -> {} (apex ruling {}). \
             It confers no local force; the binding record is the apex ruling.",
            path.display(),
            if self.referral.from.trim().is_empty() {
                "(source not stated)"
            } else {
                self.referral.from.trim()
            },
            self.referral
                .apex_location
                .as_deref()
                .unwrap_or("(location not stated)"),
            self.referral.apex_ruling.trim(),
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReferralTarget {
    /// The local matter the referral came from, e.g. `[2026] VJS-CC-OPBOX 79`.
    #[serde(default)]
    pub from: String,
    /// The canonical apex citation the matter was re-homed to. Required, and the reason it is
    /// required is that it is the whole point: a referral record's only content is the pointer to
    /// where the ruling actually lives. Without it there is nothing to refer TO, and the file is
    /// an order missing its holding after all.
    pub apex_ruling: String,
    #[serde(default)]
    pub apex_location: Option<String>,
    #[serde(default)]
    pub outcome: Option<String>,
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, serde_yaml::Value>,
}
