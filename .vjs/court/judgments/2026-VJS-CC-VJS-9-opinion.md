# [2026] VJS-CC-VJS 9

## County Court (First Instance), sitting at the vibe-justice-system jurisdiction

**Submission:** SUBMISSION-2026-07-27-164512
**Case file digest:** sha256:e8e282ba81ee0d821b5713c99afa82113d074c6c747ad041353b349d01bd9277
**Convening:** CONVENING-county-2026-07-27-200358
**Bench:** Marchbanks CCJ (sitting alone)
**Vote:** 1-0

---

## Opinion of Marchbanks CCJ

### The question, and why the reference nearly answered it and stopped short

The reference asks whether two files sharing an order id and citation across `.vjs/orders` and
`.vjs/court/orders` are one record in two projections or two records, and whether
`check_citation_uniqueness` may widen to all governed-record roots before that is settled.

It is a good reference. It is symmetric, it carries no recommendation, and it sets out both readings
with their strongest arguments and their weaknesses. It is also, on its central factual claim,
correct. But it treats the consequence question (Q3) as merely blocked on the identity question
(Q1), and it is that framing which has kept the guard frozen. The consequence question has an answer
of its own, and stating it is the more useful half of this judgment.

### I did not take the facts on trust, and the first look disagreed with the reference

The reference asserts that the `.vjs/orders` copy "carries no VALUE the court copy lacks". A bench
that accepts an advocate's characterisation of bytes it can read for itself is not doing its job, so
I parsed all four files.

The first pass **contradicted** the reference. It reported that `directives` differed between the
two copies of both pairs, and that `convened_at` and `created_at` differed too. Directives are
operative parts. Had that stood, the reference's argument against reading B, that "voiding or
renumbering one of a pair whose operative parts are identical changes no law", would have collapsed,
and with it the safety of reading A.

It did not stand, and the reason matters:

- `convened_at` and `created_at` are the **same instant** in both files. One file quotes the
  timestamp and the other does not, so a YAML parser returns a string from one and a datetime from
  the other. That is an encoding artefact of the serialiser, not a difference in the record.
- `directives[0]` differs by exactly one key: the `.vjs/orders` copy carries `when`, and its value is
  `null`. Every one of the three keys unique to that copy (`repo_code`, `exceptions`,
  `source_opinion`) is likewise `null`.

Once null-valued keys are dropped and the timestamps normalised, the two files are identical on every
shared key, the `.vjs/orders` copy has **no** key the court copy lacks, and the court copy carries
`title` and `question` with real, substantial values.

So the reference was right, and I record that it was right on a point I initially read against it.
But the route to it is the holding, not a footnote: **the apparent differences between the pair are
entirely artefacts of how a struct round-trips through a serialiser.** A rule that voided or
renumbered a record on the strength of those differences would be reacting to serialisation, not to
law.

### Q1: one record, two projections

Content-wise the `.vjs/orders` file is a **proper subset** of the `.vjs/court/orders` file. That
alone would be suggestive. Two further facts make it decisive.

First, the mechanism is known and one-directional. The orders copy's key order is exactly the field
order of the `Order` struct, and `Store::write_order` serialises that struct into `.vjs/orders`. The
court copy is not generated from the orders copy by anything.

Second, and this is the fact I regard as conclusive, **the projection was lossy**. `title` and
`question` are not named fields of the `Order` struct and were dropped on parse until the 2026-07-27
serde-flatten repair. A file that silently discards the question an order answers is not a candidate
for being the authoritative record. Losing content is not a mark against being a projection; it is
the proof of it. Only a derived artefact can lose what its source holds and still be produced.

The realm has no formal concept of a projection, which is the reference's best argument against
reading A. That is a gap in vocabulary, not a fact about these files. A court that declined to name a
thing because the schema had no field for it would be letting the schema legislate. D4 closes the gap
by requiring the relationship be recorded in code, so this is not litigated twice.

### Q2: the court copy is authoritative, and both files are legitimate

`.vjs/court/orders` is authoritative. `.vjs/orders` is the kernel-store projection.

I add a point the reference supplies but does not press: **nothing resolves `.vjs/court/orders` at
runtime.** `overlay_filed_orders` reads `.vjs/orders` only. So the authoritative record has no
runtime effect and the runtime projection is not authoritative. That is not a defect. They are two
jobs: one is the record of what the court decided, the other is what the resolver loads. The pair is
the correct arrangement.

I therefore refuse the argument, which nobody made but which is the obvious next move, that the
runtime copy should be promoted because it is the one that is read. Being the file the machine reads
is not a claim to authority. It is a claim to being the projection.

### Q3 and Q4: the ratio the reference did not reach

Here I go beyond what was argued, because the reference's own facts compel it.

`governed_record_roots` names three roots and the allocator reads all three.
`check_citation_uniqueness` reads `lawpack/v2` alone, where it can see **neither copy of either
pair**, nor any County order in the estate.

So the allocator and the uniqueness guard read **different registers**. That is not a small
inconsistency. They are two halves of one rule. The allocator answers "what number is free"; the
guard answers "has this number been taken twice". A guard narrower than the allocator passes
collisions the allocator can mint. A guard wider than the allocator fails on records the allocator
cannot see. Neither half is meaningful except relative to a register, and if the two halves disagree
about the register then at least one of them is wrong **by construction**, whatever the code does.

That is the ratio: **the citation allocator and the citation uniqueness guard must read the same
register.** It generalises past this pair and past this estate, and it is the rule that would have
prevented the defect the allocator repair fixed this morning, where `lawpack/v2` held 86 citations
and not one of them County.

It follows that the guard may widen, and must, but bound to the same `governed_record_roots`
function the allocator calls, so neither can drift from the other (D1). And it must collapse by
record `id` before testing uniqueness, because two files sharing an id are one claim (D2). Without
D2 the widened guard would report the existing pairs as fatal collisions, which is the outcome the
reference correctly identified as unacceptable and wrongly concluded was a reason to wait.

On Q4, PC-13 D2's uniqueness directive reaches `.vjs`. `is_governed_record` counts both roots. A
uniqueness duty that stopped at the lawpack would place every County order in the estate outside the
rule that exists to protect it, which cannot be what D2 meant.

### Q5: County

This is machinery under an existing duty (ACT-004:s8, given effect by PC-13 D2). It creates no new
duty, touches no constitution, and reallocates no competence. It stays at First Instance.

### A note on what is NOT ordered

I decline to order any change to the two existing pairs (D5). The defect was in the guard's register.
It was never in the records. Renumbering or deleting a live County order to make a gate pass would be
altering the record to suit the instrument, which is the wrong way round, and it would destroy either
a citator entry or the question the order answers.

---

**Disposition:** One record in two projections; `.vjs/court/orders` authoritative. The guard may and
must widen, bound to `governed_record_roots` and collapsing by record id first, proven red in both
directions. County. Directives D1 to D5 as recorded in the order.

*Marchbanks CCJ*
