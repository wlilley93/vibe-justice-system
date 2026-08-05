use std::path::Path;
use std::process::Command;

use vjs_core::*;

pub struct GitIntegration;

impl GitIntegration {
    /// EVERY git subprocess goes through here. Under a git hook, GIT_DIR and friends
    /// override `current_dir`, so a child git aimed at a fixture writes THROUGH those
    /// variables into the REAL repository - measured in the subscribing jurisdiction
    /// 2026-08-05: a fixture `git init` under pre-push set core.bare=true on the live
    /// config and broke every subsequent git command in the tree. The constructor
    /// scrubs the hook environment so forgetting is structurally impossible.
    /// Public: every crate that spawns git against a repo goes through here, so the
    /// hook-environment scrub exists ONCE (CC-VJS 12: a second copy of a gate is a
    /// hole with a gate next to it).
    pub fn git_command(repo_root: &Path) -> Command {
        let mut c = Command::new("git");
        c.current_dir(repo_root);
        for var in [
            "GIT_DIR",
            "GIT_WORK_TREE",
            "GIT_INDEX_FILE",
            "GIT_OBJECT_DIRECTORY",
        ] {
            c.env_remove(var);
        }
        c
    }

    pub fn find_repo_root(start: &Path) -> Result<Option<std::path::PathBuf>, KernelError> {
        let mut current = start;
        loop {
            let git_dir = current.join(".git");
            if git_dir.exists() {
                return Ok(Some(current.to_path_buf()));
            }
            match current.parent() {
                Some(parent) => current = parent,
                None => return Ok(None),
            }
        }
    }

    pub fn is_git_repo(path: &Path) -> bool {
        path.join(".git").exists()
    }

    pub fn read_staged_files(repo_root: &Path) -> Result<Vec<String>, KernelError> {
        let output = Self::git_command(repo_root)
            .args(["diff", "--name-only", "--cached"])
            .output()
            .map_err(|e| KernelError::Io(e.to_string()))?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let files: Vec<String> = stdout
            .lines()
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(files)
    }

    /// The set of paths committed at HEAD, as ONE deterministic git read. Replaces the former
    /// per-record `git cat-file -e HEAD:<rel>` shell-out in the assent resolver, which spawned a
    /// subprocess for every governed record in every validate (and every test), so a transient
    /// fork/exec failure under load was silently mapped to "not established" - stripping a
    /// genuinely-assented record of its floor (an ACT-010 breach) and making the assent check
    /// non-deterministic (a REG-KERNEL-001 violation). One read, with the sibling error contract:
    /// a genuine spawn failure propagates as a loud `Io` error (never a silent floor-strip); a
    /// non-success exit (e.g. no HEAD on a fresh repo) yields the empty set, so nothing is
    /// established - the correct reading when there is no committed history.
    pub fn tracked_at_head(
        repo_root: &Path,
    ) -> Result<std::collections::HashSet<String>, KernelError> {
        let output = Self::git_command(repo_root)
            .args(["ls-tree", "-r", "--name-only", "HEAD"])
            .output()
            .map_err(|e| KernelError::Io(e.to_string()))?;
        if !output.status.success() {
            return Ok(std::collections::HashSet::new());
        }
        Ok(String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect())
    }

    /// ONE tracked file's content as committed at HEAD. `None` means the path is not in HEAD
    /// at all - a first declaration, or a repo with no commits yet - which callers must
    /// report as such rather than as an empty prior value. Same error contract as
    /// `tracked_at_head`: a genuine spawn failure propagates as a loud `Io` error, a
    /// non-success exit is the honest "not at HEAD".
    pub fn read_blob_at_head(repo_root: &Path, rel: &str) -> Result<Option<String>, KernelError> {
        let output = Self::git_command(repo_root)
            .args(["show", &format!("HEAD:{rel}")])
            .output()
            .map_err(|e| KernelError::Io(e.to_string()))?;
        if !output.status.success() {
            return Ok(None);
        }
        Ok(Some(String::from_utf8_lossy(&output.stdout).to_string()))
    }

