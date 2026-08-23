//! The lawpack LOCK tests, split from `lawpack_resolution.rs` when the structural
//! ceiling ordered it (rustfmt rewraps pushed the file to 610 lines during the
//! 2026-08-05 fmt-debt clearance). Same split, same seam, as the subscribing
//! jurisdiction made first: resolution tests stay; the lock's write/verify/falsify
//! family lives here.

use std::path::Path;
use std::process::Command;

const VJS: &str = env!("CARGO_BIN_EXE_vjs");

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("vjs-lawpack-lock-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn real_lawpack() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .map(|a| a.join("lawpack/v2"))
        .find(|p| p.join("manifest.toml").is_file())
        .expect("these tests read the lawpack")
        .canonicalize()
        .expect("the lawpack path resolves once found")
}

fn run(repo: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(VJS)
        .args(args)
        .arg("--repo")
        .arg(repo)
        .env_remove("VJS_LAWPACK")
        .output()
        .unwrap();
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    (out.status.success(), text)
}

#[test]
fn the_lock_invoke_writes_is_the_one_validate_checks() {
    // THE DEFECT THIS CATCHES COST THE WHOLE FIRST IMPLEMENTATION OF THIS ORDER.
    //
    // `load_lawpack` and `compute_digest` existed TWICE - once in vjs-cli, once in
    // vjs-engine - as the same law expressed in two places. The ruling was implemented
    // against the CLI copy, every test above passed, and `vjs validate` went on computing
    // the digest the superseded way, reporting a Fatal LAWPACK_LOCK_DRIFT against the lock
    // `vjs invoke` had just written. Two copies of a rule are one copy and one silent
    // disagreement.
    //
    // Asserted BEHAVIOURALLY rather than by grepping for a second `fn compute_digest`: a
    // grep is satisfied by renaming, and what actually matters is that the artefact one
    // command writes is the artefact the other command accepts.
    let lawpack = real_lawpack();
    let repo = scratch("one-implementation");

    let (ok, text) = run(
        &repo,
        &[
            "invoke",
            "--jurisdiction",
            "t",
            "--principal",
            "p",
            "--lawpack",
            lawpack.to_str().unwrap(),
        ],
    );
    assert!(ok, "{text}");

    let (_, text) = run(&repo, &["validate"]);
    assert!(
        !text.contains("LAWPACK_LOCK_DRIFT"),
        "the digest invoke pinned is not the digest validate computes - there are two \
         implementations of the lawpack digest again:\n{text}"
    );

    // AND THE POSITIVE CONTROL, without which the assertion above proves nothing.
    //
    // [2026] VJS-CC-VJS 14 C6: this fixture resolves its lawpack OUT OF TREE, so it has no
    // vendored copy, so the guard that used to wrap the drift check was false here and this
    // test passed whatever the lock said. It asserted the ABSENCE of a finding in a repo
    // where the finding could not be produced. Falsify the digest: the absence above is only
    // evidence of agreement if the presence below is evidence of disagreement.
    falsify_lock_digest(&repo);
    let (ok, text) = run(&repo, &["validate"]);
    assert!(
        text.contains("LAWPACK_LOCK_DRIFT") && !ok,
        "a falsified lock digest must be caught, or the assertion above is vacuous:\n{text}"
    );
}

/// Overwrite the pinned digest in `.vjs/lawpack.lock` with one that is well-formed and wrong.
///
/// Well-formed on purpose: a lock that fails to PARSE exercises a different arm (see
/// `a_corrupt_lock_is_a_finding_and_never_an_ok`), and the arm under test here is the one
/// where the lock reads cleanly and simply disagrees with the law on disk.
fn falsify_lock_digest(repo: &Path) {
    let lock_path = repo.join(".vjs/lawpack.lock");
    let before = std::fs::read_to_string(&lock_path).unwrap();
    let after = before
        .lines()
        .map(|l| {
            if l.trim_start().starts_with("digest") {
                "digest = \"sha256:0000000000000000000000000000000000000000000000000000000000000000\""
                    .to_string()
            } else {
                l.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert_ne!(
        after,
        before,
        "the seed did not land: no digest line in {}",
        lock_path.display()
    );
    std::fs::write(&lock_path, format!("{after}\n")).unwrap();
}

#[test]
fn a_falsified_lock_digest_is_fatal_in_an_out_of_tree_jurisdiction() {
    // THE MEASURED DEFECT OF [2026] VJS-CC-VJS 14, as a test.
    //
    // Three checks - referential integrity, citation uniqueness and this one - hung off a
    // single condition keyed to a VENDORED lawpack directory, which a jurisdiction resolving
    // out of tree does not have. Before the fix this exact fixture printed `Validation: OK`
    // and exited 0 with a digest of all zeroes pinned in its lock. The guard was keyed to a
    // different referent than the gate, so the gate never ran.
    let lawpack = real_lawpack();
    let repo = scratch("falsified-lock");

    let (ok, text) = run(
        &repo,
        &[
            "invoke",
            "--jurisdiction",
            "t",
            "--principal",
            "p",
            "--lawpack",
            lawpack.to_str().unwrap(),
        ],
    );
    assert!(ok, "{text}");
    // The fixture must really be out of tree, or this passes for the wrong reason.
    assert!(
        !repo.join("lawpack/v2").exists(),
        "this test is only about a jurisdiction that vendors NO lawpack"
    );

    falsify_lock_digest(&repo);

    let (ok, text) = run(&repo, &["validate"]);
    assert!(
        !ok,
        "a falsified lock digest must exit non-zero, got success:\n{text}"
    );
    assert!(
        text.contains("[Fatal] LAWPACK_LOCK_DRIFT"),
        "ACT-007:s7 must be Fatal wherever the lawpack resolved:\n{text}"
    );
}

#[test]
fn a_corrupt_lock_is_a_finding_and_never_an_ok() {
    // [2026] VJS-CC-VJS 14 C5: no silent arm. The drift check was written as one `if let`
    // chain over `read_lawpack_lock` and `compute_digest`, so an unparseable lock or an
    // unreadable lawpack deleted the Fatal exactly as thoroughly as the missing guard did.
    // A check that did not run is not a check that passed, and the finding must say WHICH
    // half failed or an operator is left guessing between their lock and their canon.
    let lawpack = real_lawpack();
    let repo = scratch("corrupt-lock");

    let (ok, text) = run(
        &repo,
        &[
            "invoke",
            "--jurisdiction",
            "t",
            "--principal",
            "p",
            "--lawpack",
            lawpack.to_str().unwrap(),
        ],
    );
    assert!(ok, "{text}");

    std::fs::write(repo.join(".vjs/lawpack.lock"), "this is not toml = = =\n").unwrap();

    let (ok, text) = run(&repo, &["validate"]);
    assert!(!ok, "an unreadable lock must not validate OK:\n{text}");
    assert!(
        text.contains("LAWPACK_LOCK_UNREADABLE"),
        "the finding must name the LOCK as the half that failed:\n{text}"
    );
}
