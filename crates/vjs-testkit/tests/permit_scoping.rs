//! Permit-scoping: a permit excuses only the governed paths it names. A permit
//! with no scope blanket-covered everything (the gate was fatal in form but
//! hollow); this proves the fix - out-of-scope governed writes are blocked even
//! with an active permit.

use std::path::PathBuf;
use vjs_core::governance::PermitGate;
use vjs_core::spec::Permit;
use vjs_core::types::*;

fn permit(scope_paths: Option<Vec<&str>>) -> Permit {
    Permit {
        id: PermitId("PERMIT-TEST".into()),
        route_id: RouteId("ROUTE-TEST".into()),
        actor: "lexby".into(),
        scope: scope_paths.map(|ps| Scope {
            paths: Some(ps.into_iter().map(|s| s.to_string()).collect()),
            jurisdictions: None,
            action_kinds: None,
            issue_tags: None,
            records: None,
        }),
        obligations: Vec::new(),
        expires_at: "2099-01-01T00:00:00+00:00".into(),
        status: PermitStatus::Active,
    }
}

fn gate_ok(staged: &[&str], permits: &[Permit]) -> bool {
    let staged: Vec<PathBuf> = staged.iter().map(PathBuf::from).collect();
    let required = vec!["lawpack/**".to_string(), "crates/**".to_string()];
    PermitGate::evaluate(&staged, permits, &[], &[], &required, &[]).ok
}

#[test]
fn a_scoped_permit_covers_only_its_paths() {
    let p = permit(Some(vec!["lawpack/v2/orders/x.yaml"]));
    assert!(
        gate_ok(&["lawpack/v2/orders/x.yaml"], std::slice::from_ref(&p)),
        "an in-scope governed write passes"
    );
    assert!(
        !gate_ok(&["crates/vjs-core/src/route.rs"], std::slice::from_ref(&p)),
        "an OUT-OF-SCOPE governed write is blocked even with an active permit"
    );
}

#[test]
fn a_no_scope_permit_covers_nothing() {
    let p = permit(None);
    assert!(
        !gate_ok(&["lawpack/v2/orders/x.yaml"], std::slice::from_ref(&p)),
        "a no-scope permit no longer blanket-covers governed writes"
    );
}

#[test]
fn an_ungoverned_path_needs_no_permit() {
    assert!(gate_ok(&["notes/scratch.txt"], &[]), "an ungoverned path passes with no permit");
}
