---
tier: privy-council
kind: request_for_ruling
status: good-law
per_incuriam: false
date: 2026-06-06
panel: ["Coade J", "Goffe J", "Sumberly J"]
referred_by: "Sovereign Founder (open question on the SI citation form left by REALM-PC 10)"
---

# [2026] REALM-PC 11

| Field | Value |
|-------|-------|
| **Citation** | [2026] REALM-PC 11 |
| **Tier** | Privy Council (constitutional first instance) |
| **Before** | Coade J, Goffe J, Sumberly J |
| **Kind** | Determination of the SI citation form (Request for Ruling) |
| **Status** | good-law |
| **Cites** | CASE-LAW s. 9, s. 11(d), s. 22(2); Bill 16 ss. 5, 7, 8, 10, 11, 12, 13, 15, 19, 21; Bill 14 ss. 6(i), 18, 28; extends [2026] REALM-PC 10; applies [2026] REALM-PC 4 |

## The question

The Privy Council in [2026] REALM-PC 10 recommended a single Statutory Instruments framework and, at Schedule 1 paragraph 2, illustrated an SI grammar slot for Bill 16 s. 7 "e.g. `[YEAR] REALM-SI N`". That "e.g." was illustrative, not decided. The Founder now squarely raises the open question: when the realm cites a statutory instrument, does the citation get a flat number only, does the number also encode the parent Act, or does the parent linkage live only in the recitals? We sit at constitutional first instance to determine the canonical SI citation form, exactly how the engine mints it, the recitals rule, and the closed SI status vocabulary.

## The three forms weighed

**Form A (FLAT)** mirrors the UK model exactly: `[2026] REALM-SI 1`, with the parent Act stated only in the recitals. It is mechanically the cleanest fit for the existing engine and is collision-free by the Part-4 gate. Its weakness is constitutional, not operational: it would make the SI series the one place in the realm's entire citational architecture where the visible label carries no provenance, in a settlement whose deliberate design (Bill 16 s. 7(3); CASE-LAW s. 11(d)) is that the code encodes authority and origin, not house style.

**Form B (PARENT-KEYED)** encodes the parent in the number itself, e.g. `[2026] REALM-SI 21.1` or a `REALM-SI-B21` code. It is the most faithful to the provenance slogan in the abstract, but it fails on the hard edges and on the engine. First, Bill 16 s. 7(3) requires each code to map to exactly one AUTHORITY LEVEL; an SI's authority level is invariant ("subordinate law, below Acts and case law", PC-10 ratio limb (i)). The variable is the enabling power, which is a vires fact, not a tier fact. A per-parent code conflates the two and, worse, splinters the one series into a per-parent family that the entrenched singleness invariant (Bill 16 s. 10, giving effect to CASE-LAW s. 9 / s. 22(2)) forbids: a routing label may record a local seat but "never asserts a separate law, a separate citator, a separate register" (s. 10(2)). Second, the `21.N` sub-ordinal is not expressible in the present grammar: s. 7(1) fixes N as "the deterministic ordinal within that code and year" (a flat integer), and the engine's `nextCitation` captures the ordinal as `(\d+)` only. Form B therefore cannot be minted without amending both the grammar and the engine's capture - and even then it breaks on the multi-parent case (which Act owns the slot? a `21.N-13.M` notation is ambiguous and strains the deterministic gate) and on the amendment case (an amending SI drawing a NEW parent would be mis-keyed). Form B promises provenance but delivers a brittle, collision-prone, singleness-offending number.

**Form C (HYBRID)** keeps the number flat, deterministic, and collision-free - `[2026] REALM-SI 1` - and ALWAYS shows an adjacent, derived parent tag: `[2026] REALM-SI 1 (under Bill 21)`. The number is a clean global annual ordinal minted by the unchanged engine algorithm; the tag is a mechanical extraction from the instrument's own recitals.

## Ratio: Form C, and why

We adopt **Form C (HYBRID)** as the canonical SI citation form. The ratio rests on four pillars, each grounded in enacted law.

