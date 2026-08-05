//! [2026] VJS-CC-VJS 15: the MCP door resolves the canon like every other door.
//!
//! Until this ruling `vjs-mcp` carried its OWN `load_lawpack` and `compute_digest`. The
//! loader named `lawpack/v2` and returned an EMPTY lawpack when it was absent, so it never
//! reached the CC-VJS 12 D1 refusal; the digest hashed `manifest.toml` alone, the superseded
//! computation D4 rejected. The consequence was not a thin answer, it was a fail-OPEN
//! constitutive gate: with an empty lawpack the `record` verb found no
//! `2026-VJS-COURTS-CONSTITUTION-001`, so `verify_bench` sat inside an `if let
//! Some(constitution)` that never ran, and a two-judge County order with no opinion was
//! RECORDED. The vendored control refuses the same order with BENCH_SIZE_MISMATCH and
//! BENCH_OPINION_MISSING.
//!
//! These drive the BINARY over stdio, because the framing is part of the ruling: the door
//! must answer a JSON-RPC ERROR, not an empty result carrying a caveat. `vjs.lookup` has no
//! warning channel - it returns an authority array - so "empty with a note" is not available
//! to it even in principle, and an error is the only honest answer.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const MCP: &str = env!("CARGO_BIN_EXE_vjs-mcp");

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("vjs-mcp-door-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    // The server resolves its repo root from `git rev-parse --show-toplevel` and falls back
    // to the cwd. Make the fixture its own repo so the answer is the fixture either way,
    // never a repository that happens to sit above the temp dir.
    let ok = Command::new("git")
        .args(["init", "-q"])
        .current_dir(&dir)
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    assert!(ok, "git init failed in {}", dir.display());
    dir
}

fn real_lawpack() -> PathBuf {
    let p = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../lawpack/v2")
        .canonicalize()
        .expect("the lawpack must exist for these tests to mean anything");
    assert!(
        p.join("manifest.toml").is_file(),
        "not a lawpack: {}",
        p.display()
    );
    p
}

fn write_config(repo: &Path, lawpack_path: Option<&Path>) {
    std::fs::create_dir_all(repo.join(".vjs")).unwrap();
    let mut cfg = String::from(
        "version = \"2\"\njurisdiction_id = \"t\"\nrepo_code = \"T\"\n\
         lawpack = \"vjs-v2@0.1.0\"\nprincipal = \"p\"\n",
    );
    // Top level, BEFORE any table header: `resolve_lawpack_dir` reads this key with a line
    // scan (deliberately - it runs before the kernel context exists), so a key written after
    // `[paths]` would be TOML-valid and resolution-invisible.
    if let Some(p) = lawpack_path {
        cfg.push_str(&format!("lawpack_path = \"{}\"\n", p.display()));
    }
    cfg.push_str(
        "\n[paths]\norders = \".vjs/orders\"\nlogs = \".vjs/logs\"\n\
         submissions = \".vjs/submissions\"\nspecs = \"lawpack/v2/specs\"\n\
         decisions = \"lawpack/v2/decisions\"\nproofs = \".vjs/proofs\"\n\
         permits = \".vjs/permits\"\nprivate = \".vjs/private\"\ncache = \".vjs/cache\"\n",
    );
    std::fs::write(repo.join(".vjs/config.toml"), cfg).unwrap();
}

/// A JSON-RPC response carries an `error` object. The success framing writes `"error":
/// null` rather than omitting the key, so presence of the key is not the test.
fn is_error(resp: &serde_json::Value) -> bool {
    !resp["error"].is_null()
}

/// One request per line in, one response object per line out.
fn call(repo: &Path, requests: &[serde_json::Value]) -> Vec<serde_json::Value> {
    let mut child = Command::new(MCP)
        .current_dir(repo)
        .env_remove("VJS_LAWPACK")
        .env_remove("VJS_MCP_TOKEN")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    {
        let stdin = child.stdin.as_mut().unwrap();
        for r in requests {
            writeln!(stdin, "{r}").unwrap();
        }
    }
    let out = child.wait_with_output().unwrap();
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("a JSON-RPC response per line"))
        .collect()
}

