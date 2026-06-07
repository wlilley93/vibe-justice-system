# Outstanding Work Register: Agent Lawfulness Hooks

**Date:** 2026-06-07  
**Owner:** Ministry of Justice policy arm  
**Status:** live public tracker; system-data only; not law, not a judgment, and not an order

This register tracks the remaining work needed to turn the proposed agent lawfulness hooks into a working, agent-bound system. It is deliberately public/system-data only.

## Current Position

The SI and policy briefing now exist. The hook system is made as [2026] REALM-SI 8, awaiting commencement on lapse of the Bill 14 s.14 objection window, clarified by [2026] REALM-SI 10 as an agent-agnostic workflow contract, and clarified by [2026] REALM-SI 11 as a best-efforts trigger duty for every agent. It is mechanically scaffolded for Claude-style runtimes, Codex CLI hooks, and portable `.vjs/hooks/`, but not yet automatically bound to every possible agent runtime.

Existing implementation coverage:

- Claude-style plugin hooks exist for a pre-answer advisory stub, post-answer stub, post-turn watchdog, and git gates.
- Codex CLI hooks are supported through `.codex/hooks.json`; they become automatic after Codex's hook review/trust flow.
- Gemini-style and opencode-style source-checkout adapters exist as best-efforts lifecycle bindings to the portable hook scripts.
- Git hooks protect commits and pushes only.
- The deterministic CLI gate now includes `cdd check`, which aggregates new-judgment provenance, citator consistency, and bench-name checks so hooks can call one CLI spine.
- Agent instructions now state the deterministic CLI as the required spine where a command exists:
  filings, retrieval, citation checks, graph movement, repo initialisation, judgment lodgement,
  deterministic validation, and public-release gates must go through `cdd` or an adapter invoking
  the same command unless the competent route records why the CLI is unavailable or insufficient.
- Local CI is now a CLI command, `cdd local-ci`, and the public pre-push gate invokes it before
  release-warrant validation. The VJS compliance checkpoint is local and deterministic; it does not
  depend on GitHub Actions or hosted CI. It includes public-law index consistency checks for
  repeated citations, slugs, source paths, and failed graph-validation facts.
- `cdd init` now performs a deterministic repo-root and local system-declaration preflight before installation. The preflight records or verifies `.vjs/system.json` as the local sovereign act; it is not a Repositories House approval and does not confer status by operator choice.
- The lawfulness hook proposal has been made as [2026] REALM-SI 8.
- The agent-agnostic and delegable-workflow requirement has been made as [2026] REALM-SI 10.
- The per-agent best-efforts trigger duty has been made as [2026] REALM-SI 11.
- The invented-bench-name principle is currently guidance/convention, not yet an express binding rule.
- The Gazette graph proposal has been made as [2026] REALM-SI 9, and an initial derived pointer-only graph has been built.
- The public branch sprawl has been reduced: public and development remotes now retain only `master` and
  `public-vjs-canonical-preview`, in lockstep at the same SHA. The release route remains too manual and
  has been referred for a simplifying court order.

## Open Work

