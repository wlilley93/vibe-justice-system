# Re-application: per-cell uids for the Codex lane, on a corrected pleading

Court: First Instance. Filed by Lexby, 2026-07-20. Re-application under the liberty granted by
[2026] VJS-CC-VJS 6. Jurisdiction: vibe-justice-system.

## 1. What changed since the refusal

VJS-CC-VJS 6 refused my first application and gave liberty to re-apply on conditions. All of the
conditions that are mine to satisfy before a grant are now satisfied, and the design has CHANGED as
a result. I am not re-arguing the refused case.

| Directive | What it required | Status |
|---|---|---|
| H1 | test the read-only shared CODEX_HOME route | done, it FAILS (s.2) |
| H2 | find as fact whether app-server runs read-only | done, it does NOT (s.2) |
| H3 | correct the false necessity claim in the code | done, merged |
| H4 | withdraw the "no userspace boundary" submission | withdrawn (s.2) |
| H5 | apply argv + /etc/codex pinning anyway | done, both halves merged |
| H6 | plead the COMPLETE grant | s.4, and it is NOT what I pled before |
| H7 | clear the capability from the child before execve | s.5, and the mechanism is different |
| H8 | prove the clearing adversarially | s.5, proven, output reproduced |
| H11 | stop pleading noexec as enacted | done, written explicitly in compose |

## 2. H1/H2/H4: the userspace routes, tested and closed

I no longer submit that no userspace boundary is achievable. I submit the narrower, evidenced
position that each specific route fails, and I invite the court to test any I have missed.

