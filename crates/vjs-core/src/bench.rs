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
        // a RECOGNISED tier the constitution does not constitute is a defect, not "no
        // constraint" - mirror verify_bench's TierNotConstituted (fail-closed, audit 2026-06-26).
        return Err(format!(
            "court '{court_str}' ({tier:?}) is not constituted by the constitution ([2026] VJS-SC 2)"
        ));
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

/// All BOUNDARY positions of `key` in `text_lower`, in order (so "justice i" matches
/// "## justice i" / "justice i," but never the inside of "justice ii"). Empty if absent.
fn key_positions(text_lower: &str, key: &str) -> Vec<usize> {
    let mut out = Vec::new();
    if key.is_empty() {
        return out;
    }
    let mut start = 0;
    while let Some(pos) = text_lower[start..].find(key) {
        let abs = start + pos;
        let after = text_lower[abs + key.len()..].chars().next();
        if after.is_none_or(|c| !c.is_alphanumeric()) {
            out.push(abs);
        }
        start = abs + 1;
    }
    out
}

/// Seats with no present, non-empty opinion in `opinion_text`. A seat fails when its key
/// is absent (boundary-aware) OR the LARGEST block it owns - from one of its boundary
/// occurrences to the next occurrence of a DIFFERENT seat (or end of text) - holds less
/// than MIN_SEAT_CONTENT non-whitespace chars beyond the name. Taking the max over ALL of
/// a seat's occurrences (not just its first) fixes the coram false positive (audit 2026-06-26):
/// a leading "Before: A, B, C" line clusters every seat's FIRST occurrence adjacently, so the
/// earlier seats would falsely own only the tiny coram gap; but their real section occurrence
/// owns the whole section, so they pass, while a seat that only ever appears in the coram
/// cluster still fails. Boundary keys keep "Justice I / II / ... / N" from collapsing to one
/// offset (the bug that earlier false-flagged a full apex bench).
pub fn silent_seats(bench: &[String], opinion_text: &str) -> Vec<String> {
    let lower = opinion_text.to_ascii_lowercase();
    // Every boundary occurrence of any seat: (offset, bench_index), in document order.
    let mut occ: Vec<(usize, usize)> = Vec::new();
    let mut present = vec![false; bench.len()];
    for (i, entry) in bench.iter().enumerate() {
        for off in key_positions(&lower, &seat_key(entry)) {
            occ.push((off, i));
            present[i] = true;
        }
    }
    occ.sort_by_key(|(off, _)| *off);
    let mut silent: Vec<String> = Vec::new();
    for (i, entry) in bench.iter().enumerate() {
        if !present[i] {
            silent.push(entry.clone());
            continue;
        }
        let key = seat_key(entry);
        // The largest block this seat owns across all its occurrences. A coram-only seat
        // owns only clustered scraps; a real section owns the whole section.
        let mut best = 0usize;
        for (rank, (off, idx)) in occ.iter().enumerate() {
            if *idx != i {
                continue;
            }
            let end = occ
                .iter()
                .skip(rank + 1)
                .find(|(_, j)| *j != i)
                .map(|(o, _)| *o)
                .unwrap_or(lower.len());
            let content: usize = lower[*off..end]
                .replace(&key, " ")
                .chars()
                .filter(|c| !c.is_whitespace())
                .count();
            best = best.max(content);
        }
        // [2026] VJS-PC 21 D4. `best` measures BREVITY, not participation, and the Board
        // held unanimously that the two cannot be treated as the same thing. A judge who
        // writes "I agree with Atkin and have nothing to add" has not been silent:
        // concurrence without separate reasons is ordinary and often the most disciplined
        // form of agreement, and a rule under which a short concurrence voids the order
        // would require every judge to pad. Reporting such a seat as having "no present,
        // non-empty opinion" states more than was computed, which is the CC-VJS 11 vice.
        //
        // So an EXPRESS CONCURRENCE counts as speech regardless of length. A seat that
        // neither writes at length nor expressly concurs is the only one now reported.
        // What follows for such a seat is expressly LEFT OPEN by PC 21 and must not be
        // read out of this code.
        if best < MIN_SEAT_CONTENT && !expressly_concurs(&lower, &key) {
            silent.push(entry.clone());
        }
    }
    silent
}

/// The concurrence forms a seat may use instead of separate reasons (PC 21 D4).
///
/// Deliberately a closed, explicit list rather than a fuzzy match. A seat is taken to
/// have spoken only when the document says so in terms; anything looser would let a
/// passing mention of the word "concur" anywhere near a name count as participation,
/// which would be the same over-claim in the opposite direction.
const CONCURRENCE_FORMS: &[&str] = &[
    "concur",
    "i agree",
    "agrees",
    "agreeing",
    "nothing to add",
    "dissent",       // a recorded dissent is participation, not silence
    "dissenting",
];

