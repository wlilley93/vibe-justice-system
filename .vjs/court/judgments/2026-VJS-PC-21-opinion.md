# [2026] VJS-PC 21

## Privy Council (constitutional first instance), on referral from the County Court

**Referred by:** [2026] VJS-CC-VJS 11 (Hairline CCJ), Q1 to Q3
**Submission:** SUBMISSION-2026-07-27-201553
**Case file digest:** sha256:6efd2af0db904a2d9a75a717ea4816cae6f31f12bee6d0c2e9ae6bab1fbaaad2
**Convening:** CONVENING-privy_council-2026-07-27-205444
**Bench:** Atkin, Bingham, Denning (odd bench of 3)
**Vote:** 3-0 on the disposition; Denning concurring in part and dissenting in part

**Recusals recorded.** The Board was first constituted as Marchmont, Rowanne and Tindale, and that
convening is abandoned and superseded by this one. Tindale sits on [2026] VJS-PC 8, one of the two
orders whose bench is in issue, and Tindale's own participation is the very fact to be determined.
Marchmont and Rowanne sit on the same order. Reid, Wilberforce and Hoffmann are likewise
disqualified as the bench of [2026] VJS-PC 19. No judge may sit on the question whether their own
silence voided their own order.

---

## Opinion of Atkin

### The question, and the distinction on which it turns

Thirteen binding records declare a bench and carry no `source_opinion`. The kernel places
`BENCH_OPINION_MISSING` on the constitutive list, and [2026] VJS-PC 16 holds that bench-integrity
goes to whether a record IS an order, is void ab initio, and is never softened by an assent claim.
Taken at its widest, that disposes of the matter in a line and voids thirteen orders.

I would not take it at its widest, and the reason is a distinction the reference does not draw and
the kernel does not encode. The bench code covers four defects that are not of one kind:

| code | what it establishes |
|---|---|
| `TIER_NOT_CONSTITUTED` | the court could not issue an order at all |
| `BENCH_SIZE_MISMATCH` | the body that sat was not the constituted court |
| `BENCH_OPINION_MISSING` | there is no document in which the seats' participation is recorded |
| `BENCH_SILENT_SEAT` | a document exists and a named seat does not appear in it |

The first two go to **the bench**. If the tier was not constituted, or the wrong number sat, then
whatever happened was not a court and there is nothing for an assent claim to protect. PC-16 is
plainly right about those, and I would not disturb it by a word.

The third goes to **the record of the bench**. The court was the right court. The bench was the
constituted odd size. The named seats are the seats. What is absent is the document evidencing what
each of them said. That is an evidential gap, not a constitutional one.

To collapse those two into one category is to hold that a court which sat properly and decided
properly ceases retrospectively to have been a court because a file is missing. I can find nothing
in ACT-002, in the courts constitution, or in PC-16 itself that requires that, and a great deal that
tells against it.

### PC-16 decided a different mischief

PC-16's own words, as the kernel records them, name what it closed: an allow-list membership that
"downgraded these on mere allow-list membership, the very laundering [2026] VJS-PC 16 closed". The
mischief was **laundering**. A party with a defect it knew about typed an assent claim and thereby
softened a hard gate. The rule "never softened by an assent claim" is aimed at that act.

Not one of these thirteen records did that. None asserts a bench it did not have. None claims an
opinion that does not exist. They declare `standing_bounded_assent`, which is true of them, and they
are silent about the opinion because nothing ever asked them for one. There is no laundering here
because there was nothing to launder: the defect was invisible to every instrument that ran.

A ratio is bounded by its mischief. Applying PC-16's words past the act it was directed at, to void
thirteen orders that made no false claim, would be to use the authority past its ratio. I decline.

### Whether the gate's blindness matters

The reference asks (Q2) whether it matters that no gate could detect this. I hold: **not to what the
record IS, but decisively to what the remedy IS.**

Reading A is right that an instrument's coverage cannot be a source of validity. A record is what it
is whether or not anyone looked. I adopt that without reservation, and it disposes of any argument
that these orders are cured merely by having gone unnoticed.

But it does not follow that the consequence of a defect is fixed without regard to who was at fault
for its persistence. Here the fault is entirely the system's. `validate --staged` was the only
instrument that ever ran this check, it evaluates staged files alone, and the pre-commit hook that
would have invoked it is gitignored, regenerated per clone, and was absent from the clone that made
these records, while the committed install manifest listed it by name with a pinned digest. The
estate asserted an enforcement surface it did not have.

Where a duty-holder's own instrument failed to ask for a thing, and the record-maker made no false
claim, the answer is to ask for it now. The remedy is correction, not destruction.

