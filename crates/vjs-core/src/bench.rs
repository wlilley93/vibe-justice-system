//! Bench-integrity and tier-floor verification (PC-13 D10 + D7).
//!
//! The constituted odd bench size for each tier is read BY REFERENCE from the
//! constitution order [2026] VJS-SC 2 (id 2026-VJS-COURTS-CONSTITUTION-001) at
//! runtime - never restated or hard-coded here. A gate that hard-coded "privy = 3"
//! would amend the constitution and be ultra vires ACT-CONSOLIDATION-FRAMEWORK:s7
//! (the anti-Henry-VIII limit; PC-13 forbidden list). So if the constitution is
//! amended, this gate follows automatically.
//!
//! D10 (bench-integrity): refuse to record an order unless its recorded bench size
//! equals the constituted odd size for its tier AND every counted seat is matched
//! by a present, non-empty opinion in source_opinion (a named-but-silent seat fails
//! - the defect that struck this very matter's preparation).
//!
//! D7 (tier-floor): an order whose court is below the required tier is not validly
//! constituted. Both gates are bifurcated by assent under ACT-ASSENTED-RECORD-PROTECTION:
//! hard-block a NON-assented defective order; for a record declaring a valid
//! assent_source, only ever route-for-correction (never void or block).

use crate::types::{Court, Order};

/// The actor token a tier carries in the VJS-SC 2 constitution directives.
pub fn court_actor_token(court: &Court) -> &'static str {
    match court {
        Court::County => "county_court",
        Court::PrivyCouncil => "privy_council",
        Court::SupremeCourt => "supreme_court",
    }
}

/// The first integer appearing after `marker` in `text`, if any.
fn int_after(text: &str, marker: &str) -> Option<usize> {
    let idx = text.find(marker)? + marker.len();
    let tail = &text[idx..];
    let digits: String = tail
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect();
    digits.parse().ok()
}

/// The constituted odd bench size(s) for `court`, parsed from the constitution
/// order's directives. "...odd bench of N..." yields [N]; an added "...expandable
/// to M..." yields [N, M]. None when the constitution does not constitute the tier.
pub fn constituted_sizes(constitution: &Order, court: &Court) -> Option<Vec<usize>> {
    let token = court_actor_token(court);
    let dir = constitution.directives.iter().find(|d| d.actor == token)?;
    let mut sizes = Vec::new();
    if let Some(n) = int_after(&dir.must, "odd bench of ") {
        sizes.push(n);
    }
    if let Some(m) = int_after(&dir.must, "expandable to ") {
        sizes.push(m);
    }
    if sizes.is_empty() { None } else { Some(sizes) }
}

/// The minimum attributed non-whitespace content (chars) a seat must own in the
/// opinion document to count as a present, non-empty opinion. A bare name in a
/// bench list owns ~0; a real opinion owns hundreds+.
const MIN_SEAT_CONTENT: usize = 120;

/// The seat key used to find a seat in the opinion text: the first whitespace-
/// delimited token of the bench entry (e.g. "Tindale" from "Tindale" or
/// "Tindale J."), lowercased.
fn seat_key(entry: &str) -> String {
    entry
        .split_whitespace()
        .next()
        .unwrap_or(entry)
        .trim_matches(|c: char| !c.is_alphanumeric())
        .to_ascii_lowercase()
}

/// Seats that are absent or silent in `opinion_text`. A seat fails when its key is
/// absent entirely, or when the text it owns (from its first mention to the next
/// seat's first mention, in document order) holds less than MIN_SEAT_CONTENT chars
/// of non-whitespace content beyond the name itself.
pub fn silent_seats(bench: &[String], opinion_text: &str) -> Vec<String> {
    let lower = opinion_text.to_ascii_lowercase();
    // Locate each seat's first occurrence.
    let mut located: Vec<(usize, usize)> = Vec::new(); // (offset, bench_index)
    let mut absent: Vec<String> = Vec::new();
    for (i, entry) in bench.iter().enumerate() {
        let key = seat_key(entry);
        if key.is_empty() {
            absent.push(entry.clone());
            continue;
        }
        match lower.find(&key) {
            Some(off) => located.push((off, i)),
            None => absent.push(entry.clone()),
        }
    }
    // Attribute spans by document order of first occurrence.
    located.sort_by_key(|(off, _)| *off);
    let mut silent = absent;
    for (rank, (off, bench_idx)) in located.iter().enumerate() {
        let end = located
            .get(rank + 1)
            .map(|(next_off, _)| *next_off)
            .unwrap_or(lower.len());
        let span = &opinion_text[*off..end];
        let key = seat_key(&bench[*bench_idx]);
        // Content owned by the seat, minus its own name occurrences and whitespace.
        let content: String = span
            .to_ascii_lowercase()
            .replace(&key, " ")
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        if content.len() < MIN_SEAT_CONTENT {
            silent.push(bench[*bench_idx].clone());
        }
    }
    silent
}

/// A bench/tier defect found on an order.
#[derive(Clone, Debug, PartialEq)]
pub enum BenchDefect {
    /// The order's court is not constituted by VJS-SC 2 (no constituted bench).
    TierNotConstituted { court: String },
    /// The recorded bench size is not a constituted odd size for the tier.
    WrongBenchSize {
        court: String,
        got: usize,
        allowed: Vec<usize>,
    },
    /// One or more counted seats have no present, non-empty opinion.
    SilentSeats { seats: Vec<String> },
    /// A bench is declared but no source_opinion is present to verify it against.
    MissingOpinionSource,
}

