# VJS V2 Constitutional Migration Charter

## 1. Founding declaration

Vibe Justice System Version 1 (V1) is hereby preserved as the historical research constitution and archive of the Agent Universe. It contains the case-law journey, the founding 25 Acts, Bills 26 through 31, the statutory instrument layer, the central citator, the Gazette, the law reports, and the full doctrinal record of discovery.

V2 is a consolidation, not a repudiation. Only expressly extracted V1 learnings become V2 runtime law. Long-form judgments, full Acts, and SI materials remain archive and explanation, not agent context. The V2 kernel lawpack is the sole runtime authority surface.

## 2. Status of V1

V1 becomes the VJS Gazette and Archive. It is read-only by default. New doctrine shall not expand it except for a final migration record and archive maintenance. It remains citable as historical authority and migration evidence. It is not loaded as live agent context unless a V2 statute, rule, or order expressly incorporates it.

## 3. Status of V2

V2 begins with a compact legislative base that absorbs the settled V1 learnings. The runtime hierarchy is:

1. real-world law warning boundary
2. V2 Constitution
3. V2 primary Acts
4. valid Kernel Regulations
5. Supreme Court orders
6. Privy Council orders
7. County Court repo orders
8. repo decision logs
9. V1 archive material (only if expressly incorporated)

## 4. What V2 carries forward

Every settled V1 learning is preserved in one of three forms:

- compact statute (the 7 Acts)
- kernel regulation (short, schema-bound, machine-readable)
- rule atom (extracted legal instruction)
- court order (binding operative record)
- decision log (repo-local convention record)

## 5. What V2 does not carry forward

V2 does not carry forward as runtime law:

- the full V1 case-law settlement text (archive only)
- the 31 Bills as runtime context (extracted into statutes)
- the 11 SIs as a large ceremonial layer (compressed into regulations)
- the Court of Appeal (removed from MVP)
- the Gazette as a runtime source (remains archive/publication)
- swarm deliberation as a route mechanism
- model-mediated constitutional review (replaced by deterministic kernel)
- the four-branch ministry directory structure (replaced by configurable record roles)

## 6. Role of the Principal

The Principal remains the human legitimacy source. The Principal may:

- set objectives
- give policy direction
- approve this migration
- authorise external/release acts
- approve local sovereignty changes
- request a court route
- assent to compact legislation

The Principal may not:

- make unsafe or unlawful conduct valid by preference alone
- authorise real-world illegality
- turn Lexby into the judge
- silently bypass record duties
- force public disclosure of private facts

## 7. Role of Lexby

Lexby retains the three offices: Advocate, Advisor, Engineer.

Lexby may:

- argue the strongest case for an idea and put it to the court
- give advice straight
- ship, then record why
- call the kernel for authority
- file submissions to court
- write decision logs
- delegate to subagents where permitted

Lexby may not:

- be the source of legal force
- draft binding law and have it become binding by the fact of being written
- sit on the bench
- self-authorise law
- bypass the kernel route

## 8. Court structure in V2

- County Court: repo-local operational decisions
- Privy Council: jurisdiction, routing, constitutional machinery, public/private boundary
- Supreme Court: rare foundational doctrine
- Court of Appeal: not carried forward in V2 MVP

## 9. The kernel as clerk

`vjs-core` is a deterministic authority resolver and route clerk. It receives structured facts about a proposed act, applies compact V2 law, orders, rules, and logs, and returns a bounded instruction. It never litigates by prose, never searches by vibes, never calls a model, and never makes long judgments part of normal runtime.

## 10. The agent operating contract

The V2 agent loop is a short deterministic lifecycle:

```
route -> permit -> obligations -> proof -> log -> validate
```

Before governed load-bearing work, the agent calls `vjs.route`.
The kernel returns a permit and obligations.
The agent acts.
The agent attaches proof.
The agent writes required decision logs.
The agent runs `vjs validate` before commit.

Hooks do not explain the law. They enforce the lifecycle:

- Session hook: 40 words max, points to `vjs.route`.
- Pre-write hook: checks active permit for governed writes.
- Pre-commit hook: runs `vjs validate --staged`.

## 11. Migration acceptance rule

A V1 doctrine is not live in V2 unless it appears in the migration ledger and has a V2 statute, regulation, rule atom, or order destination. The migration ledger is the single source of truth for what crossed the boundary.

## 12. Principal assent

This migration is authorised by the Principal. The Principal assents to:

- freezing V1 as the archive
- adopting the V2 compact lawpack as the runtime authority
- governing the V2 repo itself by the V2 kernel
- building the Rust kernel, CLI, and MCP adapter as specified
- beginning with the 7 compact statutes and the spec/invariant/decision/proof/log system

## 13. First dogfood orders

The following orders bind the V2 repo from inception:

- ORDER-BOOT-001: V2 uses orders, rules, and logs as runtime law.
- ORDER-BOOT-002: V1 archive is not runtime context unless incorporated.
- ORDER-BOOT-003: No Court of Appeal in V2 MVP.
- ORDER-BOOT-004: Logs are mandatory for material implementation decisions.
- ORDER-BOOT-005: MCP adapter is thin and local-first.
- ORDER-BOOT-006: Kernel must remain deterministic, model-free, and network-free.
- ORDER-BOOT-007: Invariants are deterministic, typed, and evaluated mechanically.
- ORDER-BOOT-008: Specs are first-class law objects with purpose, scope, decisions, invariants, and obligations.
- ORDER-BOOT-009: Permits are required for governed writes; hooks enforce the lifecycle, not the law.
- ORDER-BOOT-010: The public/private boundary is enforced by deterministic scanner, not model judgement.

Signed: Principal, 2026-06-08

## 14. Royal Assent and commencement (2026-06-09)

The constitutional relay is complete and **V2 has commenced**. The Computer-First Realm Act 2026 (Bill 32), drafted by the Standing Committee on the V2 Kernel Team's void first draft and settled on [2026] REALM-PC 24 and [2026] REALM-SC 10 (which enacted the CASE-LAW s.23 Sovereign-assent floor), received **Royal Assent from the Sovereign Founder on 2026-06-09**, pinned against the assented-text digest `sha256:8e1d3f51...6b9a0c`. Per [2026] REALM-SC 10 both gates are satisfied (Gate A: the Sovereign's antecedent, external, digest-pinned assent; Gate B: the fail-closed Assent-Source Invariant, the constituted V2 courts, and the validated, locked lawpack). V2 is the live computer-first runtime jurisdiction; V1 is the Gazette and Archive estate. Canonical records: `lawpack/v2/provenance/founding/COMMENCEMENT-V2-0001.yaml`, `V1-SETTLEMENT-OUTCOME.md`, the sealed `bill-32-adopted-final-text.md`, and order `2026-VJS-COURTS-CONSTITUTION-001`. The Act's VJS-ACT ordinal is minted at commencement; the earlier self-mint "[2026] VJS-ACT 8" was void and is not used.
