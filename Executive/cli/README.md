# vibe-justice (CLI)

Zero-dependency Node CLI for the Vibe Justice System. Binaries: `cdd` and `vjs` (same tool).

```bash
# from the repo (no publish needed)
node cli/bin/cdd.js --help
npm link ./cli          # then: cdd --help   /   vjs --help
```

## Commands

| Command | What it does |
|---|---|
| `cdd init [dir]` | Install VJS into a repo: vendor `CASE-LAW.md` / `VPR.md` / `CDD.md`, scaffold `.justice/` (citator + `caselaw/` + `pdfs/`), and append the binding `plugin/CLAUDE.md` block to the target `CLAUDE.md` (idempotent). |
| `cdd next-citation <tier> [--year YYYY] [--citator PATH] [--json]` | **Deterministic** next neutral citation, computed from the citator (`.justice/INDEX.md`, legacy fallback `caselaw/INDEX.md`). `tier` = `first-instance` \| `court-of-appeal` \| `supreme-court` (or `FI`/`CA`/`SC`). |
| `cdd submit-request "<question>"` | Print the `Workflow` invocation to file a Request for Ruling (the court runs inside Claude Code). |
| `cdd submit-breach "<charge>"` | Print the `Workflow` invocation to file a Breach. |
| `cdd --version` | Print version. |

## Citation numbering

`lib/citation.js` is the **source of truth** for deterministic numbering: it parses the citator for the
highest `N` already issued for a tier+year and returns the next, in the operative tiered form
`[YEAR] LEXBY-<TIER> N`. The three court Workflow scripts inline a minimal mirror of this (the Workflow
sandbox has no `require`) - keep them in sync.

```bash
npm test   # node lib/citation.test.js
```

*Spec is law. Rulings are precedent. Lexby is your lawyer.*