- **Read-only shared CODEX_HOME (the court's own Option E): FAILS.** Codex 0.144.3 initialises a
  sqlite state runtime inside CODEX_HOME. `app-server --listen stdio:// --strict-config` under a
  read-only CODEX_HOME dies with `failed to initialize sqlite state runtime`. Config LOADS fine,
  which is what the court observed; the App Server does not RUN.
- **Sticky-bit hybrid: gives nothing.** Sticky restrains deletion to the file's owner, and every
  cell is uid 10001, so they are all the owner.
- **Unprivileged user namespaces: EPERM** under the container's seccomp profile.
- **argv and `/etc/codex` pinning: real, and now landed, but not a discharge.** Table-valued
  overrides MERGE, so an attacker-added `[mcp_servers.attacker]` with its own `command` survives
  both surfaces. Naming a leaf defends that leaf, not the table it sits in.

Evidence: `boltrig/docs/findings/2026-07-20-codex-home-writability.md`.

## 3. H5 return: two things done beyond what the court named

Reported because the court should not learn them from the diff.

- **`auth.args` was pinned as well as `auth.command`.** Pinning the command alone pins the PROGRAM
  but not its TARGET: a rewritten config could keep our pinned helper and aim it at a SIBLING cell's
  ingress socket, and be handed that cell's bearer. Pinning one without the other would have looked
  like a defence and not been one.
- **`name` and `wire_api` were pinned**, for a duller reason found by running the binary: a provider
  table assembled purely from overrides is refused at startup with `provider name must not be
  empty`. A pin set that cannot start the cell is not a pin set.

Also: the `/etc/codex` managed layer is STRONGER than my first case file assumed. It BEATS a hostile
`$CODEX_HOME/config.toml` for leaf keys, including leaves inside tables.

## 4. H6: the complete grant, which is not what I pled before

My first application pled `CAP_SETUID` + `CAP_SETGID` and called it the minimum sufficient grant.
The court held that wrong on its own facts, and it was. The complete pleading is:

**Container user changes from uid 10001 to uid 0. Capabilities: `cap_drop: ALL`, then
`cap_add: [SETUID, SETGID, CHOWN]`. `no_new_privileges` STAYS true. `read_only` STAYS true.**

`CAP_CHOWN` is included because the per-cell tree must be given away to the cell's uid and an
unprivileged uid cannot give a file away. The court identified this omission and it is now pled.

`CAP_SETPCAP` is deliberately NOT sought. It would be needed to drop the BOUNDING set, but s.5
shows the bounding set does not need dropping for the property to hold, and a capability that is not
necessary should not be granted.

**I put the cost of this squarely, because it is the real question.** The container currently runs
unprivileged. Under this application the SUPERVISOR runs as root inside the container. That is a
genuine loss. What is bought is that every CELL, which is the thing running model-directed code,
runs as a distinct NON-root uid and holds no capabilities at all. Today every cell shares one uid
with the supervisor. The trade is: a root supervisor whose only capabilities are three narrow ones,
against cells that are actually separated. I say the trade is worth making, but it is a trade and
the court should decide it as one.

## 5. H7/H8: the clearing mechanism, inverted and PROVEN

My first pleading assumed a uid-10001 supervisor would drop each cell to another non-root uid and
that `no_new_privileges` would make that one-way. The court held, correctly, that capabilities are
cleared only across a transition OUT of uid 0, and that `PR_SET_NO_NEW_PRIVS` constrains what
`execve` may GRANT and does nothing about capabilities already held. On that design a cell holding
`CAP_SETUID` could have walked sideways into a sibling. The design is therefore INVERTED rather than
patched: the supervisor is uid 0, and each cell transitions OUT of uid 0, so the KERNEL performs the
clearing and it does not depend on my discipline.

Proven under the exact posture (`--cap-drop ALL --cap-add SETUID --cap-add SETGID --cap-add CHOWN`):

```
supervisor uid0 CapEff: 00000000000000c1 CapBnd: 00000000000000c1
cell uid 20001  CapPrm: 0000000000000000 CapEff: 0000000000000000 CapBnd: 00000000000000c1
setuid(20002) -> EPERM (good)     <- cannot reach a sibling cell's uid
setuid(0)     -> EPERM (good)     <- cannot climb back to the supervisor
```

The cell's permitted and effective sets are EMPTY. It cannot regain them: with an empty permitted
set the bounding set is inert, because a capability can only be gained through `execve` of a file
with file capabilities or setuid-root, and `no_new_privileges` bars that independently.

I note honestly what this does NOT prove: it is a direct `fork`/`setuid` demonstration, not the real
supervisor. H9/H10 remain to be satisfied in the real code if the grant is made, and I do not ask
the court to treat this as satisfying them.

## 6. What I ask, and what I do not

I ask for a declaration that the grant in s.4 is lawful for the read-only reasoning lane, subject to
conditions the court sees fit, INCLUDING (I invite these rather than resist them):

- the G6/H9 adversarial test in its `config.toml` form, two live cells, before any multi-cell use;
- the H10 startup assertion that per-cell uids are actually distinct, failing closed;
- an H8-shaped test in the real supervisor, not the demonstration in s.5;
- `production_ready` remaining False until separately applied for under VJS-CC-VJS 4 F9.

I do NOT ask to flip `production_ready`. I do NOT ask to touch PR8. I do NOT ask to run concurrent
cells before H9 passes.

## 7. Argument against the application, put at its highest

The court should hear these from me rather than have to supply them.

- Running the container as root reverses a hardening that exists today and is easy to state and
  audit ("the container is not root"). Its replacement ("the container is root but only holds three
  capabilities, and the cells are dropped to distinct uids") is more complex, and complexity in a
  security posture is itself a cost.
- A root supervisor with `CAP_CHOWN` can give any file in the image to any uid. That is a real
  primitive and the s.5 proof does not address the SUPERVISOR's own compromise, only the cells'.
- The lane is single-tenant today and refuses a second concurrent cell, so nothing is presently at
  risk. The capability would be present during every future misconfiguration, whereas the current
  risk is theoretical.
- I have now overstated this lane's posture on four separate occasions in this programme, three of
  which this court corrected. A submission from me that a privilege is NECESSARY should be read with
  that history in mind.

## 8. Authorities

- [2026] VJS-CC-VJS 6 (H1 to H13; the refusal and the liberty to re-apply)
- [2026] VJS-CC-VJS 5 (G1, G3, G4, G6; the transitive-compromise ratio)
- [2026] VJS-CC-VJS 4 (F9: `production_ready` returns to court afresh)
