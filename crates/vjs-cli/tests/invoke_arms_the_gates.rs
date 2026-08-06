//! A FRESH JURISDICTION MUST NOT COME UP WITH A GATE DISARMED.
//!
//! WHY. Measured 2026-08-06 on a scratch subscriber tree built exactly as the install
//! instructions describe: `vjs invoke --install-hooks` wrote the config, the lawpack
//! lock, the invocation record and the hooks, and then the very first `vjs validate`
//! reported
//!
//!   STORE-REGISTER-UNTRACKED: the store register DID NOT RUN - no register at
//!   <repo>/.vjs/store-register.yaml. This is a disclosure, not a pass.
//!
//! The disclosure was honest and the gate was off. That is the same defect class as a
//! check that cannot fail: the subscriber's day one had the ACT-PROCEEDINGS-DISCIPLINE
//! s13 store-register gate silently inert, and nothing in the install flow told them to
//! arm it. A governance kernel that ships its own gates switched off is not governing.
//!
//! The first cure armed the register but named `lawpack/v2` and two record roots that a
//! fresh tree does not have, so validate answered with three STORE-REGISTER-GHOST
//! warnings instead of one UNTRACKED disclosure. That is not a fix, it is the wolf-crying
//! failure wearing the fix's clothes: three warnings on day one is how a subscriber
//! learns this gate is noise. So the register names the lawpack actually subscribed to,
//! and invoke creates the record roots it registers.
//!
//! These tests therefore assert BOTH limbs. Either alone passes vacuously.

use std::path::Path;
use std::process::Command;

const VJS: &str = env!("CARGO_BIN_EXE_vjs");

fn real_lawpack() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .map(|a| a.join("lawpack/v2"))
        .find(|p| p.join("manifest.toml").is_file())
        .expect("these tests read the lawpack")
        .canonicalize()
        .expect("the lawpack path resolves once found")
}

/// A scratch git repo, standing in for a subscriber's own project.
fn subscriber_repo(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("vjs-invoke-arms-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    // SCRUB THE HOOK ENVIRONMENT: a fixture git run under a hook writes through GIT_DIR
    // into the real repository.
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
                .success(),
            "git {args:?} failed in the fixture"
        );
    }
    dir
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

fn invoke(repo: &Path) {
    let lawpack = real_lawpack();
    vjs(
        repo,
        &[
            "invoke",
            "--jurisdiction",
            "acme",
            "--principal",
            "Alice",
            "--lawpack",
            lawpack.to_str().unwrap(),
        ],
    );
}

/// LIMB ONE: the gate RUNS. A tree that has just been invoked must not answer
/// "DID NOT RUN" to the store-register check.
#[test]
fn a_freshly_invoked_jurisdiction_has_its_store_register_gate_armed() {
    let repo = subscriber_repo("armed");
    invoke(&repo);

    assert!(
        repo.join(".vjs/store-register.yaml").is_file(),
        "invoke must write the store register; without it the s13 gate is inert on day one"
    );

    let out = vjs(&repo, &["validate"]);
    assert!(
        !out.contains("STORE-REGISTER-UNTRACKED"),
        "the store-register gate reported DID NOT RUN on a freshly invoked tree. A \
         disclosure is not a pass, and a gate that ships disarmed is a gate that cannot \
         fail. Output:\n{out}"
    );
}

/// LIMB TWO: the gate runs CLEAN. Registering roots that do not exist trades one
/// disclosure for three ghost warnings and teaches the subscriber to ignore the gate.
#[test]
fn a_freshly_invoked_jurisdiction_raises_no_store_register_ghosts() {
    let repo = subscriber_repo("noghosts");
    invoke(&repo);

    let out = vjs(&repo, &["validate"]);
    assert!(
        !out.contains("STORE-REGISTER-GHOST"),
        "the register names a store this tree does not have. Three warnings on day one \
         is how a subscriber learns that this gate is noise. Output:\n{out}"
    );

    // The registered lawpack must be the one actually subscribed to, not a hardcoded
    // `lawpack/v2` that is only true for canon's own vendored layout.
    let register = std::fs::read_to_string(repo.join(".vjs/store-register.yaml")).unwrap();
    let lawpack = real_lawpack();
    assert!(
        register.contains(lawpack.to_str().unwrap()),
        "the register must name the subscribed lawpack. Register:\n{register}"
    );
    for root in [".vjs/orders", ".vjs/court"] {
        assert!(
            repo.join(root).is_dir(),
            "invoke registers {root}, so invoke must create it, or the register has a ghost"
        );
    }
}

/// A register the subscriber has curated is theirs. Invoke must never clobber it.
#[test]
fn invoke_never_overwrites_a_register_that_already_exists() {
    let repo = subscriber_repo("preserve");
    invoke(&repo);

    let mine = "stores: []\n# curated by the local Principal\n";
    std::fs::write(repo.join(".vjs/store-register.yaml"), mine).unwrap();
    invoke(&repo);

    assert_eq!(
        std::fs::read_to_string(repo.join(".vjs/store-register.yaml")).unwrap(),
        mine,
        "a second invoke overwrote the Principal's own register"
    );
}
