//! WHERE THE LAW IS. The one resolution of a jurisdiction's lawpack, and nothing else.
//!
//!
//! Until [2026] VJS-CC-VJS 12 there were TWO implementations of `load_lawpack` and
//! `compute_digest`: one in this crate and one in `vjs-cli/src/context.rs`, byte-for-byte
//! the same law expressed twice. The ruling was implemented against the CLI copy and the
//! repository's own `vjs validate` gate went on reading THIS one, so the fix passed every
//! test and the gate still enforced the old rule - it reported LAWPACK_LOCK_DRIFT against a
//! digest computed the superseded way. Two copies of a rule are one copy and one silent
//! disagreement. The CLI now delegates here.
//!
//! Lifted out of `lib.rs` when [2026] VJS-CC-VJS 16 C3 and C5 pushed that file past the
//! 600-line structural ceiling machine-checked by `crates/vjs-testkit/tests/structural_ceiling.rs`.
//! A MOVE, not a redesign, apart from the declaration predicate on the vendored branch and
//! the empty-env filter, both marked below.

use std::path::{Path, PathBuf};

use vjs_core::KernelError;

/// Whether this repository has been invoked as a VJS jurisdiction.
///
/// The gate is `.vjs/config.toml` and not `.vjs/`, deliberately. `vjs invoke` creates
/// `.vjs/invocation/` BEFORE it writes the config, so keying on the directory would make
/// invocation refuse itself half-way through its own first run.
pub fn is_invoked_jurisdiction(repo: &Path) -> bool {
    repo.join(".vjs/config.toml").exists()
}

/// WHICH of the three candidate sources answered. Recorded rather than inferred, so an
/// artefact can name the tree it was built from ([2026] VJS-CC-VJS 15 C4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LawpackSource {
    Vendored,
    Config,
    Env,
}

/// A resolved lawpack: the directory, and which source produced it.
#[derive(Clone, Debug)]
pub struct LawpackResolution {
    pub source: LawpackSource,
    pub dir: PathBuf,
}

/// Does this directory DECLARE itself a lawpack?
///
/// The system already HAS the canon's self-declaration and already relies on it:
/// `manifest.toml`, read by `resolve_canon_repo_code` and by `lawpack_id_of`. Keying the
/// resolver on a declaration the tree already carries is strictly narrower than minting a
/// provenance file, which would be a second source of truth about the canon. READABLE is
/// the whole test, exactly as [2026] VJS-CC-VJS 16 C3 words it: a manifest that does not
/// parse is a DEFECTIVE lawpack, which is a different finding, and making resolution depend
/// on parsing would let a one-character typo silently unsubscribe a jurisdiction.
fn declares_a_lawpack(dir: &Path) -> bool {
    std::fs::read_to_string(dir.join("manifest.toml")).is_ok()
}

/// `$VJS_LAWPACK`, treating an empty value as unset.
///
/// A shell that exports the variable empty is not naming a lawpack. It never won resolution
/// (an empty path is not a directory), but after C3 it would otherwise count as "another
/// source is recorded" and start requiring a manifest on the vendored branch of every
/// repository whose shell happened to carry it - a behaviour change in exactly the
/// configuration the ruling says it leaves alone. `var_os` and not `var`: the K-26 fence
/// (`compile_time_and_drift.rs`) forbids `env::var(` anywhere in this crate.
pub fn env_lawpack_path() -> Option<PathBuf> {
    std::env::var_os("VJS_LAWPACK")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
}

