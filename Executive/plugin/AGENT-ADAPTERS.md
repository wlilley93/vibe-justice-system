# VJS Agent Adapter Record

**Status:** public system-data adapter record under [2026] REALM-SI 10 and [2026] REALM-SI 11.  
**Owner:** Ministry of Business, Engineering and Skills.  
**Scope:** how the agent-agnostic lawfulness hook contract is made capable of triggering across agent runtimes.

This record states adapter surfaces and trigger quality only. Concrete prompts, credentials, tenants,
model settings, hostnames, logs, private repo facts, and volatile thresholds stay in the private
operational registry or the local `_private/` working area.

Runtime watchdog review uses the CLI family that fired the hook. Adapters set `VJS_AGENT_RUNTIME`
so Codex sessions call `codex exec`, opencode sessions call `opencode run`, Gemini sessions call
`gemini -p`, and Claude sessions call `claude -p`. Where the runtime CLI exposes a named agent or
subagent selector, the review runs through that separated reviewer: Claude gets an inline
`vjs-watchdog` agent by default, and `VJS_WATCHDOG_AGENT` may route opencode or Claude through a
configured reviewer. Where the runtime CLI exposes structured output, the watchdog captures
JSON/JSONL and uses schema-bound final output where supported. The watchdog does not call a vendor
API endpoint directly and does not require a vendor API key in the hook script.

| Runtime / workflow class | Adapter surface | Trigger quality | Delegable workflow support | Best-efforts substitute where automatic hooks are absent | Verification |
|---|---|---|---|---|---|
| Generic VJS-capable agent | root `AGENTS.md`; portable `.vjs/hooks/`; canonical source checkout `Executive/plugin/hooks/`; `cdd` commands | manual / wrapper-mediated unless the runtime binds a hook directory | runtime-dependent | read `AGENTS.md`, run deterministic checks, use available delegation, record limitation | `cdd init` installs `AGENTS.md` and `.vjs/hooks/`; canonical source checkout tracks `Executive/plugin/hooks/` |
| Claude Code | `.claude/settings.json`; `.claude/hooks/`; `CLAUDE.md` | automatic for configured Claude hook events; advisory where hook scripts are stubs | supported where Claude workflow tools are available | use `AGENTS.md`, `CLAUDE.md`, deterministic checks, and workflow tools | Claude `/hooks`; `cdd init` merge output |
| Codex CLI / Codex-style sessions | project `.codex/hooks.json` or inline `.codex/config.toml` `[hooks]`; root `AGENTS.md`; portable `.vjs/hooks/`; source checkout `Executive/plugin/hooks/` | automatic for trusted project Codex hook events (`SessionStart`, `SubagentStart`, `UserPromptSubmit`, `PreToolUse`, `PostToolUse`, `Stop`); advisory where hook scripts are stubs | supported where Codex subagents are available | if project hooks are untrusted or disabled, use `AGENTS.md`, subagents, deterministic checks, and record the limitation | Codex `/hooks`; project `.codex/hooks.json`; project trust in `~/.codex/config.toml`; command output from `cdd` checks |
| Gemini-style sessions | project `.gemini/settings.json`; portable `.vjs/hooks/`; source checkout `Executive/plugin/hooks/` | automatic only where the runtime honours configured Gemini hook events; advisory where hook scripts are stubs | runtime-dependent | use `AGENTS.md`, available agent/reviewer separation, retrieval commands, and deterministic checks if hooks are disabled | project `.gemini/settings.json`; command output from `cdd` checks |
| opencode-style sessions | project `opencode.json` instructions; root `AGENTS.md`; project `.opencode/plugins/vjs-lawfulness.js`; `Executive/plugin/opencode-vjs-lawfulness.js`; portable `.vjs/hooks/`; source checkout `Executive/plugin/hooks/` | static project-instruction context is model-visible where loaded; plugin/check events are automatic only where the runtime loads the VJS plugin and emits the mapped events; generic per-prompt dynamic `additionalContext` is not asserted live unless later proven | runtime-dependent | use `AGENTS.md`, instruction files named in `opencode.json`, available agent/reviewer separation, retrieval commands, and deterministic `cdd` checks if dynamic prompt injection is absent or unproven | project `opencode.json`; project `.opencode/plugins/vjs-lawfulness.js`; command output from `cdd` checks; [2026] REALM-PC 21 |
| Git commit workflow | `.git/hooks/pre-commit` chained or symlinked to `.vjs/hooks/vjs-pre-commit.sh` or `Executive/plugin/hooks/vjs-pre-commit.sh` in the canonical source checkout | automatic when installed in the git repository | not applicable | install or expose the `cdd` CLI, then run `cdd check` manually if the hook cannot run | `cdd init`; `.git/hooks/pre-commit`; `cdd check` |
| Git public-push workflow | `.git/hooks/pre-push` chained or symlinked to `.vjs/hooks/vjs-pre-push.sh` or `Executive/plugin/hooks/vjs-pre-push.sh` in the canonical source checkout | automatic when installed in the git repository | not applicable | do not push publicly until a release warrant exists and local CI passes; retrieve the warrant with `cdd release-warrant`; run pre-push gate manually if needed | `cdd init`; `.git/hooks/pre-push`; `cdd release-warrant`; `cdd local-ci`; [2026] REALM-SI 7 warrant |
| Other IDE/chat agents | root `AGENTS.md`; wrapper or plugin if supplied by the runtime | unknown until adapter supplied | runtime-dependent | read the contract, use runtime tasks/reviewers if available, run deterministic checks, record limitation | adapter-specific documentation |

## Minimum Best-Efforts Sequence

For governed load-bearing work, every agent should:

1. Check for `AGENTS.md`, `.vjs/hooks/`, runtime-specific hook settings, and available `cdd` commands.
2. Enable or use the available adapter where it can be done safely.
3. Use delegable workflows, subagents, reviewers, or equivalent separation where available and material.
4. Use `cdd` as the deterministic filing and movement spine wherever a command exists: `cdd check`,
   `cdd local-ci`,
   `cdd submit-request`, `cdd submit-breach`, `cdd law`, `cdd graph`, `cdd init`,
   `cdd lodge-judgment`, `cdd release-warrant`, and public-release checkpoint gates.
5. If an automatic trigger is unavailable, record that limitation and use a manual checklist or referral.

Hosted CI is not part of the required VJS binding route. The public-release gate is local:
`vjs-pre-push.sh` evaluates release-warrant authority for the canonical public remote before it
spends time on `cdd local-ci`.

Hook outputs are routing and safety signals. They do not adjudicate breach, punish, sanction, or create
automatic invalidity.
