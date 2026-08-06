//! The publication trust surface: the meta block binds the artifact to the
//! lawpack digest, assent is echoed never minted, the JSON twin matches the
//! JS payload, and the Atom feed is deterministic and complete.

use std::collections::HashSet;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    // The PUBLISHING estate's root, FOUND by walking up rather than counting levels:
    // in a vendored tree the crates sit one level deeper than the law.
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        if d.join("lawpack/v2/manifest.toml").is_file() {
            return d;
        }
        assert!(
            d.pop(),
            "no lawpack/v2 above CARGO_MANIFEST_DIR: these tests need one"
        );
    }
}

/// `None` means THIS ESTATE HAS NEVER PUBLISHED A GAZETTE (a subscriber is born
/// unpublished; `vjs invoke` publishes nothing), disclosed on stderr - a statement
/// about this estate, never about the corpus. In the publishing estate the artefacts
/// exist and every assertion below bites.
fn published_estate() -> Option<PathBuf> {
    let root = repo_root();
    if root.join("gazette-data.js").is_file() {
        Some(root)
    } else {
        eprintln!(
            "SKIP: {} carries no published Gazette artefacts. This is a statement \
             about this estate, never about the corpus.",
            root.display()
        );
        None
    }
}

fn data() -> serde_json::Value {
    serde_json::from_str(
        &std::fs::read_to_string(repo_root().join("gazette-data.json"))
            .expect("gazette-data.json exists (run: vjs gazette)"),
    )
    .expect("valid JSON")
}

#[test]
fn the_meta_block_binds_the_publication_to_the_record() {
    let Some(_) = published_estate() else {
        return;
    };
    let d = data();
    let meta = &d["meta"];

    // The digest equals the lock file's, parsed the same lenient way the
    // generator parses it (the Store struct's keys do not match the file).
    let lock = std::fs::read_to_string(repo_root().join(".vjs/lawpack.lock")).unwrap();
    let lock_digest = lock
        .lines()
        .find_map(|l| l.strip_prefix("digest = "))
        .map(|v| v.trim().trim_matches('"').to_string())
        .expect("lock carries a digest");
    assert_eq!(meta["lawpack"]["digest"].as_str().unwrap(), lock_digest);

    let commit = meta["source_commit"].as_str().unwrap();
    assert!(
        commit.is_empty() || (commit.len() == 40 && commit.chars().all(|c| c.is_ascii_hexdigit())),
        "source_commit is a full sha or empty"
    );

    let items = d["items"].as_array().unwrap();
    assert_eq!(
        meta["counts"]["total"].as_u64().unwrap() as usize,
        items.len()
    );
}

#[test]
fn assent_is_echoed_from_the_law_never_minted() {
    let Some(_) = published_estate() else {
        return;
    };
    let d = data();
    const ALLOWED: [&str; 3] = [
        "sovereign_assent",
        "standing_bounded_assent",
        "pending_v1_constitutional_route",
    ];
    for item in d["items"].as_array().unwrap() {
        if item["estate"] != "v2" {
            continue;
        }
        let path = repo_root().join(item["path"].as_str().unwrap());
        let yaml = std::fs::read_to_string(&path).unwrap();
        let in_law = yaml
            .lines()
            .find_map(|l| l.strip_prefix("assent_source:"))
            .map(|v| v.trim().trim_matches('"').to_string());
        match in_law {
            Some(v) => {
                assert_eq!(
                    item["assent_source"].as_str(),
                    Some(v.as_str()),
                    "{}: published assent must equal the law's",
                    item["id"]
                );
                assert!(
                    ALLOWED.contains(&v.as_str()),
                    "{}: '{}' not an allowed form",
                    item["id"],
                    v
                );
            }
            None => assert!(
                item.get("assent_source").is_none(),
                "{}: the Gazette must not mint assent the law does not declare",
                item["id"]
            ),
        }
    }
}

#[test]
fn the_json_twin_matches_the_js_payload() {
    let Some(_) = published_estate() else {
        return;
    };
    let d = data();
    let raw = std::fs::read_to_string(repo_root().join("gazette-data.js")).unwrap();
    let start = raw.find('{').unwrap();
    let end = raw.rfind('}').unwrap();
    let js: serde_json::Value =
        serde_json::from_str(&raw[start..=end].replace("<\\/", "</")).unwrap();
    let ids = |v: &serde_json::Value| -> HashSet<String> {
        v["items"]
            .as_array()
            .unwrap()
            .iter()
            .map(|i| i["id"].as_str().unwrap().to_string())
            .collect()
    };
    assert_eq!(ids(&d), ids(&js));
}

