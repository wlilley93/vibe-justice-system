//! [2026] VJS-CC-VJS 15: the Gazette resolves the canon like every other door, and the
//! artefact names the tree it published.
//!
//! `cmd_gazette` named `lawpack/v2` itself, so it never reached the CC-VJS 12 D1 refusal:
//! an invoked jurisdiction that vendored no copy published an EMPTY register and stamped
//! the pinned digest on it. The `?` that stood beside the literal looked like the guard and
//! was inert - every subtree read inside `LawpackLoader::load` is wrapped in `.exists()`, so
//! a missing directory returns Ok with empty vectors and there is nothing to propagate.
//!
//! These drive the BINARY, for the reason the CC-VJS 12 suite gives: the defect was never
//! in the resolving logic, it was in what a real command did with a `None` over a real
//! directory layout, and a unit test over a helper would have passed on the broken code.

use std::path::{Path, PathBuf};
use std::process::Command;

const VJS: &str = env!("CARGO_BIN_EXE_vjs");

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("vjs-gazette-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// The real lawpack, found from the crate rather than hard-coded, so a repo move does not
/// silently turn every one of these into the very defect under test.
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

/// A pinned lock, so the false-record shape is reproducible: a digest sitting in
/// `.vjs/lawpack.lock` is what the artefact used to publish regardless of what was read.
fn write_lock(repo: &Path) {
    std::fs::create_dir_all(repo.join(".vjs")).unwrap();
    std::fs::write(
        repo.join(".vjs/lawpack.lock"),
        "lawpack_id = \"vjs-v2@0.1.0\"\n\
         lawpack_version = \"0.1.0\"\n\
         digest = \"sha256:deadbeef\"\n\
         schema_version = 1\n",
    )
    .unwrap();
}

fn write_config(repo: &Path, lawpack_path: Option<&Path>) {
    std::fs::create_dir_all(repo.join(".vjs")).unwrap();
    let mut cfg = String::from(
        "version = \"2\"\njurisdiction_id = \"t\"\nrepo_code = \"T\"\n\
         lawpack = \"vjs-v2@0.1.0\"\nprincipal = \"p\"\n",
    );
    if let Some(p) = lawpack_path {
        cfg.push_str(&format!("lawpack_path = \"{}\"\n", p.display()));
    }
    std::fs::write(repo.join(".vjs/config.toml"), cfg).unwrap();
}

fn artefacts(repo: &Path) -> Vec<PathBuf> {
    [
        "gazette-data.js",
        "gazette-data.json",
        "gazette-text.js",
        "gazette.xml",
    ]
    .iter()
    .map(|f| repo.join(f))
    .filter(|p| p.exists())
    .collect()
}

fn data(repo: &Path) -> serde_json::Value {
    serde_json::from_str(&std::fs::read_to_string(repo.join("gazette-data.json")).unwrap()).unwrap()
}

/// C2 + C3, in ONE fixture and both directions, because either half alone proves nothing.
///
/// With `lawpack_path` the subscriber publishes the subscribed canon; REMOVE that one line
/// from the SAME fixture and the same command must refuse and write nothing. A refusal test
/// in a fixture where success is unreachable is a test that cannot fail (CC-VJS 14 obiter
/// (i)); a success test in a fixture where refusal is unreachable is the same defect wearing
/// the other hat.
#[test]
fn the_gazette_publishes_a_subscribed_canon_and_refuses_when_it_cannot_resolve_one() {
    let repo = scratch("subscriber");
    let lawpack = real_lawpack();
    write_lock(&repo);
    write_config(&repo, Some(&lawpack));

    // The reachable-opposite direction: a jurisdiction that vendors NOTHING publishes the
    // canon it subscribes to. Before the fix this produced an empty register.
    let (ok, text) = run(&repo, &["gazette"]);
    assert!(ok, "a resolvable out-of-tree lawpack must publish:\n{text}");
    assert!(
        !repo.join("lawpack/v2").exists(),
        "the subscriber must vendor nothing, or this proves nothing"
    );
    let d = data(&repo);
    let total = d["meta"]["counts"]["total"].as_u64().unwrap();
    assert!(total > 0, "the subscribed canon must reach the register:\n{d}");

    // Now flip the SAME fixture: drop the one line that resolves the lawpack.
    for f in artefacts(&repo) {
        std::fs::remove_file(f).unwrap();
    }
    write_config(&repo, None);
    let (ok, text) = run(&repo, &["gazette"]);
    assert!(
        !ok,
        "an invoked jurisdiction with no resolvable lawpack must be refused:\n{text}"
    );
    assert!(
        text.contains("no lawpack could be resolved"),
        "the refusal must say what could not be resolved:\n{text}"
    );
    // All three candidate sources NAMED, per CC-VJS 12 D1. A refusal that does not say
    // where it looked leaves an operator with the problem the silent fallback left them.
    for source in ["lawpack/v2", "lawpack_path", "VJS_LAWPACK"] {
        assert!(text.contains(source), "the refusal must name {source}:\n{text}");
    }
    assert!(
        artefacts(&repo).is_empty(),
        "a refused publication must write NO artefact: {:?}",
        artefacts(&repo)
    );
}

/// The limb CC-VJS 12 preserved, and the negative control for the refusal above: without
/// it, D1 could be satisfied by a Gazette that refuses everywhere.
#[test]
fn a_repo_that_is_not_a_jurisdiction_still_publishes_an_empty_register() {
    let repo = scratch("not-a-jurisdiction");
    let (ok, text) = run(&repo, &["gazette"]);
    assert!(ok, "a repo with no .vjs/config.toml must not be refused:\n{text}");
    assert_eq!(data(&repo)["meta"]["counts"]["total"], 0);
}

/// C4: the artefact NAMES the tree it published, and never wears a pin it did not earn.
///
/// Measured 2026-08-01: `digest = sha256:5481b9e2...` published beside `counts.total: 0`.
/// Nothing had been read, so the digest attested to a provenance that had not happened,
/// and a reader had no way to tell. Both directions run here so neither is vacuous: the
/// resolvable fixture must carry a digest, the empty one must not.
#[test]
fn the_artefact_names_the_tree_it_published_and_never_pins_an_empty_register() {
    let lawpack = real_lawpack();

    let published = scratch("meta-published");
    write_lock(&published);
    write_config(&published, Some(&lawpack));
    let (ok, text) = run(&published, &["gazette"]);
    assert!(ok, "{text}");
    let d = data(&published);
    let meta = &d["meta"]["lawpack"];
    assert_eq!(
        meta["source"], "config",
        "the artefact must record WHICH source answered:\n{meta}"
    );
    // WHERE TWO RULINGS MEET. [2026] VJS-CC-VJS 15 C4 required the artefact to record the
    // directory it actually read, and this assertion used to require the ABSOLUTE path.
    // [2026] VJS-CC-VJS 17 C4 then held that the publication surface publishes no absolute
    // path - the operator-account segment of a checkout path is itself a denylist entry, so
    // the Gazette refused ITSELF, and would have published a private repo path had it not
    // (ACT-005:s1 publish_private_repo_paths).
    //
    // C17 C4's own words are "a repo-relative directory, OR OMIT THE FIELD", so omission is
    // expressly authorised, and this fixture is the case that needs it: the lawpack is
    // subscribed OUT OF TREE, which is the live subscriber's configuration, and an
    // out-of-tree tree has no repo-relative form to publish.
    //
    // What survives of C15 C4 is that the artefact still records WHICH SOURCE answered
    // (asserted above) and still pins the DIGEST (asserted below), and the digest is the
    // tree's identity where the path is only its location. Whether that satisfies "records
    // the tree it read" for an out-of-tree subscriber is not mine to settle and is FILED.
    // This assertion is narrowed, not deleted: it still forbids the absolute path.
    assert!(
        meta["path"].is_null()
            || meta["path"]
                .as_str()
                .is_some_and(|p| !p.starts_with('/') && !p.contains(':')),
        "the artefact publishes a repo-relative directory or omits the field, never an \
         absolute checkout path ([2026] VJS-CC-VJS 17 C4):\n{meta}"
    );
    assert!(
        !meta["source"].is_null(),
        "and whichever way the path goes, the SOURCE that answered is still recorded \
         ([2026] VJS-CC-VJS 15 C4):\n{meta}"
    );
    assert!(
        d["meta"]["counts"]["total"].as_u64().unwrap() > 0,
        "the pinned direction must have something to pin, or the assertion below is vacuous"
    );
    assert!(
        !meta["digest"].is_null(),
        "a real publication carries its pin:\n{meta}"
    );

    // The false-record shape: a lock on disk, nothing published. Not a jurisdiction, so
    // this is the ONE surviving path to an empty register - the invoked one now refuses.
    let empty = scratch("meta-empty");
    write_lock(&empty);
    let (ok, text) = run(&empty, &["gazette"]);
    assert!(ok, "{text}");
    let d = data(&empty);
    assert_eq!(d["meta"]["counts"]["total"], 0);
    assert!(
        d["meta"]["lawpack"]["digest"].is_null(),
        "a non-null digest must NEVER appear beside counts.total == 0; it attests to a \
         reading that did not happen ([2026] VJS-CC-VJS 15 C4):\n{}",
        d["meta"]
    );
    assert!(
        d["meta"]["lawpack"]["source"].is_null() && d["meta"]["lawpack"]["path"].is_null(),
        "nothing resolved, so there is no source and no path to name:\n{}",
        d["meta"]
    );
}
