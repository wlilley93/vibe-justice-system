//! Shared authority-reference extraction + grounding ([2026] VJS-PC 17 D6).
//!
//! ONE boundary-aware, line-wrap-tolerant extractor, used by BOTH the lawpack-wide
//! DANGLING_REFERENCE check and the PC-17 order citation-grounding gate - a single source
//! of truth for "what authorities does this text cite, and do they exist".
//!
//! PC-17 holds (Position B) that the kernel grounds only the EXISTENCE limb of per
//! incuriam: does a cited authority/section resolve to a defined object? It never reads
//! whether the cited section SAYS what the order claims (fidelity) or whether the error
//! changed the result (materiality) - those are merits, reserved to the bench and a court
//! on appeal (REG-KERNEL-001, clerk not court). Deterministic, model-free, no network.

use regex::Regex;
use std::collections::HashSet;
use std::sync::OnceLock;

/// A canonical authority reference found in operative text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuthorityRef {
    /// An instrument id, optionally section-granular: "ACT-X" or "ACT-X:s23".
    Instrument(String),
    /// A case citation: "[2026] VJS-PC 16" (canon) or "[2026] VJS-CC-OPBOX 79" (subscriber).
    Citation(String),
}

impl AuthorityRef {
    pub fn token(&self) -> &str {
        match self {
            AuthorityRef::Instrument(s) | AuthorityRef::Citation(s) => s,
        }
    }
}

fn id_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    // An instrument id, optionally followed by a section in any of :sN / s.N / s N form.
    RE.get_or_init(|| {
        Regex::new(
            r"\b((?:ACT|DEC|INV|OBL|SPEC|REG)-[A-Z0-9][A-Za-z0-9-]*[A-Za-z0-9])(?:[:\s]s\.?\s*(\d+))?",
        )
        .unwrap()
    })
}

fn citation_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\[(\d{4})\]\s+VJS-[A-Za-z]+(?:-[A-Za-z0-9]+)?\s+\d+").unwrap())
}

fn dewrap_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    // A hyphen followed by whitespace then an id-continuation char: a YAML folded-scalar
    // soft wrap that split an id ("REG-FEDERATION-\n COORDINATION-001"). Rejoin it so a
    // soft wrap can never manufacture a partial-id false positive (the REG-FEDERATION
    // line-wrap class). Only an UPPERCASE/digit continuation rejoins, so ordinary prose
    // ("a build- it pattern") is untouched.
    RE.get_or_init(|| Regex::new(r"-\s+([A-Z0-9])").unwrap())
}

/// Rejoin folded-scalar id splits before matching (PC-17 D6). The continuation char is
/// captured and re-inserted (the regex crate has no look-ahead).
pub fn dewrap(text: &str) -> String {
    dewrap_re().replace_all(text, "-$1").into_owned()
}

/// Normalize a (instrument, optional section-number) to the canonical "ID:sN" key.
pub fn section_key(instrument: &str, section: Option<&str>) -> String {
    match section {
        Some(n) => format!("{instrument}:s{n}"),
        None => instrument.to_string(),
    }
}

/// Extract every canonical authority reference (instrument ids, section-granular, and
/// case citations) from a text, line-wrap-tolerant and whole-token. A reference that is
/// NEGATED ("no ACT-X", "without DEC-Y") is omitted - it is a statement about absence,
/// not reliance (PC-17 D4 carve-out a).
pub fn extract_refs(text: &str) -> Vec<AuthorityRef> {
    let joined = dewrap(text);
    let mut out = Vec::new();
    for c in id_re().captures_iter(&joined) {
        let whole = c.get(0).unwrap();
        if is_negated(&joined, whole.start()) {
            continue;
        }
        let instrument = c.get(1).unwrap().as_str();
        let section = c.get(2).map(|m| m.as_str());
        out.push(AuthorityRef::Instrument(section_key(instrument, section)));
    }
    for m in citation_re().find_iter(&joined) {
        if is_negated(&joined, m.start()) {
            continue;
        }
        // Normalize internal whitespace so "[2026]  VJS-PC  16" keys the same.
        let norm = m.as_str().split_whitespace().collect::<Vec<_>>().join(" ");
        out.push(AuthorityRef::Citation(norm));
    }
    out
}

/// A reference is negated when the immediately-preceding word is a negator (PC-17 D4(a),
/// generalising the prior "ends with no" heuristic). Resolves ambiguity toward the
/// carve-out.
fn is_negated(text: &str, at: usize) -> bool {
    let before = text[..at].trim_end().to_ascii_lowercase();
    const NEGATORS: &[&str] = &["no", "not", "never", "without", "absent"];
    let last = before.rsplit(|c: char| c.is_whitespace()).next().unwrap_or("");
    NEGATORS.contains(&last) || before.ends_with("no such")
}

/// The grounding of a single reference against the defined corpus.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Grounding {
    /// Resolves to a defined, in-force object - clean.
    Resolved,
    /// Resolves to a defined object that is NOT in force (superseded / spent) - advisory
    /// only, never blocks (PC-17 D3). Existence is satisfied; aptness of relying on
    /// historical law is a merits read, out of scope.
    NotInForce,
    /// Resolves to NO defined object - the fail-closed trigger (PC-17 D1/D2).
    Unresolved,
}

