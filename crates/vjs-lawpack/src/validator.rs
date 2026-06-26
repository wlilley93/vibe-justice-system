//! The lawpack validator: structural, referential-integrity, and citation-uniqueness checks over

//! a loaded Lawpack, plus the live citation allocator (live_citation_max). Operates on the data

//! model in this crate root; emits the report types in `report`.

use super::*;

pub struct LawpackValidator;

impl LawpackValidator {
    pub fn validate(lawpack: &Lawpack) -> Result<ValidationReport, KernelError> {
        let mut findings = Vec::new();
        let mut ok = true;

        // Check for duplicate IDs
        let mut ids = std::collections::HashSet::new();
        for statute in &lawpack.statutes {
            for section in &statute.sections {
                if !ids.insert(section.id.0.clone()) {
                    findings.push(ValidationFinding {
                        severity: Severity::Error,
                        code: "DUPLICATE_ID".into(),
                        path: None,
                        message: format!("Duplicate authority ID: {}", section.id.0),
                        suggested_fix: Some("Change the ID to a unique value".into()),
                    });
                    ok = false;
                }
            }
        }

        // Check for citation collisions
        let mut citations = std::collections::HashSet::new();
        for order in &lawpack.orders {
            if !citations.insert(order.id.clone()) {
                findings.push(ValidationFinding {
                    severity: Severity::Error,
                    code: "CITATION_COLLISION".into(),
                    path: None,
                    message: format!("Duplicate order ID: {}", order.id),
                    suggested_fix: Some("Use vjs next-citation to get a unique citation".into()),
                });
                ok = false;
            }
        }

        // Check supersession targets exist
        for order in &lawpack.orders {
            for superseded in &order.supersedes {
                if !lawpack.orders.iter().any(|o| o.id == superseded.0) {
                    findings.push(ValidationFinding {
                        severity: Severity::Warning,
                        code: "ORPHAN_SUPERSESSION".into(),
                        path: None,
                        message: format!(
                            "Order {} supersedes non-existent authority {}",
                            order.id, superseded.0
                        ),
                        suggested_fix: Some("Verify the superseded ID exists".into()),
                    });
                }
            }
        }

        // Check text present in statute sections
        for statute in &lawpack.statutes {
            for section in &statute.sections {
                if section.text.is_empty() {
                    findings.push(ValidationFinding {
                        severity: Severity::Error,
                        code: "MISSING_TEXT".into(),
                        path: None,
                        message: format!("Section {} has no text", section.id.0),
                        suggested_fix: Some("Add text field with binding legal text".into()),
                    });
                    ok = false;
                }
                match &section.kernel_effect {
                    None => findings.push(ValidationFinding {
                        severity: Severity::Warning,
                        code: "NO_KERNEL_EFFECT".into(),
                        path: None,
                        message: format!("Section {} has no kernel_effect", section.id.0),
                        suggested_fix: Some("Add kernel_effect with structured executable rules".into()),
                    }),
                    // s.5(a) teeth-gate ([2026] VJS-CC 15): a kernel_effect that is
                    // present but binds to no recognized operation is inert ceremony,
                    // routed for correction (never voided).
                    //
                    // ENTRENCHED (ACT-ASSENTED-RECORD-PROTECTION, Sovereign-assented
                    // 2026-06-12, [2026] VJS-ACT 10): this severity must remain Warning.
                    // A Sovereign-assented record may never be voided or blocked by
                    // subordinate validation; its defects are always routed for
                    // correction. This is the general assented-record FLOOR, given full
                    // constitutional rank by Sovereign Assent (completing the invitation
                    // of [2026] VJS-SC 3; the s.5(a) gate is one instance). Changing a
                    // route-for-correction code to a blocking severity is amendable only
                    // by a Sovereign-assented constitutional Act citing
                    // ACT-ASSENTED-RECORD-PROTECTION by number, and breaks the
                    // assented-record-floor test by design.
                    Some(ke) if is_inert_kernel_effect(ke) => findings.push(ValidationFinding {
                        severity: Severity::Warning,
                        code: "S5_INERT_KERNEL_EFFECT".into(),
                        path: None,
                        message: format!(
                            "Section {} declares a kernel_effect that binds to no recognized operation (inert)",
                            section.id.0
                        ),
                        suggested_fix: Some(
                            "ACT-COMPUTER-FIRST-REALM s.5 / [2026] VJS-CC 15: routed for correction. Declare a recognized operation (defines/prohibits/must/must_not/may/proof/status) or extend the recognized set; never remove an assented record (D2-D5)."
                                .into(),
                        ),
                    }),
                    Some(_) => {}
                }
            }
        }

        // Check text present in regulations
        for regulation in &lawpack.regulations {
            if regulation.text.is_empty() {
                findings.push(ValidationFinding {
                    severity: Severity::Error,
                    code: "MISSING_TEXT".into(),
                    path: None,
                    message: format!("Regulation {} has no text", regulation.id.0),
                    suggested_fix: Some("Add text field with binding legal text".into()),
                });
                ok = false;
            }
            // s.5(a) teeth-gate ([2026] VJS-CC 15): a present-but-inert kernel_effect
            // is routed for correction (never voided). Severity ENTRENCHED as Warning
            // ([2026] VJS-PC 12 D3): never void/block an assented record; amendable
            // only by Sovereign-assented primary law citing s.5.
            if let Some(ke) = &regulation.kernel_effect
                && is_inert_kernel_effect(ke)
            {
                findings.push(ValidationFinding {
                        severity: Severity::Warning,
                        code: "S5_INERT_KERNEL_EFFECT".into(),
                        path: None,
                        message: format!(
                            "Regulation {} declares a kernel_effect that binds to no recognized operation (inert)",
                            regulation.id.0
                        ),
                        suggested_fix: Some(
                            "ACT-COMPUTER-FIRST-REALM s.5 / [2026] VJS-CC 15: routed for correction. Declare a recognized operation (defines/prohibits/must/must_not/may/proof/status) or extend the recognized set; never remove an assented record (D2-D5)."
                                .into(),
                        ),
                    });
            }
        }

        // Check word limits
        for order in &lawpack.orders {
            let word_count = order.runtime_summary.split_whitespace().count();
            if word_count > 500 {
                findings.push(ValidationFinding {
                    severity: Severity::Error,
                    code: "WORD_LIMIT_EXCEEDED".into(),
                    path: None,
                    message: format!(
                        "Order {} runtime summary exceeds word limit: {}/500",
                        order.id, word_count
                    ),
                    suggested_fix: Some("Shorten the runtime_summary to 500 words max".into()),
                });
                ok = false;
            }
        }

        Ok(ValidationReport { ok, findings })
    }

