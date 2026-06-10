use serde::{Deserialize, Serialize};
use std::path::Path;

use vjs_core::*;
use vjs_core::spec::{Permit, Proof};

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
            "court/convenings",
            "permits",
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

        // Decision logs are part of the public record; the boundary scan runs
        // BEFORE the bytes hit disk, not post-hoc in validate. Fail closed:
        // secrets and private facts belong in .vjs/private, not the record.
        let findings = vjs_redact::RedactScanner::scan_file(&path, &content);
        if !vjs_redact::RedactScanner::check_public_safe(&findings) {
            let kinds: Vec<String> = findings.iter().map(|f| f.message.clone()).collect();
            return Err(KernelError::InvalidInput(format!(
                "decision log '{}' fails the public/private boundary scan ({}); keep secrets and private facts out of the record or use .vjs/private",
                log.id,
                kinds.join("; ")
            )));
        }

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

    pub fn write_permit(repo_root: &Path, permit: &Permit) -> Result<(), KernelError> {
        let permits_dir = repo_root.join(".vjs/permits");
        std::fs::create_dir_all(&permits_dir)
            .map_err(|e| KernelError::Io(e.to_string()))?;

        let filename = format!("{}.yaml", permit.id.0);
        let path = permits_dir.join(&filename);
        let content = serde_yaml::to_string(permit)
            .map_err(|e| KernelError::Serialization(e.to_string()))?;
        std::fs::write(&path, content)
            .map_err(|e| KernelError::Io(e.to_string()))?;

        Ok(())
    }

    pub fn read_permits(repo_root: &Path) -> Result<Vec<Permit>, KernelError> {
        let permits_dir = repo_root.join(".vjs/permits");
        let mut permits = Vec::new();

        if !permits_dir.exists() {
            return Ok(permits);
        }

        for entry in std::fs::read_dir(&permits_dir)
            .map_err(|e| KernelError::Io(e.to_string()))? {
            let entry = entry.map_err(|e| KernelError::Io(e.to_string()))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                let content = std::fs::read_to_string(&path)
                    .map_err(|e| KernelError::Io(e.to_string()))?;
                let permit: Permit = serde_yaml::from_str(&content)
                    .map_err(|e| KernelError::Serialization(e.to_string()))?;
                permits.push(permit);
            }
        }

        Ok(permits)
    }

    pub fn write_proof(repo_root: &Path, proof: &Proof) -> Result<(), KernelError> {
        let proofs_dir = repo_root.join(".vjs/proofs");
        std::fs::create_dir_all(&proofs_dir)
            .map_err(|e| KernelError::Io(e.to_string()))?;

        let filename = format!("{}.yaml", proof.id.0);
        let path = proofs_dir.join(&filename);
        let content = serde_yaml::to_string(proof)
            .map_err(|e| KernelError::Serialization(e.to_string()))?;
        std::fs::write(&path, content)
            .map_err(|e| KernelError::Io(e.to_string()))?;

        Ok(())
    }

    pub fn read_proofs(repo_root: &Path) -> Result<Vec<Proof>, KernelError> {
        let proofs_dir = repo_root.join(".vjs/proofs");
        let mut proofs = Vec::new();

        if !proofs_dir.exists() {
            return Ok(proofs);
        }

        for entry in std::fs::read_dir(&proofs_dir)
            .map_err(|e| KernelError::Io(e.to_string()))? {
            let entry = entry.map_err(|e| KernelError::Io(e.to_string()))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                // Same contract as read_permits/read_logs: a corrupt proof is
                // an error, not a silent absence (a vanished proof would read
                // as an unmet obligation, or worse, mask a tampered record).
                let content = std::fs::read_to_string(&path)
                    .map_err(|e| KernelError::Io(format!("{}: {}", path.display(), e)))?;
                let proof: Proof = serde_yaml::from_str(&content)
                    .map_err(|e| KernelError::Serialization(format!("{}: {}", path.display(), e)))?;
                proofs.push(proof);
            }
        }
        Ok(proofs)
    }

    pub fn write_convening(repo_root: &Path, rec: &ConveningRecord) -> Result<(), KernelError> {
        let dir = repo_root.join(".vjs/court/convenings");
        std::fs::create_dir_all(&dir).map_err(|e| KernelError::Io(e.to_string()))?;
        let path = dir.join(format!("{}.yaml", rec.id));
        let content = serde_yaml::to_string(rec)
            .map_err(|e| KernelError::Serialization(e.to_string()))?;
        // Convening records are public record: the boundary scan runs first.
        let findings = vjs_redact::RedactScanner::scan_file(&path, &content);
        if !vjs_redact::RedactScanner::check_public_safe(&findings) {
            return Err(KernelError::InvalidInput(format!(
                "convening record '{}' fails the public/private boundary scan; keep secrets out of the record",
                rec.id
            )));
        }
        std::fs::write(&path, content).map_err(|e| KernelError::Io(e.to_string()))?;
        Ok(())
    }

    pub fn read_convenings(repo_root: &Path) -> Result<Vec<ConveningRecord>, KernelError> {
        let dir = repo_root.join(".vjs/court/convenings");
        let mut out = Vec::new();
        if !dir.exists() {
            return Ok(out);
        }
        for entry in std::fs::read_dir(&dir).map_err(|e| KernelError::Io(e.to_string()))? {
            let path = entry.map_err(|e| KernelError::Io(e.to_string()))?.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                let content = std::fs::read_to_string(&path)
                    .map_err(|e| KernelError::Io(e.to_string()))?;
                if let Ok(rec) = serde_yaml::from_str::<ConveningRecord>(&content) {
                    out.push(rec);
                }
            }
        }
        Ok(out)
    }

    pub fn read_submissions(repo_root: &Path) -> Result<Vec<Submission>, KernelError> {
        let dir = repo_root.join(".vjs/submissions/filed");
        let mut out = Vec::new();
        if !dir.exists() {
            return Ok(out);
        }
        for entry in std::fs::read_dir(&dir).map_err(|e| KernelError::Io(e.to_string()))? {
            let path = entry.map_err(|e| KernelError::Io(e.to_string()))?.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                let content = std::fs::read_to_string(&path)
                    .map_err(|e| KernelError::Io(e.to_string()))?;
                if let Ok(sub) = serde_yaml::from_str::<Submission>(&content) {
                    out.push(sub);
                }
            }
        }
        Ok(out)
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

    pub fn read_repo_config(repo_root: &Path) -> Result<Option<JurisdictionConfig>, KernelError> {
        let config_path = repo_root.join(".vjs/config.toml");
        if !config_path.exists() {
            return Ok(None);
        }
        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| KernelError::Io(e.to_string()))?;
        let config: JurisdictionConfig = toml::from_str(&content)
            .map_err(|e| KernelError::Serialization(e.to_string()))?;
        Ok(Some(config))
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
pub struct JurisdictionConfig {
    pub version: String,
    pub jurisdiction_id: String,
    pub lawpack: String,
    pub paths: PathsConfig,
    pub governance: Option<GovernanceConfig>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PathsConfig {
    pub orders: String,
    pub logs: String,
    pub submissions: String,
    pub specs: String,
    pub decisions: String,
    pub proofs: String,
    pub permits: String,
    pub private: String,
    pub cache: String,
    pub public: Option<PublicPathsConfig>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicPathsConfig {
    pub enabled: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GovernanceConfig {
    pub permit_required: Vec<String>,
    pub permit_exempt: Vec<String>,
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

/// A convening record: the auditable proof that a court sat. It pins the
/// sha256 of the symmetric case file (the submission) it decided and the bench
/// that decided, so a ruling can be traced to exactly what was before it.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConveningRecord {
    pub id: String,
    pub court: String,
    pub submission_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue: Option<String>,
    pub case_file_digest: String,
    pub bench: Vec<String>,
    pub convened_at: String,
}

fn default_config() -> JurisdictionConfig {
    JurisdictionConfig {
        version: "2".into(),
        jurisdiction_id: "default".into(),
        lawpack: "vjs-v2@0.1.0".into(),
        paths: default_paths(),
        governance: Some(default_governance()),
    }
}

fn default_paths() -> PathsConfig {
    PathsConfig {
        orders: ".vjs/orders".into(),
        logs: ".vjs/logs".into(),
        submissions: ".vjs/submissions".into(),
        specs: "lawpack/v2/specs".into(),
        decisions: "lawpack/v2/decisions".into(),
        proofs: ".vjs/proofs".into(),
        permits: ".vjs/permits".into(),
        private: ".vjs/private".into(),
        cache: ".vjs/cache".into(),
        public: Some(PublicPathsConfig { enabled: false }),
    }
}

fn default_governance() -> GovernanceConfig {
    GovernanceConfig {
        permit_required: vec![
            "crates/**".into(),
            "lawpack/v2/**".into(),
            "Cargo.toml".into(),
            "AGENTS.md".into(),
            "VJS.md".into(),
            "README.md".into(),
        ],
        permit_exempt: vec![
            ".vjs/logs/**".into(),
            ".vjs/permits/**".into(),
            ".vjs/proofs/**".into(),
            ".vjs/cache/**".into(),
            ".vjs/private/**".into(),
            "target/**".into(),
        ],
    }
}

