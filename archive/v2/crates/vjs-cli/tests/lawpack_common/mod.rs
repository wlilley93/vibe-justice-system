//! Shared helpers for the lawpack test binaries (`lawpack_resolution`, `lawpack_lock`).

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

pub const VJS: &str = env!("CARGO_BIN_EXE_vjs");

#[allow(dead_code)] // each test binary compiles this module; not every binary uses every helper
pub fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("vjs-lawpack-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// The real lawpack, found by SEARCHING UP from the crate rather than by a fixed relative
/// depth, so a repo move does not silently turn every one of these into the very defect
/// under test.
///
/// It did exactly that. The path was `../../lawpack/v2`, which is right in canon, where the
/// crates sit at the repository root. In a VENDORED tree the kernel lives one level deeper
/// (`opbox-kernel/governance/crates/...`) and the lawpack it governs is at
/// `opbox-kernel/lawpack/v2`, so the fixed depth resolved to `governance/lawpack/v2`, which
/// does not exist - and all NINE tests in this file died in the fixture with
/// `NotFound`. They have never executed in the subscribing jurisdiction: the tests written
/// to prove an unresolvable lawpack is refused were themselves unable to resolve the
/// lawpack, and the failure looked like nine broken tests rather than one wrong constant.
///
/// Searching upward is not a convenience. It resolves the lawpack the way the thing under
/// test does - by looking for it - instead of asserting where it ought to be.
#[allow(dead_code)] // each test binary compiles this module; not every binary uses every helper
pub fn real_lawpack() -> std::path::PathBuf {
    let start = Path::new(env!("CARGO_MANIFEST_DIR"));
    let found = start
        .ancestors()
        .map(|a| a.join("lawpack/v2"))
        .find(|p| p.join("manifest.toml").is_file())
        .unwrap_or_else(|| {
            panic!(
                "no lawpack/v2 with a manifest.toml above {} - these tests mean nothing \
                 without one, and a missing lawpack must fail HERE rather than be \
                 silently substituted",
                start.display()
            )
        });
    found
        .canonicalize()
        .expect("the lawpack path resolves once found")
}

#[allow(dead_code)] // each test binary compiles this module; not every binary uses every helper
pub fn run(repo: &Path, args: &[&str]) -> (bool, String) {
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

#[allow(dead_code)]
pub fn copy_tree(from: &Path, to: &Path) {
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

/// Overwrite the pinned digest in `.vjs/lawpack.lock` with one that is well-formed and wrong.
///
/// Well-formed on purpose: a lock that fails to PARSE exercises a different arm (see
/// `a_corrupt_lock_is_a_finding_and_never_an_ok`), and the arm under test here is the one
/// where the lock reads cleanly and simply disagrees with the law on disk.
#[allow(dead_code)] // each test binary compiles this module; not every binary uses every helper
pub fn falsify_lock_digest(repo: &Path) {
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
