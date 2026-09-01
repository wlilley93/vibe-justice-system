
<div align="center">

<img src="assets/vjs-header.png" alt="The Vibe Justice System" width="100%">

*AI governance for your repo. The court is AI. Not legal advice.*

[![license](https://img.shields.io/badge/license-PolyForm_Noncommercial-blue?style=flat-square)](LICENSE)
[![status](https://img.shields.io/badge/status-alpha-orange?style=flat-square)](https://github.com/wlilley93/vibe-justice-system)
[![Lean 4.33.1](https://img.shields.io/badge/Lean-4.33.1-6f6f6f?style=flat-square)](https://github.com/leanprover/lean4/releases/tag/v4.33.1)
[![community](https://img.shields.io/badge/community-open-green?style=flat-square)](https://github.com/wlilley93/vibe-justice-system)

**Read the law:** [The VJS Gazette](https://wlilley93.github.io/vibe-justice-system/) ·
[classic reading view](https://wlilley93.github.io/vibe-justice-system/gazette.html)

</div>

---

# Vibe Justice System

Vibe Justice System (VJS) is a reusable court for AI-assisted work. It records decisions,
checks whether a question has already been decided, convenes a probabilistic bench when
meaning is genuinely contested, and files the result as durable precedent.

The current v3 is a small TypeScript court over the
[Vibe Proof System](https://github.com/wlilley93/vibe-proof-system). VJS owns the court
surface — questions, filings, benches, appeals, judgments and the citator. VPS owns the
constitutional spine — the typed statute book, lawful enactment and fail-closed gate.

**Not a real court. Not legal advice.** Model rulings are not legal instruments, and this
repository does not replace engineering review, security review, human sign-off or the law
that applies in the real world.

## v3 — the court, standing on its own

VJS v1 was a prose realm: useful doctrine, nothing enforced. VJS v2 was a Rust kernel that
needed increasingly elaborate machinery to verify its own integrity. Those generations are
preserved under the archive and remain useful historical authority.

VJS v3 takes the cleaner boundary. The court is implemented in TypeScript; the statute book
and gate are Lean; the Lean kernel is consumed from VPS as a pinned Lake dependency. The
earlier Rust-kernel line is therefore not the thing that now decides whether a filing is
lawful. The current proof artifact does.

That split gives one engine many jurisdictions. A jurisdiction supplies its own book and
genesis digest; the upstream kernel proves the rules that keep one jurisdiction from
borrowing another's authority.

## What it does

VJS answers a narrow, valuable question:

> Has this question been decided, and what was decided?

The exact-match lookup is the res judicata door. A standing ruling on the same hashed
question key is reused; it is not re-litigated. Full-text search is a separate human
affordance and is never used as the route to a prior decision.

When a question is open, VJS can convene a first-instance model judge. An appeal runs three
deliberative personas — textualist, purposivist and pragmatist — followed by a synthesis.
Those judges are probabilistic. They supply the interpretation and reasoning that a human
may review; they do not alter the Lean statute book or bypass its enactment checks.

A filed ruling carries a question key, matter, facts, reasoning, law applied, citation,
court and standing status. If an appeal overturns it, the replacement is enacted through
the kernel and the old ruling is marked superseded.

## What is proved, and what is not

Proved upstream by VPS, over every lawful book:

- append-only legitimacy and authority chains;
- fresh, unique citations;
- rank-guarded supersession;
- entrenchment immunity and force;
- the soundness of the precedent table; and
- denial-naming: a denial carries the instrument that caused it.

Trusted, not proved: the content of a model's ruling, the facts supplied to the bench, the
small TypeScript shell that extracts facts and writes payloads, the compiler, the checker,
git and the human sign-off. The court keeps those boundaries visible. It does not turn
model confidence into proof.

## Quickstart

The published primary branch is pinned to Lean 4.33.1 and the VPS dependency is pinned to a
reviewed commit. Requirements are Node.js 20 or newer, git and elan.

    git clone https://github.com/wlilley93/vibe-justice-system.git
    cd vibe-justice-system
    npm ci
    npm run build

    cd lean
    lake build
    lake env leanchecker Vjs
    cd ..

    node dist/cli.js doctor
    node dist/cli.js book

The Lean build fetches the pinned VPS kernel from GitHub. It does not require a sibling
checkout of VPS. The first build may take a little longer while Lake materialises the
dependency.

For a prepared jurisdiction, the useful surfaces are:

    node dist/cli.js ask "op:boltrig:dev-egress-loopback"
    node dist/cli.js search "egress"
    node dist/cli.js docket
    node dist/cli.js gate --staged

The gate fails closed when Lean is unavailable; it never silently substitutes a second
implementation for the proof kernel. To put a new question before a bench, configure an
LLM provider in vjs.config.json and use the rule command:

    node dist/cli.js rule "op:example:cache-location" \
      --question "Where should the cache live?" \
      --facts "The gateway cannot see per-tenant scope."

## The workflow

Every governed fork follows the same shape:

1. The agent states the question and the facts.
2. VJS checks the exact question key for standing precedent.
3. If no standing ruling applies, a probabilistic bench proposes a ruling and reasoning.
4. The result is filed with a citation and, where applicable, enacted through VPS.
5. Future sessions cite the result instead of silently choosing again.
6. A genuine disagreement can be appealed; an overturned ruling is superseded, not erased.

This is Caselaw Driven Development. TDD records that code satisfies a test. CDD records
why the project chose a direction, so the next session inherits a reason rather than a vibe.

## The constitutional boundary

VJS is the court, not the law. VPS is the constitutional substrate:

- VJS owns natural-language questions, court procedure, model deliberation, judgments and
  appeals.
- VPS owns typed instruments, authority, citation, supersession, entrenchment and the
  compiled gate.
- The gate extracts simple facts from a change and asks the Lean artifact for the verdict.
- A shell or model can explain a verdict; it cannot make an unlawful instrument lawful.

This is why the move from the former Rust kernel matters. A separate Rust implementation
and Lean model would create a correspondence problem — a watcher watching a watcher. VJS now
uses Lean as the proof substrate for the filings and governance decisions that need to be
mechanically enforceable.

## Read the law

The public record is available in the [VJS Gazette](https://wlilley93.github.io/vibe-justice-system/).
The repository contains the current statutes, judgment index and source used to render it.
The archive contains the earlier VJS v1 and v2 lines; those are historical, not the current
runtime.

## Repository map

- [src/court/](src/court/) — first-instance bench, appeals and citator-facing logic.
- [src/kernel/](src/kernel/) — book, enactment, gate bridge and Lean runner.
- [lean/](lean/) — the VJS statute book and pinned VPS dependency.
- [law/](law/) — prose mirrors of enacted instruments.
- [record/](record/) — build and case history.
- [.justice/](.justice/) — the jurisdiction's machine-readable book and judgments.
- [AGENTS.md](AGENTS.md) — the agent-facing operating contract.
- [PUBLISHING.md](PUBLISHING.md) — publication boundaries and the Gazette.

Related projects:

- [Vibe Proof System](https://github.com/wlilley93/vibe-proof-system) — the Lean governance
  kernel.
- [Foundry](https://github.com/wlilley93/foundry) — the prose-to-proof requirements
  pipeline built on that kernel.

## Status and licence

VJS v3 is alpha research software. The current public primary branch builds its TypeScript
court and Lean statute book on Lean 4.33.1, with model judgment and human sign-off kept
explicitly outside the proof claim.

VJS is released under the PolyForm Noncommercial License 1.0.0 for noncommercial use.
See [LICENSE](LICENSE), [NOTICE.md](NOTICE.md) and [COMMERCIAL_LICENSE.md](COMMERCIAL_LICENSE.md).