    /// Referential integrity: every law-object id cited anywhere in the
    /// lawpack must resolve to a defined object. Reported as warnings (the
    /// drift is real but the remedy is lawmaking, not a blocked commit).
    /// A negated mention ("no DEC-X") is a statement, not a reference.
    pub fn check_referential_integrity(
        lawpack_dir: &Path,
        lawpack: &Lawpack,
    ) -> Result<Vec<ValidationFinding>, KernelError> {
        let defined = defined_ids(lawpack);

        let id_pattern = regex::Regex::new(
            r"\b((?:ACT|DEC|INV|OBL|SPEC|REG)-[A-Z0-9][A-Za-z0-9-]*[A-Za-z0-9](?::s\d+)?)",
        )
        .map_err(|e| KernelError::InvalidInput(e.to_string()))?;

        let mut dangling: std::collections::BTreeMap<String, Vec<String>> =
            std::collections::BTreeMap::new();

        for entry in WalkDir::new(lawpack_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
                continue;
            }
            let raw = std::fs::read_to_string(path).map_err(|e| KernelError::Io(e.to_string()))?;
            // PC-17 D6: rejoin folded-scalar id splits (shared with the order gate) so a
            // YAML soft wrap cannot manufacture a partial-id false positive - the
            // REG-FEDERATION-COORDINATION-001 line-wrap class this session kept tripping.
            let content = crate::refs::dewrap(&raw);
            let rel = path
                .strip_prefix(lawpack_dir)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();
            for line in content.lines() {
                for m in id_pattern.find_iter(line) {
                    let id = m.as_str();
                    let negated = line[..m.start()].trim_end().ends_with("no");
                    if negated || defined.contains(id) {
                        continue;
                    }
                    dangling
                        .entry(id.to_string())
                        .or_default()
                        .push(rel.clone());
                }
            }
        }