/// Where this repository's lawpack actually is, and which source answered, or `None`.
///
/// Three sources, most specific first. Until [2026] VJS-CC-VJS 12 there was ONE source -
/// `<repo>/lawpack/v2` - and a repository that did not VENDOR a copy of the canon silently
/// resolved against nothing. Measured 2026-07-31: `vjs lookup --issue enforcement` returned
/// four constitutional sections in `vibe-justice-system` and no output at all in
/// `vibe-design-system`, same binary, same flags, neither answer marked.
///
/// THIS IS THE ONLY PLACE IN THE WORKSPACE THAT MAY NAME THE LAWPACK IN ORDER TO READ THE
/// CANON ([2026] VJS-CC-VJS 15). Every other canon-read site takes the directory from here,
/// so the D1 refusal reaches every door, not only the callers of `load_lawpack`.
pub fn resolve_lawpack(repo: &Path) -> Option<LawpackResolution> {
    // LAWPACK-LITERAL: referent=resolver; status=reserved; authority=[2026] VJS-CC-VJS 15.
    // The one canon-read literal the ruling leaves standing: collapse it and the resolver
    // has nothing to resolve.
    let vendored = repo.join("lawpack/v2");
    // Relative paths resolve against the repo, so a config is portable between clones.
    let configured =
        lawpack_path_from_config(repo).map(|p| if p.is_absolute() { p } else { repo.join(p) });
    let env = env_lawpack_path();

    // THE VENDORED BRANCH TESTS A DECLARATION, NOT A PATH ([2026] VJS-CC-VJS 16 C3).
    //
    // Measured 2026-08-01: the MCP `record` verb created an order file under the canon tree
    // in a jurisdiction that subscribed to the canon out of tree. The directory now existed,
    // this branch preferred it BECAUSE IT EXISTED, and a 160-file constitutional canon was
    // replaced by a one-file directory - silently, and reported by `vjs status` as a
    // subscription to `vjs-v2@0.1.0`. A directory a verb made is not a canon.
    //
    // NARROW ON PURPOSE. The declaration is required only where the vendored candidate would
    // DISPLACE a recorded subscription. Where nothing else is recorded the branch is
    // unchanged, so CC-VJS 12 D1's hard refusal is not widened by one repository and no
    // existing fixture moves.
    //
    // This does NOT decide whether vendored-before-config is the right order where BOTH
    // sources declare; that question is expressly left open, and where both declare the
    // vendored copy still wins here and `validate` reports the disagreement (C5).
    let another_source_is_recorded = configured.is_some() || env.is_some();
    let vendored = if another_source_is_recorded && !declares_a_lawpack(&vendored) {
        None
    } else {
        Some(vendored)
    };

    // Most specific first; the first candidate that is a real directory wins.
    [
        (LawpackSource::Vendored, vendored),
        (LawpackSource::Config, configured),
        (LawpackSource::Env, env),
    ]
    .into_iter()
    .find_map(|(source, dir)| {
        let dir = dir.filter(|d| d.is_dir())?;
        Some(LawpackResolution { source, dir })
    })
}

/// The directory alone. A projection of `resolve_lawpack`, never a second resolution.
pub fn resolve_lawpack_dir(repo: &Path) -> Option<PathBuf> {
    resolve_lawpack(repo).map(|r| r.dir)
}

/// Refuse a write whose target lies inside the canon tree ([2026] VJS-CC-VJS 16 D2 / C2).
///
/// C2 is stated as a CLASS - "no kernel write path may bring into being the directory the
/// resolver reads the canon from" - and a class needs a guard, not a per-caller habit. The
/// first cure deleted the one caller that had the defect and left the class open: measured
/// 2026-08-01 on a fresh repo with no canon, `vjs audit --out <repo>/lawpack/v2/orders/probe.md`
/// created `<repo>/lawpack/v2` and exited 0. The verb is a REPORT WRITER, not an authoring
/// act, and an operator-supplied `--out` was all it took. Nothing in the tree stopped it,
/// and the compliance record asserted the rule held.
///
/// Compared on the LEXICAL path (both sides normalised through `absolutise`), because the
/// target usually does not exist yet - which is the whole point - so `canonicalize` cannot
/// see it. The canon root is `<repo>/lawpack/v2`, the vendored candidate the resolver
/// prefers; a configured out-of-tree canon belongs to another repository and is not this
/// repository's to write through a `--out` flag either, so it is refused on the same terms
/// when it is named.
///
/// The exception CC-VJS 16 preserves is a DELIBERATE PERMITTED AUTHORING ACT in the
/// repository that owns the canon. This guard is deliberately not applied to those: it
/// guards the verbs that take a caller-supplied output path and would otherwise create the
/// directory as a side effect of writing a report.
pub fn refuse_write_into_canon_tree(repo: &Path, target: &Path) -> Result<(), KernelError> {
    let absolutise = |p: &Path| -> PathBuf {
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            repo.join(p)
        }
    };
    let target = absolutise(target);
    // LAWPACK-LITERAL: referent=write-target; status=reserved; authority=[2026] VJS-CC-VJS 16.
    // This names the canon root as a place NOT to write. It is the one literal whose whole
    // purpose is refusal, so collapsing it into the resolver would be wrong: the resolver
    // answers "where do I READ the law from" and may return an out-of-tree directory, while
    // this asks "is this target inside the tree nothing may manufacture". Both roots are
    // checked below, so the guard covers the vendored candidate AND a recorded subscription.
    let mut roots = vec![repo.join("lawpack/v2")];
    if let Some(configured) = lawpack_path_from_config(repo) {
        roots.push(absolutise(&configured));
    }
    if let Some(env) = env_lawpack_path() {
        roots.push(absolutise(&env));
    }
    for root in roots {
        if target == root || target.starts_with(&root) {
            return Err(KernelError::InvalidInput(format!(
                "refusing to write '{}': it is inside the canon tree at '{}'. No kernel write \
                 path may bring into being, or write through, the directory the resolver reads \
                 the canon from ([2026] VJS-CC-VJS 16 D2). One valid record call once replaced a \
                 160-file constitutional canon with a one-file directory this way. Choose an \
                 output path outside the canon tree; authoring canon is a deliberate permitted \
                 act, not the side effect of a verb writing a report.",
                target.display(),
                root.display()
            )));
        }
    }
    Ok(())
}

