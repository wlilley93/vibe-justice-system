//! [2026] VJS-CC-VJS 12: an invoked jurisdiction whose lawpack cannot be resolved is an
//! error, never an empty canon.
//!
//! These drive the BINARY rather than the functions, deliberately. `load_lawpack` and
//! `resolve_lawpack_dir` are `pub(crate)` inside a bin crate, and the defect they fix was
//! never in the resolving logic - it was in what the kernel did with a `None`, which only
//! shows up once a real command has run over a real directory layout. A unit test over a
//! helper would have passed on the broken code.

use std::path::Path;
use std::process::Command;

const VJS: &str = env!("CARGO_BIN_EXE_vjs");

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("vjs-lawpack-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// The real lawpack, found from the crate rather than hard-coded, so a repo move does not
/// silently turn every one of these into the very defect under test.
fn real_lawpack() -> std::path::PathBuf {
    let p = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../lawpack/v2")
        .canonicalize()
        .expect("the lawpack must exist for these tests to mean anything");
    assert!(p.join("manifest.toml").is_file(), "not a lawpack: {}", p.display());
    p
}

fn run(repo: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(VJS)
        .args(args)
        .arg("--repo")
        .arg(repo)
        // The env fallback is one of the three resolution sources, so it must be cleared or
        // a developer with it set would see these pass for the wrong reason.
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
fn an_invoked_jurisdiction_with_no_resolvable_lawpack_is_refused_not_emptied() {
    let repo = scratch("invoked-empty");
    // A jurisdiction is `.vjs/config.toml`, and nothing else. Written by hand here so the
    // test does not depend on invoke succeeding, which is a different rule (D2).
    std::fs::create_dir_all(repo.join(".vjs")).unwrap();
    std::fs::write(
        repo.join(".vjs/config.toml"),
        "version = \"2\"\njurisdiction_id = \"t\"\nrepo_code = \"T\"\nlawpack = \"vjs-v2@0.1.0\"\nprincipal = \"p\"\n",
    )
    .unwrap();

    let (ok, text) = run(&repo, &["lookup", "--issue", "enforcement"]);
    assert!(!ok, "an unresolvable lawpack must FAIL, got success:\n{text}");
    assert!(
        text.contains("no lawpack could be resolved"),
        "the refusal must say what could not be resolved:\n{text}"
    );
    // The three sources must be NAMED. A refusal that does not say where it looked leaves an
    // operator with the same problem the silent fallback left them with.
    for source in ["lawpack/v2", "lawpack_path", "VJS_LAWPACK"] {
        assert!(text.contains(source), "the refusal must name {source}:\n{text}");
    }
}

#[test]
fn a_repo_that_is_not_a_jurisdiction_still_runs_on_an_empty_canon() {
    // THE LIMB THE ORDER PRESERVED, and the negative control for the test above. Without
    // this, D1 could have been satisfied by a kernel that refuses everywhere, which would
    // break `invoke` itself and every command in an uninvoked checkout. The distinction is
    // acquisition: a repo acquires ORDERS by operating and never acquires a LAWPACK that way.
    let repo = scratch("not-a-jurisdiction");
    let (ok, text) = run(&repo, &["status"]);
    assert!(ok, "a repo with no .vjs/config.toml must not be refused:\n{text}");
}

#[test]
fn the_lawpack_flag_selects_what_is_loaded_and_refuses_a_path_that_does_not_resolve() {
    let repo = scratch("flag-selects");

    // D3, the failing direction. Before the ruling this SUCCEEDED and wrote the string into
    // config.toml and lawpack.lock as though a subscription had happened.
    let bad = repo.join("no-such-lawpack");
    let (ok, text) = run(
        &repo,
        &["invoke", "--jurisdiction", "t", "--principal", "p", "--lawpack", bad.to_str().unwrap()],
    );
    assert!(!ok, "a --lawpack that does not resolve must be refused:\n{text}");
    assert!(text.contains("does not resolve to a directory"), "{text}");
    assert!(
        !repo.join(".vjs/lawpack.lock").exists(),
        "a refused invocation must not leave a lock asserting the subscription"
    );

    // And the passing direction, in a repo that vendors NOTHING - which is the whole case.
    let lawpack = real_lawpack();
    let (ok, text) = run(
        &repo,
        &["invoke", "--jurisdiction", "t", "--principal", "p", "--lawpack", lawpack.to_str().unwrap()],
    );
    assert!(ok, "a --lawpack naming a real lawpack must resolve:\n{text}");

    let config = std::fs::read_to_string(repo.join(".vjs/config.toml")).unwrap();
    assert!(
        config.contains(&format!("lawpack_path = \"{}\"", lawpack.display())),
        "invoke must record WHERE the lawpack resolved, not only what it was called:\n{config}"
    );

    // The flag now SELECTS: the canon is reachable from a repo that vendors none of it.
    let (ok, text) = run(&repo, &["lookup", "--issue", "enforcement"]);
    assert!(ok, "{text}");
    assert!(
        text.contains("ACT-001"),
        "a resolved lawpack must answer the query that returned silence before:\n{text}"
    );
}

#[test]
fn the_pinned_digest_moves_when_a_statute_moves() {
    // D4. The old digest hashed manifest.toml alone, so every statute in the canon sat
    // outside the pin. Measured 2026-07-31: appending to statutes/01-authority.yaml left it
    // at 14cdb3337039ffdb, byte-identical to the digest this repository had pinned since
    // 07-27. A pin that cannot move when the law moves is not a pin.
    let src = real_lawpack();
    let repo = scratch("digest-tree");
    let copy = repo.join("lawpack/v2");
    copy_tree(&src, &copy);

    let digest = |repo: &Path| -> String {
        let (ok, text) = run(repo, &["invoke", "--jurisdiction", "t", "--principal", "p"]);
        assert!(ok, "{text}");
        std::fs::read_to_string(repo.join(".vjs/lawpack.lock")).unwrap()
    };

    let before = digest(&repo);

    // SEED, and assert it landed. A seed that silently misses reads exactly like a dead gate.
    let statute = copy.join("statutes/01-authority.yaml");
    let original = std::fs::read_to_string(&statute).unwrap();
    std::fs::write(&statute, format!("{original}\n# seeded\n")).unwrap();
    assert_ne!(std::fs::read_to_string(&statute).unwrap(), original, "the seed did not land");

    let after = digest(&repo);
    let d = |lock: &str| {
        lock.lines()
            .find(|l| l.starts_with("digest"))
            .unwrap()
            .to_string()
    };
    assert_ne!(
        d(&before),
        d(&after),
        "a statute changed and the pinned digest did not move"
    );

    // Reversible, so the digest is a function of the bytes and not of the order of runs.
    std::fs::write(&statute, &original).unwrap();
    assert_eq!(d(&before), d(&digest(&repo)), "the digest is not reproducible");
}

#[test]
fn status_says_when_no_canon_is_loaded() {
    // D6, the aggravating fact. status printed "VJS installed: true" and named a lawpack in
    // a jurisdiction where lookup returned nothing at all.
    let repo = scratch("status-unresolved");
    std::fs::create_dir_all(repo.join(".vjs")).unwrap();
    std::fs::write(
        repo.join(".vjs/config.toml"),
        "version = \"2\"\njurisdiction_id = \"t\"\nrepo_code = \"T\"\nlawpack = \"vjs-v2@0.1.0\"\nprincipal = \"p\"\n",
    )
    .unwrap();

    let (_, text) = run(&repo, &["status"]);
    assert!(
        text.contains("UNRESOLVED"),
        "status must not name a lawpack it cannot resolve as though it had one:\n{text}"
    );

    // The negative control: the same line must NOT cry wolf where a canon IS loaded.
    let lawpack = real_lawpack();
    let repo2 = scratch("status-resolved");
    let (ok, _) = run(
        &repo2,
        &["invoke", "--jurisdiction", "t", "--principal", "p", "--lawpack", lawpack.to_str().unwrap()],
    );
    assert!(ok);
    let (_, text2) = run(&repo2, &["status"]);
    assert!(
        !text2.contains("UNRESOLVED"),
        "status must not report a resolved lawpack as unresolved:\n{text2}"
    );
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
        &["invoke", "--jurisdiction", "t", "--principal", "p", "--lawpack", lawpack.to_str().unwrap()],
    );
    assert!(ok, "{text}");

    let (_, text) = run(&repo, &["validate"]);
    assert!(
        !text.contains("LAWPACK_LOCK_DRIFT"),
        "the digest invoke pinned is not the digest validate computes - there are two \
         implementations of the lawpack digest again:\n{text}"
    );
}

fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let dest = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &dest);
        } else {
            std::fs::copy(entry.path(), dest).unwrap();
        }
    }
}
