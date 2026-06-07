# Statutory Instruments - the subordinate-law register

The realm's register of **statutory instruments (SIs)**: subordinate legislation of binding operative
effect, made under the Statutory Instruments framework (the Statutory Instruments (Framework) Act 2026,
Bill 26, in force) in the new Bill 14 s. 11(1)(g) tier, strictly below the Acts of the Realm and below
case law.

## How an SI is made

- **Maker:** the **Standing Committee of the Legislature** makes (and amends) statutory instruments
  (Bill 26 s. 15) **in exercise of the power conferred on a parent office** by that office's enabling
  clause (Bill 26 s. 14). The parent office is the authority on the Bill 14 s. 8 roll; the Committee is
  the maker. (No Royal Assent: SIs are subordinate. Primary Acts still require the Sovereign's assent.)
- **Procedure:** negative by default (published, then the Bill 14 s. 14 objection window; commences on
  lapse without a valid objection), affirmative where the parent Act so states, emergency only through
  the Bill 9 cage.
- **Citation (Form C, [2026] REALM-PC 11):** a flat annual ordinal `[YEAR] REALM-SI N`, always shown
  with a derived parent tag `(under Bill NN)` read mechanically from the instrument's enabling recital
  (the engine, `cli/lib/citation.js`). An amending SI is a fresh flat SI with its own tag; the amend
  link is a `supersedes:` / `amended-by:` pointer in the register, never in the number.
- **Status (closed vocabulary, Bill 16 s. 15(1A)):** `made`, `in-force`, `amended`, `revoked`, `spent`.
- **Enforcement (Bill 13 s. 5A):** machine-checkable rules run on the deterministic fail-closed gate
  (CASE-LAW s. 19(5)) with an audited break-glass (Bill 13 s. 7); soft rules via the watchdog; disputes
  to the single judiciary. Never punitive (Bill 13 s. 5(4)).

This register is a **derived, pointer-only** member of the realm's register family (with the citator,
the rulings ledger, the reasons ledger): the committed instrument markdown is canonical
([2026] REALM-PC 4); this index points to it.

## Register

See [`INDEX.md`](INDEX.md).

**UP:** [`../README.md`](../README.md) (the statute book). **Framework:** the Statutory Instruments
(Framework) Act 2026 ([`../../legislature/bills/26-statutory-instruments-framework-act-2026.md`](../../legislature/bills/26-statutory-instruments-framework-act-2026.md)).
