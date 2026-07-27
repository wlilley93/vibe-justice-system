//! The lawpack validator: structural, referential-integrity, and citation-uniqueness checks over

//! a loaded Lawpack, plus the live citation allocator (live_citation_max). Operates on the data

//! model in this crate root; emits the report types in `report`.

use super::*;

pub struct LawpackValidator;

impl LawpackValidator {
    pub fn validate(lawpack: &Lawpack) -> Result<ValidationReport, KernelError> {
        let mut findings = Vec::new();
        let mut ok = true;

        // Duplicate authority-id detection across the WHOLE lawpack, not statute
        // sections alone. An id is how a reference (supersedes / cites_authorities / a
        // predicate's authority arg) resolves to exactly ONE authority; if two
        // authorities of ANY kind share an id, that resolution is silently ambiguous.
        // Members are walked in a fixed kind order so the findings are deterministic.
        // Pure order-vs-order collisions are owned by the constitutive CITATION_COLLISION
        // check below (and so not double-reported here); every other repeat - statute,
        // section, regulation, rule, spec, invariant, decision, obligation, and any
        // cross-kind reuse (e.g. an order id reused as an invariant id) - is DUPLICATE_ID.
        let mut members: Vec<(&str, &'static str)> = Vec::new();
        for s in &lawpack.statutes {
            members.push((s.id.0.as_str(), "statute"));
            for sec in &s.sections {
                members.push((sec.id.0.as_str(), "statute-section"));
            }
        }
        for r in &lawpack.regulations {
            members.push((r.id.0.as_str(), "regulation"));
        }
        for ru in &lawpack.rules {
            members.push((ru.id.0.as_str(), "rule"));
        }
        for sp in &lawpack.specs {
            members.push((sp.id.0.as_str(), "spec"));
        }
        for inv in &lawpack.invariants {
            members.push((inv.id.0.as_str(), "invariant"));
        }
        for d in &lawpack.decisions {
            members.push((d.id.0.as_str(), "decision"));
        }
        for ob in &lawpack.obligations {
            members.push((ob.id.as_str(), "obligation"));
        }
        for o in &lawpack.orders {
            members.push((o.id.as_str(), "order"));
        }
        let mut seen: std::collections::HashMap<&str, &'static str> =
            std::collections::HashMap::new();
        for (id, kind) in members {
            if let Some(prev) = seen.insert(id, kind) {
                // The order-vs-order namespace is the constitutive CITATION_COLLISION
                // check's domain; do not double-report it here.
                if prev == "order" && kind == "order" {
                    continue;
                }
                findings.push(ValidationFinding {
                    severity: Severity::Error,
                    code: "DUPLICATE_ID".into(),
                    path: None,
                    message: format!(
                        "Duplicate authority ID {id}: claimed by a {prev} and a {kind} (an id must resolve to exactly one authority)"
                    ),
                    suggested_fix: Some(
                        "Give each authority a unique id (use vjs next-citation for orders)".into(),
                    ),
                });
                ok = false;
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
            if !is_lawpack_yaml(path) {
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
            if !is_lawpack_yaml(path) {
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
    /// scanning every governed record's own top-level citation. This is the persisted
    /// register D2 requires the allocator to read - the citator INDEX is the count,
    /// not an empty in-memory registry. Returns 0 when the series is unstarted.
    ///
    /// TAKES ALL THE ROOTS, and that is the whole point of the signature. It used to
    /// take `lawpack/v2` alone. That directory holds 86 defining citations and NOT ONE
    /// of them is County, so `vjs next-citation CC 2026` returned `1` unconditionally
    /// while the series stood at 8, and the canon PC series mis-allocated the same way,
    /// offering `[2026] VJS-PC 20` while that citation was held by a live order. The
    /// D2 ruling this implements was recorded, and then the allocator was pointed at one
    /// register of three, so the ruling read as satisfied while the defect was live.
    /// Callers pass `front_door::governed_record_roots`.
    pub fn live_citation_max(
        roots: &[PathBuf],
        series: &str,
        repo: Option<&str>,
        year: i32,
    ) -> Result<u32, KernelError> {
        let want_series = series.to_ascii_uppercase();
        let want_repo = repo.map(|r| r.to_ascii_uppercase());
        let mut max = 0u32;
        for root in roots {
            for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
                let path = entry.path();
                if !is_lawpack_yaml(path) {
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
        }
        Ok(max)
    }
}

#[cfg(test)]
mod dup_id_coverage_tests {
    use super::*;

    fn empty_lawpack() -> Lawpack {
        Lawpack {
            statutes: vec![],
            regulations: vec![],
            rules: vec![],
            orders: vec![],
            specs: vec![],
            invariants: vec![],
            decisions: vec![],
            obligations: vec![],
        }
    }

    fn regulation(id: &str) -> Regulation {
        Regulation {
            id: AuthorityId(id.into()),
            citation: None,
            title: "t".into(),
            authority: "a".into(),
            status: AuthorityStatus::Binding,
            text: "binding text".into(),
            kernel_effect: None,
        }
    }

    fn obligation(id: &str) -> LawpackObligation {
        LawpackObligation {
            id: id.into(),
            title: "t".into(),
            status: "open".into(),
            kind: "k".into(),
            due: "2026-12-31".into(),
            required: false,
            text: "obligation text".into(),
            basis: vec![],
        }
    }

    fn dup_id_count(report: &ValidationReport) -> usize {
        report
            .findings
            .iter()
            .filter(|f| f.code == "DUPLICATE_ID")
            .count()
    }

    /// The old check covered statute sections ONLY: a regulation and an obligation could
    /// share an id with no complaint, even though a reference to that id then resolves
    /// ambiguously. The global namespace catches the cross-kind collision now.
    #[test]
    fn duplicate_id_across_previously_unchecked_kinds_is_caught() {
        let mut lp = empty_lawpack();
        lp.regulations.push(regulation("DUP-X"));
        lp.obligations.push(obligation("DUP-X"));
        let report = LawpackValidator::validate(&lp).unwrap();
        assert_eq!(
            dup_id_count(&report),
            1,
            "cross-kind id reuse must be one DUPLICATE_ID"
        );
        assert!(!report.ok, "a duplicate id is a hard error");
    }

    /// Intra-kind duplicates among a kind that was never checked before (obligations).
    #[test]
    fn duplicate_id_within_an_unchecked_kind_is_caught() {
        let mut lp = empty_lawpack();
        lp.obligations.push(obligation("OB-DUP"));
        lp.obligations.push(obligation("OB-DUP"));
        let report = LawpackValidator::validate(&lp).unwrap();
        assert_eq!(dup_id_count(&report), 1);
        assert!(!report.ok);
    }

    /// Distinct ids across kinds stay clean - no false positive.
    #[test]
    fn distinct_ids_across_kinds_are_clean() {
        let mut lp = empty_lawpack();
        lp.regulations.push(regulation("REG-A"));
        lp.obligations.push(obligation("OB-B"));
        let report = LawpackValidator::validate(&lp).unwrap();
        assert_eq!(dup_id_count(&report), 0);
    }
}

#[cfg(test)]
mod live_register_tests {
    use super::*;

    /// The allocator must read EVERY governed-record root, not just `lawpack/v2`.
    ///
    /// This is the regression lock for a live defect: `lawpack/v2` holds 86 defining
    /// citations and not one of them is County, so a single-root read returned 1 for
    /// every CC request while the series stood at 8. The canon PC series mis-allocated
    /// the same way, offering `[2026] VJS-PC 20` while a live order held it.
    ///
    /// BOTH series are asserted deliberately. A test that only measured CC would have
    /// passed against the PC half of the very same defect.
    #[test]
    fn reads_orders_and_court_registers_not_only_the_lawpack() {
        let tmp = std::env::temp_dir().join(format!("vjs-live-reg-{}", std::process::id()));
        let orders = tmp.join(".vjs/orders");
        let court = tmp.join(".vjs/court/orders");
        std::fs::create_dir_all(&orders).unwrap();
        std::fs::create_dir_all(&court).unwrap();
        // Deliberately NO lawpack/v2: a missing register is not evidence that a series
        // is unstarted, and the old code short-circuited that case straight to 0.
        std::fs::write(orders.join("a.yaml"), "id: a\ncitation: '[2026] VJS-CC-TK 4'\n").unwrap();
        std::fs::write(court.join("b.yaml"), "id: b\ncitation: \"[2026] VJS-PC 20\"\n").unwrap();

        let roots = vjs_core::front_door::governed_record_roots(&tmp);
        let cc = LawpackValidator::live_citation_max(&roots, "CC", Some("TK"), 2026).unwrap();
        let pc = LawpackValidator::live_citation_max(&roots, "PC", None, 2026).unwrap();
        std::fs::remove_dir_all(&tmp).ok();

        assert_eq!(cc, 4, "the County register under .vjs/orders was not read");
        assert_eq!(pc, 20, "the canon register under .vjs/court was not read");
    }
}
