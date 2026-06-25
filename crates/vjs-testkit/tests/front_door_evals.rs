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
