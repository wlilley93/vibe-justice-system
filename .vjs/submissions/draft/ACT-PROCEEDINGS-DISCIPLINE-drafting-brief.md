# Drafting brief: the Proceedings Discipline and Citation Integrity Bill

**To:** the Standing Committee (Restraint, Codification, Guardrail, Operability)
**From:** Lexby, Clerk-Drafter
**Date:** 2026-08-05
**Bill:** `.vjs/submissions/draft/ACT-PROCEEDINGS-DISCIPLINE.yaml` (first draft, 15 sections, 2,789 words)
**First-draft digest (sha256):** `f15bb2fcd5cb6fb79e2ae537…` (full digest recomputed at adoption; assent pins the ADOPTED text, not this one)

This brief follows the Framework Act precedent, where a drafting brief accompanied the first draft
(`.vjs/submissions/filed/2026-06-09-framework-act-drafting-brief.md`). It should have been written
before the draft rather than after it; that it was not is recorded here rather than tidied away.

---

## Why the Principal asked for this

The Principal asked, in terms: *"has vjs become a money pit? like real litigation i suppose. is there
legislation that could be introduced to straighten things out"*.

I answered with measurement rather than impression, and the answer was yes.

## The measured case for the Bill

All figures measured 2026-08-04/05 in the opbox subscribing jurisdiction, by counting the stores
rather than recalling them.

| Measure | Value |
|---|---|
| Orders in the jurisdiction | 116 |
| Opinions filed | 31 |
| Submissions | 79 |
| Decision logs | 121 |
| Permits | 143 |
| Words of governance prose (orders + opinions) | ~221,000 |
| Orders whose subject is the governance MACHINERY, not the product | **68 of 116 (58%)** |
| Words likewise | **58,034 of 93,759 (61%)** |
| Rulings recorded in ONE DAY (CC-OPBOX 158-164) | **7** |
| Of those, rulings existing only to correct the previous one | **4** |

**The diagnosis is not litigiousness.** Three of those four corrections happened for one reason: the
Clerk-Drafter asserted the behaviour of machinery he had not run, and a bench relied on it.

- `159` corrected `158`: I told the bench "only the TIER limb is implicated", read off the first
  refusal a fail-closed chain printed, without measuring the rest of the chain.
- `161` read down `160`: I wrote an absolute prohibition with the six causes of failure listed in
  front of me, without checking the remedy reached all six.
- `164` corrected `163`: I assumed the citation-grounding check read only directives.
  `[2026] VJS-PC 17` D1-D5 defines the surface expressly as holding plus each directive's `must` plus
  each `forbidden` clause, and the kernel says so in a comment four lines above the finding my order
  turned on (`governance/crates/vjs-engine/src/staged.rs:318-333`).

And `163` and `164` carry the **identical issue tag**. Two sittings, one matter, and nothing detected
it.

**What is NOT waste, and the Bill must not treat it as such.** `[2026] VJS-CC-OPBOX 160` found **55 of
109 order files unreadable** and absent from the citator while `vjs route` answered confidently from
the remainder. That sitting was worth many times its cost. The 58% machinery share is not the problem;
the correction rate is. Sections 1 and 2 are aimed at the correction rate alone, and the Committee
should test them against that claim specifically.

## What the Bill does

| s | Effect | Origin |
|---|---|---|
| 1 | An assertion about machinery must cite the address where it was observed, and say whether EXECUTED or READ. An inference from a name is not an observation. | Would have prevented 159 and 164 |
| 2 | One live issue tag, one order, unless the second declares its relation | 163/164 shared a tag |
| 3 | Enacts the existence limb the kernel already enforces | The check cites `ACT-002:s7`, which does not contain it |
| 4 | Citations are allocated, never asserted; interim measured-maximum rule | PC 13: eleven files self-asserted `VJS-DEC 15-22` into canon |
| 5 | Registrar track: machinery directed by regulation, no opinion | 160/161/162 were three opinions on one question |
| 6 | Consolidation duty after the second supplement | The rule now requires chasing 160→164 |
| 7 | The deliberation budget made enforceable | It exists in guidance and nothing enforces it |
| 8 | **Opening and amendment of a filed order** — slip track and substantive track | Asked for by the Principal |
| 9 | A citation may be amended only with a **forwarding record** | Asked for by the Principal |
| 10 | A NOT FOUND must state the search that produced it | A bench declined to rely on real binding law in 2026-07-31 |
| 11 | An unreadable instrument is not in force | The 55 |
| 12 | A directive with no named actor binds nobody | ~40 filed orders omit `actor` |
| 13 | A reserved question carries a review date | Reservations accumulate with no expiry |
| 14 | A duty whose gate has no negative control is UNENFORCED | Every defect this week was found by seeding or running, never by reading |
| 15 | Extent; what the Bill does not touch | — |

