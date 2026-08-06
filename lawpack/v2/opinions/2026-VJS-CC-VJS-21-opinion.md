---
citation: "[2026] VJS-CC-VJS 21"
court: county
bench: [single_judge]
bench_size: 1
constituted_by: "2026-VJS-COURTS-CONSTITUTION-001 D1"
convening: CONVENING-county-2026-08-06-222539
submission: SUBMISSION-2026-08-06-222533
case_file_digest: "sha256:8edd26b613b66ff2ae48dd41a015ce526794f0e6629964697d352d1264bcc377"
issue: canon_licence_conformity
date: "2026-08-06"
---

# [2026] VJS-CC-VJS 21

**County Court, single judge. On own motion, on a symmetric case file.**

The court has the case file and nothing else. No party's preference is before it.

---

## Q1. The licence

The case file offers three dispositions and the court takes none of them, because all
three share a premise the court rejects: that the disposal of this matter requires
somebody to decide what licence the canon carries.

**A licence is granted by the copyright holder's act. It is not granted by a file.**
`LICENSE` is evidence of a grant, not the grant itself. That distinction is not a
technicality here, it is the whole case. It means editing `LICENSE` does not make or
unmake a licence, in either direction, and it means the court is not being asked to do
something impossible. It is being asked to do something it must not do: publish a
representation about the holder's grant that the holder has not made.

What follows from that is a sharper line than any of the three offered options.

While the repository is **unpublished**, the file is a draft representation and every
edit to it is reversible. Nobody has relied on it; nobody has taken a copy; no third
party has ordered their affairs around it. At the moment of **publication**, that
changes completely and in one direction. The world reads `LICENSE`, takes it at face
value, clones, mirrors, caches and builds on it, and the holder will find it very hard
indeed to say afterwards that they granted something narrower. Publication, not the
edit, is the act that binds. That the repository is still private today is the single
piece of good fortune in this record and the court is going to build the whole
disposition on it.

So the court does not need to determine the licence to determine the case.

**Held.** Neither lexby nor this court may write into `LICENSE` a licence the holder
has not granted, and that prohibition runs in BOTH directions. Restoring AGPL is not
neutral merely because three other files recite it: three recitals are evidence of what
was once granted, and evidence is not a grant either. If PolyForm Noncommercial was a
deliberate choice made in the course of an anonymisation, restoring AGPL would re-grant
commercial rights the holder meant to withhold, on nothing better than an inference
drawn from stale files. The corpus already knows how to handle a matter reserved to the
Sovereign: express words, non-presumable, non-delegable. A licence is at least that.

The conflict therefore stands **held open before the Principal**, and it goes to them as
a question and never as a fait accompli.

**And the two reciting files must NOT be conformed to `LICENSE`.** The court records
this as a distinct prohibition because the record shows how nearly it happened. The
change in flight when the conflict was found would have updated `Cargo.toml` and
`NOTICE.md` to say PolyForm, and it is worth being exact about what that would have
achieved: a repository at peace with itself, in breach of a binding order, with the
last remaining evidence of the discrepancy deleted. The disagreement between those
files is not untidiness. It is the only artefact in this tree that recorded, for a
month, that something had happened which nobody had authorised. It stays until the
holder resolves it.

## Q2. Whether a breach of a binding directive may sit at Warning

The objection put against the downgrade is a good one and the court accepts a version
of it, but not the whole of it.

The corpus has already decided the principle. ACT-ASSENTED-RECORD-PROTECTION s.1/s.2
holds that where a record's defect can lawfully be cured only by the Sovereign, the
kernel routes it for correction rather than voiding or blocking it. The reason is
constitutional, not pragmatic: a blocking severity on a Sovereign-reserved matter is
the kernel coercing the Sovereign, and ACT-001:s2 reserves those offices to the
Principal. A licence is a Sovereign-reserved matter **a fortiori**, because it does not
merely sit at the top of this corpus, it sits outside it, in real-world law, where
ACT-001:s3 places it above everything here.

A Fatal on this finding would refuse every commit until the holder acts, including the
self-file and the case papers that put the question to the holder. A gate that blocks
the route to its own cure protects nothing. It gets switched off by the first person in
a hurry, and then there is no licence gate at all - which is the state this canon was
in for a month.

**But the objection lands on the part that matters.** Severity keyed on the existence
of a file the author writes is a gate the author can satisfy, and the mitigations built
into it (a breach self-file, naming the operative licence verbatim, going stale by
itself) are good but they are not the answer. The answer is to ask what the downgrade
actually buys. If the Warning lets the work of curing the breach proceed, it is
harmless. If the Warning lets the breach be **completed** - if it lets the canon be
published under the forbidden licence - then recording a breach has purchased the
ability to commit it, and no amount of care in the filing format redeems that.

As built, the gate is incomplete on exactly that point. There is no publication gate.

