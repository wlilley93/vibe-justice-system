<div align="center">

<pre>
        ___  ___  ___  ___  ___  ___  ___  ___  ___
       /   \/   \/   \/   \/   \/   \/   \/   \/   \
      |  V  |  I  |  B  |  E     J  |  U  |  S  |  T  |
       \___/\___/\___/\___/\___/\___/\___/\___/\___/

              ⚖    THE COURT IS IN SESSION    ⚖

          .-------.       .-------.       .-------.
         |  SPEC   |---->|CASELAW |---->| RULING  |
         |   LAW   |     |  CDD   |     |  CARD   |
          '-------'       '-------'       '-------'
                    [ gavel strikes ]
</pre>

# Vibe Justice System

*Your AI just lawyered up.*

![license](https://img.shields.io/badge/license-MIT-blue?style=flat-square)
![status](https://img.shields.io/badge/status-alpha-orange?style=flat-square)
![vibes](https://img.shields.io/badge/vibes-litigated-purple?style=flat-square)
![community](https://img.shields.io/badge/community-open-green?style=flat-square)

</div>

---

**Your AI makes a hundred silent decisions every session.**
The good ones you never notice.
The bad ones surface three weeks later, everything is on fire, and nobody can say who decided what, or why.

**The Vibe Justice System fixes that.** For any repo. New project, greenfield, brownfield - it does not matter. Wherever AI is helping make decisions, those decisions deserve a record.

You move fast. You keep the paper trail. You have someone in your corner who pushes back when you are about to break your own rules.

Spec is law. Rulings are precedent. Lexby is your lawyer.

---

## The problem

Vibe coding works. Until it doesn't.

A decision gets made at 2am. It feels right. Nobody writes it down.
Six PRs later, a different AI session contradicts it.
Now you have two conventions, zero explanation, and a codebase that has lost the plot.

You don't need a slower process. You need a **justice system**.

---

## Meet Lexby

Lexby is your lawyer. Three things at once:

| Role | What he does |
|---|---|
| **ADVOCATE** | Builds the strongest possible case for your idea and puts it to the court. He argues hard FOR you precisely because he does not decide the outcome. |
| **ADVISOR** | Gives it to you straight. Not a yes-man. Not a hype machine. If your idea has a fatal flaw, he names it before the judges do. |
| **ENGINEER** | Ships the code. Then records why. |

Because Lexby is bound by the record, he can push hard for you without it being personal. The court decides. He executes. The precedent is permanent.

---

## How it works

**Spec is law.** Your project spec becomes SPEC-LAW (statute). The court's rulings become case law (precedent). New decisions must be consistent with prior rulings or the court rejects them. The methodology is **Caselaw Driven Development (CDD)**.

**Most forks never go to court.** Before the bench convenes, Lexby checks the citator. If a binding ratio already covers the question, the matter is disposed of on citation with no sitting required. The court only convenes for genuine first-impression questions, challenges to existing precedent, or breach charges.

**Three tiers. Mandatory progression.**

```
                    .-------------------------------.
                    |       SUPREME COUNCIL         |
                    |  5 judges (9 for const.)      |
                    |  Foundational + statute-making |
                    '-------------------------------'
                               ^
                               | permission to appeal
                               |
                    .-------------------------------.
                    |       COURT OF APPEAL         |
                    |           3 judges            |
                    |   Disputed / load-bearing     |
                    '-------------------------------'
                               ^
                               | permission to appeal
                               |
                    .-------------------------------.
                    |        FIRST INSTANCE         |
                    |            1 judge            |
                    |         Routine forks         |
                    '-------------------------------'
```

Matters start at First Instance and climb by permission. You cannot jump the queue. Every case produces a neutral-citation ruling artefact (`[YEAR] LEXBY n`) committed to `caselaw/`. **Every ruling is also rendered as a PDF judgment** - formatted exactly as a UK court document, with the court logo, numbered paragraphs, ratio/obiter sections, and a plain-English translation from Lexby.

**Mistakes are civil, not criminal.** There is always a duty of care. The only remedy is to make the work good. No blame, no punishment, just a finding and a fix.

---

## What's in this repo

There is a lot here. Here is the map:

| File / Directory | What it is |
|---|---|
| `SPEC-LAW.md` | The sovereign statute book. S-1 through S-14. The supreme law of every VJS project. |
| `VPR.md` | Vibe Procedure Rules. How matters move through the courts. The Civil Procedure Rules analogue. |
| `CDD.md` | Caselaw Driven Development - the methodology manifesto. What CDD is, how it fits beside TDD. |
| `caselaw/` | Rulings committed to this repo (the VJS founding case lives here). |
| `caselaw/INDEX.md` | The citator. One row per ruling. The fast-path lookup for agents. |
| `court/workflows/` | Three runnable Claude Code Workflow scripts - one per court tier. These ARE the courts. |
| `court/renderer/` | PDF judgment renderer. Node.js + Puppeteer. Produces UK-court-style PDFs. |
| `plugin/CLAUDE.md` | The binding injection block. `cdd init` appends this to your repo's `CLAUDE.md`. |
| `community/` | Community caselaw library. Anonymised rulings submitted from all VJS projects. |
| `docs/DESIGN-NOTES.md` | Full design record. Architecture, decisions, open questions, phase-2 vision. |
| `.github/workflows/clerk.yml` | The clerk bot. Auto-reviews PRs for constitutional compliance and merges if clear. |

---

## The courts (runnable now)

The three court workflows live in `court/workflows/`. They are Claude Code Workflow scripts. Run them via the `Workflow` tool:

```js
Workflow({
  scriptPath: 'court/workflows/first-instance.js',
  args: {
    kind: 'request_for_ruling',
    question: 'Should we use server-side rendering or a SPA?',
  }
})
```

Each workflow:
1. **Loads the live law** (reads `SPEC-LAW.md` and `caselaw/INDEX.md` from the repo - never stale args)
2. **Checks standing and the fast path** (most matters resolve here, no bench required)
3. **Deliberates** (the assigned judge(s) render a full opinion in formal legalese)
4. **Translates** (Lexby gives the plain-English ruling and what it means in practice)
5. **Generates a PDF judgment** (UK-court-style, with court logo and numbered paragraphs)
6. **Submits to the community record** (anonymised PR to `community/caselaw/` under VPR 8)

See `court/README.md` for the full invocation reference.

---

## PDF judgments

Every ruling produces a PDF formatted as a real UK court judgment: court logo at the top, numbered paragraphs, a ruled ratio section, obiter section, and a Lexby translation panel. Drop your logos into `court/renderer/assets/` (see the README there). A placeholder scales-of-justice SVG is used until you do.

```bash
# Install the renderer once
cd court/renderer && npm install

# Test it (renders the founding case)
node index.js --test
# -> You can read the judgment here: /tmp/vjs-test-judgment.pdf
```

---

## 📜 Things you can say to Lexby

```
"I think we should go in this direction. Submit it to the Court."

"I don't agree with the outcome. Can we appeal?"

"The Court of Appeal was split. I think this needs to go to the Supreme Council."

"What did we decide about X, and why?"

"Is this allowed under our spec?"
```

Natural language. No syntax. Lexby handles the filing.

---

## Community

**VJS is building the common law of vibe coding.**

Every project that runs VJS contributes to a shared understanding of what counts as good practice in AI-assisted development. When you get a ruling on "should we use server-side rendering or a SPA", that decision - anonymised - joins a growing library of precedent. The next project asking the same question finds it on the fast path. No sitting. No cost. Disposed in seconds on citation.

**The more people contribute, the better every repo gets.** That is the mechanical consequence of how common law works. More rulings = richer precedent = more fast-path disposals. VJS is a legal commons for AI-assisted projects. The first contributors shape the law that everyone inherits.

Project-specific identifiers (repo names, file paths, variable names) are stripped before submission. The legal question, the ratio, and the law applied are preserved. You share the reasoning, not the source code.

**What the community is building:**

- **Community caselaw library.** Every ruling from every tier is submitted anonymised to `community/caselaw/`. Persuasive precedent across all VJS jurisdictions.
- **Landmark cases gallery.** Browse real precedents from real projects.
- **Case law website (Phase 2).** Public read interface for community rulings, with semantic search.
- **Bring your own bench.** Share judge rosters. Borrow someone else's.
- **Statute packs.** Shareable starter SPEC-LAW for common stacks.

---

## Commands

```bash
cdd
# Install VJS into the current repo and run the live demo case.
# (CLI in development - the court workflows are the current interface)

submit-request-to-court "<question>"
# Ask the court to rule on a fork, a design decision, or a scope question.

submit-breach-to-court "<charge>"
# Confess or charge that the duty of care was not met.
```

---

## WAGMI

You built the thing. The AI helped. Nobody knows who decided what.

Now you have a record.
Now you have precedent.
Now you have Lexby.

Ship fast. Cite everything. The court is in session.

---

<div align="center">

*Vibe Justice System is open source. MIT licensed.*

*Contributions, bench rosters, statute packs, and landmark cases all welcome.*

*Names on the bench are inventions adjacent to the giants of the English bench, never real sitting or living jurists.*

</div>
