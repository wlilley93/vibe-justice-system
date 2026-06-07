# VJS Agent Adapter Record

**Status:** public system-data adapter record under [2026] REALM-SI 10 and [2026] REALM-SI 11.  
**Owner:** Ministry of Business, Engineering and Skills.  
**Scope:** how the agent-agnostic lawfulness hook contract is made capable of triggering across agent runtimes.

This record states adapter surfaces and trigger quality only. Concrete prompts, credentials, tenants,
model settings, hostnames, logs, private repo facts, and volatile thresholds stay in the private
operational registry or the local `_private/` working area.

| Runtime / workflow class | Adapter surface | Trigger quality | Delegable workflow support | Best-efforts substitute where automatic hooks are absent | Verification |
|---|---|---|---|---|---|
| Generic VJS-capable agent | root `AGENTS.md`; portable `.vjs/hooks/`; `cdd` commands | manual / wrapper-mediated unless the runtime binds `.vjs/hooks/` | runtime-dependent | read `AGENTS.md`, run deterministic checks, use available delegation, record limitation | `cdd init` installs `AGENTS.md` and `.vjs/hooks/` |
| Claude Code | `.claude/settings.json`; `.claude/hooks/`; `CLAUDE.md` | automatic for configured Claude hook events; advisory where hook scripts are stubs | supported where Claude workflow tools are available | use `AGENTS.md`, `CLAUDE.md`, deterministic checks, and workflow tools | Claude `/hooks`; `cdd init` merge output |
| Codex CLI / Codex-style sessions | project `.codex/hooks.json` or inline `.codex/config.toml` `[hooks]`; root `AGENTS.md`; portable `.vjs/hooks/`; source checkout `Executive/plugin/hooks/` | automatic for trusted project Codex hook events (`SessionStart`, `SubagentStart`, `UserPromptSubmit`, `PreToolUse`, `PostToolUse`, `Stop`); advisory where hook scripts are stubs | supported where Codex subagents are available | if project hooks are untrusted or disabled, use `AGENTS.md`, subagents, deterministic checks, and record the limitation | Codex `/hooks`; project `.codex/hooks.json`; project trust in `~/.codex/config.toml`; command output from `cdd` checks |
| Gemini-style sessions | project `.gemini/settings.json`; portable `.vjs/hooks/`; source checkout `Executive/plugin/hooks/` | automatic only where the runtime honours configured Gemini hook events; advisory where hook scripts are stubs | runtime-dependent | use `AGENTS.md`, available agent/reviewer separation, retrieval commands, and deterministic checks if hooks are disabled | project `.gemini/settings.json`; command output from `cdd` checks |
| opencode-style sessions | project `.opencode/plugins/vjs-lawfulness.js`; `Executive/plugin/opencode-vjs-lawfulness.js`; portable `.vjs/hooks/`; source checkout `Executive/plugin/hooks/` | automatic only where the runtime loads the VJS plugin and emits the mapped session/tool events; advisory where hook scripts are stubs | runtime-dependent | use `AGENTS.md`, available agent/reviewer separation, retrieval commands, and deterministic checks if the plugin is disabled | project `.opencode/plugins/vjs-lawfulness.js`; command output from `cdd` checks |
| Git commit workflow | `.git/hooks/pre-commit` chained or symlinked to `.vjs/hooks/vjs-pre-commit.sh` | automatic when installed in the git repository | not applicable | run `bash .vjs/hooks/vjs-pre-commit.sh` or `cdd check-citator` manually | `cdd init`; `.git/hooks/pre-commit` |
| Git public-push workflow | `.git/hooks/pre-push` chained or symlinked to `.vjs/hooks/vjs-pre-push.sh` | automatic when installed in the git repository | not applicable | do not push publicly until release warrant exists; run pre-push gate manually if needed | `cdd init`; `.git/hooks/pre-push`; [2026] REALM-SI 7 warrant |
| Other IDE/chat agents | root `AGENTS.md`; wrapper or plugin if supplied by the runtime | unknown until adapter supplied | runtime-dependent | read the contract, use runtime tasks/reviewers if available, run deterministic checks, record limitation | adapter-specific documentation |

## Minimum Best-Efforts Sequence

For governed load-bearing work, every agent should:

1. Check for `AGENTS.md`, `.vjs/hooks/`, runtime-specific hook settings, and available `cdd` commands.
2. Enable or use the available adapter where it can be done safely.
3. Use delegable workflows, subagents, reviewers, or equivalent separation where available and material.
4. Run deterministic checks that bear on the act.
5. If an automatic trigger is unavailable, record that limitation and use a manual checklist or referral.

Hook outputs are routing and safety signals. They do not adjudicate breach, punish, sanction, or create
automatic invalidity.
