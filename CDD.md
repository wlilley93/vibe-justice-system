# CDD - Caselaw Driven Development

**The methodology behind the Vibe Justice System.**

---

## What it is

Caselaw Driven Development is a practice for recording AI-assisted decisions as binding precedent. Every meaningful fork in your project - architecture, conventions, scope, naming - produces a ruling. That ruling is committed to `.justice/judgments/`, carries a neutral citation, and governs every future session that touches the same ground. The AI does not re-litigate settled points; it cites them and moves on.

---

## The core loop

```
fork arises
  -> file a Request for Ruling (submit-request-to-court)
  -> court convenes and rules
  -> ruling committed to .justice/judgments/
  -> future forks on the same point cite the ratio and close without a sitting
```

One pass through the loop converts a silent decision into a permanent, searchable, citable fact about your project.

---

## The fast path (most forks never sit)

Before the bench convenes, the citator is searched. If a binding ratio covers the point on all fours, the matter is disposed of on citation with no sitting required (VPR 2). In a mature project most forks close this way, in seconds, with zero deliberation cost. The court convenes only for genuine first-impression matters, material distinctions, attempted overrulings, or contested breaches.

---

## When the court convenes

A sitting is warranted when:

- the question is first-impression (no existing ratio covers it)
- prior precedent arguably does not fit the current facts (a distinction)
- a ruling is challenged as wrong or outdated (an overruling application)
- a breach of the duty of care is charged (submit-breach-to-court)

Everything else is a citation, not a sitting.

---

## The three commands

```bash
cdd
# Install VJS into the current repo and run a live demo case.

submit-request-to-court "<question>"
# Ask the court to rule on a fork, a design decision, or a scope question.

submit-breach-to-court "<charge>"
# Raise that a prior decision or a piece of work did not meet the duty of care.
```

Natural language. No filing syntax. Lexby handles intake, standing, and routing.

---

## How CDD fits into a normal workflow

CDD is additive, not a gate. You do not stop to ask permission before writing code. You write; when a non-trivial fork emerges, you file it. The ruling artefact is a committed markdown file in .justice/judgments/. It adds one file per decision, sits alongside your normal commits, and costs nothing on the hot path. Sessions that follow inherit the full precedent record automatically.

---

## CDD vs TDD

| | TDD | CDD |
|---|---|---|
| What it captures | Behaviour | Decisions |
| Artefact | Test file | Ruling file |
| Failure mode it fixes | Regressions in behaviour | Regressions in intent |
| When it runs | CI | When a fork is raised |
| Value over time | Catches breakage | Prevents drift |

TDD tests that the code does what you said. CDD records why you said it.

---

## The ruling artefact

Every ruling is a markdown file committed to `.justice/judgments/` with:

- **Neutral citation** - `[YEAR] LEXBY n` (sequential per repo)
- **Tier** - First Instance, Court of Appeal, or Supreme Court
- **Panel** - named judges (ephemeral stances seeded only on first impression)
- **Ratio** - the binding holding, stated precisely
- **Obiter** - non-binding observations (persuasive only, VPR 6)
- **Remedy** - for breach matters: make good, restore the position; no punishment (CASE-LAW s. 6)
- **Lexby TL;DR** - plain-English translation of the holding

Only the ratio binds. Obiter is persuasive. A ruling made in ignorance of binding law or precedent is per incuriam and void.

---

## Binding your agents (the CLAUDE.md block)

`cdd init` appends the binding block to your repo's `CLAUDE.md` automatically. That block is what makes the
law real: Claude Code loads `CLAUDE.md` into every agent session, so every agent sees the duty of care, the
enumerable trigger list, the commands, and the pointer to the citator before it writes a single line.

The binding block lives at `plugin/CLAUDE.md` in the VJS repo. It covers:
- The duty of care and the graded endeavours standard (CASE-LAW s. 4 through s. 8)
- The citator check (grep .justice/INDEX.md first; cite if covered, convene if not)
- The enumerable trigger list (exactly five conditions that warrant a sitting; nothing else)
- The commands (`submit-request-to-court`, `submit-breach-to-court`)
- The progression rules summary (VPR)

**The trigger list is the critical part.** An agent told "convene when uncertain" will convene on every
trivial fork and cost a fortune. An agent given five precise conditions will convene only when the law
actually requires it, and cite existing precedent for everything else.

---

## Cross-references

- **CASE-LAW.md** - the sovereign statute book; case law is subordinate to it and void where it conflicts
- **VPR.md** - the Vibe Procedure Rules; governs standing, progression, the fast path, bench sizes, and the leapfrog certificate
- **plugin/CLAUDE.md** - the binding injection block; what gets appended to your repo's CLAUDE.md by `cdd init`
- **court/README.md** - how to invoke the three court workflows directly
