<div align="center">

<img src="assets/vjs-header.png" alt="The Vibe Justice System" width="100%">

*AI governance for your repo. The court is AI. Not legal advice.*

![license](https://img.shields.io/badge/license-PolyForm_Noncommercial-blue?style=flat-square)
![status](https://img.shields.io/badge/status-alpha-orange?style=flat-square)
![vibes](https://img.shields.io/badge/vibes-litigated-purple?style=flat-square)
![community](https://img.shields.io/badge/community-open-green?style=flat-square)

**Read the law:** [`The VJS Gazette`](https://wlilley93.github.io/vibe-justice-system/) &middot; [`classic reading view`](https://wlilley93.github.io/vibe-justice-system/gazette.html)

Search the record, filter by estate and class, open the main points of any Act or case. The two estates - the living canon and the honoured archive - resolve in one place.

</div>


---

## v3 — the court, standing on its own

**v1** was a prose realm: real doctrine, nothing enforced. **v2** was a Rust kernel that
needed ever more law to trust itself — of roughly 46 opinions, essentially all were about
VJS's own machinery. Both are preserved under `archive/v1/` and `archive/v2/` and remain
citable as historical authority.

**v3 is neither.** The court is a small TypeScript program over a Lean statute book it does
not own. The kernel — append-only legitimacy, authority chains, citation freshness,
rank-guarded supersession, entrenchment, denial-naming, `res_judicata` — lives upstream in
the [Vibe Proof System](https://github.com/wlilley93/vibe-proof-system) and is consumed here
as a **pinned lake dependency**, not a vendored copy. What is proved stays proved upstream;
what is judged stays judged here.

That split is only possible because the kernel's sovereign digest is now a *parameter*.
Before that, a jurisdiction could obtain its own genesis only by editing constitutional text,
so every jurisdiction was a private fork of the engine and nothing could check the forks
still agreed. Now: one engine, many jurisdictions, and a jurisdiction provably cannot borrow
another's authority.

### What it is for

A court answers one question — *has this been decided, and what was decided* — and remembers
the answer. Nothing about that requires a repository, a specification, or a build. Rulings
carry no operative rule; they have a citation, a standing, an authority chain and a payload.
That is exactly what makes the court reusable: an operational question ("may this deploy
divert egress?") is a question with a key, just like a modelling one.

```sh
vjs ask    "op:boltrig:dev-egress-loopback"   # decided already? (exact key — this is res judicata)
vjs rule   "op:boltrig:dev-egress-loopback" --question "…" --facts "…"
vjs appeal "[2026] VJS 6" --grounds "…"       # three benches and a synthesis
vjs search "egress"                            # full text, for people — never the route to prior law
vjs book                                       # the statute book in force
vjs gate --staged                              # exit 6 on denial, each denial naming its law
```

**One boundary matters more than the rest.** `ask` is an exact match on a hashed key, and
that exactness *is* res judicata. `search` is a human affordance and is never wired into it —
a ranking function anywhere near the memo table would let the same question be answered two
ways while the theorem still compiled.

### What is proved, and what is not

Proved upstream, over every lawful book: append-only legitimacy, authority chains, citation
freshness, rank-guarded supersession, entrenchment immunity and force, denial-naming, and
that a sound precedent table can never disagree with deliberation.

Trusted, not proved: the content of every ruling. A bench is a language model. The kernel
makes the record incorruptible; it does not make the decisions correct, and nothing this
court produces may claim otherwise.


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

## Get started

You need Rust (stable) and git. Build the kernel once:

```bash
git clone https://github.com/wlilley93/vibe-justice-system.git
cd vibe-justice-system
cargo build --release
export PATH="$PWD/target/release:$PATH"    # so `vjs` is on your PATH
```

Then **invoke** your own repo as a jurisdiction. This is the constitutional act: it
subscribes your repo to the lawpack, pins that lawpack's digest so the law you are
governed by cannot change under you, records the invocation, arms the store register, and
installs the enforcement hooks.

```bash
cd /path/to/your-project
vjs invoke --jurisdiction acme \
           --principal "Your Name" \
           --lawpack /path/to/vibe-justice-system/lawpack/v2 \
           --install-hooks
```

That writes `.vjs/` into your project: the config, the lawpack lock, the invocation
record, the hooks, and your own empty record stores. Your repo is now a jurisdiction with
its own County Court, and you are its Principal.

Check it came up clean:

```bash
vjs validate                        # the law loads, the gates are armed
vjs status                          # what this jurisdiction is subscribed to
```

The loop your agent runs, for every governed act:

```bash
vjs route --kind code_change --intent "add a caching layer" \
          --risk medium --path src/api.rs --issue caching_strategy
```

`route` is the clerk. It returns the binding authorities, whether a court is required and
which one, whether a decision log is owed, and an explicit **must do / must not do** list.
It is deterministic and never calls a model. If it says a court is required, the agent
convenes on its own motion and files a symmetric case file. If it does not, the agent
records the call and proceeds:

```bash
vjs log decision --kind code_change --issue caching_strategy \
                 --decision "cache at the client, not the gateway" \
                 --risk medium --why "the gateway cannot see per-tenant scope"
```

With `--install-hooks`, the pre-commit hook fails closed: a governed write without a
routed permit does not land. That is the whole point. Enforcement is a function of repo
state, not a paragraph in a prompt that a tired model skips.

**Point your agent at [`AGENTS.md`](AGENTS.md)** (or copy it into `CLAUDE.md`,
`.cursorrules`, or your runtime's equivalent). That file is what turns the CLI into
Lexby.

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

Everything resolves in one Gazette, two estates: **the living canon** and the read-only **honoured archive**.

- **The Gazette:** [`the constellation`](https://wlilley93.github.io/vibe-justice-system/) - the law as an explorable graph: search, browseable list, full text in the reading panel, dockets and lineage.
- **The classic view:** [`gazette.html`](https://wlilley93.github.io/vibe-justice-system/gazette.html) - the Realm Law Reports as cards: estate and class filters, the main points of every item.
- **The index:** [`GAZETTE.md`](GAZETTE.md)
- **The live law:** the compact lawpack under [`lawpack/`](lawpack/) and the court record under [`.vjs/submissions/filed/`](.vjs/submissions/filed/).
- **The archive:** the first generation, preserved on the `v1` branch and the `v1-archive-2026-06-09` tag.

---

## License

PolyForm Noncommercial License 1.0.0 for noncommercial use. See [`LICENSE`](LICENSE) and [`NOTICE.md`](NOTICE.md).

For commercial use, a separate license is required. See [`COMMERCIAL_LICENSE.md`](COMMERCIAL_LICENSE.md).
