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
    // FIND the lawpack rather than counting `../..` levels to it: the vendored layout
    // carries these crates one level deeper (under `governance/`) while the law does
    // not, so a counted path is layout-dependent and broke these tests in the
    // subscribing jurisdiction at the 2026-08-06 re-pull.
    let mut d = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    loop {
        let cand = d.join("lawpack/v2");
        if cand.join("manifest.toml").is_file() {
            return cand
                .canonicalize()
                .expect("the lawpack must exist for these tests to mean anything");
        }
        assert!(
            d.pop(),
            "no lawpack/v2 above CARGO_MANIFEST_DIR: these tests need one"
        );
    }
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
    assert!(
        !ok,
        "an unresolvable lawpack must FAIL, got success:\n{text}"
    );
    assert!(
        text.contains("no lawpack could be resolved"),
        "the refusal must say what could not be resolved:\n{text}"
    );
    // The three sources must be NAMED. A refusal that does not say where it looked leaves an
    // operator with the same problem the silent fallback left them with.
    for source in ["lawpack/v2", "lawpack_path", "VJS_LAWPACK"] {
        assert!(
            text.contains(source),
            "the refusal must name {source}:\n{text}"
        );
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
    assert!(
        ok,
        "a repo with no .vjs/config.toml must not be refused:\n{text}"
    );
}

#[test]
fn the_lawpack_flag_selects_what_is_loaded_and_refuses_a_path_that_does_not_resolve() {
    let repo = scratch("flag-selects");

    // D3, the failing direction. Before the ruling this SUCCEEDED and wrote the string into
    // config.toml and lawpack.lock as though a subscription had happened.
    let bad = repo.join("no-such-lawpack");
    let (ok, text) = run(
        &repo,
        &[
            "invoke",
            "--jurisdiction",
            "t",
            "--principal",
            "p",
            "--lawpack",
            bad.to_str().unwrap(),
        ],
    );
    assert!(
        !ok,
        "a --lawpack that does not resolve must be refused:\n{text}"
    );
    assert!(text.contains("does not resolve to a directory"), "{text}");
    assert!(
        !repo.join(".vjs/lawpack.lock").exists(),
        "a refused invocation must not leave a lock asserting the subscription"
    );

    // And the passing direction, in a repo that vendors NOTHING - which is the whole case.
    let lawpack = real_lawpack();
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
    assert!(
        ok,
        "a --lawpack naming a real lawpack must resolve:\n{text}"
    );

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
    assert_ne!(
        std::fs::read_to_string(&statute).unwrap(),
        original,
        "the seed did not land"
    );

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
    assert_eq!(
        d(&before),
        d(&digest(&repo)),
        "the digest is not reproducible"
    );
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
    assert!(ok);
    let (_, text2) = run(&repo2, &["status"]);
    assert!(
        !text2.contains("UNRESOLVED"),
        "status must not report a resolved lawpack as unresolved:\n{text2}"
    );
}

#[test]
fn a_relative_lawpack_path_is_recorded_as_given_so_a_clone_can_resolve_it() {
    // The config is COMMITTED. The first version canonicalised every path, so
    // `--lawpack ../vibe-justice-system/lawpack/v2` was written into the config as
    // `/home/<someone>/Projects/...` and every other clone resolved nothing - which, after
    // D1, is now a hard refusal rather than a silent empty canon, so the portability defect
    // and the fix for the silence would have collided in the worst way.
    let repo = scratch("relative-path");
    let lawpack = real_lawpack();
    // Sit the scratch repo beside the lawpack's grandparent so a relative path is expressible.
    let rel = {
        let root = lawpack.parent().unwrap().parent().unwrap();
        let mut p = std::path::PathBuf::from("..");
        p.push(root.file_name().unwrap());
        p.push("lawpack/v2");
        p
    };
    let sibling = lawpack
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(repo.file_name().unwrap());
    let _ = std::fs::remove_dir_all(&sibling);
    std::fs::create_dir_all(&sibling).unwrap();

    let (ok, text) = run(
        &sibling,
        &[
            "invoke",
            "--jurisdiction",
            "t",
            "--principal",
            "p",
            "--lawpack",
            rel.to_str().unwrap(),
        ],
    );
    assert!(ok, "a repo-relative lawpack must resolve:\n{text}");

    let config = std::fs::read_to_string(sibling.join(".vjs/config.toml")).unwrap();
    assert!(
        config.contains(&format!("lawpack_path = \"{}\"", rel.display())),
        "a relative path must be recorded AS GIVEN, not canonicalised into one machine's \
         home directory:\n{config}"
    );

    // And it must actually resolve through that recorded relative path.
    let (ok, text) = run(&sibling, &["lookup", "--issue", "enforcement"]);
    assert!(ok && text.contains("ACT-001"), "{text}");
    let _ = std::fs::remove_dir_all(&sibling);
}

