# Canonicalisation Migration Runbook + Release Warrant (Principal's warranted acts)

**Authority:** Realm Consolidation and Reconciliation Framework Act s.17, s.22; REG-CANONICALISATION-MIGRATION-001; [2026] VJS-PC 2; [2026] REALM-SI 7 (release warrant); [2026] REALM-PC 16/20 (post-push review).
**Status:** runbook for the Principal to execute. The agent does not push, rename, tag, or re-licence (outward acts). Every step is lock-preserving; **halt and return if any invariant check fails.**

---

## 0. Release warrant (fill + record before pushing)

```
remote:   git@github.com:wlilley93/vibe-justice-system.git   # the canonical line
refs:     v2-agent-harness-doctrine -> main (canon);  bill-32-computer-first-realm-founding -> v1 archive
SHAs:     <fill: git rev-parse HEAD on each branch>
boundary: public/system-data only (run: vjs local-ci -> boundary_scan PASS)
warrant:  granted by the Sovereign Founder, 2026-06-09 ("assent given, go")
```

## 1. Pre-flight (prove the baseline)
```bash
cd ~/Projects/agent-universe-v2
git rev-parse HEAD                                  # record canon SHA
./target/debug/vjs local-ci                         # must PASS (incl. boundary_scan)
./target/debug/vjs validate --all || ./target/debug/vjs validate --staged   # 20 invariants pass
# confirm the fresh lock matches what is recorded in COMMENCEMENT-V2-0002:
find lawpack/v2 -type f | LC_ALL=C sort | xargs sha256sum | sha256sum   # expect 8d075f7d...
```

## 2. Push the two estates (no rename yet)
```bash
# canon (V2 runtime + lawpack + kernel) -> the canonical repo's main
git push git@github.com:wlilley93/vibe-justice-system.git v2-agent-harness-doctrine:refs/heads/main-canon-staging
# V1 back-fill (s.23 + SC-10 minute) -> the V1 line
cd ~/Projects/vibe-justice-system
git push origin bill-32-computer-first-realm-founding
```
> Push to a **staging** branch first (`main-canon-staging`), verify on GitHub, then fast-forward `main`. Do not force-push; the canon is append-only.

## 3. Establish the V1 archive estate (immutable tag + protected branch)
On the canonical repo (`vibe-justice-system`), with the current V1 tree:
```bash
git tag -s v1-archive-2026-06-09 -m "V1 Archive estate (immutable); see CASE-LAW s.23, REALM-SC 10 derived minute"
git push origin v1-archive-2026-06-09
git branch v1 v1-archive-2026-06-09 && git push origin v1
```
Then in GitHub settings: **protect** the `v1` branch (read-only, no force-push, no deletion); the tag is the estate anchor. The V1 content renders as a distinct **Archive** directory in the Gazette build.

## 4. Make the canon the default line (the "drop v2" step)
- `vibe-justice-system` is already free of the "v2" ordinal -> **no repo rename is needed** for the canon; the spent "v2" lives only in `agent-universe-v2`.
- Set `main` = the canon (fast-forward from `main-canon-staging` once verified).
- **`agent-universe-v2`:** archive it on GitHub (Settings -> Archive) and add a README pointer/redirect to `vibe-justice-system`. Its history is preserved; this is the inbound redirect PC-2 required. Do **not** delete it.

## 5. Re-prove invariants after the move (the halt gate)
```bash
# in the canonical repo, after main = canon:
vjs local-ci            # MUST PASS; boundary_scan, invariant_eval, citation_check all green
vjs validate --all      # 20 invariants still bind a non-empty set
```
**If any fatal invariant no longer binds, or a path-scoped glob no longer resolves: STOP, revert `main`, keep the two repos, and re-refer to the Privy Council.** "Better two repos than a fail-open lawpack."

## 6. Fresh lock + AGPL full text
```bash
# record the post-move lock (a fresh COMMENCEMENT addendum), old locks left pinned:
find lawpack/v2 -type f | LC_ALL=C sort | xargs sha256sum | sha256sum
# vendor the full verbatim GNU AGPL-3.0 text into LICENSE:
curl -fsSL https://www.gnu.org/licenses/agpl-3.0.txt -o LICENSE.agpl   # then prepend the adoption record
```

## 7. Post-push review (mandatory)
Convene a Privy Council post-push review ([2026] REALM-PC 16/20): confirm the public boundary held, the graph resolves in one place, the V1 estate is read-only, and the canon is on `main`. Record it.

---

## What is already done (no action needed)
- The law is **lodged and live** in the runtime lawpack (Framework Act + 6 regulations, `vjs local-ci` PASS).
- AGPL **adopted** in-repo (LICENSE + README); only the full-text vendor (step 6) remains.
- The V1 record is **whole** (CASE-LAW s.23 transcribed; REALM-SC 10 derived minute).
- The **fresh lock** is recorded; the old lock + assented digests are pinned and untouched.

## Still outstanding beyond this runbook
- Each repository's **mandatory subscription** within 90 days (REG-REPOS-REGISTER-001).
- **Schedule-3 items 2-6** (SIs `made`->`in-force`; s.19/s.20 classification; Bill 13 title; store-of-authority question; Bill 11 vote) - each with a machine-checkable predicate.
- Task #15: route-engine on-point-silence detection (a code follow-up).
