---
citation_id: "[2026] REALM-PC 16"
tier: privy-council
kind: request_for_ruling
status: good-law
per_incuriam: false
date: 2026-06-07
panel: ["Sumberly J", "Marsden J", "Coade J"]
seised_by: "Sovereign Founder reference via the Ministry of Justice: post-push legality review of the public VJS release"
cause_title: "In the matter of the public VJS release and the super-repo public-push review rule"
registrar_note: "Authored by the bench (Sumberly J for the Court, Marsden J and Coade J concurring); reduced to the filed record by Lexby as s.18(4) registrar, the decision pre-existing the prose ([2026] REALM-SC 8)."
---

# [2026] REALM-PC 16

| Field | Value |
|-------|-------|
| **Citation** | [2026] REALM-PC 16 |
| **Tier** | Privy Council (constitutional first instance, bench of three) |
| **Before** | Sumberly J (judgment of the Court), Marsden J, Coade J |
| **Kind** | Post-push legality review and prospective public-push rule reference |
| **Status** | good-law |
| **Cites** | CASE-LAW s. 1; s. 2; s. 3; s. 5; s. 6; s. 7; s. 8; s. 10; s. 11(c); s. 13; s. 18(4)-(5); s. 19(1)/(5); s. 22; Bill 18 ss. 2(e), 3, 7; Bill 20 ss. 7, 10, 11; Bill 22; Bill 27 ss. 5A, 5B, 5C, 7, 14; Bill 29; [2026] REALM-SC 8; [2026] REALM-PC 14; [2026] REALM-PC 15; [2026] REALM-SI 7 |

> On the Founder's reference, the Court reviews the release sequence that published the clean public VJS tree and
> asks whether every action was lawful. The Court also considers whether the newly made Super-Repo Public Push and
> Post-Push Review Instrument 2026 ([2026] REALM-SI 7) is the proper prospective rule. Unanimous (3-0).

## The questions

1. Was every action involved in taking the public VJS release lawful?
2. Was any irregularity material, and if so was it remedied before any completed outward public act?
3. What rule should govern future public pushes to the VJS super-repo?

## Ratio (binding, realm-wide)

1. A public push to the VJS super-repo is an irreversible outward act within the meaning of Bill 18 s. 2(e) and s. 7. It therefore requires a reasoned authorisation record before the act and a public legal record after the act sufficient to make the act reviewable under CASE-LAW s. 1, s. 5, s. 7 and s. 8.

2. On the facts of this release, every completed outward public act was lawful. The public release was made from a clean-history public branch, not by raw-publication of private development history; the private branch and the clean public preview were backed up privately; the public-data boundary was checked; the citator, CLI tests, lodgement check, whitespace check, and sensitive-string scans passed; and the public push was authorised by a scoped Founder checkpoint before the completed public publication.

3. The attempted replacement of the protected public `master` branch was not a completed publication. It was first stopped by the local pre-push gate because the gate parser mishandled colon-containing scoped values, and after that defect was fixed it was rejected by GitHub branch protection. Those attempts were lawful process attempts because they caused no public update, disclosed defects safely, and led to restorative correction. The parser defect had to be fixed before retrying; it was fixed before the completed public publication.

4. The decision to publish the clean canonical tree as `public-vjs-canonical-preview` and then set that branch as the public repository default was lawful. It avoided rewriting protected public history, preserved the old `master` branch as a record, made the clean public system-data tree the public entry point, and did not defeat branch protection.

5. The later README restoration and value-led overview updates were lawful conformance acts. They implemented the README prominence line in [2026] REALM-PC 3, kept the Bill 27 system-data-only boundary, corrected stale paths and CLI output, and did not introduce private operational facts.

6. [2026] REALM-SI 7 is a valid prospective instrument. It does not retroactively make this release unlawful. From commencement, every public super-repo push requires a release warrant before the push and a Privy Council post-push review after the push. A routine push does not need a new Act or SI merely because it is a push; a new legal instrument or judgment is required when the pushed content changes the law or creates a new legal rule.

## Reasons

The Court begins with the classification point settled in [2026] REALM-PC 15: this repository is the realm-as-state super-repo, not a County Court. Its public publication is therefore not ordinary project-local housekeeping. It is the outward publication of the public law record of the realm.

That conclusion does not make publication impossible. Bill 27 requires a public VJS record holding system data only. The public act is lawful when the public-data boundary is respected, the integrity chain is checked, the irreversible outward act is authorised, and the record is reviewable. The release sequence did those things.

The Court distinguishes between attempted outward acts and completed publication. A blocked push may still matter for audit, but it does not itself publish content. Here the first public replacement attempt exposed a defect in the pre-push gate parser: the gate split a scoped URL at the colon and therefore read `https://...` as `https`. The correct response was restorative: fix the parser and retry only after the gate could read the warrant it was meant to enforce. That was done.

