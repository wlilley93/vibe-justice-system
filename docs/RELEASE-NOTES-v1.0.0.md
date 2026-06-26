# VJS Kernel v1.0.0

The Vibe Justice System kernel: the reference implementation of **machine-checked
governance**. A single deterministic kernel (no model calls, no network) that makes the
lawful thing the only thing the tree will accept - correctness enforced at the gate, not
trusted to review. Prompts guide, tools empower, evals measure, the kernel governs.

## What v1.0.0 guarantees

All five measurable done-criteria are met AND machine-checked on every commit (the required
CI re-runs the same deterministic gate a local `--no-verify` cannot reach):

- **`vjs local-ci` + `vjs validate` green** - lawpack validation, citation uniqueness, the
  canon-write boundary, order/bench integrity, invariant evaluation, the permit gate.
- **Zero source files over the 600-line ceiling** - enforced by `structural_ceiling.rs`.
- **Zero enforcement-surface drift; every bright-line gate pinned and witnessed** - the
  assent floor, bench integrity, apex/federation routing, the canon-write boundary, citation
  uniqueness, the staged-commit gates, and the citation-grounding teeth. The pin's own
  mechanism (an edited gate trips a Fatal finding) is proven.
- **K-29 binding debt: 0** - all 27 in-scope invariants (K-1..K-30 minus 3 n/a) bound to
  deterministic tests; the ratchet can only decrease.
- **Forks disposed by ruling** - doctrinal questions resolved by recorded court orders in
  the citator, not ad-hoc choice; Supreme judgments delivered in full.

## The invariant set (K-1..K-30)

26 of 27 in-scope invariants are `met`; **one (K-1, "no path around the chokepoint") is
honestly `partial`** - it is bound by five real chokepoint tests, but "no path around it" is
a universal claim no single test settles, so it is recorded truthfully rather than papered to
`met`. Groups: chokepoint, capability primitive (deny-dominance, one-shot reserve/consume,
attenuating delegation), determinism, the entrenched floor (constitutive-vs-correctable, the
assent-resolution floor, every-grant-carries-its-law_source), hash-chained audit,
reversibility + the decided-once approval queue, surface integrity + the required-CI trust
root, and the binding ratchet itself.

## Highlights of the road to v1

- A whole-repo audit closed (enforcement-surface completeness, the canon-write secret/PII
  scanner, MCP verb hardening, validator-coverage seams, latent kernel-correctness fixes).
- The unanimous Supreme judgment **[2026] VJS-SC 6** reconciled the Court-of-Appeal /
  constitution and constitutive-bench questions.
- Structural sweep: every kernel source file brought under 600 lines.
- An adversarial **goal-completion audit** then made the goal's own criteria machine-checked
  (the line ceiling and the dependency fence were manual/asserted; now both are gates),
  strengthened weak bindings, and closed the two closeable honest partials (K-17, K-20).

## Boundaries

The VJS canon is generic - it names no subscriber repo codes or verticals. Subscribing
kernels (Opbox, Agent libOS) overlay the same invariant set with anti-relaxation at load;
they never fork it. The kernel is clerk, not court: a permit is an agent-routed self-issue,
not an external approval, and an assented record is routed-for-correction, never voided.
