use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::*;
use crate::spec::*;
use crate::types::*;

pub struct RepoScanner;

impl RepoScanner {
    pub fn build_repo_state(repo_root: &Path) -> Result<RepoState, KernelError> {
        let mut state = RepoState {
            root: repo_root.to_path_buf(),
            head_sha: None,
            changed_paths: Vec::new(),
            added_files: Vec::new(),
            modified_files: Vec::new(),
            deleted_files: Vec::new(),
            file_contents: HashMap::new(),
            dependency_changes: Vec::new(),
            permits: Vec::new(),
            proofs: Vec::new(),
            logs: Vec::new(),
            orders: Vec::new(),
            boundary_findings: Vec::new(),
        };

        // Read git status
        if let Ok(staged) = Self::read_staged_files(repo_root) {
            state.changed_paths = staged.clone();
            state.modified_files = staged;
        }

        // Read file contents for changed paths, EXCEPT the published Gazette
        // presentation, which is constitutively inert (REG-GAZETTE-CONTINUITY-
        // 001) and must never be read as law: the archive's frozen V1 copies
        // and the generated data artifacts quote legal phrases in prose, on
        // which a content predicate (field_equals, string_contains) would
        // false-positive. Law and governed code are scanned; presentation is not.
        let is_presentation = |p: &PathBuf| -> bool {
            let s = p.to_string_lossy();
            s.starts_with("archive/")
                || s.ends_with(".html")
                || matches!(
                    p.file_name().and_then(|f| f.to_str()),
                    Some("gazette-data.js")
                        | Some("gazette-data.json")
                        | Some("gazette-text.js")
                        | Some("gazette.xml")
                )
        };
        for path in &state.changed_paths.clone() {
            if is_presentation(path) {
                continue;
            }
            let full_path = repo_root.join(path);
            if full_path.exists()
                && let Ok(content) = std::fs::read_to_string(&full_path)
            {
                state.file_contents.insert(path.clone(), content);
            }
        }

        // Read dependency changes from Cargo.toml
        if let Some(cargo_toml) = state.file_contents.get(&PathBuf::from("Cargo.toml")) {
            state.dependency_changes = Self::parse_cargo_changes(cargo_toml);
        }

        // Read .vjs records
        let vjs_dir = repo_root.join(".vjs");
        if vjs_dir.exists() {
            state.logs = Self::read_logs(&vjs_dir)?;
            state.orders = Self::read_orders(&vjs_dir)?;
            state.permits = Self::read_permits(&vjs_dir)?;
            state.proofs = Self::read_proofs(&vjs_dir)?;
        }

        Ok(state)
    }

    fn read_staged_files(repo_root: &Path) -> Result<Vec<PathBuf>, KernelError> {
        let output = Command::new("git")
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .env_remove("GIT_INDEX_FILE")
            .env_remove("GIT_OBJECT_DIRECTORY")
            .args(["diff", "--name-only", "--cached"])
            .current_dir(repo_root)
            .output()
            .map_err(|e| KernelError::Io(e.to_string()))?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.lines().map(PathBuf::from).collect())
    }

    fn parse_cargo_changes(cargo_toml: &str) -> Vec<DependencyChange> {
        let mut changes = Vec::new();
        // Simplified: just check if reqwest, hyper, etc. are mentioned
        for name in ["reqwest", "hyper", "ureq", "curl", "openai", "anthropic"] {
            if cargo_toml.contains(name) {
                changes.push(DependencyChange {
                    name: name.to_string(),
                    added: true,
                    removed: false,
                });
            }
        }
        changes
    }

    fn read_logs(vjs_dir: &Path) -> Result<Vec<DecisionLog>, KernelError> {
        let mut logs = Vec::new();
        let logs_dir = vjs_dir.join("logs/decisions");
        if !logs_dir.exists() {
            return Ok(logs);
        }

        for entry in std::fs::read_dir(&logs_dir).map_err(|e| KernelError::Io(e.to_string()))? {
            let entry = entry.map_err(|e| KernelError::Io(e.to_string()))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml")
                && let Ok(content) = std::fs::read_to_string(&path)
                && let Ok(log) = serde_yaml::from_str::<DecisionLog>(&content)
            {
                logs.push(log);
            }
        }
        Ok(logs)
    }

    fn read_orders(vjs_dir: &Path) -> Result<Vec<Order>, KernelError> {
        let mut orders = Vec::new();
        let orders_dir = vjs_dir.join("orders");
        if !orders_dir.exists() {
            return Ok(orders);
        }

        for entry in std::fs::read_dir(&orders_dir).map_err(|e| KernelError::Io(e.to_string()))? {
            let entry = entry.map_err(|e| KernelError::Io(e.to_string()))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml")
                && let Ok(content) = std::fs::read_to_string(&path)
                && let Ok(order) = serde_yaml::from_str::<Order>(&content)
            {
                orders.push(order);
            }
        }
        Ok(orders)
    }

    fn read_permits(vjs_dir: &Path) -> Result<Vec<Permit>, KernelError> {
        let mut permits = Vec::new();
        let permits_dir = vjs_dir.join("permits");
        if !permits_dir.exists() {
            return Ok(permits);
        }

        for entry in std::fs::read_dir(&permits_dir).map_err(|e| KernelError::Io(e.to_string()))? {
            let entry = entry.map_err(|e| KernelError::Io(e.to_string()))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml")
                && let Ok(content) = std::fs::read_to_string(&path)
                && let Ok(permit) = serde_yaml::from_str::<Permit>(&content)
            {
                permits.push(permit);
            }
        }
        Ok(permits)
    }

    fn read_proofs(vjs_dir: &Path) -> Result<Vec<Proof>, KernelError> {
        let mut proofs = Vec::new();
        let proofs_dir = vjs_dir.join("proofs");
        if !proofs_dir.exists() {
            return Ok(proofs);
        }

        for entry in std::fs::read_dir(&proofs_dir).map_err(|e| KernelError::Io(e.to_string()))? {
            let entry = entry.map_err(|e| KernelError::Io(e.to_string()))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml")
                && let Ok(content) = std::fs::read_to_string(&path)
                && let Ok(proof) = serde_yaml::from_str::<Proof>(&content)
            {
                proofs.push(proof);
            }
        }
        Ok(proofs)
    }
}