## What the Committee is asked to test, and what I expect to be wrong

I am the wrong person to be confident about this text, for the reason the Bill itself is about. Two
drafting errors were already caught, both of them the failure class s1 addresses:

1. **All six prohibition clauses were first written as `forbids:`.** That is not a `KernelEffect`
   field — the recognised set is `when / must / may / must_not / exceptions / proof / defines /
   prohibits / status` (`crates/vjs-lawpack/src/lib.rs:288-298`, READ) — and the struct is not
   `deny_unknown_fields`, so every clause would have been **silently dropped**. Canon uses `must_not`
   33 times and `forbids` zero.
2. **The citation was checked by measuring every store**, not by trusting `vjs next-citation`, which
   returns `[2026] VJS-VJS-CC-OPBOX 1` — doubled prefix, ordinal 1 — while `164` is in force
   (EXECUTED).

So the specific things I ask the Committee to attack:

- **Operability**: are the `kernel_effect` tokens real, or is this Bill itself a page of tokens
  declared and reaching nothing? If so, s14 condemns the Bill on the day it commences, and the
  commencement sequence must be ordered so it does not.
- **Guardrail**: **s7 is the section I am least comfortable with.** It forbids the route returning
  `CourtRequired` for a low-risk reversible non-boundary matter. If the agent self-classifies, the
  Bill may have handed the agent a route around the court — the precise opposite of its purpose. And
  **s8's substantive track** lets a making court amend "an error of its own recorded on its face";
  that is how 159 and 164 lawfully worked, but I cannot see what stops a court rewriting its own
  ratio after the fact.
- **Codification**: this may be four amendments wearing an Act's clothes. s1 arguably belongs in
  ACT-003, s2 and s6 in ACT-002, s4 in ACT-004. Consolidation over fragmentation is a governing
  principle here and a new Act that should have been amendments is itself the defect the Bill
  complains of. Also: **is a self-minted ordinal even lawful?** ACT-COMPUTER-FIRST-REALM records its
  own earlier self-mint `[2026] VJS-ACT 8` as **VOID**, the ordinal being minted deterministically at
  commencement. My `[2026] VJS-ACT 11` may be void on its face.
- **Restraint**: fifteen sections to cure a correction rate. Which of them earn their place, and which
  should be struck? I would rather adopt six sections that bite than fifteen that read well.

## Standing and route

This is **agent-originated text**. `ACT-COMPUTER-FIRST-REALM` records that force comes from "the
Committee's drafting and the Sovereign's Sovereign Assent … never from the self-assertion of V2 or any
agent", the agent-originated text there being "a void first draft cured by Assent". I drafted as
Clerk-Drafter, which is the lawful role on the Framework Act precedent, and I make no claim beyond it.

Route: **first draft → the four counsel's aye-conditions → conditions incorporated as a second-draft
cure → Standing Committee adoption (constitutive, [2026] REALM-SC 8) → Sovereign Assent pinned to the
adopted text's digest.**

My first label on the Bill was `DRAFT_PENDING_SOVEREIGN_ASSENT`, which skipped the Committee. That was
corrected before this brief was written, on the Principal's own suggestion that the committee look it
over — which was not a courtesy but the route. The drafting committee has stopped me once before for
self-commencing an instrument (`BREACH-2026-06-09-self-commenced-instrument`), and that is the relevant
precedent on my standing here.

Nothing has been enacted. No lawpack digest has been re-pinned. No subscriber mirror has been touched;
the opbox mirror was verified byte-identical to canon across all ten statutes on 2026-08-05 (EXECUTED,
`cmp` per file).
