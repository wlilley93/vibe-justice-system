# Outstanding Work Register: Agent Lawfulness Hooks

**Date:** 2026-06-07  
**Owner:** Ministry of Justice policy arm  
**Status:** live public tracker; system-data only; not law, not a judgment, and not an order

This register tracks the remaining work needed to turn the proposed agent lawfulness hooks into a working, agent-bound system. It is deliberately public/system-data only.

## Current Position

The SI and policy briefing now exist. The hook system is made as [2026] REALM-SI 8, awaiting commencement on lapse of the Bill 14 s.14 objection window, clarified by [2026] REALM-SI 10 as an agent-agnostic workflow contract, and clarified by [2026] REALM-SI 11 as a best-efforts trigger duty for every agent. It is mechanically scaffolded for Claude-style runtimes and portable `.vjs/hooks/`, but not yet automatically bound to every agent runtime.

Existing implementation coverage:

- Claude-style plugin hooks exist for a pre-answer advisory stub, post-answer stub, post-turn watchdog, and git gates.
- Git hooks protect commits and pushes only.
- This Codex session is not currently bound to a visible runtime hook runner.
- The lawfulness hook proposal has been made as [2026] REALM-SI 8.
- The agent-agnostic and delegable-workflow requirement has been made as [2026] REALM-SI 10.
- The per-agent best-efforts trigger duty has been made as [2026] REALM-SI 11.
- The invented-bench-name principle is currently guidance/convention, not yet an express binding rule.
- The Gazette graph proposal has been made as [2026] REALM-SI 9, and an initial derived pointer-only graph has been built.

## Open Work

| ID | Work item | Status | Owner route | Evidence / pointer |
|---|---|---|---|---|
| ALH-1 | Make or reject the Agent Lawfulness Hooks Instrument | complete | Legislature / Standing Committee | `[2026] REALM-SI 8`; `Legislature/statutes/instruments/2026-realm-si-8-agent-lawfulness-hooks.md` |
| ALH-2 | Add role-separation wording: Lexby must not perform bench/review/subagent functions alone where separation is available | complete | MoJ policy -> Standing Committee | policy briefing and SI referral updated |
| ALH-3 | Add retrieval-first wording so agents do not rely on holding the whole record in context | complete | MoJ policy -> Standing Committee | policy briefing and SI referral updated |
| ALH-4 | Bind Claude runtime hooks through `.claude/settings.json` and `.claude/hooks/` on installation | complete | MBES engineering | `Executive/plugin/settings.json`; `Executive/cli/bin/cdd.js`; `Executive/plugin/hooks/README.md` |
| ALH-5 | Decide and implement a Codex-facing binding route | partial | MBES engineering / runtime capability | root `AGENTS.md` gives instruction-level binding; no automatic `.codex` hook runner found |
| ALH-6 | Extend the watchdog or add explicit pre-answer/post-answer scripts | partial | MBES engineering | non-blocking stubs added at `Executive/plugin/hooks/vjs-pre-answer.sh` and `vjs-post-answer.sh`; full deterministic law/validity engine remains future work |
| ALH-7 | Make canonical `Judicature/.justice` detectable by the watchdog, not only root `.justice/` | complete | MBES engineering | `Executive/plugin/hooks/vjs-watchdog.sh` detects root `.justice/` and `Judicature/.justice/` |
| ALH-8 | Add a deterministic or semi-deterministic bench-name scanner for judgment records | open | MoJ / MBES engineering | source judgment files under `Judicature/.justice/judgments/` |
| ALH-9 | Correct judgments that use real jurist names, by lawful reissue or append-with-supersede | complete | Court / registrar conformance route | `[2026] REALM-PC 17`, `18`, and `19` now use invented bench names with conformance notes; corpus regenerated |
| ALH-10 | Decide whether the invented-bench-name rule is already binding law or needs an instrument/order | open | Court or Legislature | guidance exists; binding status requires route |
| ALH-11 | Add a retrieval/index freshness check to stop agents relying on stale memory of the record | open | MBES engineering | citator, reasons ledger, and hook implementation |
| ALH-12 | Record how hooks are visibly triggered in each runtime | partial | MBES engineering / docs | Claude trigger points documented in `Executive/plugin/hooks/README.md`; Codex binding still unresolved |
| ALH-13 | Maintain an agent-agnostic adapter record for runtimes that support delegable workflows | complete | MBES engineering | `Executive/plugin/AGENT-ADAPTERS.md`; `[2026] REALM-SI 10` and `[2026] REALM-SI 11` |
| ALH-14 | Each agent must make good hook triggering on a best-efforts basis | complete | Legislature / every governed agent | `[2026] REALM-SI 11` |
| GAZ-1 | Make or reject the Gazette Graph Database Instrument | complete | Legislature / Standing Committee | `[2026] REALM-SI 9`; `Legislature/statutes/instruments/2026-realm-si-9-gazette-graph-database.md` |
| GAZ-2 | MBES to settle graph database technical specification and execution plan | complete | MBES engineering | derived static JSON graph documented in `Judicature/law-reports/README.md` and built by `build/build-citator-graph.js` |
| GAZ-3 | Backfill Gazette graph nodes and edges from existing public law records | complete | MBES engineering with MoJ review | `Judicature/law-reports/site/citator-graph.json` currently has 69 nodes and 701 edges |
| GAZ-4 | Add forward-facing graph metadata to new Gazette filings | partial | MBES engineering with MoJ process | build derives edges on ingestion; explicit no-edge declarations and filing validation remain open |
| GAZ-5 | Add user-facing lineage display to the Gazette | complete | MBES engineering | `Judicature/law-reports/site/app.js`; `Judicature/law-reports/site/index.html` |

