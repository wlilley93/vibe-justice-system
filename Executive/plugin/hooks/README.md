# VJS hook and workflow contract

This directory contains the portable VJS hook scripts. They are agent-agnostic at the contract
level: any runtime that can bind shell hooks, delegable workflows, subagents, tasks, or equivalent
independent checks can use them. Claude Code, Codex, Gemini-style, and opencode-style bindings are
bundled adapters, not the design boundary. The public trigger-quality record is
`../AGENT-ADAPTERS.md`.

Three active backstops ship today: one soft and behavioural, two hard and mechanical. The Claude
and Codex adapters also carry pre/post answer binding stubs for the REALM-SI 8 lawfulness hooks.
Together they close the gap the alpha had: VJS was entirely trust-based, invisible the moment the
agent stopped cooperating.

> The agent's job is to produce value the way it sees best, not to hold the entire statute book in
> its head every turn. These hooks are the safety net for the turns where it was heads-down and
> missed something. The court still judges lawfulness after the fact.

---

## 1. The turn watchdog (soft, model-based) - `vjs-watchdog.sh`

A **token-light Stop hook**. After every agent turn it asks the active runtime CLI (`codex`,
`opencode`, `gemini`, or `claude`) to review **only that turn** (not the transcript) and answer
three yes/no questions:

1. **Breach** - did this turn fall below the duty of care (s. 4 / s. 5) without self-reporting it?
2. **Fork** - did a load-bearing decision hit a convening trigger but skip the court (neither
   disposed on citation nor sent up)?
3. **Appeal** - is there an arguable ground to appeal a ruling, or to take a point to the Supreme
   Court (this is the backstop for the s. 17 / VPR 9 duty to self-appeal)?

If any answer is yes, the hook hands the agent the reason and (in the default `block` mode)
refuses to let the turn end until the agent disposes of it **by the law**: file the breach,
convene, or seek leave. It never adjudicates and never punishes - that is the court's job.

**Inert by design** unless `.justice/` exists and a runtime CLI can review the turn. It **fails
open** if the CLI is absent, unauthenticated, times out, or cannot return a parseable answer: a
watchdog that wedges your session would itself breach the duty of care.

| Env var | Default | Effect |
|---|---|---|
| `VJS_WATCHDOG` | `on` | `off` disables the hook |
| `VJS_WATCHDOG_MODE` | `block` | `warn` surfaces the finding but does not block the turn |
| `VJS_AGENT_RUNTIME` | adapter-set | runtime that fired the hook: `codex`, `opencode`, `gemini`, or `claude` |
| `VJS_WATCHDOG_RUNTIME` | `auto` | optional override for runtime detection |
| `VJS_WATCHDOG_AGENT` | runtime default | optional runtime agent/subagent selector where the CLI supports one |
| `VJS_WATCHDOG_MODEL` | runtime default | optional runtime-specific model override |
| `VJS_WATCHDOG_MAXCHARS` | `6000` | cap on last-turn characters sent to the model |
| `VJS_WATCHDOG_TIMEOUT` | `45` | timeout in seconds for the runtime CLI review |

Installed generically under `.vjs/hooks/` and tracked in the canonical source checkout under
`Executive/plugin/hooks/`. The Claude adapter wires it through
`.claude/settings.json` as a `Stop` hook (see `plugin/settings.json`). The Codex adapter wires it
through project `.codex/hooks.json` as a `Stop` hook (see `plugin/codex-hooks.json`; the canonical
source checkout has its own `.codex/hooks.json` pointing at `Executive/plugin/hooks/`). In a
canonical source checkout it detects `Judicature/.justice/`; in an installed local jurisdiction it
detects root `.justice/`.

The adapter sets `VJS_AGENT_RUNTIME` so the watchdog uses the same CLI family the user is already
running. The watchdog prefers a separated runtime review surface where one exists:

- Codex runs an independent read-only `codex exec` child reviewer, captures `--json` JSONL events,
  and writes the schema-constrained final message through `--output-last-message`.
- opencode runs `opencode run --format json`; when `VJS_WATCHDOG_AGENT` is set, the hook passes it
  through as `--agent`.
- Claude runs `claude -p --output-format stream-json` with a JSON schema and an inline
  `vjs-watchdog` agent. When `VJS_WATCHDOG_AGENT` is set, the hook passes it through as `--agent`
  instead of defining the inline default.
- Gemini currently runs `gemini -p` with the same compact verdict prompt, because the local source
  checkout has not proven a richer Gemini structured-output or agent-selection flag.

The parser accepts a direct verdict JSON object, JSONL event streams that contain the verdict text,
or a compact JSON object embedded in fallback text output. The child review disables VJS hooks in
its own environment to avoid recursive watchdog loops.

---

## 2. Lawfulness trigger points - `vjs-pre-answer.sh` and `vjs-post-answer.sh`

The pre-answer hook is event-sensitive. `SessionStart` is a short bootstrap notice so it does not
duplicate the prompt-level context. `UserPromptSubmit` carries the full Bill 31 preloop reminder.
Delegated agent starts (`SubagentStart` / `BeforeAgent`) carry a narrower delegated-work reminder.
Tool hooks remain deterministic check points and do not inject the full prompt reminder unless an
adapter explicitly enables that route.

The VJS contract names pre-answer, post-answer, previous-answer, source-of-authority, role
separation, and data-boundary checks. A runtime adapter maps those checks to its own event surface.

Claude Code does not expose literal `PreAnswer` or `PostAnswer` event names. The bundled Claude
adapter maps them this way:

- `UserPromptSubmit` - fires after the user submits a prompt and before Claude processes it. VJS
  wires `vjs-pre-answer.sh` here as the pre-answer binding point.
