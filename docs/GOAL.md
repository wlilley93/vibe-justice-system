# VJS Kernel — Goal Statement

## North star

The VJS kernel is the reference implementation of **machine-checked governance**: a single
deterministic kernel that makes the lawful thing the only thing the tree will accept, so
correctness is **enforced at the gate, not trusted to review**. Prompts guide, tools empower,
evals measure, the kernel governs.

## What "best-in-class" means here (the three prongs)

1. **One invariant set, machine-checked.** Every governance rule that matters is expressed
   once as a kernel-evaluable invariant and checked deterministically (no model calls, no
   network) at the commit gate and in CI. Subscribing repos (Acmeco, Agent kernelB) overlay onto
   the SAME set with anti-relaxation at load - never a fork of it. The CLI and MCP front doors
   are thin transports over one kernel, so they cannot drift (D4: one smart point).

2. **Structurally clean.** Every source file under the 600-line ceiling; one concern per
   place; the security-critical gates small enough to read in full. Consolidation over
   fragmentation - never split a single concern across systems.

3. **Achieved securely.** Every change is behavior-preserving or court-gated. The bright-line
   gates (assent-resolution floor, bench integrity, apex/federation routing, canon-write
   boundary, citation uniqueness) are digest-pinned so a weakening edit is non-silent. Every
   load-bearing invariant is bound to a real test (the K-29 ratchet). An assented record is
   never voided, only routed-for-correction. Doctrinal forks are resolved by a recorded
   ruling in the citator, not by ad-hoc choice; Supreme judgments are delivered in full.

## Done criteria (measurable)

- `vjs local-ci` and `vjs validate` green on canonical `master`, every commit.
- Zero source files over the 600-line ceiling.
- Zero enforcement-surface drift; every bright-line gate file pinned and witnessed.
- Zero unbound load-bearing invariants (K-29 binding debt at 0).
- Every first-impression design/scope fork disposed by citator ratio, decisive call, or court
  ruling - never routed to the Principal except for an external dependency or an irreversible
  outward-facing action.

## Current state — v1.0.0 (2026-06-26) — all measurable criteria MET, machine-checked

- **`local-ci` + `validate`:** green (all six checks pass; canon validates).
- **600-line ceiling:** zero source files over it - now MACHINE-CHECKED (`structural_ceiling.rs`
  under required CI), not a manual snapshot.
- **Enforcement surface:** zero drift; every bright-line gate file pinned and witnessed,
  including the staged-pipeline gates (staged.rs / validator.rs / refs.rs); the drift
  mechanism's positive direction is itself proven (`check_drift_flags_an_edited_gate`).
- **Dependency fence (K-12):** ENFORCED against the real `Cargo.lock`
  (`dependency_fence.rs`), not merely asserted in config.
- **K-29 binding debt: 0.** 27 in-scope invariants (30 minus 3 n/a), all 27 bound to
  deterministic tests; the ratchet is at its floor.
- **26 of 27 in-scope invariants are `met`; one (K-1) is honestly `partial`** (see below).
- **Forks:** doctrinal contradictions disposed by [2026] VJS-SC 6 (full bench), recorded in
  the citator; reversible calls were decisive-call + work-log.

### The honest continuing edge — one partial, not papered over

A goal-completion audit (an adversarial multi-agent sweep, 2026-06-26) re-verified every
criterion AND audited binding QUALITY (the K-29 ratchet checks a test exists, not that it
proves its claim). Its real findings were all closed: the two criteria that were manual
snapshots became machine-checks; the unpinned gates were pinned; the weak bindings (K-4, K-12,
K-18) were strengthened; and the two closeable partials were genuinely closed:

- **K-17 ("every grant carries its law_source") -> met.** A self-issued permit now records its
  `law_source` (the route's binding authorities) at issue time, in both issue paths.
- **K-20 ("no raw identity stored by default") -> met.** Proven that the store boundary blocks
  identity (email / internal hostname), not only secrets - both fail closed at write.

- **K-1 (chokepoint) stays `partial` - deliberately, and now precisely.** K-1's statement is
  a conjunction of two different kinds of claim:
  - The ENUMERABLE conjuncts - "every action passes through one chokepoint" + the two front
    doors are thin transports that cannot drift - are bound (forged-order fail-closed,
    governed-record classification, capability equivalence, deny-dominance, MCP apex routing).
    A post-v1 hardening then made the commit-time integrity gate cover EVERY governed record
    (`front_door::is_governed_record`), not just the lawpack canon tree - closing a real
    coverage gap - and bound it with a content-driven-mediation test (a governed order written
    raw, skipping every verb, is still gated) and a coverage no-drift test (the gate's set is
    derived from the front door, so it cannot silently diverge).
  - The UNIVERSAL-NEGATIVE conjunct - "no path around it" - is not settleable by any finite
    test (a coverage test proves the modeled paths, never the absence of an unmodeled bypass),
    AND the kernel's own `crates/vjs-core/src/enforcement.rs` candidly records an irreducible
    remainder: an author with full write access who edits a gate and re-locks is beyond any
    in-binary check; the backstop is the Sovereign's gate + the duty of reasonable care, which
    are NON-machine. You cannot mark `met` an invariant your own enforcement surface says has
    a standing remainder.
  So K-1 stays `partial` - not as a vague edge but as a principled one. Promoting it to `met`
  would be exactly the paper-claim the audit warned against. If a `met` status is ever sought,
  that is a first-impression doctrinal question (can an invariant with a universal-negative
  conjunct ever be `met`?) for the court, not a unilateral call. This is the strength of the v1
  record, not a gap in it.

## Boundaries

VJS-side is the lane; the Acmeco kernel is another agent's. Canon prose stays generic (no
subscriber repo codes / vertical names). Out-of-lane remainders are handed off, not actioned
here: the stale Acmeco vendored mirror (canon is ahead), the canon-note relay to the Acmeco
agent, and tracking the data-subject-rights requirement as engineering rather than canon.