## Working Principles

1. Lexby is Advocate, Advisor, Engineer, and registrar when authorised. Lexby is not the bench and should not impersonate every independent checking function.
2. Where the runtime can spawn subagents or supports delegable workflows, independent review, bench, verification, or sidecar research should be delegated instead of performed only by the main Lexby thread.
3. Where the runtime cannot spawn subagents, the agent must say so and use the next-best separation available: citator lookup, deterministic checks, explicit role-labelled reasoning, and referral.
4. The agent must retrieve the relevant law and record. It must not rely on the whole VJS record being held in context.
5. Hook results are routing and safety signals. They do not adjudicate breach, punish, or create automatic invalidity.

## Immediate Next Steps

1. Decide the Codex-facing binding route, because this repository has no confirmed Codex hook runner.
2. Turn the Claude pre-answer and post-answer stubs into proportionate deterministic checks where the runtime can support that without blocking ordinary work.
3. Add a bench-name scanner and settle whether the invented-bench-name rule should be made express law for future judgments.
4. Add forward-facing Gazette graph validation for new filings, including explicit no-edge declarations where a Gazette item has no public lineage.

## Runtime Notes

Claude-style binding is through `.claude/settings.json` and `.claude/hooks/`. The existing plugin settings wire a Stop hook, and `cdd init` copies hook scripts and merges the settings.

Codex-style binding is not yet implemented in this repository. A root `AGENTS.md` or Codex-specific configuration could bind instructions if the runtime reads it, but automatic pre-answer and post-answer execution needs a Codex-supported hook surface or wrapper. Until that exists, Codex sessions must use instruction-level compliance plus available subagents, retrieval, and deterministic checks.

[2026] REALM-SI 10 clarifies that Claude hooks are only one adapter. The hook is a generic VJS workflow contract that must be bound, where technically possible, by each agent runtime's own hook, wrapper, instruction, skill, or plugin surface. Runtimes with delegable workflows must expose that separation for governed load-bearing work; runtimes without it must record the substitute check.

## Bench-Name Notes

The central convention is that bench names are invented and must not be real sitting or living jurists. Current public guidance says this, but current binding law appears to regulate bench size, constitution, authorship, and source of force rather than fictionalisation of names as a standalone validity rule.

The suspected public-record conformance issue is therefore not treated as voidness. It should be routed as a narrow nomenclature/conformance correction unless the competent court or Legislature makes the invented-name rule binding for future records.
