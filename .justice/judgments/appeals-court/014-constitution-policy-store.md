---
citation: "[2026] VJS 14"
court: appeals-court
questionKey: "constitution:policy-store"
caseId: constitution
date: 2026-09-17
status: standing
---
## Question
What must be true of where an organisation keeps its policies?

## Facts
{"two_orders":"agents follow both the pinned lawpack (higher order, travels with the court) and the organisation law (operational, edited by humans)","access":"everyone must be able to reach the court to ask questions, hear answers, argue cases, and find policy and law","editing":"policies are edited and versioned by humans, at any time","already_required":"[2026] VJS 11 requires each policy to be a versioned document, requires a ruling applying one to cite the version in force at the time of the facts, and makes a version unenforceable until the work it demands has been carried out; [2026] VJS 12 requires the routing step to record which branch it took","validation":"the court validates a change with a gate that evaluates STAGED GIT CHANGES against the book and names the law behind any denial; it cannot see rows in a database","candidates":"a repository holding the code that enforces the policies, or the jurisdiction repository holding the law that defers to them","binding":"some policies are bound by NAME to systems outside the record, such as a CRM pipeline stage, so a version in force ahead of its provisioning stops work silently"}

## Ruling
Appeal allowed in part. Condition (a) is varied; (b) and (c) stand. An organisation's policies must be kept in a store with two properties: a version must be recoverable, unaltered, at the date of the facts a ruling reasons from; and adopting a version must present itself to the validation gate as a staged change the gate can evaluate and deny by name. What must be staged is the adopted version's identity — its name, its digest, and the date it came into force. The prose may live wherever the people who write it reach it, provided that digest still resolves to a fetchable document that matches it; a store that overwrites a version in place fails the first property and may not be used. (b) stands, read to its purpose: the adopted version's identity and the enforcing code that depends on it are staged in the same change set. (c) stands: store and repository must be reachable by everyone who must find policy and law. Policies may not be kept in the jurisdiction repository, and a store no staged change names — rows in a database among them — remains excluded. Files under the version control the enforcing repository uses satisfy both properties and remain available; they are no longer the only thing that does. The actor is the humans who already edit and version the policies, and, for staging the adopted version's identity, whoever already ships the enforcing code; nothing here permits any other actor to move them.

## Reasoning
Two of three allow in part on the same ground, and that is the panel's disposition. The textualist is right that the first-instance bench could not have found a third route on the record before it — but an appeal is where the record is corrected, and 'an organisation should not have to rely on git to function' now sits before us as a fact, not a preference. The majority's ground is r.5's weakest reading: (a)'s purpose is recoverability at the date of the facts and a change the gate can deny by name. Git is the present instance of a store with those properties, not the content of the rule. Fixing the instance in the rule makes this court's machinery a precondition of the organisation's operating rules — r.15's failure mode arriving from outside, and the deferral VJS 11 promised withdrawn by the back door.

The challenge's route is not free, so the pragmatist's bill is adopted as a condition rather than treated as an objection: a digest verifies a document you can already fetch, it does not retrieve one, so a store that overwrites versions defeats VJS 11's version-in-force citation six weeks later and is excluded here. That also answers the textualist's surviving point — a bare digest does not stage the enforcing code beside the version, but the adopted version's identity does, which is why (b) is preserved rather than dissolved. Staging the cheap thing keeps co-location, makes divergence show as a mismatch, and stops the alternative failure the pragmatist names: authors who do not use git editing elsewhere while the repository files become a fiction the gate validates cleanly. (c) was never in issue. The actor is stated, per r.7, and is not widened.

## Law applied
- appeal of [2026] VJS 13