/// Ground one reference: existence first (PC-17 D3 - existence, not in-force, is the
/// fail-closed test). `defined` carries every defined id incl. section ids; `citations`
/// every defined case citation; `in_force` the subset still in force.
pub fn ground(
    r: &AuthorityRef,
    defined: &HashSet<String>,
    citations: &HashSet<String>,
    in_force: &HashSet<String>,
) -> Grounding {
    let tok = r.token();
    let exists = match r {
        AuthorityRef::Instrument(_) => defined.contains(tok),
        AuthorityRef::Citation(_) => citations.contains(tok),
    };
    if !exists {
        return Grounding::Unresolved;
    }
    if in_force.contains(tok) {
        Grounding::Resolved
    } else {
        Grounding::NotInForce
    }
}

/// Ground an order's OPERATIVE parts (PC-17 D1): the holding + each directive's must +
/// each forbidden clause, already concatenated by the caller. The caller seeds the
/// order's own id and citation into `defined`/`citations` so a forward self-reference
/// resolves to itself (D4(c)). Returns the non-clean references as (token, grounding),
/// de-duplicated, so each defect is reported once. Existence-only (D1/D7) - never reads
/// what the cited authority says.
pub fn ground_operative(
    operative: &str,
    defined: &HashSet<String>,
    citations: &HashSet<String>,
    in_force: &HashSet<String>,
) -> Vec<(String, Grounding)> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for r in extract_refs(operative) {
        let g = ground(&r, defined, citations, in_force);
        if g == Grounding::Resolved {
            continue;
        }
        let tok = r.token().to_string();
        if seen.insert(tok.clone()) {
            out.push((tok, g));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defined() -> HashSet<String> {
        ["ACT-FOO", "ACT-FOO:s23", "DEC-BAR", "REG-FEDERATION-COORDINATION-001"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn dewrap_rejoins_a_folded_id_split() {
        // The REG-FEDERATION line-wrap class: a hyphen-then-space split rejoins.
        let refs = extract_refs("see REG-FEDERATION- COORDINATION-001 here");
        assert!(refs.contains(&AuthorityRef::Instrument(
            "REG-FEDERATION-COORDINATION-001".into()
        )));
        // And it does NOT produce a phantom bare "REG-FEDERATION".
        assert!(!refs.contains(&AuthorityRef::Instrument("REG-FEDERATION".into())));
    }

    #[test]
    fn section_granularity_and_existence() {
        let d = defined();
        let cites = HashSet::new();
        // ACT-X:s23 exists; ACT-X:s99 does not (section token unresolved).
        let r23 = extract_refs("under ACT-FOO s.23 the rule");
        assert_eq!(ground(&r23[0], &d, &cites, &d), Grounding::Resolved);
        let r99 = extract_refs("under ACT-FOO s.99 the rule");
        assert_eq!(ground(&r99[0], &d, &cites, &d), Grounding::Unresolved);
    }

    #[test]
    fn negation_is_carved_out() {
        // "no DEC-Z" is a statement about absence, not reliance.
        assert!(extract_refs("there is no DEC-ZED here").is_empty());
        assert!(extract_refs("without ACT-QUX").is_empty());
        // But a plain reference is extracted.
        assert!(!extract_refs("under DEC-BAR").is_empty());
    }

    #[test]
    fn not_in_force_is_advisory_not_unresolved() {
        let d = defined();
        let cites = HashSet::new();
        let in_force: HashSet<String> = ["ACT-X"].iter().map(|s| s.to_string()).collect();
        // DEC-Y is defined but not in force -> NotInForce (advisory), not Unresolved.
        let r = extract_refs("per DEC-BAR");
        assert_eq!(ground(&r[0], &d, &cites, &in_force), Grounding::NotInForce);
    }

    #[test]
    fn self_reference_resolves_when_seeded() {
        // D4(c): an order citing its own allocated citation resolves once the caller
        // seeds the order's own id/citation into the corpus.
        let mut cites: HashSet<String> = HashSet::new();
        let d = HashSet::new();
        // Before seeding, the order's own forward citation is unresolved...
        let r = extract_refs("this order [2026] VJS-PC 18 directs");
        assert_eq!(ground(&r[0], &d, &cites, &cites), Grounding::Unresolved);
        // ...after the caller seeds it, it resolves to itself.
        cites.insert("[2026] VJS-PC 18".into());
        assert_eq!(ground(&r[0], &d, &cites, &cites), Grounding::Resolved);
    }

    #[test]
    fn ground_operative_dedups_and_drops_resolved() {
        let d = defined();
        let cites = HashSet::new();
        // DEC-BAR (resolved) is dropped; ACT-NOPE (twice) reported once.
        let out = ground_operative(
            "rely on DEC-BAR and ACT-NOPE and again ACT-NOPE",
            &d,
            &cites,
            &d,
        );
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].0, "ACT-NOPE");
        assert_eq!(out[0].1, Grounding::Unresolved);
    }

    #[test]
    fn citations_resolve_against_the_register() {
        let d = HashSet::new();
        let cites: HashSet<String> = ["[2026] VJS-PC 16"].iter().map(|s| s.to_string()).collect();
        let real = extract_refs("following [2026] VJS-PC 16");
        assert_eq!(ground(&real[0], &d, &cites, &cites), Grounding::Resolved);
        let fake = extract_refs("following [2026] VJS-PC 99");
        assert_eq!(ground(&fake[0], &d, &cites, &cites), Grounding::Unresolved);
    }
}
