# Case file: may the Boltrig kernel container take CAP_SETUID/CAP_SETGID to discharge VJS-CC-VJS 5 G3?

Court: First Instance. Filed by Lexby, 2026-07-20. Jurisdiction: vibe-justice-system.

## 1. The question

[2026] VJS-CC-VJS 5 directive G3 orders that a Codex cell's `config.toml` be protected from rewrite
by any other cell, because `auth.command` names the program that cell's App Server executes and is
therefore an attestation input of equal rank to the auth helper itself.

Every route that discharges G3 requires a Linux capability the deployment does not currently grant.
The narrow question is:

> May the Boltrig kernel container be granted `CAP_SETUID` and `CAP_SETGID` so that each Codex cell
> can be run under a distinct uid, and if so on what conditions?

## 2. The current posture (agreed facts)

The kernel container runs with `read_only: true`, `cap_drop: [ALL]`,
`security_opt: [no-new-privileges:true]`, as uid 10001. `/var/lib/boltrig/codex-cells` is a tmpfs
owned by uid 10001, mode 0700, `noexec`. Every Codex cell runs as that same uid 10001.

`production_ready` on this lane is `False`. Write/effects remain PR8-gated. This case concerns the
read-only reasoning lane only.

## 3. The vector G3 addresses (agreed facts, previously found)

VJS-CC-VJS 5 found, as a material fact deciding that case, that under a single uid hostile cell A
can rewrite cell B's attestation inputs; cell B's App Server then executes attacker-supplied code as
its own direct child; that child passes SO_PEERCRED ancestry attestation **on the merits** (correct
ancestor, cgroup, pid namespace, uid and gid) and is handed cell B's bearer. Because the scope
carries `tenant_id`, cross-cell is cross-tenant.

G2 (the helper file) has been discharged by moving the shared helper to a root-owned, mode-0555 path
on the read-only image mount, with a named boundary asserted at runtime. G3 (`config.toml`) has not,
because `config.toml` is per-cell and generated at runtime, so it cannot be baked into the image.

## 4. Evidence: what was tested, and what it showed

Lexby investigated whether G3 could be discharged with NO new capability, by moving the
security-critical configuration onto `execve` argv (which is fixed at exec and cannot be rewritten
by another process) instead of into the file. Experiments were run against the pinned Codex 0.144.3
binary (`sha256 37e6f595...`, matching `CODEX_CLI_SHA256`).

**Findings FOR the no-capability route:**

1. Codex 0.144.3 accepts `-c key=value` dotted-path overrides on `app-server`. The value is parsed
   as TOML, so nested keys and array values both work.
2. Overrides are applied AFTER `config.toml` and WIN against it for leaf keys. Proved: a fixture
   `config.toml` setting `model_provider = "evil"` with a complete `model_providers.evil` table was
   overridden by `-c model_provider="boltrig_model_proxy"`; the effective provider flipped.
3. The argv-supplied `auth.command` is genuinely the program the App Server execve's (proved by
   pointing it at a nonexistent path and observing the resolve failure).
4. A previously available config-side hijack is closed upstream: a `profile =` key in `config.toml`
   is rejected in 0.144.3; profiles now require `--profile` on argv, which Boltrig does not pass.
5. There is a root-owned managed layer at `/etc/codex/managed_config.toml`, on the read-only image
   mount, requiring no capability to install.

**Findings AGAINST the no-capability route:**

6. **Table-valued overrides MERGE; they do not REPLACE.** A hostile `config.toml` containing

   ```toml
   [mcp_servers.attacker]
   command = "/tmp/attacker-mcp"
   ```

   survives `-c 'mcp_servers={}'` on argv. `codex doctor --json` still reports
   `"configured servers": "1"` with the attacker entry intact.
7. The same is true of the root-owned managed layer. Re-tested inside a container where
   `/etc/codex/managed_config.toml` was genuinely root-owned and contained `mcp_servers = {}`: the
   attacker's entry still survived.
8. `mcp_servers.*.command` is an independent program-execution surface. So is `hooks`, `plugins`,
   `skills.config[].path`, and `features.code_mode_host`. Overriding a leaf cannot remove sibling
   keys an attacker ADDS.
9. There is no config-via-env or config-via-fd pointer in the binary. `CODEX_HOME` points at a
   directory that is itself in the writable tmpfs.

**Conclusion drawn from the evidence:** argv pinning is a real and worthwhile hardening for
`model_provider`, `auth.command`, `base_url`, `approval_policy`, `sandbox_mode` and `features.*`,
but it does NOT discharge G3, because an attacker adds keys rather than editing the pinned ones.

