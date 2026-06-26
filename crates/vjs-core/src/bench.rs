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

/// Map a court string (as written in a convening or order's `court:`) to its tier.
/// One definition (#12), shared by the CLI convene path and the MCP convene verb so
/// they cannot drift.
pub fn court_from_str(s: &str) -> Option<Court> {
    let l = s.to_ascii_lowercase();
    if l.contains("county") {
        Some(Court::County)
    } else if l.contains("privy") {
        Some(Court::PrivyCouncil)
    } else if l.contains("supreme") {
        Some(Court::SupremeCourt)
    } else if l.contains("appeal") {
        // PC-19 + VJS-SC 2 D4: the Court of Appeal tier. "court_of_appeal" carries no
        // county/privy/supreme token, so without this arm it fell through to None and
        // convening_bench_check silently SKIPPED the bench-size check for a CoA convening.
        Some(Court::CourtOfAppeal)
    } else {
        None
    }
}

/// The D10 convening-time bench-size check, shared (#12): Ok when the tier is
/// unknown (no constraint) or the bench is a constituted odd size; Err with a
/// citable message when under- or over-strength. Reads sizes BY REFERENCE from the
/// constitution.
pub fn convening_bench_check(
    constitution: &Order,
    court_str: &str,
    bench_len: usize,
) -> Result<(), String> {
    let Some(tier) = court_from_str(court_str) else {
        return Ok(());
    };
    let Some(allowed) = constituted_sizes(constitution, &tier) else {
        return Ok(());
    };
    if allowed.contains(&bench_len) {
        Ok(())
    } else {
        Err(format!(
            "bench of {bench_len} is not the constituted odd size {allowed:?} for '{court_str}' ([2026] VJS-SC 2)"
        ))
    }
}

/// The subject-matter tier-floor ADVISORY (#9, PC-14 D7's reserved fuzzy limb).
/// ACT-002 routes constitutional / jurisdiction / routing / public-private-boundary
/// matters to the Privy Council (s3) and foundational doctrine to the Supreme Court
/// (s4). This flags an order whose `issue`/subject reads as a higher-tier matter than
/// its court. It is deliberately CONSERVATIVE and advisory only (the caller emits a
/// Warning, never a block): subject classification is fuzzy, so it surfaces a likely
/// mis-tiering for human judgement rather than voiding anything. Returns the advisory
/// message, or None.
pub fn subject_tier_advisory(issue: &str, court: &Court) -> Option<String> {
    let s = issue.to_ascii_lowercase();
    let privy_words = [
        "constitutional",
        "jurisdiction",
        "routing",
        "boundary",
        "federation",
        "sovereignty",
    ];
    let supreme_words = ["foundational", "doctrine", "settlement"];
    let hits = |words: &[&str]| words.iter().any(|w| s.contains(w));
    match court {
        Court::County if hits(&supreme_words) => Some(format!(
            "issue '{issue}' reads as foundational doctrine (ACT-002:s4 -> Supreme Court), but the \
             court is County. Likely under-tiered."
        )),
        Court::County if hits(&privy_words) => Some(format!(
            "issue '{issue}' reads as a constitutional / jurisdiction / routing / boundary matter \
             (ACT-002:s3 -> Privy Council), but the court is County. Likely under-tiered."
        )),
        Court::PrivyCouncil if hits(&supreme_words) => Some(format!(
            "issue '{issue}' reads as foundational doctrine (ACT-002:s4 -> Supreme Court), but the \
             court is the Privy Council. Possibly under-tiered."
        )),
        _ => None,
    }
}

/// The actor token a tier carries in the VJS-SC 2 constitution directives.
pub fn court_actor_token(court: &Court) -> &'static str {
    match court {
        Court::County => "county_court",
        Court::CourtOfAppeal => "court_of_appeal",
        Court::PrivyCouncil => "privy_council",
        Court::SupremeCourt => "supreme_court",
    }
}

