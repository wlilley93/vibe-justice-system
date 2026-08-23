//! [2026] VJS-CC-VJS 20 D1 and D18: a removed governed record has THREE dispositions,
//! and only one of them is destructive.
//!
//! The gate this replaces asked one question of a PATH - is a governed record file
//! being deleted - and gave everything that answered yes the same destructive-delete
//! warning. Three different acts answer yes to that question.
//!
//! The occasion was 626 untrackings in one commit on 2026-08-06. Every one would have
//! raised a destructive-delete warning on a record that had not been deleted, and a
//! REAL deletion in that commit would have been the 627th line of an alarm nobody
//! could read. That is why the volume case below is a test and not a footnote.

use std::path::{Path, PathBuf};
use std::process::Command;

const VJS: &str = env!("CARGO_BIN_EXE_vjs");

fn git(dir: &Path, args: &[&str]) {
    let ok = Command::new("git")
        .args(args)
        .current_dir(dir)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_INDEX_FILE")
        .output()
        .unwrap_or_else(|e| panic!("git {args:?}: {e}"))
        .status
        .success();
    assert!(ok, "git {args:?} failed");
}

fn order(id: &str) -> String {
    format!(
        "id: {id}\ncourt: county\njurisdiction: test\nstatus: binding\nissue: fixture\n\
         holding: a fixture order\ndirectives:\n- id: D1\n  actor: lexby\n  must: exist\n\
         runtime_summary: a fixture\ncreated_at: \"2026\"\n"
    )
}

