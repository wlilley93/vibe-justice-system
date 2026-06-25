//! Install-completeness invariant and the atomic install manifest (PC-13 D4 + D5).
//!
//! REG-INVOCATION-001 fixes what an installation must bring into being: a
//! .vjs/config.toml anchor, a .vjs/lawpack.lock pinning the lawpack digest, the
//! enforcement hooks, and a recorded local_sovereign_invocation. The disease PC-13
//! named is an agent "operating some but not all mechanisms of the system from
//! context alone": a half-installed jurisdiction is exactly that. D4 gives the
//! existing REG-INVOCATION-001 / ACT-007:s1 duty teeth - fail closed at pre_write
//! and validate --staged unless the surface is present and active. D5 records that
//! surface as ONE atomic sha256-locked manifest (.vjs/install.lock), mirroring the
//! REG-ACCESSION-001 / REG-BUNDLE-001 lock pattern; no external signing key, the
//! sha256 lock suffices.

use serde::{Deserialize, Serialize};
use std::path::Path;

/// The fixed manifest filename, the atomic lock of the install surface.
pub const MANIFEST_FILE: &str = ".vjs/install.lock";
pub const MANIFEST_SCHEMA_VERSION: u32 = 1;

/// The agent runtimes for which `vjs invoke` emits thin hook adapters (D8). The
/// flag is true when the runtime passes its hook payload as JSON on stdin.
pub const RUNTIMES: &[(&str, bool)] = &[
    ("claude", true),
    ("codex", true),
    ("mcp", true),
    ("gemini", false),
    ("opencode", false),
    ("shell", false),
];

/// The agent-runtime hook events adapters are emitted for (D8/D9). These are three
/// of the closed five-event surface of REG-HOOKS-001; the other two (pre_commit,
/// pre_push) are git hooks, installed separately. "prepare-commit-msg" and
/// "post-checkout" are git names OUTSIDE the closed surface and are never emitted.
pub const AGENT_EVENTS: &[&str] = &["pre_write", "session_start", "post_action"];

/// The adapter directory under .vjs/hooks (regenerated per clone, like the git
/// hooks; gitignored).
pub const ADAPTERS_DIR: &str = ".vjs/hooks/adapters";

/// A missing or inactive limb of the REG-INVOCATION-001 surface. Each names the
/// instrument behind it, so a denial cites the law (D4).
#[derive(Clone, Debug, PartialEq)]
pub enum InstallDefect {
    MissingConfig,
    MissingLawpackLock,
    HooksNotInstalled,
    MissingInvocation,
    ManifestAbsent,
    ManifestStale { limb: String },
}

impl InstallDefect {
    pub fn code(&self) -> &'static str {
        match self {
            InstallDefect::MissingConfig => "INSTALL_CONFIG_MISSING",
            InstallDefect::MissingLawpackLock => "INSTALL_LAWPACK_LOCK_MISSING",
            InstallDefect::HooksNotInstalled => "INSTALL_HOOKS_MISSING",
            InstallDefect::MissingInvocation => "INSTALL_INVOCATION_MISSING",
            InstallDefect::ManifestAbsent => "INSTALL_MANIFEST_ABSENT",
            InstallDefect::ManifestStale { .. } => "INSTALL_MANIFEST_STALE",
        }
    }
    pub fn message(&self) -> String {
        match self {
            InstallDefect::MissingConfig => {
                "Install incomplete: .vjs/config.toml absent (ACT-007:s1; REG-INVOCATION-001).".into()
            }
            InstallDefect::MissingLawpackLock => {
                "Install incomplete: .vjs/lawpack.lock absent - the lawpack digest is unpinned (REG-INVOCATION-001).".into()
            }
            InstallDefect::HooksNotInstalled => {
                "Install incomplete: enforcement hooks not installed under .vjs/hooks (REG-INVOCATION-001 install_enforcement_hooks). Run vjs invoke --install-hooks.".into()
            }
            InstallDefect::MissingInvocation => {
                "Install incomplete: no local_sovereign_invocation recorded under .vjs/invocation (REG-INVOCATION-001).".into()
            }
            InstallDefect::ManifestAbsent => {
                "Install manifest .vjs/install.lock absent: the surface is not atomically locked (REG-INSTALL-MANIFEST-001).".into()
            }
            InstallDefect::ManifestStale { limb } => format!(
                "Install manifest is stale: the recorded digest for '{limb}' does not match the file on disk (REG-INSTALL-MANIFEST-001)."
            ),
        }
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::Digest;
    format!("sha256:{}", hex::encode(sha2::Sha256::digest(bytes)))
}

fn digest_file(p: &Path) -> Option<String> {
    std::fs::read(p).ok().map(|b| sha256_hex(&b))
}

/// The first local_sovereign_invocation record under .vjs/invocation, if any.
fn first_invocation(repo: &Path) -> Option<std::path::PathBuf> {
    let dir = repo.join(".vjs/invocation");
    let mut entries: Vec<_> = std::fs::read_dir(&dir)
        .ok()?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("yaml"))
        .collect();
    entries.sort();
    entries.into_iter().next()
}

