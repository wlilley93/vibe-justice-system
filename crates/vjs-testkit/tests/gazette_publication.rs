//! The publication trust surface: the meta block binds the artifact to the
//! lawpack digest, assent is echoed never minted, the JSON twin matches the
//! JS payload, and the Atom feed is deterministic and complete.

use std::collections::HashSet;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
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
    assert_eq!(meta["counts"]["total"].as_u64().unwrap() as usize, items.len());
}

#[test]
fn assent_is_echoed_from_the_law_never_minted() {
    let d = data();
    const ALLOWED: [&str; 3] =
        ["sovereign_assent", "standing_bounded_assent", "pending_v1_constitutional_route"];
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
                assert!(ALLOWED.contains(&v.as_str()), "{}: '{}' not an allowed form", item["id"], v);
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
    let xml = std::fs::read_to_string(repo_root().join("gazette.xml"))
        .expect("gazette.xml exists (run: vjs gazette)");
    let d = data();
    let items = d["items"].as_array().unwrap();

    assert!(xml.starts_with("<?xml"));
    assert!(xml.contains(r#"xmlns="http://www.w3.org/2005/Atom""#));
    assert_eq!(xml.matches("<entry>").count(), items.len(), "every item is an entry");
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
    assert_eq!(seen.len(), items.len() + 1, "one feed id plus one per entry");

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
    assert!(!xml.contains(&d["generated_at"].as_str().unwrap()[..16]), "no generation timestamp in the feed");
}

#[test]
fn the_pages_are_archival_offline_and_self_hosted() {
    for page in ["index.html", "gazette.html"] {
        let html = std::fs::read_to_string(repo_root().join(page)).unwrap();
        assert!(
            !html.contains("src=\"http") && !html.contains("unpkg"),
            "{}: a record must not load script from a third party",
            page
        );
        assert!(html.contains("<noscript"), "{}: noscript trust statement", page);
        assert!(html.contains("application/atom+xml"), "{}: feed link", page);
        assert!(html.contains("og:image"), "{}: social metadata", page);
    }
    let vendored = repo_root().join("assets/vendor/force-graph.min.js");
    assert!(
        std::fs::metadata(&vendored).map(|m| m.len() > 50_000).unwrap_or(false),
        "vendored force-graph present and non-trivial"
    );
}