/// A jurisdiction with a REGISTERED store, because registration is the condition the
/// whole exemption turns on: "a record untracked out of every register, or held in a
/// store on no register, is deleted in law though no byte is erased."
fn estate(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("vjs-remove-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join(".vjs/orders")).unwrap();
    std::fs::create_dir_all(dir.join("lawpack/v2")).unwrap();
    git(&dir, &["init", "-q"]);
    git(&dir, &["config", "user.email", "t@example.invalid"]);
    git(&dir, &["config", "user.name", "T"]);
    std::fs::write(
        dir.join(".vjs/store-register.yaml"),
        "stores:\n- path: lawpack/v2\n  kind: test\n- path: .vjs/orders\n  kind: test\n",
    )
    .unwrap();
    dir
}

fn findings(dir: &Path) -> String {
    let out = Command::new(VJS)
        .args(["validate", "--staged", "--repo"])
        .arg(dir)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_INDEX_FILE")
        .output()
        .expect("vjs validate runs");
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

#[test]
fn untracking_a_record_that_stays_in_a_registered_store_is_not_a_deletion() {
    let dir = estate("unpublished");
    std::fs::write(dir.join(".vjs/orders/a.yaml"), order("2026-VJS-CC-TEST-1")).unwrap();
    git(&dir, &["add", "-A"]);
    git(
        &dir,
        &["-c", "core.hooksPath=/dev/null", "commit", "-qm", "seed"],
    );

    git(&dir, &["rm", "-q", "--cached", ".vjs/orders/a.yaml"]);
    let out = findings(&dir);
    assert!(
        out.contains("RECORD_UNPUBLISHED"),
        "untracking into a registered store is a publication decision:\n{out}"
    );
    assert!(
        !out.contains("DESTRUCTIVE_RECORD_DELETE"),
        "and must never be reported as destruction:\n{out}"
    );
}

#[test]
fn a_renamed_record_reports_the_rename_and_never_a_deletion() {
    // D18 in terms. The projection work of 2026-08-06/07 moved records between stores
    // constantly; every one of those moves would have read as a deletion.
    let dir = estate("renamed");
    std::fs::write(
        dir.join(".vjs/orders/old.yaml"),
        order("2026-VJS-CC-TEST-2"),
    )
    .unwrap();
    git(&dir, &["add", "-A"]);
    git(
        &dir,
        &["-c", "core.hooksPath=/dev/null", "commit", "-qm", "seed"],
    );

    // WHERE THIS BRANCH ACTUALLY BITES, measured rather than assumed. Git detects
    // renames itself and reports them as `R`, which `--diff-filter=D` never returns -
    // so a byte-identical move was always safe, and so was a lightly-edited one (a move
    // plus a projection note still measured 74% similar and was reported as R). The
    // branch is reachable only BELOW git's 50% similarity threshold: a record
    // substantially rewritten in the same act that moves it, which git splits into D +
    // A and which, before D18, made the D half read as a destroyed record.
    //
    // That is a narrow gap and the comment says so rather than letting the test imply
    // the gate is doing more work than it is.
    std::fs::remove_file(dir.join(".vjs/orders/old.yaml")).unwrap();
    let rewritten = format!(
        "{}{}",
        order("2026-VJS-CC-TEST-2"),
        (0..80)
            .map(|i| format!("note_{i}: the record was substantially rewritten as it moved\n"))
            .collect::<String>()
    );
    std::fs::write(dir.join(".vjs/orders/new.yaml"), rewritten).unwrap();
    git(&dir, &["add", "-A"]);
    let out = findings(&dir);
    assert!(
        out.contains("RECORD_RENAMED"),
        "the id survives at another path, so the record went nowhere:\n{out}"
    );
    assert!(
        !out.contains("DESTRUCTIVE_RECORD_DELETE"),
        "D18: a rename reports RECORD_RENAMED and never a deletion code:\n{out}"
    );
}

#[test]
fn a_record_that_survives_in_no_registered_store_is_still_destruction() {
    // THE CONTROL, and the half that must not move. Without it the two cases above are
    // indistinguishable from deleting the gate.
    let dir = estate("deleted");
    std::fs::write(
        dir.join(".vjs/orders/gone.yaml"),
        order("2026-VJS-CC-TEST-3"),
    )
    .unwrap();
    git(&dir, &["add", "-A"]);
    git(
        &dir,
        &["-c", "core.hooksPath=/dev/null", "commit", "-qm", "seed"],
    );

    std::fs::remove_file(dir.join(".vjs/orders/gone.yaml")).unwrap();
    git(&dir, &["add", "-A"]);
    let out = findings(&dir);
    assert!(
        out.contains("DESTRUCTIVE_RECORD_DELETE"),
        "the id survives nowhere the register knows about:\n{out}"
    );
    assert!(
        out.contains("2026-VJS-CC-TEST-3"),
        "and the finding names the RECORD, not just the path - that is D1:\n{out}"
    );
}

#[test]
fn a_record_moved_into_a_store_on_no_register_is_deleted_in_law() {
    // The court was precise about this and it is the sharpest edge of the ratio: "a
    // record untracked out of every register, or held in a store on no register, is
    // DELETED IN LAW though no byte is erased." The file below still exists. It is
    // still a deletion, because nothing that keeps a register would ever find it.
    let dir = estate("unregistered-store");
    std::fs::write(
        dir.join(".vjs/orders/moved.yaml"),
        order("2026-VJS-CC-TEST-4"),
    )
    .unwrap();
    git(&dir, &["add", "-A"]);
    git(
        &dir,
        &["-c", "core.hooksPath=/dev/null", "commit", "-qm", "seed"],
    );

    std::fs::create_dir_all(dir.join("attic")).unwrap();
    std::fs::remove_file(dir.join(".vjs/orders/moved.yaml")).unwrap();
    // Rewritten as it moves, so git reports D + A rather than collapsing it to an R
    // this gate would never see. See the note in the rename case above.
    let filed_away = format!(
        "{}{}",
        order("2026-VJS-CC-TEST-4"),
        (0..80)
            .map(|i| format!("note_{i}: put in the attic, out of every register\n"))
            .collect::<String>()
    );
    std::fs::write(dir.join("attic/moved.yaml"), filed_away).unwrap();
    git(&dir, &["add", "-A"]);
    let out = findings(&dir);
    assert!(
        dir.join("attic/moved.yaml").is_file(),
        "the bytes are still there, which is the point"
    );
    assert!(
        out.contains("DESTRUCTIVE_RECORD_DELETE"),
        "a store on no register is not a store the record survives in:\n{out}"
    );
}

#[test]
fn a_bulk_untracking_raises_no_destruction_alarm_at_all() {
    // The 2026-08-06 commit in miniature. Under the old path-keyed gate this printed
    // one destructive-delete warning per record, which is the failure mode that
    // matters: not a wrong answer, but a true one buried under fifty false ones.
    let dir = estate("bulk");
    for i in 0..50 {
        std::fs::write(
            dir.join(format!(".vjs/orders/o{i}.yaml")),
            order(&format!("2026-VJS-CC-BULK-{i}")),
        )
        .unwrap();
    }
    git(&dir, &["add", "-A"]);
    git(
        &dir,
        &["-c", "core.hooksPath=/dev/null", "commit", "-qm", "seed"],
    );

    git(&dir, &["rm", "-r", "-q", "--cached", ".vjs/orders"]);
    let out = findings(&dir);
    assert_eq!(
        out.matches("DESTRUCTIVE_RECORD_DELETE").count(),
        0,
        "fifty lawful untrackings must raise nothing:\n{out}"
    );
    assert_eq!(
        out.matches("RECORD_UNPUBLISHED").count(),
        50,
        "and each must still be REPORTED - silence would be its own defect:\n{out}"
    );
}
