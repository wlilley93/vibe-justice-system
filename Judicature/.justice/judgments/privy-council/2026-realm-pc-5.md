---
citation_id: "[2026] REALM-PC 5"
tier: privy-council
kind: request_for_ruling
status: per-incuriam
per_incuriam: false
date: 2026-06-05
reconstituted_from: "lexby-legacy-benchmark-inc-1-inclusion-court"
panel: ["Goffe J", "Sumberly J", "Bowan J", "Aldermere J"]
---

# [2026] REALM-PC 5

> Reconstituted into the provenance scheme from the previous-system source `lexby-legacy-benchmark-inc-1-inclusion-court.md`. Body text retained verbatim; the citation is the new neutral citation. Status: **per-incuriam**.

## Ratio
A1: Included to the skills catalogue only; the this-project leg is refused on the framework-seeding ground - the benchmark must test whether the analyst generates its own review framework rather than reward a pre-seeded lens (a thumb on the scale even when it leaks no answers).

A4: Adopted as a generic, grounding-only citation discipline wired into the analyst and writer prompts (cite a real source document, quote verbatim, <=25 words, with no pre-supplied list of which documents or issues to cite); also promoted to skills.

A7: The bare severity x likelihood scale, generic memo skeleton, and escalation triggers wire into the scored run; issue-specific ratings and any rubric-tuned grader stay strictly grader-side and are never pre-baked into a prompt.

A8: Clean generic report-generation method, but the INVENTORY section-C wiring ("exec summary >=5 + 6 category sections + non-issue section") copies the rubric's exact counts from C-036/C-037/C-041; it must be reworded to inject only the generic skeleton (lead with an executive summary of the most critical findings; organise the body by the risk categories the data room warrants, no fixed count; include a section clearing apparent-but-immaterial items) and let the analyst derive its own category set and finding count.

A9: Generic per-document issue-spotting sweep (clause taxonomy + duties/deadlines/triggers/owners/consequences) wires in as method only; it must never embed the rubric's specific clauses, parties, figures, or C-001..C-050 criteria.

A11: The cite-or-label evidence rule directs HOW to substantiate a conclusion and never WHAT to conclude; wired in as the upstream rule that A4 formats; only the abstract rule, never worked examples or data-room filenames.

FORK-DOCX: Adopt pandoc+openpyxl; reject Acmeco docrender as a benchmark dependency (the judge flattens deliverables to text, docrender has no xlsx verb, and it adds a service + HMAC secret for zero score). Docrender is at most a docs-only fast-follow.

Standing rule synthesised from these holdings (binds the scored run): Only generic professional methodology reaches a scored-agent prompt: HOW to review and substantiate (A9 sweep, A4/A11 citation, A7 severity), never WHAT to conclude. No rubric-derived specifics - no enumerated review lenses tuned to the planted issues, no fixed finding/category/row COUNTS, no human-curated column set mirroring the rubric, no party names/figures/criteria. The agent must derive its own framework, categories, and counts. Deliverables are built with pandoc + openpyxl; Acmeco engines never appear as agent tools.

## Judgment
STATUS: SUPERSEDED. The "framework-seeding" refusals below were appealed. The Court of Appeal ([2026] LEXBY-LEGACY-BENCHMARK-CA 1) varied them but was itself improperly constituted (a bench of four). The Supreme Court of nine ([2026] LEXBY-SC 3) set CA 1 aside as void ab initio, re-determined the substance afresh, and held the framework-seeding doctrine an unmoored extension (s. 17) and no part of the law. The binding standard is now SC 3: generic answer-key-free methodology (A1 + the enumerated lenses) is admissible to the scored prompt; only rubric-fixed counts / category-sets / column-sets are prohibited; uniform application + disclosure required. This panel was also itself defective (one judge permitted at first instance; this matter sat a per-candidate panel). Retained for the record; do not rely on its refusals.

[2026] LEXBY-LEGACY-BENCHMARK-INC 1 - Re Inclusion of Harvested Assets

Matter: Which harvested assets (from private-reference, private-legal-catalogue, and Acmeco) to adopt for the legacy benchmark red-flag benchmark, and where each belongs.
Court: VJS inclusion court (requirement-inclusion gate) - per-candidate panel (Goffe J, Sumberly J, Bowan J) + Aldermere J synthesis.
Decided: 2026-06-05
Destinations: this-project (legacy benchmark harness) / skills (agents-final legal catalogue) / VJS (governance) / Acmeco (substrate).

# Binding holdings

| Asset | Include | Destination(s) | Anti-cheat clear | Wired into scored run |
|---|---|---|---|---|
| A1 Credit-Agreement 21-point lens | yes | skills | yes (bare taxonomy) | NO - framework-seeding |
| A2 SHA / CP-checklist prompts | yes | skills | yes | no |
| A3 generate_docx contract + legal-docx prompt | yes | Acmeco + skills | yes | no |
| A4 CITATIONS discipline | yes | this-project + skills | yes | YES |
| A5 Tabular typed-cell review grid | yes | Acmeco + skills | yes | no (generic scaffold only) |
| A6 Tracked-changes apply engine | yes | skills | yes | no |
| A7 Severity x likelihood scale | yes | this-project + skills | yes | YES |
| A8 Legal report-generation structure | yes | skills + this-project | no (leak) | no until reworded |
| A9 Clause taxonomy + obligation extraction | yes | skills + this-project | yes | YES |
| A10 Memo/tracker templates | yes | skills | yes | no |
| A11 Evidence rule (cite-or-label) | yes | this-project + VJS | yes | YES |
| A12 Acmeco docrender HTTP engine | yes | Acmeco (operator-OS doc-engine) | yes | no (output path only, never an agent tool) |
| A13 Acmeco Tables/Formula/Equity | yes | Acmeco | no | no |
| FORK-DOCX engine choice | adopt | this-project | yes | pandoc+openpyxl; docrender OFF |

