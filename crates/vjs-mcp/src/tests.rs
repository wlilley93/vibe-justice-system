//! Unit coverage for the MCP server. Split out of lib.rs (behavior-preserving)
//! to keep each file under the structural-cleanliness ceiling. Included as a
//! `#[cfg(test)] mod tests;`, so `super` here is the crate root.

use super::*;

/// PC-14 D5: the front-door gap is closed - the surface now exposes the
/// governed-record-creation verbs (allocate, convene, record) alongside the
/// lifecycle six, nine in all.
#[test]
fn surface_exposes_the_record_creation_verbs() {
    let names: Vec<String> = get_tool_schemas().into_iter().map(|t| t.name).collect();
    assert_eq!(
        names.len(),
        9,
        "six lifecycle + three record-creation verbs"
    );
    for v in ["vjs.allocate", "vjs.convene", "vjs.record"] {
        assert!(names.contains(&v.to_string()), "{v} must be exposed");
    }
}

/// An unknown method is refused - the surface is a closed set, not an open shell.
#[test]
fn unknown_method_is_refused() {
    let srv = McpServer::new(std::path::PathBuf::from("."));
    let req = r#"{"jsonrpc":"2.0","id":1,"method":"vjs.exec","params":{}}"#;
    assert!(srv.handle_request(req).is_err());
}

fn scratch_dir(tag: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("vjs_mcp_{tag}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

/// allocate: the Cc series is repo-scoped and must carry the `-<REPO>` segment, the
/// canon series must NOT. A caller-supplied `repo` selects a subscriber's Cc line;
/// absent it, the segment defaults to this server's own repo_code. (Audit #10: the
/// verb was minting segment-less Cc citations that collide across repos.)
#[test]
fn allocate_scopes_cc_to_a_repo_segment_and_leaves_canon_unsegmented() {
    let dir = scratch_dir("alloc"); // no config.toml -> repo_code defaults to VJS
    let srv = McpServer::new(dir.clone());

    let cc_named = srv
        .handle_allocate(Some(
            serde_json::json!({"series":"CC","year":2026,"repo":"opbox"}),
        ))
        .unwrap();
    assert_eq!(cc_named["citation"], "[2026] VJS-CC-OPBOX 1");

    let cc_default = srv
        .handle_allocate(Some(serde_json::json!({"series":"cc","year":2026})))
        .unwrap();
    assert_eq!(cc_default["citation"], "[2026] VJS-CC-VJS 1");

    let canon = srv
        .handle_allocate(Some(serde_json::json!({"series":"PC","year":2026})))
        .unwrap();
    assert_eq!(canon["citation"], "[2026] VJS-PC 1");

    let _ = std::fs::remove_dir_all(&dir);
}

/// record: PC-19 apex routing in the typed verb. A subscribing jurisdiction (here
/// "opbox") may record only its first-instance County order; an above-County ruling
/// refers up to the apex seat. The commit hook's path scan excludes lawpack/, so this
/// verb is the chokepoint. The refusal fires BEFORE the lawpack is loaded, so the
/// fixture needs only the config. (Audit #10: the verb had no apex check at all.)
#[test]
fn record_refuses_an_above_county_order_from_a_subscriber() {
    let dir = scratch_dir("apex");
    std::fs::create_dir_all(dir.join(".vjs")).unwrap();
    std::fs::write(
            dir.join(".vjs/config.toml"),
            "version = \"2\"\njurisdiction_id = \"opbox\"\nrepo_code = \"OPBOX\"\nlawpack = \"vjs-v2@0.1.0\"\n\n[paths]\norders = \".vjs/orders\"\nlogs = \".vjs/logs\"\nsubmissions = \".vjs/submissions\"\nspecs = \"lawpack/v2/specs\"\ndecisions = \"lawpack/v2/decisions\"\nproofs = \".vjs/proofs\"\npermits = \".vjs/permits\"\nprivate = \".vjs/private\"\ncache = \".vjs/cache\"\n\n[paths.public]\nenabled = false\n",
        )
        .unwrap();
    let srv = McpServer::new(dir.clone());

    let order = serde_json::json!({
        "id": "TEST-SUPREME-1",
        "court": "supreme_court",
        "jurisdiction": "opbox",
        "repo_code": "OPBOX",
        "status": "binding",
        "issue": "test/apex",
        "holding": "a subscriber tries to record an apex order",
        "directives": [],
        "forbidden": null,
        "exceptions": null,
        "supersedes": [],
        "source_opinion": null,
        "runtime_summary": "test",
        "created_at": "2026-06-26"
    });
    let err = srv.handle_record(Some(order)).unwrap_err();
    let msg = format!("{err:?}");
    assert!(
        msg.contains("refers up") && msg.contains("VJS-PC 19"),
        "subscriber apex record must refer up, got: {msg}"
    );

    // And a County order from the same subscriber is NOT refused by the apex gate
    // (it falls through to the ordinary bench/lawpack path - a different failure, or
    // success, but never the apex refusal).
    let county = serde_json::json!({
        "id": "TEST-COUNTY-1", "court": "county", "jurisdiction": "opbox",
        "repo_code": "OPBOX", "status": "binding", "issue": "test/county",
        "holding": "first-instance", "directives": [], "forbidden": null,
        "exceptions": null, "supersedes": [], "source_opinion": null,
        "runtime_summary": "test", "created_at": "2026-06-26"
    });
    let county_res = srv.handle_record(Some(county));
    if let Err(e) = county_res {
        assert!(
            !format!("{e:?}").contains("refers up"),
            "a County order must never trip the apex refusal"
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}
