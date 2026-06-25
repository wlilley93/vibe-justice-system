use regex::Regex;
use std::path::Path;

use vjs_core::*;

// This test module sits at the top by design; the lint that wants it last does not
// apply to a deliberately-led file.
#[cfg(test)]
#[allow(clippy::items_after_test_module)]
mod canon_gate_tests {
    use super::*;
    use std::path::PathBuf;

    fn is_blocked(findings: &[BoundaryFinding]) -> bool {
        !RedactScanner::check_public_safe(findings)
    }

    // The real DEC-OPBOX-UNITARY-STACK-001 shape: canon-format citation, no explicit
    // repo_code, subscriber org scope path, subscriber repo_code in the id.
    const OPBOX_DECISION: &str = r#"
id: DEC-OPBOX-UNITARY-STACK-001
citation: "[2026] VJS-DEC 15"
title: The Unitary Stack - one source of truth for Opbox kernel-owned data
scope:
  paths:
    - Executive/ministry-of-business-engineering-and-skills/engineering-department/projects/opbox/**
"#;

    #[test]
    fn blocks_the_opbox_decision_that_self_asserted_into_canon() {
        let (f, code) = RedactScanner::scan_canon_record(
            &PathBuf::from("lawpack/v2/decisions/DEC-OPBOX-UNITARY-STACK-001.yaml"),
            OPBOX_DECISION,
            "VJS",
            &[], // empty registry: scope-path corroboration alone must catch it
        );
        assert!(
            is_blocked(&f),
            "subscriber-scoped canon record must be blocked"
        );
        assert!(
            f.iter()
                .any(|x| matches!(x.kind, BoundaryFindingKind::PrivateRepoPath)),
            "the foreign scope path must surface as PrivateRepoPath"
        );
        assert_eq!(
            code.as_deref(),
            Some("OPBOX"),
            "the foreign code is corroborated"
        );
    }

    #[test]
    fn blocks_explicit_foreign_repo_code() {
        let rec = "id: DEC-X-001\nrepo_code: OPBOX\ncitation: \"[2026] VJS-DEC 99\"\n";
        let (f, _) = RedactScanner::scan_canon_record(
            &PathBuf::from("lawpack/v2/decisions/x.yaml"),
            rec,
            "VJS",
            &[],
        );
        assert!(is_blocked(&f));
        assert!(
            f.iter()
                .any(|x| matches!(x.kind, BoundaryFindingKind::UnredactedEvidence))
        );
    }

    #[test]
    fn registry_catches_a_foreign_id_code_without_scope_corroboration() {
        // No scope path, no repo_code field - only the id carries OPBOX. Corroboration
        // alone would miss it; the federation registry (#11) catches it.
        let rec = "id: DEC-OPBOX-SNEAKY-001\ncitation: \"[2026] VJS-DEC 98\"\ntitle: x\n";
        let codes = vec!["OPBOX".to_string()];
        let (f, code) = RedactScanner::scan_canon_record(
            &PathBuf::from("lawpack/v2/decisions/y.yaml"),
            rec,
            "VJS",
            &codes,
        );
        assert!(
            is_blocked(&f),
            "a registered subscriber code in the id must block"
        );
        assert_eq!(code.as_deref(), Some("OPBOX"));
        // Without the registry, the same record (no scope, no repo_code) passes.
        let (f2, _) = RedactScanner::scan_canon_record(
            &PathBuf::from("lawpack/v2/decisions/y.yaml"),
            rec,
            "VJS",
            &[],
        );
        assert!(
            !is_blocked(&f2),
            "uncorroborated, unregistered id-code does not trip"
        );
    }

    #[test]
    fn does_not_flag_a_clean_canon_record() {
        let clean = r#"
id: REG-KERNEL-001
citation: "[2026] VJS-REG 1"
title: The kernel is the single smart enforcement point
scope:
  paths:
    - crates/vjs-core/**
    - lawpack/v2/**
    - .vjs/**
"#;
        let (f, code) = RedactScanner::scan_canon_record(
            &PathBuf::from("lawpack/v2/regulations/REG-KERNEL-001.yaml"),
            clean,
            "VJS",
            &["OPBOX".to_string()],
        );
        assert!(
            !is_blocked(&f),
            "a clean canon record (KERNEL is not a foreign code) must pass: {f:?}"
        );
        assert_eq!(code, None);
    }

    #[test]
    fn prose_body_naming_a_subscriber_is_blocked_but_the_registry_is_exempt() {
        // PC-15 boundary cure: a canon record whose BODY/prose names a registered
        // subscriber - even with a clean id, citation, and no scope path - is blocked.
        let codes = vec!["OPBOX".to_string()];
        let leak = "id: 2026-VJS-PC-099\ncitation: \"[2026] VJS-PC 99\"\nholding: A subscriber (Opbox) asked canon to build a keystone.\n";
        let (f, _) = RedactScanner::scan_canon_record(
            &PathBuf::from("lawpack/v2/orders/2026-VJS-PC-099.yaml"),
            leak,
            "VJS",
            &codes,
        );
        assert!(is_blocked(&f), "a subscriber named in prose must block");
        // The registry file itself lists the codes by design - it is exempt.
        let registry = "id: FEDERATION-SUBSCRIBER-REGISTRY\ncodes:\n  - OPBOX\n";
        let (rf, _) = RedactScanner::scan_canon_record(
            &PathBuf::from("lawpack/v2/federation/subscriber-registry.yaml"),
            registry,
            "VJS",
            &codes,
        );
        assert!(!is_blocked(&rf), "the registry is exempt: {rf:?}");
    }

    #[test]
    fn wildcard_and_root_file_scopes_are_canon() {
        for p in ["*", "**", "Cargo.toml", "AGENTS.md", "public/**", "src/**"] {
            assert!(
                !RedactScanner::is_foreign_canon_path(p),
                "{p} must be treated as canon surface, not a foreign path"
            );
        }
        assert!(RedactScanner::is_foreign_canon_path(
            "Executive/ministry-of-business-engineering-and-skills/engineering-department/projects/opbox/**"
        ));
        assert!(RedactScanner::is_foreign_canon_path(
            "frontend-v2/prisma/**"
        ));
    }
}

pub struct RedactScanner;

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
    // by which eleven DEC-OPBOX/INV-OPBOX/SPEC-OPBOX files self-asserted into the
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
    /// `<TYPE>-<CODE>-...` (e.g. DEC-OPBOX-UNITARY-001 -> "OPBOX"). Returns the
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
                     ACT-007:s4: local law must not bind other repos; it cannot live in canon."
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
        canon_repo_code: &str,
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
        let mut foreign_codes: Vec<String> = Vec::new();
        for rel in &canon {
            let abs = repo_root.join(rel);
            let name = rel.to_string_lossy();
            let is_yaml = name.ends_with(".yaml") || name.ends_with(".yml");
            if !is_yaml {
                continue;
            }
            if let Ok(content) = std::fs::read_to_string(&abs) {
                let (mut fs, code) =
                    Self::scan_canon_record(rel, &content, canon_repo_code, &subscriber_codes);
                findings.append(&mut fs);
                if let Some(c) = code {
                    foreign_codes.push(c.to_ascii_lowercase());
                }
            }
        }

        // Companion pass: a non-YAML canon file (e.g. DEC-OPBOX-*.opinions.md) whose
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
/// by non-alphanumerics on both sides - so a subscriber code like "opbox" is caught in
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
