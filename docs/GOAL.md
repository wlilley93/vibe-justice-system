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

- **K-1 (chokepoint, "no path around it") stays `partial` - deliberately.** It is bound by five
  real chokepoint tests (forged-order fail-closed, governed-record classification, capability
  equivalence, deny-dominance, MCP apex routing), but "no path around it" is a UNIVERSAL claim:
  proving the absence of any bypass is not a single test. Forcing it to `met` would be exactly
  the paper-claim the audit warned against, so it is recorded truthfully as the one remaining
  edge. This is the honest state of a v1, not a gap in it.

## Boundaries

VJS-side is the lane; the Acmeco kernel is another agent's. Canon prose stays generic (no
subscriber repo codes / vertical names). Out-of-lane remainders are handed off, not actioned
here: the stale Acmeco vendored mirror (canon is ahead), the canon-note relay to the Acmeco
agent, and tracking the data-subject-rights requirement as engineering rather than canon.
