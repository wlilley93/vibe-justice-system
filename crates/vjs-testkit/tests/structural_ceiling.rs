//! The structural-cleanliness ceiling, MACHINE-CHECKED (goal prong 2). No kernel source
//! file may exceed 600 lines. This runs under `cargo test --workspace`, which the required
//! CI re-runs (K-27), so the ceiling is enforced at the gate, not trusted to review.
//!
//! Before this test the ceiling was a manual snapshot: the goal-completion audit
//! (2026-06-26) found that nothing - no `vjs local-ci` step, no CI step, no test - counted
//! source lines, so a single refactor could silently breach it. This closes that gap.

use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

const CEILING: usize = 600;

#[test]
fn no_kernel_source_file_exceeds_the_line_ceiling() {
    let crates = workspace_root().join("crates");
    let mut over: Vec<(String, usize)> = Vec::new();
    let mut stack = vec![crates];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                // target/ holds generated artifacts, never source.
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
            let lines = src.lines().count();
            if lines > CEILING {
                let rel = p
                    .strip_prefix(workspace_root())
                    .map(|r| r.display().to_string())
                    .unwrap_or_else(|_| p.display().to_string());
                over.push((rel, lines));
            }
        }
    }
    over.sort();
    assert!(
        over.is_empty(),
        "structural-cleanliness ceiling ({CEILING} lines) breached by: {over:?}. \
         Split the file into cohesive modules (behavior-preserving); see the two splits in \
         PR #32 / #33 for the pattern."
    );
}
