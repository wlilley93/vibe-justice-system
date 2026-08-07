//! [2026] VJS-CC-VJS 20 D13: the order gates run AT REST, not only over a staged set.
//!
//! Before this, a governed record that was committed once was never looked at again. A
//! defect introduced before a gate existed, or by a commit that touched some other
//! path, stayed invisible for as long as nobody happened to re-stage the file. The
//! corpus was banking exactly the debt these checks describe and reporting itself
//! clean.
//!
//! The court sequenced D13 LAST and expressly after D10, in terms: "D13 does not land
//! before D10. If it does, plain `vjs validate` turns Fatal on thirteen orders PC-21
//! holds are in force."

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

/// An order malformed in one visible way: an EMPTY `runtime_summary`, which ACT-002:s10
/// requires be stated.
///
/// Empty and not ABSENT, deliberately. A record omitting the field fails to deserialise
/// as an `Order` at all and both doors skip it - which is its own gap, but a different
/// one. The present-but-empty form is what this gate is written to catch.
fn malformed(id: &str) -> String {
    format!(
        "id: {id}\ncourt: county\njurisdiction: test\nstatus: binding\nissue: at_rest_fixture\n\
         holding: it has a holding and directives but an EMPTY runtime_summary\ndirectives:\n\
         - id: D1\n  actor: lexby\n  must: exist\nruntime_summary: \"\"\ncreated_at: \"2026\"\n"
    )
}

