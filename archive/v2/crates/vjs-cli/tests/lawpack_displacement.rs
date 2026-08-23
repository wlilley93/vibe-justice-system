//! [2026] VJS-CC-VJS 16: the directory the resolver prefers.
//!
//! C3, the vendored branch tests a DECLARATION and not a path; C5, displacement is its own
//! Fatal and the cure is not a re-pin; C6, the re-pin cannot ratify.
//!
//! These drive the BINARY, for the reason the CC-VJS 12 suite gives: the defect was never in
//! the resolving logic, it was in what a real command did over a real directory layout, and a
//! unit test over a helper would have passed on the broken code. A new file rather than more
//! of `lawpack_resolution.rs`, which is close to a 600-line ceiling that counts test sources.

use std::path::{Path, PathBuf};
use std::process::Command;

const VJS: &str = env!("CARGO_BIN_EXE_vjs");

/// A minimal but REAL `Order`. It must parse, or the loader errors and the fixture fails for
/// the wrong reason.
const SEEDED_ORDER: &str = "id: 2026-ACMECO-CC-999\ncourt: county\njurisdiction: acmeco\n\
repo_code: ACMECO\nstatus: binding\nissue: enforcement\nholding: fixture\ndirectives: []\n\
forbidden: null\nsource_opinion: null\ncreated_at: 2026-08-01T00:00:00Z\n";

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("vjs-displace-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn real_lawpack() -> PathBuf {
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

/// stdout and stderr together, for the human-readable assertions.
fn run(repo: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(VJS)
        .args(args)
        .arg("--repo")
        .arg(repo)
        // The env fallback is one of the three resolution sources AND, after C5, one of the
        // two recorded subscriptions, so it must be cleared or a developer with it set would
        // see these pass - or fail - for the wrong reason.
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

/// stdout alone, so `--json` output parses.
fn run_json(repo: &Path, args: &[&str]) -> serde_json::Value {
    let mut argv = args.to_vec();
    argv.push("--json");
    let out = Command::new(VJS)
        .args(&argv)
        .arg("--repo")
        .arg(repo)
        .env_remove("VJS_LAWPACK")
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("{argv:?} did not print a JSON report ({e}):\n{stdout}"))
}

/// A jurisdiction that vendors NOTHING and subscribes to the real canon out of tree - the
/// configuration the harm lives in.
fn subscriber(name: &str) -> (PathBuf, PathBuf) {
    let canon = real_lawpack();
    let repo = scratch(name);
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
        "the fixture must vendor nothing, or every assertion below passes for the wrong reason"
    );
    (repo, canon)
}

/// Seed a directory that DECLARES itself a lawpack, and assert it landed. It must declare:
/// after C3 an undeclared directory does not displace anything, so a fixture without a
/// manifest would be seeding nothing and the finding would be unreachable.
fn seed_a_declared_displacement(repo: &Path) -> PathBuf {
    let vendored = repo.join("lawpack/v2");
    std::fs::create_dir_all(vendored.join("orders")).unwrap();
    std::fs::write(
        vendored.join("manifest.toml"),
        "id = \"vjs-v2\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    std::fs::write(
        vendored.join("orders/2026-ACMECO-CC-999.yaml"),
        SEEDED_ORDER,
    )
    .unwrap();
    assert!(
        vendored.join("manifest.toml").is_file(),
        "the seed did not land"
    );
    vendored
}

