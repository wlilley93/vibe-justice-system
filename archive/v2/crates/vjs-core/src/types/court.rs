//! The court seats, and the READER that turns a written seat into one.
//!
//! Split out of `types.rs` under the 600-line structural ceiling when the seat
//! stopped being readable by enumeration and became readable by rule.

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Court {
    /// The long form `County Court at <jurisdiction>` is accepted as well as `county`,
    /// on the same footing as `appeal` and `privy` below: three of a subscriber's filed
    /// orders write the long form, and until this was read all three were absent
    /// from the citator, so every route in that repository resolved without them
    /// ([2026] VJS-CC-OPBOX 160 O2, adopted upstream 2026-08-06).
    ///
    /// Read by RULE and not by enumeration - see the hand-written `Deserialize` below.
    County,
    /// The intermediate appellate tier. Persists in law (s.10; [2026] VJS-SC 2 D4) and is now
    /// representable at the canonical seat so vjs can convene and record a Court of Appeal order
    /// ([2026] VJS-PC 19). Serialises as `court_of_appeal`.
    ///
    /// `appeal` is accepted as well, on the same footing as `privy` below: the filed order
    /// `2026-VJS-CA-BOLTRIG-CODEX-APPROVAL-ROUTING-001` writes `court: appeal`, and until this
    /// alias existed that order did not parse - so it was silently absent from the citator and
    /// bound nothing. An order that does not parse is not a lenient reader's problem, it is an
    /// order that has no effect, and the cure is to read the record as written rather than edit
    /// a filed record to suit the reader.
    #[serde(alias = "appeal")]
    CourtOfAppeal,
    /// `privy` is accepted as well as `privy_council`: a filed order already uses it, and a filed
    /// record is read as written rather than edited to fit the reader (the never-rewrite-history
    /// rule). Serialises as `privy_council`.
    #[serde(alias = "privy")]
    PrivyCouncil,
    SupremeCourt,
}

/// Read by rule, not by enumeration.
///
/// The long form of a County seat is `County Court at <jurisdiction>`, and the
/// jurisdiction is whatever the subscriber called itself. That was previously read by
/// a serde alias naming ONE subscriber literally, which is wrong twice over. It named
/// a real subscriber in the canon's own source, which the pseudonymity Acts forbid in
/// a published record. And it meant every OTHER subscriber's long-form County orders
/// failed to deserialise - silently, because a filed order that does not parse is not
/// a parse error anyone sees, it is an order that is simply absent from the citator
/// and binds nothing. The enumeration could only ever be right for the one estate that
/// happened to be in the author's tree.
///
/// So: a case-insensitive `county court at <anything>` is a County seat, for any
/// subscriber, named or not, existing or not yet invoked. Everything else is read from
/// the same wire names the derive would have produced, aliases included. An
/// unrecognised court is still an ERROR and not a silent default: `Court` carries no
/// `Unrecognised` variant, and inventing one here would let a typo resolve as a seat.
///
/// [2026] VJS-CC-VJS 20; the never-rewrite-a-filed-record rule (widen the reader, do
/// not edit the record) is unchanged - this only widens it correctly.
impl<'de> Deserialize<'de> for Court {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CourtVisitor;

        impl serde::de::Visitor<'_> for CourtVisitor {
            type Value = Court;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(
                    "a court seat: `county` (or `County Court at <jurisdiction>`), \
                     `court_of_appeal` (or `appeal`), `privy_council` (or `privy`), \
                     or `supreme_court`",
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Court, E>
            where
                E: serde::de::Error,
            {
                let t = v.trim();
                let lower = t.to_ascii_lowercase();
                if lower.starts_with("county court at ") {
                    return Ok(Court::County);
                }
                match lower.as_str() {
                    "county" | "county_court" => Ok(Court::County),
                    "court_of_appeal" | "appeal" => Ok(Court::CourtOfAppeal),
                    "privy_council" | "privy" => Ok(Court::PrivyCouncil),
                    "supreme_court" => Ok(Court::SupremeCourt),
                    _ => Err(E::custom(format!("unknown court seat: `{t}`"))),
                }
            }
        }

        deserializer.deserialize_str(CourtVisitor)
    }
}
