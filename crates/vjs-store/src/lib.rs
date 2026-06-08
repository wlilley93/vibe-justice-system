use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use vjs_core::*;
use vjs_core::types::*;
use vjs_core::error::*;

pub struct Store;

impl Store {
    pub fn init_repo(repo_root: &Path) -> Result<(), KernelError> {
        let vjs_dir = repo_root.join(".vjs");
        std::fs::create_dir_all(&vjs_dir)
            .map_err(|e| KernelError::Io(e.to_string()))?;

        let dirs = [
            "orders",
            "logs/decisions",
            "logs/actions",
            "logs/breaches",
            "submissions/draft",
            "submissions/filed",
            "cache",
            "private",
        ];

        for dir in &dirs {
            std::fs::create_dir_all(vjs_dir.join(dir))
                .map_err(|e| KernelError::Io(e.to_string()))?;
        }

        let config_path = vjs_dir.join("config.toml");
        if !config_path.exists() {
            let config = default_config();
            let content = toml::to_string(&config)
                .map_err(|e| KernelError::Serialization(e.to_string()))?;
            std::fs::write(&config_path, content)
                .map_err(|e| KernelError::Io(e.to_string()))?;
        }

        let private_readme = vjs_dir.join("private/README.md");
        if !private_readme.exists() {
            let content = "# Private Working Papers\n\nThis directory contains unredacted local evidence, operational notes, and private working papers. Do not commit to public repositories.\n";
            std::fs::write(&private_readme, content)
                .map_err(|e| KernelError::Io(e.to_string()))?;
        }

        Ok(())
    }

    pub fn write_log(repo_root: &Path, log: &DecisionLog) -> Result<(), KernelError> {
        let logs_dir = repo_root.join(".vjs/logs/decisions");
        std::fs::create_dir_all(&logs_dir)
            .map_err(|e| KernelError::Io(e.to_string()))?;

        let filename = format!("{}.yaml", log.id);
        let path = logs_dir.join(&filename);
        let content = serde_yaml::to_string(log)
            .map_err(|e| KernelError::Serialization(e.to_string()))?;
        std::fs::write(&path, content)
            .map_err(|e| KernelError::Io(e.to_string()))?;

        Ok(())
    }

    pub fn write_order(repo_root: &Path, order: &Order) -> Result<(), KernelError> {
        let orders_dir = repo_root.join(".vjs/orders");
        std::fs::create_dir_all(&orders_dir)
            .map_err(|e| KernelError::Io(e.to_string()))?;

        let filename = format!("{}.yaml", order.id);
        let path = orders_dir.join(&filename);
        let content = serde_yaml::to_string(order)
            .map_err(|e| KernelError::Serialization(e.to_string()))?;
        std::fs::write(&path, content)
            .map_err(|e| KernelError::Io(e.to_string()))?;

        Ok(())
    }

    pub fn write_submission(repo_root: &Path, submission: &Submission) -> Result<(), KernelError> {
        let submissions_dir = repo_root.join(".vjs/submissions/filed");
        std::fs::create_dir_all(&submissions_dir)
            .map_err(|e| KernelError::Io(e.to_string()))?;

        let filename = format!("{}.yaml", submission.id);
        let path = submissions_dir.join(&filename);
        let content = serde_yaml::to_string(submission)
            .map_err(|e| KernelError::Serialization(e.to_string()))?;
        std::fs::write(&path, content)
            .map_err(|e| KernelError::Io(e.to_string()))?;

        Ok(())
    }

    pub fn read_logs(repo_root: &Path) -> Result<Vec<DecisionLog>, KernelError> {
        let logs_dir = repo_root.join(".vjs/logs/decisions");
        let mut logs = Vec::new();

        if !logs_dir.exists() {
            return Ok(logs);
        }

        for entry in std::fs::read_dir(&logs_dir)
            .map_err(|e| KernelError::Io(e.to_string()))? {
            let entry = entry.map_err(|e| KernelError::Io(e.to_string()))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                let content = std::fs::read_to_string(&path)
                    .map_err(|e| KernelError::Io(e.to_string()))?;
                let log: DecisionLog = serde_yaml::from_str(&content)
                    .map_err(|e| KernelError::Serialization(e.to_string()))?;
                logs.push(log);
            }
        }

        Ok(logs)
    }

    pub fn read_orders(repo_root: &Path) -> Result<Vec<Order>, KernelError> {
        let orders_dir = repo_root.join(".vjs/orders");
        let mut orders = Vec::new();

        if !orders_dir.exists() {
            return Ok(orders);
        }

        for entry in std::fs::read_dir(&orders_dir)
            .map_err(|e| KernelError::Io(e.to_string()))? {
            let entry = entry.map_err(|e| KernelError::Io(e.to_string()))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                let content = std::fs::read_to_string(&path)
                    .map_err(|e| KernelError::Io(e.to_string()))?;
                let order: Order = serde_yaml::from_str(&content)
                    .map_err(|e| KernelError::Serialization(e.to_string()))?;
                orders.push(order);
            }
        }

        Ok(orders)
    }

    pub fn write_lawpack_lock(
        repo_root: &Path,
        lawpack_id: &str,
        version: &str,
        digest: &str,
    ) -> Result<(), KernelError> {
        let lock_path = repo_root.join(".vjs/lawpack.lock");
        let lock = LawpackLock {
            lawpack_id: lawpack_id.into(),
            lawpack_version: version.into(),
            digest: digest.into(),
            generated_at: chrono::Utc::now().to_rfc3339(),
        };
        let content = toml::to_string(&lock)
            .map_err(|e| KernelError::Serialization(e.to_string()))?;
        std::fs::write(&lock_path, content)
            .map_err(|e| KernelError::Io(e.to_string()))?;
        Ok(())
    }

    pub fn read_lawpack_lock(repo_root: &Path) -> Result<Option<LawpackLock>, KernelError> {
        let lock_path = repo_root.join(".vjs/lawpack.lock");
        if !lock_path.exists() {
            return Ok(None);
        }
        let content = std::fs::read_to_string(&lock_path)
            .map_err(|e| KernelError::Io(e.to_string()))?;
        let lock: LawpackLock = toml::from_str(&content)
            .map_err(|e| KernelError::Serialization(e.to_string()))?;
        Ok(Some(lock))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct JurisdictionConfig {
    version: String,
    jurisdiction_id: String,
    lawpack: String,
    paths: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LawpackLock {
    pub lawpack_id: String,
    pub lawpack_version: String,
    pub digest: String,
    pub generated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Submission {
    pub id: String,
    pub court_requested: String,
    pub jurisdiction: String,
    pub question: String,
    pub facts: String,
    pub requested_order: String,
    pub private_boundary: String,
    pub word_count: usize,
}

fn default_config() -> JurisdictionConfig {
    let mut paths = HashMap::new();
    paths.insert("orders".into(), ".vjs/orders".into());
    paths.insert("logs".into(), ".vjs/logs".into());
    paths.insert("submissions".into(), ".vjs/submissions".into());
    paths.insert("cache".into(), ".vjs/cache".into());
    paths.insert("private".into(), ".vjs/private".into());

    JurisdictionConfig {
        version: "2".into(),
        jurisdiction_id: "default".into(),
        lawpack: "vjs-v2@0.1.0".into(),
        paths,
    }
}

trait CreateDirAll {
    fn create_dir_all(&self) -> std::io::Result<()>;
}

impl CreateDirAll for Path {
    fn create_dir_all(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self)
    }
}
