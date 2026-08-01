# [2026] VJS-CC-OPBOX 2 - opinion of Hairline CCJ

County Court, sitting at first instance. Bench of one. Vote 1-0.
Issue: `<subscriber>_design_control_boundary_contrast`.
Case file: `SUBMISSION-2026-07-25-080957`,
digest `sha256:b567b056d2c8a1af712788b03a5fd952fd3008873f62e622b34c1a5e55d9dd07`.
Convened `CONVENING-county-2026-07-25-081746`.

Recorded under ACT-002:s7 (orders bind, opinions explain) and within the 2,000 word
opinion limit at ACT-002:s2.

---

## Q1. Does SC 1.4.11 bind?

WCAG 2.2 SC 1.4.11 is not itself law; it is a technical standard that becomes legally
operative through instruments that adopt it, and none of those instruments attaches to a
design file. ACT-001:s3 places real-world law at the head of the hierarchy and ACT-001:s6
states that it prevails, but ACT-COMPUTER-FIRST-REALM:s10 describes that limb as a
real-world-law warning boundary, and ACT-CONSOLIDATION-FRAMEWORK:s21 entrenches it as a
non-derogable floor. A floor and a warning boundary constrain what we may ship; they do
not turn every internal artefact into a regulated object.

The case for the file being outside is therefore genuinely strong, and it is the argument
I found hardest to answer: nobody has been served a pixel from `BLUFvQVcE127jvXLKowY9y`,
and a court that legislates about a Figma file is legislating about a preference.

What defeats it is the record rather than the theory. I opened the shipped application.
`src/components/ui/checkbox.tsx:16` and `src/components/ui/input.tsx:14` paint the boundary
of an unchecked checkbox and of a text input with `var(--border)`, which
`app/globals.css:62` sets to `#cbd5e1`. That measures 1.47:1 against `#ffffff`. The dark
theme sets `--border: #2a2a2a` against `--bg-primary: #1a1a1a`, which measures 1.21:1. So
the criterion already bites, on real controls, in the product that real users are already
served.

The design file is not an alternative universe: this repository's own house rules make the
Geist Edition file the declared migration target for that shipped code, so its `border`
value is the upstream determinant of the same boundary. On those facts the obligation
reaches the token derivatively, and only to the width of the criterion. It does not reach
the file because it is a design file; it reaches it because these particular token values
decide whether a shipped control has a perceivable boundary.

## Q2. Which remedy?

I adopt option B, narrowed and conditioned.

The strongest case against B is option A's: B has a silent failure mode, a boundary
misclassified as decorative shows green in the audit while the control fails, and A cannot
be got wrong at a call site. My own checking made that objection worse, not better, and I
say so plainly. The filed counts do not reproduce. I measure 1,475 direct `border` paints,
2,162 `hair(...,border)` calls and 183 `hl*(...,border)` calls, roughly twice the corpus
the file describes, and the proxy the file uses to call 1,276 sites decorative is a generic
one-side stroke helper, `hair(n,side,c)`, which also draws tab-strip underlines and grid
header rules. The 623-versus-1276 split is therefore itself an unverified classification,
which is precisely the vice charged against B.

But that finding cuts both ways and in the end cuts against A harder. A's blast radius is
doubled by the same measurement, so A would repaint on the order of two thousand three
hundred boundaries that SC 1.4.11 expressly exempts, at the density on which this system's
whole visual thesis rests, in order to satisfy a criterion that does not ask for them.
Worse, A destroys the information: once every line is 3:1 nobody can ever tell which line
was load-bearing, so the system becomes permanently unauditable against the criterion it
was changed to satisfy. The classification work is not avoided by A, it is concealed by A.
B forces it into the open, and I condition B so that a misclassification is a visible
ledger row rather than silence.

Option C fails on its own facts. Its premise, that the shipped app has a different
`--border`, is true and unhelpful: the shipped value is 1.47:1, itself under half the
floor, on the same two controls. Its remedy, control fill, has no answer for the checkbox
stroke, and the audit's own measurement of the plane delta at 1.04:1 shows why fill cannot
carry it. A recorded non-conformance is a defensible instrument for a legacy defect you
inherited; it is not a defensible instrument for a defect you are about to author into
every screen derived from the system.