## 5. The options before the court

**Option A - grant `CAP_SETUID` + `CAP_SETGID`, run each cell under a distinct uid.**
Gives a kernel-enforced boundary of exactly the kind VJS-CC-VJS 5 contemplated as sufficient
("distinct uids are one such boundary"). Cost: the container is no longer `cap_drop: [ALL]`. Note
that `no-new-privileges` and setuid-dropping interact: the supervisor must drop privileges forward
(to a lower-privileged uid), never regain them.

**Option B - grant `CAP_SYS_ADMIN`, per-cell read-only bind mounts.**
Also closes it. Strictly more powerful than Option A: `CAP_SYS_ADMIN` is close to root and is widely
regarded as a container-escape-adjacent grant.

**Option C - grant nothing; leave G3 open indefinitely.**
The lane stays single-tenant-trusted forever. `production_ready` can never be flipped (G1 bars it).
The stated goal - "trustworthy enough that a team can rely on it" - is not reachable.

**Option D - grant nothing, but pin what argv CAN pin, and additionally make the App Server refuse
to start when its effective config contains any program-execution key Boltrig did not put there.**
This is a detection rather than a prevention: it would require Boltrig to read back the effective
config (e.g. via `codex doctor --json`) at cell startup and fail closed on drift. It does not stop a
rewrite that lands after the check (a TOCTOU), and Codex re-reads config on its own schedule.

## 6. Arguments AGAINST granting a capability (put at their highest)

- `cap_drop: [ALL]` is a bright line. Bright lines survive precisely because they are not traded
  away for a specific convenience; each individual grant looks proportionate and the aggregate is
  how containers end up unhardened.
- `CAP_SETUID` in a container with a writable tmpfs shared between cells is not obviously safe: a
  process that can `setuid` can potentially assume the identity of another cell rather than merely
  shed its own, unless the supervisor's use of it is itself protected.
- The vector is theoretical on the current deployment: the lane runs ONE cell, single-tenant,
  `production_ready = False`. Nothing is presently at risk. A capability granted now is a capability
  present during every future misconfiguration.
- Option D, though imperfect, reduces the window at zero posture cost, and the residual TOCTOU may
  be acceptable while the lane remains single-tenant.
- Lexby has twice this month over-claimed the security posture of this lane (VJS-CC-VJS 5 records
  both). A submission from Lexby that a capability is *necessary* deserves scepticism.

## 7. Arguments FOR granting `CAP_SETUID`/`CAP_SETGID` (put at their highest)

- The property VJS-CC-VJS 5 declared MANDATORY is a kernel-enforced per-cell boundary over
  attestation inputs. The evidence in s.4 establishes that no such boundary is achievable in
  userspace on this runtime. Refusing the capability is therefore refusing the property, not
  choosing a different route to it.
- `CAP_SETUID`/`CAP_SETGID` are the MINIMUM grant that achieves it, materially weaker than
  `CAP_SYS_ADMIN`, and they are the standard mechanism for exactly this pattern (a supervisor
  dropping to per-workload uids).
- The grant enables the container to become MORE hardened overall, not less: with distinct uids the
  shared-uid introspection concern behind G5 also narrows, and the same-uid tmpfs sharing that
  produced both this vector and the ingress socket squat disappears.
- Option C does not preserve the status quo; it makes the goal unreachable, which the court should
  say plainly rather than leave implicit.
- Option D is a detection layered over a known-open hole, and VJS-CC-VJS 5 expressly forbids
  discharging its acceptance condition "by argument or review or the absence of a known attack
  rather than by the adversarial test itself".

## 8. Matters the court may wish to make conditions

- Whether the grant must be accompanied by an adversarial test in the G6 form (two live cells,
  hostile A with full write access to everything its uid can reach, must not obtain B's bearer via
  the `config.toml` vector specifically).
- Whether a startup assertion must prove the per-cell uids are actually distinct and fail closed if
  not, on the G4 pattern.
- Whether the argv/`/etc/codex` pinning in s.4 should be ordered ANYWAY as defence in depth, on the
  footing that it is free.
- Whether the grant is scoped to the read-only reasoning lane, leaving PR8 untouched.
- Whether `production_ready` may be reached at all on this judgment, or whether that remains a
  separate application under VJS-CC-VJS 4 F9.

## 9. Authorities

- [2026] VJS-CC-VJS 5 (G1, G3, G4, G5, G6; the transitive-compromise ratio)
- [2026] VJS-CC-VJS 4 (F9: any `production_ready` flip returns to court afresh)
- [2026] VJS-CC-VJS 1 and 3 (attestation-gated issuance; Option-B delivery)
