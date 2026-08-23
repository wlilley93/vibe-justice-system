---
citation: "[2026] VJS 9"
court: first-instance
questionKey: "model:2026-08-23-court-client-proof:predicate:meetsAuditRequirement"
caseId: 2026-08-23-court-client-proof
date: 2026-08-23
status: standing
---
## Question
What does meetsAuditRequirement mean?

## Facts
{"args":[{"left":{"op":"field","path":"requirement.kind"},"op":"ne","right":{"op":"const","type":"String","value":"minAuditsCompleted"}},{"left":{"op":"field","path":"reviewer.auditsCompleted"},"op":"ge","right":{"op":"field","path":"requirement.threshold"}}],"op":"or"}|Single dispatching predicate over requirement kinds. For kind 'minAuditsCompleted' the reviewer's completed-audit count must be greater than or equal to the threshold, because 'at least 2' is inclusive of 2. Any other kind is vacuously satisfied, so adding a kind requires adding a branch.

## Ruling
meetsAuditRequirement is a single dispatching predicate over requirement kinds: it is satisfied if either (a) requirement.kind is not 'minAuditsCompleted', or (b) reviewer.auditsCompleted is greater than or equal to requirement.threshold. For kind 'minAuditsCompleted' the requirement is met on equality as well as excess — 'at least N' includes N. For every other kind, including kinds not yet defined, the predicate is vacuously satisfied; it imposes no condition and denies nothing. It therefore means 'this reviewer does not fail the stated audit requirement', not 'this reviewer satisfies some audit requirement'. Adding a new kind that is meant to constrain requires adding a branch; until that branch exists the new kind passes.

## Reasoning
The facts record an AST and its gloss, and nothing else. The genuine choice is between reading the predicate as an affirmative test of reviewer competence and reading it as a non-failure test over a disjunction whose left arm is a kind guard. I take the second, weaker reading, because it is what the disjunction states on its face: the left arm is true whenever the kind does not match, so the predicate returns true without ever consulting the reviewer. I state expressly that vacuous satisfaction is part of the meaning, since a later bench asked about an unbranched kind must be bound to 'passes', not left to infer 'denies'. I decline the wider question of whether open-by-default dispatch is the right design, or whether unknown kinds ought to fail closed; the record puts no such question, and answering it would be making law.

## Law applied
- Law of this court, II.5 — decide the question asked; prefer the weakest reading the facts support; name the choice taken
- Law of this court, II.6 — a condition that cannot be checked is not a condition (the left arm imposes none)
- Law of this court, IV.14 — 150-word cap on bench opinions
