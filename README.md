# VJS - the Vibe Justice System

The canonical, computer-first Realm. (Developed as "V2"; the spent ordinal is dropped from the name per [2026] VJS-PC 2 / SC-1. The first generation is preserved as the read-only **V1 archive estate** on the `v1` branch and the immutable `v1-archive-2026-06-09` tag.)

## Status and known limitations

VJS V2 is founded, assented, and running as live local law, but it is early and not yet publicly released. Known limitations a reader should weigh before adopting it:

- The canonical migration onto the public line is **staged, not executed** (the runbook is prepared; the byte-move and push are warranted acts).
- Some carried-forward provisions are **staged** pending machine-checkable resolution (see the Framework Act Schedule 3); they do not yet commence.
- The kernel's first-impression detection routes to court on **empty** authority; precise "on-point but non-matching" detection is a known follow-up.
- The full GNU AGPL-3.0 text is to be vendored on the canonical line (this repo records the adoption).

The law binds local work today; treat public-facing claims as alpha until the staged release completes.

## What is VJS V2?

VJS V2 is a deterministic authority resolver and route clerk for AI-assisted software engineering. It replaces long constitutional prose with a compact, machine-checkable lawpack and a Rust kernel that returns bounded instructions.

## Core principle

> The agent reads the smallest binding instruction required to act lawfully.

## V2 vs V1

| | V1 | V2 |
|---|---|---|
| Runtime | Full case law, Acts, SIs | Compact lawpack |
| Context | Long judgments | Short orders, rules, logs |
| Hooks | Long prose | 40-word state checks |
| Courts | 4 tiers + CoA | County, Privy, Supreme |
| Enforcement | Agent discretion | Deterministic kernel |
| Gazette | Runtime source | Archive only |

## Architecture

```
vjs-core       Deterministic authority resolver
vjs-lawpack    Parse, validate, build authority graph
vjs-store      Text-backed records + SQLite cache
vjs-cli        Human and hook commands
vjs-mcp        Thin MCP adapter (6 tools)
vjs-git        Repo detection, diff, hooks
vjs-redact     Boundary scanner
vjs-testkit    Fixtures, golden tests
```

## Installation

```bash
# Clone
git clone https://github.com/wlilley93/agent-universe-v2
cd agent-universe-v2

# Build
cargo build --release

# Install into a repo
./target/release/vjs init
```

## Quick start

```bash
# Route a decision
vjs route --kind implementation-decision --issue dependency_policy --risk low --intent "Add a new crate"

# Write a log
vjs log decision --issue dependency_policy --decision "Keep kernel dependency-free" --basis AGENT-LOOKUP-001 --risk low --why "Kernel determinism requires auditable local execution."

# Validate
vjs validate --staged

# Full CI
vjs local-ci
```

## Lawpack

The V2 lawpack is in `lawpack/v2/`:

- `constitution.yaml` — V2 constitution
- `statutes/` — 7 compact Acts
- `regulations/` — Kernel regulations
- `rules/` — Rule atoms
- `orders/` — Court orders
- `specs/` — Specs (machine-checkable contracts)
- `invariants/` — Deterministic invariants
- `decisions/` — Compact decisions

## Self-governance

This repo is governed by VJS V2. See `AGENTS.md` for the agent contract.

## License

GNU Affero General Public License v3.0 (AGPL-3.0). Adopted by the Sovereign
Founder as copyright holder under the Realm Consolidation and Reconciliation
Framework Act s.22 (2026-06-09), superseding the prior MIT notice. See `LICENSE`.
