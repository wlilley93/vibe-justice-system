//! The seven check families of the Clerk Gate (`vjs draft check`), split from the
//! command plumbing under the structural ceiling. Each is deterministic, reuses the
//! kernel's own readers where one exists, and is seeded from a REAL historical error
//! (see the module doc in ../draft_check.rs's mod.rs and the red-seed tests).

use super::*;
use std::collections::{HashMap, HashSet};

/// Top-level keys the kernel reads OUTSIDE the Statute struct (text-scanners), so a
/// draft carrying them is not carrying dropped law: `assent_source` (vjs-engine
/// assent.rs top_level) and `created_at` (the Gazette's declared-date read).
const EXTRA_TOP_KEYS: &[&str] = &["assent_source", "created_at"];

fn mapping_keys(v: &serde_yaml::Value) -> Vec<String> {
    v.as_mapping()
        .map(|m| {
            m.keys()
                .filter_map(|k| k.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

/// Keys in `draft` that the kernel's parse DROPS: present in the draft mapping,
/// absent after a round-trip through the real struct. The round-trip is the point -
/// the check can never disagree with the parser, because it IS the parser.
fn dropped_keys(draft: &serde_yaml::Value, roundtrip: &serde_yaml::Value) -> Vec<String> {
    let kept: HashSet<String> = mapping_keys(roundtrip).into_iter().collect();
    mapping_keys(draft)
        .into_iter()
        .filter(|k| !kept.contains(k))
        .collect()
}

pub(super) fn struct_key_findings(
    draft_val: &serde_yaml::Value,
    statute: &Statute,
    out: &mut Vec<DFind>,
) {
    let rt = serde_yaml::to_value(statute).expect("statute reserializes");
    for k in dropped_keys(draft_val, &rt) {
        if EXTRA_TOP_KEYS.contains(&k.as_str()) {
            continue;
        }
        out.push(d(
            Severity::Error,
            "DRAFT-UNKNOWN-KEY",
            format!(
                "top-level key '{k}' is not a Statute field: the kernel parse DROPS it and \
                 everything under it. Law written under this key does not exist at runtime."
            ),
        ));
    }
    let empty = Vec::new();
    let draft_sections = draft_val["sections"].as_sequence().unwrap_or(&empty);
    for (i, (ds, ps)) in draft_sections.iter().zip(&statute.sections).enumerate() {
        let prt = serde_yaml::to_value(ps).expect("section reserializes");
        for k in dropped_keys(ds, &prt) {
            out.push(d(
                Severity::Error,
                "DRAFT-UNKNOWN-KEY",
                format!(
                    "section {} ('{}'): key '{k}' is not a StatuteSection field - dropped by \
                     the kernel parse (the silently-dropped-law class).",
                    i + 1,
                    ps.id.0
                ),
            ));
        }
        let dke = &ds["kernel_effect"];
        if dke.is_mapping()
            && let Some(pke) = &ps.kernel_effect
        {
            let kert = serde_yaml::to_value(pke).expect("kernel_effect reserializes");
            for k in dropped_keys(dke, &kert) {
                out.push(d(
                    Severity::Error,
                    "DRAFT-UNKNOWN-KEY",
                    format!(
                        "section '{}' kernel_effect: key '{k}' is not a KernelEffect field \
                             (must/may/must_not/exceptions/proof/defines/prohibits/status/when). \
                             The ACT-COMPUTER-FIRST-REALM `force_source` class: a duty spelled \
                             wrong is a duty that does not exist.",
                        ps.id.0
                    ),
                ));
            }
            if dke["when"].is_mapping() {
                for k in mapping_keys(&dke["when"]) {
                    if k != "any" && k != "all" {
                        out.push(d(
                            Severity::Error,
                            "DRAFT-UNKNOWN-KEY",
                            format!(
                                "section '{}' kernel_effect.when: key '{k}' is not a \
                                     Condition field (any/all) - dropped by the kernel parse.",
                                ps.id.0
                            ),
                        ));
                    }
                }
            }
        }
    }
}

pub(super) fn draft_duty_tokens(statute: &Statute) -> Vec<String> {
    let mut toks = Vec::new();
    for s in &statute.sections {
        if let Some(ke) = &s.kernel_effect {
            for items in [&ke.must, &ke.must_not, &ke.prohibits]
                .into_iter()
                .flatten()
            {
                toks.extend(items.iter().cloned());
            }
        }
    }
    toks
}

pub(super) fn duty_findings(
    statute: &Statute,
    lawpack: &Lawpack,
    out: &mut Vec<DFind>,
) -> (usize, usize) {
    let toks = draft_duty_tokens(statute);
    let mut seen: HashMap<&str, usize> = HashMap::new();
    for t in &toks {
        *seen.entry(t.as_str()).or_default() += 1;
    }
    for (t, n) in seen {
        if n > 1 {
            out.push(d(
                Severity::Warning,
                "DRAFT-DUTY-DUP",
                format!(
                    "duty token '{t}' declared {n} times in this draft - the gate registry \
                     maps a token once, so duplicates are one duty wearing two claims."
                ),
            ));
        }
    }
    let existing = vjs_lawpack::conformance_audit(lawpack);
    let draft_id = &statute.id.0;
    for dc in &existing.duties {
        if !dc.instrument.starts_with(draft_id.as_str()) && toks.contains(&dc.token) {
            out.push(d(
                Severity::Warning,
                "DRAFT-DUTY-COLLISION",
                format!(
                    "duty token '{}' is already declared by {} - two instruments claiming one \
                     token make the conformance map ambiguous about whose duty a gate holds.",
                    dc.token, dc.instrument
                ),
            ));
        }
    }
    let synthetic = Lawpack {
        statutes: vec![statute.clone()],
        regulations: Vec::new(),
        rules: Vec::new(),
        orders: Vec::new(),
        specs: Vec::new(),
        invariants: Vec::new(),
        decisions: Vec::new(),
        obligations: Vec::new(),
    };
    let preview = vjs_lawpack::conformance_audit(&synthetic);
    (preview.wired, preview.unwired)
}

/// Ground the draft's citations against the SAME corpus the commit gate uses.
/// PC-17 analog for statutes: SECTIONS are operative; the purpose block is recital
/// and is not checked. Section text grounds at Error; commentary at Warning.
pub(super) fn citation_findings(
    repo: &Path,
    statute: &Statute,
    lawpack: &Lawpack,
    out: &mut Vec<DFind>,
) {
    let corpus = vjs_engine::grounding::grounding_corpus(repo, lawpack);
    let mut defined = corpus.defined;
    let mut in_force = corpus.in_force;
    defined.insert(statute.id.0.clone());
    in_force.insert(statute.id.0.clone());
    for s in &statute.sections {
        defined.insert(s.id.0.clone());
        in_force.insert(s.id.0.clone());
    }
    let mut operative = String::new();
    let mut commentary = String::new();
    for s in &statute.sections {
        operative.push_str(&s.text);
        operative.push('\n');
        if let Some(c) = &s.commentary {
            commentary.push_str(c);
            commentary.push('\n');
        }
    }
    for (text, unresolved_sev, label) in [
        (&operative, Severity::Error, "section text"),
        (&commentary, Severity::Warning, "commentary"),
    ] {
        for (tok, g) in
            vjs_lawpack::refs::ground_operative(text, &defined, &corpus.citations, &in_force)
        {
            match g {
                vjs_lawpack::refs::Grounding::Unresolved => out.push(d(
                    unresolved_sev.clone(),
                    "DRAFT-CITE-UNRESOLVED",
                    format!(
                        "{label} cites '{tok}', which resolves to no defined authority in \
                         this corpus (existence limb, PC-17 D1)."
                    ),
                )),
                vjs_lawpack::refs::Grounding::NotInForce => out.push(d(
                    Severity::Warning,
                    "DRAFT-CITE-NOT-IN-FORCE",
                    format!("{label} cites '{tok}', which is defined but not in force."),
                )),
                vjs_lawpack::refs::Grounding::Resolved => {}
            }
        }
    }
}

/// Resolve a cited path the way adopted law actually cites: exact from the repo root
/// or governance/, then with the leading component stripped (the `opbox-kernel/...`
/// repo-qualified form), then by suffix/basename over a one-walk index (bare
/// `types.rs:371`-style crate cites). All three forms appear in the ENACTED Acts;
/// a resolver narrower than the citing conventions makes the check cry wolf.
fn resolve_cited(repo: &Path, index: &HashMap<String, Vec<PathBuf>>, rel: &str) -> Vec<PathBuf> {
    let mut cands = vec![repo.join(rel), repo.join("governance").join(rel)];
    if let Some((_, stripped)) = rel.split_once('/') {
        cands.push(repo.join(stripped));
        cands.push(repo.join("governance").join(stripped));
    }
    let mut hits: Vec<PathBuf> = cands.into_iter().filter(|p| p.is_file()).collect();
    if hits.is_empty()
        && let Some(base) = rel.rsplit('/').next()
        && let Some(named) = index.get(base)
    {
        // Prefer suffix matches on the cited path; fall back to any file of that name.
        let suffix: Vec<PathBuf> = named
            .iter()
            .filter(|p| p.to_string_lossy().ends_with(rel))
            .cloned()
            .collect();
        hits = if suffix.is_empty() {
            named.clone()
        } else {
            suffix
        };
    }
    hits
}

fn file_index(repo: &Path) -> HashMap<String, Vec<PathBuf>> {
    let mut index: HashMap<String, Vec<PathBuf>> = HashMap::new();
    for e in walkdir::WalkDir::new(repo)
        .into_iter()
        .filter_entry(|e| {
            e.file_name() != ".git" && e.file_name() != "target" && e.file_name() != "node_modules"
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        index
            .entry(e.file_name().to_string_lossy().to_string())
            .or_default()
            .push(e.path().to_path_buf());
    }
    index
}

/// Every `path:line` / `path:a-b` claim is opened and checked: the file exists, the
/// line exists, and where the SAME draft line backtick-quotes a token, that token
/// appears within +-2 lines of the cited address.
pub(super) fn address_findings(repo: &Path, draft_text: &str, out: &mut Vec<DFind>) {
    // The path class starts with `.` too: `.justice/...` and `.vjs/...` are real cited
    // roots, and clipping the dot made a resolvable cite unresolvable (found on the
    // enacted ACT 11 smoke run).
    let addr = regex::Regex::new(
        r"([A-Za-z0-9_.][A-Za-z0-9_./-]*\.(?:rs|md|toml|yaml|yml|sh|json|ts|js)):(\d+)(?:-(\d+))?",
    )
    .unwrap();
    let tick = regex::Regex::new(r"`([^`]+)`").unwrap();
    let index = file_index(repo);
    for (ln, line) in draft_text.lines().enumerate() {
        for cap in addr.captures_iter(line) {
            let rel = &cap[1];
            // A COLON-QUALIFIED cite (`repo:path/file.md:44`) names a file in ANOTHER
            // repository, on its face. Erroring on it made the checker cry wolf twice
            // on enacted law (measured 2026-08-05: the .justice continuity cites). It
            // is disclosed, never silently passed - and never checked here, because
            // this tree is the wrong referent to check it against.
            let m = cap.get(1).unwrap();
            if m.start() > 0 && line.as_bytes()[m.start() - 1] == b':' {
                out.push(d(
                    Severity::Info,
                    "DRAFT-ADDRESS-CROSS-REPO",
                    format!(
                        "draft line {}: cites {rel} under a repo qualifier - a file in \
                         another repository, not verifiable from this tree. Disclosed, \
                         not checked.",
                        ln + 1
                    ),
                ));
                continue;
            }
            let start: usize = cap[2].parse().unwrap_or(0);
            let end: usize = cap
                .get(3)
                .map(|m| m.as_str().parse().unwrap_or(start))
                .unwrap_or(start);
            let hits = resolve_cited(repo, &index, rel);
            if hits.is_empty() {
                out.push(d(
                    Severity::Error,
                    "DRAFT-ADDRESS-BAD",
                    format!(
                        "draft line {}: cites {rel}:{} but no such file exists anywhere in \
                         this repo (tried exact, governance/-relative, repo-prefix-stripped, \
                         and basename forms).",
                        ln + 1,
                        &cap[2]
                    ),
                ));
                continue;
            }
            // A cite is line-valid if ANY resolution of it has the cited lines; the
            // token window is then checked against the first line-valid resolution.
            let mut valid: Option<Vec<String>> = None;
            let mut max_lines = 0usize;
            for h in &hits {
                let Ok(content) = std::fs::read_to_string(h) else {
                    continue;
                };
                let n = content.lines().count();
                max_lines = max_lines.max(n);
                if start > 0 && start <= n && end <= n {
                    valid = Some(content.lines().map(|l| l.to_string()).collect());
                    break;
                }
            }
            let Some(lines) = valid else {
                out.push(d(
                    Severity::Error,
                    "DRAFT-ADDRESS-BAD",
                    format!(
                        "draft line {}: cites {rel}:{}{} but the longest matching file has \
                         {max_lines} lines.",
                        ln + 1,
                        start,
                        cap.get(3)
                            .map(|m| format!("-{}", m.as_str()))
                            .unwrap_or_default(),
                    ),
                ));
                continue;
            };
            // Token proximity: the `:6`-for-`:7` class. Only when the draft line quotes
            // a token; +-2 tolerates rewraps without letting a dangling cite pass.
            let tokens: Vec<&str> = tick
                .captures_iter(line)
                .map(|c| c.get(1).unwrap().as_str())
                .filter(|t| !t.contains(rel) && !rel.contains(t))
                .collect();
            if !tokens.is_empty() {
                let lo = start.saturating_sub(3);
                let hi = (end + 2).min(lines.len());
                let window = &lines[lo..hi];
                if !tokens.iter().any(|t| window.iter().any(|l| l.contains(*t))) {
                    out.push(d(
                        Severity::Warning,
                        "DRAFT-ADDRESS-TOKEN",
                        format!(
                            "draft line {}: cites {rel}:{start} and quotes {:?}, but none of \
                             those tokens appear within 2 lines of the cited address.",
                            ln + 1,
                            tokens
                        ),
                    ));
                }
            }
        }
    }
}

/// A commencement provision exists, and every internal `sN` reference resolves to a
/// real section of THIS draft. (Cross-instrument `ACT-X:sN` references are grounded
/// by the citation check; this one catches the bare in-act references.)
pub(super) fn commencement_findings(statute: &Statute, out: &mut Vec<DFind>) {
    let sec_nums: HashSet<u64> = statute
        .sections
        .iter()
        .filter_map(|s| {
            s.id.0
                .rsplit(":s")
                .next()
                .and_then(|n| n.parse::<u64>().ok())
        })
        .collect();
    let has_commencement = statute.sections.iter().any(|s| {
        s.title.to_lowercase().contains("commence") || s.text.to_lowercase().contains("commence")
    });
    if !has_commencement {
        out.push(d(
            Severity::Warning,
            "DRAFT-COMMENCEMENT-MISSING",
            "no section commences this Act (no 'commence' in any title or text) - an Act \
             that never commences binds nothing."
                .into(),
        ));
    }
    let sref = regex::Regex::new(r"(^|[^A-Za-z0-9:/-])ss?\.?(\d+)").unwrap();
    // A line that names another instrument ("ACT-X:s10 and s25", "[2026] VJS-PC 17")
    // uses bare sN as shorthand for THAT instrument's sections, not this draft's -
    // proven on the enacted ACT 11, where s15's recital of the entrenched floor drew
    // three false s25 findings. Such lines are skipped, and the skip is the disclosed
    // limit of this check: a broken internal ref on a line that also cites an
    // instrument escapes it.
    let external = |line: &str| {
        ["ACT-", "REG-", "RULE-", "DEC-", "OBL-", "INV-", "[20"]
            .iter()
            .any(|m| line.contains(m))
    };
    for s in &statute.sections {
        for text in [Some(&s.text), s.commentary.as_ref()].into_iter().flatten() {
            for line in text.lines().filter(|l| !external(l)) {
                for cap in sref.captures_iter(line) {
                    if let Ok(n) = cap[2].parse::<u64>()
                        && !sec_nums.contains(&n)
                    {
                        out.push(d(
                            Severity::Error,
                            "DRAFT-SECTION-REF-BAD",
                            format!(
                                "section '{}' references s{n}, but this draft has no section {n} \
                             (sections present: {:?}).",
                                s.id.0,
                                {
                                    let mut v: Vec<_> = sec_nums.iter().collect();
                                    v.sort();
                                    v
                                }
                            ),
                        ));
                    }
                }
            }
        }
    }
}

/// Two totals for one measurement: the 84/85 class - a draft asserted "84 registered
/// stores" in one section and "85" in another after a re-measure, and a committee
/// round was spent finding it. Deterministic form: the SAME two-word phrase preceded
/// by two DIFFERENT numbers in one draft is one measurement wearing two totals.
/// Warning, not Error - two counts of genuinely different things can share a phrase;
/// the counts block below is the real cure (the kernel measures, the draft recites).
pub(super) fn count_findings(draft_text: &str, out: &mut Vec<DFind>) {
    let numphrase = regex::Regex::new(r"\b(\d+)\s+([a-z][a-z-]+\s+[a-z][a-z-]+)").unwrap();
    // "Part 1 of this Act" / "s4 and s7" are structure, not measurements.
    const STOP: &[&str] = &["of", "and", "or", "to", "the", "in", "a", "is", "for", "at"];
    let mut by_phrase: HashMap<String, HashSet<String>> = HashMap::new();
    for cap in numphrase.captures_iter(&draft_text.to_lowercase()) {
        let first = cap[2].split_whitespace().next().unwrap_or("");
        if STOP.contains(&first) {
            continue;
        }
        by_phrase
            .entry(cap[2].to_string())
            .or_default()
            .insert(cap[1].to_string());
    }
    let mut phrases: Vec<_> = by_phrase
        .into_iter()
        .filter(|(_, ns)| ns.len() > 1)
        .collect();
    phrases.sort_by(|a, b| a.0.cmp(&b.0));
    for (phrase, nums) in phrases {
        let mut ns: Vec<_> = nums.into_iter().collect();
        ns.sort();
        out.push(d(
            Severity::Warning,
            "DRAFT-INCONSISTENT-COUNT",
            format!(
                "'{phrase}' appears with {} different totals ({}) - if these are one \
                 measurement, one of them is wrong; re-measure and state it once.",
                ns.len(),
                ns.join(", ")
            ),
        ));
    }
}

/// The publication denylist, applied at DRAFTING instead of at the enactment commit.
///
/// REPAIRED 2026-08-05 on the Guardrail seat's proof-by-execution: the first version
/// compared each REGISTER LINE as a plaintext substring against the draft. The register
/// holds sha256 hashes, never plaintext ([2026] VJS-CC-VJS 17 C7), so the limb could
/// never fire - it reported zero findings on texts independently proven to carry twenty
/// registered tokens - and had the register ever been seeded with plaintext, the finding
/// message would have PRINTED the private term. One matcher now serves every door
/// (vjs_redact::Denylist, the same tokeniser the publication gate runs), the line is
/// disclosed, and the term never is.
pub(super) fn denylist_findings(repo: &Path, draft_text: &str, out: &mut Vec<DFind>) {
    match vjs_redact::Denylist::load(repo) {
        Err(e) => out.push(d(
            Severity::Info,
            "DRAFT-DENYLIST-UNCHECKED",
            format!("the denylist limb DID NOT RUN: {e}. This is a disclosure, not a pass."),
        )),
        Ok(deny) => {
            for n in deny.hit_lines(draft_text) {
                out.push(d(
                    Severity::Error,
                    "DRAFT-DENYLISTED-TERM",
                    format!(
                        "draft line {n} carries a term on the publication denylist \
                         (.vjs/publication-denylist.txt). The term is NOT named here: naming \
                         it would publish it. It will be refused at the canon enactment \
                         commit - redact to the generic form or the accessioned pseudonym \
                         now, not then."
                    ),
                ));
            }
        }
    }
}
