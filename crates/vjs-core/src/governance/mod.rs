//! Governance: path classification + the permit gate.
//!
//! Split into two cohesive files - `path_class` (the Governed/Exempt/Ungoverned classifier + the
//! kernel's ONE glob engine) and `permit_gate` (the live pre-write permit authorization, routed
//! through the unified capability primitive). The public surface is re-exported unchanged.

mod path_class;
mod permit_gate;

pub use path_class::{PathClassification, PathClassifier};
pub use permit_gate::{
    PathScope, PermitGate, PermitGateFinding, PermitGateResult, path_escapes_root,
};