/// True when this directory is an invoked (or partly-invoked) VJS jurisdiction:
/// it has a .vjs/ store. A bare directory with no .vjs/ is not governed and the
/// completeness invariant does not apply.
pub fn is_vjs_jurisdiction(repo: &Path) -> bool {
    repo.join(".vjs").is_dir()
}

/// The four-limb surface check (D4). Returns the defects; empty means complete.
/// Does NOT inspect the manifest - see verify_manifest. Returns empty for a
/// non-jurisdiction directory (no .vjs/).
pub fn verify_surface(repo: &Path) -> Vec<InstallDefect> {
    let mut defects = Vec::new();
    if !is_vjs_jurisdiction(repo) {
        return defects;
    }
    if !repo.join(".vjs/config.toml").is_file() {
        defects.push(InstallDefect::MissingConfig);
    }
    if !repo.join(".vjs/lawpack.lock").is_file() {
        defects.push(InstallDefect::MissingLawpackLock);
    }
    // Hooks active: the executable wrappers git runs are present. core.hooksPath is
    // set by invoke; presence of the wrappers is the on-disk proof they were laid.
    let hooks_ok =
        repo.join(".vjs/hooks/pre-commit").is_file() && repo.join(".vjs/hooks/pre-push").is_file();
    if !hooks_ok {
        defects.push(InstallDefect::HooksNotInstalled);
    }
    if first_invocation(repo).is_none() {
        defects.push(InstallDefect::MissingInvocation);
    }
    defects
}

/// The atomic install manifest: one sha256-locked record of the surface. The
/// volatile lawpack digest is pinned only by reference (the lawpack.lock has its
/// own consistency check, ACT-007:s7), so a routine canon change does not stale
/// this manifest; the stable limbs (config.toml, the invocation record) are pinned
/// by digest. The hook set is recorded by name (the wrappers are regenerated per
/// clone and so are not digest-pinned).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstallManifest {
    pub schema_version: u32,
    pub generated_at: String,
    pub config_digest: String,
    pub invocation: String,
    pub invocation_digest: String,
    pub lawpack_lock: String,
    pub hooks: Vec<String>,
    /// sha256 of each hook in `hooks` (parallel), pinned so a tampered hook is
    /// detected (#13). Default empty for manifests predating it.
    #[serde(default)]
    pub hook_digests: Vec<String>,
    /// The agent-runtime thin adapters emitted by invoke (D8), recorded by name so
    /// the manifest is bound to them. Default empty for manifests predating D8.
    #[serde(default)]
    pub adapters: Vec<String>,
}

/// Hooks whose on-disk content no longer matches the digest pinned in the manifest
/// (#13) - a likely tamper or a drift after an invoke-template change. Skips a hook
/// that is absent (verify_surface handles absence) or unpinned (older manifest).
/// Advisory: the caller emits a Warning, never a block, and improvement #1
/// (server-side enforcement) is the un-bypassable backstop regardless.
pub fn hook_tamper(repo: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let Ok(text) = std::fs::read_to_string(repo.join(MANIFEST_FILE)) else {
        return out;
    };
    let Ok(manifest) = toml::from_str::<InstallManifest>(&text) else {
        return out;
    };
    for (hook, pinned) in manifest.hooks.iter().zip(manifest.hook_digests.iter()) {
        if pinned.is_empty() {
            continue;
        }
        let p = repo.join(".vjs/hooks").join(hook);
        if p.is_file() && digest_file(&p).as_deref() != Some(pinned) {
            out.push(hook.clone());
        }
    }
    out
}

/// Emit the thin agent-runtime hook adapters (D8): for every runtime in RUNTIMES
/// and every event in AGENT_EVENTS, a script .vjs/hooks/adapters/<runtime>-<event>.sh
/// whose ONLY job is to call `vjs hook --event <event>` - the logic lives in the
/// kernel (hook.rs), per REG-HOOKS-001's thin-adapter rule. Create-if-absent so a
/// hand-customised adapter is never clobbered. Returns the adapter filenames.
pub fn generate_adapters(repo: &Path) -> std::io::Result<Vec<String>> {
    let dir = repo.join(ADAPTERS_DIR);
    std::fs::create_dir_all(&dir)?;
    let mut written = Vec::new();
    for (runtime, stdin_json) in RUNTIMES {
        for event in AGENT_EVENTS {
            let name = format!("{runtime}-{event}.sh");
            let call = if *stdin_json {
                format!("exec \"$bin\" hook --event {event} --stdin-json\n")
            } else {
                format!(
                    "args=(); for p in \"$@\"; do args+=(--path \"$p\"); done\n\
                     exec \"$bin\" hook --event {event} \"${{args[@]}}\"\n"
                )
            };
            let body = format!(
                "#!/usr/bin/env bash\n\
                 # Thin adapter (REG-HOOKS-001): the kernel decides; this only points at it.\n\
                 # Runtime: {runtime}  Event: {event}  (do not add logic here)\n\
                 bin=\"$(git rev-parse --show-toplevel 2>/dev/null)/target/debug/vjs\"\n\
                 [ -x \"$bin\" ] || bin=\"vjs\"\n\
                 {call}"
            );
            let path = dir.join(&name);
            if !path.exists() {
                std::fs::write(&path, body)?;
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(meta) = std::fs::metadata(&path) {
                        let mut perm = meta.permissions();
                        perm.set_mode(0o755);
                        let _ = std::fs::set_permissions(&path, perm);
                    }
                }
            }
            written.push(name);
        }
    }
    written.sort();
    Ok(written)
}