fn county_order(id: &str, bench: &[&str], opinion: Option<&str>) -> serde_json::Value {
    let mut o = serde_json::json!({
        "id": id,
        "court": "county",
        "jurisdiction": "t",
        "status": "binding",
        "issue": "operational.door",
        "holding": "a holding",
        "directives": [{ "id": "D1", "actor": "lexby", "must": "do the thing", "when": null }],
        "forbidden": null,
        "exceptions": null,
        "source_opinion": null,
        "created_at": "2026-08-01T00:00:00Z",
        "bench": bench,
    });
    if let Some(p) = opinion {
        o["source_opinion"] = serde_json::Value::String(p.to_string());
    }
    o
}

fn err_message(resp: &serde_json::Value) -> String {
    assert!(is_error(resp), "expected a JSON-RPC error, got: {resp}");
    assert!(
        resp["result"].is_null(),
        "an error carries no result: {resp}"
    );
    resp["error"]["message"].as_str().unwrap_or_default().into()
}

/// C2 + C3, the reachable-opposite direction: a jurisdiction that vendors NOTHING answers
/// from the canon it subscribes to, and the constitutive bench gate actually runs there.
#[test]
fn an_out_of_tree_canon_answers_lookup_and_arms_the_bench_gate() {
    let repo = scratch("out-of-tree");
    write_config(&repo, Some(&real_lawpack()));

    let resp = call(
        &repo,
        &[
            serde_json::json!({"jsonrpc":"2.0","id":1,"method":"vjs.lookup",
                               "params":{"issue":"enforcement"}}),
            // Two judges where the constitution seats ONE, and no opinion for either. Before
            // the fix this was RECORDED: the empty lawpack had no constitution, so the whole
            // check was skipped. A gate that is skipped when its instrument cannot be found
            // is a gate that fails open.
            serde_json::json!({"jsonrpc":"2.0","id":2,"method":"vjs.record",
                               "params": county_order("2026-VJS-CC-T-001", &["Judge A", "Judge B"], None)}),
        ],
    );
    assert_eq!(resp.len(), 2, "one response per request: {resp:?}");

    let authorities = &resp[0]["result"]["authorities"];
    assert!(
        authorities
            .as_array()
            .map(|a| !a.is_empty())
            .unwrap_or(false),
        "a subscribed canon must answer lookup rather than return []: {}",
        resp[0]
    );
    assert!(
        serde_json::to_string(authorities)
            .unwrap()
            .contains("ACT-001"),
        "the subscribed canon must reach the door: {}",
        resp[0]
    );

    let msg = err_message(&resp[1]);
    for code in ["BENCH_SIZE_MISMATCH", "BENCH_OPINION_MISSING"] {
        assert!(msg.contains(code), "the refusal must name {code}: {msg}");
    }
    assert!(
        !repo
            .join("lawpack/v2/orders/2026-VJS-CC-T-001.yaml")
            .exists(),
        "a refused order must not be written"
    );
}

/// The negative control for the refusal above: the same door in the same shape RECORDS a
/// properly constituted order. Without this, "record refuses" could be satisfied by a door
/// that refuses everything, which proves nothing about the bench gate.
#[test]
fn a_properly_constituted_county_order_is_still_recorded() {
    let repo = scratch("constituted");
    write_config(&repo, Some(&real_lawpack()));
    // A single judge (the constituted odd bench of 1 for the County Court) with a real
    // opinion, so no seat is silent.
    std::fs::create_dir_all(repo.join(".vjs/opinions")).unwrap();
    std::fs::write(
        repo.join(".vjs/opinions/op.md"),
        "## Judge A\n\nJudge A sets out the reasons at length, because a seat that owns no \
         attributed content in the opinion document is a silent seat and the record does not \
         evidence its participation. This paragraph exists to own more than the minimum \
         attributed content the bench gate measures.\n",
    )
    .unwrap();

    let resp = call(
        &repo,
        &[
            serde_json::json!({"jsonrpc":"2.0","id":1,"method":"vjs.record",
                             "params": county_order("2026-VJS-CC-T-002", &["Judge A"],
                                                    Some(".vjs/opinions/op.md"))}),
        ],
    );
    assert!(
        !is_error(&resp[0]),
        "a constituted bench must still record: {}",
        resp[0]
    );
    assert_eq!(resp[0]["result"]["recorded"], "2026-VJS-CC-T-002");
}

