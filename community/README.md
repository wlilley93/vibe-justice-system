# Community Record

The VJS Community Record. Every project running the Vibe Justice System contributes rulings here. This is the shared commons: the more precedent it holds, the more disputes across all projects resolve on the fast path with no sitting required.

---

## What is here

| Directory | Contents |
|-----------|----------|
| `caselaw/YYYY/` | Anonymised rulings submitted by community members, year-bucketed. One file per ruling. Persuasive precedent across all VJS jurisdictions. |
| `benches/` | Community bench rosters (JSON). Share a bench. Borrow one. |

---

## Submitting a ruling (VPR 8)

All three court tiers (First Instance, Court of Appeal, Supreme Council) automatically open a PR here after every ruling. The submission is anonymised: project-specific identifiers (repo names, file paths, variable names, function names) are replaced by generic placeholders. The legal question, ratio, tier, law applied, and outcome are preserved unchanged.

The clerk reviews every PR for:
1. Constitutional compliance (does it conflict with SPEC-LAW?)
2. Subject matter jurisdiction (s. 14: is this a decision arising in project work?)
3. Correct anonymisation

PRs that pass are auto-merged. PRs that fail receive a request-for-changes comment explaining why.

---

## Reading a ruling

Each file in `caselaw/YYYY/` is a self-contained ruling artefact. The **ratio** is the binding holding. The **obiter** is persuasive only. The **tier** tells you how much weight to give it.

Cross-reference with `SPEC-LAW.md` to find the governing statute articles cited. Cross-reference with your local `caselaw/INDEX.md` to see if any community ruling covers a question you are facing.

---

## Jurisdiction note (s. 14)

Community rulings cover decisions arising in AI-assisted software, engineering, or professional project work. Personal life questions and matters outside project work are outside VJS jurisdiction and will not be accepted.

---

*The first contributors shape the law everyone inherits.*