/// The adapter filenames currently present under .vjs/hooks/adapters.
fn present_adapters(repo: &Path) -> Vec<String> {
    let dir = repo.join(ADAPTERS_DIR);
    let mut names: Vec<String> = std::fs::read_dir(&dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|n| n.ends_with(".sh"))
        .collect();
    names.sort();
    names
}

/// Build the manifest from the current surface. Returns None if a pinned limb is
/// absent (build only over a complete surface).
pub fn build_manifest(repo: &Path, generated_at: String) -> Option<InstallManifest> {
    let config_digest = digest_file(&repo.join(".vjs/config.toml"))?;
    let inv_path = first_invocation(repo)?;
    let invocation_digest = digest_file(&inv_path)?;
    let invocation = inv_path
        .strip_prefix(repo)
        .unwrap_or(&inv_path)
        .to_string_lossy()
        .to_string();
    let mut hooks = Vec::new();
    let mut hook_digests = Vec::new();
    for h in ["pre-commit", "pre-push"] {
        let p = repo.join(".vjs/hooks").join(h);
        if p.is_file() {
            hooks.push(h.to_string());
            // #13: pin the hook CONTENT so a later tamper (e.g. editing it to exit 0)
            // is detected. Best-effort digest.
            hook_digests.push(digest_file(&p).unwrap_or_default());
        }
    }
    Some(InstallManifest {
        schema_version: MANIFEST_SCHEMA_VERSION,
        generated_at,
        hook_digests,
        config_digest,
        invocation,
        invocation_digest,
        lawpack_lock: ".vjs/lawpack.lock".into(),
        hooks,
        adapters: present_adapters(repo),
    })
}

/// Verify the recorded manifest against the surface on disk (D5 re-verify). Returns
/// defects: absent manifest, or a stale pinned digest. The surface-presence defects
/// come from verify_surface; this adds the atomic-lock check.
pub fn verify_manifest(repo: &Path) -> Vec<InstallDefect> {
    let mut defects = Vec::new();
    if !is_vjs_jurisdiction(repo) {
        return defects;
    }
    let manifest_path = repo.join(MANIFEST_FILE);
    let Ok(text) = std::fs::read_to_string(&manifest_path) else {
        defects.push(InstallDefect::ManifestAbsent);
        return defects;
    };
    let Ok(manifest) = toml::from_str::<InstallManifest>(&text) else {
        defects.push(InstallDefect::ManifestStale {
            limb: "manifest_parse".into(),
        });
        return defects;
    };
    if digest_file(&repo.join(".vjs/config.toml")).as_deref() != Some(&manifest.config_digest) {
        defects.push(InstallDefect::ManifestStale {
            limb: ".vjs/config.toml".into(),
        });
    }
    if digest_file(&repo.join(&manifest.invocation)).as_deref() != Some(&manifest.invocation_digest)
    {
        defects.push(InstallDefect::ManifestStale {
            limb: manifest.invocation.clone(),
        });
    }
    defects
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_jurisdiction_dir_is_exempt() {
        let dir = std::env::temp_dir().join(format!("vjs_install_none_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        assert!(verify_surface(&dir).is_empty());
        assert!(verify_manifest(&dir).is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn adapters_are_emitted_thin_for_every_runtime_and_event() {
        let dir = std::env::temp_dir().join(format!("vjs_adapters_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let written = generate_adapters(&dir).unwrap();
        // Every runtime x every agent event is present.
        assert_eq!(written.len(), RUNTIMES.len() * AGENT_EVENTS.len());
        for (runtime, _) in RUNTIMES {
            for event in AGENT_EVENTS {
                let name = format!("{runtime}-{event}.sh");
                let body =
                    std::fs::read_to_string(dir.join(ADAPTERS_DIR).join(&name)).unwrap_or_default();
                // Thin: the adapter only calls the kernel for THIS event; no logic.
                assert!(
                    body.contains(&format!("hook --event {event}")),
                    "{name} must call the kernel for {event}"
                );
            }
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn incomplete_surface_is_caught() {
        let dir = std::env::temp_dir().join(format!("vjs_install_partial_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(".vjs")).unwrap();
        let defects = verify_surface(&dir);
        assert!(defects.contains(&InstallDefect::MissingConfig));
        assert!(defects.contains(&InstallDefect::MissingLawpackLock));
        assert!(defects.contains(&InstallDefect::HooksNotInstalled));
        assert!(defects.contains(&InstallDefect::MissingInvocation));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
