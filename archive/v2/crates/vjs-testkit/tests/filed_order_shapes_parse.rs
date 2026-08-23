//! A filed order that does not PARSE is an order that does not BIND.
//!
//! This is the third instance of the same class in this repo's history. The
//! `supersedes` field once made SIX filed orders unparseable (recorded in the doc
//! comment on `Order::supersedes`), `privy` had to be accepted as an alias for
//! `privy_council`, and on 2026-07-28 two more were found in the SUB2
//! jurisdiction: `2026-VJS-CA-BOLTRIG-CODEX-APPROVAL-ROUTING-001` writes
//! `court: appeal`, and `2026-VJS-CC-BOLTRIG-CODEX-APPROVAL-ROUTING-001` writes
//! its `exceptions` as a single `|` prose block rather than a sequence.
//!
//! The failure mode is the dangerous one, and it is why this test exists rather
//! than a note in a doc comment: each order was authored, validated, committed,
//! and then **silently absent from the citator**. `vjs route` printed a warning to
//! stderr and carried on returning authorities, so the order bound nothing while
//! everything downstream looked healthy. Nothing went red.
//!
//! The standing rule is `never rewrite a filed record to satisfy a struct; widen
//! the struct` - so these assertions are pinned on the SHAPES THAT ARE ON FILE,
//! not on shapes we would prefer. Each case below is a verbatim extract of a real
//! filed order.
//!
//! MUTATION-CHECKED 2026-07-28: removing `#[serde(alias = "appeal")]` fails
//! `court_appeal_alias_parses` and nothing else; removing the
//! `string_or_seq_opt` deserialiser fails `exceptions_as_prose_block_parses` and
//! nothing else. Neither removal fails the other test, so the two cases are
//! independently guarded rather than one assertion doing both jobs.

use vjs_core::types::{Court, Order};

/// A minimal but REAL `Order`, so these assertions exercise the shipped struct
/// rather than a stand-in. An earlier draft of this test declared its own
/// `exceptions: Option<Vec<String>>` and therefore proved nothing about the type
/// the resolver actually reads - the same "assert on the artefact that ships"
/// mistake this whole test guards against.
fn order_yaml(extra: &str) -> String {
    format!(
        "id: TEST-ORDER-001\ncourt: county\njurisdiction: default\nrepo_code: null\n\
         status: binding\nissue: test.issue\nholding: a holding\ndirectives: []\n\
         forbidden: null\nsource_opinion: null\ncreated_at: 2026-07-28T00:00:00Z\n{extra}"
    )
}

/// `court: appeal`, as filed in 2026-VJS-CA-BOLTRIG-CODEX-APPROVAL-ROUTING-001.
#[test]
fn court_appeal_alias_parses() {
    let court: Court = serde_yaml::from_str("appeal").expect(
        "the filed order 2026-VJS-CA-BOLTRIG-CODEX-APPROVAL-ROUTING-001 writes `court: appeal`; \
         if this fails the order is absent from the citator and binds nothing",
    );
    assert_eq!(court, Court::CourtOfAppeal);
}

/// The canonical spelling must keep working; an alias must not displace it.
#[test]
fn court_canonical_spellings_still_parse() {
    assert_eq!(
        serde_yaml::from_str::<Court>("court_of_appeal").unwrap(),
        Court::CourtOfAppeal
    );
    assert_eq!(
        serde_yaml::from_str::<Court>("county").unwrap(),
        Court::County
    );
    assert_eq!(
        serde_yaml::from_str::<Court>("privy").unwrap(),
        Court::PrivyCouncil
    );
}

/// A court that is not a court must STILL be refused. Widening the reader for a
/// filed shape must not turn the enum into a shrug: if this ever passes, the
/// aliases have been replaced by something that accepts anything, and a
/// misfiled order would then be read as a real one.
#[test]
fn an_unknown_court_is_still_refused() {
    assert!(serde_yaml::from_str::<Court>("high_court").is_err());
    assert!(serde_yaml::from_str::<Court>("").is_err());
}

/// `exceptions:` as a `|` prose block, as filed in
/// 2026-VJS-CC-BOLTRIG-CODEX-APPROVAL-ROUTING-001. A single string means one
/// exception, so it reads as a one-element list.
#[test]
fn exceptions_as_prose_block_parses() {
    // Verbatim shape from the filed order (abridged in length, not in form).
    let yaml = order_yaml(
        "exceptions: |\n  Contingency: should the pinned binary's ordering ever be\n  POST-forward, the handler must instead BLOCK on the kernel decision.\n",
    );
    let parsed: Order = serde_yaml::from_str(&yaml).expect(
        "the filed order 2026-VJS-CC-BOLTRIG-CODEX-APPROVAL-ROUTING-001 writes `exceptions` as a \
         prose block; if this fails the order is absent from the citator and binds nothing",
    );
    let got = parsed.exceptions.expect("exceptions should be present");
    assert_eq!(got.len(), 1, "a single prose block is ONE exception");
    assert!(got[0].contains("Contingency"));
}

/// The sequence form is the common one and must keep working unchanged.
#[test]
fn exceptions_as_sequence_still_parses() {
    let parsed: Order =
        serde_yaml::from_str(&order_yaml("exceptions:\n  - first\n  - second\n")).unwrap();
    assert_eq!(
        parsed.exceptions.unwrap(),
        vec!["first".to_string(), "second".to_string()]
    );
}

/// An absent `exceptions` is not an error, and is not an empty list either -
/// "this order states no exceptions" and "this order states zero exceptions" are
/// the same thing here, but None must not become Some(vec![]) silently.
#[test]
fn absent_exceptions_stays_none() {
    let parsed: Order = serde_yaml::from_str(&order_yaml("")).unwrap();
    assert!(parsed.exceptions.is_none());
}
