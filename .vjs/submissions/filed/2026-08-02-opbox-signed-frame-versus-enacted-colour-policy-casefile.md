# Case file: the signed frame and the enacted colour-fill policy collide, and they collide in both directions

Filed 2026-08-02 by Lexby, County Court, jurisdiction `opbox`, issue
`opbox_signed_frame_versus_enacted_colour_policy`. Symmetric: neither side is the filer's
preference, and no disposition is proposed.

Repo under examination: `/home/jellytot/Projects/opbox-prod/opbox-frontend`, at commit
`4042eaa2`. Every path below is relative to that repo unless it begins `lawpack/` or `.vjs/`,
in which case it is in `vibe-justice-system`.

---

## A. Citator check, done first

`vjs lookup --issue opbox_design_control_boundary_contrast` returns
`2026-VJS-CC-OPBOX-CONTROL-BOUNDARY-CONTRAST-002` ([2026] VJS-CC-OPBOX 2). It is on the same
subject matter and it is NOT on all fours, for a reason the order states in its own words. Its
`reserved` list at `.vjs/orders/2026-VJS-CC-OPBOX-CONTROL-BOUNDARY-CONTRAST-002.yaml` includes:

> which of the Figma file or the repository is the source of truth for --border; the authority
> limb of the divergence is a separate matter

That is this matter, generalised from one token to the colour of a control fill. CC-OPBOX 2
decided a CONTRAST FLOOR on a boundary stroke. It did not decide WHICH INSTRUMENT WINS when a
signed drawing and an enacted written policy disagree about a fill.

`vjs lookup --issue opbox_signed_frame_versus_enacted_colour_policy` returns ACT-001:s1 to s5 and
no order. `vjs route --kind court-filing --issue <same>` returns `Decision: CourtRequired`,
`Court required: true`, and the duties `convene_the_named_court_on_own_motion`,
`file_symmetric_case_file_no_preference`, `route_the_fork_to_the_principal` FORBIDDEN.

Searched for any order on colour fills across `.vjs/orders`, `lawpack/v2/orders`,
`lawpack/v2/judgments`, `.vjs/court`: no hit for `uniformity`, `accent`, `colour-fill`,
`POLICY_CONFLICT`, or `signed frame`. There is no ruling to follow and none to distinguish.

---

## B. The two instruments, quoted

### B1. The enacted policy

`internal-docs/planning/UNIFORMITY-CONTRACT.md:82`, verbatim:

> **No accent-colored or colored-danger fills:** primary and danger stay ink-black by policy;
> destructive semantics carry through copy/icons. `--accent` is reserved for active/selection
> emphasis and focus rings. Evidence: `app/globals.css:4525-4540`,
> `src/components/onyx/ActionButton.tsx:23-33`.

The document describes itself at `UNIFORMITY-CONTRACT.md:3` as "the gradeable rubric that defines
a 'finished' Opbox screen ... the bar every `app/(app)` screen is graded against". Section 6 opens
at `:79`: "tone colors live on chips/status, never on filled action buttons."

### B2. The signed frames

`internal-docs/design/frame-registry.json`, key `_enactedBy`: "[2026] VJS-SC-OPBOX 1, orders 6, 9,
13, 14, 15, 16, 22, 23, 24, 25, 27, 30, 31". Key `entries` holds 160 rows. Key
`_signatureDirection` records the Principal's own words of 2026-08-02:

> Everything in figma is signed with variation, meaning, signed off, but where the site has more
> features than the figma suggests, default to finding a nice way to include those additional
> pieces on the page

and its recorded effects include: "The signature is QUALIFIED 'with variation'. It is not a claim
that the shipped screen is pixel-identical to the drawing", "WHERE THE SITE HAS MORE THAN THE
FRAME DRAWS, the excess is NOT a defect", and "WHERE THE SITE HAS LESS THAN THE FRAME DRAWS, that
is unchanged: the frame binds and the gap is owed work".

`.vjs/court/2026-VJS-SC-OPBOX-1-design-constitution.md` (in the opbox repo), the operative orders
relied on by both sides:

- order 6, line 415: "A frame binds only from hash-bound entry in the sign-off registry, only for
  the surface it covers, and only for what it draws in the states it draws; a frame's silence
  contracts nothing."
