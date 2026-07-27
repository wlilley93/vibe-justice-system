# [2026] VJS-CC-VJS 11

## County Court (First Instance), sitting at the vibe-justice-system jurisdiction

**Submission:** SUBMISSION-2026-07-27-201553
**Case file digest:** sha256:6efd2af0db904a2d9a75a717ea4816cae6f31f12bee6d0c2e9ae6bab1fbaaad2
**Convening:** CONVENING-county-2026-07-27-201613
**Bench:** Hairline CCJ (sitting alone)
**Vote:** 1-0

---

## Opinion of Hairline CCJ

### What this court can and cannot decide

I take Q5 first, because it governs everything else.

Q1 to Q3 ask whether thirteen binding orders are void ab initio. One of them,
`[2026] VJS-PC 20`, is a **three-seat Privy Council order**. A County court has no competence to
declare a Privy Council order void, and it would be a plain constitutional error for me to do so
under cover of answering a general question. The same questions also ask whether the ratio of
`[2026] VJS-PC 16` reaches a class of records. Determining the reach of a Privy Council ratio is for
the Privy Council or above, not for First Instance.

So on Q1, Q2 and Q3 this court **refers up** and decides nothing. I record my reasons for referring
rather than dismissing: the questions are properly framed, the facts are established by seeded test
rather than argument, and the matter is urgent, because thirteen records of uncertain status are
being relied on daily while the reference sits.

I note, without deciding, that the reference discloses that its filer benefits from reading B. That
disclosure is correct and I record that it does not affect the referral, which turns on competence
alone.

### Q4 is severable, and this court answers it

Q4 asks whether the two blind validation surfaces may be repaired independently of Q1.

They may, and they must. This is not a question about the validity of any order. It is a question
about whether a command named `order validate` should perform the validation of an order. Nothing in
Q1 to Q3 bears on it: whatever the higher court decides about the thirteen, a validator that cannot
detect a constitutive defect is defective on any reading.

The facts here are not in doubt and I have satisfied myself of them independently of the reference.
Three surfaces claim to validate an order. One runs `verify_bench`. Two check three emptiness
conditions and report success in language ("Order validation: OK", "order_validate: PASS", "Orders
valid") that asserts something far broader than what was done. The seeded case is decisive: the same
bytes return `OK` from one surface and `[Fatal] BENCH_OPINION_MISSING` from another.

I would add the aggravating feature, which the reference states but does not press. `local_ci`
iterates `lawpack.orders` alone. **No County order has ever been within the scope of that stage.**
So the estate's pre-push gate has never, on any run, in any clone, evaluated a single order of the
court that produces most of the estate's orders. That is not a gate with a gap. On this subject
matter it is not a gate.

### The rule I state, because the realm keeps meeting it

**A check that reports a broader result than it computed is worse than no check**, because it
consumes the attention that would otherwise notice the absence. The realm already holds that a green
is not evidence and that a check that cannot fail is not a check. This case adds the third form: a
check that answers a question it never asked. The remedy is not merely to widen it. The remedy is
that its NAME and its OUTPUT must describe what it actually computed.

### The staging problem, and why I do not order the widening outright

The reference is right that fixing the surfaces is not neutral between the readings. Widening
`local-ci` to the `.vjs` roots today would turn the estate red on the thirteen and block every push
until the higher court rules. That would convert a referral into a stoppage, and a First Instance
court should not achieve by remedy what it lacks competence to order directly.

So I separate the two repairs by their blast radius:

- `vjs order validate` is a single-file command. It reports on exactly what it is pointed at, blocks
  nothing, and its repair cannot brick the estate. **Repair it now** (D1).
- `local-ci`'s `order_validate` is a push gate. Its scope may be widened only in a way that does not
  pre-empt the referral. It must run `verify_bench` over the orders already in its scope now (D2),
  and its widening to the `.vjs` roots is **stayed** pending the higher court (D3), because that
  widening is the step whose outcome depends on Q1.
- Whatever the higher court decides, the misleading NAMES and OUTPUTS must be corrected now (D4).
  That costs nothing and pre-empts nothing.

### The fourteenth record

`[2026] VJS-CC-OPBOX 2` is not in the affected class and I so find. Its opinion exists and is intact
in `lawpack/v2/judgments/`; the order records a bare filename where `verify_bench` resolves
`repo.join(p)`. The seat is not silent, the opinion was written, and nothing about the record's
integrity is in question. Correcting a path to point at a file that already exists changes no law and
is within this court's competence to direct (D5). I direct it be repaired immediately and separately,
so that it is not swept into the referred class and does not inflate the count the higher court is
asked to consider.

---

**Disposition:** Q1, Q2, Q3 REFERRED UP: a County court cannot declare a Privy Council order void
nor determine the reach of a Privy Council ratio. Q4 ANSWERED: repair the single-file validator now,
run the check within the existing scope now, stay the widening pending the referral, and correct the
misleading names and outputs. The fourteenth record is not in the affected class and its path is to
be repaired. Directives D1 to D5 as recorded in the order.

*Hairline CCJ*
