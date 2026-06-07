# STRUCTURE.md - the four-branch layout and its path couplings (maintainer notes)

The repository is organised into four top-level branches mandated by the VJS (Constitution and Machinery)
Act 2026 (Bill 27): **Constitution / Judicature / Legislature / Executive**. Top-level holds only those four
branches plus the GitHub and agent-adapter files (`README.md`, `AGENTS.md`, `.gitignore`,
`.github/`, `.vjs/`, `.claude/`, `assets/`).

## Where the machinery lives (and the path couplings to respect)

The deterministic spine spans branches, so several scripts hold cross-branch paths. When moving anything,
keep these in sync (the pre-commit gate + the build/ledger/render suite will catch breakage):

- **The citator + judgments:** `Judicature/.justice/` (central courts only). `Executive/cli/lib/citator-audit.js`
  finds the repo root by the presence of `Judicature/.justice/` and reads judgments from there.
- **The cdd CLI + gate:** `Executive/cli/` (engine + audit) and `Executive/plugin/hooks/vjs-pre-commit.sh`
  (the hard gate + the lockstep projection rebuild). `.git/hooks/pre-commit` is a symlink to
  `../../Executive/plugin/hooks/vjs-pre-commit.sh` - re-point it if `plugin/` moves. The hook greps staged
  paths `^Judicature/.justice/...` and `^Legislature/legislature/bills/` and rebuilds the projections.
- **The law-reports build:** `Judicature/law-reports/build/` computes `ROOT` as the repo root and scans
  `Judicature/.justice` + `Legislature/legislature/bills`; outputs `corpus.json` + `site/search-index.json`
  under `Judicature/law-reports/`. It is **central-courts only** (Bill 27 s. 14): local CC-<repo> / Division
  judgments are excluded from the public projections.
- **The ledgers:** `Judicature/ministry-of-justice/{ledger,reasons-ledger}/` anchor on the `Judicature`
  branch (`parents[2]`); the reasons-ledger reaches the bills cross-branch via `REALM.parent/Legislature/...`
  and its `SIGNIFICANT_PATH_PREFIXES` are branch-prefixed. Both are central-only.
- **The renderers:** `Judicature/court/renderer/` (judgments) and `Legislature/legislature/renderer/` (bills +
  SIs). The legislature renderer reuses the court renderer's Chromium via a cross-branch path
  (`../../../Judicature/court/renderer/node_modules/puppeteer`) - a fragile coupling; if `court/` moves, fix it.

## Public vs private

The public tree is **system data only** (Bill 27). Personal/operational data lives in separate, gitignored
repos: the Executive ministries' operational repos and the Ministry of Data Security's private estate registry
(`Executive/ministry-of-data-security/_private-estate/`, gitignored - holds the facts the principles-only
`[2026] REALM-SI 1` deliberately omits). See `.gitignore`.

## Verifying the spine after any structural change

From the repo root: `node Executive/cli/bin/cdd.js check-citator` (gate) · `node Executive/cli/lib/citation.test.js`
· `node Judicature/law-reports/build/ingest.js && node Judicature/law-reports/build/build-search-index.js`
· `python3 Judicature/ministry-of-justice/ledger/build-ledger.py` · `python3
Judicature/ministry-of-justice/reasons-ledger/build-reasons-ledger.py` · render-test both renderers.
