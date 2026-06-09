use regex::Regex;
use std::path::Path;

use vjs_core::*;

pub struct RedactScanner;

impl RedactScanner {
    pub fn scan_file(path: &Path, content: &str) -> Vec<BoundaryFinding> {
        let mut findings = Vec::new();

        let patterns = [
            (
                r"\b(sk-[a-zA-Z0-9]{48})\b",
                BoundaryFindingKind::Token,
                "OpenAI API key detected",
            ),
            (
                r"\b(gh[pousr]_[A-Za-z0-9_]{36,})\b",
                BoundaryFindingKind::Token,
                "GitHub token detected",
            ),
            (
                r"\b(AKIA[0-9A-Z]{16})\b",
                BoundaryFindingKind::Token,
                "AWS access key detected",
            ),
            (
                r"\b([a-zA-Z0-9_-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})\b",
                BoundaryFindingKind::Email,
                "Email address detected",
            ),
            (
                r#"\b(password|passwd|pwd)\s*=\s*["][^"]+["]\b"#,
                BoundaryFindingKind::Secret,
                "Password assignment detected",
            ),
            (
                r#"\b(api[_-]?key|apikey)\s*=\s*["][^"]+["]\b"#,
                BoundaryFindingKind::Token,
                "API key assignment detected",
            ),
            (
                r"\b(192\.168\.\d{1,3}\.\d{1,3}|10\.\d{1,3}\.\d{1,3}\.\d{1,3}|172\.(1[6-9]|2\d|3[01])\.\d{1,3}\.\d{1,3})\b",
                BoundaryFindingKind::PrivateHostname,
                "Private IP address detected",
            ),
            (
                r"\b([a-zA-Z0-9_-]+\.(local|internal|private|lan))\b",
                BoundaryFindingKind::PrivateHostname,
                "Internal hostname detected",
            ),
        ];

        for (pattern, kind, message) in &patterns {
            if let Ok(re) = Regex::new(pattern) {
                for mat in re.find_iter(content) {
                    findings.push(BoundaryFinding {
                        severity: Severity::Error,
                        path: Some(path.to_path_buf()),
                        kind: kind.clone(),
                        message: format!("{} at position {}", message, mat.start()),
                        suggested_route: BoundaryRoute::Redact,
                    });
                }
            }
        }

        findings
    }

    pub fn scan_directory(dir: &Path) -> Result<Vec<BoundaryFinding>, KernelError> {
        let mut findings = Vec::new();

        for entry in walkdir::WalkDir::new(dir)
            .max_depth(5)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file()
                && let Ok(content) = std::fs::read_to_string(path) {
                    let file_findings = Self::scan_file(path, &content);
                    findings.extend(file_findings);
                }
        }

        Ok(findings)
    }

    pub fn check_public_safe(findings: &[BoundaryFinding]) -> bool {
        !findings.iter().any(|f| matches!(f.severity, Severity::Fatal | Severity::Error))
    }
}
