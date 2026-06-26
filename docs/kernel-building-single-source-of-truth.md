# Kernel Building: Single Source of Truth

A unified specification for a best-in-class agent/governance kernel, synthesized from three
brownfield specs (`kernel-specs/agent-kernelb.md`, `kernel-specs/acmeco-kernel.md`,
`kernel-specs/vjs-kernel.md`), captured 2026-06-25. It defines a **set of global invariants,
each bound to a test**, and measures the VJS and Acmeco kernels against it with a remediation
plan. The goal: both our kernels best-in-class on one shared, machine-checked invariant set.

## The core finding: the three kernels are complementary, not competing

Each kernel mastered a different plane, and each is weak exactly where another is strong. A
best-in-class kernel is the union, not any one of them.

A clarification that matters for how this document is read: **all three kernels already carry a
capability primitive.** VJS has permits (its real authorization path) and now `capability.rs`; the
Acmeco kernel has an authenticated bearer/grant model with scope fences and RLS at the verb
chokepoint. Agent kernelB is the **reference exemplar** for a few specific properties of that
primitive (deny-dominance, one-shot reserve/consume, attenuating-only delegation, typed
prefix-collision resources), not the sole holder of the concept. "Reference" below means the
cleanest hand-written exemplar of a property we harvested - not an owner the others lack. The
column reads "reference exemplar", not "owner".

| Plane | Reference exemplar | What it contributes |
|---|---|---|
| Capability primitive (all three have one) | **Agent kernelB** (cleanest of three) | the full capability record, deny-dominance, one-shot, attenuating delegation, revocation-on-next-check, typed resources, names-aren't-capabilities - the properties we harvested into the unified primitive |
| Invariant->test binding | **Agent kernelB** | the meta-gate that fails the build if any claim lacks a deterministic test (the discipline that makes everything else real) |
| Irreversible-effect + human authority | **Agent kernelB** | effect reversibility classification + blocking, one-shot, decided-once approval queue |
| Runtime chokepoint + identity | **Acmeco** | one in-process dispatch, identity authenticated-by-construction, door-stamped immutable source |
| Storage-layer isolation + audit | **Acmeco** | RLS (FORCE, non-superuser, null-GUC->0), encrypt-before-write with AAD, hash-chained audit in the write tx |
| Compile-time fail-closed + drift-proof attrs | **Acmeco** | dev/no-engine paths excised from release; derived attributes + CI gates on undeclared egress/stub/schema |
| Entrenched floor + record validity | **VJS** | a floor machinery may not soften (protected from itself), constitutive-vs-correctable, every-denial-names-its-instrument, read-policy-by-reference |
| Enforcement-surface integrity + trust root | **VJS** | digest-pinned gate surface outside the witnessed code, required-CI trust root re-running the same deterministic gate |

The synthesis: **harvest the cleanest capability properties + the invariant->test gate +
reversibility/approval from Agent kernelB (the reference exemplar, since all three already carry a
capability primitive at differing maturity); the runtime chokepoint + identity + RLS + audit +
compile-time fail-closed from Acmeco; the entrenched floor + constitutive split + instrument-naming +
digest-pin from VJS; and weld them under one invariant->test binding meta-gate.**

## Part I - The layered model

A kernel is the sole, unbypassable mediator between an actor and a resource. Three planes, clearly
separated (the mistake to avoid is collapsing them or leaving any one advisory):

```
  GOVERNANCE-LAW plane   (VJS)     entrenched floor, precedent, the law a decision cites;
                                    deterministic, model-free, the policy ENGINE
        |  consulted by (never bypassed by)
  ACTION/ENFORCEMENT plane (Acmeco) the sole chokepoint: identity, capability check, the gate
        |  the only path to
  SUBSTRATE plane         (kernelB)  the primitive boundary that actually touches the resource,
                                    holds the capability, classifies the effect, audits
        |
  RESOURCE                          DB / files / network / tools / canon
```

Rule: a higher plane may be *consulted* by a lower one, but a lower plane must be the *only path* to
the resource. The governance plane decides; the action plane enforces; the substrate plane touches.
Advisory-as-law is fine; advisory-as-enforcement-boundary is the defect.

