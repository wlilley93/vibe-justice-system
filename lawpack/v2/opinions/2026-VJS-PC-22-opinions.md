---
citation: "[2026] VJS-PC 22"
court: privy_council
bench: [Tindale, Rowanne, Marchmont]
bench_size: 3
convening: CONVENING-privy_council-2026-08-07-023002
submission: SUBMISSION-2026-08-07-022943
case_file_digest: "sha256:fbce167e6210e01cd62fb684b881c658f34de61537c24db447c7524f3e572224"
issue: canon_licence_term
date: "2026-08-07"
disposition: "VARY, 3-0 on Q1 and Q4; 2-1 on Q3"
---

# [2026] VJS-PC 22

**Privy Council. On the licence term of [2026] VJS-PC 11 D2.**

Disposition: **D2 is VARIED**, unanimously. Q2 and Q4 unanimous. Q3 by a majority,
Marchmont dissenting in part.

---

## Tindale J.

### Q1: vary or hold

I begin with what a court can actually order, because the answer decides this case and
most of the argument on both sides is beside it.

D2 says the canon "must REMAIN a separate, PUBLIC, AGPL-3.0 repository". Read against a
copyright holder, the third of those is not a term this Court has power to impose. A
licence is a grant. It comes into existence by the holder's act and by nothing else, and
ACT-001:s3 puts real-world law above every instrument in this corpus precisely so that we
do not pretend otherwise. We may order `lexby` to do a great many things. We may not order
a person to give away rights in their own work, and an order that purports to is not
stern, it is void for want of power.

So the honest reading of D2 as it stands is not "the canon shall be AGPL". It is "IF the
canon is published, it shall be published under AGPL" - a condition on lexby's act of
publishing, not a command to the holder. That reading is available and it is the only one
within power.

But it has a consequence the case file does not draw out and which I think is decisive.
On that reading, D2 does not put the canon in breach at all on the licence limb. It puts
publication out of reach. And since D2's OTHER limb requires the canon to be public, the
order as it stands requires lexby to do something and simultaneously forbids the only
lawful means of doing it. That is not a stern order. It is an incoherent one, and it has
been quietly incoherent since 2026-07-11.

The objection from Reading B is that a court which varies a term the moment the executive
finds it inconvenient binds nothing. I take it seriously and it does not apply here. What
has happened is not that lexby found AGPL inconvenient. The holder has stated, having been
shown the order and the alternative in terms, what they in fact grant. That is new
evidence about a fact the order was built on, not a change of mind about the order. A
court that cannot receive evidence that its factual premise has changed is not principled,
it is deaf.

**Vary.**

### Q2: what survives of PC 11

Nearly all of it, and I want to be exact about why, because the case file's framing
("licence was reasoning toward the firewall, not the holding") is too generous to the
outcome I am reaching.

PC 11's ratio is dependency direction and pinnability: the canon is the acyclic root that
independent subscribers pin by digest, so it must not be folded into a product monorepo,
and every subscriber must keep pinning the one piece it vendors. Nothing in that turns on
AGPL. It would be equally true of a canon under MIT, under PolyForm, or under no licence
at all, and I note the opinions verified it by measurement - the lattice, the locks, the
absence of `kern` in the canon's Cargo.lock - none of which mentions a licence.

The AGPL stamp appears in PC 11 as a MECHANISM: it was what made the firewall hold "by
construction" rather than by anyone's care. That is a real function and it does not
survive, which is Q3's problem. But a mechanism that fails is not a ratio that falls.

The separateness, publicness and zero-inbound-dependency limbs of D2 stand untouched.

### Q3: the firewall

The case file says PolyForm Noncommercial "is in neither category" of
BUNDLE_COPYLEFT/BUNDLE_PERMISSIVE. That is true and it understates the problem.

REG-BUNDLE-001(4) exists to stop a RECIPROCITY obligation leaking into a distribution
boundary that promises none. A noncommercial term is not a reciprocity obligation. It is a
USE RESTRICTION, and use restrictions travel differently and more dangerously: a permissive
bundle that co-resides with an AGPL component leaks a source-disclosure duty, which is
embarrassing; a permissive bundle that co-resides with a noncommercial component leaks a
prohibition on the commercial use its own licence expressly grants, which is a
contradiction the downstream consumer cannot cure by disclosing anything.

The firewall's two-way sort therefore does not merely fail to classify the canon. It is
now sorting on the wrong axis. I would hold that REG-BUNDLE-001(4) requires a third
category and a stricter rule for it, and I would direct that `bundle verify` FAIL CLOSED on
any licence it cannot classify rather than pass it through an allow-list it does not
appear on.

### Q4: publication

Severable, and publication may proceed on the varied term.

