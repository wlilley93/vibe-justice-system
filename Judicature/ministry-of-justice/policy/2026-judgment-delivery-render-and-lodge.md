# Policy briefing: deterministic delivery of a judgment (render to PDF and lodge)

**Type:** Ministry policy briefing (policy-arm: Ministry policy -> Standing Committee drafting)
**To:** the Ministry of Justice (MoJ), governance ministry of the Judicature
**From:** the Prime Minister (the Founder, acting in the executive office, CASE-LAW s.2)
**Subject:** whether the court system should deliver every judgment through a CLI / deterministic mechanism
that renders it to PDF and takes all actions necessary to properly lodge and file it
**Date:** 2026-06-06
**Status:** referred to the Standing Committee for drafting as a statutory instrument

> This is a **policy briefing**, not an instrument of law. The MoJ develops the policy; the Standing Committee
> drafts the binding instrument; the Sovereign enacts; the owning ministry implements. Authorship of a policy
> proposal is the advocate/advisor function (CASE-LAW s.3), not an exercise of law-making power.

---

## 1. The request (PM -> MoJ)

The court has, until now, rendered judgments to PDF by convention and by a pre-commit hook. The PM asks the
MoJ to consider, as a matter of policy, whether the **delivery of a judgment** should be a **deterministic,
first-class act of the court system**: on delivery of a judgment the system should **render it to a formal
PDF** and **take all actions necessary to properly lodge and file it** (the citator row, the derived
projections, the permanent record), with **no step left to model judgement**. If the MoJ recommends this be
set as law, the briefing asks that the instrument **mandate the Engineering department (Ministry of Business,
Engineering and Skills) to take all actions necessary to implement and remedy**.

## 2. The MoJ's policy analysis

The realm already treats two record-integrity duties as deterministic, gate-enforced, fail-closed acts rather
than matters of discretion: citation numbering (`cdd next-citation`) and the citator-integrity gate
(CASE-LAW s.19(5); `[2026] REALM-PC 4`). Rendering and lodgement of a judgment is the same kind of duty: it is
clerical, it must never be forgotten, and it must never depend on an agent remembering to do it. The principle
of the projection-lockstep gate (the law-site corpus, the search index, and the rulings ledger rebuild in
lockstep whenever the law changes) applies equally to the judgment PDF: **the rendered judgment is a derived
projection of the filed judgment and must stay in lockstep with it.**

The MoJ therefore recommends that judgment delivery be specified as a deterministic mechanism with three
duties, each capable of running without a model in the loop:

1. **Render.** On a new or amended judgment, render it to a formal court PDF (the established cream A4 form),
   idempotently: render only when the PDF is missing or older than its source; never re-commit a PDF that
   changed only through non-deterministic re-render.
2. **Lodge.** Stage and record everything a properly filed judgment requires: the citator row (already
   gate-guarded), the derived projections (corpus, search index, rulings ledger), and the rendered PDF.
3. **Verify, fail-closed where the record's integrity is at stake** (citation/citator), **fail-open for the
   convenience layer** (projections, PDF), consistent with the existing gate's split.

The MoJ further recommends that the mechanism be exposed as a **first-class CLI verb** (for example
`cdd lodge-judgment <file>` / `cdd render-judgment <file>`), so that delivery is one auditable command, and
that the pre-commit gate continue to invoke it automatically so a filed judgment is always rendered and lodged.

## 3. Recommendation: set it as a statutory instrument owned by the court system, implemented by Engineering

The MoJ recommends the Standing Committee draft a **Judgment Rendering and Lodgement Instrument** (a statutory
instrument under the Bill 26 framework), in exercise of the court-administration / record-integrity power, that:

- enshrines the **principles** above (render, lodge, the fail-closed/fail-open split) as durable law, holding
  **no operative facts** (no scripts, no paths) in the instrument itself, consistent with the principles-at-law
  rule (Bill 27);
- **mandates the Engineering department of the Ministry of Business, Engineering and Skills (MBES)** to take
  all actions necessary to implement and maintain the mechanism (the renderer, the idempotent render-all step,
  the CLI verb, and the gate wiring), and to remedy any defect, as part of the engineering it owns under
  Bill 27; and
- is amendable by the ordinary SI amendment route as the mechanism evolves.

The engineering that gives effect to this already substantially exists (`render-all-judgments.sh` and the
pre-commit render-and-stage step). Setting it as an instrument makes the duty **law** rather than convention,
names the **owner of the remedy** (Engineering), and brings judgment delivery under the same deterministic,
gate-enforced discipline as the rest of the record.

## 4. Referral

The MoJ refers this briefing to the **Standing Committee on the Laws of the Realm** to draft the Judgment
Rendering and Lodgement Instrument accordingly, the Committee being the drafting authority and the Sovereign
the enacting authority. The Engineering department of MBES is the proposed implementing body.

---

**UP:** [`../README.md`](../README.md) (the Ministry of Justice) · the realm: [`../../../README.md`](../../../README.md).
