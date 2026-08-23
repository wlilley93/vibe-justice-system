use std::path::{Path, PathBuf};

use vjs_core::*;

pub struct TestKit;

impl TestKit {
    pub fn sample_repo_simple() -> SampleRepo {
        SampleRepo {
            name: "simple-rust-app".into(),
            files: vec![
                (
                    "Cargo.toml".into(),
                    "[package]\nname = \"app\"\nversion = \"0.1.0\"\n".into(),
                ),
                (
                    "src/main.rs".into(),
                    "fn main() { println!(\"hello\"); }\n".into(),
                ),
            ],
            expected_route: RouteOutcome::Allowed,
            issue_tags: vec!["hello_world".into()],
        }
    }

    pub fn sample_repo_dependency_choice() -> SampleRepo {
        SampleRepo {
            name: "dependency-choice".into(),
            files: vec![(
                "Cargo.toml".into(),
                "[package]\nname = \"app\"\n\n[dependencies]\n".into(),
            )],
            expected_route: RouteOutcome::AllowedWithConditions,
            issue_tags: vec!["dependency_policy".into()],
        }
    }

    pub fn sample_repo_private_boundary() -> SampleRepo {
        SampleRepo {
            name: "private-boundary".into(),
            files: vec![
                ("Cargo.toml".into(), "[package]\nname = \"app\"\n".into()),
                (
                    "src/main.rs".into(),
                    "// API key: sk-test1234567890abcdef\nfn main() {}\n".into(),
                ),
            ],
            expected_route: RouteOutcome::Blocked,
            issue_tags: vec!["public_private.repo_facts".into()],
        }
    }

    pub fn sample_repo_external_release() -> SampleRepo {
        SampleRepo {
            name: "external-release".into(),
            files: vec![("Cargo.toml".into(), "[package]\nname = \"app\"\n".into())],
            expected_route: RouteOutcome::ReleaseWarrantRequired,
            issue_tags: vec!["release.push".into()],
        }
    }

    pub fn sample_repo_first_impression() -> SampleRepo {
        SampleRepo {
            name: "first-impression".into(),
            files: vec![("Cargo.toml".into(), "[package]\nname = \"app\"\n".into())],
            expected_route: RouteOutcome::CourtRequired,
            issue_tags: vec!["novel_architecture".into()],
        }
    }

    pub fn create_fixture_repo(repo: &SampleRepo, base_dir: &Path) -> Result<PathBuf, KernelError> {
        let repo_dir = base_dir.join(&repo.name);
        std::fs::create_dir_all(&repo_dir).map_err(|e| KernelError::Io(e.to_string()))?;

        for (path, content) in &repo.files {
            let full_path = repo_dir.join(path);
            if let Some(parent) = full_path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| KernelError::Io(e.to_string()))?;
            }
            std::fs::write(&full_path, content).map_err(|e| KernelError::Io(e.to_string()))?;
        }

        // Initialize git
        std::process::Command::new("git")
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .env_remove("GIT_INDEX_FILE")
            .env_remove("GIT_OBJECT_DIRECTORY")
            .args(["init", "--quiet"])
            .current_dir(&repo_dir)
            .output()
            .map_err(|e| KernelError::Io(e.to_string()))?;

        Ok(repo_dir)
    }

    pub fn golden_route_output(repo: &SampleRepo) -> RouteDecision {
        RouteDecision {
            decision: repo.expected_route.clone(),
            jurisdiction: JurisdictionId("test".into()),
            court_required: repo.expected_route == RouteOutcome::CourtRequired,
            court: if repo.expected_route == RouteOutcome::CourtRequired {
                Some(Court::County)
            } else {
                None
            },
            court_trigger: if repo.expected_route == RouteOutcome::CourtRequired {
                Some(CourtTrigger::FirstImpression)
            } else {
                None
            },
            log_required: repo.expected_route != RouteOutcome::Allowed,
            binding: Vec::new(),
            must_do: Vec::new(),
            must_not_do: Vec::new(),
            warnings: Vec::new(),
            max_context: ContextBudget::default(),
            summary: format!("Test fixture for {}", repo.name),
            obligations: Vec::new(),
            permit_id: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SampleRepo {
    pub name: String,
    pub files: Vec<(String, String)>,
    pub expected_route: RouteOutcome,
    pub issue_tags: Vec<String>,
}
