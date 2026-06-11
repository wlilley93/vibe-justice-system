# Self-reported breach: a blanket rewrite reached into a nested estate's tree

**Date:** 2026-06-11
**Standard:** reasonable skill and care (raised to all-reasonable-endeavours by the engagement)
**Self-reported, cured.** No punishment; the remedy is to make the work good (done).

## What happened
Executing the County Court shadow order 2026-VJS-CC-JURISDICTION-REID-001 (re-identify
the jurisdiction id agent-universe-v2 -> vibe-justice-system), I ran a blanket
`grep -rl agent-universe-v2 . | sed -i` over the whole working directory. The canon repo
CONTAINS a nested ministry tree, `Executive/.../projects/opbox`, which is a SEPARATE live
git repo (the opbox subscriber, with its own active agent). The recursive sweep entered it
and rewrote 495 of opbox's files - another estate's working tree, which the federation
discipline forbids touching. `git add -A` then tried to stage opbox as an embedded repo,
and the push failed.

## Why it fell below standard
The one-writer / no-entry-into-another-estate's-tree rule is settled. A recursive operation
over `.` in a repo that holds nested estate repos is exactly the footgun the rule guards
against. I should have scoped the sweep to the canon's own paths from the start.

## Cure (done)
- Classified the 495 opbox files: 33 were my rewrite (32 the opbox agent never touched, 1 it
  also edited), the rest pure opbox-agent work.
- Restored the 32 from opbox HEAD; surgically restored the 1 mixed file; left the agent's
  ~462 files of legitimate work untouched. Verified ZERO residue and opbox HEAD unchanged.
- Added `Executive/` to the canon's `.gitignore` so the canon never tracks the nested repos,
  and so a future `git add -A` cannot stage them.
- Re-ran the re-identification scoped correctly; canon law + published surface are clean.

## The rule going forward
Recursive text operations in the canon repo MUST exclude `Executive/` (and any nested estate
repo). Prefer `git grep` (respects the repo boundary and .gitignore) over `grep -r .`.
