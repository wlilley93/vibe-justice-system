# [2026] REALM-PC 4

| Field | Value |
|-------|-------|
| **Citation** | [2026] REALM-PC 4 |
| **Tier** | Privy Council |
| **Judge** | Coade J |
| **Kind** | Request for Ruling |
| **Status** | good-law |
| **Cites** | s. 1, s. 8, s. 11(c), s. 12, s. 15, s. 19(5); VPR 2, VPR 8; extends [2026] REALM-PC 3; persuasive [2026] REALM-SC 1, [2026] REALM-SC 3 |

## The question

Should VJS adopt a deterministic semantic retrieval layer (a vector index, e.g. sqlite-vec, built over the canonical committed markdown and queried by a deterministic command returning top-K citations without spending model tokens on the search) to reduce token consumption and raise the fast-path hit rate; and if so: (1) must it be index-not-replacement with the committed markdown remaining the single source of law under s. 1; (2) what measurable condition triggers adoption, and must cheaper intermediates (tag-sharding, grep/keyword pre-filtering) be exhausted first; (3) does it offend s. 12 anti-bloat or fall within its allowance for what compresses or screens?

## Standing / fast path

Standing established (a live, forward-looking architectural fork; the conceded fact that demonstrated-need is unmet at seven rulings goes to disposal, not standing). No fast path: nearest authority [2026] REALM-PC 3 (summary-with-pointer) is strongly persuasive but governs README information architecture, not a machine-built index over the whole corpus; genuine first-impression fork.

## Ratio

A deterministic, token-free semantic retrieval layer (a vector index built over the canonical committed markdown and queried by a deterministic command returning top-K citations) is a **screening device within the express allowance of s. 12 and does NOT offend it** - but only on three cumulative and binding conditions, the absence of any one of which renders adoption unlawful as bloat (s. 12) or as a competing source of law (s. 1):

1. **Index-not-replacement is MANDATORY.** The committed markdown (the citator and the full judgments) remains the single and only source of law under s. 1. The index must be wholly derived, deterministically rebuildable from that markdown, and never the store of any ratio, status, or citation. It points to canonical text; it never speaks the law. A query result is a pointer to be verified against the committed markdown, never itself authority. To guarantee this against drift, the index must be regenerated deterministically as part of the same operation that amends the citator, so it can never silently diverge from the source. This extends the summary-with-pointer doctrine of [2026] REALM-PC 3 from a human onboarding view to a machine retrieval view; severability holds because the index can drift only into harmless staleness (a missed pointer cured by the unchanged markdown), not into a competing rule.

2. **Adoption is GATED on demonstrated need and is presently FORBIDDEN.** At the current corpus the machinery costs more tokens to build and embed than it saves, so the demonstrated-need gate of the three-gate split test is not met (s. 12; s. 15). Adoption becomes permissible only upon a stated, measured condition being met on the field record: (a) the citator, loaded entire, exceeding a token budget fixed by Lexby in advance, OR (b) a measured fast-path miss rate (on-point precedent that better matching would have surfaced) exceeding a stated rate, measured not speculated.

3. **Cheaper deterministic intermediates must be exhausted first.** Tag-sharding the citator (VPR 8) and grep/keyword pre-filtering (both spending no model tokens) must be specified, tried, and shown insufficient before the vector layer is built; their exhaustion is a precondition of the demonstrated-need gate, because s. 12 obliges screening by the least-cost means that suffices. Until the gate is met, the s. 8 forward duty is to monitor and report the two measures, not to build.

## Obiter

The single-source risk (an operationally-queried index becoming de facto authority) is answered by design, not prohibition: pointers-only results plus same-act regeneration collapse the drift window. Jurisdiction and progression objections rejected: designing VJS's own retrieval substrate is a s. 14 design question, properly determined at First Instance under s. 13. The particular numeric thresholds are reversible, low-blast engineering choices for Lexby to fix by a decisive call and a one-line note, provided they are stated in advance and measurable. Nothing authorises any non-deterministic or model-token-spending retrieval (consistent with the s. 19(5) preference for deterministic machinery on integrity-bearing functions).

## Enactment - the forward duty and the stated thresholds (s. 8)

Adoption is **not** undertaken now (forbidden). The forward duty is to monitor and report two measures; the trigger thresholds are fixed in advance here as the decisive call the ruling delegates (reversible; amendable by a one-line note):

- **Trigger (a) - citator token budget:** build is permitted once `.justice/INDEX.md`, loaded entire, exceeds **50,000 tokens** (roughly 1,200+ ruling rows at the current one-line-per-ruling density). Below that the always-on load cost is immaterial.
- **Trigger (b) - fast-path miss rate:** build is permitted once the measured fast-path miss rate exceeds **20%** over a rolling window of at least **20** fork checks (more than one in five forks that should have cited an on-point precedent instead convened).
- **Cheaper intermediates to exhaust first (in order):** (i) grep/keyword pre-filter of the citator before reading ratios (zero model tokens); (ii) shard the citator by tag/subject and load only the relevant shard. The vector layer is built only if both are shown insufficient after a trigger trips.
- **If/when built:** deterministic and token-free; fully derived from and rebuildable from the markdown; regenerated in the same act that amends the citator; returns pointers verified against the committed markdown; never stores a ratio/status/citation.

## Lexby TL;DR

The court agreed the search layer is a legitimate screening tool, not bloat, but **forbade building it now**: at seven rulings it would cost more than it saves. Keep the committed markdown as the one source of law; if a search layer is ever built it must be a derived, rebuildable, pointer-only index regenerated in lockstep with the citator. Build is unlocked only when a pre-stated measure is actually hit (citator over ~50K tokens, or fast-path misses over 20% across 20+ checks), and only after the free options (grep pre-filter, tag-sharding) are tried and shown insufficient. Until then: monitor and report those two numbers; do not build.

**Appeal:** First Instance ruling; permission to appeal to the Court of Appeal lies only on an arguable point of law or a binding-precedent conflict (VPR 3 / s. 10).
