# vibe-justice (CLI)

Zero-dependency Node CLI for the Vibe Justice System. Binaries: `cdd` and `vjs` (same tool).

```bash
# from the repo (no publish needed)
node Executive/cli/bin/cdd.js --help
npm link ./Executive/cli   # then: cdd --help   /   vjs --help
```

## Commands

| Command | What it does |
|---|---|
| `cdd init [dir]` | Install VJS into a repo: vendor `CASE-LAW.md` / `VPR.md` / `CDD.md`, scaffold local `.justice/` (citator, `judgments/`, `caselaw/`, `pdfs/`, `suites/`), append the generic `Executive/plugin/AGENTS.md` contract to `AGENTS.md`, install portable scripts under `.vjs/hooks/`, and bind the bundled Claude adapter through `CLAUDE.md` / `.claude/settings.json` (idempotent). |
| `cdd next-citation <tier> [--year YYYY] [--citator PATH] [--json]` | **Deterministic** next neutral citation, computed from the local citator (`.justice/INDEX.md`) or the canonical source citator (`Judicature/.justice/INDEX.md`). `tier` = `privy-council` \| `court-of-appeal` \| `supreme-court` \| `si` (the SI series numbers from the SI register). |
| `cdd check-citator` | **Deterministic** citator audit (the hard gate): fails closed on citation collisions and ruling-file/row mismatches. |
| `cdd check-bench-names [--source-only] [--corpus-only] [--json]` | **Deterministic** scan for prohibited real jurist labels such as `Hale J`, `Bingham J`, `Lord Neuberger` in judgment markdown and law-report case corpus projections. |
| `cdd lodge-judgment [--check-only] [--no-render]` | Render-and-lodge a judgment ([2026] REALM-SI 2): render PDFs (idempotent, fail-open), rebuild the corpus/index/ledger projections in lockstep (fail-open), verify the citation layer (fail-closed). |
| `cdd submit-request "<question>"` | Print a delegable workflow invocation to file a Request for Ruling. Claude Code can run the printed `Workflow` example; other agents should use their equivalent delegable workflow adapter. |
| `cdd submit-breach "<charge>"` | Print a delegable workflow invocation to file a Breach. |
| `cdd --version` | Print version. |

## Citation numbering

`lib/citation.js` is the **source of truth** for deterministic numbering: it parses the citator for the
highest `N` already issued for a tier+year and returns the next, in the operative provenance form
`[YEAR] REALM-<TIER> N` (Privy Council `REALM-PC`, Court of Appeal `REALM-CA`, Supreme Court `REALM-SC`,
statutory instruments `REALM-SI`; High Court by Division, County Court by repo `CC-<repo>`). The court
workflow scripts inline a minimal mirror of this for runtimes whose delegated workflow sandbox has no
`require` - keep them in sync.

```bash
npm test   # citation + bench-name scanner tests
npm run test:bench-names
```

*Spec is law. Rulings are precedent. Lexby is your lawyer.*
