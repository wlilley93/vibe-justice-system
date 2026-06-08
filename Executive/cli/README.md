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
| `cdd check [--json]` | Run the deterministic repo gate: new-judgment provenance, citator consistency, and bench-name scan. Hooks call this aggregate command so the CLI is the single deterministic spine. |
| `cdd local-ci [--json]` | Run local, no-hosted-CI VJS verification: syntax checks, JSON parse checks, public-law index consistency, CLI tests, deterministic aggregate checks, law/graph smoke checks, and whitespace checks. Public-release hooks call this before warrant validation. |
| `cdd init [dir] --declare-system-repo` | Install VJS into a git repo root after recording the local system-repo declaration: vendor `CASE-LAW.md` / `VPR.md` / `CDD.md`, scaffold local `.justice/` (citator, `judgments/`, `caselaw/`, `pdfs/`, `suites/`), append the generic `Executive/plugin/AGENTS.md` contract to `AGENTS.md`, install portable scripts under `.vjs/hooks/`, and bind the bundled Claude, Codex, Gemini-style, and opencode-style adapters through `CLAUDE.md` / `.claude/settings.json` / `.codex/hooks.json` / `.gemini/settings.json` / `.opencode/plugins/vjs-lawfulness.js` (idempotent). |
| `cdd next-citation <tier> [--year YYYY] [--citator PATH] [--json]` | **Deterministic** next neutral citation, computed from the local citator (`.justice/INDEX.md`) or the canonical source citator (`Judicature/.justice/INDEX.md`). `tier` = `privy-council` \| `court-of-appeal` \| `supreme-court` \| `si` (the SI series numbers from the SI register). |
| `cdd check-citator` | **Deterministic** citator audit (the hard gate): fails closed on citation collisions and ruling-file/row mismatches. |
| `cdd check-bench-names [--source-only] [--corpus-only] [--json]` | **Deterministic** scan for prohibited real jurist labels such as `Hale J`, `Bingham J`, `Lord Neuberger` in judgment markdown and law-report case corpus projections. |
| `cdd check-judgment-provenance [--json]` | **Deterministic** scan for newly added central judgment files without explicit court-workflow or authorised-registrar provenance metadata. |
| `cdd lodge-judgment [--check-only] [--no-render]` | Render-and-lodge a judgment ([2026] REALM-SI 2): render PDFs (idempotent, fail-open), rebuild the corpus/index/ledger projections in lockstep (fail-open), verify the citation layer (fail-closed). |
| `cdd law search "<query>" [--kind case\|bill\|si\|all] [--limit N] [--json]` | Token-efficient search over `site/search-index.json`. Results are pointer records only and are retrieval aids, not legal force. |
| `cdd law get "<citation\|id>" [--include-source] [--max-chars N] [--json]` | Resolve one public law pointer. Source text is omitted by default and bounded when explicitly requested. |
| `cdd graph node "<node\|citation>" [--json]` | Resolve one public Gazette graph node from `site/citator-graph.json`. |
| `cdd graph edges "<node\|citation>" [--dir in\|out\|both] [--type TYPE] [--limit N] [--json]` | Return bounded adjacent graph edges with compact neighbouring node summaries and `briefWhy`. |
| `cdd release-warrant [--remote-url URL] [--remote-ref REF] [--local-sha SHA] [--json]` | Retrieve push/release warrant evidence for a proposed public VJS push. Aliases: `cdd push-licence`, `cdd push-license`. |
| `cdd submit-request "<question>"` | Print a delegable workflow invocation to file a Request for Ruling. Claude Code can run the printed `Workflow` example; other agents should use their equivalent delegable workflow adapter. |
| `cdd submit-breach "<charge>"` | Print a delegable workflow invocation to file a Breach. |
| `cdd --version` | Print version. |

## Agent law retrieval

MBES Engineering implements the MoJ request for agent-facing retrieval in
[`LAW-RETRIEVAL-SPEC.md`](LAW-RETRIEVAL-SPEC.md). Agents should prefer these commands before
load-bearing answers about VJS law or routing:

```bash
cdd law search "superrepo court order" --limit 5 --json
cdd law get "[2026] REALM-SI 7" --json
cdd graph node "si:7" --json
cdd graph edges "si:7" --dir both --limit 10 --json
```

The commands read the search index and graph projection, not the whole public corpus. The canonical
law remains the committed source record; CLI output is a retrieval pointer.

## Init preflight

`cdd init` must run at a git worktree root. It refuses subdirectories because repository inclusion and
conformance are gate-plus-git facts, not loose folder facts. A repo must also declare that it is a VJS
system repo. Pass `--declare-system-repo` on first install to write `.vjs/system.json`; later installs
reuse that declaration. The file records a local sovereign act and the authority path. It is not a
Repositories House approval and does not confer status by operator choice.

## Citation numbering

`lib/citation.js` is the **source of truth** for deterministic numbering: it parses the citator for the
highest `N` already issued for a tier+year and returns the next, in the operative provenance form
`[YEAR] REALM-<TIER> N` (Privy Council `REALM-PC`, Court of Appeal `REALM-CA`, Supreme Court `REALM-SC`,
statutory instruments `REALM-SI`; High Court by Division, County Court by repo `CC-<repo>`). The court
workflow scripts inline a minimal mirror of this for runtimes whose delegated workflow sandbox has no
`require` - keep them in sync.

```bash
cdd local-ci
cdd check
npm test   # citation + bench-name scanner + law lookup + judgment provenance tests
npm run test:bench-names
```

*Spec is law. Rulings are precedent. Lexby is your lawyer.*