### What I would NOT extend this to

Nothing in this opinion touches `TIER_NOT_CONSTITUTED` or `BENCH_SIZE_MISMATCH`, which remain
constitutive and void on PC-16's unaltered terms. Nor does it shelter a record that DECLARES an
opinion which does not exist, or names a bench that did not sit. Those are false claims and they are
the laundering PC-16 forbids.

### Answers

**Q1.** Route for correction. `BENCH_OPINION_MISSING` is an evidential defect in the record of a
properly constituted bench, not a constitutive defect in the bench.
**Q2.** The blindness does not change what the record is; it is decisive as to remedy.
**Q3.** Interim status: **binding, and flagged**. They remain in force and may be relied on, carrying
a visible correction obligation until the opinion is supplied. An order that is binding-but-flagged
is not a novelty; it is the ordinary shape of route-for-correction.
**Q4.** Already answered by the County Court, correctly, and I would not disturb it.

---

## Opinion of Bingham

I agree with Atkin and with the disposition. I add two points, one of which I regard as the
constitutional answer to the objection Denning presses.

### Why this Board may do this without troubling the Sovereign

The obvious objection is that PC-16 drew a line around the entrenched floor of
ACT-ASSENTED-RECORD-PROTECTION, and that this Board is now moving it. If we were narrowing the
floor, that objection would be fatal: an entrenched floor may be narrowed only by a
Sovereign-assented constitutional Act citing it by number, and no Board may do by interpretation
what is reserved to the Sovereign.

But observe the DIRECTION. PC-16 carved an exception OUT of the floor: bench-integrity defects are
not protected by it. What we do today is read that carve-out more narrowly, so that fewer records
fall outside the floor and MORE records enjoy its protection.

**Narrowing an entrenched protection is reserved. Enlarging it is not.** The floor is a minimum, not
a maximum. A rule that says "an assented record is never voided, only corrected" is not offended by
a holding that extends its reach; it is offended only by one that contracts it. So the reservation
does not bite, and this Board is competent.

I would put it as a general rule of construction for this realm: **when a court is uncertain how far
an exception to an entrenched protection extends, it construes the exception narrowly**, because the
error that can be corrected later by a court is the error that leaves the protection too wide, while
the error that voids records is not correctable at all once relied upon.

### The gate that measures the wrong thing

I turn to the two canon orders, [2026] VJS-PC 8 and [2026] VJS-PC 19, raised by the repaired gate
under `BENCH_SILENT_SEAT`.

I have read what that check actually computes. It measures the largest contiguous block of text a
seat's name "owns" before the next seat is named. On PC-8 it finds Tindale owning 29 characters
against ~930 each for Marchmont and Rowanne; on PC-19, Wilberforce owning 51 characters of 16,080.

The code calls this a silent seat, and the message says the seat has "no present, non-empty opinion".
That is not what was measured. What was measured is **brevity**.

A judge who writes "I agree with Atkin and have nothing to add" has not been silent. Concurrence
without separate reasons is not merely lawful, it is the ordinary and often the most disciplined
form of judicial agreement. A rule that a short concurrence voids the order would require every
judge to pad, which is the opposite of what a court should want.

So the check cannot presently distinguish a judge who did not participate from a judge who concurred
briefly. It reports the first while measuring the second. That is precisely the vice the County
Court named in CC-VJS 11: a check that reports a broader result than it computed. It cannot ground a
void, because it does not establish the fact that voiding would depend on.

The cure is not to relax it but to make it measure what it claims. A seat should be counted as
having spoken if it either writes reasons OR **expressly concurs**. A seat that does neither is then
genuinely silent, and I would hold such a record constitutively defective, because a name attached
to a decision the person never joined is a false claim about the bench, and that IS PC-16's mischief.

### On the remedy

Correction must not be indefinite. A flagged order that stays flagged forever is a void order with
better manners. I would attach the obligation to a visible, ratcheting register that may only
shrink, on the same discipline this estate already applies to structural debt, so that the number of
uncorrected records is always known and can never quietly grow.

---

## Opinion of Denning, concurring in part and dissenting in part

I concur in the disposition on Q1 to Q3. I differ on two matters, one of emphasis and one of
substance, and I record them because a bench that agrees about everything has not tested anything.

### Where I go further than my colleagues

Atkin says the blindness of the gate is decisive as to remedy. I would say something sharper.

