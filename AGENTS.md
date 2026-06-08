# VJS V2 Agent Contract

This repo is governed by VJS V2.

Before governed load-bearing work, call `vjs.route`.
If the route is settled, follow the returned orders/rules.
If `court_required=true`, file a short submission.
After material implementation decisions, write a decision log.
Do not place private repo facts in public records.
The kernel answer is the runtime authority surface.

## Quick reference

```bash
# Before work
vjs route --kind implementation-decision --issue <issue> --intent "<description>"

# After work
vjs log decision --issue <issue> --decision "<decision>" --basis <authority> --risk low --why "<reason>"

# Before commit
vjs validate --staged

# Full check
vjs local-ci
```

## Lifecycle

```
route -> permit -> obligations -> proof -> log -> validate
```

## Hooks

- Session: 40 words max, points to `vjs.route`
- Pre-write: checks active permit
- Pre-commit: `vjs validate --staged`

## Rules

- No long hook prose
- No bypassing the kernel
- No publishing private facts
- No model calls in core
- No network calls in core

## Permits

Governed writes require a permit. Close permits with proof. Unpermitted writes are blocked at commit.

## Logs

Material decisions require a 50-150 word log. Missing logs block validation.

## Boundary

Public records contain system data only. Private facts stay in `.vjs/private` or configured private path.

## Courts

- County Court: repo-local decisions
- Privy Council: routing, jurisdiction, boundary
- Supreme Court: foundational doctrine
- No Court of Appeal in MVP

## Authority hierarchy

1. Real-world law
2. V2 Constitution
3. V2 primary Acts
4. Kernel regulations
5. Supreme Court orders
6. Privy Council orders
7. County Court orders
8. Local decision logs
9. V1 archive (only if expressly incorporated)

## Kernel non-goals

The kernel does NOT:
- call LLMs
- use vector search
- render PDFs
- host the Gazette
- auto-publish
- replace human approval

The kernel IS a deterministic clerk.