*1. Provenance is honoured without corrupting the number.* The realm's philosophy (Bill 16 s. 7(3); s. 11(d) as amended) is that authority and origin are legible at a glance. Form A abandons that for SIs; Form C keeps it. But provenance for an SI is its VIRES (which Act empowered it), and vires is a fact derived from the recited enabling power, not an authority tier. So provenance belongs in a tag drawn from the recitals, adjacent to the number, not baked into the ordinal. The number remains what s. 7(1) says it is - a deterministic integer - and the provenance rides alongside it.

*2. Deterministic, collision-free minting is preserved exactly.* Bill 16 s. 8 commits numbering to the engine with zero policy choice, and s. 19(2)(a) fails closed on a duplicate CODE+N+year. Form C uses the engine's existing algorithm verbatim on a single code `REALM-SI`: scan the citator for the max flat integer, return N+1. No per-parent counter, no sub-ordinal, no new collision surface. The parent tag is minted by a SEPARATE deterministic pass over canonical markdown (zero model tokens, s. 19(3)), so it adds no judgement on the hot path. This is decisive against Form B, which the engine cannot mint without grammar surgery, and it is the synthesis that keeps Form A's mechanical virtues.

*3. The hard edge cases resolve cleanly only under C.* (a) MULTI-PARENT: an SI made under Bill 21 and Bill 13 is `[2026] REALM-SI 4 (under Bill 21 and Bill 13)` - one flat number, a tag listing all parents, all powers recited in full. Form B has no honest home for this; Form A hides it. (b) AMENDMENT: an amending SI is itself a fresh SI (PC-10 Schedule 1 para 5) with the next flat number and its own parent tag; the relationship to the SI it amends lives as a `supersedes:` / `amended-by:` pointer in the register on the append-with-supersede rule (Bill 16 s. 13), exactly as case-law amendment chains already work - never in the number. This keeps the number stable and avoids Form B's false implication that `21.6` amends `21.5`.

*4. The derived/pointer-only register doctrine is respected, not strained.* [2026] REALM-PC 4 and Bill 16 s. 12 / s. 21 hold the committed markdown is the sole source of law and any register is derived, pointer-only, deterministically rebuildable. Form C's parent tag is regenerated from the recitals in the same atomic act that writes the register row (s. 12(2)), so it can never silently diverge from the canonical text; if tag and recital ever disagree, the recited power (canonical markdown) governs and the tag is corrected on rebuild. Form A would push the parent into a register-lookup-only posture (answering "which Act?" by reading a derived store), which sits less comfortably with making provenance visible on the face. Form C makes the provenance fact visible AND keeps it derived.

UK practice is the realm's template, not its master: the UK puts the parent in the recitals and uses a flat number. Form C keeps the UK flat number and the UK recitals rule untouched, and adds the realm's signature - on-the-face provenance - as a derived gloss. It is the only form that reconciles the template with the realm's constitution.

## The recitals rule

In all three forms the enabling power is ALWAYS in the recitals; that is how vires is shown and is non-optional (Bill 14 s. 6(i), s. 18(1)). The mandatory formula is "In exercise of the powers conferred by section X of Bill NN, the [s. 8-roll office] makes the following Regulations." Where an SI is made under more than one Act, every enabling Act and section is recited in full - no principal/incidental demotion - because vires must be shown for the whole instrument, and liability for a defective SI lies on the parent authority so recited (Bill 14 s. 28). The hybrid tag is derived FROM this recital and lists the same Acts; the recital governs if they ever differ.

## Disposition

We DETERMINE Form C as the canonical SI citation form, to be enacted in the recommended Bill 14 amendment with the Bill 16 s. 5 / s. 7 / s. 15 wiring. Canonical short-cite: `[YEAR] REALM-SI N (under Bill NN)`. Status vocabulary (closed, gate-checked, Bill 16 s. 15): made, in-force, amended, revoked, spent. This completes the citation scheme rather than amending the settlement: it adds no self-commencing supreme law, splits no series, touches no entrenched article, and stays within the Bill 14 ceiling and the s. 7 grammar the Council already certified. It therefore needs NO leapfrog to the Supreme Court and is enacted by the ordinary route (Committee draft, Sovereign Royal Assent), consistent with [2026] REALM-PC 10.

