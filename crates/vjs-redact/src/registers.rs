//! The two REGISTERS the canon-write gate reads, and the content limbs keyed on them.
//!
//! [2026] VJS-CC-VJS 17, the ratio: "Two registers of terms that must not enter canon do
//! not merge merely because both can block a canon write: each answers to the harm it was
//! made for (federation authority under ACT-007:s4; confidentiality under ACT-005:s1), and
//! a register whose entries are hashed BECAUSE THE TERMS ARE PRIVATE may never be
//! accessioned into a register whose entries are published in cleartext."
//!
//! So the two live side by side here - separately loaded, separately coded, separately
//! messaged - and NEITHER is ever silently empty. An unreadable or empty register is an
//! ERROR naming the path (C3; [2026] VJS-CC-VJS 12 D1 on all fours), never an empty one.

use std::collections::HashSet;
use std::path::Path;

use sha2::Digest;
use vjs_core::*;

use crate::RedactScanner;

/// The C3 refusal, in ONE place, so every register says the same thing and NAMES the path
/// it could not read. Four sites read an absent register as an empty one on 2026-08-01; a
/// gate that does that reports "canon is clean" when it means "I could not look".
pub fn unreadable_register_error(path: &Path, why: &str) -> KernelError {
    KernelError::InvalidInput(format!(
        "the register at {} could not be read: {why}. A gate must treat an unreadable \
         register as an ERROR and never as an empty one - read as empty, it reports canon \
         clean when it means it could not look ([2026] VJS-CC-VJS 17 C3; [2026] VJS-CC-VJS \
         12 D1). Restore the file before writing canon.",
        path.display()
    ))
}

