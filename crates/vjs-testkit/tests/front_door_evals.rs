//! Eval coverage for the PC-14 single governed-record front door (INV-AGENT-EVALS-001).
//!
//! Two halves of [2026] VJS-PC 14, measured here so harness changes do not rely on
//! review-by-vibes: (D5) the MCP server_of_law surface exposes the governed-record-
//! creation verbs alongside the lifecycle six; (D3) the front-door assent check
//! recognises only the INV-ASSENT-SOURCE-001 allow-list, the predicate the validate
//! floor keys on to route an assented record for correction rather than block it.

use vjs_core::front_door;

#[test]
fn mcp_surface_exposes_the_nine_verbs() {
    let names: Vec<String> = vjs_mcp::get_tool_schemas()
        .into_iter()
        .map(|t| t.name)
        .collect();
    // six lifecycle verbs + the three PC-14 D5 record-creation verbs.
    for v in [
        "vjs.route",
        "vjs.lookup",
        "vjs.validate",
        "vjs.log",
        "vjs.file",
        "vjs.status",
        "vjs.allocate",
        "vjs.convene",
        "vjs.record",
    ] {
        assert!(names.contains(&v.to_string()), "{v} must be exposed");
    }
    assert_eq!(names.len(), 9, "the surface is a closed nine-verb set");
}

#[test]
fn mcp_tool_schemas_survive_the_module_split() {
    // The tool surface moved out of lib.rs into schemas.rs (a behavior-preserving split
    // to keep each file under the structural-cleanliness ceiling). Guard that the move
    // changed no schema: every verb still carries a non-empty object input_schema, and
    // allocate still advertises the repo-segment `repo` property added in audit #10.
    let tools = vjs_mcp::get_tool_schemas();
    for t in &tools {
        assert!(
            t.input_schema.get("type") == Some(&serde_json::json!("object")),
            "{} must keep an object input_schema after the split",
            t.name
        );
    }
    let allocate = tools
        .iter()
        .find(|t| t.name == "vjs.allocate")
        .expect("allocate present");
    assert!(
        allocate
            .input_schema
            .get("properties")
            .and_then(|p| p.get("repo"))
            .is_some(),
        "allocate must keep its repo-segment property after the split"
    );
}

#[test]
fn unknown_mcp_method_is_refused() {
    let srv = vjs_mcp::McpServer::new(std::path::PathBuf::from("."));
    let req = r#"{"jsonrpc":"2.0","id":1,"method":"vjs.exec","params":{}}"#;
    assert!(
        srv.handle_request(req).is_err(),
        "the surface is closed, not an open shell"
    );
}

#[test]
fn assent_floor_predicate_keys_only_on_the_allow_list() {
    // Compliant: the two allow-listed sources protect a record (route-for-correction).
    assert!(front_door::declares_valid_assent(
        "id: x\nassent_source: sovereign_assent\n"
    ));
    assert!(front_door::declares_valid_assent(
        "id: x\nassent_source: standing_bounded_assent\n"
    ));
    // Violation caught: an invented source confers no protection; a commented-out or
    // absent source confers none either, so the limb stays hard on a non-assented
    // off-door record.
    assert!(!front_door::declares_valid_assent(
        "id: x\nassent_source: i_made_this_up\n"
    ));
    assert!(!front_door::declares_valid_assent(
        "id: x\n# assent_source: sovereign_assent\n"
    ));
    assert!(!front_door::declares_valid_assent(
        "id: x\nstatus: binding\n"
    ));
}

// #19 behavioral evals: exercise the record-creation verbs end-to-end through the
// MCP server against the real canon (read-only paths; no record is written).
fn workspace_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn mcp_allocate_reads_the_live_register() {
    let srv = vjs_mcp::McpServer::new(workspace_root());
    let resp = srv
        .handle_request(
            r#"{"jsonrpc":"2.0","id":1,"method":"vjs.allocate","params":{"series":"pc","year":2026}}"#,
        )
        .expect("allocate should succeed");
    // PC-14 is enacted, so the live max is >= 14 and the next is a real PC citation.
    assert!(
        resp.contains("VJS-PC"),
        "allocate returns the next PC citation from the register: {resp}"
    );
}

