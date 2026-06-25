# Order authoring: grounding operative authorities (PC-17 D7)

[2026] VJS-PC 17 gives the per-incuriam doctrine deterministic teeth: every authority an
order cites in its **operative parts** (the `holding`, each directive's `must`, each
`forbidden` clause) must resolve to a defined object, or the kernel raises
`ORDER_CITATION_UNRESOLVED` (Fatal-but-correctable). The gate is **existence-only** - it
never reads what the authority says, only that it exists.

## The snake_case directive gap

The gate reliably grounds the **holding** and any operative text written in canonical
form (`ACT-COMPUTER-FIRST-REALM s.23`, `[2026] VJS-PC 16`). It does **not** ground a
directive body written as a lossy snake_case token: `act_010_s2` does not mechanically
resolve to `ACT-ASSENTED-RECORD-PROTECTION:s2`, and `act_computer_first_realm_s23` is not
a canonical id. PC-17 D7 records this as a **known, recorded gap**, not a silent one.

## The convention: `cites_authorities`

To extend the teeth past the snake_case bodies, an order may carry an OPTIONAL
machine-resolvable list of the authorities its operative parts rely on, mirroring
`supersedes`:

```yaml
cites_authorities:
  - ACT-COMPUTER-FIRST-REALM:s23
  - ACT-ASSENTED-RECORD-PROTECTION:s2
  - INV-ASSENT-SOURCE-001
  - "[2026] VJS-PC 16"
  - REG-KERNEL-001
```

The citation-grounding gate appends this list to the operative text and grounds it. So
the **author lists, in canonical form, the load-bearing authorities the directives lean
on**, and the kernel verifies they exist - the prose stays human-readable, the machine
gets a resolvable handle. An entry that resolves to nothing is caught exactly as a
hallucinated holding citation would be.

**Convention for new orders:** populate `cites_authorities` with the canonical ids /
citations the holding and directives rely on. It is optional (existing orders remain
valid), but using it is what makes a directive's authority machine-checkable rather than
trusting the snake_case prose. The disposition stays correctable: on an assented,
resolving order an unresolved entry routes for correction; it never voids the order
(per-incuriam voidness is for a court on appeal, not the clerk - PC-17 Position B).
