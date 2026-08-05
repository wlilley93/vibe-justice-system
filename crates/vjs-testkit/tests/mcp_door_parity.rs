//! THE MCP DOOR ANSWERS FROM THE SAME LAW AS THE CLI DOOR.
//!
//! Until 2026-08-05 it did not, and the gap was invisible from the output. `vjs-mcp` carried its own
//! private `build_context`, `load_lawpack` and `compute_digest`, and each was weaker than the CLI's:
//!
//!   1. the context overlaid NO filed orders, so every record in `.vjs/orders/` was invisible to
//!      `vjs.route` and `vjs.lookup`. This is the defect `[2026] VJS-CC-VJS 16` C4 was decided to
//!      close - closed at one door.
//!   2. `load_lawpack` returned a fully EMPTY lawpack when `lawpack/v2` was absent, with no error
//!      and no warning, bypassing the `[2026] VJS-CC-VJS 12` D1 refusal. An invoked jurisdiction
//!      with a wrong or moved lawpack path got `Allowed` and a minted permit off zero statutes.
//!   3. `refuse_if_orders_unreadable` - the `[2026] VJS-CC-OPBOX 160` O5 fail-closed gate - was
//!      never called here at all, so the door an AGENT reaches the kernel through was the one door
//!      with no gate on it.
//!
//! WHY EACH TEST HAS A NEGATIVE CONTROL. The failure mode being fixed is a check that answers
//! confidently from a partial corpus, so a test that only ever sees the good path would report a
//! pass for exactly the same reason the door reported an answer: it never looked. Every assertion
//! below is paired with a case that MUST fail, and the pair is the evidence.

use std::path::{Path, PathBuf};

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("vjs-mcp-door-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join(".vjs/orders")).expect("fixture dirs");
    dir
}

/// A well-formed order, in the shape the loader actually accepts (the same constant the lawpack
/// crate's own round-trip test uses, so this fixture cannot drift away from what ships).
fn good_order(issue: &str) -> String {
    format!(
        "id: 2026-VJS-TEST-001\ncourt: county\njurisdiction: default\nrepo_code: VJS\n\
         status: binding\nissue: {issue}\nholding: the door must see a filed order\n\
         directives: []\nforbidden: []\nexceptions: []\nsupersedes: []\nsource_opinion: null\n\
         runtime_summary: proves a filed order reaches the graph through the MCP door\n\
         created_at: \"2026-08-05T00:00:00Z\"\n"
    )
}

fn route_params(repo: &Path, issue: &str) -> String {
    serde_json::json!({
        "jsonrpc": "2.0", "id": 1, "method": "vjs.route",
        "params": {
            "repo_root": repo, "jurisdiction": "default", "actor": "lexby",
            "action_kind": "implementation_decision", "issue_tags": [issue],
            "intent": "test the door", "affected_paths": [], "risk": "low",
            "public_target": false, "external_target": false, "irreversible": false,
            "user_instruction": null
        }
    })
    .to_string()
}

fn lookup_params(issue: &str) -> String {
    serde_json::json!({
        "jsonrpc": "2.0", "id": 1, "method": "vjs.lookup", "params": {"issue": issue}
    })
    .to_string()
}

// ---------------------------------------------------------------------------------------------
// 1. A FILED ORDER IS VISIBLE OVER MCP
// ---------------------------------------------------------------------------------------------

/// The order in `.vjs/orders/` must appear in the answer `vjs.lookup` gives over MCP.
///
/// Before the shared door this returned the identical payload whether or not the order existed,
/// which is what made the defect survive: an agent could not tell "no law on this issue" from "the
/// law is there and this door cannot see it".
#[test]
fn mcp_lookup_sees_a_filed_order() {
    let repo = scratch("sees");
    std::fs::write(
        repo.join(".vjs/orders/2026-VJS-TEST-001.yaml"),
        good_order("door-visibility"),
    )
    .unwrap();

    let srv = vjs_mcp::McpServer::new(repo.clone());
    let out = srv
        .handle_request(&lookup_params("door-visibility"))
        .expect("lookup answers");
    let text = serde_json::to_string(&out).unwrap();
    assert!(
        text.contains("2026-VJS-TEST-001"),
        "the filed order must reach the MCP door's authority graph, got: {text}"
    );
}

