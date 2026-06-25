//! PC-16 D1/D2 ([2026] VJS-PC 16, The Assent-Resolution Floor): the deterministic,
//! model-free assent-RESOLUTION check, plus the constitutive-validity codes no assent
//! claim ever softens.
//!
//! ACT-COMPUTER-FIRST-REALM s.23 defines a valid assent_source as one "resolving to a
//! specific Sovereign-assent event" (sovereign_assent) or "tracing to specific Sovereign
//! assent" (standing_bounded_assent), and lists "an unresolved trace" as a ground of
//! rejection. VJS-ACT 10 s.1 shelters a record only "within the meaning of s.23". The
//! kernel formerly checked only allow-list MEMBERSHIP (front_door::declares_valid_assent),
//! the under-implementation [2026] VJS-PC 16 found, which let a forged record launder
//! Fatal findings by typing two known words. This restores the resolution half.
//!
//! Two teeth, per the bench's "void ab initio on BOTH grounds". First, RESOLUTION (D1):
//! a record's declared assent must trace to real Sovereign authority (a naming/lodging
//! provenance event, or - for the founding corpus - the commenced, established canon); a
//! fresh forged record traces to neither and does not resolve. Second, the CONSTITUTIVE
//! CODES: findings that go to whether the record IS a valid record/order of its kind
//! (bench-integrity, apex-singleness, citation collision) are NEVER softened by any
//! assent claim. No assent makes a bench-less order a real order.
//!
//! Deterministic, model-free, network-free (INV-KERNEL-001 / REG-KERNEL-001).

use std::path::Path;

/// Finding codes that go to a record's CONSTITUTIVE validity - whether it IS a valid
/// record/order of its kind at all - as opposed to a correctable defect. Per
/// [2026] VJS-PC 16 these are NEVER softened by an assent claim (ACT-COMPUTER-FIRST-REALM
/// s.14/s.15): the assent floor shelters a record's correctable defects; it does not
/// manufacture the record's standing.
pub const CONSTITUTIVE_CODES: &[&str] = &[
    "BENCH_REQUIRED",
    "TIER_NOT_CONSTITUTED",
    "BENCH_SIZE_MISMATCH",
    "BENCH_SILENT_SEAT",
    "BENCH_OPINION_MISSING",
    "CITATION_COLLISION",
    "APEX_RECORD_IN_SUBSCRIBING_JURISDICTION",
];

/// True when a finding code goes to constitutive validity (never assent-downgradeable).
pub fn is_constitutive(code: &str) -> bool {
    CONSTITUTIVE_CODES.contains(&code)
}

/// Read a top-level `key:` value (column zero), trimmed of quotes. None if absent.
fn top_level(content: &str, key: &str) -> Option<String> {
    let prefix = format!("{key}:");
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix(prefix.as_str()) {
            let v = rest.trim().trim_matches('"').trim_matches('\'').trim();
            return Some(v.to_string());
        }
    }
    None
}

/// Does `haystack` name `needle` as a whole token (so DEC-KERNEL-001 does not match
/// DEC-KERNEL-0010)? Deterministic boundary match.
fn names_token(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return false;
    }
    let bytes = haystack.as_bytes();
    let mut from = 0;
    while let Some(pos) = haystack[from..].find(needle) {
        let start = from + pos;
        let end = start + needle.len();
        let before_ok = start == 0 || !is_id_char(bytes[start - 1] as char);
        let after_ok = end >= bytes.len() || !is_id_char(bytes[end] as char);
        if before_ok && after_ok {
            return true;
        }
        from = start + 1;
    }
    false
}

fn is_id_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-' || c == '_'
}

/// The concatenated text + the pinned digests of the whole Sovereign-assent provenance
/// corpus: the per-instrument assent events (provenance/assent/) and the founding
/// commencement (provenance/founding/, which commenced the V2 lawpack under Bill 32's
/// Sovereign assent). A record "resolves by naming" iff its id/citation appears here, or
/// its declared instrument_digest matches a pinned one.
fn provenance_corpus(repo: &Path) -> (String, Vec<String>) {
    let mut text = String::new();
    let mut digests = Vec::new();
    for sub in ["lawpack/v2/provenance/assent", "lawpack/v2/provenance/founding"] {
        let dir = repo.join(sub);
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            match path.extension().and_then(|s| s.to_str()) {
                Some("yaml") | Some("yml") | Some("md") => {}
                _ => continue,
            }
            if let Ok(content) = std::fs::read_to_string(&path) {
                for line in content.lines() {
                    if let Some(d) = line.split("digest:").nth(1) {
                        let d = d.trim().trim_matches('"').trim_matches('\'').trim();
                        if d.starts_with("sha256:") {
                            digests.push(d.to_string());
                        }
                    }
                }
                text.push_str(&content);
                text.push('\n');
            }
        }
    }
    (text, digests)
}