fn sound(id: &str) -> String {
    format!(
        "id: {id}\ncourt: county\njurisdiction: test\nstatus: binding\nissue: at_rest_fixture\n\
         holding: a sound order\ndirectives:\n- id: D1\n  actor: lexby\n  must: exist\n\
         runtime_summary: a sound order\ncreated_at: \"2026\"\n"
    )
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

/// A real jurisdiction with a real lawpack, because the sweep needs a courts
/// constitution to verify a bench against and refuses to run without one.
fn estate(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("vjs-atrest-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join(".vjs/orders")).unwrap();
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
    Command::new(VJS)
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
    dir
}

fn validate(dir: &Path, staged: bool) -> String {
    let mut args = vec!["validate"];
    if staged {
        args.push("--staged");
    }
    args.push("--repo");
    let out = Command::new(VJS)
        .args(&args)
        .arg(dir)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("VJS_LAWPACK")
        .output()
        .expect("vjs validate runs");
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

#[test]
fn a_defect_in_a_record_nobody_is_touching_is_found() {
    // THE POINT OF D13. The malformed order is written, committed, and then left alone.
    // Nothing stages it ever again. Under the staged-only gate it was invisible from
    // the moment it landed.
    let dir = estate("resting");
    std::fs::write(
        dir.join(".vjs/orders/resting.yaml"),
        malformed("2026-VJS-CC-ATREST-1"),
    )
    .unwrap();

    let out = validate(&dir, false);
    assert!(
        out.contains("AT_REST_ORDER_MALFORMED"),
        "a record at rest must still be examined:\n{out}"
    );
    assert!(
        out.contains("2026-VJS-CC-ATREST-1") || out.contains("resting.yaml"),
        "and the finding must be addressable to the record:\n{out}"
    );
}

#[test]
fn a_sound_corpus_at_rest_earns_silence() {
    // The control against crying wolf. Every gate that sweeps a whole tree must be
    // silent on a sound one, or the noise makes the real findings unreadable - which is
    // the failure this whole docket keeps circling.
    let dir = estate("sound");
    std::fs::write(
        dir.join(".vjs/orders/sound.yaml"),
        sound("2026-VJS-CC-ATREST-2"),
    )
    .unwrap();
    // Scoped to THIS record, not to global silence: the estate carries a full copy of
    // the real lawpack, whose own orders have their own standing at-rest findings. A
    // test asserting the whole sweep is silent would be asserting the corpus is perfect,
    // which is a different claim and not this one.
    let out = validate(&dir, false);
    assert!(
        !out.contains("2026-VJS-CC-ATREST-2") && !out.contains("sound.yaml"),
        "a sound record must raise nothing about ITSELF at rest:\n{out}"
    );
}

#[test]
fn the_staged_gate_stays_fatal_while_the_at_rest_sweep_warns() {
    // THE ADMISSION PROOF for `order_checks.rs` joining ENFORCEMENT_SURFACE, and the
    // property that makes D13 landable at all.
    //
    // A record being WRITTEN makes a claim about itself, and a Fatal is the right answer
    // to a false one. A record AT REST is already in force and relied upon; its defects
    // are the correction register's business (PC 21 D2/D3), not a reason to refuse every
    // future commit in the jurisdiction until somebody repairs history.
    //
    // The two severities live one function apart, and a careless edit generalising the
    // at-rest downgrade back into `order_findings` would silently turn the COMMIT gate
    // advisory - a confined change to one file flipping a bright-line outcome, which is
    // exactly the CC-VJS 18 C6 test for entrenchment.
    let dir = estate("both-doors");
    std::fs::write(
        dir.join(".vjs/orders/both.yaml"),
        malformed("2026-VJS-CC-ATREST-3"),
    )
    .unwrap();

    // At rest: Warning, and prefixed so the two are never confused in a log.
    let at_rest = validate(&dir, false);
    assert!(
        at_rest.contains("[Warning] AT_REST_ORDER_MALFORMED"),
        "at rest the finding is a Warning:\n{at_rest}"
    );
    assert!(
        !at_rest.contains("[Fatal] ORDER_MALFORMED"),
        "and must not also fire the staged Fatal on a record nobody staged:\n{at_rest}"
    );
    assert!(
        at_rest.contains("both.yaml"),
        "the at-rest finding names the record it is about:\n{at_rest}"
    );

    // Staged: the same defect, the same code body, Fatal.
    assert!(
        Command::new("git")
            .args(["add", "-A"])
            .current_dir(&dir)
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .env_remove("GIT_INDEX_FILE")
            .status()
            .unwrap()
            .success()
    );
    let staged = validate(&dir, true);
    assert!(
        staged.contains("[Fatal] ORDER_MALFORMED"),
        "writing the same record is still refused outright:\n{staged}"
    );
}

#[test]
fn the_sweep_discloses_rather_than_passing_when_it_cannot_run() {
    // A lawpack with no courts constitution cannot verify a bench against anything. The
    // sweep must say it did not run - a statement about this estate - and never report
    // a clean result it did not earn.
    let dir = estate("no-constitution");
    let orders = dir.join("lawpack/v2/orders");
    for entry in std::fs::read_dir(&orders).unwrap().flatten() {
        let text = std::fs::read_to_string(entry.path()).unwrap_or_default();
        if text.contains("COURTS-CONSTITUTION") {
            std::fs::remove_file(entry.path()).unwrap();
        }
    }
    let out = validate(&dir, false);
    assert!(
        out.contains("AT_REST_ORDERS_UNCHECKED"),
        "an unrunnable sweep discloses; it never passes:\n{out}"
    );
}

#[test]
fn two_files_sharing_a_record_id_are_reported_once() {
    // [2026] VJS-CC-RECORD-PROJECTION-009 D2/D4, applied by CC-VJS 20: "collapse two
    // files sharing a record id into one record before counting any per-record duty."
    //
    // This is not hypothetical tidiness. The corpus deliberately keeps projections - the
    // same order filed under `.vjs/court/orders` and overlaid from `.vjs/orders` - and
    // the first version of this sweep walked FILES. It reported 15 findings over 13
    // records against a correction register that correctly holds 13 rows, and a register
    // and a gate disagreeing by two reads as two unrecorded obligations rather than as
    // double vision.
    let dir = estate("projection");
    std::fs::create_dir_all(dir.join(".vjs/court/orders")).unwrap();
    let body = malformed("2026-VJS-CC-ATREST-PROJECTED");
    std::fs::write(dir.join(".vjs/orders/p.yaml"), &body).unwrap();
    // The projection: same record id, different path, and deliberately NOT byte-identical
    // so nothing can pass this by comparing contents instead of ids.
    std::fs::write(
        dir.join(".vjs/court/orders/p.yaml"),
        format!("# the filed original\n{body}"),
    )
    .unwrap();

    let out = validate(&dir, false);
    let hits = out
        .lines()
        .filter(|l| l.contains("AT_REST_ORDER_MALFORMED") && l.contains("p.yaml"))
        .count();
    assert_eq!(
        hits, 1,
        "one record, one finding - two files sharing an id are one record:\n{out}"
    );
}
