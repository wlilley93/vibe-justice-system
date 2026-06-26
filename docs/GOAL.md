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

## Current state (2026-06-26, master e1cb045) — all measurable criteria MET

- **`local-ci` + `validate`:** green (all six checks pass; canon validates).
- **600-line ceiling:** zero source files over it (the last two, vjs-mcp 715 and vjs-redact
  659, were split behind their pins).
- **Enforcement surface:** zero drift; every gate file pinned and witnessed (the surface
  completeness was the audit's first fix).
- **K-29 binding debt: 0.** 27 in-scope invariants (30 minus 3 n/a), all 27 bound to
  deterministic tests; the ratchet is at its floor. (An earlier note carried a stale "15 -> 7";
  the canon has since reached 0.)
- **Forks:** the audit's doctrinal contradictions were disposed by [2026] VJS-SC 6 (full
  bench), recorded in the citator; reversible calls were decisive-call + work-log.

### The honest continuing edge (not binding debt - genuine partial satisfaction)

Three invariants sit at `partial`: bound to real tests, but not exhaustively proven, and each
needing more than another test to reach `met`. They are tracked, not papered over:

- **K-1 (chokepoint, "no path around"):** strengthened this session - the MCP record verb's
  apex bypass was closed (audit #10) and bound (`mcp_record_refers_...`). "No path around"
  is a universal claim; it stays `partial` until both front doors are exhaustively swept.
- **K-17 ("every grant carries its law_source"):** decision-log grants carry `basis`; a
  self-issued permit records `route_id` + `meaning` but no explicit `law_source` field -
  closing that is a structural change, not a test.
- **K-20 ("no raw identity stored by default"):** secrets hard-block at the store boundary;
  identity (email / private hostname) is flagged at Warning, not blocked - whether "bounded"
  means flagged or blocked is an interpretive question for the court, not a test.

## Boundaries

VJS-side is the lane; the Acmeco kernel is another agent's. Canon prose stays generic (no
subscriber repo codes / vertical names). Out-of-lane remainders are handed off, not actioned
here: the stale Acmeco vendored mirror (canon is ahead), the canon-note relay to the Acmeco
agent, and tracking the data-subject-rights requirement as engineering rather than canon.