The contrary argument - that PC 11's publicity limb was justified by subscribers pinning a
shared higher law, and a noncommercial licence narrows who may lawfully subscribe - has
force and I have thought about it. It fails on the facts of what publicity is FOR here.
Subscribers pin the canon by digest to establish that the law they are governed by is the
law of record. That function is about verifiability, not commerce. A commercial subscriber
who may not lawfully use this canon is not helped by the canon being secret; they are
helped by being able to READ it and know that they may not.

Publication also does something the licence cannot undo: it makes the record checkable by
people with no relationship to the holder at all. I would not narrow that.

---

## Rowanne J.

I agree with Tindale J. on Q1, Q2 and Q4 and would add one thing on Q1, and I differ from
her emphasis on Q3 without differing in result.

On Q1, the point that persuades me is narrower than incoherence and I think it is the one
that should be the ratio. **Every term of an order binds the person it is addressed to.**
D2's actor is `lexby`. A term of an order addressed to lexby which can only be performed
by a third party - here, the copyright holder - is not a term lexby can breach, and a
standing breach recorded against lexby for it is a fiction that the corpus has now been
carrying for four weeks.

That fiction had a cost, and it is worth naming because it is the kind of cost this Court
rarely sees. It made the canon's own breach ledger say something untrue about who was at
fault, and it did so in a record which is itself published. I would not let that pass with
a variation and no comment. The self-file at
`BREACH-2026-08-06-canon-licence-not-conforming` is accepted for the two failures of care
it actually names - working toward publication without reading the order that governs it,
and nearly conforming the reciting files - but the standing licence non-conformity was
never lexby's to breach and the ledger should say so.

On Q3 I reach Tindale J.'s result and I want to record that I do not think we should be
rewriting a regulation in a case about an order. We heard argument on a term of D2. Nobody
argued REG-BUNDLE-001(4). I join the direction only because the alternative is worse: the
firewall is currently a check that CANNOT FIRE on the canon's own licence, and leaving a
gate in that state while knowing it is in that state is not judicial restraint, it is
negligence with a good excuse. But I would confine the direction to fail-closed
classification and reserve the substantive third category to a case where somebody argues
it.

---

## Marchmont J., dissenting in part

I agree that D2 must be varied and I agree publication may proceed. I dissent on Q3 and I
would go further than my colleagues on what the variation must carry with it.

**On Q3.** My colleagues direct a fail-closed classifier and reserve the third category.
I would decide it now. The reason is that "reserve it to a case where somebody argues it"
assumes such a case will come, and the whole of this docket is evidence that it will not.
The canon's licence changed in July and no case came; it was found by someone reading a
grep result four weeks later. REG-BUNDLE-001(4) will now sit with a fail-closed classifier
that refuses PolyForm, which means the canon cannot be bundled at all, which means nobody
will bundle it, which means nobody will bring the case. We are not reserving a question.
We are closing a door and calling it restraint.

**On what the variation must carry.** My colleagues vary the term and leave it there. I
would attach a condition, and I put it as strongly as I can because it is the only part of
this judgment that protects anyone other than the holder.

PolyForm Noncommercial forbids commercial use. Every subscriber currently pinning this
canon did so when it was AGPL, which permitted it. Those subscribers have ordered their
affairs around a grant that has now narrowed under them, and they were not heard - PC 11
D4 expressly required that any future topology change come back "with the subscriber
heard", and the same principle must reach a licence change that is far more consequential
to them than a repository move.

I would therefore require, as a condition of publication and not merely as good practice,
that the canon publish a NOTICE stating: the licence in force, the date it changed, that
it changed without an adoption record for four weeks, and that anyone who vendored a
digest of this canon before 2026-07-11 took it under AGPL-3.0. Nothing in this judgment
purports to reach back and narrow a grant already made; a licence that has been given for
a specific digest has been given. Publishing under the new term while silent about the old
one would let the record imply otherwise, and the implication would be false.

Subject to that condition, I concur in the result.

---

## The Court

**Q1: VARY, 3-0.** D2's licence term is varied to the licence the holder has stated. The
ratio is Rowanne J.'s: a term of an order addressed to lexby which only a third party can
perform is not a term lexby can breach.

**Q2: PC 11's ratio SURVIVES, 3-0.** Dependency direction and pinnability turn on nothing
about a licence. Separateness, publicness and zero-inbound-dependency are untouched.

**Q3: 2-1.** `bundle verify` must FAIL CLOSED on any licence it cannot classify, rather
than pass it through an allow-list it does not appear on. The substantive third category
for use-restricted licences is reserved (Marchmont J. dissenting: it should be decided
now).

**Q4: PUBLICATION MAY PROCEED, 3-0**, on the varied term and a clean boundary.

**Marchmont J.'s condition is ADOPTED, 3-0**, the other members agreeing on reflection that
it costs nothing and states only what is true: the notice must go up before publication.
