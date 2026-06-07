<div align="center">

```
  +---------------+
  | INTAKE        |
  |  -> GATE      |
  |  -> BENCH     |
  |  -> RULING    |
  +---------------+
```

# VPR

**Vibe Procedure Rules**

</div>

---

## Matter Flow

```
INTAKE
  |
  +- Standing check fails -------------------------------------------------> DISMISSED
  |
  +- Binding ratio on all fours? -----------> FAST PATH: disposed on citation (no bench)
  |
  |  [Leapfrog certificate from Sovereign]
  +- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - +
  |  (bypasses all lower tiers, express certificate required)              |
  |                                                                        v
  v                                                          .-----------------------------.
.-------------------------.                                 |      SUPREME COURT         |
|    FIRST INSTANCE       |                                 |        bench of 5            |
|       bench of 1        |                                 |  (9 for foundational         |
'-------------------------'                                 |   questions)                 |
  |                                                         '-----------------------------'
  | permission to appeal                                                   ^
  | (arguable point of law /                                               |
  |  binding-precedent conflict)                                           |
  v                                                                        |
.-------------------------.                                                |
|    COURT OF APPEAL      |  permission to appeal -------------------------+
|       bench of 3        |  (same gate)
'-------------------------'
```

---

## Rule Index

| Rule | Summary |
|------|---------|
| VPR 1 | Every matter commences at First Instance via a Request for Ruling or a Breach; standing checked at intake. |
| VPR 2 | Binding ratio on all fours disposes the matter on citation without a sitting (fast path). |
| VPR 3 | Progression is mandatory and in order: First Instance, Court of Appeal, Supreme Court; leave required at each step. |
| VPR 4 | The Sovereign may issue a leapfrog certificate to take a matter straight to the Supreme Court; the sole exception to VPR 3. |
| VPR 5 | Bench sizes: First Instance 1, Court of Appeal 3, Supreme Court 5 (expanding to 9 for foundational questions). Every bench is odd; the size is the TOTAL deciding membership; the judgment is written by one counted member, never a synthesiser added on top (s. 18). |
| VPR 6 | Every matter yields a neutral-citation ruling artefact; ratio binds, obiter persuades, per incuriam voids. |
| VPR 7 | No costs or sanctions jurisdiction; the only remedy is to make the work good. |
| VPR 8 | Community Record publication is opt-in unless the local sovereign's law says otherwise. Local rulings stay in the project repo unless the local Principal chooses to submit an anonymised ruling to a community record. Canonical VJS accepts submissions by pull request under its maintainer rules. |
| VPR 9 | Duty to self-appeal (s. 17): on a valid appellate ground (per incuriam under s. 11(e), binding-precedent conflict, or an unmoored extension under s. 17), Lexby must on its own motion seek permission to appeal BEFORE implementing the impugned ruling irreversibly; the Principal need not prompt it. Reversible provisional steps, honestly flagged, may proceed pending determination. |

---

How a matter is brought, how it moves through the courts, and the gate at each step. Binding, and enforced by Lexby. Progression is RULE-BASED: you do not jump the queue.

---

### **`VPR 1`** - Commencement

Every matter commences at **First Instance** (a single judge), through one of two doors: a **Request for Ruling** (forward-looking, a fork) or a **Breach** (a charge in negligence that the duty of care was not met). **Standing** is checked at intake; a non-party cannot conjure a sitting.

---

### **`VPR 2`** - The fast path (most matters never sit)

Before any bench convenes, the citator is searched. If a point is governed by binding **ratio** on all fours, it is disposed of **on citation, with no sitting** (the precedent fast-path). The court convenes only for genuine first-impression matters, distinctions, overrulings, or contested breaches.

---

### **`VPR 3`** - Progression is rule-based; no leap-frogging

A matter climbs the tiers **in order**: First Instance -> **Court of Appeal** (panel of 3) -> **Supreme Court** (panel of 5). Escalation at each step requires **permission to appeal** (leave), granted only on an arguable point of law or a binding-precedent conflict. A matter destined to change CASE-LAW does **NOT** commence at the Supreme Court: the Supreme Court alone enacts statute, but it must be **reached by progression**. Lexby may not self-initiate at a higher tier; convening the Supreme Court directly is reserved.

---

### **`VPR 4`** - The leapfrog (the only exception)

The **Principal, acting as Sovereign**, may by **express instantiation** issue a **leapfrog certificate** taking a matter straight to the Supreme Court, bypassing the lower tiers. This is the **sole** exception to VPR 3. It must be express, it is recorded on the matter, and absent it the tiers are mandatory. (The tort recast, [2026] LEXBY-SC 1, proceeded under such a certificate.)