        Ok(dangling
            .into_iter()
            .map(|(id, mut cited_in)| {
                cited_in.sort();
                cited_in.dedup();
                ValidationFinding {
                    severity: Severity::Warning,
                    code: "DANGLING_REFERENCE".into(),
                    path: None,
                    message: format!(
                        "'{}' is cited in [{}] but defined nowhere in the lawpack",
                        id,
                        cited_in.join(", ")
                    ),
                    suggested_fix: Some(
                        "Author the missing object by the lawmaking route, or remove the citation"
                            .into(),
                    ),
                }
            })
            .collect())
    }

    /// ACT-004:s8 (D2, [2026] VJS-PC 13): citations are deterministic and unique;
    /// collisions are fatal. The kernel's `must: check_citation_uniqueness`, given
    /// teeth here. Scans every canon record's OWN top-level `citation:` field (column
    /// zero, so references inside holdings/supersedes/basis are not miscounted) and
    /// fails closed when two distinct records claim the same citation - the class of
    /// defect that let eleven self-asserted "[2026] VJS-DEC 15..22" citations enter
    /// canon by hand. Allocation at authoring (vjs citation next) is the affirmative
    /// half; this is the reconciliation-at-write half. Runs on the full lawpack, so
    /// it does not depend on a record being staged.
    pub fn check_citation_uniqueness(
        lawpack_dir: &Path,
    ) -> Result<Vec<ValidationFinding>, KernelError> {
        let mut by_citation: std::collections::BTreeMap<String, Vec<String>> =
            std::collections::BTreeMap::new();

        for entry in WalkDir::new(lawpack_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
                continue;
            }
            let content =
                std::fs::read_to_string(path).map_err(|e| KernelError::Io(e.to_string()))?;
            let rel = path
                .strip_prefix(lawpack_dir)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();
            // The record's OWN citation is the top-level `citation:` field (column 0).
            for line in content.lines() {
                if let Some(rest) = line.strip_prefix("citation:") {
                    let cite = rest
                        .trim()
                        .trim_matches('"')
                        .trim_matches('\'')
                        .trim()
                        .to_string();
                    if !cite.is_empty() {
                        by_citation.entry(cite).or_default().push(rel.clone());
                    }
                    break; // one defining citation per record
                }
            }
        }

        // One finding PER colliding file, each carrying that file's repo-relative
        // path (#8), so the PC-14 D3 assent floor can downgrade the finding on an
        // assented record while keeping the others Fatal. lawpack_dir is .../lawpack/v2,
        // so the repo-relative path is lawpack/v2/<file>.
        let mut findings = Vec::new();
        for (cite, mut files) in by_citation.into_iter().filter(|(_, f)| f.len() > 1) {
            files.sort();
            files.dedup();
            for f in &files {
                let others: Vec<&String> = files.iter().filter(|x| *x != f).collect();
                findings.push(ValidationFinding {
                    severity: Severity::Fatal,
                    code: "CITATION_COLLISION".into(),
                    path: Some(PathBuf::from(format!("lawpack/v2/{f}"))),
                    message: format!(
                        "Citation '{cite}' is also claimed by {others:?}. ACT-004:s8: \
                         citations are unique; collisions are fatal."
                    ),
                    suggested_fix: Some(
                        "Allocate the citation through the kernel (vjs citation next) so it is \
                         unique; do not hand-assert a citation number."
                            .into(),
                    ),
                });
            }
        }
        Ok(findings)
    }

    /// Parse a citation string into (year, series_token_uppercase, repo_opt, n).
    /// Canon form "[YYYY] VJS-<SERIES> N"; subscriber form "[YYYY] VJS-<COURT>-<REPO> N".
    /// ACT-004:s8 format. Returns None for anything that is not a citation.
    pub fn parse_citation(s: &str) -> Option<(i32, String, Option<String>, u32)> {
        let re = regex::Regex::new(r"^\[(\d{4})\]\s+VJS-([A-Za-z]+)(?:-([A-Za-z0-9]+))?\s+(\d+)$")
            .ok()?;
        let c = re.captures(s.trim())?;
        let year: i32 = c.get(1)?.as_str().parse().ok()?;
        let series = c.get(2)?.as_str().to_ascii_uppercase();
        let repo = c.get(3).map(|m| m.as_str().to_ascii_uppercase());
        let n: u32 = c.get(4)?.as_str().parse().ok()?;
        Some((year, series, repo, n))
    }

    /// The live register's highest allocated N for (series, repo, year), read by
    /// scanning every canon record's own top-level citation. This is the persisted
    /// register D2 requires the allocator to read - the citator INDEX is the count,
    /// not an empty in-memory registry. Returns 0 when the series is unstarted.
    pub fn live_citation_max(
        lawpack_dir: &Path,
        series: &str,
        repo: Option<&str>,
        year: i32,
    ) -> Result<u32, KernelError> {
        let want_series = series.to_ascii_uppercase();
        let want_repo = repo.map(|r| r.to_ascii_uppercase());
        let mut max = 0u32;
        for entry in WalkDir::new(lawpack_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
                continue;
            }
            let content =
                std::fs::read_to_string(path).map_err(|e| KernelError::Io(e.to_string()))?;
            for line in content.lines() {
                if let Some(rest) = line.strip_prefix("citation:") {
                    let cite = rest.trim().trim_matches('"').trim_matches('\'').trim();
                    if let Some((y, s, r, n)) = Self::parse_citation(cite)
                        && y == year
                        && s == want_series
                        && r.as_deref().map(|x| x.to_string()) == want_repo.clone()
                        && n > max
                    {
                        max = n;
                    }
                    break; // the record's own citation only
                }
            }
        }
        Ok(max)
    }
}
