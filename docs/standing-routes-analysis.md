# Standing-bounded assent routes (improvement #2)

[2026] VJS-PC 16 settled that a `standing_bounded_assent` record resolves only by
"tracing to specific Sovereign assent", and **reserved** the per-class standing-route
instruments as a deferred tightening (PC-16 D4; recorded in
`BREACH-2026-06-25-assent-resolution-under-implementation`, `deferred:`). This note
records the routes explicitly and scopes what remains.

## The recognised standing routes (what each class traces to)

| Class | Standing route | Terminal Sovereign assent |
|---|---|---|
| Court **orders** (County / Privy / Supreme) | issued by a **constituted bench** under ACT-002 (the Courts and Orders Act) | Bill 32 founding assent, `COMMENCEMENT-V2-0001` condition D (the courts-constitution `2026-VJS-COURTS-CONSTITUTION-001`) |
| **Regulations** (REG-*) | made under **ACT-CONSOLIDATION-FRAMEWORK s.7** (the machinery power) | Bill 32 + the Framework Act assent, `COMMENCEMENT-V2-0002` (`framework_act_text_digest`) |
| Founding statutes / invariants / boot records | lodged at commencement | `COMMENCEMENT-V2-0001/0002`, Bill 32 royal assent |

All of these trace, ultimately, to the **one** founding Sovereign assent (Bill 32,
2026-06-09) recorded under `lawpack/v2/provenance/founding/`.

## What is already closed

The **high-value forgery vector - a bench-less / never-convened apex or court order -
is closed**, independently of how permissively `standing_bounded_assent` resolves:
the bench-integrity and apex-singleness findings are **constitutive** ([2026] VJS-PC 16),
so no assent claim of either form softens them. A forged order is blocked whether it
declares `sovereign_assent` (which does not resolve) **or** `standing_bounded_assent`
(which resolves permissively but cannot launder a constitutive defect). This is proven
end-to-end in `crates/vjs-testkit/tests/e2e_gate_harness.rs`
(`forged_fresh_apex_order_stays_fatal_through_the_pipeline`,
`forged_standing_bounded_order_is_still_blocked`,
`constitutive_bench_defect_not_downgraded_even_for_an_established_assented_order`).

## The residual (and why it is not rushed)

`standing_bounded_assent` currently resolves by tracing to the realm's Sovereign
foundation (a `sovereign_assent_event` exists). The remaining gap is narrow: laundering
a **non-constitutive** Fatal (e.g. a malformed field) on a **non-order** record (a
regulation) by typing `standing_bounded_assent`. In practice the surface is tiny -
regulation defects are usually advisory (`S5_INERT_KERNEL_EFFECT` is a Warning), and the
high-value class (orders/apex) is already closed.

Fully closing the residual requires **per-instrument enactment provenance**: each
regulation carrying (or being named by) a record that traces its enactment under the
Framework Act s.7 route to the founding assent, exactly as the founding corpus is named
by `COMMENCEMENT-V2-0001/0002`. That is a **migration**, not a code tweak, and tightening
how the entrenched term `standing_bounded_assent` resolves is a **refinement of an
entrenched term** - the same character as PC-16 itself, and therefore a matter for the
Privy Council, not a decisive call. It is reserved on that footing. The interim backstop
is the constitutive-codes exclusion (above) plus the continuing duty of care
(ACT-003 s.4-s.8), exactly as PC-16 D4 recorded for the irreducible remainder.
