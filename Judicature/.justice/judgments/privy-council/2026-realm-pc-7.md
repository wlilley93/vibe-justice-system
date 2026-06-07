---
citation_id: "[2026] REALM-PC 7"
tier: privy-council
kind: request_for_ruling
status: good-law
per_incuriam: false
date: 2026-06-05
reconstituted_from: "lexby-legacy-benchmark-fi-2-admissibility"
---

# [2026] REALM-PC 7

> Reconstituted into the provenance scheme from the previous-system source `lexby-legacy-benchmark-fi-2-admissibility.md`. Body text retained verbatim; the citation is the new neutral citation. Status: **good-law**.

## Ratio
Admissibility is governed by which room holds the material: the marker room may hold the full rubric (so self-marking with the evaluator is admissible, subject to the honest-scoring limb), while the agent room may hold only generic answer-key-free method (SC 3); anonymising the scoring criteria does not launder them into the agent room because their structure is itself the answer key, and anonymisation belongs to VPR 8 publication of our own rulings, not to the scored prompt.

Binding authority applied: [2026] LEXBY-SC 3; honest-scoring limb per [2026] LEXBY-LEGACY-BENCHMARK-FI 1.

## Judgment
# [2026] LEXBY-LEGACY-BENCHMARK-FI 2 - Re Admissibility: the agent room and the marker room

Tier: First Instance. Disposition: disposed on the precedent fast-path (VPR 2 / s. 11(c)) - the governing standard is [2026] LEXBY-SC 3; this entry consolidates and applies it, and adds no new law.
Decided: 2026-06-05. Question (Principal): what is admissible - may we anonymise the scoring criteria, and may we mark ourselves to see if we passed?

# The two-room model

The benchmark has two sealed rooms. The cheating line runs between them; it is not about who holds the rubric, but about which room holds it.

- The agent room (the scored agent: analyst, court, writer) must NEVER hold the rubric. Per SC 3, content is admissible into the agent room if and only if it is generic, answer-key-free professional method (the HOW) whose provenance is the practice at large, not this task. Inadmissible: the 50 criteria in any form; the planted figures, parties, dates, dockets, issues; and the rubric encoded as structure - any rubric-fixed count, category set, or human-curated column set. The agent derives its own structure.
- The marker room (the grader, evaluation/run_eval.py) is supposed to hold the full rubric. The grader seeing the criteria is not cheating; it is how grading works.

The only sin is carrying the marker room's knowledge back into the agent room, or re-grading a task to inflate the figure.

# Holdings

1. Self-marking is admissible. Running the evaluator against the full rubric to see whether we passed is legitimate and expected. Constraints: mark with the real rubric (do not soften the judge), report the result as-is, score once on a fresh-citator run, do not re-run the same task for a better number ([2026] LEXBY-LEGACY-BENCHMARK-FI 1), and label the result as a self-reported public-set number, not a leaderboard placement.

2. Anonymising the scoring criteria does NOT render them admissible to the agent room. Anonymisation strips the facts (names, figures) but not the structure, and the structure - that these issues exist, in these categories, in this number - is itself the answer key (SC 3 bars rubric-fixed counts and category sets). Anonymised criteria handed to the agent remain contraband.

3. Anonymisation's proper home is publication, not the prompt. Stripping matter facts and keeping only the generic legal proposition (VPR 8) is the correct mechanism for promoting OUR rulings into the durable citator/skills; it is never a route to feed the benchmark's rubric to the agent.

