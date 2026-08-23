//! Red-seed tests for the route/permit ergonomics (Operation Watertight WS2), each
//! seeded from a measured 2026-08-05 failure:
//! - a globless `--path governance/crates` minted a permit whose scope covered
//!   NOTHING, silently (Defect 3 of the two-defects submission);
//! - a fresh issue tag drew CourtRequired on work settled published law governed,
//!   and one such misroute cost a full court sitting a citation would have disposed.

mod lawpack_common;
use lawpack_common::{copy_tree, real_lawpack, run, scratch};
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    real_lawpack()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

/// A minimal routable jurisdiction: the real lawpack, one governed directory.
fn route_fixture(name: &str) -> PathBuf {
    let dir = scratch(name);
    copy_tree(&real_lawpack(), &dir.join("lawpack/v2"));
    std::fs::create_dir_all(dir.join("widgets")).unwrap();
    std::fs::write(dir.join("widgets/one.rs"), "// governed file\n").unwrap();
    dir
}

#[test]
fn a_bare_directory_path_is_normalised_to_a_glob_out_loud() {
    let dir = route_fixture("route-dirnorm");
    let (ok, out) = run(
        &dir,
        &[
            "route",
            "--kind",
            "governed-load-bearing-act",
            "--risk",
            "low",
            "--intent",
            "test the directory normalisation",
            "--path",
            "widgets",
        ],
    );
    assert!(ok, "{out}");
    assert!(
        out.contains("is a directory") && out.contains("widgets/**"),
        "the normalisation is announced, not silent: {out}"
    );
    // The permit on disk carries the GLOB, not the bare literal that covers nothing.
    let permits = dir.join(".vjs/permits");
    let minted = std::fs::read_dir(&permits)
        .expect("a permit was minted")
        .flatten()
        .map(|e| std::fs::read_to_string(e.path()).unwrap())
        .collect::<String>();
    assert!(minted.contains("widgets/**"), "scope is the glob: {minted}");
}

#[test]
fn a_scope_covering_no_existing_path_is_refused_at_mint() {
    let dir = route_fixture("route-zerocover");
    let (ok, out) = run(
        &dir,
        &[
            "route",
            "--kind",
            "governed-load-bearing-act",
            "--risk",
            "low",
            "--intent",
            "test the zero-coverage refusal",
            "--path",
            "no/such/tree/**",
        ],
    );
    assert!(!ok, "a permit covering nothing must not mint: {out}");
    assert!(
        out.contains("covers no existing path") && out.contains("not a permit"),
        "{out}"
    );
    // And nothing landed on disk with that scope.
    if let Ok(rd) = std::fs::read_dir(dir.join(".vjs/permits")) {
        for e in rd.flatten() {
            let content = std::fs::read_to_string(e.path()).unwrap();
            assert!(
                !content.contains("no/such/tree/**"),
                "the refused scope must not be persisted: {content}"
            );
        }
    }
}

#[test]
fn a_court_required_refusal_surfaces_the_nearest_settled_law() {
    // Read-only against the real corpus: CourtRequired mints nothing and writes
    // nothing; the suggestion block is stderr text. The intent is chokepoint-adjacent
    // on purpose - the refusal must surface PC-13's issue tag so settled law is found
    // by reading the refusal instead of convening a sitting.
    let (ok, out) = run(
        &repo_root(),
        &[
            "route",
            "--kind",
            "governed-load-bearing-act",
            "--risk",
            "medium",
            "--issue",
            "a-completely-fresh-tag-for-this-test",
            "--intent",
            "kernel enforcement gaps at the canon write chokepoint: gates for kernel doors",
        ],
    );
    assert!(ok, "{out}");
    assert!(out.contains("Court required: true"), "{out}");
    assert!(
        out.contains("NEAREST KNOWN issue tags")
            && out.contains("--issue kernel_enforcement_gaps_canon_write_chokepoint"),
        "the refusal cites its way to settled law: {out}"
    );
}