/// True when `rel` already exists in committed canon at HEAD - i.e. the staged change is
/// an EDIT to an established (already-assented, commenced) record, not a fresh insertion.
/// A forged new record fails this (it has no committed history). Deterministic git query.
fn established_at_head(repo: &Path, rel: &str) -> bool {
    std::process::Command::new("git")
        .arg("-C")
        .arg(repo)
        .arg("cat-file")
        .arg("-e")
        .arg(format!("HEAD:{rel}"))
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// PC-16 D1: does a staged governed record's declared assent_source RESOLVE to real
/// Sovereign authority? (1) well-formed (allow-list, via front_door) AND (2) the trace
/// resolves:
///  - sovereign_assent: a Sovereign-assent provenance record NAMES this record (by id or
///    citation), OR a declared instrument_digest matches a pinned assent digest, OR the
///    record is already established in committed canon (commenced under the founding
///    Sovereign assent - so an edit to a founding/assented record keeps its floor, never
///    narrowed: D2).
///  - standing_bounded_assent: the standing-bounded route traces to specific Sovereign
///    assent - resolved as the existence of the Realm's Sovereign-assent foundation. The
///    constitutive-validity codes independently bar a bench-less/forged ORDER from
///    sheltering, so this permissive trace cannot launder an order's void-ab-initio
///    grounds.
///
/// An unresolved declaration is treated as one that LACKS a valid assent_source: the
/// record stays outside the floor and its findings keep their native severity (Fatal
/// stays Fatal). Deterministic, model-free.
pub fn assent_resolves(repo: &Path, rel: &str, content: &str) -> bool {
    if !vjs_core::front_door::declares_valid_assent(content) {
        return false;
    }
    let source = top_level(content, "assent_source").unwrap_or_default();
    let (corpus, pinned) = provenance_corpus(repo);
    let founded = corpus.contains("sovereign_assent_event") || corpus.contains("royal_assent");

    match source.as_str() {
        "sovereign_assent" => {
            // An explicit declared pointer, if present, must match a pinned assent digest.
            if let Some(digest) = top_level(content, "assent_instrument_digest") {
                return pinned.contains(&digest);
            }
            let id = top_level(content, "id").unwrap_or_default();
            let citation = top_level(content, "citation").unwrap_or_default();
            let named = (!id.is_empty() && names_token(&corpus, &id))
                || (!citation.is_empty() && names_token(&corpus, &citation));
            // Named by a Sovereign-assent provenance record, OR an edit to an established
            // (commenced) record - never narrow a genuinely-assented record (D2).
            named || established_at_head(repo, rel)
        }
        "standing_bounded_assent" => {
            // Traces to the Realm's Sovereign-assent foundation. The constitutive codes
            // bar a forged order independently of this trace.
            founded
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constitutive_codes_are_never_assent_downgradeable() {
        assert!(is_constitutive("BENCH_REQUIRED"));
        assert!(is_constitutive("CITATION_COLLISION"));
        assert!(is_constitutive("APEX_RECORD_IN_SUBSCRIBING_JURISDICTION"));
        // An ordinary, correctable defect IS downgradeable for a resolving record.
        assert!(!is_constitutive("S5_INERT_KERNEL_EFFECT"));
        assert!(!is_constitutive("DANGLING_REFERENCE"));
    }

    #[test]
    fn names_token_is_boundary_aware() {
        assert!(names_token("lodges DEC-KERNEL-001, INV-AGENT-001", "DEC-KERNEL-001"));
        assert!(!names_token("lodges DEC-KERNEL-0010", "DEC-KERNEL-001"));
        assert!(names_token("citation [2026] VJS-ACT 10 enacted", "[2026] VJS-ACT 10"));
    }
}