#[test]
fn mcp_validate_runs_the_engine_pipeline() {
    // The MCP validate verb now calls the same vjs-engine pipeline as the CLI/CI;
    // it returns a Report with an `ok` field over the real canon (which is clean).
    let srv = vjs_mcp::McpServer::new(workspace_root());
    let resp = srv
        .handle_request(r#"{"jsonrpc":"2.0","id":1,"method":"vjs.validate","params":{}}"#)
        .expect("validate should succeed");
    assert!(
        resp.contains("\"ok\""),
        "validate returns an engine Report: {resp}"
    );
}

#[test]
fn mcp_convene_refuses_an_under_strength_bench() {
    let srv = vjs_mcp::McpServer::new(workspace_root());
    // The Privy Council is constituted at 3 ([2026] VJS-SC 2); a bench of 2 is refused
    // at the door, before any submission lookup or write.
    let r = srv.handle_request(
        r#"{"jsonrpc":"2.0","id":1,"method":"vjs.convene","params":{"court":"privy_council","submission":"NONE","bench":["A","B"]}}"#,
    );
    assert!(r.is_err(), "an under-strength privy bench must be refused");
}

#[test]
fn cage_auth_requires_a_matching_token_only_when_configured() {
    use serde_json::json;
    // Dev default: no token configured -> every call is allowed.
    assert!(vjs_mcp::auth_satisfied("", None));
    assert!(vjs_mcp::auth_satisfied("", Some(&json!({}))));
    // Cage mode: a token is configured -> a matching _token is required.
    let p_ok = json!({"_token": "s3cr3t"});
    let p_bad = json!({"_token": "wrong"});
    assert!(vjs_mcp::auth_satisfied("s3cr3t", Some(&p_ok)));
    assert!(!vjs_mcp::auth_satisfied("s3cr3t", Some(&p_bad)));
    assert!(!vjs_mcp::auth_satisfied("s3cr3t", Some(&json!({}))));
    assert!(!vjs_mcp::auth_satisfied("s3cr3t", None));
}

#[test]
fn valid_assent_value_is_the_allow_list_not_merely_non_empty() {
    use vjs_core::front_door::is_valid_assent_value;
    // The two allow-listed values protect a record; a junk non-empty value does not
    // (the bug that let `assent_source: made_it_up` soften a bench defect).
    assert!(is_valid_assent_value("sovereign_assent"));
    assert!(is_valid_assent_value("standing_bounded_assent"));
    assert!(is_valid_assent_value("\"sovereign_assent\""));
    assert!(!is_valid_assent_value("made_it_up"));
    assert!(!is_valid_assent_value(""));
    assert!(!is_valid_assent_value("assent"));
}

#[test]
fn court_string_maps_to_tier() {
    use vjs_core::bench::court_from_str;
    use vjs_core::types::Court;
    assert!(matches!(court_from_str("county"), Some(Court::County)));
    assert!(matches!(
        court_from_str("privy_council"),
        Some(Court::PrivyCouncil)
    ));
    assert!(matches!(
        court_from_str("supreme_court"),
        Some(Court::SupremeCourt)
    ));
    assert!(court_from_str("kangaroo").is_none());
}

#[test]
fn front_door_governs_record_kinds_only() {
    assert!(front_door::is_governed_record(
        "lawpack/v2/orders/2026-VJS-PC-014.yaml"
    ));
    assert!(front_door::is_governed_record(
        "lawpack/v2/decisions/DEC-001.yaml"
    ));
    // Source and docs are not governed records; the limb never reaches them.
    assert!(!front_door::is_governed_record(
        "crates/vjs-core/src/lib.rs"
    ));
    assert!(!front_door::is_governed_record("docs/conformance-map.md"));
}

// audit #10 harness hardening: the three record-creation verbs are measured here so a
// change to crates/vjs-mcp does not rely on review-by-vibes (INV-AGENT-EVALS-001).

#[test]
fn mcp_allocate_segments_cc_citations_by_repo() {
    let srv = vjs_mcp::McpServer::new(workspace_root());
    // The Cc series is bound to a specific repo's code, so it carries a `-<REPO>`
    // segment; a caller allocating a subscriber's Cc line passes `repo` explicitly.
    let acmeco = srv
        .handle_request(
            r#"{"jsonrpc":"2.0","id":1,"method":"vjs.allocate","params":{"series":"CC","repo":"acmeco","year":2026}}"#,
        )
        .expect("allocate cc");
    assert!(
        acmeco.contains("VJS-CC-ACMECO"),
        "a Cc allocation must carry the repo segment: {acmeco}"
    );
    // With no `repo`, the segment defaults to this server's own repo_code (VJS).
    let here = srv
        .handle_request(
            r#"{"jsonrpc":"2.0","id":1,"method":"vjs.allocate","params":{"series":"cc","year":2026}}"#,
        )
        .expect("allocate cc default");
    assert!(
        here.contains("VJS-CC-VJS"),
        "a repo-less Cc allocation defaults to the server's repo_code: {here}"
    );
    // A canon series stays unsegmented.
    let canon = srv
        .handle_request(
            r#"{"jsonrpc":"2.0","id":1,"method":"vjs.allocate","params":{"series":"PC","year":2026}}"#,
        )
        .expect("allocate pc");
    assert!(
        canon.contains("VJS-PC") && !canon.contains("VJS-PC-"),
        "a canon series carries no repo segment: {canon}"
    );
}

#[test]
fn mcp_record_refers_a_subscribers_above_county_order_up() {
    // PC-19 apex routing in the record verb: a subscribing jurisdiction may record only
    // its first-instance County order; an above-County ruling refers up to the apex seat.
    // The commit hook's path scan excludes lawpack/, so this verb is the chokepoint.
    let dir = std::env::temp_dir().join(format!("vjs_tk_apex_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join(".vjs")).unwrap();
    std::fs::write(
        dir.join(".vjs/config.toml"),
        "version = \"2\"\njurisdiction_id = \"acmeco\"\nrepo_code = \"ACMECO\"\nlawpack = \"vjs-v2@0.1.0\"\n\n[paths]\norders = \".vjs/orders\"\nlogs = \".vjs/logs\"\nsubmissions = \".vjs/submissions\"\nspecs = \"lawpack/v2/specs\"\ndecisions = \"lawpack/v2/decisions\"\nproofs = \".vjs/proofs\"\npermits = \".vjs/permits\"\nprivate = \".vjs/private\"\ncache = \".vjs/cache\"\n\n[paths.public]\nenabled = false\n",
    )
    .unwrap();
    let srv = vjs_mcp::McpServer::new(dir.clone());
    let supreme = r#"{"jsonrpc":"2.0","id":1,"method":"vjs.record","params":{"id":"TK-SUP-1","court":"supreme_court","jurisdiction":"acmeco","repo_code":"ACMECO","status":"binding","issue":"tk/apex","holding":"x","directives":[],"forbidden":null,"exceptions":null,"supersedes":[],"source_opinion":null,"runtime_summary":"t","created_at":"2026-06-26"}}"#;
    let r = srv.handle_request(supreme);
    assert!(
        r.is_err() && format!("{r:?}").contains("refers up"),
        "a subscriber's above-County record must refer up: {r:?}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}
