//! [2026] VJS-CC-VJS 16 C5: the tree I loaded is not the tree I said I subscribe to.
//!
//! DRIFT and DISPLACEMENT are findings about different things. Drift is a statement about
//! BYTES: the tree I loaded is not the tree I pinned. Displacement is a statement about
//! IDENTITY. The jurisdiction's own record of its subscription - `lawpack_path` in
//! `.vjs/config.toml` - survives displacement untouched, still true as a declaration and
//! false as a description, and until this file nothing in the kernel compared the two. That
//! comparison IS the finding, and it is cheap: `resolve_lawpack` already returns which source
//! answered ([2026] VJS-CC-VJS 15 C4) and only the Gazette consumed it.
//!
//! IT NEEDED ITS OWN FINDING ON THREE GROUNDS. (i) The existing finding's remedy is wrong for
//! it: measured 2026-08-01, following LAWPACK_LOCK_DRIFT's own suggested fix on a displaced
//! jurisdiction re-pinned the lock over the displacing directory, `validate` returned OK exit
//! 0, `lookup` still returned only the fixture order, and the config's `lawpack_path` was left
//! standing and false. A cure that completes the disease is not a detection, it is a trap.
//! (ii) In the field the two are already confounded: the live subscriber showed drift for an
//! entirely legitimate reason, and an operator in that state reading "re-pin the lock" has no
//! way to tell whether the re-pin will subscribe them to the canon or to a directory a verb
//! made. (iii) The harm is not a stale pin. Displacement silences the canon-boundary gate
//! entirely - `resolve_canon_repo_code` reads a manifest the manufactured directory does not
//! have and falls back to the subscriber's own code, so against a canon that declares nothing
//! everything local is native - and evaluates zero invariants while reporting "all passed".
//!
//! KEYED ON THE CONFIG AND THE RESOLUTION, NOT ON THE FEDERATION SUBSCRIBER REGISTRY. That
//! registry lists only a fixture code and does not name the one real subscriber ([2026]
//! VJS-CC-VJS 14 obiter (iv)). A gate keyed to that list would be born not reaching the
//! repository that needs it.

use std::path::{Path, PathBuf};

use vjs_core::report::Finding;
use vjs_core::types::Severity;

use crate::resolver::{
    LawpackSource, env_lawpack_path, lawpack_path_from_config, resolve_lawpack,
};

/// DISTINCT FROM `LAWPACK_LOCK_DRIFT`, and it must stay distinct: a displacement finding that
/// is a severity of drift inherits drift's cure, and drift's cure ratifies displacement.
pub const DISPLACEMENT_CODE: &str = "LAWPACK_DISPLACED";

const CONFIG_SUBSCRIPTION: &str = "lawpack_path in .vjs/config.toml";
const ENV_SUBSCRIPTION: &str = "$VJS_LAWPACK";

/// A recorded subscription and a resolution that disagree.
#[derive(Clone, Debug)]
pub struct Displacement {
    /// The subscription as this jurisdiction RECORDED it, verbatim.
    pub recorded: String,
    /// Which record carries it.
    pub recorded_by: &'static str,
    /// The directory that actually answered.
    pub answered_from: PathBuf,
    /// Which of the three sources it answered from.
    pub answered_source: LawpackSource,
}

/// Where a subscription is recorded and `resolve_lawpack` answered from somewhere else.
///
/// Shared with `vjs invoke`, which refuses to re-pin over one (C6). One detection, so the
/// finding and the refusal can never come to describe different states.
pub fn detect(repo: &Path) -> Option<Displacement> {
    let configured = lawpack_path_from_config(repo);
    let env = env_lawpack_path();

    // The recorded subscription: the COMMITTED record first. The config is the jurisdiction's
    // own standing statement of what it subscribes to; the environment is one process's.
    let (recorded, recorded_by) = match (&configured, &env) {
        (Some(p), _) => (p.display().to_string(), CONFIG_SUBSCRIPTION),
        (None, Some(p)) => (p.display().to_string(), ENV_SUBSCRIPTION),
        // Nothing is recorded, so there is no declaration for the resolution to contradict.
        // A repository that vendors and subscribes to nothing else keeps today's behaviour
        // exactly - the same limb C3 leaves untouched.
        (None, None) => return None,
    };

    let resolution = resolve_lawpack(repo)?;

    // DISPLACEMENT IS A DISAGREEMENT BETWEEN TWO DIRECTORIES, NOT BETWEEN TWO LABELS.
    //
    // Keying this on `resolution.source` alone was wrong and an existing test caught it:
    // `the_pinned_digest_moves_when_a_statute_moves` vendors the canon AND records that same
    // directory as its `lawpack_path`, so the resolver answers `Vendored` while the config
    // points at precisely the tree that answered. There is no disagreement there, and calling
    // it one would fire a Fatal on a correct jurisdiction - a gate that cries wolf gets
    // switched off, which costs more than the gate was ever worth.
    //
    // Compared as CANONICAL paths, so `./lawpack/v2`, an absolute path and a symlinked route
    // to the same directory are one answer. Where a side cannot be canonicalised (it does not
    // exist), the raw path is used: an unresolvable recorded subscription is a real
    // disagreement with whatever did answer, and it must not be silently forgiven.
    let same = |a: &Path, b: &Path| -> bool {
        match (a.canonicalize(), b.canonicalize()) {
            (Ok(x), Ok(y)) => x == y,
            _ => a == b,
        }
    };
    let recorded_dirs: Vec<PathBuf> = [
        configured
            .as_ref()
            .map(|p| if p.is_absolute() { p.clone() } else { repo.join(p) }),
        env.clone(),
    ]
    .into_iter()
    .flatten()
    .collect();
    if recorded_dirs.iter().any(|d| same(d, &resolution.dir)) {
        return None;
    }

    Some(Displacement {
        recorded,
        recorded_by,
        answered_from: resolution.dir,
        answered_source: resolution.source,
    })
}

/// The Fatal, naming BOTH sides of the disagreement.
pub fn check(repo: &Path) -> Vec<Finding> {
    let Some(d) = detect(repo) else {
        return Vec::new();
    };
    vec![
        Finding::new(
            Severity::Fatal,
            DISPLACEMENT_CODE,
            format!(
                "This jurisdiction records a subscription to '{}' ({}), but its lawpack was \
                 loaded from '{}' ({:?}). The tree that answered is NOT the tree this \
                 repository says it subscribes to. This is not a stale pin: every canon-sourced \
                 check now reads the displacing directory, so the canon-boundary gate falls \
                 back to this repository's own repo_code and stops seeing a subscriber's law \
                 masquerading as canon, and canon-sourced invariants evaluate over an empty set \
                 and report as passed.",
                d.recorded,
                d.recorded_by,
                d.answered_from.display(),
                d.answered_source,
            ),
        )
        .citing("ACT-007:s3")
        .fix(format!(
            "Remove '{}'. It is a directory that should not be there, and while it exists the \
             resolver will go on preferring it. Regenerating the lawpack lock is NOT the cure \
             here and must not be attempted: a fresh pin would certify the displacing tree under \
             the recorded lawpack's id, return exit 0, and leave the recorded subscription \
             standing and false ([2026] VJS-CC-VJS 16).",
            d.answered_from.display()
        )),
    ]
}