## Part II - The global invariants (each bound to a test)

Thirty invariants in eight groups. Each carries an enforcement mechanism and the SHAPE of the binding
test. The numbering (K-1..K-30) is the shared vocabulary both kernels are measured against. Provenance
in brackets [L]=Agent kernelB, [O]=Acmeco, [V]=VJS.

### G1 - The chokepoint
- **K-1 Sole mediated path.** Every action on a governed resource passes through exactly one chokepoint; no path reaches the resource around it. [O,L] *Test:* a red-team that reaches the resource around the chokepoint fails closed (and at the DB, a non-superuser role + a `via_verb` trigger).
- **K-2 Visibility != Authority.** Seeing/selecting a verb or tool never implies authority to invoke it or touch the resource; authority is checked at the chokepoint, not the wrapper. [L] *Test:* a visible verb/tool with no capability is denied at the primitive.
- **K-3 Identity authenticated-by-construction.** The principal (workspace/subject) is resolved from the authenticated bearer, never request-asserted; the door stamps an immutable `source`. [O] *Test:* a request asserting a different workspace/source is ignored; AGENT confined to its door.

### G2 - The capability primitive
- **K-4 Unified capability record.** Authority is one record `{subject, resource(typed), rights, effect(allow/deny/ask), issuer, parent_cap_id, delegation_depth, expires_at, uses_remaining, status, constraints}`. [L] *Test:* record validates; `*`-as-a-right rejected.
- **K-5 Deny-dominance, no hidden precedence.** An unconstrained deny dominates all allows; exceptions require revoke+reissue. [L] *Test:* deny beats overlapping allow; scoped deny matches only in context.
- **K-6 One-shot consumed exactly once.** `uses_remaining=1` reserved-before-effect, consumed on success, refunded on failure, auto-revoked at zero; a name lookup can't relaunder it. [L] *Test:* concurrent one-shot crosses the resource exactly once.
- **K-7 Attenuating-only delegation.** Delegation only narrows; finite-use can't be delegated/granted onward; child can't outlive parent or drop parent constraints; parent revocation kills the child. [L] *Test:* each attenuation rule + transitive revoke.
- **K-8 Revocation-on-next-check.** Revocation/expiry takes effect on the next authorization, re-evaluated against live state (no caching). [O,L] *Test:* revoke, then next call denied.
- **K-9 Typed resources, terminal wildcards, prefix-collision rejection.** Resources typed + canonicalized; `src/*` does not cover `src2/*`; bare `*` rejected; unknown constraint keys fail closed. [L] *Test:* prefix-collision + unknown-key cases.
- **K-10 Names are not capabilities.** Knowing an id/name/handle grants nothing without the capability. [L,O] *Test:* a forged/stale handle is rejected even under a broad grant.
- **K-11 Grant is transfer, not minting.** An actor can only transfer rights it holds; cannot mint deny/ask or widen. [L] *Test:* transfer-parent validation.

### G3 - Deterministic policy + fail-closed
- **K-12 Deterministic, model-free, network-free decisions.** Authorization is a pure function of recorded state; no model/net at the decision point. [V,L,O] *Test:* dep-closure bans net/model crates; decisions reproducible.
- **K-13 Fail-closed by default.** Unmapped verb, missing classifier, unparseable expiry, null scope, unknown profile, over-budget -> deny. [O,L,V] *Test:* each fail-closed default.
- **K-14 Risk is a deterministic local fact.** Risk classification is local, not model judgment; ambiguous auto-allows downgrade to human approval (e.g. shell metasyntax). [L] *Test:* risk rules + metasyntax->approval downgrade.