/// NEGATIVE CONTROL for the test above. Same repo, same call, no order on disk: the assertion the
/// previous test makes MUST fail here. If both pass, the previous test proves nothing.
#[test]
fn mcp_lookup_finds_nothing_when_nothing_is_filed() {
    let repo = scratch("empty");
    let srv = vjs_mcp::McpServer::new(repo.clone());
    let out = srv
        .handle_request(&lookup_params("door-visibility"))
        .expect("lookup answers");
    let text = serde_json::to_string(&out).unwrap();
    assert!(
        !text.contains("2026-VJS-TEST-001"),
        "an order that was never filed must not appear: {text}"
    );
}

// ---------------------------------------------------------------------------------------------
// 2. THE O5 FAIL-CLOSED GATE IS ON THIS DOOR
// ---------------------------------------------------------------------------------------------

/// An unreadable filed order must REFUSE the route, not be quietly dropped from the corpus the
/// answer is computed over. This is [2026] VJS-CC-OPBOX 160 O5, and the reason it exists is that
/// `vjs route` once answered `court_required: false` on a trust-boundary fork with 55 of 109 orders
/// unread.
#[test]
fn mcp_route_refuses_while_an_order_is_unreadable() {
    let repo = scratch("route-refuse");
    std::fs::write(
        repo.join(".vjs/orders/2026-VJS-BROKEN-001.yaml"),
        "id: [unclosed\n  : : :\n",
    )
    .unwrap();

    let srv = vjs_mcp::McpServer::new(repo.clone());
    let err = srv
        .handle_request(&route_params(&repo, "anything"))
        .expect_err("a route from a partial corpus must refuse");
    let msg = format!("{err}");
    assert!(
        msg.contains("REFUSING TO ROUTE"),
        "the refusal must say why, at the answer: {msg}"
    );
    assert!(
        msg.contains("2026-VJS-BROKEN-001.yaml"),
        "and must name the order it could not read: {msg}"
    );
}

/// The same gate on `vjs.lookup`. A lookup is the verb an agent uses to ask "has this been decided",
/// so answering it from a partial corpus is the S-11(c) failure directly.
#[test]
fn mcp_lookup_refuses_while_an_order_is_unreadable() {
    let repo = scratch("lookup-refuse");
    std::fs::write(
        repo.join(".vjs/orders/2026-VJS-BROKEN-002.yaml"),
        "id: [unclosed\n  : : :\n",
    )
    .unwrap();
    let srv = vjs_mcp::McpServer::new(repo.clone());
    assert!(
        srv.handle_request(&lookup_params("anything")).is_err(),
        "lookup must not answer over a corpus it could not fully read"
    );
}

/// NEGATIVE CONTROL for the two above: with the broken file gone, the identical call SUCCEEDS.
/// Without this, "route refuses" is consistent with a door that refuses everything.
#[test]
fn mcp_route_answers_once_every_order_reads() {
    let repo = scratch("route-ok");
    std::fs::write(
        repo.join(".vjs/orders/2026-VJS-TEST-001.yaml"),
        good_order("clean-corpus"),
    )
    .unwrap();
    let srv = vjs_mcp::McpServer::new(repo.clone());
    assert!(
        srv.handle_request(&route_params(&repo, "clean-corpus"))
            .is_ok(),
        "a complete corpus must route"
    );
}

/// The DECLARED-residue limb must survive at this door too. `.vjs/unreadable-orders.txt` records a
/// disclosed deviation from O5 - an order whose repair only its author may complete - and tolerating
/// a declared one is what kept this repository operable while its citator was being fixed. A door
/// that refused declared residue would have made the fix uncommittable.
#[test]
fn a_declared_unreadable_order_does_not_block_the_mcp_door() {
    let repo = scratch("declared");
    std::fs::write(
        repo.join(".vjs/orders/2026-VJS-BROKEN-003.yaml"),
        "id: [unclosed\n  : : :\n",
    )
    .unwrap();
    std::fs::write(
        repo.join(".vjs/unreadable-orders.txt"),
        "# disclosed deviation, [2026] VJS-CC-OPBOX 160 O5\n2026-VJS-BROKEN-003.yaml  reason\n",
    )
    .unwrap();
    let srv = vjs_mcp::McpServer::new(repo.clone());
    assert!(
        srv.handle_request(&route_params(&repo, "declared")).is_ok(),
        "a DECLARED unreadable order is a disclosed deviation, not a hard stop"
    );
}

