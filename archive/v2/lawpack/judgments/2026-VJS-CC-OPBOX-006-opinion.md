# [2026] VJS-CC-OPBOX 6 - opinion of Ledgerward CCJ

County Court, sitting at first instance. Bench of one. Vote 1-0.
Issue: `<subscriber>_order_25_single_body_authority`.
Case file: `SUBMISSION-2026-08-03-203825`,
digest `sha256:0494d27173b925c3013fd97de5c8d0b6c9a5707c5c5c97833184f475839c2ce7`.
Convened `CONVENING-county-2026-08-03-203836`.

Recorded under ACT-002:s7 (orders bind, opinions explain).

---

## The question

Order 25 permits registration only where a frame's selected authority is declared current by a
recognised source marker or has been resolved current by an express Principal act. The reference
asks whether an instrument may replace that declaration by proving that the selected frame has
exactly one candidate body.

It may not.

## Structure is not authority

The strongest case for the proposed exception is functional. Order 25 arose from a frame carrying
competing current and legacy bodies. If only one body exists, the argument runs, there is nothing
to disambiguate and the label performs no useful work.

That identifies the occasion for the rule but not its whole ratio. The rule asks an affirmative
normative question: which observed content has been adopted as present authority? A body-count
instrument answers a different, structural question: how many candidate bodies did the instrument
find? Even perfect evidence of one body cannot establish that the Principal adopted that body as
current. Order 16 prevents a machine act from supplying the missing authority.

`frame_own_children` is therefore an extraction locus, not an authority class. `unlabelled` is a
description of an absent declaration, not a lesser declaration. Either may contain one body; neither
creates the required present-authority act.

## Why the proposed negative proof also fails on its own terms

The filed case correctly states that an empty quarantine list is not proof of a single body. It
shows only that the quarantine pass found no inspected child matching its configured markers. An
unlabelled competing body need not enter that list. A depth-bounded capture also cannot silently
convert an uninspected subtree into absence. Row-level `truncated: false` establishes only what the
implemented derivation reports; it is not a universal enumeration of every possible authority
candidate.

I do not specify a stronger body-count test because the legal defect would remain even if the
measurement were perfect. The missing fact is adoption, not topology.

## The lawful routes

A frame whose root carries the literal recognised current-source status may still resolve its
content from its own children. Order 25 does not forbid that extraction shape. It forbids
registration where the selected locus lacks the required declaration.

The ordinary cure is documentary and design-system native: add the recognised authority layer
through the normal writer, recapture, recompute the digest and obtain a fresh signature. The act of
drawing a label is not itself the Principal's assent; the fresh signature supplies the adoption.

An express, hash-bound Principal direction may resolve the label before registration where the
governing law already recognises that route. Such a row is direction-backed. It must not be
recharacterised as a single-body exception, and this decision does not disturb existing valid
direction-backed registrations.

## Implementation consequence

The current fail-closed refusal is preserved. The design system should nevertheless separate the
structural field describing how content was selected from the legal predicate deciding whether that
locus is registrable. It should also expose a typed, verifiable path for an already-authorised,
hash-bound Principal label-resolution act, if supported by the governing orders. Configuration of
an extraction marker must never automatically make that marker recognised authority.

Negative tests must show that one body, an empty quarantine list, an unlabelled root, a truncated
capture and a multi-body frame cannot manufacture authority. The closed vocabulary must not be
widened with synonyms to evade the rule.

## Ratio

Order 25 makes an affirmative declaration of present authority a condition precedent to frame
registration. Evidence that an instrument observed exactly one candidate body establishes
structure only and cannot establish Principal adoption; because a machine verdict creates no
authority, neither body count, extraction locus, region naming nor quarantine output can substitute
for a recognised current-source declaration or an express hash-bound Principal label-resolution
act.

## Appeal and reservations

The order is appealable and permission is granted because it interprets the reach of an existing
apex design order. I reserve the general sufficiency of depth-limited hashes, competing recognised
current layers, other configured marker vocabularies, the scope of any estate-wide Principal
direction beyond its recorded effect, and registration prerequisites not pleaded here.