### G4 - The entrenched floor + record validity
- **K-15 Entrenched floor, protected from itself.** A small set of protections that no policy edit, refactor, or flag can soften; changeable only by an out-of-band human-assent act; the rule guarding it cites itself by number. [V] *Test:* a machinery edit that would narrow it is blocked/escalated, not silently applied.
- **K-16 Constitutive vs correctable.** "Is this a valid object of its kind" (never waivable) is separate from "is this object defective" (waivable under authority); no authority claim launders a constitutive defect. [V] *Test:* a forged object with valid assent stays blocked on constitutive grounds (through the real pipeline).
- **K-17 Every denial names its instrument; every grant carries its law_source.** [V] *Test:* each decision carries a non-empty citation.
- **K-18 Read-policy-by-reference.** Thresholds/sizes/policy are read from the governing record at runtime, never hard-coded in the enforcer. [V] *Test:* changing the governing record changes enforcement with no code edit.

### G5 - Audit + data protection
- **K-19 Immutable, tamper-evident audit on every transition,** in the same tx as the write, hash-chained, HMAC'd for tamper-resistance. [O] *Test:* chain verifies; a reorder/edit is detected.
- **K-20 Bounded-observability audit.** No raw sensitive content (prompts, secrets, tenant payloads, subscriber identity) stored by default; preview/size/hash only; a content-scan keeps identity out of the whole estate. [L,O] *Test:* the audit/record estate is scanned; no raw secrets or subscriber identity present.
- **K-21 Encrypt-before-write for protected data at rest,** with row-bound AAD. [O] *Test:* ciphertext at rest; AAD binds the row.
- **K-22 Per-principal confidentiality at the storage layer.** Isolation (RLS/equivalent), fail-closed by null-scope, enforced by a non-bypassing role. [O] *Test:* cross-principal read denied; unbound -> 0 rows; role is non-superuser.