/// C2: with no resolvable lawpack in an INVOKED jurisdiction, every canon-reading verb
/// answers a JSON-RPC error. C3: the fixture is the one above with `lawpack_path` removed,
/// so the opposite outcome is reachable by putting one line back.
#[test]
fn an_invoked_jurisdiction_with_no_resolvable_lawpack_errors_at_every_verb() {
    let repo = scratch("unresolvable");
    write_config(&repo, Some(&real_lawpack()));
    // Prove the flip: with the path, lookup answers.
    let before = call(
        &repo,
        &[
            serde_json::json!({"jsonrpc":"2.0","id":1,"method":"vjs.lookup",
                             "params":{"issue":"enforcement"}}),
        ],
    );
    assert!(
        !is_error(&before[0]),
        "the opposite outcome must be reachable in this fixture: {}",
        before[0]
    );

    write_config(&repo, None);
    let resp = call(
        &repo,
        &[
            serde_json::json!({"jsonrpc":"2.0","id":1,"method":"vjs.lookup",
                               "params":{"issue":"enforcement"}}),
            serde_json::json!({"jsonrpc":"2.0","id":2,"method":"vjs.route",
                               "params":{"actor":"lexby","action_kind":"implementation_decision",
                                         "issue_tags":["enforcement"],"intent":"i",
                                         "affected_paths":[],"risk":"low","public_target":false,
                                         "external_target":false,"irreversible":false}}),
            serde_json::json!({"jsonrpc":"2.0","id":3,"method":"vjs.record",
                               "params": county_order("2026-VJS-CC-T-003", &["Judge A", "Judge B"], None)}),
        ],
    );
    assert_eq!(resp.len(), 3, "one response per request: {resp:?}");
    for (i, verb) in ["vjs.lookup", "vjs.route", "vjs.record"].iter().enumerate() {
        let msg = err_message(&resp[i]);
        assert!(
            msg.contains("no lawpack could be resolved"),
            "{verb} must carry the D1 refusal, got: {msg}"
        );
        // All three candidate sources NAMED (CC-VJS 12 D1).
        for source in ["lawpack/v2", "lawpack_path", "VJS_LAWPACK"] {
            assert!(msg.contains(source), "{verb} must name {source}: {msg}");
        }
    }
    assert!(
        !repo.join("lawpack/v2").exists(),
        "a refused record must not bring the canon tree into being"
    );
}

/// The limb CC-VJS 12 preserved, and the negative control for the refusal above: a repo
/// that is NOT a jurisdiction still answers on an empty canon. Without this, D1 could be
/// satisfied by a door that refuses everywhere.
#[test]
fn a_repo_that_is_not_a_jurisdiction_still_answers_on_an_empty_canon() {
    let repo = scratch("not-a-jurisdiction");
    let resp = call(
        &repo,
        &[
            serde_json::json!({"jsonrpc":"2.0","id":1,"method":"vjs.lookup",
                             "params":{"issue":"enforcement"}}),
        ],
    );
    assert!(
        !is_error(&resp[0]),
        "no .vjs/config.toml means no canon to be wrong about: {}",
        resp[0]
    );
    assert_eq!(resp[0]["result"]["authorities"], serde_json::json!([]));
}

