//! The K-12 dependency fence, ENFORCED - not merely asserted. DEC-KERNEL-001 /
//! ACT-COMPUTER-FIRST-REALM s.11: the kernel is model-free and network-free, and the sound
//! witness is the dependency CLOSURE (make the capability impossible, not merely prohibited).
//!
//! deny.toml DECLARES the ban list. This test ENFORCES it against the actual Cargo.lock,
//! under `cargo test --workspace`, which the required CI re-runs (K-27). So a banned HTTP
//! client or hosted-model SDK that enters the workspace graph fails the gate even when
//! `cargo deny` itself is never invoked - which the goal-completion audit (2026-06-26)
//! found to be the case: the old K-12 test only asserted deny.toml CONTAINED the ban
//! strings, while nothing checked the real graph.

use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The banned crate names declared in deny.toml's `[bans] deny = [ { crate = "..." }, ... ]`.
/// String-parsed (no toml dep) so the fence has no dependency of its own.
fn banned_crates(deny_toml: &str) -> Vec<String> {
    let Some(start) = deny_toml.find("deny = [").map(|i| i + "deny = [".len()) else {
        return Vec::new();
    };
    let rest = &deny_toml[start..];
    let block = &rest[..rest.find(']').unwrap_or(rest.len())];
    let needle = "crate = \"";
    let mut out = Vec::new();
    let mut i = 0;
    while let Some(p) = block[i..].find(needle) {
        let s = i + p + needle.len();
        let Some(q) = block[s..].find('"') else { break };
        out.push(block[s..s + q].to_string());
        i = s + q;
    }
    out
}

#[test]
fn no_banned_network_or_model_crate_is_in_the_lockfile() {
    let root = workspace_root();
    let deny = std::fs::read_to_string(root.join("deny.toml")).expect("deny.toml present");
    let lock = std::fs::read_to_string(root.join("Cargo.lock")).expect("Cargo.lock present");
    let banned = banned_crates(&deny);
    assert!(
        banned.len() >= 10,
        "parsed deny.toml ban list as {banned:?} - expected the full network/model ban set; \
         a mis-parse would make this fence vacuous"
    );
    // Cargo.lock lists each package as `name = "NAME"` (with the closing quote, so a ban on
    // "hyper" does not match "hyper-util" - and hyper-util is separately banned anyway).
    let present: Vec<&String> = banned
        .iter()
        .filter(|name| lock.contains(&format!("name = \"{name}\"")))
        .collect();
    assert!(
        present.is_empty(),
        "K-12 dependency fence breached: banned network/model crate(s) {present:?} are in \
         Cargo.lock. The kernel closure must stay model-free and network-free \
         (DEC-KERNEL-001; deny.toml). Remove the dependency, do not exempt it."
    );
}

#[test]
fn the_fence_parser_finds_the_known_bans() {
    // Guard the parser: if it returned empty, the enforcement above would silently pass.
    let deny =
        std::fs::read_to_string(workspace_root().join("deny.toml")).expect("deny.toml present");
    let banned = banned_crates(&deny);
    for must in ["reqwest", "hyper", "anthropic", "async-openai"] {
        assert!(
            banned.contains(&must.to_string()),
            "fence parser missed the known ban '{must}'"
        );
    }
}