- order 8, line 417: "The signed frame is the contract for the presentation of its surface, not an
  exhaustive inventory of function."
- order 11, line 425: "A signed frame proved wrong at execution is cured by filed discrepancy and
  expedited redraw-and-sign with the code state preserved meanwhile, never by code-first
  divergence; conformance to a known-wrong drawing is itself a breach."
- order 16, line 433: "Machine visual_review classifies against the registered record; it never
  adjudicates taste and its verdicts create no authority."
- order 23, line 449: the sign-off registry "is a condition precedent to enforcement of Q2-Q4
  against any surface".
- order 24, line 450: proofs "run estate-wide in report-only (burndown) mode from day one, each
  surface flipping from report to block only at the moment its frame is registered".

FACT CORRECTION OFFERED, because the referring note put it differently. Order 16 as enacted does
NOT read "only the Principal creates authority". It reads that a MACHINE visual_review verdict
creates no authority. The proposition that a signature must be the Principal's is carried by the
registry's own `_notRegistrable._corrected` field, which says "order 16 forbids a machine act
creating authority", and by `_note`, which says "An agent that writes a signature here is still
manufacturing authority contrary to order 16". Both are the registry's prose about order 16, not
order 16's text. The court should read the order.

---

## C. The collision, measured on two named routes

### C1. `/portals/[portalId]/tasks`: the frame's fill is blue, the shipped fill is near-black

`internal-docs/design/visual-reviews.json`, `records` entry `route: "/portals/[portalId]/tasks"`,
`frame.signoffId: FRM-portals-portalId-tasks`, `reviewedAt: 2026-08-02T14:16:01Z`, closing
paragraph of delta 0, verbatim:

> Colour: the frame's accent is blue (primary button, active-tab pill, TSK-0311 pill, selected
> chip outline); the site renders near-black for the primary button. Recorded as an observation
> only, since theme tokens are a separate instrument and demo may not be on the frame's theme.

MATERIAL QUALIFICATION, and the filer flags it against his own filing. The reviewer expressly did
NOT charge this as a deviation. It is recorded as an observation, on a stated doubt about whether
the demo capture was on the frame's theme. The verdict `deviate` on that route rests on the four
other items, not on the colour. Anyone arguing "there is a live blocking colour deviation on this
route today" is arguing past the record.

### C2. `/matters`: the frame's fill is BLACK where the shipped control is accent

Same file, entry `route: "/matters"`, `frame.signoffId: FRM-matters`, delta index 9, verbatim:

> Selected-chip styling differs at the status filter row: the frame renders the active 'All' chip
> as a solid black fill with white text; the page renders it as a light blue tinted pill with blue
> text and a blue border.

Delta index 6 of the same record: "the frame draws a single black 'New matter' split button".

So on `/matters` the frame wants INK where `UNIFORMITY-CONTRACT.md:82` reserves `--accent` for
"active/selection emphasis", and the shipped code has already spent accent there. The direction of
the conflict is reversed.

### C3. The frames disagree WITH EACH OTHER, and that is already recorded in shipped source

`src/components/ui/facet-strip.tsx:316-325`, verbatim:

> A RECORDED DEVIATION, NOT A SILENT ONE. The /matters frame draws the ACTIVE chip as a dark fill;
> the 2026-08-01 census read the other frames as drawing a blue OUTLINE and called the shipped
> black fill wrong everywhere it appears. Those two frame readings genuinely disagree with each
> other, and one component cannot satisfy both. Resolved at the component in favour of the accent
> outline ... Recorded in LOG-2026-08-01-134100 and flagged to the frame redraw lane, because if
> the frames really do disagree that is a defect in the frames to settle at redraw, not a
> per-route argument to have twenty times.

`.vjs/logs/decisions/LOG-2026-08-01-134100.yaml` (opbox repo) is that decision log. Its `basis`
cites [2026] VJS-CC-OPBOX 2, [2026] VJS-SC-OPBOX 1 Q2, and
`internal-docs/design/VISUAL-CENSUS-2026-08-01.md` item 5. Its `court_required` field is `false`.
Its `scope` expressly leaves the same ink-fill selection idiom in place on SegmentedControl,
ObjectViewTabs, the settings sub-page nav and WorkflowRunsView.

