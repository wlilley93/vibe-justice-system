//! The lawpack's `[limits]` table governs, and until 2026-08-07 it governed nothing.
//!
//! Eleven limits were declared in `manifest.toml`, every one of them had a field in
//! `ContextLimits`, and every consumer got its number from `Default::default()`. The
//! table was decorative. Worse, the two live consumers had not even reached the
//! defaults: `cmd_file` hardcoded 500 for every court (so the COUNTY ceiling silently
//! governed a Privy Council case file) and `cmd_log decision` hardcoded 150 twice.
//!
//! THE PROOF THAT MATTERS IS MOVEMENT. A test that asserts "the limit is 150" passes
//! just as happily against a hardcoded literal as against a wired one, which is how
//! this survived. So each case below EDITS the manifest and requires the gate to move
//! with it - the one thing a literal cannot do.

use std::path::{Path, PathBuf};
use std::process::Command;

const VJS: &str = env!("CARGO_BIN_EXE_vjs");

fn canon() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root")
        .to_path_buf()
}

/// A jurisdiction with its OWN copy of the lawpack, so the manifest can be edited
/// without touching the canon this suite is running inside.
fn estate(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("vjs-limits-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    for args in [
        vec!["init", "-q"],
        vec!["config", "user.email", "t@example.invalid"],
        vec!["config", "user.name", "T"],
    ] {
        assert!(
            Command::new("git")
                .args(&args)
                .current_dir(&dir)
                .env_remove("GIT_DIR")
                .env_remove("GIT_WORK_TREE")
                .env_remove("GIT_INDEX_FILE")
                .status()
                .unwrap()
                .success()
        );
    }
    copy_tree(&canon().join("lawpack/v2"), &dir.join("lawpack/v2"));
    let out = Command::new(VJS)
        .args([
            "invoke",
            "--jurisdiction",
            "acme",
            "--principal",
            "Alice",
            "--lawpack",
        ])
        .arg(dir.join("lawpack/v2"))
        .arg("--repo")
        .arg(&dir)
        .env_remove("VJS_LAWPACK")
        .output()
        .expect("vjs invoke runs");
    assert!(
        dir.join(".vjs/config.toml").is_file(),
        "invoke must produce a jurisdiction: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    dir
}

fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap().flatten() {
        let (src, dst) = (entry.path(), to.join(entry.file_name()));
        if src.is_dir() {
            copy_tree(&src, &dst);
        } else {
            std::fs::copy(&src, &dst).unwrap();
        }
    }
}

/// Rewrite one `key = N` under `[limits]`.
fn set_limit(repo: &Path, key: &str, value: usize) {
    let path = repo.join("lawpack/v2/manifest.toml");
    let text = std::fs::read_to_string(&path).unwrap();
    let mut out = String::new();
    let mut hit = false;
    for line in text.lines() {
        if line.trim_start().starts_with(key) && line.contains('=') {
            out.push_str(&format!("{key} = {value}\n"));
            hit = true;
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }
    assert!(hit, "manifest declares no `{key}` to move");
    std::fs::write(&path, out).unwrap();
}

fn vjs(repo: &Path, args: &[&str]) -> String {
    let out = Command::new(VJS)
        .args(args)
        .arg("--repo")
        .arg(repo)
        .env_remove("VJS_LAWPACK")
        .output()
        .expect("the vjs binary runs");
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

#[test]
fn moving_the_manifests_decision_log_ceiling_moves_the_gate() {
    let dir = estate("decision-log");
    let why: String = std::iter::repeat_n("word", 40)
        .collect::<Vec<_>>()
        .join(" ");
    let log = |d: &Path| {
        vjs(
            d,
            &[
                "log",
                "decision",
                "--kind",
                "code",
                "--issue",
                "limits_probe",
                "--decision",
                "probe",
                "--risk",
                "low",
                "--why",
                &why,
                "--basis",
                "PERMIT-NONE",
            ],
        )
    };

    // 40 words is comfortably under the shipped 150 and must pass.
    assert!(
        !log(&dir).contains("word limit exceeded"),
        "40 words is under the shipped ceiling"
    );

    // Now the estate lowers its own ceiling to 10. A hardcoded 150 cannot notice.
    set_limit(&dir, "decision_log_max_words", 10);
    let out = log(&dir);
    assert!(
        out.contains("word limit exceeded") && out.contains("/10"),
        "the estate lowered its ceiling to 10 and the gate must move with it:\n{out}"
    );
}

#[test]
fn the_tier_ceilings_are_the_courts_own_and_come_from_the_manifest() {
    let dir = estate("tiers");
    let facts = dir.join("facts.md");
    // 700 words: over the shipped county ceiling of 500, under the privy 1000.
    std::fs::write(
        &facts,
        std::iter::repeat_n("word", 700)
            .collect::<Vec<_>>()
            .join(" "),
    )
    .unwrap();
    let file = |d: &Path, court: &str| {
        vjs(
            d,
            &[
                "file",
                "--court",
                court,
                "--question",
                "q",
                "--facts-file",
                facts.to_str().unwrap(),
            ],
        )
    };

    // THE DEFECT THIS FILE WAS OPENED ON: 500 was hardcoded for every seat, so a Privy
    // Council case file had to be cut in half to fit a limit meant for the County.
    assert!(
        file(&dir, "county").contains("word limit exceeded"),
        "700 words is over the county ceiling"
    );
    assert!(
        !file(&dir, "privy").contains("word limit exceeded"),
        "and under the privy one - the seats have different ceilings, which was the bug"
    );

    // And the privy ceiling is the MANIFEST'S, not a second literal.
    set_limit(&dir, "privy_submission_max_words", 100);
    let out = file(&dir, "privy");
    assert!(
        out.contains("word limit exceeded") && out.contains("/100"),
        "the manifest's privy ceiling must govern the privy seat:\n{out}"
    );
}

#[test]
fn an_unrecognised_seat_gets_the_strictest_ceiling_never_the_loosest() {
    // Reading the limit per court means a typo could otherwise buy room. County is the
    // floor and the fallback, so `--court privvy` is refused where `--court privy`
    // would pass.
    let dir = estate("typo");
    let facts = dir.join("facts.md");
    std::fs::write(
        &facts,
        std::iter::repeat_n("word", 700)
            .collect::<Vec<_>>()
            .join(" "),
    )
    .unwrap();
    let out = vjs(
        &dir,
        &[
            "file",
            "--court",
            "privvy",
            "--question",
            "q",
            "--facts-file",
            facts.to_str().unwrap(),
        ],
    );
    assert!(
        out.contains("word limit exceeded"),
        "a misspelled seat must not inherit the higher ceiling:\n{out}"
    );
}