I record that the consolidation-over-fragmentation steering does not bar B. It is at
highest a local decision log, which ACT-001:s3 ranks below the real-world-law limb and
below a County Court order, and ACT-COMPUTER-FIRST-REALM:s10 prohibits a local log
contradicting higher law. On the merits it does not even apply: `craft-retrofit.js:68`,
`:69` and `:108` already define `border` and `borderStrong` as a two-token boundary
vocabulary and already instruct call sites to move from one to the other when a rule needs
weight. B assigns a defined role to an existing token; it does not stand up a second
system.

## Q3. The Figma and production divergence

The conformance limb is in scope, because it would be incoherent to hold that the
criterion attaches to the shipped pixel and then confine the remedy to the file. The
authority limb, which of the two artefacts is the source of truth for `--border`, is not
necessary to dispose of this reference and is a separate matter.

## Q4. Dark mode

In scope only as a floor. The dark `border` value is unresolved and unmeasurable on this
record, and I will not fix a number I cannot check, so I bind the rule and reserve the
value. The `surface` and `bg` inversion is a different defect, a system contradicting its
own stated elevation rule, and it engages the criterion only where a plane edge is the
sole boundary of an operable control. It is a separate reference, and I direct that it be
filed rather than lost.

## Q5. Appeal

Appealable, and permission to appeal is granted. The Q1 boundary question, whether
real-world law reaches internal instruments generally, is a jurisdiction and boundary
question of the kind ACT-002:s3 routes to the Privy Council. I have decided it only on
these facts and for this token, and a higher forum may wish to settle the general rule.

## The ratio

Real-world law under ACT-001:s3 attaches to the artefact a user is actually served and not
to an internal design instrument as such, but where an internal instrument's token values
are the sole determinant of whether a shipped control has a perceivable boundary, the
instrument inherits the obligation derivatively and to the exact width of the criterion,
so the remedy must raise what the criterion governs, being the boundary of an operable
control and its state, and must not repaint what the criterion exempts.

---

## Findings on the case file

The filing agent withheld its own view, as required, and the file was symmetric. It was
not, however, accurate. I record the following so the record is not built on it.

1. **The corpus counts do not reproduce.** Against `internal-docs/design/build-scripts/` I
   measure 1,475 `bp(border)`/`bindPaint(border)`, 2,162 `hair(...,border)`, 183
   `hl*(...,border)`, 45 `bp(borderStrong)` and 114 `hair(...,borderStrong)`. The filed
   623, 1,276 and 28 are not reachable by any counting method I tried. Both A's blast
   radius and B's burden are roughly double what was stated; B's 651 sites is closer to
   1,216 for direct paints alone.
2. **Calling the `hair()` calls decorative is an assumption, not a measurement.**
   `hair(n,side,c)` is a generic one-side stroke helper also used for tab-strip underlines
   and grid header rules.
3. **"Production `--border` is `#cbd5e1` across five themes" is wrong.** Five themes carry
   five different values: light `#cbd5e1` (L62), dark `#2a2a2a` (L285), ocean `#1c2a3a`
   (L371), ember `#2d1f0a` (L442), neon `#1b3350` (L513). The 1.47:1 figure for `#cbd5e1`
   on `#ffffff` is itself correct; I compute 1.4847.
4. **"Dark `border` has no value anywhere" is true only of the Figma token set.**
   Production defines dark `--border: #2a2a2a`, measuring 1.21:1 against `#1a1a1a`. That
   is an additional confirmed failure the case file did not surface.
5. **"The unchecked checkbox has no fill" is overstated.** `entities-v2-a.js:57` sets
   `b.fills=[bindPaint(surface)]` then the stroke. It has a fill; the fill supplies no
   boundary delta, `surface` on `bg` being 1.04:1. The substance holds, the wording does
   not.
6. **A trap not flagged:** production `--border-strong` `#94a3b8` measures 2.56:1 on
   `#ffffff` and cannot serve as the production analogue without a value change.
7. **An overbreadth carried from the audit:** F1 sweeps in "every panel edge" and "every
   `hair()` call". SC 1.4.11 governs information required to identify components and their
   states and expressly exempts pure decoration.
8. **Verified and correct as stated:** `rgba(0,0,0,.08)` composites to 1.20:1 on both light
   planes; `rgba(0,0,0,.42)` gives 3.04 and 3.02; `#919191` gives 3.15 and 3.02; `#949494`
   gives 2.91 on `surface` and must not be used. The 1px house rule at
   `craft-retrofit.js:108` is as described. The first-impression claim holds: nothing on
   accessibility, contrast or design tokens exists in `lawpack/v2/orders`,
   `lawpack/v2/judgments`, `.vjs/orders` or `.vjs/court/orders`.
