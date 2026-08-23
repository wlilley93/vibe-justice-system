# V2 Implementation Brief: Sovereign Assent / AI Legislature (DRAFT ONLY)

## Status: NOT BINDING
## Owner: v2-kernel-team
## Assent Source: pending_v1_constitutional_route

---

## Important Notice

This document and all associated records (SPEC-ASSENT-DRAFT-001, INV-ASSENT-DRAFT-001, INV-ASSENT-DRAFT-002, etc.) are **draft implementation scaffolds only**.

They derive **no legal force** from their own existence. They are not assented. They are not Gazette-published. They are not binding.

They exist purely to:
1. Show the technical structure V2 will implement
2. Guide kernel development
3. Prepare for the V1 constitutional route

## V2 Commitment

V2 will NOT:
- Treat this draft as binding authority
- Self-authorise constitutional law
- Implement assent enforcement before V1 settlement
- Expand its own competence
- Create force from its own output

## What V2 Will Do

1. Keep the draft scaffold in `lawpack/v2/specs/SPEC-ASSENT-DRAFT-001.yaml`
2. Mark every draft record with `status: draft` and `assent_source: pending_v1_constitutional_route`
3. Implement the technical structure (field parsing, validation rules, etc.) as **no-op stubs** that activate only when a real assented instrument arrives
4. Route any attempt to commit draft law through the normal `vjs route` flow, which will require court filing or Sovereign assent

## When This Becomes Binding

Only after:
1. V1 constitutional route completes (Privy Council -> Supreme Court if needed -> Committee -> Sovereign assent)
2. Gazette entry is published
3. V2 lawpack adopts the settled instrument
4. V2 commencement conditions are met

## Current V2 State

- `vjs route` for legislative_draft: will return `court_required: true` or `human_approval_required: true`
- `vjs validate`: will reject any authority without a proper assent_source (but current lawpack does not yet require this field)
- `vjs local-ci`: will flag draft records as non-binding

## Next Step

Wait for V1 constitutional handoff. Do not proceed.
