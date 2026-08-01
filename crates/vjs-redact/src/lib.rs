use regex::Regex;
use std::path::Path;

use vjs_core::*;

#[cfg(test)]
mod tests;

pub struct RedactScanner;

/// This canon's own repo_code, and whether the CANON declared it or the hosting
/// jurisdiction's config supplied it.
///
/// PC-13 D1 read the code off the LOCAL config at both call sites. That is right only in
/// the canonical repo. In a subscriber jurisdiction whose `lawpack/v2` is a lawful
/// read-only mirror of VJS canon, every mirrored record carries `repo_code: VJS`, so a
/// locally-derived canon code turned each of them into "a subscriber's law filed into
/// canon" - thirteen blocking errors on a commit whose content was entirely enacted canon.
/// The repo_code a canon-write gate tests against is a property of the CANON being written
/// to, not of the repository hosting it.
///
/// `declared` is carried alongside the value because the two are not interchangeable
/// downstream: signal 4 (the prose limb) skips any subscriber code equal to the canon code,
/// so a DECLARED code naming a registered subscriber would silently switch that limb off
/// for exactly that subscriber. A subscriber falling back to its own config code is the
/// ordinary, lawful case and must not be treated the same way.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonRepoCode {
    pub code: String,
    /// True when the value came from the lawpack manifest, false when it came from the
    /// config chain (which every lawpack in the federation predates).
    pub declared: bool,
}

impl CanonRepoCode {
    /// The canon's own declaration, read from its lawpack manifest.
    pub fn declared(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            declared: true,
        }
    }

    /// Inferred from the hosting jurisdiction's config, the lawpack being silent.
    pub fn inferred(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            declared: false,
        }
    }

    pub fn as_str(&self) -> &str {
        &self.code
    }
}

/// The ONE resolver for `canon_repo_code`, called by the staged commit gate
/// (`vjs-engine::staged`) and the pre_write hook (`vjs-cli::front`) alike. Before this cure
/// the two call sites carried two different chains and only one of them had the `"VJS"`
/// tail, so the same tree could be gated against two different codes depending on which
/// door the write came through.
///
/// Source order: the lawpack's declared `repo_code` if present and non-empty; else
/// `config.repo_code`; else the jurisdiction id upper-cased; else `"VJS"`. Where the
/// lawpack declares, the config plays NO part - this is a source order, not a union. Where
/// it is silent the config chain applies unchanged and non-blocking.
///
/// Takes primitives rather than a config type so the scanner keeps no dependency on the
/// store. The manifest is read from `repo_root/lawpack/v2/manifest.toml` LITERALLY, never
/// through `resolve_lawpack_dir`: `scan_canon_writes`' `in_canon` filter matches that exact
/// tree, and a gate that read its declaration from one tree while filtering another would
/// be describing two different things.
pub fn resolve_canon_repo_code(
    repo_root: &Path,
    config_repo_code: Option<&str>,
    jurisdiction_id: Option<&str>,
) -> CanonRepoCode {
    if let Some(declared) = manifest_repo_code(repo_root) {
        return CanonRepoCode::declared(declared);
    }
    if let Some(rc) = config_repo_code.map(str::trim).filter(|s| !s.is_empty()) {
        return CanonRepoCode::inferred(rc);
    }
    if let Some(j) = jurisdiction_id.map(str::trim).filter(|s| !s.is_empty()) {
        return CanonRepoCode::inferred(j.to_uppercase());
    }
    CanonRepoCode::inferred("VJS")
}

/// The lawpack's own declared `repo_code`, from the canon tree the gate filters on.
fn manifest_repo_code(repo_root: &Path) -> Option<String> {
    let text = std::fs::read_to_string(repo_root.join("lawpack/v2/manifest.toml")).ok()?;
    manifest_repo_code_in(&text)
}

/// `repo_code` from a lawpack manifest's TEXT. Read with a line scan rather than a TOML
/// parse, the sibling idiom of `lawpack_id_of`: it keeps the scanner free of a parser
/// dependency and survives a half-written manifest. Only TOP-LEVEL keys count - the scan
/// stops at the first table header, so a `repo_code` under some future `[section]` can
/// never be mistaken for the lawpack's own declaration.
pub fn manifest_repo_code_in(text: &str) -> Option<String> {
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            break;
        }
        let Some(rest) = line.strip_prefix("repo_code") else {
            continue;
        };
        let Some(v) = rest.trim_start().strip_prefix('=') else {
            continue;
        };
        let v = v.trim().trim_matches('"').trim();
        if !v.is_empty() {
            return Some(v.to_string());
        }
    }
    None
}