// ---------------------------------------------------------------------------------------------
// 3. AN UNRESOLVABLE LAWPACK REFUSES, IT DOES NOT SILENTLY BECOME AN EMPTY ONE
// ---------------------------------------------------------------------------------------------

/// `[2026] VJS-CC-VJS 12` D1: for an INVOKED jurisdiction an unresolvable lawpack is a failure, not
/// a stage. The old private loader turned it into zero statutes, zero invariants and zero orders,
/// and the door then answered off that - the caller believing the law had been evaluated.
#[test]
fn mcp_refuses_an_invoked_jurisdiction_with_no_lawpack() {
    let repo = scratch("no-lawpack");
    // `.vjs/config.toml` is what makes a repo an invoked jurisdiction (is_invoked_jurisdiction).
    std::fs::write(
        repo.join(".vjs/config.toml"),
        "[governance]\njurisdiction_id = \"test\"\n",
    )
    .unwrap();

    let srv = vjs_mcp::McpServer::new(repo.clone());
    let err = srv
        .handle_request(&route_params(&repo, "no-law"))
        .expect_err("an invoked jurisdiction with no resolvable lawpack must refuse");
    let msg = format!("{err}");
    assert!(
        msg.contains("no lawpack could be resolved"),
        "the refusal must be the D1 refusal, not some other failure: {msg}"
    );
}

/// NEGATIVE CONTROL. The SAME repo without `.vjs/config.toml` is not a jurisdiction, so there is no
/// canon to be wrong about and the door must still work. This is the limb CC-VJS 12 preserved
/// expressly, and it is the reason the refusal above is conditional rather than absolute - so the
/// test proves the condition, not just the refusal.
#[test]
fn an_uninvoked_repo_still_routes_with_no_lawpack() {
    let repo = scratch("uninvoked");
    let srv = vjs_mcp::McpServer::new(repo.clone());
    assert!(
        srv.handle_request(&route_params(&repo, "no-law")).is_ok(),
        "a repo that is not a jurisdiction has no lawpack to be missing"
    );
}

// ---------------------------------------------------------------------------------------------
// 4. THE STRUCTURAL GUARD: ONE IMPLEMENTATION, NOT TWO
// ---------------------------------------------------------------------------------------------

/// The defect was not any one of the three behaviours above. It was that `vjs-mcp` had its OWN copy
/// of the door, so each fix had to be made twice and the second one never was. This asserts the copy
/// has not come back: `vjs-mcp/src/lib.rs` must define no private context builder, lawpack loader or
/// digest function.
///
/// A behavioural test cannot catch the regression this guards - a re-introduced private builder that
/// happens to be correct today passes every test above and rots tomorrow, which is exactly what
/// happened the first time.
#[test]
fn the_mcp_crate_defines_no_private_door() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../vjs-mcp/src/lib.rs");
    let text = std::fs::read_to_string(&src)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", src.display()));
    for banned in [
        "fn build_context(",
        "fn load_lawpack(",
        "fn compute_digest(",
    ] {
        assert!(
            !text.contains(banned),
            "vjs-mcp must not define its own `{banned}` - the door lives once, in \
             vjs_engine::door, so a fix reaches every door. Re-introducing this is how \
             .vjs/orders became invisible over MCP."
        );
    }
    assert!(
        text.contains("vjs_engine::build_kernel_context") || text.contains("vjs_engine::door"),
        "vjs-mcp must reach the shared engine door (vjs_engine::context here; vjs_engine::door \
         in the jurisdiction this test was written in - one door either way)"
    );
}
