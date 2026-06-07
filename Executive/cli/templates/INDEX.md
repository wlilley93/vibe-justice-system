# Caselaw Index (Citator)

The local citator for this jurisdiction. It starts empty, while the repo subscribes to the canonical VJS
law vendored in `CASE-LAW.md` and `VPR.md`. It becomes the authoritative index of rulings handed down in
this repo. Updated whenever a new ruling is committed to `.justice/judgments/`. Lexby checks this FIRST: if a
binding ratio is on all fours, the matter is disposed of on citation (VPR 2 fast path), no sitting.

## How to cite

Use `cdd next-citation <tier>` to mint the next citation from this index. Current VJS provenance codes
are `REALM-PC`, `REALM-CA`, `REALM-SC`, and local `CC-<repo>` / division codes where a repo chooses to
create them. Only the ratio binds; obiter persuades; a ruling made in ignorance of binding statute or
precedent is per incuriam and void.

## Column guide

| Column | Meaning |
|--------|---------|
| **Citation** | Neutral citation, `[YEAR] <CODE> N`. Links to the ruling file in `.justice/judgments/`. |
| **Tier** | `privy-council` \| `court-of-appeal` \| `supreme-court` \| local court code. |
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