impl RedactScanner {
    pub fn scan_file(path: &Path, content: &str) -> Vec<BoundaryFinding> {
        let mut findings = Vec::new();

        let patterns = [
            (
                r"\b(sk-[a-zA-Z0-9]{48})\b",
                BoundaryFindingKind::Token,
                "OpenAI API key detected",
            ),
            (
                r"\b(gh[pousr]_[A-Za-z0-9_]{36,})\b",
                BoundaryFindingKind::Token,
                "GitHub token detected",
            ),
            (
                r"\b(AKIA[0-9A-Z]{16})\b",
                BoundaryFindingKind::Token,
                "AWS access key detected",
            ),
            (
                r"\b([a-zA-Z0-9_-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})\b",
                BoundaryFindingKind::Email,
                "Email address detected",
            ),
            (
                r#"\b(password|passwd|pwd)\s*=\s*["][^"]+["]\b"#,
                BoundaryFindingKind::Secret,
                "Password assignment detected",
            ),
            (
                r#"\b(api[_-]?key|apikey)\s*=\s*["][^"]+["]\b"#,
                BoundaryFindingKind::Token,
                "API key assignment detected",
            ),
            (
                r"\b(192\.168\.\d{1,3}\.\d{1,3}|10\.\d{1,3}\.\d{1,3}\.\d{1,3}|172\.(1[6-9]|2\d|3[01])\.\d{1,3}\.\d{1,3})\b",
                BoundaryFindingKind::PrivateHostname,
                "Private IP address detected",
            ),
            (
                r"\b([a-zA-Z0-9_-]+\.(local|internal|private|lan))\b",
                BoundaryFindingKind::PrivateHostname,
                "Internal hostname detected",
            ),
        ];

        for (pattern, kind, message) in &patterns {
            if let Ok(re) = Regex::new(pattern) {
                for mat in re.find_iter(content) {
                    findings.push(BoundaryFinding {
                        severity: Severity::Error,
                        path: Some(path.to_path_buf()),
                        kind: kind.clone(),
                        message: format!("{} at position {}", message, mat.start()),
                        suggested_route: BoundaryRoute::Redact,
                    });
                }
            }
        }

        findings
    }

    pub fn scan_directory(dir: &Path) -> Result<Vec<BoundaryFinding>, KernelError> {
        let mut findings = Vec::new();

        for entry in walkdir::WalkDir::new(dir)
            .max_depth(5)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file()
                && let Ok(content) = std::fs::read_to_string(path)
            {
                let file_findings = Self::scan_file(path, &content);
                findings.extend(file_findings);
            }
        }

