//! The duty-conformance audit (PC-13 D11): which kernel_effect duties in canon are bound to a gate.

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DutyConformance {
    pub instrument: String,
    pub section: Option<String>,
    pub kind: String, // must | must_not | prohibits
    pub token: String,
    pub gate: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConformanceReport {
    pub total: usize,
    pub wired: usize,
    pub unwired: usize,
    pub duties: Vec<DutyConformance>,
}

fn classify_token(token: &str) -> Option<String> {
    GATE_REGISTRY
        .iter()
        .chain(GATE_REGISTRY_TRIAGE.iter())
        .find(|(t, _)| *t == token)
        .map(|(_, g)| g.to_string())
}

fn push_duties(
    out: &mut Vec<DutyConformance>,
    instrument: &str,
    section: Option<&str>,
    ke: &KernelEffect,
) {
    let mut add = |kind: &str, list: &Option<Vec<String>>| {
        if let Some(items) = list {
            for token in items {
                out.push(DutyConformance {
                    instrument: instrument.to_string(),
                    section: section.map(|s| s.to_string()),
                    kind: kind.to_string(),
                    token: token.clone(),
                    gate: classify_token(token),
                });
            }
        }
    };
    add("must", &ke.must);
    add("must_not", &ke.must_not);
    add("prohibits", &ke.prohibits);
}

/// Enumerate every kernel_effect duty across in-force statutes and regulations and
/// classify each against the gate registry. Deterministic and total over the loaded
/// lawpack. (Order DIRECTIVES are one-time build instructions, not standing duties,
/// and are deliberately out of scope.)
pub fn conformance_audit(lawpack: &Lawpack) -> ConformanceReport {
    let mut duties = Vec::new();
    for statute in &lawpack.statutes {
        for section in &statute.sections {
            if let Some(ke) = &section.kernel_effect {
                push_duties(&mut duties, &section.id.0, Some(&section.id.0), ke);
            }
        }
    }
    for reg in &lawpack.regulations {
        if let Some(ke) = &reg.kernel_effect {
            push_duties(&mut duties, &reg.id.0, None, ke);
        }
    }
    let wired = duties.iter().filter(|d| d.gate.is_some()).count();
    let total = duties.len();
    ConformanceReport {
        total,
        wired,
        unwired: total - wired,
        duties,
    }
}
