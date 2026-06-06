# Handover - Agent Universe genesis (2026-06-05)

Session that (1) ran the Harvey LAB legal-agent benchmark to **48/50**, (2) built the VJS governance edifice
(CASE-LAW s. 17-22 + a full caselaw chain), and (3) materialised the realm at `~/agent-universe`.

## 1. The benchmark (the proof point)

- **Task:** `corporate-ma/review-data-room-red-flag-review` (1 of 1,251 LAB tasks). VJS-court-augmented agent, run
  **in-session** via the Workflow tool (not a subprocess - that 429'd on the OAuth pool).
- **Score: 48/50 criteria** (`all_pass` false; 2 misses: C-046 top-10 concentration %, C-048 add-back itemisation).
  All 13 planted issue clusters found; all 4 distractors correctly cleared. Self-marked in-session (FI-2 marker room).
- **Deliverables + records** live under
  `ministry-for-business-work-and-skills/legal-department/harvey-labs/results/corporate-ma/.../20260605-benchmark/`:
  `output/red-flag-memo.docx` + `red-flag-tracker.xlsx`; `scores.json`; `legal-analyses/RIDGELINE-ISSUE-01..16.md`
  (the 16 product courts, full deliberations); `scoring/MARKERS.md` (the 50 markers); `court_rulings.json`.
- **Harvest:** `harvey-labs/harness/harvested-skills/HARVEST-run1.md` - 8 generic, fact-free method learnings, all
  court-admitted to the **skills** catalogue (FI-1 provenance-clean).

## 2. The governance built this session (binding)

CASE-LAW (in `CASE-LAW.md`): **s. 17** (duty to self-appeal unmoored rulings) · **s. 18** (odd benches; judgment from
within the panel; the Aldermere-on-top synthesiser abolished) · **s. 21** (divisions over one spine; product is not
governance) · **s. 22** (court geography: local hearing-centres under a single central apex; the realm-as-state).
The full caselaw chain (INC-1 -> CA-1 void -> SC-3 -> FI-1..4 -> SC-DC 1 -> SC-2) is in
`harvey-labs/harvey-caselaw/` and indexed in the **universal ledger** (`ministry-of-justice/ledger/INDEX.md`, 21 rulings).

Key doctrines for anyone picking this up: **anti-cheat = HOW (generic method) admissible, WHAT (rubric/answers)
prohibited** (SC-3); the agent never sees the rubric, the marker may (FI-2); harvest only fact-free, provenance-clean
method (FI-1); a practitioner **treatise** is permitted in the skills `references/` tier, chapters per capability,
provenance-clean (FI-4).

## 3. The realm (what moved where)

`~/agent-universe` is a **private fork of vibe-justice-system** (`upstream` remote = the public VJS, which **stays** in
`~/Projects/vibe-justice-system`). The VJS law sits at the realm root (clean upstream sync). Structure + signposts:
read `README.md` (the realm map) and `constitution/REALM-TOPOLOGY.md`.

All `~/Projects` repos were git-safe-moved into departments (history + remotes intact, `.git` preserved):
- **Legal Department** (Legal Division ≈ Chancery): `harvey-labs/` (+ `references/mike`).
- **Engineering Department**: `projects/{acmeco,Operator,Onyx,fleetco-agent,Jarvis,jarvis-voice}`.
- **Skills & Education**: `scratch-to-signals`.  **Business Operations**: `Clara`.  **National Archives**:
  `archive, acmeco-legacy, ldd-plugin`.
The realm fork's `.gitignore` excludes these nested repos - they remain independent repos (push to their own remotes
for backup). The realm repo tracks governance + structure + signposts + the ledger only.

## 4. State at handover

- **Symlinks removed.** The transition symlinks at `~/Projects/<repo>` have been removed; the canonical location is now
  `~/agent-universe/...`. **Consequence to fix next session:** external references to old `~/Projects/<repo>` paths
  (MEMORY.md links, CLAUDE.md file-placement, deploy scripts, absolute paths in `harvey-labs/harness/*.workflow.js`)
  now point at nothing and should be repointed to the realm paths.
- **Private repo:** the entire `agent-universe` realm fork is committed and pushed to the private GitHub repo (see
  `git -C ~/agent-universe remote -v`).

## 5. Open / next items

- **Flagged repo placements (reversible):** Clara (business-ops vs Legal Division Probate); mike (legal reference vs
  archive). Confirm and re-sort if desired.
- **Path repointing:** update `~/Projects` references and the few absolute paths in the harness workflows.
- **The treatise (FI-4):** start Chapter 1 - Corporate Due Diligence - in the Legal Department's references tier, from
  the 16 product records + the harvest (provenance-clean only).
- **Future ministries:** Home Office, Ministry of Defence, the Sovereign/parliament layer - scaffolded empty, develop later.
- **VJS public fork:** decide whether to publish a public VJS for general use (the private realm is `agent-universe`).
- **Ledger automation:** wire `ministry-of-justice/ledger/build-ledger.py` to cron or a Haiku clerk.

## 6. How to navigate (never get lost)

Start at `~/agent-universe/README.md` -> follow `_signpost.md` down to any node, or up to `CASE-LAW.md` (the one law) and
`ministry-of-justice/ledger/INDEX.md` (every case). Each repo carries `.justice/SIGNPOST.md` pointing home.