WHETHER THAT LOG WAS ITSELF LAWFUL IS BEFORE THE COURT ON THESE FACTS, because a decision log
ranks below a County Court order in ACT-001:s3 and this fork is now said to be one.

### C4. A third limb, already flagged as a fork by the implementer

`internal-docs/design/PARITY-WORKLIST-2026-08-02.md:180`, verbatim:

> **Note the standing conflict:** an ink-filled destructive is policy
> (`internal-docs/planning/UNIFORMITY-CONTRACT.md:82`, restated at
> `src/components/ui/button.tsx:92-121`), so frames drawing a pink destructive are a fork to
> settle, not a repaint to apply.

The same section records the opposite shape on about 10 routes: `/entities/equity-share-classes`
"ships the sole CTA as a 114x24 ghost `<a>` against a drawn 254x31 filled BLACK button", the same
on `/entities/files/[id]`, `/entities/global-documents/[id]`, `/matters/new`, `/onboarding` and the
auth family. So among the signed frames there are frames drawing blue fills, frames drawing black
fills, and frames drawing pink destructive fills, for the same class of control.

### C5. One route has already refused the frame in its own source

`app/(auth)/forgot-password/page.tsx:171-181`, verbatim tail:

> The colours are a straight swap - `.minimal-button-primary` and the primary variant both resolve
> to background --ink / colour --bg-primary; the frame's blue fill is refused, not overlooked
> (UNIFORMITY-CONTRACT.md:82).

`src/components/ui/button.tsx:92-121` carries the same refusal at the primitive, in capitals: "THE
INK FILL IS POLICY, NOT AN UNFINISHED PLACEHOLDER. Do not 'fill in' a red here: a proposal to
repaint this variant has already been raised and refuted once".

---

## D. Facts about the instruments that the court will want before it rules

**D1. Both of the contract's own evidence citations have rotted.** `UNIFORMITY-CONTRACT.md:82`
cites `app/globals.css:4525-4540` and `src/components/onyx/ActionButton.tsx:23-33`.
- `app/globals.css:4525-4540` is now a `@media (max-width: 767px)` touch-target and mobile-scroll
  block. It contains no button rule and no colour.
- `src/components/onyx/ActionButton.tsx` does not exist in the working tree. `find . -name
  ActionButton.tsx -not -path "./node_modules/*"` returns hits only inside `.worktrees/` and
  `.claude/worktrees/`. `src/components/ui/button.tsx:105-110` records the deletion: "That file was
  DELETED 2026-08-01 (Onyx burn-down Phase B: it had zero call sites), so the code half of the
  evidence is gone and UNIFORMITY-CONTRACT.md:82 above now carries the rule alone. Do not read the
  deletion as a repeal - nothing decided the policy differently, an unused wrapper was removed."

**D2. The restatement's citation has rotted too, and says so.** `src/components/ui/button.tsx:117`
gives the live declarations as "globals.css:4614-4618 and :4625-4629; re-verified 2026-07 - the
4461-4465 citation this comment used to carry had drifted by ~164 lines and pointed at nothing".
Measured today: `app/globals.css:4614-4618` is the `--sev-*` and `--tone-*` token block. The actual
declarations are `.onyx-btn--primary` at `app/globals.css:4894-4897` and `.onyx-btn--danger` at
`:4905-4908`, both `background: var(--ink); color: var(--bg-primary)`. So the citation that was
corrected in July has drifted again by about 280 lines. The RULE is live and measurable in the
cascade; only its addresses are stale.

**D3. The registry's row count and its prose disagree about what blocks.**
`internal-docs/design/frame-registry.json` `entries` has 160 rows. Of those, 156 have
`kind: "frame"` and 4 have `kind: "direction"`. The 156 frames cover 156 distinct routes, which
`_atParityCeiling` confirms ("signedFrameRows": 156, "routesTracked": 199). By `bindingMode`: 156
rows are `report_only`, 3 are `report_only_until_route_registered`, and exactly 1 is `binding`
(`DIR-2026-08-01-104739`, a direction, not a frame). The `_note` prose in the same file says
"Order 24 therefore has these routes BLOCKING and not report-only."