#[test]
fn the_atom_feed_is_complete_inert_and_dateworthy() {
    let Some(_) = published_estate() else {
        return;
    };
    let xml = std::fs::read_to_string(repo_root().join("gazette.xml"))
        .expect("gazette.xml exists (run: vjs gazette)");
    let d = data();
    let items = d["items"].as_array().unwrap();

    assert!(xml.starts_with("<?xml"));
    assert!(xml.contains(r#"xmlns="http://www.w3.org/2005/Atom""#));
    assert_eq!(
        xml.matches("<entry>").count(),
        items.len(),
        "every item is an entry"
    );
    assert!(
        xml.contains("REG-GAZETTE-CONTINUITY-001"),
        "the inertness clause rides the feed itself"
    );

    // Entry ids are unique and tagged.
    let mut seen = HashSet::new();
    for line in xml.lines() {
        if let Some(id) = line.trim().strip_prefix("<id>tag:") {
            assert!(seen.insert(id.to_string()), "duplicate feed id: {}", id);
        }
    }
    assert_eq!(
        seen.len(),
        items.len() + 1,
        "one feed id plus one per entry"
    );

    // The feed's updated is the max entry updated (regeneration on unchanged
    // law is byte-identical; generation time never leaks into the feed).
    let max_updated = items
        .iter()
        .filter_map(|i| i["updated"].as_str())
        .max()
        .unwrap();
    let feed_updated = xml
        .lines()
        .find_map(|l| l.trim().strip_prefix("<updated>"))
        .unwrap()
        .trim_end_matches("</updated>");
    assert_eq!(feed_updated, format!("{}T00:00:00Z", max_updated));
    assert!(
        !xml.contains(&d["generated_at"].as_str().unwrap()[..16]),
        "no generation timestamp in the feed"
    );
}

#[test]
fn the_pages_are_archival_offline_and_self_hosted() {
    let Some(_) = published_estate() else {
        return;
    };
    for page in ["index.html", "gazette.html", "law.html"] {
        let html = std::fs::read_to_string(repo_root().join(page)).unwrap();
        assert!(
            !html.contains("src=\"http") && !html.contains("unpkg"),
            "{}: a record must not load script from a third party",
            page
        );
        assert!(
            html.contains("<noscript"),
            "{}: noscript trust statement",
            page
        );
        assert!(html.contains("application/atom+xml"), "{}: feed link", page);
        assert!(html.contains("og:image"), "{}: social metadata", page);
    }
    let vendored = repo_root().join("assets/vendor/force-graph.min.js");
    assert!(
        std::fs::metadata(&vendored)
            .map(|m| m.len() > 50_000)
            .unwrap_or(false),
        "vendored force-graph present and non-trivial"
    );
}

#[test]
fn court_orders_render_as_text_and_the_machine_record() {
    let Some(_) = published_estate() else {
        return;
    };
    // The Principal's direction (DEC-007 era): court orders render as native
    // cream HTML from their committed text - holding, directives, forbidden and
    // the plain-language 'In plain terms' summary - with the machine YAML
    // standing alongside. The white embedded PDFs were retired.
    let d = data();
    for item in d["items"].as_array().unwrap() {
        if item["kind"] != "order" {
            continue;
        }
        let id = item["id"].as_str().unwrap();
        // no embedded PDF any more - the order is native HTML
        assert!(
            item["pdf"].as_str().unwrap_or_default().is_empty(),
            "{}: order no longer carries an embedded PDF",
            id
        );
        // it renders from its committed text body
        assert_eq!(
            item["has_text"],
            serde_json::json!(true),
            "{}: order has renderable text",
            id
        );
        // the machine YAML the page links as secondary is a real committed file
        let path = item["path"].as_str().unwrap_or_default();
        assert!(
            repo_root().join(path).exists(),
            "{}: the machine record exists at {}",
            id,
            path
        );
    }
}

#[test]
fn the_workplan_order_terms_hold_on_the_register() {
    let Some(_) = published_estate() else {
        return;
    };
    // One GitHub link per page: the header logo, nothing else
    // ([2026] VJS-CC-AGENT-UNIVERSE-V2 13, forbidden list).
    for page in ["index.html", "gazette.html", "law.html"] {
        let html = std::fs::read_to_string(repo_root().join(page)).unwrap();
        assert_eq!(
            html.matches("github.com").count(),
            1,
            "{}: exactly one GitHub reference, the header logo",
            page
        );
    }

    // Every item opens as a document; the whole archive reads as PDFs the
    // Gazette itself serves (D1, D2).
    let d = data();
    for item in d["items"].as_array().unwrap() {
        let doc = item["doc"].as_str().unwrap_or_default();
        assert_eq!(
            doc,
            format!("law.html#{}", item["id"].as_str().unwrap()),
            "{}: the document URL routes to the item's own register entry",
            item["id"]
        );
        if item["estate"] == "v1" {
            let pdf = item["pdf"].as_str().unwrap_or_default();
            let webpage = item["archive_text"] == true;
            // every archive record loads as its native PDF, or (where none was
            // ever rendered in V1) as a webpage from its frozen source
            assert!(
                !pdf.is_empty() || webpage,
                "{}: archive items load a PDF or a source webpage",
                item["id"]
            );
            if !pdf.is_empty() {
                assert!(
                    repo_root().join(pdf).exists(),
                    "{}: carried PDF exists at {}",
                    item["id"],
                    pdf
                );
                assert!(
                    !pdf.starts_with("http"),
                    "{}: the Gazette serves its own PDFs",
                    item["id"]
                );
            }
        }
    }
}

#[test]
fn the_jsonld_graph_mirrors_the_register() {
    let Some(_) = published_estate() else {
        return;
    };
    let html = std::fs::read_to_string(repo_root().join("gazette.html")).unwrap();
    let start_tag = r#"<script type="application/ld+json" id="gazette-jsonld">"#;
    let s = html.find(start_tag).expect("jsonld marker present") + start_tag.len();
    let e = s + html[s..].find("</script>").expect("jsonld closes");
    let ld: serde_json::Value =
        serde_json::from_str(&html[s..e].replace("<\\/", "</")).expect("jsonld parses");
    let graph = ld["@graph"].as_array().unwrap();
    let items = data()["items"].as_array().unwrap().len();
    assert_eq!(
        graph.len(),
        items + 1,
        "one Periodical plus one Legislation per item"
    );
    assert!(graph.iter().skip(1).all(|n| n["@type"] == "Legislation"));
    assert!(
        !html[s..e].contains("legislationLegalForce"),
        "force is never asserted from the publication surface"
    );
}