### G6 - Irreversible effects + human authority
- **K-23 Effect reversibility classification** (`irreversible | rollbackable | none`); the provider must classify its own effect (fail closed if it can't). [L] *Test:* unclassifiable effect is refused.
- **K-24 Irreversible outward actions block on a durable approval queue,** resume on a one-shot human grant consumed once, decided exactly once, audited. [L] *Test:* blocking, single-decision, one-shot consumption.

### G7 - Enforcement-surface integrity + trust root
- **K-25 Enforcement surface digest-pinned outside the witnessed code;** any gate edit is non-silent and requires a deliberate re-lock. [V] *Test:* a gate edit trips drift.
- **K-26 Compile-time fail-closed.** Dev/no-engine/full-privilege paths are excised from release binaries (a type-level property, not a runtime check). [O] *Test:* a release build cannot name the dev path.
- **K-27 Required-CI trust root** re-runs the SAME deterministic gate on the canonical remote, toolchain-pinned, action-SHA-pinned; a local bypass cannot reach it. [V] *Test:* the gate is a required status check; `--no-verify` is caught on the remote.
- **K-28 Derived, drift-proof attributes.** Per-action properties (risk->autonomy, egress, budget, legal-basis) derive from one source; CI fails on undeclared egress/stub/schema. [O] *Test:* an undeclared egress/stub fails CI.

### G8 - The binding meta-gate (the keystone discipline)
- **K-29 Invariant->test binding.** Every invariant binds to at least one collectible deterministic test (and, where applicable, an adversarial attack class); the build FAILS if any invariant lacks coverage or any safety claim is unbound. [L] *Test:* the binding checker itself, run in CI.
- **K-30 No silent stubs / coverage ratchet.** Stub markers are budgeted and ratcheted down; wired-vs-unwired duty coverage is published and gated. [O,V] *Test:* the stub-budget ratchet + the conformance gate.

## Part III - The unified capability primitive

One record, generic over an OPAQUE subscriber-supplied resource/scope vocabulary (so the governance
plane never learns subscriber terms - the canon-boundary stays intact):

```
Capability {
  cap_id, subject, resource: TypedResource(opaque kind:body), rights: Set<Right>,
  effect: Allow | Deny | Ask, constraints: Map, issuer, issuer_cap_id, parent_cap_id,
  delegation_depth, max_delegation_depth, issued_at, expires_at, uses_remaining: Option<u32>,
  status: Active | Revoked | Disabled
}
```
Semantics (K-4..K-11): deny-dominance; one-shot = `uses_remaining=1` reserved-before-effect; attenuating-
only delegation with parent-chain revocation; grant-is-transfer; typed terminal-wildcard matching with
prefix-collision rejection; unknown constraint keys fail closed; names are not capabilities. A VJS "permit"
and an Acmeco "bearer/grant" both become *profiles* of this one record. The capability lifecycle governs
PROSPECTIVE authorization only and may never void/block/downgrade an entrenched assented record (K-15/K-16
dominate every capability decision).

## Part IV - Gap matrix (where we are up to)

MET = structural + test-bound. PARTIAL = present but advisory, unnamed, or not fully test-bound. GAP =
absent. (Agent kernelB shown as the reference column.)

| # | Invariant | kernelB | VJS | Acmeco |
|---|---|---|---|---|
| K-1 | Sole mediated path | MET | PARTIAL (commit-time; pre-write `covers` now routed THROUGH the capability primitive) | PARTIAL (in-proc yes; system-level: BD-5 unattached, frontend 2nd writer) |
| K-2 | Visibility != Authority | MET | **MET** (a governed/visible path with no permit-capability is denied at the primitive) | PARTIAL (present, unnamed) |
| K-3 | Identity by construction | MET | n/a (author=principal) | MET |
| K-4 | Unified capability record | MET | GAP (permits not caps) | PARTIAL (bearer close; 2 models) |
| K-5 | Deny-dominance | MET | GAP | PARTIAL (gates, no cap deny) |
| K-6 | One-shot consumed once | MET | GAP | PARTIAL (single_use) |
| K-7 | Attenuating delegation | MET | GAP | PARTIAL (convene subset) |
| K-8 | Revocation-on-next-check | MET | GAP (no call) | MET |
| K-9 | Typed resources / prefix-collision | MET | PARTIAL (globs, escape-check) | PARTIAL |
| K-10 | Names != capabilities | MET | PARTIAL | MET |
| K-11 | Grant is transfer | MET | GAP | PARTIAL |
| K-12 | Deterministic / model-free | MET | MET | MET |
| K-13 | Fail-closed default | MET | MET | MET |
| K-14 | Deterministic risk + downgrade | MET | **MET** (risk.rs: metasyntax/destructive classes; elevated auto-allow downgrades) | PARTIAL |
| K-15 | Entrenched floor (self-protected) | GAP | **MET** | GAP (overlay inert) |
| K-16 | Constitutive vs correctable | GAP | **MET** | GAP |
| K-17 | Denial names instrument | PARTIAL | **MET** | PARTIAL (receipt can't refuse) |
| K-18 | Read-policy-by-reference | PARTIAL | **MET** | PARTIAL |
| K-19 | Hash-chained audit in tx | PARTIAL | **MET** (audit.rs: sha256/HMAC chain; edit, reorder, drop detected) | **MET** |
| K-20 | Bounded-observability audit | MET | GAP (subscriber-in-logs) | PARTIAL (redaction inv) |
| K-21 | Encrypt-before-write + AAD | n/a | n/a | MET |
| K-22 | Per-principal isolation (RLS) | n/a | n/a | MET |
| K-23 | Effect reversibility class | MET | **MET** (effects.rs: 3 classes, unclassifiable refused) | GAP |
| K-24 | Durable approval queue + one-shot human | MET | **MET** (effects.rs: blocks, decided-once, consumed-once) | PARTIAL (MFA/gate.propose) |
| K-25 | Digest-pinned enforcement surface | GAP | **MET** | GAP |
| K-26 | Compile-time fail-closed | PARTIAL | **MET** (no env toggle, no feature bypass; scaffolding is cfg(test)-only) | MET |
| K-27 | Required-CI trust root (same gate) | PARTIAL | **MET** | PARTIAL |
| K-28 | Derived drift-proof attributes | PARTIAL | **MET** (lawpack digest: deterministic, content-drift detected) | MET |
| K-29 | Invariant->test binding meta-gate | **MET** | GAP (advisory map) | PARTIAL (citation not proof) |
| K-30 | No silent stubs / ratchet | PARTIAL | PARTIAL | MET |

Read-out: **VJS owns G4 + the digest-pin + trust root and is empty on the capability primitive (G2), the
runtime chokepoint, audit hardening, and reversibility/approval. Acmeco owns G1+G3+G5+G7 and is empty on the
entrenched floor/constitutive split (its governance overlay is inert) and the structural write-path. Neither
has the invariant->test binding meta-gate (K-29) that would have caught their recent gate bugs.**

## Part V - Remediation roadmap (to bring both kernels to spec)

Ordered by leverage. Each item names the target kernel and the invariants it closes.

1. **[BOTH] Stand up the invariant->test binding meta-gate (K-29, K-30) FIRST.** Port Agent kernelB's
   `check_test_invariants.py` discipline: a machine-checked `invariants.yaml` mapping every K-n to
   collectible deterministic tests, failing the build on any unbound claim. This is the highest-leverage,
   lowest-cost move and the precondition that makes every other remediation verifiable. It directly answers
   "best in class on a set of global invariants and tests." VJS today has a hand-curated advisory conformance
   map (43/281); Acmeco proves citation not property - both upgrade to the binding gate.
2. **[Acmeco] Make the write-path structural (K-1).** Attach the BD-5 `via_verb` trigger to kernel-owned
   tables, connect as a non-superuser role, finish the unitary-stack cutover (close the ~417 direct writes /
   dual-SoR mirror). Today "one write path" is convention; this makes it a DB-level guarantee.
3. **[Acmeco] Give the governance plane a real envelope or remove it (K-15, K-16, K-17).** The GOV permit gate
   is inert and grants-all with a hard-coded `Unassented`/empty envelope. Either wire a complete envelope and
   federate the VJS entrenched floor + constitutive codes into it, or remove the seam - never ship advisory-
   as-enforcement. This is where Acmeco adopts VJS's G4.
4. **[VJS] Resolve the runtime chokepoint (K-1, K-2, K-3).** VJS is commit-time by nature; cede runtime
   enforcement to the Acmeco action plane (VJS as the consulted policy engine) rather than pretending the
   advisory overlay is a boundary. Adopt the Visibility/Invocation/Authority split in commit vocabulary.
5. **[BOTH] Converge on the unified capability primitive (K-4..K-11)** (Part III), generic over an opaque
   resource vocabulary, harvesting the cleanest properties from Agent kernelB's exemplar. All three kernels
   already hold a capability primitive; this unifies them so VJS permits and Acmeco bearers become profiles of
   one record. Closes the largest cluster of gaps and removes the permit-accumulation defect class at the root.
6. **[BOTH] Reversibility classification + durable approval queue (K-23, K-24).** Adopt `external_effect`
   classification and the blocking one-shot human-approval queue for irreversible outward actions.
7. **[VJS] Bounded-observability audit + hash-chain (K-19, K-20).** Extend the boundary content-scan over the
   whole log/audit estate (the artifact_anonymity discipline) so subscriber identity cannot enter any record;
   hash-chain the decision logs.
8. **[Acmeco] Digest-pin the enforcement surface (K-25);** **[VJS] compile-time fail-closed + stub ratchet
   (K-26, K-30).** Cross-adopt each other's integrity mechanisms.

## Part VI - The keystone discipline (why K-29 comes first)

The three recent VJS bugs (permit-closed bypass, in_force mislabel, prose-boundary leak) and Acmeco's inert-
governance and unattached-writeguard gaps share one root: **safety claims not bound to a failing-by-default
test.** Agent kernelB's `check_test_invariants.py` is the cheapest, highest-leverage artifact in any of the
three repos - it converts "we believe this is enforced" into "the build fails if it isn't." Standing it up in
both kernels, populated with K-1..K-30, is the foundation: it turns this document from a plan into a ratchet,
and it is what "best-in-class on a global invariant+test set" actually means in practice.

---

*Status: synthesis complete. The PC-19 Privy reference (cross-kernel keystones K1-K4) independently granted
the same direction (the unified capability, the V/I/A split, the sole-syscall-surface, the approval queue) on
non-narrowing conditions - available to record as binding once this spec is accepted. Next: stand up K-29 in
both kernels, then work the roadmap in order, each item landing only when its K-n test is green.*
