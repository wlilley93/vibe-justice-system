# VJS V2 Specification

## Kernel design contract

> `vjs-core` is a deterministic authority resolver and route clerk. It receives structured facts about a proposed act, applies compact V2 law, orders, rules, and logs, and returns a bounded instruction. It never litigates by prose, never searches by vibes, never calls a model, and never makes long judgments part of normal runtime.

## Lifecycle

```
route -> permit -> obligations -> proof -> log -> validate
```

## Specs

Specs are first-class law objects. A spec contains:
- purpose
- scope
- decisions
- invariants
- obligations
- review triggers

## Invariants

Invariants are deterministic, typed, and evaluated mechanically. They use a fixed predicate registry:
- path_changed
- string_contains
- import_contains
- dependency_added
- decision_log_exists
- order_exists
- word_count_lte
- citation_unique

## Decisions

Decisions are compact ADR-like objects. Max 60 words for the decision, 120 for the reason.

## Permits

Permits are required for governed writes. They expire after 2 hours. Closing requires proof.

## Proofs

Proofs attach evidence to permits. Kinds: command_result, decision_log, test_result, public_private_scan, validation_report.

## Logs

Decision logs are 50-150 words. Required for material decisions.

## Hooks

- Session: 40 words max
- Pre-write: permit check
- Pre-commit: `vjs validate --staged`

## MCP

6 tools: vjs.route, vjs.lookup, vjs.validate, vjs.log, vjs.file, vjs.status.

## Performance

- route: <100ms warm
- lookup: <100ms warm
- validate: <2s
- route output: <=300 words
- authorities: <=5
- log: <=150 words

## Migration

V1 is the archive. V2 is the runtime. Only expressly extracted V1 learnings become V2 law.

## Directory structure

One fixed anchor: `.vjs/config.toml`. All other paths configurable by role.

## Boot orders

10 orders bind the V2 repo from inception. See `lawpack/v2/orders/2026-VJS-BOOT-*.yaml`.
