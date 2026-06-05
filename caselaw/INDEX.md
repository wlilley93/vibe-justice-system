# Caselaw Index (Citator)

This file is the master citator for the Vibe Justice System. It is the authoritative index of every ruling handed down in this realm. It is updated whenever a new ruling is committed to `caselaw/`.

## What this index is

The citator answers three questions at a glance:

1. **What was decided?** (the ratio, in one line)
2. **Is it still good law?** (the status column)
3. **How does it connect to other law?** (the cites and scope columns)

It is also the first place Lexby checks before convening a bench. If a point is covered by a binding ratio on all fours with the matter in hand, the VPR 2 fast path disposes of it on citation, no sitting required. A citator that is out of date costs you a sitting you did not need to have.

## How to use it

- To check whether a question is already settled: scan the `ratio` column.
- To check whether a ruling is still binding: read the `status` column (see Status Key below).
- To find the full text of a ruling: open `caselaw/<file>` - the file name matches the citation, e.g. `2026-LEXBY-SC-1.md`.
- To trace how a ruling interacts with statute: read the `cites` column (statute it relies on) and check SPEC-LAW.md for any articles it enacted.

## Column guide

| Column | Meaning |
|--------|---------|
| **Citation** | The neutral citation. Form: `[YEAR] LEXBY-<TIER> <N>`. Tier codes: SC (Supreme Council), CA (Court of Appeal), FI (First Instance). Link goes to the full ruling file. |
| **Tier** | The court that issued the ruling: `supreme`, `appeal`, or `first-instance`. |
| **Status** | Whether the ruling is currently good law. See Status Key. |
| **Ratio (one line)** | The binding holding, stripped to its minimum. Obiter is not included here. |
| **Scope** | Which repos, matters, or subject areas the ruling governs. `all repos` means realm-wide. |
| **Cites** | Statute (S-n) or prior case law the ruling expressly relies on or enacts. |

## How to cite a ruling

Neutral citation form (SPEC-LAW S-11(d)):

```
[YEAR] LEXBY-SC n      (Supreme Council)
[YEAR] LEXBY-CA n      (Court of Appeal)
[YEAR] LEXBY-FI n      (First Instance)
```

In prose, give the citation and then the point of ratio you are relying on. Example:

> The duty of care arises from the relationship itself, not from any enacted statute ([2026] LEXBY-SC 1, ratio I).

Only the ratio binds. Obiter dicta are persuasive only. A ruling made in ignorance of binding statute or prior precedent is per incuriam and void (S-11(e)).

---

## Rulings

| Citation | Tier | Status | Ratio (one line) | Scope | Cites |
|----------|------|--------|------------------|-------|-------|
| [[2026] LEXBY-SC 1](2026-LEXBY-SC-1.md) | supreme | good-law | Breach is the tort of negligence (duty + graded endeavours + remediation only); the system is a unitary realm of parliamentary sovereignty (one global SPEC-LAW, jurisdiction-local case law, one apex court). | All repos, all matters (constitutional founding ruling) | S-1, S-2, S-3 (founding); enacts S-4 through S-12 |
| [[2026] LEXBY-FI 1](2026-LEXBY-FI-1.md) | first-instance | good-law | A governance system is fit for alpha release where the core legal model is coherent, statute + procedure + founding caselaw are committed, constitutional enforcement is in place, and known gaps are disclosed; convenience-layer tooling is not a prerequisite for alpha; deterministic citation numbering is a necessary condition for v1. | vibe-justice-system repo (alpha-readiness standard for governance systems) | S-4, S-5, S-7, S-8, S-11(d), S-14 |

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

When a new ruling is committed to `caselaw/`:

1. Add a row to the Rulings table above, in ascending citation order within each year.
2. Fill every column. Do not leave `ratio` vague - one tight line is better than a paragraph.
3. Set `status` to `good-law` unless the ruling itself overrules or supersedes an earlier one.
4. If the new ruling overrules or supersedes an existing entry, update that row's status to `overruled` or `superseded-by-statute` and add a bracketed note: `(overruled by [YEAR] LEXBY-SC n)`.
5. If new SPEC-LAW articles are enacted by the ruling, record them in the `cites` column and update `SPEC-LAW.md`.
6. Commit the ruling file and this index in the same commit. The citator must never lag the caselaw.