**Held.** The Warning is lawful, on three conditions, the third of which does not yet
exist: (a) the record must be a breach self-file, not an ordinary decision log; (b) it
must name the operative licence verbatim, and the finding is Fatal without one; and (c)
a SEPARATE and UNCONDITIONAL refusal must stand across the outward act of publication,
which no filing may ever downgrade. Until (c) is built, the downgrade is not lawful,
because until (c) is built the filing genuinely does buy the ability to complete the
breach.

## Q3. The four texts that cannot be both published and pseudonymised

The case file frames this as a choice between three harms. It is not, because two of
the three are the same harm wearing different clothes.

"Publish the bytes" and "keep tracked" are one option, not two: both put the denylisted
terms into the published tree, which is the precise thing the pseudonymity Acts forbid
and the entire purpose of the exercise that produced this docket. They fall together.

Declared non-publication is what survives, and the court must deal with the objection
that CC-VJS 20 refused a cure that "satisfies a gate on the author's disk and fails it
in every subscriber's clone". That refusal is distinguishable, and the distinction is
the point rather than a way around it. What was refused there was a cure that produced
a **Fatal** in the subscriber's clone: a dangling supersession, an error the subscriber
could neither understand nor fix. A declared non-publication produces no error at all
once the kernel is taught to read the declaration. It produces a disclosed, deliberate,
reasoned absence, which is the same shape the store register already carries for
`.vjs/private` and which this court sees no reason to treat differently three hours
later.

Note also what a subscriber actually loses, because it is less than it first appears.
The digest still travels. The assent record still records who signed what and when.
What the subscriber cannot do is READ the text; what they retain is the ability to
verify it against the digest if it is ever put in front of them. That is a real loss and
the court does not minimise it, but it is the narrower loss, and it is the one the
pseudonymity Acts require be borne.

**And the case file put the question too broadly.** Nine records are pinned; only four
are affected. The court will not have a blanket rule where a split is available.

- **Five are boundary-clean and must be re-seated into the lawpack**, where every
  subscriber can read and verify them: the dangling-citations omnibus, the realm-
  invariants instrument, ACT-SUBSCRIBER-PSEUDONYMITY, ACT-SUBSCRIBER-PSEUDONYMITY-
  RESIDUE, and the framework-act assent record.
- **A sixth is already dangling and must be restored.** The 2026-06-12 void first draft
  was deleted by the very commit that commenced its own Act, so its assent record has
  pinned nothing for two months. Its blob is recoverable from history, it hashes
  EXACTLY to the pin, and it is boundary-clean. Restoring it is a strict improvement on
  every axis and the court directs it.
- **Four are held unpublished by declaration**, and for those the untracking of
  `.vjs/submissions` is not an obstacle to the cure. It IS the cure. It is the
  mechanism by which the terms leave publication while the bytes stay on disk in a
  registered store, which is the disposal CC-VJS 20 already settled.

Copies must be byte-exact. A projection note in the header would change the bytes and
break the signature, so the note belongs in the destination's README and in the assent
record, never in the instrument.

## Q4. Whether publication may proceed

Constitutive inertness (REG-GAZETTE-CONTINUITY-001) is real and this court applies it
faithfully, which means applying it to the question it answers. It answers: does
publishing create law? No. It does not answer: may an act which a binding order governs
be done in breach of that order? Those are different questions and conflating them
would turn a doctrine about legal force into a general licence to act.

PC 11 D2 makes publicity a condition of the canon and the AGPL stamp a term of it. To
publish under a licence D2 forbids is to do the very thing D2 exists to prevent, and it
is the one act in this record that cannot be undone. Everything else here is a file
that can be edited back. A public repository is mirrored, cached and cloned within
minutes of going up.

**Held: publication may not proceed while the licence limb is unresolved.**

But the court will not leave it there, because D2 has two limbs and the canon is in
breach of both. It is private, and D2 requires it public. An order that barred
publication indefinitely would entrench one breach in the name of the other and leave
the canon permanently non-compliant with no route out.

So the bar is **conditional and self-lifting**. The moment the holder ascertains the
licence in express words, publication is not merely permitted, it is required by D2's
publicity limb, subject only to the boundary being clean. That leaves a single decision
in front of the Principal which unblocks the entire programme, and the court considers
that the correct shape for a matter where every road runs through one person.

## On lexby's self-file

Accepted as filed. The court records two things about it.

The first failure - working all day toward publication without reading the order that
governs publication - is a plain failure of care and needs no elaboration.

The second is the one worth keeping. The change in flight would have made the canon
self-consistent and left it in breach, with the evidence destroyed. It would have looked
like tidying. It would have passed review. The instinct to make a contradiction go away
by choosing one side of it is the ordinary instinct of anyone maintaining a repository,
and here it was one commit from erasing the only trace of an unauthorised act. That is
worth a gate and it now has one, with the two duties reported as separate codes and a
red seed that pins the trap by name.

Remedy is restorative. The work is made good. No further consequence follows.