The filer takes no position on which is right. The court should know that the same file says both,
and that this file has already been corrected twice in one day for prose that contradicted its own
rows (`_notRegistrable._corrected`, and `_note`, which records "THIS PARAGRAPH WAS WRONG FOR A
DAY").

**D4. The "with variation" signature does not obviously reach a colour.** The 2026-08-02 direction
resolves two cases: site has MORE than the frame (keep it, accommodate it) and site has LESS than
the frame (frame binds, work is owed). A control that ships in ink where the frame drew blue is
NEITHER more nor less. It is the same element with a different value. The direction is silent on
that case, and SC-OPBOX 1 order 6 says "a frame's silence contracts nothing". Whether "with
variation" covers a value substitution on a drawn element is the question, and it is not answered
in the record.

**D5. The figure in the referring note could not be reproduced. NOT FOUND.** The referral describes
"a delta triage over 92 reviewed routes [that] found 64 separate POLICY_CONFLICT deltas". The
string `POLICY_CONFLICT` appears nowhere in the opbox repo (case-insensitive search over the tree,
excluding `node_modules`, returns one file, `internal-docs/design/parity-reverify-115-290.json`,
and only via the loose spelling "policy conflict" in prose, not as a classification). No field
named `POLICY_CONFLICT` exists in the review schema:
`internal-docs/design/visual-reviews.json` `_recordShape` gives the closed verdict set as
`conform | deviate | no_authority`. The "92" IS real: `records` has exactly 92 entries, all with
`verdict: "deviate"`. The "64" is not reproducible from any artefact in the tree and is recorded
here as NOT FOUND rather than paraphrased. The filer's own substitute measurement is in section F.

---

## E. The case for the SIGNED FRAME, at full strength

**E1. The frame is the only instrument with a signature on it.** SC-OPBOX 1 order 8 makes the
signed frame "the contract for the PRESENTATION of its surface". A fill colour is presentation and
nothing else. `UNIFORMITY-CONTRACT.md` carries no signature, no registry row, no hash binding, and
no citation in `_enactedBy`. It is a rubric authored by an implementer. On ACT-001:s3 the ranking
runs Supreme Court orders above local decision logs, and a rubric is closer to the latter. A
document that no Principal act ever adopted cannot outrank a drawing that one did.

**E2. The Geist frames are the migration TARGET; the contract documents the OUTGOING system.**
`UNIFORMITY-CONTRACT.md:3` says the rubric is "distilled from the reference surfaces (Matters,
Tables, Entities, Documents), the ONYX primitive library, and the written style law". The onyx
library is under active retirement: `internal-docs/design/ONYX-DEPRECATION-LEDGER.md` exists for
that purpose, and the very evidence file the contract cites was deleted on 2026-08-01 in "Onyx
burn-down Phase B" (`src/components/ui/button.tsx:105-107`). An instrument distilled from a system
being retired should not be permitted to veto the system replacing it. On this reading the
contract is not law that the frames breach, it is a snapshot of the state the frames were drawn to
supersede.

**E3. Enforcing the contract over the frames makes conformance to a known-wrong drawing
compulsory, which order 11 calls a breach.** If the frames are wrong about colour, order 11 gives
exactly one lawful cure: "filed discrepancy and expedited redraw-and-sign with the code state
preserved meanwhile, NEVER by code-first divergence". Refusing the fill at the code, which is what
`app/(auth)/forgot-password/page.tsx:181` and `src/components/ui/button.tsx:92-121` do today, is
code-first divergence from a signed frame with no filed discrepancy and no redraw obligation. On
the frame's case the estate is already in the posture order 11 prohibits, and has been since the
signature.

**E4. The contract's own authority has physically decayed.** Both of its evidence paths are dead
(D1). A rule whose evidence cannot be resolved is a rule a reader cannot check. `[2026] VJS-CC-VJS
17` and the corpus generally treat a citation that resolves to nothing as a defect in the citing
instrument. If the frames must yield to `UNIFORMITY-CONTRACT.md:82`, they must yield to a line
whose two supporting citations both point at nothing.

