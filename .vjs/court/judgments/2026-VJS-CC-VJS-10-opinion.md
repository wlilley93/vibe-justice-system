# [2026] VJS-CC-VJS 10

## County Court (First Instance), sitting at the vibe-justice-system jurisdiction

**Submission:** SUBMISSION-2026-07-27-165003
**Case file digest:** sha256:22c011b25693f00bf88f4250280ea2a209e8f3a4aad26792596b3bc845ca7ec7
**Convening:** CONVENING-county-2026-07-27-200625
**Bench:** Wrenfold CCJ (sitting alone)
**Vote:** 1-0

---

## Opinion of Wrenfold CCJ

### The question

Is the tool-ranking signal for progressive disclosure DECLARED by the adapter as a stored field on
the verb row, or DERIVED at offer time from the verb row and the run token's skills?

### I decline the ground the reference offers me

The reference says the derive-don't-store ratio (county, 2026-07-17) "is engaged". It is engaged, and
I will not decide on it, because reading A answers it squarely and the reference itself supplies the
answer.

That ratio bites where a record holds **every constituent** of a value. Reading A's case against B is
precisely that the verb row does **not**: "the row carries no prominence signal beyond description
text, noun and consequence, so the rank is an inference the adapter cannot correct". If that is so,
the ratio does not reach this, and a court that invoked it anyway would be citing authority past its
ratio, which is how precedent rots.

So I put it aside and decide the question on its own facts. It happens that the answer agrees with
the standing ratio. A ratio that keeps being right by a different route is a good ratio, but the
route has to be walked.

### The value being ranked is run-relative, and that decides it

Prominence is not a property of a verb. It is a property of a verb **in this run**.

Of the three signals actually available, two are properties of the run and not of the verb at all:
what skills the run loaded, and how narrowly the run token names the verb. Only the third,
consequence, is a property of the verb.

A stored field on the verb row is **run-invariant by construction**. It has one value for every run,
every tenant, and every skill combination, forever. It therefore cannot express the quantity at
issue. Declaring it would mean storing a run-invariant proxy for a run-relative value. That is a
category error, not a design trade-off, and it is exactly the mechanism by which "the code says HIGH
while the kernel gates LOW".

The same verb is the most important tool in the world to a run that loaded the skill it belongs to,
and noise to a run that did not. No column can hold both answers.

### Two facts in the reference are stronger than the reference treats them

**A declared rank is born stale.** The reference notes that `scripts/resync-builtin-verbs.py` records
that a builtin registered into a tenant keeps its ORIGINAL verb rows forever, so a later code change
never reaches it. It offers this as an argument that a declared field "stores beside a row that
already goes stale". It is much worse than that. It means a declared rank would be **unreachable for
every tenant already provisioned**, from the moment of provisioning, with no migration path short of
the resync script. A signal that cannot be corrected after the fact is not a contract. It is a
fossil.

**A consumed server ranking itself is self-dealing.** A consumed MCP server is third-party data. To
let it declare its own prominence is to let a stranger place itself at the top of the model's
context. The realm already distrusts that surface for a value it *does* accept: `_consequence_hint`
clamps a declared `consequence` to the enum and resolves silence to low, fail-closed. And
prominence is a far more attractive thing to lie about than consequence, because consequence
constrains you and prominence promotes you. The clamp that suffices for the first would not suffice
for the second, since there is no safe default for "how important is this" that a liar cannot exceed.

### What is genuinely NOT at stake, and it disposes of reading A's best argument

Reading A's strongest point is attestation: the Codex tool-ceiling attestation needs a stable
offer, and a stored column gives one.

But the reference's own facts show what disclosure can and cannot do. `_invoke_inner` gates on the
verb row at every step: existence, `input_schema`, the grant check on the verb id,
`idempotency_mode`, `consequence`, and `output_schema` after execution. It is independent of the
offer. So **disclosure can only reduce what reaches a model context; it can never change what
reaches the table.**

Attestability, then, is not bought by declaring the signal. It is bought by making the **derivation
deterministic**. A pure function of (verbs, grants, skills) with a stable tie-break is exactly as
attestable as a stored column, and it cannot go stale, which the column can and will. Reading A's
best argument turns out to point at determinism, not at storage, and determinism is available under
B.

That is why D2 requires a deterministic tie-break on verb id: not as a tidiness measure, but because
it is the thing that actually delivers what reading A wanted.

### Q2 and Q3

Q2 does not arise on the holding, but the declared slot is **reserved, not refused**, and the
conditions are the ones this opinion's reasoning forces (D4):

1. Only the tenant's own registry may declare. Never a consumed third-party server, for the
   self-dealing reason above.
2. Silence means *no declaration*, falling through to the derived signals. It must **not** resolve to
   a default rank. A default rank is a declaration the adapter never made, and it would put every
   silent verb at one arbitrary position.
3. A declaration may reorder the offer but may **never** raise a verb the grants do not already
   permit. Disclosure reduces; it does not widen. This is already true and must remain so.

On Q3, the signals and their order, from the most directly run-speaking to the least: skill affinity,
then grant specificity, then consequence, then a deterministic tie-break on verb id. That order is
not arbitrary. It ranks by how directly each signal speaks to **this** run, which is the quantity
being estimated.

### Q4: a boltrig protocol obligation, not an Opbox local choice

The ranking happens at `_list_tools`, the kernel's own MCP face, and that is the seam that already
filters by tenant grants intersected with run-token grants. The thing that filters is the thing that
should rank; they are the same computation over the same inputs.

Building it as an Opbox special case would put one concern in two systems. That is against the
consolidation steer, and against the locus ratio that domain lives on its system-of-record kernel.
The MCP door is boltrig's. Opbox's `VerbTier` is presentation-layer, is described in its own source
as presentation-layer only, and stays that way.

### Q5: County

Machinery under an existing chokepoint. No new duty, no constitutional touch. First Instance.

### One directive that is not about the answer

D3 requires the ranker be **wired**. I record why a court is saying so. An unwired ranker decides
nothing, ranks nothing, and protects nothing; it is dead code that reads as a fix. The realm has a
reachability gate that says exactly this, and the correct response to that gate is to wire the code
or delete it, never to declare a root to quiet it.

---

**Disposition:** DERIVED, on the ground that prominence is run-relative and a run-invariant column
cannot express it. Reserved declared slot on three conditions. Built at the boltrig MCP face as a
protocol obligation. County. Directives D1 to D5 as recorded in the order.

*Wrenfold CCJ*