## Schedule 1 - the engine specification (binding on the Bill 16 amendment and `cli/lib/citation.js`)

The series code is the literal string `REALM-SI` (one code, one authority level: "subordinate law", satisfying Bill 16 s. 7(3) - exactly one level per code). `seriesCode()` gains a case `'statutory-instrument'` / `'si'` returning `'REALM-SI'`, with no division/repo opt. `nextCitation(citator, 'statutory-instrument', { year })` then runs the UNCHANGED algorithm: `highestN` scans the citator for `/\[YEAR\]\s*REALM-SI\s+(\d+)/gi`, takes the max, returns N+1 as a flat integer. No parent table, no per-parent counter, no sub-ordinal grammar - so it stays inside Bill 16 s. 7(1) (N is the deterministic ordinal within that code and year, an integer) and inside the engine's existing `(\d+)` capture.

The parent tag is minted SEPARATELY and deterministically as a derived field, not by the numbering function: a helper `parentTag(instrumentMarkdown)` reads the SI's own recitals (the mandatory "In exercise of the powers conferred by section X of Bill NN ..." line that Bill 14 s. 6(i) / s. 18 already require on the face), extracts every Bill number cited as an enabling provision, sorts them ascending, and renders `" (under Bill 21[ and Bill 13])"`. This is a pure regex over canonical committed markdown - mechanical, zero model tokens (Bill 16 s. 19(3)), and deterministically rebuildable (Bill 16 s. 12(2), s. 21). The citation object returned by `nextCitation` gains a `parentTag` field and a `display` field `` `${citation} ${parentTag}` ``; the slug stays `2026-realm-si-N` (flat, from the integer only) so file paths never churn when a parent set changes.

Collision-freeness is the existing Part-4 gate verbatim (s. 19(2)(a) duplicate CODE+N+year): no two SIs share `[YEAR] REALM-SI N`. An amending SI is itself a fresh SI: minted the next flat `REALM-SI` integer and tagged with ITS OWN parent (the same parent authority that makes the amendment, PC-10 Schedule 1 para 5); the amend relationship (which SI it amends) is NOT in the number and NOT in the tag - it is a `supersedes:` / `amended-by:` pointer in the derived register row (Bill 16 s. 13 append-with-supersede), exactly as the case-law amendment chain works. Determinism holds because parent extraction reads only the committed text, and the gate regenerates the tag in the same atomic act that writes the register row (Bill 14 s. 18(2), Bill 16 s. 12(2)).

## Lexby TL;DR

You asked whether a statutory instrument just gets a number, or whether the number should point at the parent Act, or whether that linkage only lives in the recitals. The Privy Council weighed three forms and chose the **hybrid**: the SI gets a plain running number, `[2026] REALM-SI 1`, **always shown with a tag in brackets saying which Act(s) it was made under** - `[2026] REALM-SI 1 (under Bill 21)`. The number stays a clean integer the engine already knows how to mint collision-free; the `(under Bill 21)` tag is read automatically off the instrument's own recitals (no guessing, no tokens), so it can never drift from the law. The full enabling power is **always** written out in the recitals regardless - that is what actually gives the instrument its authority; the tag is just a convenience label. Made under two Acts? `(under Bill 21 and Bill 13)`. An amending instrument gets its own fresh number and tag, and a pointer in the register saying which instrument it amends. Status words for SIs: made, in-force, amended, revoked, spent. No Supreme Court referral needed - this finishes the citation scheme rather than changing the constitution.

**Appeal:** Privy Council determination at constitutional first instance. Appeal lies by automatic leapfrog certificate (s. 13, s. 20) direct to the Supreme Court - but the Council holds this completes the citation scheme rather than amending the settlement and raises no point warranting a leapfrog.