    /// Staged DELETIONS only (diff-filter=D). Used by the destructive-action gate
    /// (ACT-006:s4 / ACT-004:s9): deleting a governed record is destructive.
    pub fn read_staged_deletions(repo_root: &Path) -> Result<Vec<String>, KernelError> {
        let output = Self::git_command(repo_root)
            .args(["diff", "--name-only", "--cached", "--diff-filter=D"])
            .output()
            .map_err(|e| KernelError::Io(e.to_string()))?;
        if !output.status.success() {
            return Ok(Vec::new());
        }
        Ok(String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect())
    }

    pub fn read_unstaged_files(repo_root: &Path) -> Result<Vec<String>, KernelError> {
        let output = Self::git_command(repo_root)
            .args(["diff", "--name-only"])
            .output()
            .map_err(|e| KernelError::Io(e.to_string()))?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let files: Vec<String> = stdout
            .lines()
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(files)
    }

    pub fn read_all_changed_files(repo_root: &Path) -> Result<Vec<String>, KernelError> {
        let mut staged = Self::read_staged_files(repo_root)?;
        let unstaged = Self::read_unstaged_files(repo_root)?;
        staged.extend(unstaged);
        staged.sort();
        staged.dedup();
        Ok(staged)
    }

    pub fn read_remote_url(repo_root: &Path) -> Result<Option<String>, KernelError> {
        let output = Self::git_command(repo_root)
            .args(["remote", "get-url", "origin"])
            .output()
            .map_err(|e| KernelError::Io(e.to_string()))?;

        if !output.status.success() {
            return Ok(None);
        }

        let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if url.is_empty() {
            Ok(None)
        } else {
            Ok(Some(url))
        }
    }

    pub fn is_public_remote(repo_root: &Path) -> Result<bool, KernelError> {
        if let Some(url) = Self::read_remote_url(repo_root)? {
            let public_patterns = ["github.com", "gitlab.com", "bitbucket.org"];
            Ok(public_patterns.iter().any(|&p| url.contains(p)))
        } else {
            Ok(false)
        }
    }

    pub fn pre_commit_hook() -> &'static str {
        r#"#!/bin/sh
# VJS V2 pre-commit hook
# Deterministic validation only

set -e

if command -v vjs >/dev/null 2>&1; then
    vjs validate --staged
fi
"#
    }

    pub fn pre_push_hook() -> &'static str {
        r#"#!/bin/sh
# Short state check only (INV-HOOKS-SHORT-001): the gates live in scripts/preci.sh
# where the repository ships one, else in the kernel's local-ci.
set -e
root="$(git rev-parse --show-toplevel)"
if [ -x "$root/scripts/preci.sh" ]; then
    exec "$root/scripts/preci.sh"
fi
if command -v vjs >/dev/null 2>&1; then
    vjs local-ci
    vjs validate --external
fi
"#
    }

    pub fn install_hooks(repo_root: &Path) -> Result<(), KernelError> {
        let hooks_dir = repo_root.join(".git/hooks");
        if !hooks_dir.exists() {
            return Ok(());
        }

        // Never write through a symlinked hook: the write would land wherever
        // the link points. Replace the link with a regular file.
        for name in ["pre-commit", "pre-push"] {
            let p = hooks_dir.join(name);
            if let Ok(meta) = std::fs::symlink_metadata(&p)
                && meta.file_type().is_symlink()
            {
                std::fs::remove_file(&p).map_err(|e| KernelError::Io(e.to_string()))?;
            }
        }

        let pre_commit = hooks_dir.join("pre-commit");
        std::fs::write(&pre_commit, Self::pre_commit_hook())
            .map_err(|e| KernelError::Io(e.to_string()))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&pre_commit)
                .map_err(|e| KernelError::Io(e.to_string()))?
                .permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&pre_commit, perms)
                .map_err(|e| KernelError::Io(e.to_string()))?;
        }

        let pre_push = hooks_dir.join("pre-push");
        std::fs::write(&pre_push, Self::pre_push_hook())
            .map_err(|e| KernelError::Io(e.to_string()))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&pre_push)
                .map_err(|e| KernelError::Io(e.to_string()))?
                .permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&pre_push, perms)
                .map_err(|e| KernelError::Io(e.to_string()))?;
        }

        Ok(())
    }
}