---

### **`VPR 5`** - The bench

First Instance sits **1**; Court of Appeal sits **3**; the Supreme Court sits **5**, expanding to the full **9** only for constitutional or foundational questions. Judges are drawn from the benches (the puisne pool / the Supreme Court) and seeded with **ephemeral stances only for matters of first impression**; a settled ratio is followed, not re-polled.

**Every bench is odd-numbered, and the stated size is the TOTAL deciding membership, not a minimum (CASE-LAW s. 18, [2026] LEXBY-SC 3).** The judgment of a multi-judge court is written by ONE of its counted members (the presiding member at the Court of Appeal; the Chief Justice or a designated justice at the Supreme Court), synthesising the majority; that member is one of the three, five, or nine and gains no extra voice by holding the pen. No synthesising, presiding, or pen-holding judge may be added ON TOP of the sized panel as a separate deciding voice. A workflow or harness that seats a synthesiser in addition to the sized panel is non-conforming and must be corrected so the Court of Appeal seats three (one of whom authors the judgment) and the Supreme Court five or nine on the same pattern. A ruling from a bench that is even, or that seated a deciding voice on top of the sized panel, is **void ab initio** for want of lawful constitution and is re-determined by a properly constituted court.

---

### **`VPR 9`** - Duty to self-appeal

On encountering a valid appellate ground - a per incuriam ruling (s. 11(e)), a binding-precedent conflict, or an **unmoored extension** (a doctrine, test, or prohibition with no grounding in CASE-LAW, binding precedent, or the governing instruction; s. 17) - **Lexby must, on its own motion, seek permission to appeal before implementing the impugned ruling in any irreversible or delivered form.** The Principal need not spot the ground; waiting for the Principal, or complying-and-reporting, is itself a falling-below of the standard (s. 5), remedied by making the work good (s. 6). Reversible, low-blast steps honestly flagged as provisional may proceed pending determination. This duty does not license re-litigating a settled ratio (s. 11(c)); a well-grounded but unwelcome ruling is followed, not appealed.

---

### **`VPR 6`** - Judgment, record, remedy

The bench speaks in legalese; **Lexby translates**. Every matter yields a ruling artefact with a neutral citation under the provenance scheme: `[YEAR] REALM-PC N`, `[YEAR] REALM-CA N`, `[YEAR] REALM-SC N`, a High Court division code, or a local `CC-<repo>` code where the local jurisdiction uses one. A local jurisdiction may add its own series by local law, provided the citator remains deterministic and collision-free. The **ratio** binds, **obiter** is persuasive, **per incuriam** voids a ruling made in ignorance of binding law. A breach made out is met by **remediation and restitution only**, never punishment. The court's remediation power extends to **any project artefact**: source code, configuration, documentation, data schemas, ledgers, and audit logs - wherever the proceedings reveal a record below standard, the court may issue an ancillary order. A ruling that cannot be reconciled with CASE-LAW yields a **declaration of incompatibility**, referred up for amendment; the sovereign spec is never struck by a court.

---

### **`VPR 7`** - No costs, no sanctions

There is no costs or sanctions jurisdiction (anti-bloat, CASE-LAW-12). The only remedy is to make the work good.

---

### **`VPR 8`** - Community Record

Publication to a **Community Record** is opt-in unless the local sovereign's own law makes it mandatory. By default, all local rulings remain in the project repo under `.justice/judgments/` and are not submitted upstream.

Where the local Principal chooses to contribute, the ruling is submitted to the chosen community record by pull request or that community's equivalent route. Before submission, the ruling is **anonymised**: repo names, file paths, variable names, function names, class names, and any project-specific identifiers are replaced by generic placeholders; the legal question, the ratio, the tier, the law applied, and the outcome are preserved unchanged. Community rulings are **persuasive precedent** across participating VJS jurisdictions unless a jurisdiction expressly subscribes to them as binding law. Canonical VJS reviews PRs for constitutional compliance, subject-matter jurisdiction, anonymisation, and maintainability before merging.

A local jurisdiction may subscribe to canonical VJS law, become independent, or join/create a different multilateral community by express local law. The subscription choice affects only that jurisdiction unless and until canonical VJS or another community accepts a contribution under its own rules. No VJS hook may automatically publish local case law to canonical VJS merely because a ruling was made.

---

> VPR governs procedure; CASE-LAW governs substance. Where they touch, the binding articles are CASE-LAW-10 (court structure), CASE-LAW-11 (gates and devices), and CASE-LAW-13 (rule-based progression and the leapfrog).