| ID | Work item | Status | Owner route | Evidence / pointer |
|---|---|---|---|---|
| ALH-1 | Make or reject the Agent Lawfulness Hooks Instrument | complete | Legislature / Standing Committee | `[2026] REALM-SI 8`; `Legislature/statutes/instruments/2026-realm-si-8-agent-lawfulness-hooks.md` |
| ALH-2 | Add role-separation wording: Lexby must not perform bench/review/subagent functions alone where separation is available | complete | MoJ policy -> Standing Committee | policy briefing and SI referral updated |
| ALH-3 | Add retrieval-first wording so agents do not rely on holding the whole record in context | complete | MoJ policy -> Standing Committee | policy briefing and SI referral updated |
| ALH-4 | Bind Claude runtime hooks through `.claude/settings.json` and `.claude/hooks/` on installation | complete | MBES engineering | `Executive/plugin/settings.json`; `Executive/cli/bin/cdd.js`; `Executive/plugin/hooks/README.md` |
| ALH-5 | Decide and implement a Codex-facing binding route | complete | MBES engineering / runtime capability | `.codex/hooks.json`; `Executive/plugin/codex-hooks.json`; Codex `hooks` feature verified stable |
| ALH-6 | Extend the watchdog or add explicit pre-answer/post-answer scripts | partial | MBES engineering | non-blocking stubs added at `Executive/plugin/hooks/vjs-pre-answer.sh` and `vjs-post-answer.sh`; full deterministic law/validity engine remains future work |
| ALH-7 | Make canonical `Judicature/.justice` detectable by the watchdog, not only root `.justice/` | complete | MBES engineering | `Executive/plugin/hooks/vjs-watchdog.sh` detects root `.justice/` and `Judicature/.justice/` |
| ALH-8 | Add a deterministic or semi-deterministic bench-name scanner for judgment records | complete | MoJ / MBES engineering | `cdd check-bench-names`; aggregate `cdd check`; source judgment files and law-report case corpus projections |
| ALH-9 | Correct judgments that use real jurist names, by lawful reissue or append-with-supersede | complete | Court / registrar conformance route | `[2026] REALM-PC 17`, `18`, and `19` now use invented bench names with conformance notes; corpus regenerated |
| ALH-10 | Decide whether the invented-bench-name rule is already binding law or needs an instrument/order | open | Court or Legislature | guidance exists; binding status requires route |
| ALH-11 | Add a retrieval/index freshness check to stop agents relying on stale memory of the record | open | MBES engineering | citator, reasons ledger, and hook implementation |
| ALH-12 | Record how hooks are visibly triggered in each runtime | complete | MBES engineering / docs | Claude, Codex, Gemini-style, and opencode-style trigger points documented in `Executive/plugin/hooks/README.md` and `Executive/plugin/AGENT-ADAPTERS.md` |
| ALH-13 | Maintain an agent-agnostic adapter record for runtimes that support delegable workflows | complete | MBES engineering | `Executive/plugin/AGENT-ADAPTERS.md`; `[2026] REALM-SI 10` and `[2026] REALM-SI 11` |
| ALH-14 | Each agent must make good hook triggering on a best-efforts basis | complete | Legislature / every governed agent | `[2026] REALM-SI 11` |
| ALH-15 | Prevent direct public judgment-file creation from being mistaken for a lawful court sitting | partial | MBES engineering / Court workflow conformance | `cdd check-judgment-provenance` and aggregate `cdd check` fail newly added central judgment files without explicit court-workflow or authorised-registrar provenance; stronger workflow-run evidence remains future work |
| ALH-16 | Make `cdd init` flow through a declared system-repo and valid git-root location | complete | Existing law -> MBES engineering | Bill 30 and `[2026] REALM-PC 17` establish default local subscription on install/fork; `[2026] REALM-PC 14` and `[2026] REALM-SI 6` make formation/conformance a gate-plus-git fact. `cdd init` now requires git worktree root plus `.vjs/system.json` or `--declare-system-repo`. |
| ALH-17 | Make deterministic CLI use mandatory where the CLI supplies the filing or movement route | complete | Existing `[2026] REALM-SI 8`, `10`, `11` -> MBES adapter/agent-contract engineering | Root `AGENTS.md`, `Constitution/AGENTS.md`, `Executive/plugin/AGENTS.md`, and `Executive/plugin/AGENT-ADAPTERS.md` now require agents to use `cdd` for supported filing, retrieval, validation, init, lodgement, graph, and release-gate acts or record why the CLI route is unavailable/insufficient. |
| ALH-18 | Bind repo verification to local deterministic CI instead of hosted CI | complete | MBES engineering / public-release gate conformance | `cdd local-ci`; `Executive/plugin/hooks/vjs-pre-push.sh` runs local CI for the canonical public remote before release-warrant validation; local CI checks public-law index repetition and graph-validation status; agent contracts state that VJS compliance does not depend on GitHub Actions or hosted CI. |
| ALH-19 | Add a review route for semantic legal contradiction and redundant-law analysis | open | Court / MoJ policy / MBES tooling | `cdd local-ci` can deterministically catch duplicate identifiers and graph-validation contradictions, but it cannot prove that all legal propositions are semantically non-contradictory. A lawful review route and bounded tooling spec remain needed before this can be marked complete. |
| REL-1 | Simplify public release into a single deterministic CLI route | open | Court -> MoJ policy -> MBES engineering | Branches are now consolidated, but the release route still requires manual warrant, branch, Pages-source, live-site, and post-push handling. See `Judicature/requests/2026-06-07-branch-consolidation-release-simplification-reference.md`. |
| GAZ-1 | Make or reject the Gazette Graph Database Instrument | complete | Legislature / Standing Committee | `[2026] REALM-SI 9`; `Legislature/statutes/instruments/2026-realm-si-9-gazette-graph-database.md` |
| GAZ-2 | MBES to settle graph database technical specification and execution plan | complete | MBES engineering | derived static JSON graph documented in `Judicature/law-reports/README.md` and built by `build/build-citator-graph.js` |
| GAZ-3 | Backfill Gazette graph nodes and edges from existing public law records | complete | MBES engineering with MoJ review | `Judicature/law-reports/site/citator-graph.json` currently has 72 nodes and 752 edges |
| GAZ-4 | Add forward-facing graph metadata to new Gazette filings | partial | MBES engineering with MoJ process | build derives edges on ingestion; explicit no-edge declarations and filing validation remain open |
| GAZ-5 | Add user-facing lineage display to the Gazette | complete | MBES engineering | `Judicature/law-reports/site/app.js`; `Judicature/law-reports/site/index.html` |
| GAZ-6 | Add agent-facing CLI law search and graph retrieval | complete | MoJ request -> MBES engineering | `Judicature/ministry-of-justice/policy/2026-06-07-cli-law-search-graph-agent-retrieval-mbes-request.md`; `Executive/cli/LAW-RETRIEVAL-SPEC.md`; `cdd law ...`; `cdd graph ...` |

