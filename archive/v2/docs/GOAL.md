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
- **26 of 27 in-scope invariants are `met`; K-1 is `met-modulo-remainder`** (a status ruled into
  existence by [2026] VJS-SC 7 - see below). Zero `partial`, zero `gap`, 3 `n/a` by layer.
- **Forks:** doctrinal contradictions disposed by [2026] VJS-SC 6 (full bench) and the K-1
  terminal-status question by [2026] VJS-SC 7 (full bench), both recorded in the citator;
  reversible calls were decisive-call + work-log.

### K-1 and the ruled status `met-modulo-remainder`

A goal-completion audit (an adversarial multi-agent sweep, 2026-06-26) re-verified every
criterion AND audited binding QUALITY (the K-29 ratchet checks a test exists, not that it
proves its claim). Its real findings were all closed: the two criteria that were manual
snapshots became machine-checks; the unpinned gates were pinned; the weak bindings (K-4, K-12,
K-18) were strengthened; and the two closeable partials (K-17, K-20) were genuinely closed.

That left K-1. Its statement is a conjunction of two different kinds of claim, and the question
of how to record it was put to the Supreme Council, which ruled in **[2026] VJS-SC 7** (5-0 on
the core, 4-1 on the remedy; Everand J. dissenting in part):

- The ENUMERABLE conjuncts - "every action passes through one chokepoint" + the two front doors
  are thin transports that cannot drift, with the commit-time integrity gate now covering EVERY
  governed record (`front_door::is_governed_record`), a real coverage gap closed and bound by a
  content-driven-mediation test and a coverage no-drift test - are proven to the binding-quality
  standard. This **core is `met`.**
- The UNIVERSAL-NEGATIVE conjunct - "no path around it" - is not settleable by any finite test
  (a coverage test proves the modeled paths, never the absence of an unmodeled bypass), and the
  kernel's own `crates/vjs-core/src/enforcement.rs` records an irreducible author-edits-and-relocks
  remainder backstopped by NON-machine means (the Sovereign's gate + the duty of reasonable care).

The Council held the "met or partial?" question a **false binary**: bare `met` would overclaim
the negative (a paper claim the binding-quality standard forbids); bare `partial` would falsely
signal unfinished work where the core is complete and the residue is unprovable BY NATURE (an
inverse overclaim). It decomposed the invariant, recorded the core `met`, severed "no path around
it" as a named non-machine-backstopped remainder, and ruled a new status into existence:
**`met-modulo-remainder`** - the machine-checkable content proven to the binding-quality standard,
plus a named, recorded, unprovable-by-nature remainder. It is fenced (SC-7 D3): the status issues
only on full proof of the checkable conjuncts, a named recorded residue, and a finding that the
residue is unprovable by nature, not merely unfinished. Effort-closable residue stays `partial` or
`gap`; K-29/K-30 are undisturbed. The prospective drafting rule (SC-7 D4): no invariant statement
may assert a universal negative as a conjunct claimed-proven.

So K-1 is `met-modulo-remainder` by ruling, not `partial` by default. The open question is closed:
the universal negative is recorded as exactly what it is, neither overclaimed nor mislabelled as
unfinished. This is the strength of the record, not a gap in it.

## Boundaries

VJS-side is the lane; the Acmeco kernel is another agent's. Canon prose stays generic (no
subscriber repo codes / vertical names). Out-of-lane remainders are handed off, not actioned
here: the stale Acmeco vendored mirror (canon is ahead), the canon-note relay to the Acmeco
agent, and tracking the data-subject-rights requirement as engineering rather than canon.
