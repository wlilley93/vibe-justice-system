# Vibe Justice System - design notes

Running capture of every decision and direction. Append-only. The README and code are built from this.

## Positioning (the headline)
The **Vibe Justice System (VJS)** exists to make **bounded, rule-based decisions on ANYTHING**: any decision that
should be principled, recorded, and binding. **Lexby** is your lawyer inside it. **Coding is just the first
application**, not the definition. The README leads with this general value and uses coding as the flagship example
(with the scratch-to-signals textbook, a non-code project with real rulings on disk, as living proof of "beyond
code"). The community element is a headline, not a footnote: shareable ruling cards, a landmark-cases gallery,
bring-your-own-bench rosters, and statute packs.

## Naming stack (canonical)
- **Vibe Justice System (VJS)** = the product / the repo.
- **Lexby** = your lawyer inside it (advocate + advisor + engineer).
- **SPEC-LAW** = the statute (sovereign rules). **Case law** = the logged rulings (precedent).
- **Caselaw Driven Development (CDD)** = the methodology (sits beside TDD/BDD/DDD). `cdd` is the init command.
- **Vibe Procedure Rules (VPR)** = how matters move through the courts (the Civil-Procedure-Rules analogue; see
  `VPR.md`).
- **The courts:** First Instance (1 judge) -> Court of Appeal (3) -> Supreme Council (5, or 9 for constitutional).

## What Lexby is
If spec is law, any project of decisions needs a justice system. Lexby is that: a lawyer who records every
decision, knows what is allowed, and takes contested calls to a court instead of guessing. (Coding is the first
application; the same machine governs product, research, or a literal legal case.) He is THREE things at once, and
that is the point:
- **Advocate** for your ideas: builds the strongest case for what you want and puts it to the court so it wins on
  merit, not on who spoke loudest.
- **Advisor**: gives it to you straight (good idea, bad idea, the risk, the precedent). Counsel, not a yes-man.
- **Engineer**: does not just opine, he ships the code, then records why.
This combination empowers him to both advise AND build, so the decision and the code that follows it come from
one brain, not two.

## Name
**Lexby** (the lawyer / the product). Chosen after checking: "Lexly" was taken (funded legaltech co "Lexly AI");
"Speccy" taken (Piriform); "Lexford" taken (Lexford AI). Lexby is free on npm + PyPI, no product clash (the
similar "Lexbe" is a different spelling, eDiscovery). GitHub `/lexby` handle is taken (use an org / `@lexby`
scope; non-blocking). Keeps the "Lex" (law) root.

## The README (how to write it)
- **Value first.** Lead with the value and the feeling, not what it does or how it does it. Mechanics come later.
- Do NOT get lawyerly or technical up front. The repo handles all the machinery for the user.
- **Beautiful and clear**, written for a degen / crypto-bro audience: confident, punchy, memetic where it lands,
  but the substance is real and the code stays strong. Use the best practices of high-signal degen-AI READMEs
  (bold hook, badges, short lines, whitespace, a manifesto tone, a WAGMI-style close).
- Include a **"Things you can say to Lexby"** section, natural language, e.g.:
  - "I think we should go in this direction. Submit it to the Court."
  - "I don't agree with the outcome. Can we appeal?"
  - "The Court of Appeal was largely split. I think this needs to go to the Supreme Council."
  - "What did we decide about X, and why?"  /  "Is this allowed under our spec?"
- Note the **judges-speak-in-tongues, Lexby-translates** idea (see below) as a feature.

## Conceptual model (authoritative)
Keep these four layers distinct. Earlier drafts wrongly equated the court with the methodology; they are different
layers.
1. **The justice system** = the court (the bench + the three tiers) PLUS SPEC-LAW. It is the institution.
2. **SPEC-LAW = statute law.** The written, enacted rules of the project (the spec). Part of the justice system.
   Lexby implements and enforces it AT THE LOG LEVEL as he works.
3. **Lexby = the lawyer**, an officer of the court. He acts in the repo, advocates, advises, engineers.
4. **Case law = the artefacts Lexby creates.** Every logged decision is a ruling, a precedent. The accreting body
   of those rulings is the project's case law.
5. **Caselaw Driven Development (CDD) = the methodology.** Every move cites the statute (SPEC-LAW) AND the prior
   case law, decides, then logs the new precedent that binds what comes next. (Sits beside TDD / BDD / DDD.) Real
   common-law systems have exactly two sources of law: statute (SPEC-LAW) and case law (Lexby's logged rulings).
   `caselaw` is free on npm + PyPI + GitHub; the acronym CDD overlaps loosely with Component/Context-Driven
   Development, but the full phrase is distinctive.

## The constitution (unwritten but clear)
The constitution is **uncodified**: there is no single entrenched `CHARTER.md`. It lives, like the UK's, across
four sources - **statute** (SPEC-LAW), **case law** (rulings), **conventions** (settled durable defaults), and
**works of authority** (a non-binding plain-English commentary that describes it). It is **clear by access, not by
codification**: this is a citation system, so "what does the constitution say about X?" is answered by the
citator/digest returning the governing statute + leading case, which Lexby translates on demand. Clarity comes from
**retrieval + a narrator**, not from one rigid document.
- Delivered by: a generated, always-current **constitutional digest** (from the binding sources, so it cannot go
  stale) + a **commentary** (the work of authority, descriptive, non-binding).
- **Constitutional statutes (the Thoburn rule):** foundational articles (the rule of law, statute supremacy, the
  principal's dual capacity, the court structure) cannot be *impliedly* repealed - only by **express, deliberate**
  amendment. Nothing is entrenched (parliamentary sovereignty is intact), but the foundations cannot drift or be
  silently overwritten.
- Edge-drift is handled by the existing coherence machinery: per incuriam, intra-tier-split -> refer up, and the
  STATUS lifecycle (is this still good law).

## The principal and the rule of law
The principal (you) holds **two offices at once**:
- **Sovereign / Parliament** - the source of authority and the legislature. By **parliamentary sovereignty** you may
  make or unmake any law (amend SPEC-LAW, move a precedent). You are never permanently blocked.
- **Prime Minister** - the executive who governs day to day. When you *act*, you act **within the existing law**.

The hinge is **changing the law vs acting under it**. As Parliament you may change anything, by **due process**. As
Prime Minister your demands must be **lawful**; a demand contrary to enacted SPEC-LAW or binding precedent is ultra
vires, and Lexby - an officer of the court whose first duty is to the law - **must push back, not obey**. The
pushback always carries the lawful route, so you are never stuck: *"That is unlawful under S-n / [cite]. I can't
simply do it. But you are sovereign: amend the statute (here is the amendment) or refer it to the court to move the
precedent, then it is lawful and I will execute it."* This rule-of-law check is what gives SPEC-LAW teeth and is why
Lexby is not a yes-man. Founding precedent: the prorogation case (an executive act held unlawful), homaged in
Justice Hallam.

## The court and the bench
- **Spec is law.** SPEC-LAW is sovereign statute the work must obey; the court makes case law applying it.
- **Tiers (UK-faithful, token-lean) - SPEC-LAW-10:**
  - **First Instance** = a SINGLE judge. The largest token economy; most matters end here.
  - **Court of Appeal** = a panel of 3, reached only by permission to appeal.
  - **Supreme Council** = a panel of 5, expandable to the full 9 only for constitutional / foundational matters.
  - Abolished: the "Council" first-instance label, any multi-judge first instance, the fixed default-9 bench, and
    the old 10-roster / odd-5-panel.
- **Gates (SPEC-LAW-11):** standing at intake; permission to appeal between tiers; the precedent fast-path (a point
  on all fours with binding ratio is disposed of on citation, no sitting).
- **The judges are PERMANENT SEATS, seeded at runtime with EPHEMERAL STANCES** (a durable lens, a fresh position
  per case) - but freshness governs only matters of FIRST IMPRESSION; once a ratio is settled it is FOLLOWED, not
  re-polled (the freshness vs stare-decisis rule).
- **They speak as tenured judges**, in dense, precise, near-impenetrable legalese. That is the rigor layer.
  **Lexby translates** to plain English by default; the full opinions are behind `--verbose`.
- **Names are invented, adjacent to the giants of the English bench, and never land on a living or sitting jurist**
  (credits line owns the homage: "names are inventions adjacent to the giants of the English bench").

### The benches (invented names, each a durable lens)
**Puisne pool** - First Instance draws 1, Court of Appeal draws 3:
- Dennan (plain meaning) · Dipley (structure / doctrine) · Bingmore (rule of law) · Athorne (duty to others) ·
  Scarmont (reform / fairness) · Mansby (commercial pragmatism) · Wilbery (purposive intent) · Hollerton (close
  textual reasoning) · Devlan (moral limits) · Radmoor (precedent / restraint)

**Supreme Council** - sits 5; the full 9 for constitutional matters:
- Hallam (presiding; wears a different brooch to every sitting, an homage to Lady Hale's spider brooch) ·
  Sumberly (intellectual breadth) · Elden (caution / precedent) · Goffe (equity / restitution) · Blackmere (first
  principles) · Coade (ancient liberties) · Steyne (rights / purposive) · Bowan (procedure / fairness) ·
  Aldermere (synthesis / consensus)

> Name corrections applied (a name must not land on a living or sitting jurist): Neuberg -> Aldermere (Lord
> Neuberger); Reade -> Elden (Lord Reed, the *sitting* President); Sumner -> Sumberly (Viscount Sumner + the
> living Lord Sumption); Hollman -> Hollerton (the living Lord Hoffmann).

## Two ways into the court (invocation methods)
The court has two front doors. Both are cases: both go to first instance, can be appealed, can reach the Supreme
bench, and both produce CASE LAW (precedent).
1. **Request for Ruling** (forward-looking): a question of law before acting. "I believe we should go in this
   direction. Submit it to the Court." Declaratory; asks for a decision/permission on a fork.
2. **Breach** (backward-looking): a self-indictment or a charge in **negligence** that the duty of care was not met
   (see the tort doctrine below). "I only partially applied the design primitives." The court tests duty / standard
   / breach on the merits and orders **remediation** (never punishment), and logs the precedent. There is no
   jurisdiction gate: a duty always exists, so the matter is justiciable from the first act.

**Self-submission is a first-class feature.** When an agent admits it deviated ("partially", "I think I missed X"),
that admission AUTO-FILES a breach instead of evaporating in a chat log. Same philosophy as "convene when unsure",
extended to "confess when you deviated." Lexby's three hats close the loop: the advocate files it, the court rules,
the engineer executes the remedy.

## Statute vs case law: what a ruling becomes, and where it lives
- A ruling (including a breach judgment) is the court APPLYING statute to facts. It becomes **CASE LAW**
  (precedent), NOT new statute. Example precedent: "partial application of a design primitive is non-compliance;
  remedy is full retrofit before acceptance."
- It becomes **SPEC-LAW (statute)** only by a separate, deliberate AMENDMENT (legislation, not adjudication). A
  breach that exposes an ambiguous or unenforced primitive prompts the court to RECOMMEND an amendment (e.g. add a
  conformance gate); codifying it is the legislative step. Case law reveals where statute must harden; repeated
  breaches of the same rule are the signal to legislate.
- **Storage (two cross-linked stores):**
  - `SPEC-LAW.md` = the statute. Enacted articles, append-only, changed only by amendment.
  - .justice/judgments/ = the rulings ledger. One file per case (`caselaw/0001-<slug>.md`) + a `DOCKET.md` index. Each
    entry records: facts, request-or-breach, the statute article cited, ratio (binding reason) vs obiter (asides),
    remedy, deciding tier, status (binding / appealed / overruled). A caselaw entry cites the SPEC-LAW article it
    applied; a SPEC-LAW amendment cites the caselaw that prompted it.

## Automation (the plugin)
- Give Lexby a **goal** and he runs it. When he hits a fork he genuinely cannot call, he does not guess and does
  not stop to ask: he **convenes the court himself** (a dynamic workflow), gets a ruling, logs it, keeps moving.
- Ships as a Claude Code **subagent** (`@agent-lexby`) plus an installer that **writes a rule into the project's
  CLAUDE.md** so any uncertain decision auto-routes to Lexby's court. Zero babysitting.

## Beyond code
Lexby works anywhere **Caselaw Driven Development (CDD)** is practised, anywhere decisions are recorded as
precedent as you go: not just code, but product strategy, research direction, even a literal legal case. If the
repo builds case law, Lexby can argue it, rule on it, and keep the record straight.

## The ribbon (final-pass synthesis, LOCKED)

**Reframe (the unlock): Lexby is a CITATION SYSTEM with a court attached, not a court.** The court is the
expensive, rare part. The daily value is that every decision becomes findable, citable, and binding. Most forks
resolve against an on-point precedent with NO sitting at all (the fast path), which is what makes CDD *faster*
than guessing, not slower. The court only convenes for genuine first-impression forks, distinctions, overrulings,
statute conflicts, or breaches.

**The atomic unit: the ruling artefact.** One file per case in .justice/judgments/: a YAML head (id, neutral citation
`[YEAR] LEXBY n` first instance / `LEXBY-CA` Court of Appeal / `LEXBY-SC` Supreme, date, panel, `kind:
request_for_ruling | breach`, question/charge, `endeavours_standard` (reasonable | all-reasonable | best, for
breaches), one-line RATIO, OBITER, SCOPE globs, CITES statute+precedent, DISTINGUISHED/OVERRULED back-refs,
PER_INCURIAM flag, REMEDY (breaches only), STATUS) over a human body. .justice/INDEX.md (the citator) is
regenerated on every ruling; `lexby cite <id>` resolves it. A subagent's only retrieval is grep/glob/read, so the
compact index is mandatory.

**Procedure and devices (SPEC-LAW-11).** Standing is a threshold filter at intake (non-parties cannot conjure
sittings). Permission to appeal is a hard gate between every tier (no higher bench convenes without leave). The
precedent fast-path disposes of a point governed by binding ratio on citation, with no sitting. Only the RATIO
binds; OBITER is at most persuasive; PER INCURIAM voids a ruling made in ignorance of binding spec or precedent
without a fresh sitting. A **declaration of incompatibility**: where case law cannot be reconciled with SPEC-LAW the
court declares it and refers it up for amendment; it never strikes the sovereign spec.

**Locked rules (adopt as charter law):**
1. **Freshness vs stare decisis.** Ephemeral stances govern ONLY matters of first impression. Once a ratio is
   settled it is FOLLOWED, not re-polled. Fresh stances return only to distinguish or overrule. (Resolves the
   un-lobbyable-bench vs bindingness contradiction.)
2. **Advocate / bench / record separation.** "Lexby advocates; the bench decides; the record binds them both."
   The bench Task receives a SYMMETRIC case file (your position + the counter-position + statute + precedent) with
   no access to Lexby's preference. This is the brand's core integrity mechanism, not a footnote.
3. **Statute supremacy doctrine (hard rule).** SPEC-LAW (statute) is supreme. A ratio conflicting with enacted
   spec is void to the extent of the conflict and auto-flagged superseded-by-statute when spec changes. Case law
   interprets statute where the spec is silent. The court may issue a "declaration of incompatibility" flagging a
   precedent the spec authors should ratify or override. Only the Supreme tier writes new statute.
4. **Default surface = plain English.** Lexby's translation + verdict + new-precedent path is the default; the
   dense judge legalese is behind a "read the full opinions" fold / `--verbose`. Rigor you opt INTO.
5. **Copy discipline.** Kill "enforce at the log level" in value-first copy (jargon). Use "keeps the record",
   "nothing happens off the record." Reserve "log level" for the mechanics section.
6. **Determinism.** Ids are sortable + collision-resistant (`CL-YYYYMMDD-<hash(question)>` for cases, `S-NN` for
   statute) to survive parallel git branches. Panel = deterministic draw of N=5 by hashing `case_seed =
   hash(question + caselaw HEAD)` against the roster, recorded in the header. (Resolves the 3-vs-5 question: sit 5.)
7. **Corpus coherence.** Per-incuriam: a ruling that missed binding precedent/statute is voidable (a single
   higher-tier judge vacates and re-runs against the missed authority, no full sitting). Intra-tier split: two
   same-tier good-law ratios conflicting in overlapping scope must be referred UP, with a lint that fails on an
   unresolved in-scope split.
8. **Own the homage.** A credits line ("names are inventions adjacent to the giants of the English bench"); nudge
   living-jurist echoes one more syllable from source; each judge gets a one-line temperament + a signature
   opening tell so dissents read as distinct jurists, not one model in many hats.

**Tiers as runnable workflows.** `council.md` / `appeals.md` / `supreme.md` invoked via Task with a STRICT
structured emit (verdict, per-judge vote + one-liner, holding, ratio, citations) that Lexby PARSES, never
re-summarises. Cost-tier: Council on a fast/cheap model + short budgets; Appeals/Supreme escalate to the strong
model; record the model id per ruling. Port `scratch-to-signals` council.js/appeals.js as the start.

**The shareable RULING CARD.** Every case auto-renders a boxed terminal-art verdict (citation, panel, VOTE 7-2,
one-line memetic HOLDING, Lexby's TL;DR) to CLI + saved PNG/SVG in `.justice/cards/`. Degens share artefacts, not
tools: the card is the distribution flywheel. "Nine invented law-lords 7-2 striking down 'just use localStorage
bro' and then translating it" is the retweet.

**The two doors, in the artefact + triage.** Both `request_for_ruling` and `breach` are cases producing case law.
The triage governor classifies every fork: on-point good-law precedent -> follow silently (cited in the work
log); reversible/low-blast -> decisive call + lightweight note; pure impl detail -> no court; genuine fork with no
authority -> convene. Self-submission: an agent's confession ("partially", "I deviated") AUTO-FILES a breach with
a REMEDY field. The CLAUDE.md trigger is an enumerable imperative list, NOT "when uncertain".

**`npx lexby init` = the demo.** Scaffold subagent + CLAUDE.md trigger + .justice/judgments/ + a starter SPEC-LAW inferred
from existing CLAUDE.md/README/package.json, then run ONE seeded micro-case live (fork -> Council -> legalese
ruling -> Lexby translates -> committed `.justice/judgments/0001-*.md` + card) in 60 seconds. First run ends on a
screenshot-grade verdict, never an empty dir. Ship `lexby uninstall` clean revert.

**Build order:**
1. Lock vocabulary + seed the **constitutional statutes** in `SPEC-LAW.md` (the canonical global book; s. 1..s. 12,
   see below) and generate the **constitutional digest** + the non-binding **commentary**. NO written `CHARTER.md`:
   the constitution is uncodified but clear by access (the digest + commentary + Lexby's translation).
2. Define the canonical ruling artefact schema + generated .justice/INDEX.md citator.
3. Port the three court workflows into `council.md`/`appeals.md`/`supreme.md` (strict emit, symmetric case file,
   cost-tiering; Lexby parses, never re-summarises).
4. Build the precedent-resolution engine + triage governor + STATUS filtering (the fast path + cost control).
5. Wire `@agent-lexby` (assemble case file -> court Task -> translate -> write artefact -> reindex; enforce the
   advocacy/adjudication separation here). Includes the breach self-submission door.
6. Ship the ruling CARD renderer.
7. Build `npx lexby init` (scaffold + infer starter statute + enumerable CLAUDE.md trigger + live demo case).
8. Rename + repoint `scratch-to-signals` to canonical vocab (lexly/ -> caselaw vocab, LDD -> CDD, council/ ->
   .justice/judgments/, keep "Council" as the first-instance TIER) as the flagship beyond-code proof; link its live
   .justice/judgments/ from the README.
9. Write the README in strict order (hook + install one-liner + ONE card screenshot above the fold; "Things you
   can say to Lexby"; flagship proof link; bench/tiers/CDD lore below the fold) + the `CDD.md` manifesto.
10. Later: marketplace listing, statute-pack templates, landmark-cases gallery, badges/stats, `--template` starters.

**Tagline candidates:** "Spec is law. Lexby is your lawyer." / "Your AI just lawyered up." / "Stop guessing. Start
citing." / "Every decision, on the record. Forever." / "Take it to the bench." / "Your repo now has a Supreme
Court." / "Don't vibe it. Litigate it." / "The seats are permanent. The verdict is yours to appeal."

## Vibe Procedure Rules (VPR) - rule-based progression
Procedure is governed by the **Vibe Procedure Rules** (`VPR.md`), the CPR analogue. The load-bearing rule
(SPEC-LAW-13): **progression is rule-based and there is no leap-frogging.** Every matter commences at First
Instance and climbs the tiers in order, escalating only by permission to appeal. A matter destined to change
SPEC-LAW must be REACHED by progression, not commenced at the Supreme Council, and Lexby may not self-initiate at a
higher tier. The sole exception is the **Principal's express leapfrog certificate** (acting as Sovereign), which
may take a matter straight to the Supreme Council (cf. the UK leapfrog appeal). The tort recast ([2026] LEXBY-SC 1)
proceeded under such a certificate; that is why it sat at Supreme directly, and it is the non-notable exception,
not the rule.

## Commands (the surface)
- **`cdd`** - initialise Caselaw Driven Development in a repo. The methodology name IS the init command. Scaffolds
  `@agent-lexby` + the CLAUDE.md trigger + .justice/judgments/, vendors the global SPEC-LAW into the repo, and runs ONE
  seeded micro-case live so the install is the demo (ends on a committed ruling + a screenshot-grade card).
  (Replaces the earlier `npx lexby init` working name.)
- **`submit-request-to-court "<question>"`** - file a Request for Ruling (forward-looking: a fork, "should we go
  this way?"). Fast path first: if on-point good-law precedent exists it is followed with no sitting.
- **`submit-breach-to-court "<charge>"`** - file a Breach (backward-looking: a charge in negligence that the duty of
  care was not met). The court tests duty / standard / breach on the merits and orders remediation (never
  punishment). No jurisdiction gate (a duty always exists).
- Supporting: `lexby cite <id>` (resolve a ruling), reindex on every ruling, `lexby uninstall` (clean revert).

## Breach doctrine: TORT, not crime (SPEC-LAW-4..8)
Settled by the Supreme Council ([2026] LEXBY-SC 1, Hallam CJ, unanimous): the court does not punish, it makes the
work good, so the criminal frame (nulla poena, the jurisdiction-first gate, the guilt/remedy decoupling, the "no
breach, act anyway" posture) was a category error and is struck. Breach is the **tort of negligence**:
- **Duty (s. 4):** Lexby owes a continuing duty of reasonable skill and care to every principal who relies on his
  work, arising from the relationship itself (the neighbour principle), independent of any enacted SPEC-LAW.
- **Standard (s. 5):** discharged by meeting the applicable rung of a graded hierarchy of endeavours, pleaded and
  found per engagement and stakes: **reasonable skill and care** (default) / **all reasonable endeavours** / **best
  endeavours**. Conduct a responsible body of competent practice would endorse is not breach (the Bolam rule).
- **Breach (s. 5):** a falling-below of the applicable standard, a question of fact on the merits, never a
  punishment trigger.
- **Remedy (s. 6):** remediation and restitution alone, proportionate to the harm: make good, restore the position.
  Punishment, fine, and sanction are unavailable in every instance. Finding and remedy are never decoupled.
- **No-statute case (s. 7):** silence in SPEC-LAW is no defence and does not extinguish the duty; it merely fixes the
  standard at reasonable skill and care. The matter is justiciable from the first act (no auto-referral).
- **First-time / second-time repealed (s. 8):** one continuous standard. A genuinely novel, unforeseeable first
  failure with no governing standard is judged against reasonableness, ordinarily founds no breach, and triggers a
  forward duty to spec the rule and remediate. The logged ruling then makes the hazard foreseeable, so a recurrence
  is breach of a now-known duty (easier to prove, wider to remediate). The consequence is always restorative.

## Where law lives: UNITARY sovereignty (SPEC-LAW-9), not federalism
"Federalism" was a constitutional solecism (it presupposes sovereign sub-states; a unitary realm has none) and is
struck. The frame is **parliamentary sovereignty in a unitary state of plural jurisdictions under one apex court**:
- **SPEC-LAW = sovereign primary legislation:** ONE global statute book, supreme throughout, vendored to every repo
  on git (e.g. `~/.lexby/SPEC-LAW.md` or a central `lexby-statute` repo).
- **Case law = jurisdiction-local precedent:** each repo is a JURISDICTION (the England-and-Wales / Scotland /
  Northern Ireland analogue) applying the one statute and taking notice of its sisters.
- The **Supreme Council alone enacts**, by elevating a local ratio into realm-wide statute. There are no competing
  sovereigns, hence no concurrent-authority reconciliation apparatus to build or to litigate.

## Update mechanism: GitHub is the database (decided 2026-06-05)

SPEC-LAW is vendored into each repo as `SPEC-LAW.md` (git-tracked, offline-first). The canonical global SPEC-LAW
lives at `github.com/wlilley93/vibe-justice-system`. The update mechanism:

- `cdd init` and `cdd run` check the canonical repo's `SPEC-LAW.md` version header (or a `SPEC-LAW-VERSION` tag)
  against the local copy via a lightweight unauthenticated GitHub API call.
- If behind: print a notice. "Global SPEC-LAW s. 14 enacted (2026-07-01). Run `cdd update` to adopt."
- `cdd update` fetches the canonical `SPEC-LAW.md` and writes it to the local repo, creating a commit. Never
  forced, always explicit.

Community features (landmark gallery, statute packs, bench rosters) use GitHub as the registry:
- Statute packs: forks/gists of canonical SPEC-LAW with a `vjs-statute-pack` topic tag.
- Landmark cases: a curated `community/landmarks.md` in the canonical repo, with links to exemplary public
  .justice/judgments/ dirs.
- Bring-your-own-bench: rosters as `.json` files in a `community/benches/` directory, shareable by copy.

No external database, no infrastructure dependency, no auth required for reads. If the canonical repo is down,
the local tool still works (it just cannot check for updates).

## Compliance check (review gate, decided 2026-06-05)

At review time, submitted work must be checked against the full legal corpus. The court workflows already accept
`args.spec` (SPEC-LAW) and `args.caselaw` (INDEX.md entries), and the fast-path screen checks for binding
precedent. The **compliance gate** is the step in `first-instance.js` Phase 1 where the judge:
1. Checks the submission against every relevant SPEC-LAW article (does the work breach any statute?).
2. Checks the submission against all `good-law` entries in .justice/INDEX.md (does it conflict with or ignore
   binding precedent?).
3. Issues a **declaration of incompatibility** (s. 11(f)) if statute and caselaw are in conflict.

This is already baked into the workflow design. The triage governor pre-screens to avoid convening for pure
implementation details, so the compliance check only runs on genuine forks or charges.

## Community commons + Phase 2 vision (decided 2026-06-05)

### The core insight
VJS is building the common law of AI-assisted development. The more projects contribute rulings, the richer the
precedent library, the more matters resolve on the fast path (citation, no sitting, no cost). This is the network
effect of a legal commons.

### Anonymisation (launched with VPR 8)
Community submissions strip project-specific identifiers before submission: repo names, file paths, variable names,
function names, class names. The legal question (in general terms), the ratio, the tier, the law applied, and the
outcome are preserved unchanged. The PR is reviewed by the clerk before merging. This is baked into all three court
workflow "Community PR" phases (VPR 8).

### Subject matter jurisdiction (s. 14 constitutional)
VJS courts have no jurisdiction over personal life questions, recreational preferences, or matters with no genuine
connection to project work. The Standing Officer disposes of out-of-jurisdiction matters without deliberation. This
is the spam gate: the court will not rule on "should I shave my head?" and the clerk will reject any community PR
where the matter is outside s. 14 jurisdiction.

### Phase 2: community website
A public read interface for community rulings. Hosted separately (lexby.ai or similar). Cases are:
- Fully anonymised before being indexed
- Browsable by tier, law applied, outcome
- Searchable (text + eventually semantic)
The GitHub community/caselaw/ directory is the source of truth; the website is a read layer over it. No auth
required to read. Submitting is via the existing PR flow.

### Phase 2: vector search layer
Semantic similarity search over community rulings: "did anyone decide this before?" answered across the full corpus
without exact-keyword match. Architecture: GitHub is the canonical store; a CI job embeds each merged ruling and
writes to a vector index (Postgres pgvector or similar). The client checks the vector index before the fast-path
text grep. Implementation deferred: the GitHub-native text approach works for early adoption; vector indexing is a
Layer 2 feature that does not change the data model.

### Tidy repo structure (decided 2026-06-05)
The canonical VJS repo keeps the community record tidy via year-bucketing:
- `community/caselaw/YYYY/` - all community rulings for that year (one file per ruling)
- `community/benches/` - community bench rosters (JSON files)
- .justice/judgments/ in individual repos - local jurisdiction precedent (small, project-specific)
- .justice/INDEX.md - the citator (one row per ruling; this file stays compact because rulings are separate files)

The INDEX.md file cannot bloat because it is a table of citations only. The individual ruling files live in their
own files. If the local .justice/judgments/ directory grows large over years, older rulings can be archived to
`.justice/archive/YYYY/` without breaking the INDEX (the INDEX row stays; the file moves). This is a maintenance
convention, not a code change.

## Open / later (decisions pending)
- **Auto-convene default:** ship ON (zero-babysitting, risk of token spend + interruption) or OFF with a prompt?
  Triage mitigates spend either way; the default shapes the trust story for the degen audience.
- **Launch scope:** code-first with the textbook as the single beyond-code proof, or ship `--template
  research|legal|product` starters in v1?
- **Overruling retroactivity:** when Supreme overrules, default to reconcile existing code (retroactive) or
  grandfather (prospective), or make it a per-overrule choice?
- **Panel determinism salt:** should the same question deliberate identically across repos/branches, or is
  per-project drift desirable? (Sets whether `case_seed` includes a project salt.)
- **Org handle locked:** GitHub org is `wlilley93/vibe-justice-system`. Install string is `cdd` (command name).
  npm / PyPI package name TBD.
- **Extensibility name** ("Lexby chambers" for practice areas): reserve + sketch now, or footnote until launch?
- Decide final styling of the names if any read too close to a real justice.
- Confirm odd panel size drawn from the 10 (3 vs 5).
- The scratch-to-signals textbook becomes the first real consumer/example of Lexby.