- `Stop` - fires when Claude finishes responding. VJS wires `vjs-post-answer.sh` here as the
  post-answer binding point, and keeps `vjs-watchdog.sh` on the same event as the active
  token-light review.

The pre/post answer scripts are intentionally non-blocking scaffolds while the Agent Lawfulness
Hooks Instrument awaits commencement. `vjs-pre-answer.sh` is silent by default; setting
`VJS_PRE_ANSWER_REMINDER=on` makes it inject a short advisory context reminder in VJS
jurisdictions. `vjs-post-answer.sh` is a no-op placeholder so it cannot interfere with the
existing Stop watchdog.

Other agents should bind the same contract at the nearest equivalent points: before producing or
executing a governed load-bearing answer or act, after that answer or act, and before a later turn
continues work that may call the previous answer into question. If the runtime supports delegable
workflows, it should delegate materially separable research, verification, review, or court-routing
work rather than collapsing all roles into one thread. If no delegation surface exists, the agent
must record the substitute check used.

Use Claude Code's `/hooks` menu to verify that the project settings loaded these hooks.

Codex exposes lifecycle hooks through `hooks.json` or inline `[hooks]` tables in `config.toml`.
The bundled Codex adapter is `Executive/plugin/codex-hooks.json`, installed by `cdd init` to
`.codex/hooks.json`. The canonical source checkout also carries `.codex/hooks.json`, pointing
directly at `Executive/plugin/hooks/`. It maps:

- `SessionStart`, `SubagentStart`, and `UserPromptSubmit` to `vjs-pre-answer.sh`;
- `PreToolUse` to `vjs-pre-answer.sh`;
- `PostToolUse` to `vjs-post-answer.sh`;
- `Stop` to `vjs-post-answer.sh` and `vjs-watchdog.sh`.

Codex requires its normal hook review/trust flow before project hooks execute. Verify with Codex's
`/hooks` command in the TUI. Project hooks load only when the project `.codex/` layer is trusted.

Gemini-style and opencode-style source checkout adapters live at `.gemini/settings.json` and
`.opencode/plugins/vjs-lawfulness.js`, with distributable reference files under
`Executive/plugin/`. They are best-efforts runtime bindings under the same MBES hook implementation
route: they connect available session/tool lifecycle events to the portable VJS scripts, but they do
not change legal force, routing authority, or the public/private boundary.

---

## 3. The hard record gate (deterministic, no model) - `vjs-pre-commit.sh`

A **git pre-commit hook**. Filing and citation integrity are mechanical facts, not judgment
calls, so they are enforced **deterministically and fail closed** - no model, no trust required.
It runs `cdd check`, which catches:

- **Unauthorised-looking central judgments** - newly added central judgment files without explicit
  court-workflow or authorised-registrar provenance metadata.
- **Citation collisions** - the same `[YEAR] <CODE> N` issued twice (the manual-numbering
  hazard: two sessions both grab `N+1`).
- **Filing breaks** - a ruling file with no citator row, or a citator row with no ruling file
  (the "judgment returned but never filed" hazard).
- **Real jurist labels** - prohibited real bench-name labels in central judgment records and the
  law-report case corpus projections.

If the citator is sound it is silent and the commit proceeds. If not, the commit is blocked with
the list of problems. If the hook cannot resolve `cdd`, `vjs`, or a vendored Node CLI, it also
blocks because it cannot enforce the deterministic audit. Deliberate exception:
`git commit --no-verify`.

Install with `cdd init` (it symlinks the hook into `.git/hooks/pre-commit`), or manually from the
generic hook location:

```bash
ln -sf ../../.vjs/hooks/vjs-pre-commit.sh .git/hooks/pre-commit
```

Older Claude-only installs that symlink `.claude/hooks/vjs-pre-commit.sh` continue to work.

You can also run the audit any time:

```bash
cdd check
```

---

## 4. The public-publish checkpoint gate - `vjs-pre-push.sh`

A **git pre-push hook**. Private/dev pushes, forks, and independent local jurisdictions are allowed.
Only a push to the exact canonical public VJS remote (`wlilley93/vibe-justice-system`) is a public VJS
publication and therefore an irreversible outward act. The hook fails closed for that remote unless
the matter records express Founder authorisation and local deterministic CI passes. It checks
authorisation first, so an unwarranted public push is blocked before spending time on local CI.
This is a local checkpoint. It does not rely on GitHub Actions or any hosted CI service.

The local CI command is:

```bash
cdd local-ci
```

It runs source syntax checks, JSON parse checks, public-law index consistency, CLI unit tests,
aggregate deterministic checks, law/graph smoke checks, and workspace/staged whitespace checks.

Accepted authorisation records:

```text
Judicature/ministry-of-justice/reasons-ledger/outward-act-authorisations/public-vjs-publish.md
.vjs/checkpoints/public-vjs-publish-authorisation.env
```

Minimum fields:

```text
AUTHORISED_OUTWARD_ACT=public-vjs-publish
AUTHORISED_BY=Sovereign Founder
AUTHORISED_AT=YYYY-MM-DDTHH:MM:SSZ
```

Optional fields (`AUTHORISED_REMOTE_URL`, `AUTHORISED_REMOTE_REF`, `AUTHORISED_LOCAL_SHA`) scope the
authorisation to one exact push.

Install with `cdd init` (it symlinks the hook into `.git/hooks/pre-push`), or manually:

```bash
ln -sf ../../.vjs/hooks/vjs-pre-push.sh .git/hooks/pre-push
```

---

*The watchdog catches what the agent missed. The gates refuse to let the record lie or the public
realm publish without a checkpoint. The court decides what was lawful. None of the three has to be
perfect for the system to hold.*
