//! The `vjs gazette` command: publish the Gazette data - both estates (the V2 canon from the
//! lawpack, the V1 archive from provenance) as one machine-readable file (REG-GAZETTE-CONTINUITY-001).

use super::*;

mod parse;
mod render;
use parse::*;

pub(crate) fn cmd_gazette(
    repo: &Path,
    out: Option<PathBuf>,
    json: bool,
) -> Result<(), KernelError> {
    const V2_BASE: &str = "https://github.com/wlilley93/vibe-justice-system/blob/master/";
    const V1_BASE: &str = "https://github.com/wlilley93/vibe-justice-system/blob/v1/";

    // WHERE the canon is comes from the resolver, and nowhere else ([2026] VJS-CC-VJS 15).
    // This command named `lawpack/v2` itself, so a subscriber that vendors no copy published
    // an EMPTY register - and stamped the pinned digest on it - instead of hitting the
    // CC-VJS 12 D1 refusal. The `?` on `LawpackLoader::load` that stood here looked like the
    // guard and was inert: every subtree read inside the loader is guarded by `.exists()`,
    // so a missing directory returns Ok with empty vectors and nothing to propagate.
    // The Gazette needs the DIRECTORY, not the loaded struct (it walks the kind
    // subdirectories itself and reads provenance/gazette/*.yaml, which `Lawpack` does not
    // carry), so it takes the directory from the resolver and reproduces the refusal here.
    let resolution = match resolve_lawpack(repo) {
        Some(r) => Some(r),
        // An invoked jurisdiction with no resolvable lawpack has no canon to publish, and
        // publishing an empty register from it is a false record, not a small one.
        None if is_invoked_jurisdiction(repo) => {
            return Err(vjs_engine::unresolvable_lawpack_error(repo));
        }
        // NOT a jurisdiction: the limb CC-VJS 12 preserved. Nothing to publish from, and
        // nothing to be wrong about - the run produces an empty register, which `meta`
        // now says plainly rather than stamping a pin on it (C4).
        None => None,
    };
    let lawpack_dir: Option<&Path> = resolution.as_ref().map(|r| r.dir.as_path());

    let io = |e: std::io::Error| KernelError::Io(e.to_string());
    let ser = |e: serde_yaml::Error| KernelError::Serialization(e.to_string());

    /// Pick the whitelisted keys of a YAML mapping into a JSON object,
    /// verbatim (parsed scalars keep their paragraph breaks; no summarising).
    /// The full-text body of a law object, by kind. This is what the in-place
    /// reader renders; only whitelisted fields leave the lawpack.
    /// Every law id and neutral citation the text of a record actually
    /// mentions: this is how a case interweaves by its subject. Negated
    /// mentions ("no DEC-X") are statements, not references (the same rule
    /// as the integrity checker).
    /// Capital Case for Act and Case names: significant words capitalised,
    /// small words lowered (never first or last), tokens already carrying
    /// capitals or digits (V1, VJS-PC, 2026, Computer-First) left alone.
    /// "[2026] VJS-PC 5" from an id like "2026-VJS-PC-005"; None otherwise.

    // V1 archive: derive a REALM-form neutral citation from the id + year when
    // the curated estate left one blank (REALM-SC-1 -> [2026] REALM-SC 1;
    // SI-6 -> [2026] REALM-SI 6; BILL-1 -> [2026] REALM-BILL 1).

    // Editorial overlay: presentation copy only, never force.
    #[derive(serde::Deserialize, Default)]
    struct Editorial {
        #[serde(default)]
        summary: String,
        #[serde(default)]
        points: Vec<String>,
        #[serde(default)]
        cites: Vec<String>,
    }
    let editorial: std::collections::HashMap<String, Editorial> = {
        let p = lawpack_dir.map(|d| d.join("provenance/gazette/editorial.yaml"));
        match p.filter(|p| p.exists()) {
            Some(p) => {
                let v: serde_yaml::Value =
                    serde_yaml::from_str(&std::fs::read_to_string(&p).map_err(io)?).map_err(ser)?;
                serde_yaml::from_value(v.get("items").cloned().unwrap_or_default()).map_err(ser)?
            }
            None => Default::default(),
        }
    };

    // Dates from history, two single passes over git log: oldest-first with
    // --diff-filter=A gives first-enacted; newest-first gives last-amended.
    // (Orders carry created_at and prefer it for the enactment date.)
    let added_at = git_dates(repo, &["--reverse", "--diff-filter=A"]);
    let updated_at = git_dates(repo, &[]);

    // Publication provenance: bind the artifact to the record it was
    // generated from. The on-disk lock keys (lawpack/digest/locked_at) do not
    // match Store::read_lawpack_lock's struct, so parse leniently here.
    let mut lock_meta: std::collections::HashMap<String, String> = Default::default();
    if let Ok(lock) = std::fs::read_to_string(repo.join(".vjs/lawpack.lock")) {
        for line in lock.lines() {
            if let Some((k, v)) = line.split_once('=') {
                lock_meta.insert(k.trim().to_string(), v.trim().trim_matches('"').to_string());
            }
        }
    }
    let source_commit = std::process::Command::new("git")
        .args(["-C", &repo.to_string_lossy(), "rev-parse", "HEAD"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    let mut items: Vec<serde_json::Value> = Vec::new();
    // Full-text bodies for the in-place reader, id -> kind-specific body.
    let mut texts: std::collections::BTreeMap<String, serde_json::Value> =
        std::collections::BTreeMap::new();
    // The Gazette registers legislation and case law only; the kernel
    // machinery is scheduled under REG-REALM-INVARIANTS-001 and read within
    // it (the instrument's own terms, assented per [2026] VJS-PC 7).
    const MACHINERY_INSTRUMENT: &str = "REG-REALM-INVARIANTS-001";
    const MACHINERY: [&str; 5] = ["invariant", "obligation", "rule", "spec", "decision"];
    let mut schedules: Vec<serde_json::Value> = Vec::new();

    let kinds = [
        ("statutes", "statute"),
        ("regulations", "regulation"),
        ("rules", "rule"),
        ("orders", "order"),
        ("specs", "spec"),
        ("invariants", "invariant"),
        ("decisions", "decision"),
        ("obligations", "obligation"),
    ];
    for (dir, kind) in kinds {
        // No resolved canon means no canon items. Reachable only on the not-a-jurisdiction
        // limb: an invoked jurisdiction was refused above.
        let Some(base) = lawpack_dir else { break };
        let d = base.join(dir);
        if !d.exists() {
            continue;
        }
        let mut entries: Vec<_> = std::fs::read_dir(&d)
            .map_err(io)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("yaml"))
            .collect();
        entries.sort();
        for path in entries {
            let content = std::fs::read_to_string(&path).map_err(io)?;
            let v: serde_yaml::Value = serde_yaml::from_str(&content).map_err(ser)?;
            let id = match s(&v, "id") {
                Some(id) => id,
                None => continue,
            };
            // Name first, always: an untitled order is headed by the name of
            // its subject (the issue it was defined against), never its raw id.
            let title = s(&v, "title")
                .or_else(|| {
                    s(&v, "issue").map(|issue| {
                        let last = issue.rsplit('.').next().unwrap_or(&issue);
                        let stem = last.trim_start_matches("vjs-v2-").replace(['-', '_'], " ");
                        let mut cs = stem.chars();
                        match cs.next() {
                            Some(f) => format!("{}{} (the order)", f.to_uppercase(), cs.as_str()),
                            None => issue.clone(),
                        }
                    })
                })
                .filter(|t| !t.trim().is_empty())
                .unwrap_or_else(|| id.clone());
            let citation = s(&v, "citation").unwrap_or_default();
            let status = s(&v, "status")
                .or_else(|| s(&v, "severity").map(|sev| format!("severity {}", sev)))
                .unwrap_or_default();
            // The displayed court follows the CITATION series where one exists,
            // so the court label always matches the neutral citation a reader
            // sees: VJS-CC -> County Court, VJS-PC -> Privy Council, VJS-SC ->
            // Supreme Court, REALM-CA -> Court of Appeal. A county-coded order
            // with NO citation (the founding boot slate) keeps the [2026] VJS-PC
            // 6 first-instance-is-the-Privy-Council presentation.
            let court = if kind == "order" {
                let cite = s(&v, "citation").unwrap_or_default();
                let raw = s(&v, "court").unwrap_or_default();
                if cite.contains("-SC ") || cite.contains("REALM-SC") || raw == "supreme_court" {
                    "sc"
                } else if cite.contains("-CA ") || cite.contains("REALM-CA") {
                    "ca"
                } else if cite.contains("-CC") {
                    "county"
                } else if cite.contains("-PC ")
                    || cite.contains("REALM-PC")
                    || raw == "privy_council"
                    || raw == "county"
                {
                    "pc"
                } else {
                    ""
                }
            } else {
                ""
            }
            .to_string();
            let title = if matches!(kind, "statute" | "regulation" | "rule" | "order") {
                title_case(&title)
            } else {
                title
            };

            // Mechanical fallbacks straight from the law text.
            let summary_field = match kind {
                "statute" | "spec" => "purpose",
                "regulation" | "obligation" => "text",
                "order" => "runtime_summary",
                "decision" => "decision",
                "invariant" => "remedy",
                "rule" => "summary",
                _ => "",
            };
            let mech_summary = s(&v, summary_field)
                .map(|t| first_sentence(&t))
                .unwrap_or_else(|| title.clone());
            let mech_points: Vec<String> = match kind {
                "statute" => v
                    .get("sections")
                    .and_then(|x| x.as_sequence())
                    .map(|secs| {
                        secs.iter()
                            .filter_map(|sec| {
                                let sid = sec.get("id")?.as_str()?;
                                let stitle = sec.get("title")?.as_str()?;
                                let n = sid.rsplit(':').next().unwrap_or(sid);
                                Some(format!("{} - {}", n, stitle))
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
                "order" => v
                    .get("directives")
                    .and_then(|x| x.as_sequence())
                    .map(|ds| {
                        ds.iter()
                            .filter_map(|d| d.get("must")?.as_str().map(humanize))
                            .collect()
                    })
                    .unwrap_or_default(),
                "decision" => v
                    .get("consequences")
                    .map(|c| str_list(c, "must").iter().map(|m| humanize(m)).collect())
                    .unwrap_or_default(),
                "regulation" => v
                    .get("kernel_effect")
                    .map(|k| {
                        let mut p: Vec<String> =
                            str_list(k, "must").iter().map(|m| humanize(m)).collect();
                        p.extend(
                            str_list(k, "must_not")
                                .iter()
                                .map(|m| format!("never {}", humanize(m))),
                        );
                        p.extend(
                            str_list(k, "prohibits")
                                .iter()
                                .map(|m| format!("prohibits {}", humanize(m))),
                        );
                        p
                    })
                    .unwrap_or_default(),
                "obligation" => {
                    let mut p = Vec::new();
                    if let Some(k) = s(&v, "kind") {
                        p.push(format!("performed by a {}", humanize(&k)));
                    }
                    if let Some(d) = s(&v, "due") {
                        p.push(format!("due {}", humanize(&d)));
                    }
                    p
                }
                "spec" => {
                    let mut p = Vec::new();
                    for key in ["decisions", "invariants", "obligations"] {
                        for r in str_list(&v, key) {
                            p.push(format!("carried by {}", r));
                        }
                    }
                    p
                }
                _ => Vec::new(),
            };

            // Mechanical citation edges: the law's own fields, plus every id
            // and neutral citation its text actually mentions (the subject
            // linkage; a case connects to the legislation it construes).
            let mut cites: Vec<String> = str_list(&v, "basis");
            cites.extend(str_list(&v, "supersedes"));
            if let Some(a) = s(&v, "authority") {
                cites.push(a);
            }
            for key in ["decisions", "invariants", "obligations"] {
                cites.extend(str_list(&v, key));
            }
            cites.extend(textual_refs(&content));

            let ed = editorial.get(&id);
            let summary = ed
                .filter(|e| !e.summary.is_empty())
                .map(|e| e.summary.clone())
                .unwrap_or(mech_summary);
            let points = ed
                .filter(|e| !e.points.is_empty())
                .map(|e| e.points.clone())
                .unwrap_or(mech_points);
            if let Some(e) = ed {
                cites.extend(e.cites.clone());
            }
            cites.sort();
            cites.dedup();
            cites.retain(|c| *c != id);

            // Not a canon read: the path the record occupies in the PUBLISHED apex tree,
            // which V2_BASE links to. At the resolver it would publish a checkout path.
            // LAWPACK-LITERAL: referent=local-records; status=local; authority=[2026] VJS-CC-VJS 15
            let rel = format!(
                "lawpack/v2/{}/{}",
                dir,
                path.file_name().unwrap().to_string_lossy()
            );
            let day = |s: &str| {
                s.split('T')
                    .next()
                    .unwrap_or("")
                    .trim_matches('"')
                    .to_string()
            };
            // Full-precision sort key: the record's declared created_at if it has
            // one, else the git commit timestamp, else the day at midnight. The
            // register orders newest-first on this, so same-day records keep
            // their true chronological order, not a kind tiebreak.
            let ts = s(&v, "created_at")
                .filter(|c| c.contains('T'))
                .or_else(|| added_at.get(&rel).cloned())
                .unwrap_or_default();
            let date = s(&v, "created_at")
                .map(|c| day(&c))
                .filter(|c| !c.is_empty())
                .or_else(|| added_at.get(&rel).map(|t| day(t)))
                .unwrap_or_default();
            let ts = if ts.contains('T') {
                ts
            } else {
                format!("{}T00:00:00Z", date)
            };
            let citation = if citation.is_empty() && kind == "order" {
                derive_order_citation(&id).unwrap_or_default()
            } else {
                citation
            };
            if MACHINERY.contains(&kind) {
                schedules.push(serde_json::json!({
                    "id": id, "kind": kind, "title": title,
                    "status": status, "summary": summary, "points": points,
                    "path": rel,
                }));
                continue;
            }
            let mut item = serde_json::json!({
                "id": id, "title": title, "citation": citation, "kind": kind,
                "court": court, "estate": "v2", "status": status, "date": date, "ts": ts,
                "summary": summary, "points": points, "cites": cites,
                "supersedes": str_list(&v, "supersedes"),
                "varies": str_list(&v, "varies"),
                "affirms": str_list(&v, "affirms"),
                "has_text": true,
                // the question before the court, shown up top on court documents
                "question": s(&v, "question").unwrap_or_default(),
                "path": rel, "url": format!("{}{}", V2_BASE, rel),
            });
            item["doc"] = serde_json::Value::String(format!(
                "law.html#{}",
                item["id"].as_str().unwrap_or("")
            ));
            // Court orders render as PDF (the machine YAML stands alongside as
            // the secondary on the page). The PDF is a rendering of this very
            // record, carried under pdfs/orders/.
            if kind == "order" {
                let id = item["id"].as_str().unwrap_or("");
                let pdf_rel = format!("pdfs/orders/{}.pdf", id);
                if repo.join(&pdf_rel).exists() {
                    item["pdf"] = serde_json::Value::String(pdf_rel);
                }
            }
            if let Some(asrc) = s(&v, "assent_source").filter(|a| !a.is_empty()) {
                item["assent_source"] = serde_json::Value::String(asrc);
            }
            item["updated"] = serde_json::Value::String(
                updated_at
                    .get(&rel)
                    .map(|t| day(t))
                    .unwrap_or_else(|| date.clone()),
            );
            // A case's subject: the problem it was defined against.
            if kind == "order" {
                if let Some(issue) = s(&v, "issue").filter(|i| !i.is_empty()) {
                    item["subject"] = serde_json::Value::String(humanize(&issue));
                }
                if let Some(op) = s(&v, "source_opinion").filter(|p| !p.is_empty()) {
                    item["opinion"] = serde_json::json!({
                        "path": op,
                        "url": format!("{}{}", V2_BASE, op.trim_start_matches("./")),
                    });
                }
                // the structured court record (REG-COURT-RECORD-001): the bench,
                // the vote, the pinned case-file digest - surfaced as a headnote
                let bench = str_list(&v, "bench");
                if !bench.is_empty() {
                    item["bench"] = serde_json::json!(bench);
                }
                for f in ["vote", "case_file_digest", "convened_at"] {
                    if let Some(val) = s(&v, f).filter(|x| !x.is_empty()) {
                        item[f] = serde_json::Value::String(val);
                    }
                }
            }
            items.push(item);
            texts.insert(
                items.last().unwrap()["id"].as_str().unwrap().to_string(),
                text_body(kind, &v),
            );
        }
    }

    // The consolidating instrument carries its schedules in full, grouped by
    // kind in schedule order ([2026] VJS-PC 7 D5).
    schedules.sort_by(|a, b| {
        let ord = |k: &str| MACHINERY.iter().position(|m| *m == k).unwrap_or(9);
        ord(a["kind"].as_str().unwrap_or(""))
            .cmp(&ord(b["kind"].as_str().unwrap_or("")))
            .then(a["id"].as_str().cmp(&b["id"].as_str()))
    });
    if let Some(body) = texts.get_mut(MACHINERY_INSTRUMENT) {
        let mut counts: std::collections::BTreeMap<String, usize> = Default::default();
        for sch in &schedules {
            *counts
                .entry(sch["kind"].as_str().unwrap_or("").to_string())
                .or_default() += 1;
        }
        body["schedules"] = serde_json::Value::Array(schedules.clone());
        if let Some(item) = items.iter_mut().find(|i| i["id"] == MACHINERY_INSTRUMENT) {
            let line = counts
                .iter()
                .map(|(k, n)| format!("{} {}s", n, k))
                .collect::<Vec<_>>()
                .join(", ");
            if let Some(pts) = item["points"].as_array_mut() {
                pts.push(serde_json::Value::String(format!("Schedules: {}", line)));
            }
        }
    }

    // The V1 archive estate: curated, frozen, existence-verified provenance.
    let v1_path = lawpack_dir.map(|d| d.join("provenance/gazette/v1-estate.yaml"));
    if let Some(v1_path) = v1_path.filter(|p| p.exists()) {
        let v: serde_yaml::Value =
            serde_yaml::from_str(&std::fs::read_to_string(&v1_path).map_err(io)?).map_err(ser)?;
        if let Some(seq) = v.get("items").and_then(|x| x.as_sequence()) {
            for it in seq {
                let path = s(it, "path").unwrap_or_default();
                let v1_id = s(it, "id").unwrap_or_default();
                let v1_kind = s(it, "kind").unwrap_or_default();
                let v1_title = s(it, "title").unwrap_or_default();
                let v1_title = if matches!(v1_kind.as_str(), "act" | "instrument" | "judgment") {
                    title_case(&v1_title)
                } else {
                    v1_title
                };
                // A record with no native V1 PDF renders as a webpage from its
                // frozen source markdown, in the Gazette's own document style.
                let mut archive_text = false;
                if let Some(src) = s(it, "source_md").filter(|x| !x.is_empty())
                    && let Ok(md) = std::fs::read_to_string(repo.join(&src))
                {
                    texts.insert(v1_id.clone(), serde_json::json!({ "archive_md": md }));
                    archive_text = true;
                }
                let v1_id_for_doc = v1_id.clone();
                let v1_date = s(it, "date").unwrap_or_default();
                let v1_citation = {
                    let c = s(it, "citation").unwrap_or_default();
                    if c.is_empty() {
                        derive_v1_citation(&v1_id_for_doc, &v1_date).unwrap_or_default()
                    } else {
                        c
                    }
                };
                items.push(serde_json::json!({
                    "id": v1_id,
                    "title": v1_title,
                    "citation": v1_citation,
                    "kind": s(it, "kind").unwrap_or_default(),
                    "court": s(it, "court").unwrap_or_default(),
                    "estate": "v1",
                    "status": s(it, "status").unwrap_or_default(),
                    "date": s(it, "date").unwrap_or_default(),
                    "ts": format!("{}T00:00:00Z", s(it, "date").unwrap_or_default()),
                    "summary": s(it, "summary").unwrap_or_default(),
                    "points": str_list(it, "points"),
                    "cites": str_list(it, "cites"),
                    "supersedes": [],
                    "has_text": false,
                    "question": s(it, "question").unwrap_or_default(),
                    "archive_text": archive_text,
                    "updated": s(it, "date").unwrap_or_default(),
                    "pdf": s(it, "pdf").unwrap_or_default(),
                    "doc": format!("law.html#{}", v1_id_for_doc),
                    "path": path.clone(),
                    // The honoured archive is the frozen V1-lineage corpus,
                    // which spans more than the vibe-justice-system v1 branch
                    // (e.g. agent-universe). An estate item may name its own
                    // frozen github source; otherwise the v1 branch is assumed.
                    "url": s(it, "url").filter(|u| !u.is_empty())
                        .unwrap_or_else(|| format!("{}{}", V1_BASE, path)),
                }));
            }
        }
    }

    // Authority lineage: the force chain, derived from the enacted structure
    // rather than textual citation. A record that never quotes the founding
    // act still holds its force through it; the Gazette shows that descent.
    // Per-kind anchors (skipped when the anchor is the item itself):
    //   canon:   statute -> the founding act (which itself traces to the
    //            assented Bill 32 in the archive); regulation -> its parent
    //            act (the textual `authority` already carries this); rule,
    //            decision, invariant, obligation, spec -> the Constitution
    //            and Sources of Authority Act (which constitutes those
    //            categories); order -> the courts-constitution order.
    //   archive: act -> the Acts of Union; instrument -> the SI-delegation
    //            act; judgment -> the courts/citations act.

    // An edge may only point at an item: a section cite collapses to its
    // parent act's item; anything else unresolved is dropped.
    let known: std::collections::HashSet<String> = items
        .iter()
        .map(|i| i["id"].as_str().unwrap_or_default().to_string())
        .collect();
    // The docket thread: orders on the same subject (the order's issue tag)
    // belong to one docket. They thread to the docket's ORIGIN - its first
    // entry - not each to its predecessor, because a docket is a hub, not a
    // sequence: the founding boot slate (BOOT-001..011) was passed together,
    // so BOOT-005 does not follow from BOOT-004; the orders are siblings of
    // one docket that opened with BOOT-001. Threading to the origin makes a
    // docket circle its first order instead of streaming off as a chain. (A
    // genuine appeal is a separate relationship, carried by appeal_of.)
    {
        // Issue tags are unique per case but carry family structure:
        // "governance.x" / "constitutional.x" are dotted dockets, and the
        // "vjs-v2-*" tags are the boot-series docket. The family is the
        // docket; the full issue stays on the item as its subject.
        fn subject_family(issue: &str) -> String {
            if let Some((fam, _)) = issue.split_once('.') {
                return fam.to_string();
            }
            if issue.starts_with("vjs-v2") {
                return "vjs-v2".into();
            }
            issue.to_string()
        }
        let mut by_subject: std::collections::BTreeMap<String, Vec<(String, String, usize)>> =
            std::collections::BTreeMap::new();
        for (idx, item) in items.iter().enumerate() {
            if item["kind"] == "order"
                && let Some(subj) = item["subject"].as_str()
            {
                by_subject.entry(subject_family(subj)).or_default().push((
                    item["date"].as_str().unwrap_or_default().to_string(),
                    item["id"].as_str().unwrap_or_default().to_string(),
                    idx,
                ));
            }
        }
        for (_, mut docket) in by_subject {
            docket.sort();
            // a docket of one is no thread; otherwise every member after the
            // first threads to the origin (the hub)
            if let Some((_, origin, _)) = docket.first().cloned() {
                for (_, _id, idx) in docket.iter().skip(1) {
                    items[*idx]["thread"] = serde_json::json!([origin]);
                }
            }
        }
    }

    render::write_estate_outputs(
        out,
        repo,
        render::Estate {
            items,
            texts,
            known,
            lock_meta,
            source_commit,
            resolution,
        },
        json,
    )
}
