use std::path::Path;
use std::process::Command;
use regex::Regex;

use vjs_core::*;
use vjs_core::error::*;

pub struct GitIntegration;

impl GitIntegration {
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
        let output = Command::new("git")
            .args(["diff", "--name-only", "--cached"])
            .current_dir(repo_root)
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

    pub fn read_unstaged_files(repo_root: &Path) -> Result<Vec<String>, KernelError> {
        let output = Command::new("git")
            .args(["diff", "--name-only"])
            .current_dir(repo_root)
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
        let output = Command::new("git")
            .args(["remote", "get-url", "origin"])
            .current_dir(repo_root)
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
            let public_patterns = [
                "github.com",
                "gitlab.com",
                "bitbucket.org",
            ];
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
# VJS V2 pre-push hook
# Release and local CI checks

set -e

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
            if let Ok(meta) = std::fs::symlink_metadata(&p) {
                if meta.file_type().is_symlink() {
                    std::fs::remove_file(&p)
                        .map_err(|e| KernelError::Io(e.to_string()))?;
                }
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