/// True when, in the window this seat owns, the document expressly records the seat
/// concurring, agreeing, or dissenting. Searched from the seat's own mentions outward so
/// a concurrence attached to a DIFFERENT judge cannot be borrowed.
fn expressly_concurs(lower: &str, key: &str) -> bool {
    // The window is deliberately tight: a concurrence is written beside the name it
    // belongs to. 200 chars comfortably covers "X, concurring." and "I agree with Y and
    // have nothing to add. - X" without reaching the next judge's section.
    const WINDOW: usize = 200;
    key_positions(lower, key).into_iter().any(|off| {
        let start = off.saturating_sub(WINDOW);
        let end = (off + key.len() + WINDOW).min(lower.len());
        let Some(window) = lower.get(start..end) else {
            return false;
        };
        CONCURRENCE_FORMS.iter().any(|f| window.contains(f))
    })
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
                "Counted seat(s) {seats:?} neither write reasons nor expressly concur in source_opinion, so the record does not evidence their participation ([2026] VJS-SC 2; REG-COURT-RECORD-001; [2026] VJS-PC 21 D4). This states what was measured: a brief but EXPRESS concurrence counts as speech and is not reported here."
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
/// The id of the courts-constitution order, which is the instrument `verify_bench`
/// measures a declared bench against. Named once, here, beside the function that needs
/// it: every caller has to find the same order, and a caller that spells the id
/// differently silently gets `None` and skips the constitutive check rather than failing.
pub const COURTS_CONSTITUTION_ID: &str = "2026-VJS-COURTS-CONSTITUTION-001";

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

        // Adversarial (K-18, goal-completion audit 2026-06-26): prove the sizes are read FROM
        // the record, not a hard-coded table. Feed OFF-canonical sizes and require the function
        // to return THEM - a `match court { County => 1, .. }` hard-coding would still return
        // the canonical 1/3/5,9 here and fail, so this distinguishes by-reference from baked-in.
        let mut off = constitution();
        off.directives = vec![
            dir("county_court", "sit as an odd bench of 7 over repo matters"),
            dir("privy_council", "sit as an odd bench of 11"),
            dir(
                "supreme_court",
                "sit as an odd bench of 13, expandable to 21 for foundational questions",
            ),
        ];
        assert_eq!(constituted_sizes(&off, &Court::County), Some(vec![7]));
        assert_eq!(
            constituted_sizes(&off, &Court::PrivyCouncil),
            Some(vec![11])
        );
        assert_eq!(
            constituted_sizes(&off, &Court::SupremeCourt),
            Some(vec![13, 21])
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
    fn convening_for_an_unconstituted_tier_fails_closed() {
        // Audit 2026-06-26: convening_bench_check fail-OPENED (returned Ok) for a recognised
        // tier the constitution does not constitute, where verify_bench fails CLOSED. Now both
        // fail closed.
        let mut bare = constitution();
        bare.directives.clear();
        assert!(convening_bench_check(&bare, "privy_council", 3).is_err());
        assert!(convening_bench_check(&bare, "supreme_court", 5).is_err());
        // an UNRECOGNISED court label still imposes no constraint (it is no known tier).
        assert!(convening_bench_check(&bare, "moot_court", 1).is_ok());
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

    #[test]
    fn a_leading_coram_line_does_not_false_flag_seats_with_real_sections() {
        // Audit 2026-06-26: a "Before:" coram line listing all seats adjacently clustered
        // each seat's FIRST occurrence, so the earlier seats were falsely flagged silent
        // (it bit the SC-6 recording). With max-block-across-all-occurrences they pass,
        // because each has a real section below.
        let bench = vec!["Adair J".into(), "Calloway J".into(), "Devereux J".into()];
        let text = format!(
            "Before: Adair J, Calloway J, Devereux J\n\n\
             ## Opinion of Adair J\n{}\n## Opinion of Calloway J\n{}\n## Opinion of Devereux J\n{}",
            "a".repeat(300),
            "b".repeat(300),
            "c".repeat(300),
        );
        assert!(
            silent_seats(&bench, &text).is_empty(),
            "a coram line + three real sections must NOT flag any seat: {:?}",
            silent_seats(&bench, &text)
        );

        // A seat named ONLY in the coram (no section) is still correctly flagged silent.
        let text2 = format!(
            "Before: Adair J, Calloway J, Devereux J\n\n\
             ## Opinion of Adair J\n{}\n## Opinion of Calloway J\n{}\n",
            "a".repeat(300),
            "b".repeat(300),
        );
        assert_eq!(
            silent_seats(&bench, &text2),
            vec!["Devereux J".to_string()],
            "a seat present only in the coram cluster must still be silent"
        );
    }
}