**E5. There is no accessibility bar to the frame's colour, and it has been measured.**
`LOG-2026-08-01-134100.yaml` records `--accent` against `--bg-primary` on every shipped theme:
light 4.62, dark 7.92, ocean 10.46, ember 8.58, neon 9.22, against a 3.00 requirement. Its stated
reasoning is directly on point: "the ruling was being read as 'no ring', when what it says is 'no
ring below 3:1'. The wider reading is the kind of over-application that makes a court ruling look
like it costs a feature it never asked for." The same argument answers any suggestion that
CC-OPBOX 2 forbids accent fills: CC-OPBOX 2's ratio is about a control BOUNDARY reaching 3:1, and
an accent fill clears it comfortably.

**E6. On the practical merits, the frame's direction is cheaper.** The declaration sites are three
CSS rule blocks (section F). Changing them once repaints the estate. Redrawing the frames means
touching up to 156 signed drawings by hand in Figma, 64 of which have not even been reviewed yet.

---

## F. The case for the ENACTED CONTRACT, at full strength

**F1. It is estate-wide, it is evidenced by file and line, and it is load-bearing in shipped
source.** The rule is not floating prose. It is implemented and it is guarded: `.onyx-btn--primary`
at `app/globals.css:4894-4897` and `.onyx-btn--danger` at `:4905-4908` both resolve to
`background: var(--ink)`, deliberately identical, and `src/components/ui/button.tsx:88-124` maps
`primary`, `default`, `accent` and `destructive` all onto those two rules with an explanation that
this "is the intended end state, not a gap". A third, independent restatement sits at
`app/globals.css:2240-2244` with its own reasoning: "Auth + modal primary CTAs use --ink
(black/dark ink) per canon, not --accent. Accent blue is reserved for active links / focus rings /
accent-tinted chips - making auth CTAs blue too washed out the contrast hierarchy and competed with
the brand dot in the wordmark." Five source files cite `UNIFORMITY-CONTRACT` in seven places. This
is a rule the estate is built on, not a rule it merely records.

**F2. `--accent` carries a RESERVED meaning, and spending it on resting fills destroys the
reservation.** The line reserves `--accent` for "active/selection emphasis and focus rings". That
reservation is doing work today: measured across `app/` and `src/`, 99 occurrences in 74 files
paint a focus ring or outline with `var(--accent)` (`ring-[var(--accent)]` or
`outline-[var(--accent)]`), and 688 occurrences reference `var(--accent)` overall. If every primary
button becomes an accent fill at rest, then an accent focus ring around an accent-filled button is
a ring the user cannot see, and the selection emphasis that currently distinguishes a selected
facet chip from an unselected one loses its distinguishing property because every button nearby is
already wearing the same colour. That is not a taste objection; it is the reserved-token argument
the line itself makes.

**F3. It may collide with [2026] VJS-CC-OPBOX 2, and that collision is unmeasured.** CC-OPBOX 2's
ratio binds: where an internal instrument's token values are "the sole determinant of whether a
shipped control has a perceivable boundary, the instrument inherits the obligation derivatively",
and the remedy "must raise what the criterion governs, being the boundary of an operable control
and its state". `LOG-2026-08-01-134100.yaml` measured `--accent` against `--bg-primary` only. It
did NOT measure an accent FILL against an accent FOCUS RING, or against `--border-control`, or the
delta between a selected accent-filled control and an unselected one. Its own `residual_exposure`
concedes "A CUSTOM theme can set --accent to any value, including one below 3:1 against its own
--bg-primary." Section 1.4.11 governs state changes as much as resting boundaries, and the
frame-wins outcome creates state pairs that were never measured. CC-OPBOX 2's forbidden list
already includes `claim_conformance_from_the_figma_token_change_alone_while_the_shipped_control_measures_below_3_1`.