impl BenchDefect {
    pub fn code(&self) -> &'static str {
        match self {
            BenchDefect::TierNotConstituted { .. } => "TIER_NOT_CONSTITUTED",
            BenchDefect::WrongBenchSize { .. } => "BENCH_SIZE_MISMATCH",
            BenchDefect::SilentSeats { .. } => "BENCH_SILENT_SEAT",
            BenchDefect::MissingOpinionSource => "BENCH_OPINION_MISSING",
        }
    }
    pub fn message(&self) -> String {
        match self {
            BenchDefect::TierNotConstituted { court } => format!(
                "Court '{court}' is not constituted by [2026] VJS-SC 2; no court may issue an order until constituted."
            ),
            BenchDefect::WrongBenchSize { court, got, allowed } => format!(
                "Recorded bench of {got} does not match the constituted odd size {allowed:?} for '{court}' ([2026] VJS-SC 2; REG-COURT-RECORD-001)."
            ),
            BenchDefect::SilentSeats { seats } => format!(
                "Counted seat(s) {seats:?} have no present, non-empty opinion in source_opinion; a named-but-silent seat fails ([2026] VJS-SC 2; REG-COURT-RECORD-001)."
            ),
            BenchDefect::MissingOpinionSource => {
                "Order declares a bench but no source_opinion to verify each seat against (REG-COURT-RECORD-001).".into()
            }
        }
    }
}

/// Verify an order's bench against the constitution. `opinion_text` is the loaded
/// contents of the order's source_opinion (None if it could not be read). Only
/// orders that DECLARE a bench are subject - an order with no bench has no seats to
/// count, so it is out of scope here. Pure: the caller decides severity by assent
/// (hard-block vs route-for-correction) per ACT-ASSENTED-RECORD-PROTECTION.
pub fn verify_bench(
    order: &Order,
    constitution: &Order,
    opinion_text: Option<&str>,
) -> Vec<BenchDefect> {
    let mut defects = Vec::new();
    if order.bench.is_empty() {
        return defects; // no seats declared, nothing to verify
    }
    match constituted_sizes(constitution, &order.court) {
        None => defects.push(BenchDefect::TierNotConstituted {
            court: format!("{:?}", order.court),
        }),
        Some(allowed) => {
            if !allowed.contains(&order.bench.len()) {
                defects.push(BenchDefect::WrongBenchSize {
                    court: format!("{:?}", order.court),
                    got: order.bench.len(),
                    allowed,
                });
            }
        }
    }
    match opinion_text {
        None => defects.push(BenchDefect::MissingOpinionSource),
        Some(text) => {
            let silent = silent_seats(&order.bench, text);
            if !silent.is_empty() {
                defects.push(BenchDefect::SilentSeats { seats: silent });
            }
        }
    }
    defects
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir(actor: &str, must: &str) -> crate::types::Directive {
        crate::types::Directive {
            id: "D".into(),
            actor: actor.into(),
            must: must.into(),
            when: Some("constituted".into()),
        }
    }

    fn constitution() -> Order {
        let mut o: Order = serde_yaml::from_str(
            "id: 2026-VJS-COURTS-CONSTITUTION-001\ncourt: supreme_court\njurisdiction: default\nstatus: binding\nissue: x\nholding: h\ndirectives: []\nsupersedes: []\nruntime_summary: s\ncreated_at: \"2026\"\n",
        )
        .unwrap();
        o.directives = vec![
            dir(
                "county_court",
                "sit as a single judge (odd bench of 1) over repo matters",
            ),
            dir(
                "privy_council",
                "sit as an odd bench of 3 over constitutional matters",
            ),
            dir(
                "supreme_court",
                "sit as an odd bench of 5, expandable to 9 for foundational questions",
            ),
        ];
        o
    }

    #[test]
    fn reads_sizes_by_reference() {
        let c = constitution();
        assert_eq!(constituted_sizes(&c, &Court::County), Some(vec![1]));
        assert_eq!(constituted_sizes(&c, &Court::PrivyCouncil), Some(vec![3]));
        assert_eq!(
            constituted_sizes(&c, &Court::SupremeCourt),
            Some(vec![5, 9])
        );
    }

    #[test]
    fn silent_seat_is_detected() {
        let bench = vec!["Tindale".into(), "Rowanne".into(), "Marchmont".into()];
        // Tindale and Rowanne write; Marchmont is named only (silent).
        let text = format!(
            "Tindale J. {}\n\nRowanne J. {}\n\nMarchmont",
            "a".repeat(300),
            "b".repeat(300)
        );
        let silent = silent_seats(&bench, &text);
        assert_eq!(silent, vec!["Marchmont".to_string()]);
    }

    #[test]
    fn full_bench_with_opinions_passes() {
        let bench = vec!["Tindale".into(), "Rowanne".into(), "Marchmont".into()];
        let text = format!(
            "Tindale {}\nRowanne {}\nMarchmont {}",
            "x".repeat(200),
            "y".repeat(200),
            "z".repeat(200)
        );
        assert!(silent_seats(&bench, &text).is_empty());
    }
}
