# Completion Record: Agent Law Hooks, Supreme Court Reference, and Data Boundary

**Date:** 2026-06-07  
**Owner:** Ministry of Justice policy arm  
**Status:** Completed working todo list; not an instrument of law and not a judgment.

## Immediate Priorities

- [x] **Route hooks through MoJ policy to the Legislative Branch.**
  - Draft MoJ policy briefing for an SI on agent-law hooks.
  - Refer the briefing to the Standing Committee under Bill 29.
  - Keep the SI as public principles only; no repo-specific facts, prompts, paths, secrets, hostnames, or operational thresholds on the face of the instrument.

- [x] **Draft proposed SI: Agent Lawfulness Hooks Instrument.**
  - Lexby invocation hook: enter the settled Advocate / Advisor / Engineer office before load-bearing law research.
  - Pre-answer hook: research the applicable law before answering or acting on a governed query.
  - Post-answer hook: check whether the previous answer/action was legally valid, within authority, candid, and correctly routed.
  - Private directory rule: standard root `_private/` working area, gitignored contents, tracked instructions/placeholders only.
  - Source-of-authority record: for load-bearing actions, record authority, standard, final/provisional status, reversibility, and referral need.
  - No sanctions: hooks may detect, record, warn, stop, narrow, or refer; only the Court decides breach/remedy.
  - Private facts: operational hook config stays in private registry; public law states only principles and schema.

- [x] **Send unresolved points of law to the Supreme Court.**
  - Keep repo-level local facts out of central Judicature unless anonymised or strictly system-data.
  - Ask the Supreme Court to decide the legal questions that remain after existing authorities:
    - whether repo-level formation disputes may be centrally referred without exposing private facts;
    - what the apex record may hold;
    - what a redacted/system-data case file must contain;
    - whether existing PC14/SC7/SC8 fully dispose of local formation disputes or leave residual apex questions.

- [x] **Withdraw or supersede the current Court of Appeal central request if needed.**
  - It may contain more repo-level detail than central Judicature should hold.
  - Replace it with a redacted route note or mark it superseded by the Supreme Court data-boundary/legal-points reference.

## Superrepo Data Sweep

- [x] **Sweep central Judicature requests for repo-level/private facts.**
  - Check `Judicature/requests/`.
  - Move detailed repo evidence to the local jurisdiction evidence record or root `_private/`.
  - Leave only redacted/system-data summaries centrally.

- [x] **Sweep central law/policy branches for private operational facts.**
  - Check `Constitution/`, `Judicature/`, `Legislature/`, and public-facing docs.
  - Flag local paths, hostnames, secrets, tokens, emails, account/customer facts, live config, logs, screenshots, and project-specific operational details.
  - Do not print secret values into findings.

- [x] **Produce a redacted sweep report.**
  - Public/system-data findings only.
  - Detailed hits remain in a private/local evidence record.
  - Classify each issue as remove, redact, move local, anonymise, or lawful system-data.

## Local-Repo Cleanup

- [x] **Keep detailed local formation audit local/private.**
  - Current unredacted evidence should remain in the relevant local jurisdiction evidence area or root `_private/`.
  - The public todo must not name the local repo, paths, artefacts, clients, hostnames, or operational facts.

- [x] **Replace central local-repo documents with redacted pointers.**
  - Central record should state the legal question and existence of local evidence only.
  - It should not carry repo paths beyond what is necessary, operational facts, project-specific artefact names, or private-data indicators unless anonymised.

- [x] **Resolve tracking boundary.**
  - Decide whether the local court record should live in a separate tracked, system-data-only repo, or in a forced/tracked public court-record subtree.
  - Decision: unredacted local evidence and repo-level working papers stay in the local/private record or root `_private/`; the central public record carries only redacted route notes, system-data questions, and anonymised summaries. A forced public court-record subtree is not used for private facts.

## Hook Design Notes

- [x] **Lexby invocation hook.**
  - Trigger before governed load-bearing answers/actions.
  - Invoke Lexby's persona and office: Advocate, Advisor, Engineer, officer of the Court.
  - Preserve separation: Lexby argues, advises, ships, and records; the bench decides.

- [x] **Pre-answer law research hook.**
  - Trigger on governed project work, court-routing, legislative drafting, public-record changes, external acts, security-sensitive acts, irreversible operations, or anything the agent believes is load-bearing.
  - Required output: applicable law/precedent, authority source, and whether the answer/action is ordinary, provisional, referred, or blocked.

- [x] **Post-answer validity hook.**
  - Runs after each load-bearing answer/action.
  - Checks: law cited, authority present, no unmoored extension, no repo-level private facts leaked centrally, no false finality, appeal/referral need considered, record updated if needed.
  - If invalid: record issue, correct/supersede, refer where required.

- [x] **Previous-answer review hook.**
  - At the start of a new turn, check whether the previous answer/action remained valid in light of new user instructions or newly found law.
  - If not, append-with-supersede rather than silently rewrite history.

## Open Decisions

- [x] Should the hooks be one SI or split across:
  - agent lawfulness hooks;
  - data-boundary/public-record hooks;
  - court/legislature route hooks?

- [x] Should the Supreme Court reference be filed before or after the SI policy referral?

- [x] Should the current local-repo Court of Appeal request remain as a redacted procedural step, or be superseded entirely by the Supreme Court reference on data-boundary and residual points of law?

## Completion Evidence

- MoJ policy briefing: `Judicature/ministry-of-justice/policy/2026-06-07-agent-lawfulness-hooks-si.md`.
- Standing Committee SI referral and draft instrument: `Legislature/legislature/committee/referrals/2026-06-07-agent-lawfulness-hooks-instrument-referral.md`.
- Public redacted route note: `Judicature/requests/2026-06-07-redacted-local-vjs-formation-route-note.md`.
- Public redacted Supreme Court reference: `Judicature/requests/2026-06-07-redacted-local-vjs-formation-supreme-court-reference.md`.
- Public redacted sweep report: `Judicature/ministry-of-justice/policy/2026-06-07-superrepo-data-boundary-sweep-report.md`.
- Private working area: `_private/README.md` and `_private/.gitignore`.
- Ignore boundary: `.gitignore`.

## Closed Decisions

1. The hooks proceed as one SI because the lawfulness, route, private-directory, and data-boundary checks are one operational sequence. Splitting can be reconsidered after enactment if implementation proves too broad.
2. The SI policy referral has been filed first. The Supreme Court reference is filed as a conditional/stayed apex reference after the Court of Appeal route tests the first-instance approach.
3. The local-repo Court of Appeal material is not part of the public central record. The public route note supersedes any public central version and preserves only the redacted procedural route.
4. The tracking boundary is private/local for unredacted evidence and public/central only for redacted system-data. Root `_private/` is the standard superrepo working area for private material that must not be committed as public law or public policy.
