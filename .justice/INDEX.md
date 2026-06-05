# Caselaw Index (Citator)

This file is the master citator for the Vibe Justice System. It is the authoritative index of every ruling handed down in this realm. It is updated whenever a new ruling is committed to `.justice/judgments/`.

## What this index is

The citator answers three questions at a glance:

1. **What was decided?** (the ratio, in one line)
2. **Is it still good law?** (the status column)
3. **How does it connect to other law?** (the cites and scope columns)

It is also the first place Lexby checks before convening a bench. If a point is covered by a binding ratio on all fours with the matter in hand, the VPR 2 fast path disposes of it on citation, no sitting required. A citator that is out of date costs you a sitting you did not need to have.

## How to use it

- To check whether a question is already settled: scan the `ratio` column.
- To check whether a ruling is still binding: read the `status` column (see Status Key below).
- To find the full text of a ruling: open `.justice/judgments/<tier>/<file>` - e.g. `.justice/judgments/supreme-court/2026-LEXBY-SC-1.md`.
- To trace how a ruling interacts with statute: read the `cites` column (statute it relies on) and check SPEC-LAW.md for any articles it enacted.

## Column guide

| Column | Meaning |
|--------|---------|
| **Citation** | The neutral citation. Form: `[YEAR] LEXBY-<TIER> <N>`. Tier codes: SC (Supreme Court), CA (Court of Appeal), FI (First Instance). Link goes to the full ruling file. |
| **Tier** | The court that issued the ruling: `supreme`, `appeal`, or `first-instance`. |
| **Status** | Whether the ruling is currently good law. See Status Key. |
| **Ratio (one line)** | The binding holding, stripped to its minimum. Obiter is not included here. |
| **Scope** | Which repos, matters, or subject areas the ruling governs. `all repos` means realm-wide. |
| **Cites** | Statute (s. n) or prior case law the ruling expressly relies on or enacts. |

## How to cite a ruling

Neutral citation form (SPEC-LAW s. 11(d)):

```
[YEAR] LEXBY-SC n      (Supreme Court)
[YEAR] LEXBY-CA n      (Court of Appeal)
[YEAR] LEXBY-FI n      (First Instance)
```

In prose, give the citation and then the point of ratio you are relying on. Example:

> The duty of care arises from the relationship itself, not from any enacted statute ([2026] LEXBY-SC 1, ratio I).

Only the ratio binds. Obiter dicta are persuasive only. A ruling made in ignorance of binding statute or prior precedent is per incuriam and void (s. 11(e)).

---

## Rulings

| Citation | Tier | Status | Ratio (one line) | Scope | Cites |
|----------|------|--------|------------------|-------|-------|
| [[2026] LEXBY-SC 1](judgments/supreme-court/2026-LEXBY-SC-1.md) | supreme | good-law | Breach is the tort of negligence (duty + graded endeavours + remediation only); the system is a unitary realm of parliamentary sovereignty (one global SPEC-LAW, jurisdiction-local case law, one apex court). | All repos, all matters (constitutional founding ruling) | s. 1, s. 2, s. 3 (founding); enacts s. 4 through s. 12 |
| [[2026] LEXBY-FI 1](judgments/high-court/2026-LEXBY-FI-1.md) | first-instance | good-law | A governance system is fit for alpha release where the core legal model is coherent, statute + procedure + founding caselaw are committed, constitutional enforcement is in place, and known gaps are disclosed; convenience-layer tooling is not a prerequisite for alpha; deterministic citation numbering is a necessary condition for v1. | vibe-justice-system repo (alpha-readiness standard for governance systems) | s. 4, s. 5, s. 7, s. 8, s. 11(d), s. 14 |
| [[2026] LEXBY-FI 2](judgments/high-court/2026-LEXBY-FI-2.md) | first-instance | good-law | VJS is ready for public outreach under reasonable skill and care provided the README carries a prominently placed known-limitations section disclosing the three material gaps before any outreach act; public outreach is a materially distinct standard from alpha release (mass unsolicited audience, author credibility at stake); coherence and honest disclosure are necessary and sufficient; feature completeness is not required. | vibe-justice-system repo (public outreach readiness) | s. 4, s. 5, s. 8, s. 11(d); distinguishes [2026] LEXBY-FI 1 (alpha vs outreach standard) |
| [[2026] LEXBY-CA 1](judgments/appeals-court/2026-LEXBY-CA-1.md) | appeal | good-law | Appeal dismissed unanimously. The s. 8/s. 5 sequencing complaint is not made out on a fair reading of [2026] LEXBY-FI 1; further, s. 8 cannot be applied without implicitly applying s. 5 (the reasonableness in s. 8 is the s. 5 standard), so the sequencing complaint describes a structural impossibility. [2026] LEXBY-FI 1 confirmed as good law. | All repos (s. 5/s. 8 sequencing; appellate standards) | s. 5, s. 8, s. 12; affirms [2026] LEXBY-FI 1 |
| [[2026] LEXBY-SC 2](judgments/supreme-court/2026-LEXBY-SC-2.md) | supreme | good-law | s. 4 imposes a proactive disclosure obligation: Lexby must raise known material risks to the principal's project even when not asked; triggered by actual knowledge and materiality; silence in the face of a known material risk is breach; remedy under s. 6 is disclosure and restoration. Majority 4:1 (Blackmere J dissenting). | All repos, all engagements (general application) | s. 3, s. 4, s. 5, s. 6, s. 7, s. 13; leapfrog certificate; extends [2026] LEXBY-SC 1 |

---

## Status Key

| Status | Meaning |
|--------|---------|
| **good-law** | The ruling stands and binds. It may be cited and applied without qualification. |
| **distinguished** | The ruling is good law on its own facts but a later court found the facts of the current matter materially different. It does not control the distinguished case but is otherwise unaffected. |
| **overruled** | A higher court (or the same court in a later sitting) has expressly set the ruling aside. It no longer binds. The overruling case must be cited instead. |
| **superseded-by-statute** | The ratio has been replaced by an enacted SPEC-LAW article. The ruling is of historical interest only; the statute controls. |
| **per-incuriam** | The ruling was made in ignorance of a binding statute or prior precedent. It is void and does not bind. A fresh sitting is required to resolve the point. |

---

## Adding a new ruling

When a new ruling is committed to `.justice/judgments/`:

1. Add a row to the Rulings table above, in ascending citation order within each year.
2. Fill every column. Do not leave `ratio` vague - one tight line is better than a paragraph.
3. Set `status` to `good-law` unless the ruling itself overrules or supersedes an earlier one.
4. If the new ruling overrules or supersedes an existing entry, update that row's status to `overruled` or `superseded-by-statute` and add a bracketed note: `(overruled by [YEAR] LEXBY-SC n)`.
5. If new SPEC-LAW articles are enacted by the ruling, record them in the `cites` column and update `SPEC-LAW.md`.
6. Commit the ruling file and this index in the same commit. The citator must never lag the caselaw.
