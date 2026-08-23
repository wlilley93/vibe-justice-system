//! The gazette OUTPUT phase: resolve cross-references between estate items, assemble the
//! data payload, and write the gazette-data.js / -text.js / .json / .xml / .html artifacts.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};

use super::parse::*;
use crate::*;

/// The estate data the build phase produces and the output phase consumes.
pub(crate) struct Estate {
    pub items: Vec<serde_json::Value>,
    pub texts: BTreeMap<String, serde_json::Value>,
    pub known: HashSet<String>,
    pub lock_meta: HashMap<String, String>,
    pub source_commit: String,
    /// Which tree this publication was actually built from, and how it was found.
    /// `None` only on the not-a-jurisdiction limb, where nothing was published.
    pub resolution: Option<vjs_engine::LawpackResolution>,
}

pub(crate) fn write_estate_outputs(
    out: Option<PathBuf>,
    repo: &Path,
    estate: Estate,
    json: bool,
) -> Result<(), KernelError> {
    let Estate {
        mut items,
        texts,
        known,
        lock_meta,
        source_commit,
        resolution,
    } = estate;
    const SITE_BASE: &str = "https://wlilley93.github.io/vibe-justice-system/";
    const FEED_TAG: &str = "tag:wlilley93.github.io,2026-06-09:vibe-justice-system:gazette";
    let io = |e: std::io::Error| KernelError::Io(e.to_string());
    let resolve = |list: &[String], own_id: &str, known: &std::collections::HashSet<String>| {
        let mut out: Vec<String> = list
            .iter()
            .filter_map(|c| {
                if known.contains(c) {
                    Some(c.clone())
                } else {
                    let parent = c.split(':').next().unwrap_or(c);
                    known.contains(parent).then(|| parent.to_string())
                }
            })
            .filter(|c| c != own_id)
            .collect();
        out.sort();
        out.dedup();
        out
    };

    let mut dropped = 0usize;
    let mut superseded_by: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    // Treatment inverses: a higher court that varies/affirms a lower order is
    // recorded back on that order as varied_by/affirmed_by, so no node is a stale
    // dead-end. Mirrors superseded_by. Edges to off-gazette orders (e.g. County,
    // which live in .vjs/court/orders not lawpack/v2/orders) resolve away.
    let mut varied_by: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    let mut affirmed_by: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    for item in &mut items {
        let own_id = item["id"].as_str().unwrap_or_default().to_string();
        let estate = item["estate"].as_str().unwrap_or_default().to_string();
        let kind = item["kind"].as_str().unwrap_or_default().to_string();

        let raw_cites: Vec<String> = item["cites"]
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|c| c.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let resolved = resolve(&raw_cites, &own_id, &known);
        dropped += raw_cites.len().saturating_sub(resolved.len());
        item["cites"] = serde_json::Value::Array(
            resolved
                .iter()
                .cloned()
                .map(serde_json::Value::String)
                .collect(),
        );

        let raw_sup: Vec<String> = item["supersedes"]
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|c| c.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let sup = resolve(&raw_sup, &own_id, &known);
        for target in &sup {
            superseded_by
                .entry(target.clone())
                .or_default()
                .push(own_id.clone());
        }
        item["supersedes"] =
            serde_json::Value::Array(sup.into_iter().map(serde_json::Value::String).collect());

        for (field, map) in [("varies", &mut varied_by), ("affirms", &mut affirmed_by)] {
            let raw: Vec<String> = item[field]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|c| c.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();
            let resolved_t = resolve(&raw, &own_id, &known);
            for target in &resolved_t {
                map.entry(target.clone()).or_default().push(own_id.clone());
            }
            item[field] = serde_json::Value::Array(
                resolved_t
                    .into_iter()
                    .map(serde_json::Value::String)
                    .collect(),
            );
        }

        // A case interweaves by its SUBJECT: the legislation it construes
        // (resolved citations) or the docket thread on its issue. The
        // constitutional anchor is a last resort against orphaning only.
        let case_like = kind == "order" || kind == "judgment";
        let threaded = item["thread"]
            .as_array()
            .map(|a| !a.is_empty())
            .unwrap_or(false);
        let anchor = if case_like && (!resolved.is_empty() || threaded) {
            None
        } else {
            lineage_anchor(&estate, &kind, &own_id).or_else(|| {
                // A regulation normally hangs off its textual `authority`; if
                // that failed to resolve it still holds force under the SI power.
                (estate == "v2" && kind == "regulation" && resolved.is_empty())
                    .then_some("ACT-CONSOLIDATION-FRAMEWORK")
            })
        };
        let lineage: Vec<serde_json::Value> = anchor
            .filter(|a| known.contains(*a) && !resolved.contains(&a.to_string()))
            .map(|a| serde_json::Value::String(a.to_string()))
            .into_iter()
            .collect();
        item["lineage"] = serde_json::Value::Array(lineage);
    }
    for item in &mut items {
        let own_id = item["id"].as_str().unwrap_or_default().to_string();
        for (field, map) in [
            ("superseded_by", &superseded_by),
            ("varied_by", &varied_by),
            ("affirmed_by", &affirmed_by),
        ] {
            let mut v = map.get(&own_id).cloned().unwrap_or_default();
            v.sort();
            v.dedup();
            item[field] =
                serde_json::Value::Array(v.into_iter().map(serde_json::Value::String).collect());
        }
        // Every V1 archive node carries the uniform migration relation to the V2
        // canon, so no honoured-archive record is a navigable dead-end. It was
        // superseded as live law by the consolidation (ACT-COMPUTER-FIRST-REALM
        // s.6; preserved as archive by s.7) and its settled law was restated in
        // Schedule 1 of the framework (ACT-CONSOLIDATION-FRAMEWORK s.4), live in
        // V2 only where expressly incorporated (s.20). This is a MIGRATION edge,
        // not per-ruling court treatment: no V2 record varies/affirms/overrules an
        // individual V1 node, and V1 declares no such treatment of itself. Both
        // targets are V2 statutes in the gazette, so the edges resolve as links.
        if item["estate"] == "v1" {
            item["migration"] = serde_json::json!({
                "superseded_as_live_by": ["ACT-COMPUTER-FIRST-REALM"],
                "restated_in": ["ACT-CONSOLIDATION-FRAMEWORK"],
            });
        }
    }

    let v2_count = items.iter().filter(|i| i["estate"] == "v2").count();
    // THE ARTEFACT NAMES THE TREE IT PUBLISHED ([2026] VJS-CC-VJS 15 C4).
    //
    // `meta.lawpack` used to carry the id, digest and locked_at scraped out of the LOCAL
    // `.vjs/lawpack.lock` and nothing else, which says what this repository has PINNED and
    // is silent on what was actually read. Measured 2026-08-01 on a repository whose
    // lawpack did not resolve: `digest = sha256:5481b9e2...` published beside
    // `counts.total: 0`. Nothing was read, so the digest attested to a provenance that had
    // not happened - a false record, and the kind a reader has no way to spot.
    //
    // So the pin is published only when the publication has something to pin, and the
    // resolution (which source answered, and the directory it named) is published beside
    // it. An empty register now says it is empty rather than wearing a digest.
    let published_something = !items.is_empty();
    let pin = |key: &str| -> Option<&String> {
        if published_something {
            lock_meta.get(key)
        } else {
            None
        }
    };
    let data = serde_json::json!({
        "generated_at": chrono::Utc::now().to_rfc3339(),
        "meta": {
            "lawpack": {
                "id": pin("lawpack"),
                "digest": pin("digest"),
                "locked_at": pin("locked_at"),
                "source": resolution.as_ref().map(|r| r.source),
                // A REPO-RELATIVE directory, or nothing. `r.dir.display()` published the
                // ABSOLUTE checkout path, whose operator-account segment is itself a
                // denylist entry - so the Gazette refused ITSELF, and would have published
                // a private repo path had it not (ACT-005:s1 publish_private_repo_paths).
                // The treatment is the one already reasoned twenty lines away in
                // `gazette/mod.rs`: publish the path the tree occupies in the PUBLISHED
                // estate, never the path it occupies on this disk. A lawpack resolved
                // OUTSIDE the repo (VJS_LAWPACK) has no repo-relative form, so the field
                // is OMITTED rather than guessed ([2026] VJS-CC-VJS 17 C4).
                "path": resolution
                    .as_ref()
                    .and_then(|r| r.dir.strip_prefix(repo).ok())
                    .map(|p| p.display().to_string().replace('\\', "/")),
            },
            "source_commit": source_commit,
            "counts": { "total": items.len(), "canon": v2_count, "archive": items.len() - v2_count },
        },
        "items": items,
    });
    // A `</` inside law text would terminate the host <script> tag; emit the
    // JSON escape `<\/` instead (identical parse, inert markup).
    let guard = |s: String| s.replace("</", "<\\/");
    let out_path = out.unwrap_or_else(|| repo.join("gazette-data.js"));
    // Publication boundary gate (prevention for BREACH-2026-06-10-client-data-
    // published): the Gazette is public record, so scan everything it is about
    // to publish and FAIL CLOSED before writing. (1) RedactScanner catches
    // secrets, tokens, emails, and private-domain references. (2) A hashed
    // denylist (.vjs/publication-denylist.txt) catches private identifiers, e.g.
    // a client name on a carried external-matter artifact - which is private by
    // default. The path that carries archive PDFs and estate text now has the
    // boundary scan the governed writers always had.
    {
        // C5: THE GATE READS WHAT IT LINKS. An item carrying a `source_opinion` publishes its
        // path AND a public blob URL to its body, and the body was never scanned - so the
        // backstop credited with catching a leaked opinion was structurally blind to opinion
        // bodies. Every body reachable from a published item is scanned, whether or not it is
        // rendered; an unreadable one is a REFUSAL and never a skip (C3, on all fours).
        // A `source_opinion` path is written by the CANON'S author and is relative to the
        // repository that OWNS the canon, not to whoever is publishing it. A subscriber
        // resolving the canon out of tree has no such file under its own root, so resolving
        // against `repo` alone refuses every subscriber - which the existing suite caught.
        // The canon's home is the grandparent of the resolved `lawpack/v2`.
        let canon_home = resolution
            .as_ref()
            .and_then(|r| r.dir.parent().and_then(|p| p.parent()))
            .map(|p| p.to_path_buf());
        let no_items = Vec::new();
        let mut linked = String::new();
        for item in data["items"].as_array().unwrap_or(&no_items) {
            let Some(op) = item["opinion"]["path"].as_str() else {
                continue;
            };
            let rel = op.trim_start_matches("./");
            let candidates: Vec<PathBuf> = canon_home
                .iter()
                .map(|h| h.join(rel))
                .chain(std::iter::once(repo.join(rel)))
                .collect();
            let body = candidates
                .iter()
                .find_map(|p| std::fs::read_to_string(p).ok())
                .ok_or_else(|| {
                    KernelError::InvalidInput(format!(
                        "publication boundary: '{}' is published with a link to the \
                         source_opinion at '{rel}', which the gate cannot read from the canon's \
                         own tree or from this repository. The gate scans every body reachable \
                         from a published item, so an unreadable one is a refusal and never a \
                         skip ([2026] VJS-CC-VJS 17 C5).",
                        item["id"].as_str().unwrap_or_default()
                    ))
                })?;
            linked.push('\n');
            linked.push_str(&body);
        }
        let published = format!(
            "{}\n{}\n{}",
            serde_json::to_string(&data).unwrap_or_default(),
            serde_json::to_string(&texts).unwrap_or_default(),
            linked
        );
        // Keep the high-confidence findings (keys, tokens, emails, passwords);
        // drop PrivateHostname, whose word.local/internal/private/lan pattern
        // false-positives on legal prose and config examples (e.g. the Boundary
        // Act's own "private_store.local" illustration).
        let findings: Vec<_> = vjs_redact::RedactScanner::scan_file(&out_path, &published)
            .into_iter()
            .filter(|f| !matches!(f.kind, BoundaryFindingKind::PrivateHostname))
            .collect();
        if !findings.is_empty() {
            let kinds: Vec<String> = findings.iter().map(|f| f.message.clone()).collect();
            return Err(KernelError::InvalidInput(format!(
                "publication boundary: the Gazette would publish private data ({}); fix the record before regenerating",
                kinds.join("; ")
            )));
        }
        // ONE loader and ONE tokeniser, shared with the canon-write gate's C1 limb - so "the
        // canon gate tokenises exactly as the publication gate does" is a property of the
        // CODE, not of a comment. This was `if let Ok(deny) = read_to_string(...)`, which
        // SKIPPED THE WHOLE LIMB when the register could not be read: the gate then published,
        // and said nothing about having not looked ([2026] VJS-CC-VJS 17 C3).
        let deny = vjs_redact::Denylist::load(repo)?;
        if deny.hits_anywhere(&published) {
            return Err(KernelError::InvalidInput(
                "publication boundary: the Gazette would publish a denylisted private term; a carried external-matter artifact is private by default and must be cleared before publication (BREACH-2026-06-10). The term is not named here: naming it would publish it ([2026] VJS-CC-VJS 17 C1/C3)".into(),
            ));
        }
        // THE SEGMENT MEASURE (the residue adoption round, 2026-08-05). A private term
        // surviving as a hyphen-bounded segment inside a longer compound token is
        // invisible to the whole-token limb above by construction - the residue class
        // ACT-SUBSCRIBER-PSEUDONYMITY s2 recites. Its scope is the PUBLISHED BODIES:
        // the composite the Gazette itself writes (the index and the text store),
        // which is what the amendment Acts' closing verification names - so the
        // verification is this code, not a hand-built scan. Linked opinion bodies
        // stay under the whole-token limb above (measured 2026-08-05: all 25 linked
        // bodies at zero on the segment measure); their residue class is routed to
        // the Commission's next certified schedule, and widening this limb to the
        // full composite is one identifier once that schedule cures them.
        let written = format!(
            "{}\n{}",
            serde_json::to_string(&data).unwrap_or_default(),
            serde_json::to_string(&texts).unwrap_or_default()
        );
        if deny.hits_any_segment(&written) {
            return Err(KernelError::InvalidInput(
                "publication boundary: a body the Gazette would write carries a denylisted private term as a segment of a hyphenated compound token, the residue class the whole-token measure cannot see (ACT-SUBSCRIBER-PSEUDONYMITY s2). The term is not named here: naming it would publish it".into(),
            ));
        }
    }

    let body = format!(
        "// Generated by `vjs gazette`. Do not edit: regenerate from the lawpack.\nwindow.GAZETTE = {};\n",
        guard(serde_json::to_string_pretty(&data).expect("gazette data serializes"))
    );
    std::fs::write(&out_path, body).map_err(io)?;

    let text_path = out_path.with_file_name("gazette-text.js");
    let text_body_js = format!(
        "// Generated by `vjs gazette`. Full law text for the in-place reader.\nwindow.GAZETTE_TEXT = {};\n",
        guard(serde_json::to_string(&texts).expect("gazette text serializes"))
    );
    std::fs::write(&text_path, &text_body_js).map_err(io)?;

    // Plain JSON for tooling, beside the JS.
    let json_path = out_path.with_extension("json");
    std::fs::write(
        &json_path,
        serde_json::to_string_pretty(&data).expect("gazette data serializes"),
    )
    .map_err(io)?;

    // The Atom feed: the Gazette as a periodical of record. Deterministic on
    // unchanged law: entry ids are the lawpack's own ids, entry dates come
    // from enactment and amendment history, and the feed's updated is the max
    // entry updated, never the generation time.
    let mut feed_items: Vec<&serde_json::Value> = items.iter().collect();
    feed_items.sort_by(|a, b| {
        b["updated"]
            .as_str()
            .cmp(&a["updated"].as_str())
            .then(a["id"].as_str().cmp(&b["id"].as_str()))
    });
    let feed_updated = feed_items
        .iter()
        .filter_map(|i| i["updated"].as_str())
        .max()
        .unwrap_or("2026-06-09");
    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
    xml.push_str("<feed xmlns=\"http://www.w3.org/2005/Atom\">\n");
    xml.push_str("  <title>The VJS Gazette</title>\n");
    xml.push_str("  <subtitle>The record of the realm: the living canon and the honoured archive</subtitle>\n");
    xml.push_str(&format!("  <id>{}</id>\n", FEED_TAG));
    xml.push_str(&format!(
        "  <updated>{}T00:00:00Z</updated>\n",
        feed_updated
    ));
    xml.push_str(&format!(
        "  <link rel=\"self\" href=\"{}gazette.xml\"/>\n",
        SITE_BASE
    ));
    xml.push_str(&format!(
        "  <link rel=\"alternate\" href=\"{}\"/>\n",
        SITE_BASE
    ));
    xml.push_str("  <author><name>Vibe Justice System</name></author>\n");
    xml.push_str("  <rights>Publication is constitutively inert (REG-GAZETTE-CONTINUITY-001): force comes from the lawpack and the Sovereign's assent, never from publication or syndication.</rights>\n");
    for i in &feed_items {
        let id = i["id"].as_str().unwrap_or_default();
        xml.push_str("  <entry>\n");
        xml.push_str(&format!("    <id>{}:{}</id>\n", FEED_TAG, xml_esc(id)));
        xml.push_str(&format!(
            "    <title>{}</title>\n",
            xml_esc(i["title"].as_str().unwrap_or(id))
        ));
        xml.push_str(&format!(
            "    <link rel=\"alternate\" href=\"{}#{}\"/>\n",
            SITE_BASE,
            xml_esc(id)
        ));
        xml.push_str(&format!(
            "    <link rel=\"via\" href=\"{}\"/>\n",
            xml_esc(i["url"].as_str().unwrap_or(""))
        ));
        xml.push_str(&format!(
            "    <category term=\"{}\"/>\n",
            xml_esc(i["kind"].as_str().unwrap_or(""))
        ));
        xml.push_str(&format!(
            "    <category term=\"{}\"/>\n",
            if i["estate"] == "v1" {
                "archive"
            } else {
                "canon"
            }
        ));
        xml.push_str(&format!(
            "    <published>{}T00:00:00Z</published>\n",
            i["date"].as_str().unwrap_or("2026-06-09")
        ));
        xml.push_str(&format!(
            "    <updated>{}T00:00:00Z</updated>\n",
            i["updated"].as_str().unwrap_or("2026-06-09")
        ));
        xml.push_str(&format!(
            "    <summary type=\"text\">{}</summary>\n",
            xml_esc(i["summary"].as_str().unwrap_or(""))
        ));
        xml.push_str("  </entry>\n");
    }
    xml.push_str("</feed>\n");
    let feed_path = out_path.with_file_name("gazette.xml");
    std::fs::write(&feed_path, xml).map_err(io)?;

    // JSON-LD: each register item as schema.org Legislation, injected
    // idempotently between the marker tags in gazette.html. The
    // legislationLegalForce property is deliberately omitted: asserting
    // force from the publication surface would cut against inertness.
    let gazette_page = out_path.with_file_name("gazette.html");
    if let Ok(html) = std::fs::read_to_string(&gazette_page) {
        const START: &str = "<script type=\"application/ld+json\" id=\"gazette-jsonld\">";
        const END: &str = "</script>";
        if let Some(s_idx) = html.find(START)
            && let Some(e_off) = html[s_idx + START.len()..].find(END)
        {
            let mut graph = vec![serde_json::json!({
                "@type": "Periodical",
                "name": "The VJS Gazette",
                "url": SITE_BASE,
            })];
            for i in &items {
                let id = i["id"].as_str().unwrap_or_default();
                let citation = i["citation"].as_str().unwrap_or_default();
                graph.push(serde_json::json!({
                    "@type": "Legislation",
                    "name": i["title"],
                    "legislationIdentifier": if citation.is_empty() { id } else { citation },
                    "legislationDate": i["date"],
                    "legislationType": i["kind"],
                    "url": format!("{}law.html#{}", SITE_BASE, id),
                    "isPartOf": { "@type": "Periodical", "name": "The VJS Gazette" },
                }));
            }
            let ld = serde_json::json!({ "@context": "https://schema.org", "@graph": graph });
            let body = guard(serde_json::to_string(&ld).expect("jsonld serializes"));
            let new_html = format!(
                "{}{}\n{}\n{}",
                &html[..s_idx],
                START,
                body,
                &html[s_idx + START.len() + e_off..]
            );
            std::fs::write(&gazette_page, new_html).map_err(io)?;
        }
    }

    if json {
        println!(
            "{}",
            serde_json::json!({
                "out": out_path.to_string_lossy(),
                "json_out": json_path.to_string_lossy(),
                "feed_out": feed_path.to_string_lossy(),
                "text_out": text_path.to_string_lossy(),
                "text_bytes": text_body_js.len(),
                "items": known.len(),
                "edges_dropped_to_non_items": dropped,
            })
        );
    } else {
        println!(
            "Gazette data: {} items -> {}",
            known.len(),
            out_path.display()
        );
        println!(
            "  full text: {} bodies ({} KB) -> {}",
            texts.len(),
            text_body_js.len() / 1024,
            text_path.display()
        );
        println!("  citation edges to non-items dropped: {}", dropped);
    }
    Ok(())
}