/// The accessioned subscriber repo_codes from the federation registry (#11).
///
/// C3: an ERROR when the registry is missing, unparseable, or carries no codes. It returned
/// an empty vector on all three until [2026] VJS-CC-VJS 17, which switched signals 3 and 4
/// off while the gate still reported itself as run.
pub fn load_subscriber_codes(repo_root: &Path) -> Result<Vec<String>, KernelError> {
    // The register of the seat this repository IS, not of the canon it reads.
    // LAWPACK-LITERAL: referent=local-records; status=reserved; authority=[2026] VJS-CC-VJS 15
    let path = repo_root.join("lawpack/v2/federation/subscriber-registry.yaml");
    let content = std::fs::read_to_string(&path)
        .map_err(|e| unreadable_register_error(&path, &e.to_string()))?;
    let value: serde_yaml::Value = serde_yaml::from_str(&content).map_err(|e| {
        unreadable_register_error(&path, &format!("it does not parse as YAML ({e})"))
    })?;
    let codes: Vec<String> = value
        .get("codes")
        .and_then(|c| c.as_sequence())
        .map(|seq| {
            seq.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default();
    if codes.is_empty() {
        return Err(unreadable_register_error(
            &path,
            "it carries no `codes:` entries, so every limb keyed on it would fire on \
             nothing while still reporting itself as run",
        ));
    }
    Ok(codes)
}

/// The hash on ONE denylist line, or `None` for a blank or comment line.
///
/// C7 puts a provenance comment on every entry (`# added=YYYY-MM-DD class=...`), so the
/// hash is the field BEFORE the first '#', never the whole line. EVERY reader of this file -
/// the two scripts and the CI test included - must split the same way, or the register
/// silently stops matching anything: the exact failure C7 would otherwise have created.
pub fn hash_of_line(line: &str) -> Option<String> {
    let head = line.split('#').next().unwrap_or_default().trim();
    (!head.is_empty()).then(|| head.to_string())
}

/// True for a well-formed sha256 hex digest: 64 lowercase hex characters.
fn is_sha256_hex(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// The publication denylist: sha256 of lowercased private terms, NEVER the terms.
///
/// ONE loader and ONE tokeniser, shared by the publication gate (`vjs gazette`) and the
/// canon-write gate's C1 limb, so "tokenises exactly as the publication gate does" is a
/// property of the code and not of a comment ([2026] VJS-CC-VJS 17 C1).
pub struct Denylist {
    hashes: HashSet<String>,
}

impl Denylist {
    /// The register at `<repo>/.vjs/publication-denylist.txt`.
    ///
    /// C3: unreadable is an ERROR naming the path, and so are an unparseable entry and a
    /// register with no entry at all.
    pub fn load(repo_root: &Path) -> Result<Self, KernelError> {
        let path = repo_root.join(".vjs/publication-denylist.txt");
        let text = std::fs::read_to_string(&path)
            .map_err(|e| unreadable_register_error(&path, &e.to_string()))?;
        let mut hashes = HashSet::new();
        for (n, line) in text.lines().enumerate() {
            let Some(h) = hash_of_line(line) else {
                continue;
            };
            if !is_sha256_hex(&h) {
                return Err(unreadable_register_error(
                    &path,
                    &format!(
                        "line {} is not a sha256 hash followed by a provenance comment \
                         ([2026] VJS-CC-VJS 17 C7). Entries are hashes, never plaintext",
                        n + 1
                    ),
                ));
            }
            hashes.insert(h);
        }
        if hashes.is_empty() {
            return Err(unreadable_register_error(
                &path,
                "it carries no entries, so every limb keyed on it would fire on nothing \
                 while still reporting itself as run",
            ));
        }
        Ok(Self { hashes })
    }

    pub fn len(&self) -> usize {
        self.hashes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hashes.is_empty()
    }

    /// One bit: does `text` carry a denylisted term anywhere? The publication gate's
    /// question - it names no line and no term, because it publishes its own answer.
    pub fn hits_anywhere(&self, text: &str) -> bool {
        text.lines().any(|l| self.hit_in_line(l))
    }

    /// The 1-INDEXED line numbers of `text` that carry a denylisted term, ascending. The
    /// LINE is disclosed; the TERM never is. That tells a reader who already holds the file
    /// where to look, and tells anyone who does not, nothing (C1).
    pub fn hit_lines(&self, text: &str) -> Vec<usize> {
        text.lines()
            .enumerate()
            .filter(|(_, l)| self.hit_in_line(l))
            .map(|(i, _)| i + 1)
            .collect()
    }

    /// THE tokeniser, lifted verbatim from the publication gate: token characters are
    /// alphanumeric or '-', tokens of 3 bytes or more are hashed, lowercased, as sha256 hex.
    /// A newline is neither alphanumeric nor '-', so it terminates a token exactly as the
    /// old whole-text loop did - which is why scanning line by line is the SAME scan.
    fn hit_in_line(&self, line: &str) -> bool {
        let mut token = String::new();
        for ch in line.chars() {
            if ch.is_alphanumeric() || ch == '-' {
                token.push(ch);
            } else {
                if self.is_denied(&token) {
                    return true;
                }
                token.clear();
            }
        }
        self.is_denied(&token)
    }

    fn is_denied(&self, token: &str) -> bool {
        token.len() >= 3
            && self.hashes.contains(&format!(
                "{:x}",
                sha2::Sha256::digest(token.to_lowercase().as_bytes())
            ))
    }
}

/// C1: the DENYLIST limb of the canon-write gate.
///
/// Separate from signal 4 and separately coded: a confidentiality hit on infrastructure
/// vocabulary is not a subscriber finding and must not wear a subscriber's message ([2026]
/// VJS-CC-VJS 14, applied). One finding per HIT LINE - C8 requires a cure to name file and
/// line, and one of the three admitted records carries two occurrences, so both must be
/// named.
///
/// The term is NEVER printed. The message says so, and says what this limb is NOT: it
/// reports hits, it never certifies canon clean (C6).
pub fn denylist_findings(path: &Path, content: &str, deny: &Denylist) -> Vec<BoundaryFinding> {
    deny.hit_lines(content)
        .into_iter()
        .map(|line| BoundaryFinding {
            severity: Severity::Error,
            path: Some(path.to_path_buf()),
            kind: BoundaryFindingKind::DenylistedTerm,
            message: format!(
                "{}:{} carries a term on the publication denylist \
                 (.vjs/publication-denylist.txt). The term is NOT named here: it is private, \
                 and naming it in a gate message would publish the very thing the register \
                 exists to keep out (ACT-005:s1). Open the file at that line and redact to the \
                 generic form or to the accessioned pseudonym. This limb reports HITS ONLY: it \
                 fires on the terms this register happens to hold and never certifies that \
                 canon is clean ([2026] VJS-CC-VJS 17 C1, C6).",
                path.display(),
                line
            ),
            suggested_route: BoundaryRoute::Redact,
        })
        .collect()
}

/// Signal 4, the PROSE limb: a registered subscriber code appearing anywhere in a canon
/// record's BODY - not just its structured id/citation/repo_code - is subscriber-identifying
/// content in canon. The [2026] VJS-PC 15 holding named the subscriber in its prose and
/// slipped the id-only checks (signals 1-3); this closes that hole. Canon must be GENERIC
/// (ACT-005:s1; ACT-007:s4). The registry file itself is exempt - it IS the list of codes
/// the gate reads.
///
/// C2: this limb reads TEXT, so it lives OUTSIDE `scan_canon_record`'s YAML parse. That
/// parse returns early on every markdown body in canon, and until [2026] VJS-CC-VJS 17 that
/// early return took signal 4 with it - which, together with the `.md` skip in
/// `scan_canon_writes`, is why the limb had never once fired on a judgment opinion.
pub fn prose_subscriber_findings(
    path: &Path,
    content: &str,
    canon_repo_code: &str,
    subscriber_codes: &[String],
) -> Vec<BoundaryFinding> {
    let is_registry = path
        .to_string_lossy()
        .replace('\\', "/")
        .ends_with("federation/subscriber-registry.yaml");
    if is_registry {
        return Vec::new();
    }
    let lower = content.to_ascii_lowercase();
    for code in subscriber_codes {
        if code.eq_ignore_ascii_case(canon_repo_code) {
            continue;
        }
        if contains_word(&lower, &code.to_ascii_lowercase()) {
            // one finding per record is enough to fail closed
            return vec![RedactScanner::block(
                path,
                BoundaryFindingKind::UnredactedEvidence,
                format!(
                    "Canon record names subscriber '{code}' in its body/prose. Canon must \
                     be generic (ACT-005:s1; ACT-007:s4): refer to 'the subscriber' / 'a \
                     subscriber', never the subscriber's code or name. Only the federation \
                     subscriber-registry lists codes."
                ),
            )];
        }
    }
    Vec::new()
}

/// True when `needle` (lowercase) appears in `hay` (lowercase) as a whole token - bounded
/// by non-alphanumerics on both sides - so a subscriber code like "acmeco" is caught in
/// prose but never as a substring of a longer word. Deterministic; PC-15 boundary cure.
fn contains_word(hay: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return false;
    }
    let bytes = hay.as_bytes();
    let mut from = 0;
    while let Some(pos) = hay[from..].find(needle) {
        let start = from + pos;
        let end = start + needle.len();
        let before_ok = start == 0 || !bytes[start - 1].is_ascii_alphanumeric();
        let after_ok = end >= bytes.len() || !bytes[end].is_ascii_alphanumeric();
        if before_ok && after_ok {
            return true;
        }
        from = start + 1;
    }
    false
}