**F4. The frames cannot all be obeyed, so "the frame wins" is not a rule that decides anything.**
On the record the signed frames draw blue fills (`/portals/[portalId]/tasks`), black fills
(`/matters` delta 6, and about 10 routes in `PARITY-WORKLIST-2026-08-02.md:180`), and pink
destructive fills (same line). `facet-strip.tsx:318-320` says in terms: "Those two frame readings
genuinely disagree with each other, and one component cannot satisfy both." A ruling that the frame
governs colour therefore hands one shared primitive contradictory instructions on different routes,
and the only way to obey it is to fork the primitive per route, which is the fragmentation the
estate's own steering forbids. The enacted contract, whatever else is said of it, is at least
CONSISTENT.

**F5. Flipping it repaints every primary control at once, in a single step, with no per-route
review.** See the blast radius in section G. The change is three CSS rules. The consequence is 132
routes. There is no staged path and no per-route sign-off in between, which is precisely the class
of change SC-OPBOX 1 order 23 built the registry to prevent from happening by flag day.

**F6. The signature is qualified "with variation", and a colour is the paradigm variation.** The
Principal's own recorded words sign the frames "with variation, meaning, signed off" and the
recorded effect is expressly "It is not a claim that the shipped screen is pixel-identical to the
drawing". On this reading a hue is exactly the kind of thing the qualification absorbs, and the
frames bind for structure, composition and content, which is what every one of the 92 review
records actually charges, while the estate's colour vocabulary stays where the contract puts it.
The `/portals/[portalId]/tasks` reviewer reached the same instinct unprompted, recording the colour
"as an observation only".

**F7. A defect the frame-wins outcome would enshrine.** `src/components/ui/button.tsx:99-101` warns
that a proposal to repaint the destructive variant red "has already been raised and refuted once,
on the strength of the previous wording of this comment". If the frames govern colour, the pink
destructive fills the frames draw become mandatory, and the deliberate design decision that
destructive semantics travel by copy, icon and a type-to-confirm dialog
(`UNIFORMITY-CONTRACT.md` section 7) is reversed as a side effect of a colour ruling, with no one
having argued the safety question.

---

## G. Blast radius, measured, with the method stated

All counts taken 2026-08-02 at commit `4042eaa2`. Script preserved at
`/var/tmp/claude/claude-1011/-home-jellytot/6f4d3ba8-d8ea-404d-8502-6afe614b1092/scratchpad/blast2.py`.

### G1. If the FRAME prevails (control fills follow the drawing)

**Declaration sites that change: 3.** `.onyx-btn--primary` (`app/globals.css:4894-4897`),
`.onyx-btn--danger` (`:4905-4908`), `.minimal-button-primary` (`:2240-2255`). Counted by reading
the cascade, not by grep.

**Routes that repaint: 132 of 199.** METHOD: enumerated every App Router page by calling
`app_route_pages()` in `scripts/visual-reading-staleness.py` (199 pages, `api` and `node_modules`
pruned, route groups stripped). For each page computed the reachable file set with
`reachable_files()` from `scripts/plan-conformance.py`, the same walker the D27/D29 gates use, so
this count cannot disagree with the gates about what a route's source is. Subtracted the union of
every enclosing `layout.tsx` closure, so a route is counted only if IT renders such a control and
not merely because the shared shell does. Matched the remaining `.ts`/`.tsx` files against
`onyx-btn--primary | onyx-btn--danger | minimal-button-primary | variant="primary" |
variant="destructive" | variant="accent" | variant="default"`, which is the exact set of source
tokens that resolve to those three rules.

**If the shared shell is counted, the answer is 199 of 199**, because the app shell itself carries
one. Both numbers are given because the difference is the whole question of whether this is a
per-route repaint or an estate-wide one. It is estate-wide.

**Route-owned source files: 205.** Same walk, distinct files in the 132 routes' own closures.

**Whole-tree call sites: 411 occurrences in 250 files** across `app/` and `src/` (excluding
`node_modules` and dot-directories, so the 23 stale worktrees are not double counted).

**Ceiling on what a frame could lawfully govern: 156 of 199 routes.** Only 156 routes have a
`kind: "frame"` registry row (`frame-registry.json` `entries`, and `_atParityCeiling` agrees). The
other 43 are `no_authority` under SC-OPBOX 1 order 23 and cannot be repainted on frame authority at
all. So a frame-wins ruling repaints 132 routes but can only justify itself on 156, and the two
sets are not the same set: any route in the 132 that is not in the 156 repaints on a rule with no
frame behind it.

