//! A REFERRAL record in the orders store reads as a referral, and nothing else does.
//!
//! Adopted upstream 2026-08-06 with the type it proves (vjs_core::ReferralRecord): a
//! subscribing jurisdiction may refer a matter UP but may not hold its own Supreme
//! sitting, so a locally-recorded apex "ruling" is re-characterised as a referral by
//! `[2026] VJS-SC 4`'s own holding, and the reader is widened rather than the record
//! edited ([2026] VJS-CC-OPBOX 160 O1). The subscriber's own suite additionally proves
//! its one FILED referral against the real file; that record is subscriber-local, so
//! here the controls are the door's limbs and the overlay behavior itself.
//!
//! WHAT THESE TESTS ARE FOR. The risk in the widening is not that it fails; it is that
//! it succeeds too broadly and quietly swallows a genuinely broken order out of the
//! unreadable count - the same fail-open shape as the 55 unreadable orders that started
//! the whole matter. So every limb gets a negative control, and the overlay pair proves
//! the door ANNOUNCES a referral without counting it unreadable while a broken order
//! still refuses.

use std::path::PathBuf;
use vjs_core::ReferralRecord;

// ---------------------------------------------------------------------------
// NEGATIVE CONTROLS. Each removes exactly one limb and must be REFUSED.
// ---------------------------------------------------------------------------

#[test]
fn a_record_carrying_a_holding_is_not_a_referral() {
    let y = r#"
id: fake
correction_note: looks like a correction
referral:
  from: somewhere
  apex_ruling: "[2026] VJS-SC 4"
holding: |
  This is operative text, so this file is an ORDER that mentions a referral.
"#;
    let r: ReferralRecord = serde_yaml::from_str(y).unwrap();
    assert!(
        !r.is_referral_not_order(),
        "a file with a holding must stay an order and stay counted"
    );
}

#[test]
fn a_record_carrying_directives_is_not_a_referral() {
    let y = r#"
id: fake
correction_note: looks like a correction
referral:
  from: somewhere
  apex_ruling: "[2026] VJS-SC 4"
directives:
- id: D1
  must: do a thing
"#;
    let r: ReferralRecord = serde_yaml::from_str(y).unwrap();
    assert!(
        !r.is_referral_not_order(),
        "a file with operative directives must stay an order and stay counted"
    );
}

#[test]
fn a_referral_naming_no_apex_ruling_is_refused() {
    let y = r#"
id: fake
correction_note: corrected, allegedly
referral:
  from: somewhere
  apex_ruling: "   "
"#;
    let r: ReferralRecord = serde_yaml::from_str(y).unwrap();
    assert!(
        !r.is_referral_not_order(),
        "without an apex citation there is nothing to refer TO, so it is an order missing its holding"
    );
}

#[test]
fn a_referral_with_no_correction_note_is_refused() {
    let y = r#"
id: fake
correction_note: "  "
referral:
  from: somewhere
  apex_ruling: "[2026] VJS-SC 4"
"#;
    let r: ReferralRecord = serde_yaml::from_str(y).unwrap();
    assert!(!r.is_referral_not_order(), "a referral must say it is one");
}

/// The door must not open for an ordinary broken order. This is the limb that matters
/// most: the whole risk of this widening is a broken order going quiet.
#[test]
fn an_order_with_no_referral_block_does_not_reach_the_door_at_all() {
    let y = r#"
id: 2026-VJS-CC-VJS-999
court: county
jurisdiction: vibe-justice-system
status: binding
issue: something
"#;
    assert!(
        serde_yaml::from_str::<ReferralRecord>(y).is_err(),
        "no `referral:` block means this cannot deserialise as a referral, so the loader reports it \
         unreadable exactly as before"
    );
}

/// Invalid YAML stays invalid: no reader reaches a document the parser cannot tokenise,
/// and this door must not appear to.
#[test]
fn invalid_yaml_is_still_invalid() {
    let y = r#"
correction_note: nope
referral:
  from: (DEC-15; CC-VJS 31/35): ON_HOLD
  apex_ruling: "[2026] VJS-SC 4"
"#;
    assert!(
        serde_yaml::from_str::<ReferralRecord>(y).is_err(),
        "a document that does not tokenise must not be admitted as a referral"
    );
}

// ---------------------------------------------------------------------------
// THE OVERLAY PAIR: announced, never swallowed; and a broken order still refuses.
// ---------------------------------------------------------------------------

