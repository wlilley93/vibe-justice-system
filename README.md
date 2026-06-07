<div align="center">

<img src="assets/vjs-header.png" alt="Vibe Justice System" width="100%">

*AI governance for your repo. The court is AI. Not legal advice.*

![license](https://img.shields.io/badge/license-MIT-blue?style=flat-square)
![status](https://img.shields.io/badge/status-alpha-orange?style=flat-square)
![vibes](https://img.shields.io/badge/vibes-litigated-purple?style=flat-square)
![community](https://img.shields.io/badge/community-open-green?style=flat-square)

</div>

> **Disclaimers**
> - **Not a real court. Not legal advice.** VJS is an AI governance framework. Rulings are AI outputs, not legal instruments.
> - **Production systems need real engineers.** VJS helps record and structure AI decisions - it does not replace qualified engineering review, security audit, or human sign-off on anything that matters in the real world.
> - **It only refines what you give it.** Rulings are only as good as the spec and context you provide. Garbage in, garbage out. A weak spec produces weak law.

> **Alpha status**
> - **Citation numbering is deterministic.** The next neutral citation is computed from the citator, not guessed: run `cdd next-citation <tier>`. Current realm citations use the `REALM-*` provenance scheme, including `REALM-PC`, `REALM-CA`, `REALM-SC`, and `REALM-SI`.
> - **The CLI is local-first.** The zero-dependency Node CLI lives in [`Executive/cli/`](Executive/cli/) (`cdd` / `vjs`: `init`, `next-citation`, `check-citator`, `lodge-judgment`, `submit-request`, `submit-breach`). Use `node Executive/cli/bin/cdd.js` or `npm link ./Executive/cli`. Registry publishing remains packaging work.
> - **The public repo is system data only.** Personal, operational, and project-private facts do not belong here. The public record holds the law, central judgments, procedure, plugin machinery, and derived registers.

---

**The biggest barrier to success is being able to define correctness. This repo solves it.**

---

Your AI makes decisions every session. Nobody writes them down. Six PRs later, a different session contradicts the first one. Now you have two conventions, zero explanation, and a codebase that has lost the plot.

**VJS gives your AI a justice system.** Decisions become binding precedent. Past rulings are checked before anything new is done. If the AI breaks its own rules, it must self-report and fix it.

---

## The idea

Sometimes people break the rules. So do AI agents. That is not, on its own, a failure: an agent's job is to produce value the way it sees best, not to hold the entire rulebook in its head every single turn. The job of the court is to decide, after the fact, whether the way it worked was lawful, and to make the work good where it was not.

The agent builds. The record judges. Neither has to be perfect for the system to work, because nothing load-bearing is decided silently and nothing wrong is allowed to stand once it is seen.

---

## When the court convenes (and when it does not)

This is the part that keeps VJS cheap. An agent told "convene whenever you are unsure" will convene on every trivial fork and cost a fortune. VJS instead gives the agent **five precise conditions**, and for everything else it cites existing precedent and moves on, no sitting required.

The court convenes only when:

1. **First-impression** - no existing ruling covers the question.
2. **Distinction** - precedent exists but genuinely does not fit these facts.
3. **Overruling** - a ruling is wrong or outdated and should be set aside.
4. **Conflict** - an instruction clashes with enacted law or binding precedent.
5. **Breach** - work fell below the duty of care: self-reported, then fixed.

Everything else is a **citation, not a sitting**: before any bench sits, the citator is searched, and a binding ruling on all fours disposes of the matter instantly.

That loop is **Caselaw Driven Development (CDD)**: a fork produces a ruling; the ruling is committed with a citation; every future session cites it instead of re-deciding. Where TDD records that the code does what you said, CDD records *why* you said it.

> These five conditions are summarised here for onboarding. The canonical, binding text lives in [`Executive/plugin/CLAUDE.md`](Executive/plugin/CLAUDE.md), with the methodology in [`Constitution/CDD.md`](Constitution/CDD.md); this summary points to that source and never replaces it. *(Required form per [2026] REALM-PC 3.)*

---

## Lexby

Your AI counsel. Three things at once:

- **ADVOCATE** - builds the strongest case for your idea and argues hard for it, because he does not decide the outcome
- **ADVISOR** - gives it to you straight; if your idea has a fatal flaw he names it before the judges do
- **ENGINEER** - ships the code, then records why

The separation matters. Lexby argues the case but does not sit on the bench - he cannot tip the outcome and then quietly do the opposite. The court decides independently. Lexby executes. The record is permanent.

---

## How it works

Every time you use AI to build something, it makes silent calls: which approach, which trade-off, which direction. Most never get written down. Then a new session picks a different direction, and now nothing is consistent.

VJS catches those calls and turns them into binding decisions:

1. The AI hits a choice: "build this ourselves or use the library?", "ship now or wait for the audit?"
2. Lexby checks if that type of choice was already decided. If yes: follows the ruling instantly - no deliberation, same answer every time, for the life of the project.
3. If not: an AI court deliberates and issues a ruling. It gets committed to the repo.
4. Every future session inherits it. The AI cannot contradict its own record. If it does, it must self-report and go back to court.

**When the AI gets something wrong, it must report itself and fix it.**

### Brownfield code

If you are installing VJS on an existing codebase, the best practice is to start a fresh repo. Treat the brownfield site as requirements: read it, extract what it does and why, and use that as the input to your spec. Then build green, with VJS governing every decision from day one. Trying to retrofit a justice system onto undocumented history is harder than building clean from known requirements - and the brownfield code already contains the answers you need.

---

## The courts

VJS has one judiciary, with central realm courts and local project courts.

```
PROJECT FIRST INSTANCE     1 AI judge                    Everyday project decisions. Repo local.
COURT OF APPEAL            3 AI judges                   Disputed calls. Central citation: REALM-CA.
SUPREME COURT              5 AI judges (9 for big calls) Foundational. Central citation: REALM-SC.
PRIVY COUNCIL              3 AI judges                   Realm constitutional first instance. Central citation: REALM-PC.
```

Most project work starts at first instance. Escalate by permission. You cannot skip the route unless the law gives you a leapfrog certificate.

Local project rulings live in the project repo under `.justice/`. Central realm rulings live in this repo under [`Judicature/.justice/judgments/`](Judicature/.justice/judgments/) and are indexed in the citator at [`Judicature/.justice/INDEX.md`](Judicature/.justice/INDEX.md). The public VJS repo carries system data only: the law of the judgment is public, while personal and operational facts stay sealed.

Most things never leave first instance: one judge, a ruling, a permanent citation, and a future fast path. Higher courts are for contested calls, overruling, or questions foundational enough that every future VJS project should inherit the answer.

---

## Multiplayer and local sovereignty

Installing or forking VJS starts your repo as a local jurisdiction subscribed to the canonical VJS law. That gives your agents a working constitution, courts, procedure, citation rules, and safety hooks on day one.

It does not make your repo a subject of this GitHub repo forever. Your local Principal can branch, amend the local law, create or remove courts, change ministries, become independent, or join/create another community record. Those changes bind your repo only unless they are accepted back into canonical VJS or into another federation by that community's rules.

Nothing automatically pushes your case law upstream. If you want to contribute a generally useful ruling, you can submit an anonymised PR to the Community Record. If you do not, your rulings stay local.

That is the multiplayer model: shared starting law, local freedom, optional federation.

---

## The system at large

The repo is organised like the thing it describes: a small state for AI governance.

- [`Constitution/`](Constitution/) holds the founding settlement, procedure, and CDD method.
- [`Legislature/`](Legislature/) holds the Acts and statutory instruments: the rules the system can point to instead of improvising.
- [`Judicature/`](Judicature/) holds the courts, judgments, citator, law reports, and ledgers: the memory that stops decisions drifting.
- [`Executive/`](Executive/) holds the machinery: the CLI, hooks, skills, ministries, and operational glue that make the law bite in a real repo.

That structure is the value: the AI gets a constitution, a rulebook, a court record, and enforcement hooks. You get continuity across sessions instead of a string of fresh starts.

---

## Say this to Lexby

```
"I think we should go this way. Submit it to the court."

"I don't agree with the outcome. Can we appeal?"

"What did we decide about X, and why?"

"Is this allowed under our spec?"
```

Lexby also catches himself:

```
"I'm not sure this is right..."         -> self-files for a ruling before proceeding

"I think I broke the rules earlier..."  -> self-reports the breach and orders a fix

"I didn't follow what we decided..."    -> files it, finds the original ruling, corrects course
```

Natural language. No syntax. Lexby handles the filing.

---

## Community

VJS is building shared precedent for AI-assisted work.

When a court rules on something generally useful, that reasoning can become part of the public law record if the local Principal chooses to contribute it. What stays out: repo names where they identify private work, file paths, function names, variable names, personal facts, tokens, hostnames, and operational details. What stays in: the question that was asked, the facts necessary to understand the decision, the ruling itself, and the law applied. You share the reasoning, not the source.

**The more good rulings go in, the faster every project resolves.** Before any court sits, Lexby checks the precedent index first. If someone else already fought this battle and got a ruling, the fast path disposes of the matter on citation with no sitting. The bigger the public record gets, the more questions get answered instantly. It is the network effect of a legal commons: every ruling contributed is free advice to every future project that hits the same fork.

> **Fast path:** Project A ruled: *"always encrypt tokens at rest, even in dev."*
> Six months later, Project B hits the same question.
> Lexby finds the ruling. Done in seconds. No court needed.

> **Supreme Court:** Project C has a hard call - should AI ever modify the database schema directly?
> Five judges deliberate. They rule: no, always generate a migration for human review.
> The local Principal chooses to contribute it, anonymised, to the public record.
> Now every VJS project gets that answer on the fast path. Forever.

---

## Ship it

You built the thing. The AI helped. Nobody knows who decided what.

Now you have a record.
Now you have precedent.
Now you have Lexby.

Ship fast. The court is in session.

---

## Install

From a checkout of this repository:

```bash
node Executive/cli/bin/cdd.js init /path/to/your/repo
```

Or link the CLI locally:

```bash
npm link ./Executive/cli
cd /path/to/your/repo
cdd init
```

`cdd init` vendors the governing materials, scaffolds `.justice/`, appends the binding VJS block to `CLAUDE.md`, installs the token-light turn watchdog, and installs deterministic git gates. The watchdog is inert unless `ANTHROPIC_API_KEY` is set; the hard gates need no model.

For AI-assisted manual installation, give your agent this prompt:

```text
Install VJS into this repo. From github.com/wlilley93/vibe-justice-system, fetch and save:
- Executive/plugin/CLAUDE.md -> append to this repo's CLAUDE.md inside a VJS-marked block
- Constitution/CASE-LAW.md -> CASE-LAW.md
- Constitution/VPR.md -> VPR.md
- Constitution/CDD.md -> CDD.md
- Judicature/.justice/suites/security.md -> .justice/suites/security.md
- Judicature/.justice/suites/refactoring.md -> .justice/suites/refactoring.md
- Executive/plugin/skills/submit-request-to-court/SKILL.md -> .claude/skills/submit-request-to-court/SKILL.md
- Executive/plugin/skills/submit-breach-to-court/SKILL.md -> .claude/skills/submit-breach-to-court/SKILL.md
- Executive/plugin/hooks/vjs-watchdog.sh -> .claude/hooks/vjs-watchdog.sh
- Executive/plugin/hooks/vjs-pre-commit.sh -> .claude/hooks/vjs-pre-commit.sh
- Executive/plugin/hooks/vjs-pre-push.sh -> .claude/hooks/vjs-pre-push.sh
- Executive/plugin/settings.json -> merge its hooks block into .claude/settings.json
Create .justice/ directories: caselaw, judgments, pdfs, suites. Create .justice/INDEX.md as an empty citator.
Symlink .git/hooks/pre-commit -> ../../.claude/hooks/vjs-pre-commit.sh.
Symlink .git/hooks/pre-push -> ../../.claude/hooks/vjs-pre-push.sh.
VJS is now active.
```

For the technical map of this repo, see [`STRUCTURE.md`](STRUCTURE.md). For the public law map, see [`Constitution/docs/CITATION-MAP.md`](Constitution/docs/CITATION-MAP.md). For the CLI reference, see [`Executive/cli/README.md`](Executive/cli/README.md).

---

<div align="center">

*VJS is open source. MIT licensed. Contributions, bench rosters, statute packs, and landmark cases welcome.*

*The court structure and procedure draw from the tradition of common law courts. Names on the bench are inventions - never real sitting or living jurists.*

</div>
