<div align="center">

<img src="assets/vjs-header.png" alt="The Vibe Justice System" width="100%">

*AI governance for your repo. The court is AI. Not legal advice.*

![license](https://img.shields.io/badge/license-AGPL--3.0-blue?style=flat-square)
![status](https://img.shields.io/badge/status-alpha-orange?style=flat-square)
![vibes](https://img.shields.io/badge/vibes-litigated-purple?style=flat-square)
![community](https://img.shields.io/badge/community-open-green?style=flat-square)

**Read the law:** [`The VJS Gazette`](https://wlilley93.github.io/vibe-justice-system/gazette.html) &middot; [`constellation view`](https://wlilley93.github.io/vibe-justice-system/gazette-graph.html)

Search the record, filter by estate and class, open the main points of any Act or case. The two estates - the living canon and the honoured archive - resolve in one place.

</div>

> **Disclaimers**
> - **Not a real court. Not legal advice.** VJS is an AI governance framework. Rulings are AI outputs, not legal instruments.
> - **Real-world law still controls.** Local sovereignty means sovereignty over the local VJS copy, not immunity from real-world law. The local Principal/Sovereign remains responsible for following the real-world law that applies to them and their repo. Agents have delegated authority to refuse, stop, narrow, or escalate instructions that appear unlawful, unauthorised, or cyber-abusive.
> - **Production systems need real engineers.** VJS helps record and structure AI decisions - it does not replace qualified engineering review, security audit, or human sign-off on anything that matters in the real world.
> - **It only refines what you give it.** Rulings are only as good as the spec and context you provide. Garbage in, garbage out. A weak spec produces weak law.

> **Alpha status**
> - **Computer-first.** Live law is a compact, machine-checkable **lawpack**, loaded by a deterministic **kernel** that acts as clerk, not court - it loads, validates, routes, and records, and never calls a model. Human meaning stays the source of legitimacy.
> - **Citation numbering is deterministic.** The next neutral citation is computed from the citator, not guessed. Realm citations use the `REALM-*` (archive) and `VJS-*` (current) provenance schemes.
> - **Enforcement is a function, not a prompt.** Governed writes need a routed, scoped permit; material decisions need a logged reason; the gate fails closed at commit time. The kernel governs; the prompt only explains.
> - **The public record is system data only.** Personal, operational, and project-private facts do not belong here. Private working papers stay local and gitignored.

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

Everything else is a **citation, not a sitting**: before any bench sits, the citator is searched, and a binding ruling on all fours disposes of the matter instantly. The agent convenes on its own motion - it never routes the fork to you and never asks permission to convene.

That loop is **Caselaw Driven Development (CDD)**: a fork produces a ruling; the ruling is committed with a citation; every future session cites it instead of re-deciding. Where TDD records that the code does what you said, CDD records *why* you said it.

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

The court has three tiers - County (one judge), Privy Council (three), Supreme Court (five, expanding to nine for foundational questions) - and a single apex. Benches are always odd-numbered and decide on a symmetric, two-sided case file with no access to Lexby's preference.

---

## Local sovereignty and community

A repo joins VJS by **local sovereign invocation**, not by where it sits: subscribe to a lawpack, lock its digest, install the enforcement hooks. The person responsible becomes the local Principal, acting as sovereign for that copy. Their agents get a working constitution, courts, procedure, citation rules, and safety hooks on day one.

That subscription is a starting point, not a lock-in. The local Principal can amend the local law, pin or decline a version, fork with declared lineage, become independent, or join another community record. Those changes bind that repo only, unless accepted back into the canon by its rules. Local sovereignty is repo sovereignty, not legal immunity.

Nothing automatically pushes your case law upstream. If you want to contribute a generally useful ruling, you can submit an anonymised PR to the community record. **The more good rulings go in, the faster every project resolves** - before any court sits, Lexby checks the precedent index first, and the fast path disposes of the matter on citation with no sitting.

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

## Read the law

Everything resolves in one Gazette, two estates: the **living canon** and the read-only **V1 archive**.

- **The Gazette:** [`gazette.html`](https://wlilley93.github.io/vibe-justice-system/gazette.html) - the Realm Law Reports: search, estate and class filters, the main points of every item.
- **The constellation:** [`gazette-graph.html`](https://wlilley93.github.io/vibe-justice-system/gazette-graph.html) - the same record as a citation graph, with a reading sidebar.
- **The index:** [`GAZETTE.md`](GAZETTE.md)
- **The live law:** the compact lawpack under [`lawpack/`](lawpack/) and the court record under [`.vjs/submissions/filed/`](.vjs/submissions/filed/).
- **The archive:** the first generation, preserved on the `v1` branch and the `v1-archive-2026-06-09` tag.

---

## License

GNU Affero General Public License v3.0 (AGPL-3.0). See [`LICENSE`](LICENSE) and [`NOTICE.md`](NOTICE.md).
