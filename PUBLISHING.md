# Publishing flow

This repo is **public** and GitHub Pages serves the Gazette from `vjs-canonical`. On a public repo, push = publish and git history is forever, so the branches have distinct jobs and one direction of flow:

```
(drafting, PRIVATE)  ->  master (integration, public)  ->  vjs-canonical (published surface)
   the dev repo            merges land here                  moves ONLY by promotion
```

## The branches

| Branch | Where | Role |
|---|---|---|
| working branches (e.g. `v2-agent-harness-doctrine`) | **`wlilley93/vibe-justice-system-dev` (private) + local** | drafting. Never pushed to the public repo. |
| `master` | public | the integration line. Day-to-day work merges here first. Protected: no force-push, no deletion. |
| `vjs-canonical` | public, **default branch, Pages source** | the deliberately published surface. Moves only by `scripts/promote-canonical.sh`, fast-forward only. Protected: no force-push, no deletion. |
| `v1` | public | the frozen honoured archive. Never moves. |

## The rules

1. **Draft privately.** Working branches live locally and on the private dev remote (`git push dev <branch>`). They are never pushed to the public origin: a draft on a public repo is published the moment it lands, and history retention makes that irreversible (the SC-11 exposure is the precedent).
2. **Merge to `master` first.** Integration happens on master under the ordinary gates (pre-commit `vjs validate --staged`, pre-push `vjs local-ci`).
3. **Promote deliberately.** `vjs-canonical` only ever fast-forwards to a reviewed master commit, via `scripts/promote-canonical.sh`, which runs the **boundary pass** over everything the promotion would newly publish:
   - private-record paths (`.vjs/private/`, env files, key material),
   - high-confidence secret patterns in added lines,
   - the hashed publication denylist (`.vjs/publication-denylist.txt`, the SC-11 class: private identifiers never appear in plaintext, not even in the check),
   - dev-machine absolute paths,
   - `vjs validate`.
4. **Never force-push canonical.** Branch protection enforces it (admins included). If the promotion is not a fast-forward, the answer is to fix master, never to rewrite the published surface.

## Why

"Canonical" must mean something: a name that sits at the same tip as the working branch is decoration. This flow makes `vjs-canonical` a **promotion**, with the publication-boundary duty (ACT-005; the gazette publication gate; BREACH-2026-06-10-client-data-published and its cure) discharged at the exact moment publication happens. Publication remains constitutively inert (REG-GAZETTE-CONTINUITY-001) - the record binds, the rendering informs - but exposure is real, and the boundary duty attaches to the act of exposing.

## Quick reference

```bash
# drafting
git push dev <working-branch>          # backup/share drafts privately

# integrating
git checkout master && git merge --ff-only <working-branch>
git push origin master                 # public, gated by the hooks

# publishing
./scripts/promote-canonical.sh        # boundary pass + ff-only promotion + Pages rebuild
```
