---
citation_id: "[2026] REALM-PC 21"
tier: privy-council
kind: request_for_ruling
status: good-law
per_incuriam: false
date: 2026-06-08
panel: ["Coade J", "Sumberly J", "Marsden J"]
seised_by: "Agent Loop posthook self-referral: opencode hook equivalence under Bill 31 and [2026] REALM-SI 10 to [2026] REALM-SI 11"
cause_title: "In the matter of opencode adapter equivalence, static instruction context, dynamic prompt hooks, and honest best-efforts records"
adjudication_provenance: authorised-registrar
registrar_authority: "[2026] REALM-SC 8; [2026] REALM-PC 19; Bill 31 ss. 10-12"
registrar_note: "Authored by the bench (Coade J for the Court, Sumberly J and Marsden J concurring); reduced to the filed record by Lexby as s.18(4) registrar, the decision pre-existing the prose ([2026] REALM-SC 8)."
---

# [2026] REALM-PC 21

| Field | Value |
|-------|-------|
| **Citation** | [2026] REALM-PC 21 |
| **Tier** | Privy Council (constitutional first instance, bench of three) |
| **Before** | Coade J (judgment of the Court), Sumberly J, Marsden J |
| **Kind** | Request for ruling |
| **Status** | good-law |
| **Cites** | CASE-LAW s. 1; s. 3; s. 5; s. 6; s. 8; s. 13; s. 18(4)-(5); s. 19(1)/(5); Bill 6; Bill 16; Bill 20; Bill 22; Bill 27; Bill 31; [2026] REALM-SC 8; [2026] REALM-PC 19; [2026] REALM-PC 20; [2026] REALM-SI 8; [2026] REALM-SI 10; [2026] REALM-SI 11 |

> The Court answers the Agent Loop self-referral about opencode. The present opencode route is a live static instruction route and a plugin/check route. It is not, on the present record, a proven generic dynamic prompt-context route equivalent to Codex or Claude `additionalContext`. Unanimous (3-0).

## Questions

1. Does the current opencode adapter prove the same live model-visible dynamic hook injection as the Codex and Claude adapters?
2. If not, does the opencode adapter still satisfy the agent-agnostic best-efforts duty?
3. What must the adapter record say so agents do not overstate the hook position?

## Ratio (binding, realm-wide)

1. Source equivalence is functional, not nominal. Under [2026] REALM-SI 10, [2026] REALM-SI 11, and Bill 31, an agent-runtime adapter need not expose the same API name as another runtime. It must, however, be honest about what has actually been made capable of triggering and what has actually been proven by the available CLI or runtime evidence.

2. Codex and Claude stand differently on the present record from opencode. Codex has a model-visible hook-context probe for the Bill 31 and REALM-SI 8/10/11 VJS reminder. Claude has recorded `SessionStart` and `UserPromptSubmit` hook events returning the VJS payload, subject only to a later account spend-limit interruption before the model could answer. Those are dynamic hook-context or hook-event routes.

3. Opencode is proven on the present record to load project-level static instructions through `opencode.json` and instruction files, and the model probe returned `OPENCODE_CONTEXT_SEEN` from that static instruction context. Opencode is also configured to load the VJS plugin where the runtime accepts that plugin. That is a real best-efforts route.

4. Opencode is not proven on the present record to provide a generic per-prompt dynamic `additionalContext` hook equivalent to the Codex or Claude route. No VJS agent may mark such a route as live merely because the VJS plugin can run scripts, because running a script is not the same thing as injecting arbitrary reminder text into the next model prompt.

5. The lawful opencode position is therefore:

   - `AGENTS.md`, `GEMINI.md`, `CLAUDE.md`, or another project instruction file named in `opencode.json` may carry model-visible VJS loop text where the opencode runtime loads that instruction surface;
   - `.opencode/plugins/vjs-lawfulness.js` may run VJS advisory scripts where the opencode runtime loads the plugin and emits the mapped events;
   - `cdd` remains the deterministic substitute check and filing spine;
   - any absent or unproven dynamic prompt-injection capability must be recorded as a limitation or exempt route.

6. This clarification adds no new law. It applies the already-enacted agent-agnostic and best-efforts rules. The duty is not "make opencode behave like Codex." The duty is "use the strongest truthful adapter the runtime supports, and record the difference where it matters."

7. If opencode later exposes an official generic per-prompt context-extension mechanism, or if a CLI probe proves such a mechanism is loaded and model-visible, MBES may update the adapter record and binding files without a fresh merits ruling, provided the change is confined to recording and wiring that already-authorised capability. A new public-law duty or public release still follows [2026] REALM-PC 19 and [2026] REALM-SI 7.

## Reasons

The Principal's concern was correct. The purpose of the Agent Loop is to stop invisible ambiguity at the moment the agent acts. A record that says "hooks work" while meaning three different technical things would recreate the same ambiguity in adapter form.

The law does not require impossible uniformity across runtimes. [2026] REALM-SI 10 makes the hook contract agent-agnostic; [2026] REALM-SI 11 requires best efforts in the runtime actually being used. Bill 31 consolidates that position by requiring a preloop plan, legal evidence, source-equivalence scheduling, exemptions where needed, and a posthook self-referral question. Those provisions permit a substitute route, but they do not permit an overclaim.

The opencode evidence shows a model-visible static instruction route. That matters. A model that can see the project instruction text is receiving the VJS loop instruction. It is enough for the present best-efforts duty when paired with the plugin route and `cdd` checks.

The evidence does not show the stronger thing: a per-prompt dynamic context-extension hook equivalent to Codex or Claude `additionalContext`. A plugin event that runs a shell script may log, warn, block, or trigger a deterministic check. Unless the runtime passes that script's text back into the next model prompt, it is not a dynamic prompt hook. The adapter record must preserve that distinction.

This is also a source-of-force point under [2026] REALM-SC 8. Lexby may implement adapters and file the record. Lexby may not turn an unproven runtime capability into law by naming it as equivalent. The legal force comes from the enacted instruments and this judgment; the technical assertion must still be true.

## Disposal

1. The opencode adapter is approved as a best-efforts adapter on the present record.
2. The adapter is approved on the basis that opencode has a live static project-instruction route and a plugin/check route.
3. A generic opencode per-prompt dynamic `additionalContext` route is not approved as live on the present record.
4. The MBES adapter record must state that limitation plainly.
5. The VJS hook/instruction work may be conformed to this ruling.
6. The matter does not climb. No conflict with Supreme Court authority appears.

## Appendix A - technical record accepted

| Runtime | Present finding | Limitation |
|---|---|---|
| Codex CLI | Dynamic hook-context reminder proven model-visible by CLI probe. | Subject to project trust and hook enablement. |
| Claude Code | Hook events returned the VJS payload in CLI output. | Model echo was not obtained because the account hit a spend limit after hook execution. |
| Gemini CLI | Binding files are configured. | Runtime not installed in the present environment, so no live CLI probe was performed. |
| Opencode | Static instruction context proven model-visible; plugin loaded by config inspection. | Generic per-prompt dynamic `additionalContext` equivalent not proven. |
