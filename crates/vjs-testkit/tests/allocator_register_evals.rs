//! Eval coverage for the citation allocator's read of the live register
//! (INV-AGENT-EVALS-001, for the crates/vjs-mcp change that widened it).
//!
//! The defect being locked out: `live_citation_max` walked `lawpack/v2` alone. That
//! directory holds 86 defining citations and NOT ONE of them is County, so
//! `vjs.allocate` returned 1 for every CC request while the series stood at 8, and the
//! canon PC series offered `[2026] VJS-PC 20` while a live order held it.
//!
//! Measured through the MCP door specifically. The unit test in vjs-lawpack covers
//! `live_citation_max` itself; this covers the OTHER front door, which is the one an
//! agent actually reaches, and which had its own copy of the single-root read.

use std::fs;

fn temp_repo(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("vjs-alloc-eval-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join(".vjs/orders")).unwrap();
    fs::create_dir_all(dir.join(".vjs/court/orders")).unwrap();
    dir
}

fn allocate(repo: &std::path::Path, series: &str, year: i32) -> serde_json::Value {
    let server = vjs_mcp::McpServer::new(repo.to_path_buf());
    let req = serde_json::json!({
        "jsonrpc": "2.0", "id": 1, "method": "vjs.allocate",
        "params": {"series": series, "year": year}
    });
    let raw = server.handle_request(&req.to_string()).expect("allocate");
    let parsed: serde_json::Value = serde_json::from_str(&raw).unwrap();
    parsed["result"].clone()
}

/// The County register lives under `.vjs/orders`, and the allocator must read it even
/// when `lawpack/v2` does not exist at all. A missing register is not evidence that a
/// series is unstarted, and the old code short-circuited that case straight to 0.
#[test]
fn allocate_reads_the_county_register_outside_the_lawpack() {
    let repo = temp_repo("cc");
    fs::write(
        repo.join(".vjs/orders/held.yaml"),
        "id: held\ncitation: '[2026] VJS-CC-VJS 8'\n",
    )
    .unwrap();

    let result = allocate(&repo, "CC", 2026);
    let n = result["n"].as_u64().unwrap();
    fs::remove_dir_all(&repo).ok();

    assert_eq!(n, 9, "allocate must continue the live County series, not restart it");
}

/// The canon series is not exempt. A gate that measured only County would have passed
/// against the PC half of the identical defect, which is exactly what happened: the
/// allocator offered a held `[2026] VJS-PC 20` while the CC symptom was the visible one.
#[test]
fn allocate_reads_the_canon_register_under_court_orders() {
    let repo = temp_repo("pc");
    fs::write(
        repo.join(".vjs/court/orders/held.yaml"),
        "id: held\ncitation: \"[2026] VJS-PC 20\"\n",
    )
    .unwrap();

    let result = allocate(&repo, "PC", 2026);
    let n = result["n"].as_u64().unwrap();
    fs::remove_dir_all(&repo).ok();

    assert_eq!(n, 21, "allocate must not re-offer a citation the court register holds");
}