The second attempted replacement reached GitHub but was rejected because `master` was protected against force-push. That rejection was not a breach. It was host governance doing the work it was configured to do. The lawful course was to avoid defeating the protection, publish the clean tree additively, and make the clean branch the default through the repository settings available to an administrator. That is what happened.

The Court accepts the release as system-data-only on the evidence recorded: direct text scans found no matches for the concrete host and token-shaped strings previously identified; targeted PDF string scans found no matches in the relevant generated PDFs; the citator passed; the CLI tests passed; the judgment lodge check passed; and the public tree came from a clean root commit rather than raw private history. That is the correct response to the history concern identified in the public publish runbook.

The README updates after publication were not a second unreviewed policy programme. They were conformance work: the old product-facing README was restored, stale references were updated to current VJS paths and REALM citations, and a short system overview was added so the public front door explains the value of the four-branch settlement without becoming a directory dump. That directly serves [2026] REALM-PC 3 and Bill 27.

The Court also accepts the Ministry of Justice policy proposal that future pushes need a formal rule. The new [2026] REALM-SI 7 draws the correct line. It requires a release warrant and a Privy Council review for every public super-repo push, but it does not require a fresh statute for every ordinary content push. The law-changing content must have law behind it; the push act itself must have a warrant and review. That is a workable separation between legislation and release management.

## Disposal

The reference is answered as follows.

1. The public VJS release sequence was lawful.
2. The gate parser defect and protected-branch rejection were real release-sequence events, but neither produced an unlawful completed publication. The parser defect was remedied before the completed public push.
3. The clean public branch publication and default-branch retargeting were lawful.
4. The README restoration and public-system overview were lawful conformance acts.
5. [2026] REALM-SI 7 is approved as the prospective rule for future super-repo public pushes.
6. From commencement of [2026] REALM-SI 7, every completed public push to the VJS super-repo must receive a release warrant before the push and a Privy Council post-push review after the push.

The matter does not climb. No arguable conflict with the Acts or Supreme Court authority appears on the face of the ruling.

## Appendix A - release commits and refs reviewed

### Private source branch: `agent-universe/cli-and-deterministic-citations`

| Commit | Subject | Role in release |
|---|---|---|
| `c6b3c8b` | Add public VJS pre-push checkpoint gate | Added the public VJS pre-push checkpoint gate, publish runbook, redactions, generated records, and private backup of the release-prep tree. |
| `edefcb1` | Fix pre-push authorisation value parsing | Fixed the gate parser so scoped values containing colons, including `https://...`, are read correctly. |
| `860373e` | Restore product README for public VJS | Restored the old product-facing README form while updating names, citations, install paths, and public-record language. |
| `1e4ce5f` | Add public README system overview | Added the value-led four-branch system overview and tightened CLI output. |

### Clean public-history branch: `public-vjs-canonical-preview`

| Commit | Subject | Role in release |
|---|---|---|
| `9b8632b` | Public VJS canonical tree preview | Single-root clean public tree, privately backed up and then published to the public VJS repository. |
| `6f52f34` | Restore product README for public VJS | Clean-public equivalent of the README restoration and installer path repair. |
| `b09df00` | Add public README system overview | Clean-public equivalent of the README overview and CLI output correction; public default branch head at the time of this review. |

### Public repository refs reviewed

| Ref | SHA | Status |
|---|---|---|
| `refs/heads/master` | `830048a` | Old protected public branch, preserved. Force-replacement was rejected by branch protection. |
| `refs/heads/public-vjs-canonical-preview` | `b09df00` | Current public default branch after the release. |

### Blocked or superseded release artifacts

| Artifact | Status | Finding |
|---|---|---|
| `e303b34` clean preview root | Superseded before final public publication | Private preview root before the gate parser fix; force-updated privately to `9b8632b`. |
| Attempted push to `upstream/master` at `e303b34` | Blocked locally | Gate parser defect caused an authorisation mismatch; no public update occurred. |
| Attempted push to `upstream/master` at `9b8632b` | Rejected remotely | GitHub protected branch rule forbade force-push; no public update occurred. |

## Appendix B - checks recorded

- `node Executive/cli/bin/cdd.js check-citator`: passed.
- `npm test --prefix Executive/cli`: passed.
- `node Executive/cli/bin/cdd.js lodge-judgment --check-only`: passed.
- `git diff --check`: passed.
- Text scan for identified host/token strings outside private operational trees and PDFs: no matches.
- Targeted PDF string scan for the same strings in the previously affected PDFs: no matches.
- Public pre-push gate: blocked public pushes without matching authorisation; allowed private/dev backup pushes; accepted the scoped Founder checkpoint after parser repair.