/// [2026] VJS-CC-VJS 16 C1 + C2: the record verb writes to this jurisdiction's own order
/// register, and no write path brings the resolver's directory into being.
///
/// The fixture vendors NOTHING and subscribes out of tree, which is the configuration the
/// harm lives in: in a repository that already vendors a canon, the write would land beside
/// 160 other files and displace nothing, so the assertion could not fail on the defect.
#[test]
fn record_writes_to_the_local_register_and_never_manufactures_the_canon_tree() {
    let repo = scratch("write-target");
    write_config(&repo, Some(&real_lawpack()));
    assert!(
        !repo.join("lawpack/v2").exists(),
        "this test is only about a jurisdiction that vendors NO canon"
    );
    std::fs::create_dir_all(repo.join(".vjs/opinions")).unwrap();
    std::fs::write(
        repo.join(".vjs/opinions/op.md"),
        "## Judge A\n\nJudge A sets out the reasons at length, because a seat that owns no \
         attributed content in the opinion document is a silent seat and the record does not \
         evidence its participation. This paragraph exists to own more than the minimum \
         attributed content the bench gate measures.\n",
    )
    .unwrap();

    let resp = call(
        &repo,
        &[
            serde_json::json!({"jsonrpc":"2.0","id":1,"method":"vjs.record",
                             "params": county_order("2026-VJS-CC-T-016", &["Judge A"],
                                                    Some(".vjs/opinions/op.md"))}),
        ],
    );
    assert!(
        !is_error(&resp[0]),
        "a constituted order must record: {}",
        resp[0]
    );

    // C1: the destination is the LOCAL register, and the verb SAYS so.
    assert!(
        repo.join(".vjs/orders/2026-VJS-CC-T-016.yaml").is_file(),
        "the record must land in this jurisdiction's own order register"
    );
    let reported = resp[0]["result"]["path"].as_str().unwrap_or_default();
    assert!(
        reported.contains(".vjs/orders"),
        "the verb must report where it actually wrote, got: {reported}"
    );

    // C2: and the resolver's directory must not have been brought into being. This is the
    // whole ruling in one assertion - the write is what manufactured the canon tree, and a
    // manufactured canon tree is what silenced the canon-sourced half of enforcement.
    assert!(
        !repo.join("lawpack/v2").exists(),
        "a WRITE path created the directory the resolver reads the canon from"
    );
}

/// [2026] VJS-CC-VJS 16 C4: both doors answer from ONE graph, and that graph carries this
/// repository's own filed orders as well as the canon it subscribes to.
///
/// THE FIXTURE VENDORS NOTHING AND RESOLVES OUT OF TREE, and it asserts so. Recording through
/// the verb and reading it back in a repository that VENDORS the canon would be vacuous: the
/// record would be visible either way, so the assertion could not fail on the defect.
///
/// AND IT ASSERTS BOTH DIRECTIONS. The failure being cured is one register replacing the
/// other, so a test that only counted the local order would pass on the very displacement
/// this order forbids. The recorded order and the constitutional stack must come back in ONE
/// answer to ONE question.
#[test]
fn one_graph_carries_the_local_register_and_the_subscribed_canon() {
    let repo = scratch("one-graph");
    write_config(&repo, Some(&real_lawpack()));
    assert!(
        !repo.join("lawpack/v2").exists(),
        "the fixture must vendor nothing, or the record is visible either way"
    );
    std::fs::create_dir_all(repo.join(".vjs/opinions")).unwrap();
    std::fs::write(
        repo.join(".vjs/opinions/op.md"),
        "## Judge A\n\nJudge A sets out the reasons at length, because a seat that owns no \
         attributed content in the opinion document is a silent seat and the record does not \
         evidence its participation. This paragraph exists to own more than the minimum \
         attributed content the bench gate measures.\n",
    )
    .unwrap();

    // The order's issue is the canon's own busiest issue, so ONE lookup can prove both
    // halves: the order is hoisted on-point, the constitutional stack follows it.
    let mut order = county_order(
        "2026-VJS-CC-T-017",
        &["Judge A"],
        Some(".vjs/opinions/op.md"),
    );
    order["issue"] = serde_json::Value::String("enforcement".into());

    let resp = call(
        &repo,
        &[
            serde_json::json!({"jsonrpc":"2.0","id":1,"method":"vjs.record","params": order}),
            serde_json::json!({"jsonrpc":"2.0","id":2,"method":"vjs.lookup",
                               "params":{"issue":"enforcement"}}),
        ],
    );
    assert_eq!(resp.len(), 2, "one response per request: {resp:?}");
    assert!(
        !is_error(&resp[0]),
        "a constituted order must record: {}",
        resp[0]
    );

    let answer = serde_json::to_string(&resp[1]["result"]).unwrap();
    assert!(
        answer.contains("2026-VJS-CC-T-017"),
        "the door must read back what it recorded - at HEAD the only thing that ever made it \
         legible was that the write displaced the canon: {answer}"
    );
    assert!(
        answer.contains("ACT-001"),
        "and the canon stack must still be there: one register replacing the other is the \
         failure being cured, not the cure: {answer}"
    );
}
