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
.-------------------------.                                 |      SUPREME COUNCIL         |
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
| VPR 3 | Progression is mandatory and in order: First Instance, Court of Appeal, Supreme Council; leave required at each step. |
| VPR 4 | The Sovereign may issue a leapfrog certificate to take a matter straight to the Supreme Council; the sole exception to VPR 3. |
| VPR 5 | Bench sizes: First Instance 1, Court of Appeal 3, Supreme Council 5 (expanding to 9 for foundational questions). |
| VPR 6 | Every matter yields a neutral-citation ruling artefact; ratio binds, obiter persuades, per incuriam voids. |
| VPR 7 | No costs or sanctions jurisdiction; the only remedy is to make the work good. |
| VPR 8 | Every ruling, regardless of tier, is submitted to the Community Record (community/caselaw/YYYY/ in the canonical VJS repo) as a pull request. Submissions are anonymised: project-specific identifiers are stripped, legal facts preserved. |

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

A matter climbs the tiers **in order**: First Instance -> **Court of Appeal** (panel of 3) -> **Supreme Council** (panel of 5). Escalation at each step requires **permission to appeal** (leave), granted only on an arguable point of law or a binding-precedent conflict. A matter destined to change SPEC-LAW does **NOT** commence at the Supreme Council: the Supreme Council alone enacts statute, but it must be **reached by progression**. Lexby may not self-initiate at a higher tier; convening the Supreme Council directly is reserved.

---

### **`VPR 4`** - The leapfrog (the only exception)

The **Principal, acting as Sovereign**, may by **express instantiation** issue a **leapfrog certificate** taking a matter straight to the Supreme Council, bypassing the lower tiers. This is the **sole** exception to VPR 3. It must be express, it is recorded on the matter, and absent it the tiers are mandatory. (The tort recast, [2026] LEXBY-SC 1, proceeded under such a certificate.)

---

### **`VPR 5`** - The bench

First Instance sits **1**; Court of Appeal sits **3**; the Supreme Council sits **5**, expanding to the full **9** only for constitutional or foundational questions. Judges are drawn from the benches (the puisne pool / the Supreme Council) and seeded with **ephemeral stances only for matters of first impression**; a settled ratio is followed, not re-polled.

---

### **`VPR 6`** - Judgment, record, remedy

The bench speaks in legalese; **Lexby translates**. Every matter yields a ruling artefact with two identifiers: a tier-coded neutral citation `[YEAR] LEXBY-FI N` / `[YEAR] LEXBY-CA N` / `[YEAR] LEXBY-SC N` (human-readable, local), and a **UUID** (globally unique, assigned at judgment and fixed). The UUID is the canonical reference for the Community Record and for cross-project citation; the neutral citation is for human use within the project. The **ratio** binds, **obiter** is persuasive, **per incuriam** voids a ruling made in ignorance of binding law. A breach made out is met by **remediation and restitution only**, never punishment. The court's remediation power extends to **any project artefact**: source code, configuration, documentation, data schemas, ledgers, and audit logs - wherever the proceedings reveal a record below standard, the court may issue an ancillary order. A ruling that cannot be reconciled with SPEC-LAW yields a **declaration of incompatibility**, referred up for amendment; the sovereign spec is never struck by a court.

---

### **`VPR 7`** - No costs, no sanctions

There is no costs or sanctions jurisdiction (anti-bloat, SPEC-LAW-12). The only remedy is to make the work good.

---

### **`VPR 8`** - Community Record

**Supreme Council rulings only** are submitted to the **Community Record** in the canonical VJS repo (`wlilley93/vibe-justice-system`, path `community/caselaw/YYYY/`) by opening a pull request. First Instance and Court of Appeal rulings remain in the project repo under `.justice/judgments/` and are not submitted upstream.

Before submission, the ruling is **anonymised**: repo names, file paths, variable names, function names, class names, and any project-specific identifiers are replaced by generic placeholders; the legal question, the ratio, the tier, the law applied, and the outcome are preserved unchanged. Community rulings are **persuasive precedent** across all VJS jurisdictions (any other repo running VJS). The clerk reviews each PR for constitutional compliance and subject-matter jurisdiction before merging. Supreme Council rulings that enact statute additionally open a separate statute PR touching `SPEC-LAW.md` (VPR 6).

---

> VPR governs procedure; SPEC-LAW governs substance. Where they touch, the binding articles are SPEC-LAW-10 (court structure), SPEC-LAW-11 (gates and devices), and SPEC-LAW-13 (rule-based progression and the leapfrog).
