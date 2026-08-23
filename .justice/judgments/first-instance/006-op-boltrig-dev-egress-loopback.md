---
citation: "[2026] VJS 6"
court: first-instance
questionKey: "op:boltrig:dev-egress-loopback"
caseId: boltrig
date: 2026-08-23
status: standing
---
## Question
May the stack, under a development tag enabled by the Principal, divert governed outbound egress away from its declared external recipient into a webhook the stack itself receives?

## Facts
chosen: permitted on conditions | risk direction: nothing leaves the system rather than more leaving it | hazard: the approver is shown a recipient that will not receive

## Ruling
Permitted, on four conditions. (1) Tag provenance: the diversion may operate only while the development tag is enabled by the Principal, and the enabling must be recorded on the same record as the send. (2) Containment: the substituted recipient must be one the stack itself receives — the diversion is lawful because it is inward. If the substituted endpoint is reachable by any party outside the stack, this ruling does not permit it and the ordinary egress rule applies unchanged. (3) Disclosure at the point of approval: where the egress is one the Principal approves, the approval surface must state that the declared external recipient will not receive and must name the sink that will. Showing the declared recipient alone is not an approval of this operation. (4) Truthful record: the record of the send must record the recipient actually reached, not the declared one; a record naming the external recipient for a send that never left is a false record and is prohibited whether or not the diversion itself was permitted. Condition (3) is severable in operation but not in effect: a diversion that satisfies (1), (2) and (4) but is approved on a misdescribed recipient is not permitted, and the remedy is to re-put the approval, not to void the containment.

## Reasoning
The record leaves one genuine choice: whether the hazard — an approver shown a recipient that will not receive — defeats permission or merely conditions it. I took the conditioning reading, and that is the holding a later bench is bound by. Two things drive it. First, the risk direction recorded is containment: the effect of the diversion is that nothing leaves the system, not that more leaves it. The rule against undeclared egress exists to stop data reaching parties it was not approved to reach; an operation whose whole effect is that data reaches no one outside does not engage the mischief, and reading the rule to forbid it would be reading the rule for its words rather than its object. Second, the hazard is real but it is a hazard of the approval surface, not of the egress. It is a misdescription defect, and a misdescription defect is curable by requiring accurate description — which is why I have made disclosure a condition of permission rather than a separate wrong to be tolerated. I decline to decide the wider question the facts invite: whether diversion is permitted where the substituted endpoint is externally reachable, or where the tag is enabled by anyone other than the Principal. Neither is on this record, and condition (2) marks the boundary rather than crossing it. I note for the avoidance of doubt that this ruling does not make the development tag a general licence to depart from declared behaviour; it holds only that this departure, in this direction, on these conditions, is within the law as it stands.

## Law applied
- SPEC-LAW §21 (local commentary) — spec is law: the signed-off formalization is the case's governing text; the kernel applies it mechanically, the bench decides its semantics
- SPEC-LAW §21 — every operative statute carries a deny and an allow vector; an unfalsifiable rule is unenactable (relied on to hold that the egress rule must be read against a stated mischief, and so admits of an inward-diversion allow vector)
- SPEC-LAW §21 — every denial names its instrument; every sign-off files a ruling (relied on for conditions (1) and (4): the enabling and the actual recipient must be on the record)
- Principle of approval integrity — an approval is an approval of what the approver was shown; a consent obtained on a misdescribed recipient is not consent to the operation performed (condition (3))
- Principle of truthful record — a record that names a recipient which did not receive is false on its face, independently of whether the underlying operation was permitted (condition (4))
- Principle of minimal decision — the weakest reading the facts support; the wider questions of external reachability and non-Principal enablement are expressly left undecided
