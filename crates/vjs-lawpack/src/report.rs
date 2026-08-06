//! The validation report types (ValidationReport / ValidationFinding) and the duty-token tables
//! the validator emits.

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidationReport {
    pub ok: bool,
    pub findings: Vec<ValidationFinding>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidationFinding {
    pub severity: Severity,
    pub code: String,
    pub path: Option<PathBuf>,
    pub message: String,
    pub suggested_fix: Option<String>,
}
