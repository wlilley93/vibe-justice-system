//! The one finding/report type the gate pipeline produces.
//!
//! Before this, the validate pipeline mapped between six near-identical types
//! (two ValidationFindings, BoundaryFinding, PermitGateFinding, BenchDefect,
//! InstallDefect). That hand-mapping is where a field was dropped (the
//! CITATION_COLLISION `path: None` bug). One `Finding` ends it; every gate returns
//! `Vec<Finding>`, and REG-KERNEL-001's "name the instrument behind every denial"
//! is a first-class field (`citation`).

use crate::types::Severity;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Finding {
    pub severity: Severity,
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    pub message: String,
    /// The instrument behind the finding (REG-KERNEL-001: every denial names its law).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggested_fix: Option<String>,
}

impl Finding {
    pub fn new(severity: Severity, code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity,
            code: code.into(),
            path: None,
            message: message.into(),
            citation: None,
            suggested_fix: None,
        }
    }
    pub fn at(mut self, path: impl Into<PathBuf>) -> Self {
        self.path = Some(path.into());
        self
    }
    pub fn citing(mut self, citation: impl Into<String>) -> Self {
        self.citation = Some(citation.into());
        self
    }
    pub fn fix(mut self, fix: impl Into<String>) -> Self {
        self.suggested_fix = Some(fix.into());
        self
    }
    pub fn is_blocking(&self) -> bool {
        matches!(self.severity, Severity::Fatal | Severity::Error)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Report {
    pub ok: bool,
    pub findings: Vec<Finding>,
}

impl Report {
    /// Build a report from findings; `ok` is true iff nothing blocks.
    pub fn from_findings(findings: Vec<Finding>) -> Self {
        let ok = !findings.iter().any(Finding::is_blocking);
        Self { ok, findings }
    }
}
