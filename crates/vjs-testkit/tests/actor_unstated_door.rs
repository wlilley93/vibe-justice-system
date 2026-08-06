//! A directive that names no actor READS, as `UNSTATED` - never refused, never assigned.
//!
//! [2026] VJS-CC-OPBOX 160 O3, adopted upstream 2026-08-06: the strict `actor` parse
//! refused fifty-four of a subscriber's binding filed orders at the reconciliation
//! re-pull (the O5 gate caught the loss and named every file). A directive is a DUTY:
//! defaulting the bearer to anyone would be the reader deciding who is bound, and
//! refusing the record entirely strips the citator of binding precedent. The record is
//! read as written: nobody was named, and the sentinel says so in terms.

use std::path::PathBuf;
use vjs_core::types::{ACTOR_UNSTATED, Order};

const LEGACY_ORDER: &str = r#"id: 2026-VJS-CC-SUBX-037
court: county
jurisdiction: subx
status: binding
issue: legacy_fixture_issue
holding: a binding holding from the era before directives named their actor
directives:
- id: D1
  must: do the recorded thing
runtime_summary: a legacy order whose directive names no actor
created_at: "2026"
"#;

#[test]
fn an_actor_less_directive_reads_as_unstated_and_nothing_is_invented() {
    let o: Order = serde_yaml::from_str(LEGACY_ORDER)
        .expect("a legacy order with an actor-less directive must PARSE");
    assert_eq!(
        o.directives[0].actor, ACTOR_UNSTATED,
        "the sentinel is the exact spelling: the record says nobody was named"
    );
}

#[test]
fn a_directive_naming_an_actor_keeps_it() {
    let named = LEGACY_ORDER.replace("  must:", "  actor: engineer\n  must:");
    let o: Order = serde_yaml::from_str(&named).unwrap();
    assert_eq!(
        o.directives[0].actor, "engineer",
        "a named actor is never clobbered"
    );
}

/// The overlay consequence, which is what the fifty-four orders actually lost: an
/// actor-less legacy order is not an unreadable order and does not trip the O5 gate.
#[test]
fn a_legacy_order_does_not_trip_the_unreadable_gate() {
    let repo: PathBuf =
        std::env::temp_dir().join(format!("vjs-actor-unstated-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&repo);
    std::fs::create_dir_all(repo.join(".vjs/orders")).unwrap();
    std::fs::write(
        repo.join(".vjs/orders/2026-VJS-CC-SUBX-037.yaml"),
        LEGACY_ORDER,
    )
    .unwrap();
    vjs_engine::context::refuse_if_orders_unreadable(&repo)
        .expect("a legacy actor-less order is READ, not refused");
    let _ = std::fs::remove_dir_all(&repo);
}