### G2. If the CONTRACT prevails (frames redraw to ink)

**Signed frames potentially requiring redraw: up to 156.** Every `kind: "frame"` row.

**Frames MEASURED as carrying a colour-on-control conflict: 60 routes, 83 deltas.** METHOD: read
all 92 records in `internal-docs/design/visual-reviews.json`, all of which carry
`verdict: "deviate"`. Classified a delta as colour-on-control when its text names a colour
(`blue|accent|indigo|amber|red|green|purple|teal|black|near-black|white`) AND a fill treatment
(`fill|filled|tint|tinted|pill|chip|button|badge|ring|outline|border|background|bg`) AND a control
(`button|cta|chip|pill|tab|badge|toggle|link|control|primary|selected|active|focus`). Result: 83
deltas over 60 of the 92 reviewed routes. Narrowing to deltas that name `blue`, `accent` or
`indigo`: 54 deltas over 43 routes.

**This is a REPORTED count, not the true count, and the gap is 64 frames.** 156 frames are signed;
92 routes have a review record. 64 signed frames have never been held against their shipped screen,
so their colour state is unmeasured in either direction. (The coincidence with the referral's
unfound "64" is noted and is a coincidence: 156 minus 92 is 64, and that arithmetic is the only
place in the corpus the filer can produce a 64.)

**The reverse-direction repaint, if the `/matters` frame governs: about 14 routes named, 77 files
reachable.** `internal-docs/design/VISUAL-CENSUS-2026-08-01.md:26` gives the facet-chip row as
"approx 14 routes: dashboards 5, entities 4, automations 3, finance 2". `grep -rln
"FacetStrip\|FacetChips" app src --include=*.tsx --include=*.ts` returns 77 files. Under this limb
`LOG-2026-08-01-134100` is reversed and `src/components/ui/facet-strip.tsx:257-260` goes back to an
ink fill, which the log's own verification says is asserted ABSENT in the test suite, so the revert
breaks tests deliberately built to stop it.

**The deferred idiom, untouched by either limb so far: 89 files** use `bg-[var(--ink)]` in `app/`
and `src/`. `LOG-2026-08-01-134100.yaml` `scope` expressly leaves SegmentedControl
(`src/components/ui/segmented-control.tsx:127`), ObjectViewTabs
(`src/components/ui/object-view-tabs.tsx:186`), the settings sub-page nav and WorkflowRunsView on
the ink-fill selection idiom. A ruling on `--accent` for selection reaches all of them, whichever
way it goes.

---

## H. What is squarely before the court, and what is not

BEFORE THE COURT:
1. Where a signed, registered frame and the enacted `UNIFORMITY-CONTRACT.md:82` disagree about the
   COLOUR OF A CONTROL FILL, which instrument governs, and on what reasoning that survives the
   collision running in both directions.
2. Whether "signed with variation" reaches a value substitution on an element the frame draws, when
   the direction expressly resolves only MORE and LESS.
3. Whether `LOG-2026-08-01-134100` was lawful as a decision log, given the fork it disposed of is
   now said to be first-impression, and whether it stands, falls, or stands pendente lite.
4. Whether the frames' mutual disagreement (blue, black and pink for the same control class) is a
   defect in the frames curable only by redraw under order 11 and order 25, and if so what governs
   the estate meanwhile.

NOT BEFORE THE COURT, and not to be decided in passing:
- the contrast floor itself, settled by [2026] VJS-CC-OPBOX 2 and not challenged here;
- whether the 43 routes with no registry row should get frames;
- the 56 parked frames and the 2026-09-15 fence (SC-OPBOX 1 order 27);
- whether `UNIFORMITY-CONTRACT.md` should be re-evidenced, which is owed work either way and does
  not depend on the outcome.

---

## I. Filer's declaration

Nothing in the opbox tree was edited in the preparation of this file. No colour was changed, no
frame was registered, no log was written to dispose of the fork. The one measurement script written
lives in the scratchpad, outside both repos. The filer holds a view and it is deliberately not
recorded here, because ACT-002 and the route kernel's
`file_symmetric_case_file_no_preference` require the bench to decide on a file that does not
disclose it.
