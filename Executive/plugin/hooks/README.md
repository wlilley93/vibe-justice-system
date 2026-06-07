# VJS hooks

Three backstops: one soft and behavioural, two hard and mechanical. Together they close the gap
the alpha had: VJS was entirely trust-based, invisible the moment the agent stopped cooperating.

> The agent's job is to produce value the way it sees best, not to hold the entire statute book in
> its head every turn. These hooks are the safety net for the turns where it was heads-down and
> missed something. The court still judges lawfulness after the fact.

---

## 1. The turn watchdog (soft, model-based) - `vjs-watchdog.sh`

A **token-light Stop hook**. After every agent turn it makes ONE small Haiku call that reviews
**only that turn** (not the transcript) and asks three yes/no questions:

1. **Breach** - did this turn fall below the duty of care (s. 4 / s. 5) without self-reporting it?
2. **Fork** - did a load-bearing decision hit a convening trigger but skip the court (neither
   disposed on citation nor sent up)?
3. **Appeal** - is there an arguable ground to appeal a ruling, or to take a point to the Supreme
   Court (this is the backstop for the s. 17 / VPR 9 duty to self-appeal)?

If any answer is yes, the hook hands the agent the reason and (in the default `block` mode)
refuses to let the turn end until the agent disposes of it **by the law**: file the breach,
convene, or seek leave. It never adjudicates and never punishes - that is the court's job.

**Inert by design** unless `.justice/` exists *and* `ANTHROPIC_API_KEY` is set, and it **fails
open**: a watchdog that wedges your session would itself breach the duty of care.

| Env var | Default | Effect |
|---|---|---|
| `VJS_WATCHDOG` | `on` | `off` disables the hook |
| `VJS_WATCHDOG_MODE` | `block` | `warn` surfaces the finding but does not block the turn |
| `VJS_WATCHDOG_MODEL` | `claude-haiku-4-5` | model used for the check |
| `VJS_WATCHDOG_MAXCHARS` | `6000` | cap on last-turn characters sent to the model |

Wired via `.claude/settings.json` as a `Stop` hook (see `plugin/settings.json`).

---

## 2. The hard record gate (deterministic, no model) - `vjs-pre-commit.sh`

A **git pre-commit hook**. Filing and citation integrity are mechanical facts, not judgment
calls, so they are enforced **deterministically and fail closed** - no model, no trust required.
It runs `cdd check-citator`, which catches:

- **Citation collisions** - the same `[YEAR] <CODE> N` issued twice (the manual-numbering
  hazard: two sessions both grab `N+1`).
- **Filing breaks** - a ruling file with no citator row, or a citator row with no ruling file
  (the "judgment returned but never filed" hazard).

If the citator is sound it is silent and the commit proceeds. If not, the commit is blocked with
the list of problems. Deliberate exception: `git commit --no-verify`.

Install with `cdd init` (it symlinks the hook into `.git/hooks/pre-commit`), or manually:

```bash
ln -sf ../../.claude/hooks/vjs-pre-commit.sh .git/hooks/pre-commit
```

You can also run the audit any time:

```bash
cdd check-citator
```

---

## 3. The public-publish checkpoint gate - `vjs-pre-push.sh`

A **git pre-push hook**. Private/dev pushes, forks, and independent local jurisdictions are allowed.
Only a push to the exact canonical public VJS remote (`wlilley93/vibe-justice-system`) is a public VJS
publication and therefore an irreversible outward act. The hook fails closed for that remote unless the
matter records express Founder authorisation.

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
ln -sf ../../Executive/plugin/hooks/vjs-pre-push.sh .git/hooks/pre-push
```

---

*The watchdog catches what the agent missed. The gates refuse to let the record lie or the public
realm publish without a checkpoint. The court decides what was lawful. None of the three has to be
perfect for the system to hold.*
