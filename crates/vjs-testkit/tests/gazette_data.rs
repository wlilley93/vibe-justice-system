//! The published Gazette data must stay true to the lawpack: every law object
//! appears as an item, every citation edge points at an item, every V2 source
//! path exists, and both estates are present. If law changes without
//! `vjs gazette` being re-run, this fails and says so.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn gazette_items() -> Vec<serde_json::Value> {
    let raw = std::fs::read_to_string(repo_root().join("gazette-data.js"))
        .expect("gazette-data.js exists at the repo root (run: vjs gazette)");
    let start = raw.find('{').unwrap();
    let end = raw.rfind('}').unwrap();
    let data: serde_json::Value = serde_json::from_str(&raw[start..=end]).expect("valid JSON");
    data["items"].as_array().expect("items array").clone()
}

#[test]
fn every_law_object_is_published_and_every_edge_resolves() {
    let items = gazette_items();
    let ids: HashSet<String> = items
        .iter()
        .map(|i| i["id"].as_str().unwrap().to_string())
        .collect();
    assert_eq!(ids.len(), items.len(), "item ids must be unique");

    // Both estates present.
    for estate in ["v1", "v2"] {
        assert!(
            items.iter().any(|i| i["estate"] == estate),
            "the one Gazette carries two estates; '{}' is missing",
            estate
        );
    }

    // Every lawpack object is published: legislation and case law as register
    // items; the kernel machinery as schedules read within the Realm
    // Invariants Instrument (REG-REALM-INVARIANTS-001, [2026] VJS-PC 7).
    // Nothing is curated out of the record either way.
    let text_raw = std::fs::read_to_string(repo_root().join("gazette-text.js")).unwrap();
    let tstart = text_raw.find('{').unwrap();
    let tend = text_raw.rfind('}').unwrap();
    let texts: serde_json::Value =
        serde_json::from_str(&text_raw[tstart..=tend].replace("<\\/", "</")).unwrap();
    let scheduled: HashSet<String> = texts["REG-REALM-INVARIANTS-001"]["schedules"]
        .as_array()
        .map(|a| a.iter().map(|s| s["id"].as_str().unwrap().to_string()).collect())
        .unwrap_or_default();

    let lawpack = repo_root().join("lawpack/v2");
    for (dir, machinery) in [
        ("statutes", false), ("regulations", false), ("rules", true), ("orders", false),
        ("specs", true), ("invariants", true), ("decisions", false), ("obligations", true),
    ] {
        let d = lawpack.join(dir);
        if !d.exists() {
            continue;
        }
        for entry in std::fs::read_dir(&d).unwrap().filter_map(|e| e.ok()) {
            let p = entry.path();
            if p.extension().and_then(|x| x.to_str()) != Some("yaml") {
                continue;
            }
            let content = std::fs::read_to_string(&p).unwrap();
            let id = content
                .lines()
                .find_map(|l| l.strip_prefix("id: "))
                .map(|s| s.trim().trim_matches('"').to_string())
                .unwrap_or_default();
            if machinery {
                assert!(
                    scheduled.contains(&id),
                    "machinery '{}' ({}) is not carried in the instrument's schedules - rerun vjs gazette",
                    id,
                    p.display()
                );
                assert!(!ids.contains(&id), "machinery '{}' must not be a separate register item", id);
            } else {
                assert!(
                    ids.contains(&id),
                    "law object '{}' ({}) is not published in gazette-data.js - rerun vjs gazette",
                    id,
                    p.display()
                );
            }
        }
    }

    // Degree over citations plus derived authority lineage: the constellation
    // may never show a star floating free of the constitutional centre.
    let mut degree: std::collections::HashMap<String, usize> = HashMap::new();
    for item in &items {
        // No edge to a non-item: citation, lineage, or docket thread.
        for key in ["cites", "lineage", "thread"] {
            for edge in item[key].as_array().map(|a| a.as_slice()).unwrap_or(&[]) {
                assert!(
                    ids.contains(edge.as_str().unwrap()),
                    "item '{}' has {} edge to non-item '{}'",
                    item["id"],
                    key,
                    edge
                );
                *degree.entry(item["id"].as_str().unwrap().to_string()).or_default() += 1;
                *degree.entry(edge.as_str().unwrap().to_string()).or_default() += 1;
            }
        }
        // Every V2 source path exists; every URL is branch-correct (the public
        // repo has master and v1 branches; /blob/main/ 404s).
        let url = item["url"].as_str().unwrap();
        if item["estate"] == "v2" {
            let path = item["path"].as_str().unwrap();
            assert!(
                repo_root().join(path).exists(),
                "item '{}' points at missing path {}",
                item["id"],
                path
            );
            assert!(url.contains("/blob/master/"), "v2 url on wrong branch: {}", url);
        } else {
            assert!(url.contains("/blob/v1/"), "v1 url on wrong branch: {}", url);
        }
        assert!(!url.contains("/blob/main/"), "the remote has no main branch: {}", url);

        // The reading surface is real: a title, a summary, a date, a source.
        assert!(!item["title"].as_str().unwrap().is_empty());
        assert!(!item["summary"].as_str().unwrap().is_empty());
        assert!(
            !item["date"].as_str().unwrap_or_default().is_empty(),
            "item '{}' has no date; the Gazette orders newest first",
            item["id"]
        );
    }

    for item in &items {
        assert!(
            degree.get(item["id"].as_str().unwrap()).copied().unwrap_or(0) > 0,
            "item '{}' is an orphan star: no citation or lineage edge touches it",
            item["id"]
        );
    }
}