We are asked to decide the fate of thirteen orders because a hook was in a `.gitignore`. Let us be
plain about what happened. This estate committed a manifest that listed two enforcement hooks by
name, pinned their digests, and shipped it as evidence that the surface existed. The files were
absent. Every fresh clone of this repository begins with its git-level gates switched off and
nothing anywhere says so, while a committed file asserts the contrary.

That is not an oversight to be noted in passing on the way to a remedy. It is the same defect as the
one we are judging, one level up: **a record asserting a state of affairs that nobody verified.** The
thirteen orders declared a bench nobody verified; the install manifest declared hooks nobody
verified. We should not void the first while treating the second as background.

I therefore agree that the orders survive, but I would have the Board say expressly that the
system, and not the record, is the defaulter, and that the correction obligation falls on the
duty-holder who let the instrument lie, not on the courts that sat.

### Where I dissent

Bingham would have the Board hold that a seat which neither writes nor expressly concurs renders the
record constitutively defective. I would not decide that today, and I dissent from it.

We have no such case before us. We have two records that the gate flagged on a measure of BREVITY,
and Bingham rightly demonstrates that the measure does not establish participation. Having held that
the evidence cannot support the finding, the Board should stop there. To go on and announce what
would happen in the case we have found ourselves unable to reach is obiter dressed as holding, and
this realm has been burned before by a ruling recorded in prose that later read as binding.

There is a further reason for caution. Bingham's proposed rule would make the validity of an order
depend on a judge performing a formula. I am uneasy about any rule under which a court's decision
stands or falls on whether a judge remembered to write a sentence of concurrence. If that rule is to
be made, it should be made prospectively, on a case where the question actually arises, and with the
seats concerned heard. It should not be made by a Board of three deciding the status of two orders
on which it has just held the evidence insufficient.

So I would confine the holding to this: **the `BENCH_SILENT_SEAT` check as presently written
measures brevity and not participation, cannot ground a void, and must be corrected to require an
express concurrence before it is relied on for any purpose.** What follows once it is corrected is
for another day.

### On timing

I agree with Bingham that the correction obligation must be bounded and visible, and I would add
that the register must record, for each flagged order, the date the obligation arose. An obligation
without a date is an obligation nobody is failing.

---

## Disposition of the Board

**Q1.** ROUTE FOR CORRECTION. `BENCH_OPINION_MISSING` is an evidential defect in the record of a
properly constituted bench and is protected by the assented-record floor. `TIER_NOT_CONSTITUTED` and
`BENCH_SIZE_MISMATCH` remain constitutive and void on PC-16's unaltered terms.

**Q2.** The gate's blindness does not change what the record is, but is decisive as to remedy.

**Q3.** The thirteen are BINDING AND FLAGGED, in force and relied on, carrying a visible correction
obligation on a register that may only shrink.

**Q4/Q5.** Affirmed as decided by [2026] VJS-CC-VJS 11.

**On `BENCH_SILENT_SEAT`:** unanimous that the check measures brevity, not participation, and cannot
ground a void as written. Bingham would hold that a seat which neither writes nor expressly concurs
is constitutively defective; Denning dissents from deciding it; Atkin does not reach it. It is
therefore **not** the ratio, and the check must be corrected before it is relied on.

---

## Correction recorded post-judgment (2026-07-27, before publication)

The Board described the `BENCH_SILENT_SEAT` check as measuring BREVITY. That understated it, and
the correction is recorded here rather than made silently, because the Board's own rule is that a
record must state what it actually established.

On implementing D4 the two flagged seats were examined directly. **Both wrote full concurring
opinions.** The document carries "## Opinion of Tindale (concurring)" and "## Opinion of
Wilberforce J. (concurring)", each followed by substantive reasoning.

They were flagged because the heuristic ends a seat's owned block at the next occurrence of ANY
OTHER seat's name. Tindale's opinion opens "I join Marchmont and add the guard that keeps the test
honest"; Wilberforce's opens "I concur in full with Reid J.". Each judge, by crediting a colleague
in their first sentence, handed their entire opinion to that colleague and was left owning the
29 and 51 characters that preceded the credit.

So the check did not penalise brevity. **It penalised courtesy**, and it did so most severely
against the collegial style that good judgments use. The findings against PC-8 and PC-19 were false
positives and both orders are clear.

This strengthens rather than disturbs the ratio. The Board held that the check does not measure
participation and cannot ground a void. That holding is correct and is if anything better supported
by the true mechanism than by the one the Board described. Denning's dissent, which would decide
nothing further on evidence the Board had held insufficient, is vindicated: had Bingham's proposed
rule been adopted as ratio, it would have been built on two findings that were not findings at all.

*Atkin. Bingham. Denning.*