/// The integer IMMEDIATELY following `marker` (modulo whitespace) in `text`, if any.
/// It does not skip over intervening words to a far-away digit, so a directive like
/// "...odd bench of, see clause 3..." yields None rather than 3 (audit robustness).
fn int_after(text: &str, marker: &str) -> Option<usize> {
    let idx = text.find(marker)? + marker.len();
    let tail = text[idx..].trim_start();
    let digits: String = tail.chars().take_while(|c| c.is_ascii_digit()).collect();
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

/// The seat key used to find a seat in the opinion text: the bench entry up to the
/// first parenthetical descriptor, whitespace-normalised and lowercased. So
/// "Justice I (separation-of-powers originalist)" keys to "justice i" and
/// "Tindale J." to "tindale j." - distinct per seat. (The old key took only the
/// first token, collapsing "Justice I" / "Justice II" / ... to "justice" and
/// false-flagging a full apex bench - bug fixed here.)
fn seat_key(entry: &str) -> String {
    let base = entry.split('(').next().unwrap_or(entry);
    base.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

/// The first BOUNDARY position of `key` in `text_lower` (so "justice i" matches
/// "## justice i" / "justice i," but never the inside of "justice ii"/"justice iii"),
/// or None if the key never appears at a boundary.
fn key_first_pos(text_lower: &str, key: &str) -> Option<usize> {
    if key.is_empty() {
        return None;
    }
    let mut start = 0;
    while let Some(pos) = text_lower[start..].find(key) {
        let abs = start + pos;
        let after = text_lower[abs + key.len()..].chars().next();
        if after.is_none_or(|c| !c.is_alphanumeric()) {
            return Some(abs);
        }
        start = abs + 1;
    }
    None
}

/// Seats with no present, non-empty opinion in `opinion_text`. A seat fails when its
/// key is absent (boundary-aware) OR the text it owns (from its boundary position to
/// the next seat's, in document order) holds less than MIN_SEAT_CONTENT non-whitespace
/// chars beyond the name. Boundary keys mean "Justice I / II / ... / N" no longer
/// collapse to one offset (the bug that false-flagged a full apex bench).
pub fn silent_seats(bench: &[String], opinion_text: &str) -> Vec<String> {
    let lower = opinion_text.to_ascii_lowercase();
    let mut located: Vec<(usize, usize)> = Vec::new(); // (offset, bench_index)
    let mut silent: Vec<String> = Vec::new();
    for (i, entry) in bench.iter().enumerate() {
        match key_first_pos(&lower, &seat_key(entry)) {
            Some(off) => located.push((off, i)),
            None => silent.push(entry.clone()),
        }
    }
    located.sort_by_key(|(off, _)| *off);
    for (rank, (off, idx)) in located.iter().enumerate() {
        let end = located
            .get(rank + 1)
            .map(|(next, _)| *next)
            .unwrap_or(lower.len());
        let key = seat_key(&bench[*idx]);
        let content: usize = lower[*off..end]
            .replace(&key, " ")
            .chars()
            .filter(|c| !c.is_whitespace())
            .count();
        if content < MIN_SEAT_CONTENT {
            silent.push(bench[*idx].clone());
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
            dir(
                "court_of_appeal",
                "may be convened on an odd bench of 3 if and when an appeal arises",
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
    fn court_of_appeal_convening_is_bench_checked() {
        // Regression (PC-19 D4 / VJS-SC 2): court_from_str must map "court_of_appeal" so the
        // convene-time bench-size check actually FIRES for a CoA. It was silently skipped
        // because the appeal arm was missing and the tier resolved to None (no constraint).
        assert_eq!(
            court_from_str("court_of_appeal"),
            Some(Court::CourtOfAppeal)
        );
        let c = constitution();
        assert_eq!(constituted_sizes(&c, &Court::CourtOfAppeal), Some(vec![3]));
        // an even / under- / over-strength CoA bench is REJECTED ...
        assert!(convening_bench_check(&c, "court_of_appeal", 2).is_err());
        assert!(convening_bench_check(&c, "court_of_appeal", 4).is_err());
        // ... and the constituted odd bench of 3 is accepted.
        assert!(convening_bench_check(&c, "court_of_appeal", 3).is_ok());
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
    fn roman_numeral_bench_is_not_falsely_flagged() {
        // The realm's own apex naming: "Justice I (descriptor)" ... "Justice V (...)",
        // each with a full opinion. The old first-token key collapsed them all to
        // "justice" and false-flagged four as silent. Must now pass clean.
        let bench: Vec<String> = (1..=5)
            .map(|n| {
                let r = ["I", "II", "III", "IV", "V"][n - 1];
                format!("Justice {r} (lens {n})")
            })
            .collect();
        let text = (1..=5)
            .map(|n| {
                let r = ["I", "II", "III", "IV", "V"][n - 1];
                format!("## Justice {r}\n{}\n", "opinion ".repeat(40))
            })
            .collect::<String>();
        assert!(
            silent_seats(&bench, &text).is_empty(),
            "a full Justice I..V bench must not be flagged: {:?}",
            silent_seats(&bench, &text)
        );
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
