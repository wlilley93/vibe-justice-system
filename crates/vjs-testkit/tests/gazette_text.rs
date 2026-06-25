//! The full-text artifact behind the in-place reader: every canon item has a
//! body, no body exists without an item, the shapes are renderable, and no
//! `</script` can terminate the host tag.

use std::collections::HashSet;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn parse_js(name: &str) -> serde_json::Value {
    let raw = std::fs::read_to_string(repo_root().join(name))
        .unwrap_or_else(|_| panic!("{} exists at the repo root (run: vjs gazette)", name));
    assert!(
        !raw.to_lowercase().contains("</script"),
        "{} must never contain a script terminator",
        name
    );
    let start = raw.find('{').unwrap();
    let end = raw.rfind('}').unwrap();
    serde_json::from_str(&raw[start..=end].replace("<\\/", "</")).expect("valid JSON payload")
}

#[test]
fn the_text_artifact_is_bijective_with_the_canon_and_renderable() {
    let data = parse_js("gazette-data.js");
    let texts = parse_js("gazette-text.js");
    let bodies = texts.as_object().unwrap();

    // bodies serve the canon (has_text) and the archive webpages (archive_text:
    // records with no native V1 PDF). No body without an item, either way.
    let text_ids: HashSet<&str> = data["items"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|i| i["has_text"] == true || i["archive_text"] == true)
        .map(|i| i["id"].as_str().unwrap())
        .collect();
    let mut body_ids: HashSet<&str> = bodies.keys().map(|k| k.as_str()).collect();
    // the consolidating instrument's schedules ride inside its own body, not
    // as separate text entries; that body is the instrument item itself
    body_ids.insert("REG-REALM-INVARIANTS-001");
    assert_eq!(
        text_ids, body_ids,
        "every has_text/archive_text item has a body and no body lacks an item - rerun vjs gazette"
    );

    // has_text marks exactly the canon (the archive's text lives on the v1 branch).
    for item in data["items"].as_array().unwrap() {
        assert_eq!(
            item["has_text"] == true,
            item["estate"] == "v2",
            "has_text must mark exactly the canon: {}",
            item["id"]
        );
    }

    // Kind-shape checks: the reader renders these fields.
    for item in data["items"].as_array().unwrap() {
        let id = item["id"].as_str().unwrap();
        if item["estate"] != "v2" {
            continue;
        }
        let body = &bodies[id];
        let nonempty = |key: &str| {
            assert!(
                body[key]
                    .as_str()
                    .map(|s| !s.trim().is_empty())
                    .unwrap_or(false),
                "{} body field '{}' must be non-empty text",
                id,
                key
            )
        };
        match item["kind"].as_str().unwrap() {
            "statute" => {
                let secs = body["sections"].as_array().unwrap();
                assert!(!secs.is_empty(), "{} has sections", id);
                // the settled format: contiguous ordinals from s1, absent
                // ones present as Reserved placeholders, enacted ones with text
                for (i, sec) in secs.iter().enumerate() {
                    let sid = sec["id"].as_str().unwrap();
                    assert_eq!(
                        sid,
                        format!("{}:s{}", id, i + 1),
                        "{}: sections number contiguously from s1",
                        id
                    );
                    if sec["reserved"] == true {
                        assert_eq!(sec["title"], "Reserved");
                    } else {
                        assert!(
                            !sec["text"].as_str().unwrap_or("").trim().is_empty(),
                            "{}: enacted section {} carries text",
                            id,
                            sid
                        );
                    }
                }
                assert!(
                    secs.last().unwrap()["reserved"] != true,
                    "{}: the final section is enacted, never Reserved",
                    id
                );
            }
            "regulation" | "obligation" => nonempty("text"),
            "order" => {
                nonempty("holding");
                nonempty("runtime_summary");
            }
            "decision" => nonempty("decision"),
            "invariant" => {
                nonempty("remedy");
                assert!(body["rule"].is_object(), "{} rule tree", id);
            }
            "spec" => nonempty("purpose"),
            "rule" => nonempty("summary"),
            _ => {}
        }
    }

    // Size budget: a whole-YAML dump would blow past this.
    let bytes = std::fs::metadata(repo_root().join("gazette-text.js"))
        .unwrap()
        .len();
    // Soft guard against a whole-YAML dump (which would be megabytes). Raised from
    // 400 KB as the realm grew (the PC-15/16/17 and SC-5 orders each carry substantial
    // holdings); 600 KB still enforces the "summaries, not full YAML" discipline.
    assert!(
        bytes < 600_000,
        "gazette-text.js stays under 600 KB, got {}",
        bytes
    );
}

#[test]
fn treatment_fields_resolve_and_reciprocate() {
    let data = parse_js("gazette-data.js");
    let items = data["items"].as_array().unwrap();
    let ids: HashSet<&str> = items.iter().map(|i| i["id"].as_str().unwrap()).collect();

    for item in items {
        let id = item["id"].as_str().unwrap();
        for key in ["supersedes", "superseded_by", "thread"] {
            if let Some(arr) = item[key].as_array() {
                for t in arr {
                    assert!(
                        ids.contains(t.as_str().unwrap()),
                        "{} {} -> non-item",
                        id,
                        key
                    );
                }
            }
        }
        // supersession reciprocity
        if let Some(sup) = item["supersedes"].as_array() {
            for t in sup {
                let target = items.iter().find(|x| x["id"] == *t).unwrap();
                assert!(
                    target["superseded_by"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|b| b.as_str() == Some(id)),
                    "{} supersedes {} but the reverse edge is missing",
                    id,
                    t
                );
            }
        }
        // opinions point at real committed files
        if item["opinion"].is_object() {
            let p = item["opinion"]["path"].as_str().unwrap();
            assert!(repo_root().join(p).exists(), "opinion path missing: {}", p);
            assert!(
                item["opinion"]["url"]
                    .as_str()
                    .unwrap()
                    .contains("/blob/master/")
            );
        }
    }
}