## Working Principles

1. Lexby is Advocate, Advisor, Engineer, and registrar when authorised. Lexby is not the bench and should not impersonate every independent checking function.
2. Where the runtime can spawn subagents or supports delegable workflows, independent review, bench, verification, or sidecar research should be delegated instead of performed only by the main Lexby thread.
3. Where the runtime cannot spawn subagents, the agent must say so and use the next-best separation available: citator lookup, deterministic checks, explicit role-labelled reasoning, and referral.
4. The agent must retrieve the relevant law and record. It must not rely on the whole VJS record being held in context.
5. Hook results are routing and safety signals. They do not adjudicate breach, punish, or create automatic invalidity.

## Immediate Next Steps

1. Obtain the branch-consolidation and release-simplification ruling, then route any MBES `cdd release`
   implementation order through MoJ policy.
2. Turn the pre-answer and post-answer stubs into proportionate deterministic checks where the runtime can support that without blocking ordinary work.
3. Settle whether the invented-bench-name rule should be made express law for future judgments.
4. Add forward-facing Gazette graph validation for new filings, including explicit no-edge declarations where a Gazette item has no public lineage.
5. Replace provenance-by-metadata with verifiable court-workflow run evidence once the court workflow runtime exposes stable run IDs or signed artefacts.
6. Settle a bounded review process for semantic contradiction and redundant-law analysis, distinct from deterministic duplicate-citation checks.

## Runtime Notes

Claude-style binding is through `.claude/settings.json` and `.claude/hooks/`. The existing plugin settings wire a Stop hook, and `cdd init` copies hook scripts and merges the settings.

Codex-style binding is through `.codex/hooks.json`. The Codex `hooks` feature is stable in the local CLI, and `cdd init` installs the bundled adapter from `Executive/plugin/codex-hooks.json` into initialized repositories. Codex still requires its hook review/trust flow before project hooks execute automatically; until trusted, Codex sessions must use instruction-level compliance plus available subagents, retrieval, and deterministic checks.

[2026] REALM-SI 10 clarifies that Claude hooks are only one adapter. The hook is a generic VJS workflow contract that must be bound, where technically possible, by each agent runtime's own hook, wrapper, instruction, skill, or plugin surface. Runtimes with delegable workflows must expose that separation for governed load-bearing work; runtimes without it must record the substitute check.

## Bench-Name Notes

The central convention is that bench names are invented and must not be real sitting or living jurists. Current public guidance says this, but current binding law appears to regulate bench size, constitution, authorship, and source of force rather than fictionalisation of names as a standalone validity rule.

The suspected public-record conformance issue is therefore not treated as voidness. It should be routed as a narrow nomenclature/conformance correction unless the competent court or Legislature makes the invented-name rule binding for future records.