fn codes(r: &serde_json::Value) -> Vec<String> {
    r["findings"]
        .as_array()
        .map(|a| {
            a.iter()
                .map(|f| f["code"].as_str().unwrap_or_default().to_string())
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn a_directory_a_verb_made_does_not_displace_a_recorded_subscription_but_a_declared_one_does() {
    let (repo, canon) = subscriber("declaration");
    let vendored = repo.join("lawpack/v2");

    // The state before, so the assertions below are a CHANGE and not a coincidence.
    let (ok, before) = run(&repo, &["lookup", "--issue", "enforcement"]);
    assert!(
        ok && before.contains("ACT-001"),
        "the canon must answer first:\n{before}"
    );

    // DIRECTION ONE. Seed exactly what the MCP `record` verb used to write: an order file
    // under the canon tree, and NO manifest. Assert the seed LANDED - a seed that silently
    // misses reads exactly like a dead gate.
    std::fs::create_dir_all(vendored.join("orders")).unwrap();
    std::fs::write(
        vendored.join("orders/2026-ACMECO-CC-999.yaml"),
        SEEDED_ORDER,
    )
    .unwrap();
    assert!(
        vendored.join("orders/2026-ACMECO-CC-999.yaml").is_file(),
        "the seed did not land"
    );
    assert!(
        !vendored.join("manifest.toml").exists(),
        "direction one is about a directory that declares NOTHING"
    );

    let (ok, text) = run(&repo, &["lookup", "--issue", "enforcement"]);
    assert!(ok, "{text}");
    assert!(
        text.contains("ACT-001"),
        "a directory a verb made must not displace the recorded subscription:\n{text}"
    );
    assert!(
        !text.contains("2026-ACMECO-CC-999"),
        "the undeclared directory must not be loaded as canon at all:\n{text}"
    );
    let (_, st) = run(&repo, &["status"]);
    assert!(
        st.contains(&canon.display().to_string()),
        "status must still name the subscribed canon:\n{st}"
    );

    // DIRECTION TWO, AND IT IS REQUIRED. Direction one alone would pass on a resolver that
    // had simply been switched to config-first, and it is the DECLARATION that must be doing
    // the work, not the file count. Write a manifest into the SAME directory: nothing else
    // changes, and the resolution must move.
    std::fs::write(
        vendored.join("manifest.toml"),
        "id = \"vjs-v2\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    assert!(
        vendored.join("manifest.toml").is_file(),
        "the second seed did not land"
    );

    let (ok, text) = run(&repo, &["lookup", "--issue", "enforcement"]);
    assert!(ok, "{text}");
    assert!(
        text.contains("2026-ACMECO-CC-999") && !text.contains("ACT-001"),
        "a vendored tree that DECLARES itself a lawpack must be preferred, or the predicate \
         is not a declaration test at all:\n{text}"
    );
}

#[test]
fn a_displaced_lawpack_is_its_own_fatal_and_the_cure_is_never_a_re_pin() {
    let (repo, canon) = subscriber("c5");

    // REACHABILITY FIRST. An assertion of absence proves nothing unless the fixture is one
    // where the finding is reachable ([2026] VJS-CC-VJS 14 obiter (i)), so the healthy state
    // is measured in the SAME repository that is about to be displaced.
    let healthy = run_json(&repo, &["validate"]);
    assert!(
        !codes(&healthy).contains(&"LAWPACK_DISPLACED".to_string()),
        "no displacement before there is one: {:?}",
        codes(&healthy)
    );

    let vendored = seed_a_declared_displacement(&repo);

    let report = run_json(&repo, &["validate"]);
    let findings = report["findings"].as_array().unwrap();
    let disp = findings
        .iter()
        .find(|f| f["code"] == "LAWPACK_DISPLACED")
        .unwrap_or_else(|| {
            panic!(
                "the displacement must be its own finding: {:?}",
                codes(&report)
            )
        });

    assert_eq!(disp["severity"], "fatal", "displacement is Fatal: {disp}");
    assert_eq!(
        report["ok"], false,
        "a displaced jurisdiction must not validate OK"
    );

    // IT NAMES BOTH SIDES OF THE DISAGREEMENT. A finding that names only one leaves the
    // operator with the problem the silent fallback left them with.
    let msg = disp["message"].as_str().unwrap();
    assert!(
        msg.contains(&canon.display().to_string()),
        "the finding must name the RECORDED subscription: {msg}"
    );
    assert!(
        msg.contains(&vendored.display().to_string()),
        "the finding must name the directory that ANSWERED: {msg}"
    );

    // THE ASSERTION THAT MATTERS. A displacement finding that still recommends a re-pin has
    // changed the label and kept the trap.
    let fix = disp["suggested_fix"]
        .as_str()
        .expect("a displacement finding must offer a cure");
    let lowered = fix.to_lowercase();
    assert!(
        lowered.contains("remove"),
        "the cure is to remove the directory that should not be there: {fix}"
    );
    assert!(
        !lowered.contains("re-pin") && !lowered.contains("repin"),
        "the drift finding's cure RATIFIES displacement - measured 2026-08-01, following it \
         returned exit 0 over a one-order directory and left the false declaration standing. \
         It must not be what is offered here: {fix}"
    );
}

#[test]
fn an_unnamed_re_pin_refuses_to_ratify_a_displacement_and_leaves_the_lock_untouched() {
    let (repo, _canon) = subscriber("c6");
    let lock_path = repo.join(".vjs/lawpack.lock");

    // THE HEALTHY DIRECTION FIRST, so the refusal below is proved to be about the displacement
    // and not about `invoke` having been made to refuse every second run - which would break
    // CI, whose first act in the apex is exactly this command.
    let (ok, text) = run(
        &repo,
        &["invoke", "--jurisdiction", "t", "--principal", "p"],
    );
    assert!(
        ok,
        "an unnamed re-pin in a healthy jurisdiction must still pin:\n{text}"
    );

    let before = std::fs::read(&lock_path).unwrap();
    let vendored = seed_a_declared_displacement(&repo);

    // THE MEASUREMENT AT 3.4, AS A TEST. Before C6 this exits 0, prints a digest, and the
    // jurisdiction goes green.
    let (ok, text) = run(
        &repo,
        &["invoke", "--jurisdiction", "t", "--principal", "p"],
    );
    assert!(
        !ok,
        "the re-pin must refuse over a source that contradicts the recorded lawpack_path:\n{text}"
    );
    assert!(
        text.contains(&vendored.display().to_string()),
        "the refusal must name the directory that should not be there:\n{text}"
    );
    assert_eq!(
        std::fs::read(&lock_path).unwrap(),
        before,
        "the lawpack lock must be byte-identical before and after a refused invocation - a \
         refusal that still rewrites the pin has certified the displacement anyway"
    );

    // AND THE FALSE GREEN MUST NOT FOLLOW. Without this the refusal could be satisfied by a
    // command that refuses and a validate that passes anyway, which is the state the ruling
    // is about.
    let (ok, text) = run(&repo, &["validate"]);
    assert!(
        !ok,
        "a displaced jurisdiction must not validate OK:\n{text}"
    );
    assert!(text.contains("LAWPACK_DISPLACED"), "{text}");

    // THE NAMED ESCAPE HATCH STILL WORKS: `--lawpack <path>` is the operator saying which tree,
    // in terms. Only the UNNAMED re-pin is refused.
    let (ok, text) = run(
        &repo,
        &[
            "invoke",
            "--jurisdiction",
            "t",
            "--principal",
            "p",
            "--lawpack",
            vendored.to_str().unwrap(),
        ],
    );
    assert!(ok, "a named --lawpack must still be honoured:\n{text}");
}

/// [2026] VJS-CC-VJS 16 C2 as a CLASS, not as one caller.
///
/// The first cure deleted the one write site that had the defect. That satisfied the grep
/// the condition names and left the class wide open: measured 2026-08-01, on a fresh repo
/// with no canon at all,
///
///     vjs audit --out <repo>/lawpack/v2/orders/probe.md
///
/// created `<repo>/lawpack/v2` and exited 0. The verb writes a REPORT; an operator-supplied
/// `--out` was the whole attack surface, and the compliance record asserted the rule held.
///
/// The condition says in terms that it is "stated as a class and not as one caller, because
/// the defect IS the class". A class needs a guard.
#[test]
fn no_operator_supplied_output_path_can_manufacture_the_canon_tree() {
    let repo = scratch("c2-class");
    let canon = repo.join("lawpack/v2");

    // Both verbs that take a caller-supplied --out and create its parent.
    let v1 = repo.join("v1-archive");
    std::fs::create_dir_all(&v1).unwrap();
    for (verb, extra, target) in [
        ("audit", vec![], "lawpack/v2/orders/probe.md"),
        (
            "migrate-v1",
            vec!["--v1-path", v1.to_str().unwrap()],
            "lawpack/v2/decisions/probe.yaml",
        ),
    ] {
        assert!(
            !canon.exists(),
            "the fixture must start with NO canon, or this proves nothing"
        );
        let out_arg = repo.join(target);
        let mut argv = vec![verb];
        argv.extend(extra);
        argv.extend(["--out", out_arg.to_str().unwrap()]);
        let (ok, text) = run(&repo, &argv);
        assert!(
            !ok,
            "`vjs {verb} --out` inside the canon tree must be REFUSED:\n{text}"
        );
        assert!(
            text.contains("canon tree"),
            "the refusal must say why:\n{text}"
        );
        assert!(
            !canon.exists(),
            "`vjs {verb}` created the directory the resolver reads the canon from"
        );
    }

    // THE NEGATIVE CONTROL. Without it this passes just as well on a verb that refuses
    // every --out, which would be a gate that cries wolf rather than one that discriminates.
    let out = repo.join("docs/conformance-map.md");
    let (ok, text) = run(&repo, &["audit", "--out", out.to_str().unwrap()]);
    assert!(
        ok,
        "an --out OUTSIDE the canon tree must still work:\n{text}"
    );
    assert!(out.is_file(), "and must actually write its report");
}
