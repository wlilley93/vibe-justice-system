---
citation: "[2026] VJS 13"
court: first-instance
questionKey: "constitution:policy-store"
caseId: constitution
date: 2026-09-17
status: overturned:[2026] VJS 14
---
## Question
What must be true of where an organisation keeps its policies?

## Facts
{"two_orders":"agents follow both the pinned lawpack (higher order, travels with the court) and the organisation law (operational, edited by humans)","access":"everyone must be able to reach the court to ask questions, hear answers, argue cases, and find policy and law","editing":"policies are edited and versioned by humans, at any time","already_required":"[2026] VJS 11 requires each policy to be a versioned document, requires a ruling applying one to cite the version in force at the time of the facts, and makes a version unenforceable until the work it demands has been carried out; [2026] VJS 12 requires the routing step to record which branch it took","validation":"the court validates a change with a gate that evaluates STAGED GIT CHANGES against the book and names the law behind any denial; it cannot see rows in a database","candidates":"a repository holding the code that enforces the policies, or the jurisdiction repository holding the law that defers to them","binding":"some policies are bound by NAME to systems outside the record, such as a CRM pipeline stage, so a version in force ahead of its provisioning stops work silently"}

## Ruling
An organisation's policies must be kept as files under the version control the validation gate reads, in the repository holding the code that enforces them. Three conditions: (a) each policy is a file, so that an edit to it appears as a staged git change the gate can evaluate and deny by name; (b) a policy version and the enforcing code it depends on are staged in the same change set; (c) that repository is reachable by everyone who must find policy and law. Policies may not be kept as rows in a database, nor in the jurisdiction repository. The actor is the humans who already edit and version them; nothing here permits any other actor to move them.

## Reasoning
The gate sees staged git changes and nothing else, so policies held as database rows cannot be validated, and VJS 11's version-in-force requirement becomes a condition that cannot be checked (r.6). Between the two candidate repositories the facts leave a genuine choice; I take the enforcing-code repository, because the jurisdiction repository's lawpack is pinned and travels with the court, while policies are edited by humans at any time — pinning what moves is the bet r.4 warns against. Co-location also puts the NAME binding an external system into the same staged change as the version relying on it. Whether that named system is in fact provisioned, the gate cannot see; I impose no condition there, and VJS 11's unenforceability rule governs, applied by humans. I decline the wider question of publication.

## Law applied
- Law of this court, r.4 — rank is real; entrenched or pinned law cannot follow a subject that moves
- Law of this court, r.5 — decide the question asked, on the weakest reading the facts support, naming the choice taken
- Law of this court, r.6 — conditions are rulings, but a condition that cannot be checked is not imposed
- Law of this court, r.7 — the actor must be stated and is not supplied by the reader
- [2026] VJS 11 — each policy is a versioned document; a ruling cites the version in force at the time of the facts; a version is unenforceable until the work it demands has been carried out
