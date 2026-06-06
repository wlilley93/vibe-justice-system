# Caselaw Index (Citator)

The master citator for this jurisdiction. The authoritative index of every ruling handed down in this
repo. Updated whenever a new ruling is committed to `.justice/caselaw/`. Lexby checks this FIRST: if a
binding ratio is on all fours, the matter is disposed of on citation (VPR 2 fast path), no sitting.

## How to cite (CASE-LAW S-11(d))

Neutral citation form: `[YEAR] LEXBY-<TIER> N`. Tier codes: `SC` (Supreme Court), `CA` (Court of
Appeal), `FI` (First Instance). The next number is assigned deterministically from this index - run
`cdd next-citation <tier>` or let the court Workflow assign it. Only the ratio binds; obiter persuades;
a ruling made in ignorance of binding statute or precedent is per incuriam and void.

## Column guide

| Column | Meaning |
|--------|---------|
| **Citation** | Neutral citation, `[YEAR] LEXBY-<TIER> N`. Links to the ruling file in `.justice/caselaw/`. |
| **Tier** | `supreme` \| `appeal` \| `first-instance`. |
| **Status** | `good-law` \| `distinguished` \| `overruled` \| `superseded-by-statute` \| `per-incuriam`. |
| **Ratio (one line)** | The binding holding, stripped to its minimum. Obiter excluded. |
| **Scope** | Repos / matters / subject areas governed. `all repos` = realm-wide. |
| **Cites** | Statute (S-n) or prior case law relied on or enacted. |

---

## Rulings

| Citation | Tier | Status | Ratio (one line) | Scope | Cites |
|----------|------|--------|------------------|-------|-------|
| _(none yet - this is a fresh jurisdiction)_ | | | | | |

---

## Status Key

| Status | Meaning |
|--------|---------|
| **good-law** | Stands and binds. Cite and apply without qualification. |
| **distinguished** | Good law on its own facts; a later court found the present facts materially different. |
| **overruled** | Expressly set aside by a higher court (or the same court later). Cite the overruling case. |
| **superseded-by-statute** | The ratio was replaced by an enacted CASE-LAW article; the statute controls. |
| **per-incuriam** | Made in ignorance of binding statute/precedent. Void; a fresh sitting is required. |