        Ok(findings)
    }

    pub fn check_public_safe(findings: &[BoundaryFinding]) -> bool {
        !findings
            .iter()
            .any(|f| matches!(f.severity, Severity::Fatal | Severity::Error))
    }

    // ---------------------------------------------------------------------
    // Canon-write gate (D1, [2026] VJS-PC 13 "Teeth For The Front Door").
    //
    // The deterministic boundary scanner (ACT-005:s7, no LLM) extended to fire on
    // canon (lawpack/v2) records and inspect their STRUCTURED fields, so
    // subscriber-scoped content can never enter canon by self-assertion - the vector
    // by which eleven DEC-ACMECO/INV-ACMECO/SPEC-ACMECO files self-asserted into the
    // public lawpack. It gives teeth to existing law, it does not legislate:
    //   - ACT-005:s1  public records must_not contain private_repo_paths / client_facts
    //   - ACT-005:s5  private facts from contributor repos must_not enter the public record
    //   - ACT-007:s4  local (subscriber) law must_not bind other repos
    // The block is content-based and jurisdiction-blind: canon must be clean no matter
    // who writes it, which is exactly the "apex records subscriber-scoped content"
    // inverse of the apex_routing_decision bright-line the order asked to generalise.
    // ---------------------------------------------------------------------

    /// The only top-level path segments a canon (lawpack/v2) record may legitimately
    /// scope: VJS's own governed surface. A concrete scope path rooted anywhere else
    /// is a private/subscriber repo path (ACT-005:s1 publish_private_repo_paths).
    pub const CANON_ROOTS: &'static [&'static str] =
        &["crates", "lawpack", ".vjs", "public", "src"];

    /// True when `raw` is a concrete filesystem path rooted OUTSIDE canon's own
    /// surface (so it points into a subscriber/private repo). Wildcards ("*", "**")
    /// and bare non-path tokens (action-kind / record-id list items) are not paths
    /// and never match.
    fn is_foreign_canon_path(raw: &str) -> bool {
        let p = raw.trim().trim_matches('"').trim_matches('\'').trim();
        if p.is_empty() || p == "*" || p == "**" {
            return false;
        }
        let p = p.strip_prefix("./").unwrap_or(p);
        // A path must have a separator to be a directory/glob reference. A bare token
        // (e.g. "court_order", "Cargo.toml") is either an action kind or a canon root
        // file; neither is a foreign repo path.
        let first = match p.split('/').next() {
            Some(seg) if p.contains('/') => seg,
            _ => return false,
        };
        if first.is_empty() {
            return false; // absolute "/..." - not a repo-relative subscriber path
        }
        !Self::CANON_ROOTS.contains(&first)
    }

    /// The candidate subscriber repo_code carried in a canon record id of the shape
    /// `<TYPE>-<CODE>-...` (e.g. DEC-ACMECO-UNITARY-001 -> "ACMECO"). Returns the
    /// second hyphen segment when it is an all-caps code (>=2 alphanumerics, starts
    /// with a letter) and not a pure number. Corroboration (below) decides whether it
    /// is actually foreign, so this never false-positives on its own.
    fn id_code_candidate(id: &str) -> Option<String> {
        let mut segs = id.split('-');
        let _ty = segs.next()?;
        let code = segs.next()?;
        let is_code = code.len() >= 2
            && code.chars().next().is_some_and(|c| c.is_ascii_uppercase())
            && code
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
            && code.chars().any(|c| c.is_ascii_uppercase());
        if is_code {
            Some(code.to_string())
        } else {
            None
        }
    }

    fn block(path: &Path, kind: BoundaryFindingKind, message: String) -> BoundaryFinding {
        BoundaryFinding {
            severity: Severity::Error,
            path: Some(path.to_path_buf()),
            kind,
            message,
            suggested_route: BoundaryRoute::Block,
        }
    }

    /// Scan ONE canon record's structured fields. `canon_repo_code` is this canon's
    /// own repo_code (e.g. "VJS"). Returns the boundary findings plus the foreign
    /// repo_code it established (for write-set companion-file corroboration).
    pub fn scan_canon_record(
        path: &Path,
        content: &str,
        canon_repo_code: &str,
        subscriber_codes: &[String],
    ) -> (Vec<BoundaryFinding>, Option<String>) {
        let mut findings = Vec::new();
        let mut foreign_code: Option<String> = None;

        let value: serde_yaml::Value = match serde_yaml::from_str(content) {
            Ok(v) => v,
            Err(_) => return (findings, foreign_code), // not a structured record
        };
        let map = match value.as_mapping() {
            Some(m) => m,
            None => return (findings, foreign_code),
        };
        let get_str = |key: &str| -> Option<String> {
            map.get(serde_yaml::Value::String(key.into()))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        };

        // Signal 1: foreign scope paths -> private repo path (ACT-005:s1; ACT-007:s4).
        let mut foreign_paths: Vec<String> = Vec::new();
        if let Some(scope) = map.get(serde_yaml::Value::String("scope".into()))
            && let Some(paths) = scope
                .as_mapping()
                .and_then(|s| s.get(serde_yaml::Value::String("paths".into())))
                .and_then(|p| p.as_sequence())
        {
            for item in paths {
                if let Some(s) = item.as_str()
                    && Self::is_foreign_canon_path(s)
                {
                    foreign_paths.push(s.to_string());
                    findings.push(Self::block(
                        path,
                        BoundaryFindingKind::PrivateRepoPath,
                        format!(
                            "Canon record scopes a private repo path '{s}' (ACT-005:s1 \
                             publish_private_repo_paths; ACT-007:s4 local_order_bind_other_repos). \
                             Subscriber-scoped law belongs in the subscriber's own .justice/, not VJS canon."
                        ),
                    ));
                }
            }
        }

        // Signal 2: an explicit repo_code field other than canon's own.
        if let Some(rc) = get_str("repo_code")
            && !rc.eq_ignore_ascii_case(canon_repo_code)
        {
            foreign_code = Some(rc.clone());
            findings.push(Self::block(
                path,
                BoundaryFindingKind::UnredactedEvidence,
                format!(
                    "Canon record carries a subscriber repo_code '{rc}' (canon is '{canon_repo_code}'). \
                     ACT-007:s4: local law must not bind other repos; it cannot live in canon. \
                     If this lawpack/v2 is a read-only mirror of another jurisdiction's canon, \
                     the cure is to declare that canon's OWN repo_code as `repo_code` in \
                     lawpack/v2/manifest.toml: the gate reads the lawpack's declaration in \
                     preference to the hosting repo's config."
                ),
            ));
        }

        // Signal 3: an id carrying a subscriber repo_code, established EITHER by
        // corroboration (a foreign scope path or repo_code, so canon ids like
        // REG-KERNEL-001 never trip) OR by the federation subscriber registry (#11),
        // which lets a foreign id-code be caught even without scope corroboration.
        if let Some(id) = get_str("id")
            && let Some(code) = Self::id_code_candidate(&id)
        {
            let low = code.to_ascii_lowercase();
            let corroborated_by_path = foreign_paths.iter().any(|p| {
                p.split(['/', '-', '_', '.'])
                    .any(|seg| seg.eq_ignore_ascii_case(&low))
            });
            let corroborated_by_rc = foreign_code
                .as_ref()
                .is_some_and(|rc| rc.eq_ignore_ascii_case(&code));
            let in_registry = subscriber_codes
                .iter()
                .any(|c| c.eq_ignore_ascii_case(&code));
            if (corroborated_by_path || corroborated_by_rc || in_registry)
                && !code.eq_ignore_ascii_case(canon_repo_code)
            {
                foreign_code = Some(code.clone());
                findings.push(Self::block(
                    path,
                    BoundaryFindingKind::UnredactedEvidence,
                    format!(
                        "Canon record id '{id}' embeds subscriber repo_code '{code}' \
                         (corroborated by its own subscriber scope). ACT-007:s4: \
                         a subscriber's law cannot be filed into VJS canon."
                    ),
                ));
            }
        }

        // Signal 4 (the PROSE limb): a registered subscriber code appearing anywhere in
        // the record's BODY - not just its structured id/citation/repo_code - is
        // subscriber-identifying content in canon. The [2026] VJS-PC 15 holding named the
        // subscriber in its prose and slipped the id-only checks (signals 1-3); this
        // closes that hole. Canon must be GENERIC (ACT-005:s1; ACT-007:s4): refer to "the
        // subscriber" / "a subscriber", never the code/name. The registry file itself is
        // exempt - it IS the list of codes the gate reads.
        let is_registry = path
            .to_string_lossy()
            .replace('\\', "/")
            .ends_with("federation/subscriber-registry.yaml");
        if !is_registry {
            let lower = content.to_ascii_lowercase();
            for code in subscriber_codes {
                if code.eq_ignore_ascii_case(canon_repo_code) {
                    continue;
                }
                if contains_word(&lower, &code.to_ascii_lowercase()) {
                    findings.push(Self::block(
                        path,
                        BoundaryFindingKind::UnredactedEvidence,
                        format!(
                            "Canon record names subscriber '{code}' in its body/prose. Canon must \
                             be generic (ACT-005:s1; ACT-007:s4): refer to 'the subscriber' / 'a \
                             subscriber', never the subscriber's code or name. Only the federation \
                             subscriber-registry lists codes."
                        ),
                    ));
                    break; // one finding per record is enough to fail closed
                }
            }
        }

        (findings, foreign_code)
    }

    /// The accessioned subscriber repo_codes from the federation registry (#11).
    /// Empty when the registry is absent or unparseable, so the gate degrades to the
    /// corroboration signals.
    fn load_subscriber_codes(repo_root: &Path) -> Vec<String> {
        let path = repo_root.join("lawpack/v2/federation/subscriber-registry.yaml");
        let Ok(content) = std::fs::read_to_string(&path) else {
            return Vec::new();
        };
        let Ok(value) = serde_yaml::from_str::<serde_yaml::Value>(&content) else {
            return Vec::new();
        };
        value
            .get("codes")
            .and_then(|c| c.as_sequence())
            .map(|seq| {
                seq.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Fire the canon-write gate over a set of repo-relative writes. Filters to
    /// lawpack/v2 records, scans each structured record, then a second pass blocks
    /// any companion file (e.g. a `.opinions.md`) in the same write-set whose name
    /// carries a foreign repo_code established by a sibling record. Deterministic,
    /// no LLM (ACT-005:s7).
    pub fn scan_canon_writes(
        repo_root: &Path,
        rel_paths: &[std::path::PathBuf],
        canon_repo_code: &CanonRepoCode,
    ) -> Vec<BoundaryFinding> {
        let in_canon = |p: &Path| {
            let mut comps = p.components();
            matches!(
                comps.next().and_then(|c| c.as_os_str().to_str()),
                Some("lawpack")
            ) && matches!(
                comps.next().and_then(|c| c.as_os_str().to_str()),
                Some("v2")
            )
        };
        let canon: Vec<&std::path::PathBuf> = rel_paths.iter().filter(|p| in_canon(p)).collect();

        // #11: the accessioned subscriber repo_codes, so a foreign id-code is caught
        // even without scope-path corroboration. Read from the federation registry;
        // absent registry => empty (the corroboration signals still apply).
        let subscriber_codes = Self::load_subscriber_codes(repo_root);

        let mut findings = Vec::new();

        // No code capture. A DECLARED canon repo_code equal to an accessioned subscriber's
        // is itself a boundary violation: signal 4 skips any subscriber code equal to the
        // canon code, so such a declaration would silently blind the prose limb for exactly
        // that subscriber - the gate would still report itself as running. Only a DECLARED
        // value can do this; a subscriber falling back to its OWN config code is the
        // ordinary case and must not trip here.
        if canon_repo_code.declared
            && !canon.is_empty()
            && let Some(hit) = subscriber_codes
                .iter()
                .find(|c| c.eq_ignore_ascii_case(canon_repo_code.as_str()))
        {
            findings.push(Self::block(
                &std::path::PathBuf::from("lawpack/v2/manifest.toml"),
                BoundaryFindingKind::UnredactedEvidence,
                format!(
                    "The lawpack manifest declares canon repo_code '{hit}', which is an \
                     accessioned subscriber code in lawpack/v2/federation/subscriber-registry.yaml. \
                     A canon code equal to a registered subscriber's switches the prose limb off \
                     for that subscriber, so its facts could enter canon unseen (ACT-005:s1; \
                     ACT-007:s4). Declare the canon's own repo_code, or de-accession the code \
                     from the subscriber registry."
                ),
            ));
        }

        let mut foreign_codes: Vec<String> = Vec::new();
        for rel in &canon {
            let abs = repo_root.join(rel);
            let name = rel.to_string_lossy();
            let is_yaml = name.ends_with(".yaml") || name.ends_with(".yml");
            if !is_yaml {
                continue;
            }
            if let Ok(content) = std::fs::read_to_string(&abs) {
                let (mut fs, code) = Self::scan_canon_record(
                    rel,
                    &content,
                    canon_repo_code.as_str(),
                    &subscriber_codes,
                );
                findings.append(&mut fs);
                // Secret/PII scan over the canon record (audit 2026-06-26: scan_file never ran
                // over the canon tree, so a credential committed into a lawpack/v2 record reached
                // the public repo's git history undetected by both the local gate and CI). Real
                // credentials (Token/Secret) HARD-BLOCK; the noisier Email / private-hostname
                // patterns surface as non-blocking Warnings (canon prose legitimately carries
                // boundary examples, e.g. "private_store.local" in statute 05).
                for mut sf in Self::scan_file(rel, &content) {
                    if !matches!(
                        sf.kind,
                        BoundaryFindingKind::Token | BoundaryFindingKind::Secret
                    ) {
                        sf.severity = Severity::Warning;
                    }
                    findings.push(sf);
                }
                if let Some(c) = code {
                    foreign_codes.push(c.to_ascii_lowercase());
                }
            }
        }

        // Companion pass: a non-YAML canon file (e.g. DEC-ACMECO-*.opinions.md) whose
        // filename carries a foreign code established by a sibling record is itself
        // subscriber-scoped and is blocked too.
        if !foreign_codes.is_empty() {
            for rel in &canon {
                let name = rel.to_string_lossy();
                if name.ends_with(".yaml") || name.ends_with(".yml") {
                    continue;
                }
                let stem = rel
                    .file_name()
                    .map(|n| n.to_string_lossy().to_ascii_lowercase())
                    .unwrap_or_default();
                if foreign_codes
                    .iter()
                    .any(|c| stem.split(['-', '_', '.']).any(|seg| seg == c))
                {
                    findings.push(Self::block(
                        rel,
                        BoundaryFindingKind::UnredactedEvidence,
                        format!(
                            "Canon companion file '{name}' carries a subscriber repo_code \
                             of a blocked sibling record (ACT-005:s5; ACT-007:s4)."
                        ),
                    ));
                }
            }
        }

        findings
    }
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