/// What `vjs invoke --lawpack` names: the label to record, and the directory it resolved to.
///
/// D3 of [2026] VJS-CC-VJS 12: "a flag that labels without selecting must be made to select
/// or be removed". The value may be
///   - a path to a lawpack directory (absolute, or relative to the repo), or
///   - a registered id like `vjs-v2@0.1.0`, resolved against the vendored copy, and
///   - omitted, which means the vendored copy and nothing else.
///
/// A path that does not resolve is an ERROR, never a label. That is the whole ruling: the
/// old code accepted any string and wrote it into two artefacts as though a subscription had
/// happened.
pub fn resolve_invocation_lawpack(
    repo: &Path,
    lawpack: Option<String>,
) -> Result<(String, Option<PathBuf>, Option<PathBuf>), KernelError> {
    let Some(named) = lawpack else {
        // Nothing named: the vendored copy, or nothing. `invoke` is allowed to run in a
        // repository that has neither, because the refusal in `load_lawpack` is keyed on a
        // config that invocation has not written yet - see D1/D2, which do not collide.
        let d = resolve_lawpack_dir(repo);
        return Ok(("vjs-v2@0.1.0".to_string(), d.clone(), d));
    };

    // A registered id, not a path. Resolve it the only way this kernel currently can.
    if !named.contains('/') && !named.contains(std::path::MAIN_SEPARATOR) {
        let d = resolve_lawpack_dir(repo);
        return Ok((named, d.clone(), d));
    }

    let raw = PathBuf::from(&named);
    let candidate = if raw.is_absolute() {
        raw.clone()
    } else {
        repo.join(&raw)
    };
    if !candidate.is_dir() {
        return Err(KernelError::InvalidInput(format!(
            "--lawpack {} does not resolve to a directory (looked at {}). A named lawpack that \
             cannot be resolved is refused rather than recorded: writing it into config.toml \
             and lawpack.lock would assert a subscription that did not happen. \
             ([2026] VJS-CC-VJS 12 D3)",
            named,
            candidate.display()
        )));
    }
    // Resolve for the DIGEST, but record a RELATIVE path as given.
    //
    // The first version canonicalised unconditionally, on the reasoning that the config
    // should not be a second opinion about where the law is. That is right for an absolute
    // path and wrong for a relative one: canonicalising `../vibe-justice-system/lawpack/v2`
    // bakes one machine's home directory into a committed config, so every other clone
    // resolves nothing. A repo-relative path is not ambiguous - `resolve_lawpack_dir` joins
    // it to the repo root - and it is the only form that survives a clone.
    let dir = candidate.canonicalize().unwrap_or(candidate);
    let recorded = if raw.is_absolute() {
        dir.clone()
    } else {
        raw.clone()
    };
    let id = lawpack_id_of(&dir).unwrap_or_else(|| "vjs-v2@0.1.0".to_string());
    Ok((id, Some(dir), Some(recorded)))
}

/// The lawpack's own declared id, read from its manifest.
fn lawpack_id_of(dir: &Path) -> Option<String> {
    let text = std::fs::read_to_string(dir.join("manifest.toml")).ok()?;
    let mut id = None;
    let mut version = None;
    for line in text.lines() {
        let line = line.trim();
        if let Some(v) = line.strip_prefix("id")
            && let Some(v) = v.trim_start().strip_prefix('=')
        {
            id = Some(v.trim().trim_matches('"').to_string());
        } else if let Some(v) = line.strip_prefix("version")
            && let Some(v) = v.trim_start().strip_prefix('=')
        {
            version = Some(v.trim().trim_matches('"').to_string());
        }
    }
    match (id, version) {
        (Some(i), Some(v)) => Some(format!("{i}@{v}")),
        (Some(i), None) => Some(i),
        _ => None,
    }
}

/// The `lawpack_path` recorded by `vjs invoke`, if the config carries one.
///
/// Read with a line scan rather than a TOML parse on purpose: this runs before the kernel
/// context exists, on a file that may be half-written by a concurrent invoke, and a parse
/// error here would turn a recoverable absence into a hard failure of every command.
/// Public since [2026] VJS-CC-VJS 16 C5: the displacement finding is keyed on the RECORDED
/// subscription against the RESOLUTION, so the thing that reads the record and the thing that
/// resolves must be the same reader.
pub fn lawpack_path_from_config(repo: &Path) -> Option<PathBuf> {
    let text = std::fs::read_to_string(repo.join(".vjs/config.toml")).ok()?;
    for line in text.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("lawpack_path") else {
            continue;
        };
        let Some(rest) = rest.trim_start().strip_prefix('=') else {
            continue;
        };
        let v = rest.trim().trim_matches('"');
        if !v.is_empty() {
            return Some(PathBuf::from(v));
        }
    }
    None
}