#[test]
fn referential_integrity_runs_against_the_lawpack_that_was_actually_loaded() {
    // [2026] VJS-CC-VJS 14 C2. The scanned tree and the defined-id set must describe ONE
    // tree: the check took the vendored path while the `Lawpack` beside it had been loaded
    // from wherever `resolve_lawpack_dir` pointed, so in an out-of-tree jurisdiction it
    // scanned nothing and reported nothing. A dangling id in the canon this jurisdiction
    // actually subscribes to must be seen from a repo that vendors none of it.
    let src = real_lawpack();
    let elsewhere = scratch("dangling-canon");
    let canon = elsewhere.join("lawpack/v2");
    copy_tree(&src, &canon);

    // SEED, and assert it landed. ACT-999 is defined nowhere in the canon.
    let statute = canon.join("statutes/01-authority.yaml");
    let original = std::fs::read_to_string(&statute).unwrap();
    std::fs::write(
        &statute,
        format!("{original}\n# cites ACT-999 deliberately\n"),
    )
    .unwrap();
    assert_ne!(
        std::fs::read_to_string(&statute).unwrap(),
        original,
        "the seed did not land"
    );

    // A SEPARATE repo, which vendors nothing and resolves that copy through lawpack_path.
    let repo = scratch("dangling-subscriber");
    let (ok, text) = run(
        &repo,
        &[
            "invoke",
            "--jurisdiction",
            "t",
            "--principal",
            "p",
            "--lawpack",
            canon.to_str().unwrap(),
        ],
    );
    assert!(ok, "{text}");
    assert!(
        !repo.join("lawpack/v2").exists(),
        "the subscriber must vendor nothing"
    );

    let (_, text) = run(&repo, &["validate"]);
    assert!(
        text.contains("DANGLING_REFERENCE") && text.contains("ACT-999"),
        "referential integrity must read the lawpack that was loaded, not a vendored copy \
         that does not exist:\n{text}"
    );
}

#[test]
fn citation_uniqueness_runs_on_local_records_wherever_the_lawpack_lives() {
    // [2026] VJS-CC-VJS 14 C3. This check's referent is `front_door::governed_record_roots`
    // - the LOCAL records - which has nothing to do with where the lawpack resolved, so the
    // vendored-directory guard was not a narrow condition on it, it was the wrong condition
    // entirely. Two orders claiming one citation in an out-of-tree jurisdiction were fatal
    // nowhere. The register stays the local roots ([2026] VJS-CC-VJS 9 D1: the allocator and
    // the guard read the same register); only the condition is removed.
    let lawpack = real_lawpack();
    let repo = scratch("collision-out-of-tree");

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
    assert!(
        !repo.join("lawpack/v2").exists(),
        "the subscriber must vendor nothing"
    );

    // Two DISTINCT records claiming one citation. Distinct ids matter: two files sharing an
    // id are one record in two projections and are not a collision (CC-VJS 9 D2).
    std::fs::create_dir_all(repo.join(".vjs/orders")).unwrap();
    std::fs::write(
        repo.join(".vjs/orders/one.yaml"),
        "id: ORD-ONE\ncitation: \"[2026] VJS-CC-T 1\"\n",
    )
    .unwrap();
    std::fs::write(
        repo.join(".vjs/orders/two.yaml"),
        "id: ORD-TWO\ncitation: \"[2026] VJS-CC-T 1\"\n",
    )
    .unwrap();

    let (ok, text) = run(&repo, &["validate"]);
    assert!(
        !ok,
        "a citation collision is fatal (ACT-004:s8), so validate must fail:\n{text}"
    );
    assert!(
        text.contains("[Fatal] CITATION_COLLISION"),
        "two records claiming one citation must collide wherever the lawpack lives:\n{text}"
    );
}

#[test]
fn a_repo_that_is_not_a_jurisdiction_still_validates_clean() {
    // THE NEGATIVE CONTROL for the test above, and the limb that keeps the removed condition
    // from being replaced by a check that cries wolf. Citation uniqueness now runs
    // unconditionally, so it must be harmless where none of the three governed-record roots
    // exist: the scan is empty by construction, not by a guard.
    let repo = scratch("not-a-jurisdiction-validate");
    let (ok, text) = run(&repo, &["validate"]);
    assert!(
        ok,
        "a repo that is not a jurisdiction must validate clean:\n{text}"
    );
    assert!(text.contains("Validation: OK"), "{text}");
    for code in ["CITATION_COLLISION", "DANGLING_REFERENCE", "LAWPACK_LOCK"] {
        assert!(
            !text.contains(code),
            "no {code} where there are no records at all:\n{text}"
        );
    }
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
