//! A record LEAVING the commit makes no claim about its form.
//!
//! [2026] VJS-CC-VJS 20 D2 created a state that did not previously exist: a governed
//! record removed from version tracking but LEFT ON DISK inside a registered store.
//! The staged well-formedness gate reads the file from disk, so until this was fixed
//! it read a record it was not being asked about and applied the requirements of a
//! record being WRITTEN to one being UNPUBLISHED. The cost was concrete: it refused
//! the removal of a record carrying seven denylisted terms from a tree about to be
//! published, because the record it was removing was malformed.
//!
//! WHAT THIS FILE PROVES AND WHAT IT DOES NOT. It proves the PREDICATE: that the gate
//! now asks git which paths are staged deletions, and that `read_staged_deletions`
//! answers correctly for the untracking shape (`git rm --cached`, file still on disk).
//! The end-to-end behaviour was verified LIVE on the canon on 2026-08-06 and is
//! recorded here rather than simulated: with 626 untrackings staged, ORDER_MALFORMED
//! went from 1 to 0, and re-staging one of the same malformed records as a WRITE
//! brought it straight back to 1. That control matters more than the fix - a change
//! indistinguishable from switching the gate off is not a fix.
//!
//! An end-to-end fixture was attempted and DELETED rather than shipped. A bare git
//! repo made both cases pass vacuously (the staged order checks need a loaded
//! lawpack), and `vjs invoke` could not produce a runnable one either: a freshly
//! invoked jurisdiction writes a config that `validate` refuses to parse, "missing
//! field `specs`". That is a real defect in invoke, filed separately, and it is the
//! reason this proof stops at the predicate. A test that cannot go red is worse than
//! an honest gap.

use std::path::PathBuf;
use std::process::Command;

fn git(dir: &PathBuf, args: &[&str]) {
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

#[test]
fn an_untracked_record_that_stays_on_disk_reads_as_a_staged_deletion() {
    let dir = std::env::temp_dir().join(format!("vjs-unpub-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join(".vjs/orders")).unwrap();
    git(&dir, &["init", "-q"]);
    git(&dir, &["config", "user.email", "seed@vjs.test"]);
    git(&dir, &["config", "user.name", "seed"]);
    std::fs::write(dir.join(".vjs/orders/o.yaml"), "id: x\n").unwrap();
    std::fs::write(dir.join(".vjs/orders/kept.yaml"), "id: y\n").unwrap();
    git(&dir, &["add", "-A"]);
    git(
        &dir,
        &["-c", "core.hooksPath=/dev/null", "commit", "-qm", "seed"],
    );

    // The untracking shape: out of the index, still on disk. This is the state
    // CC-VJS 20 D2 authorised and the one the old gate could not tell apart from a
    // write.
    git(&dir, &["rm", "-q", "--cached", ".vjs/orders/o.yaml"]);
    assert!(
        dir.join(".vjs/orders/o.yaml").is_file(),
        "the whole point: untracking leaves the record on disk"
    );

    let deletions = vjs_git::GitIntegration::read_staged_deletions(&dir).expect("git answers");
    assert!(
        deletions.iter().any(|d| d == ".vjs/orders/o.yaml"),
        "an untracked-but-present record must read as a staged deletion, or the gate \
         goes on validating a record it is not being asked about: {deletions:?}"
    );
    assert!(
        !deletions.iter().any(|d| d == ".vjs/orders/kept.yaml"),
        "and a record nobody touched must NOT read as one, or the filter would excuse \
         every record in the tree: {deletions:?}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}