fn scratch(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("vjs-referral-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(d.join(".vjs/orders")).unwrap();
    d
}

const REFERRAL: &str = r#"id: 2026-VJS-SC-SUBX-001
correction_note: recorded in error as a local apex order; re-characterised per the apex holding
referral:
  from: "[2026] VJS-CC-SUBX 79"
  apex_ruling: "[2026] VJS-SC 4"
"#;

/// A valid referral in the orders store is ANNOUNCED and does not trip the O5 gate:
/// it is not an unreadable order, and it confers no force (it never enters `orders`).
#[test]
fn a_filed_referral_does_not_trip_the_unreadable_gate() {
    let repo = scratch("clean");
    std::fs::write(repo.join(".vjs/orders/2026-VJS-SC-SUBX-001.yaml"), REFERRAL).unwrap();
    vjs_engine::context::refuse_if_orders_unreadable(&repo)
        .expect("a referral is a READ record, not an unreadable order");
    let _ = std::fs::remove_dir_all(&repo);
}

/// The same store with a genuinely broken order still REFUSES, naming the broken file
/// and not the referral - so the widening opened exactly one door and no other.
#[test]
fn a_broken_order_beside_a_referral_still_refuses_and_names_only_itself() {
    let repo = scratch("broken");
    std::fs::write(repo.join(".vjs/orders/2026-VJS-SC-SUBX-001.yaml"), REFERRAL).unwrap();
    std::fs::write(
        repo.join(".vjs/orders/2026-VJS-CC-VJS-998.yaml"),
        "id: 2026-VJS-CC-VJS-998\ncourt: county\n", // no holding/directives/issue: unreadable
    )
    .unwrap();
    let e = vjs_engine::context::refuse_if_orders_unreadable(&repo)
        .expect_err("a broken order must still refuse");
    let msg = e.to_string();
    assert!(
        msg.contains("2026-VJS-CC-VJS-998"),
        "the refusal names the broken order: {msg}"
    );
    assert!(
        !msg.contains("SC-SUBX-001"),
        "the referral is not among the unreadable: {msg}"
    );
    let _ = std::fs::remove_dir_all(&repo);
}

// ---------------------------------------------------------------------------
// THE OTHER TWO DOORS. The door above was widened in `vjs-engine` alone, so the
// two remaining order readers still stated the rule their own way and drifted -
// the CC-OPBOX 16 C1 shape, a second statement of where records go. `vjs-store`
// hard-failed on the referral (so `vjs status` could not run at all against a
// store holding one) and `vjs-core::repo` silently dropped it into a set that
// feeds `evaluate_invariants`. Each now gets the same pair: the referral passes
// and is not counted, and a genuinely broken order still does what it did.
// ---------------------------------------------------------------------------

/// `Store::read_orders` backs `vjs status`. A referral must not take the command down,
/// and must not be counted as an order either.
#[test]
fn the_store_reader_reads_past_a_referral_without_counting_it() {
    let repo = scratch("store-clean");
    std::fs::write(repo.join(".vjs/orders/2026-VJS-SC-SUBX-001.yaml"), REFERRAL).unwrap();
    std::fs::write(
        repo.join(".vjs/orders/2026-VJS-CC-VJS-997.yaml"),
        "id: 2026-VJS-CC-VJS-997\ncourt: county\njurisdiction: vjs\nstatus: binding\nissue: x\n\
         holding: a real holding\ndirectives: []\n",
    )
    .unwrap();
    let orders = vjs_store::Store::read_orders(&repo)
        .expect("a referral must not fail the whole read: this is what broke `vjs status`");
    assert_eq!(orders.len(), 1, "the referral is not admitted as an order");
    assert!(
        orders.iter().all(|o| !o.id.contains("SUBX")),
        "the referral must never reach the citator"
    );
    let _ = std::fs::remove_dir_all(&repo);
}

/// NEGATIVE CONTROL for the store reader: a genuinely broken order must STILL fail
/// closed. The fix must not have converted a hard failure into a silent skip.
#[test]
fn the_store_reader_still_fails_closed_on_a_broken_order() {
    let repo = scratch("store-broken");
    std::fs::write(repo.join(".vjs/orders/2026-VJS-SC-SUBX-001.yaml"), REFERRAL).unwrap();
    std::fs::write(
        repo.join(".vjs/orders/2026-VJS-CC-VJS-996.yaml"),
        "id: 2026-VJS-CC-VJS-996\ncourt: county\n", // no holding: unreadable, not a referral
    )
    .unwrap();
    assert!(
        vjs_store::Store::read_orders(&repo).is_err(),
        "a broken order must still refuse - widening the door must not open it for everything"
    );
    let _ = std::fs::remove_dir_all(&repo);
}

/// `build_repo_state` feeds `evaluate_invariants`. A referral must not be counted as an
/// order there either, or an invariant would be evaluating against a record apex law says
/// confers no force.
#[test]
fn the_invariant_state_reader_does_not_count_a_referral_as_an_order() {
    let repo = scratch("state");
    std::fs::write(repo.join(".vjs/orders/2026-VJS-SC-SUBX-001.yaml"), REFERRAL).unwrap();
    let state = vjs_core::RepoScanner::build_repo_state(&repo)
        .expect("building state must not fail on a referral");
    assert!(
        state.orders.is_empty(),
        "a referral confers no force and must not enter the invariant evaluation set"
    );
    let _ = std::fs::remove_dir_all(&repo);
}
