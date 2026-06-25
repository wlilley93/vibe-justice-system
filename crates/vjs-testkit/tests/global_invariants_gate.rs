//! K-29 / K-30 - the global-invariant binding gate (the keystone discipline).
//!
//! Levels the kernel up toward best-in-class on a shared, machine-checked invariant set
//! (docs/global-invariants.yaml, K-1..K-30; see docs/kernel-building-single-source-of-truth.md).
//! Modelled on Agent kernelB's `check_test_invariants.py`. This gate HARD-FAILS if any test
//! named in a `vjs.tests` binding does not exist (a broken claim), and RATCHETS the VJS
//! binding debt (in-scope invariants with no bound test) so it can only ever decrease - a
//! future change cannot quietly leave a safety claim unbound.
//! Because it runs under `cargo test --workspace` (which the required CI re-runs), it is a
//! required-CI gate by construction (K-27).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Every `fn NAME` defined anywhere under crates/ (test fns + helpers). An existence check:
/// a binding is valid if the named test is a real function in the tree.
fn all_fn_names(root: &Path) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    let mut stack = vec![root.join("crates")];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                if p.file_name().and_then(|s| s.to_str()) != Some("target") {
                    stack.push(p);
                }
                continue;
            }
            if p.extension().and_then(|s| s.to_str()) != Some("rs") {
                continue;
            }
            let Ok(src) = std::fs::read_to_string(&p) else {
                continue;
            };
            for line in src.lines() {
                let t = line.trim();
                let rest = t
                    .strip_prefix("fn ")
                    .or_else(|| t.strip_prefix("async fn "))
                    .or_else(|| t.strip_prefix("pub fn "))
                    .or_else(|| t.strip_prefix("pub async fn "));
                if let Some(rest) = rest {
                    let name: String = rest
                        .chars()
                        .take_while(|c| c.is_alphanumeric() || *c == '_')
                        .collect();
                    if !name.is_empty() {
                        names.insert(name);
                    }
                }
            }
        }
    }
    names
}

#[test]
fn global_invariants_are_bound_and_debt_ratchets_down() {
    // The VJS binding debt may only DECREASE. Lower this as invariants are bound to tests
    // (K-30). 27 in-scope invariants (30 minus 3 n/a); 15 bound, 12 unbound after binding
    // K-12 (determinism fence), K-25 (surface pin), K-27 (CI trust root) with real tests.
    const VJS_DEBT_BASELINE: usize = 12;

    let root = workspace_root();
    let yaml = std::fs::read_to_string(root.join("docs/global-invariants.yaml"))
        .expect("docs/global-invariants.yaml must exist");
    let doc: serde_yaml::Value = serde_yaml::from_str(&yaml).expect("global-invariants.yaml parses");
    let invs = doc["invariants"]
        .as_sequence()
        .expect("invariants is a sequence");
    let fn_names = all_fn_names(&root);

    let mut debt = 0usize;
    let mut in_scope = 0usize;
    let mut missing: Vec<String> = vec![];
    let mut unbound: Vec<String> = vec![];

    for inv in invs {
        let id = inv["id"].as_str().unwrap_or("?");
        let vjs = &inv["vjs"];
        let status = vjs["status"].as_str().unwrap_or("gap");
        let tests: Vec<String> = vjs["tests"]
            .as_sequence()
            .map(|s| {
                s.iter()
                    .filter_map(|t| t.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        // 1. HARD: every named binding must be a real function.
        for t in &tests {
            if !fn_names.contains(t) {
                missing.push(format!("{id} -> {t}"));
            }
        }

        // n/a invariants are out of the VJS layer and excluded from debt.
        if status == "n/a" {
            continue;
        }
        in_scope += 1;
        if tests.is_empty() {
            debt += 1;
            unbound.push(format!("{id}({status})"));
        }
    }

    assert!(
        missing.is_empty(),
        "K-29 broken bindings - a named test does not exist (fix the name or write the test): {missing:?}"
    );

    println!(
        "K-29: VJS in-scope={in_scope}, bound={}, binding debt={debt} (baseline {VJS_DEBT_BASELINE}); unbound={unbound:?}",
        in_scope - debt
    );

    assert!(
        debt <= VJS_DEBT_BASELINE,
        "K-30 ratchet: VJS binding debt rose to {debt} (baseline {VJS_DEBT_BASELINE}). \
         Bind a test to the new/loosened invariant; the debt may only decrease."
    );
}
