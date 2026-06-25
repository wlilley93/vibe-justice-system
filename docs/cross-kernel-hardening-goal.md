# Goal: Cross-Kernel Capability Hardening (the Unbypassable Door)

Status: proposed (pre-court). Owner: Lexby. Governance: VJS.
Reference substrate: Agent kernelB (Yingqi Zhang, arXiv 2606.03895),
pulled to `~/Projects/Agent-kernelB` for grounding.

## North star

Make both kernels - the **VJS law kernel** and the **Acmeco runtime kernel** -
*unbypassable, capability-controlled, and audited*, by adopting one coherent
model: an Agent-kernelB-style **primitive boundary** where the only path to a
resource is a checked verb, with **VJS as the law/policy engine behind that
boundary** and the **entrenched assent floor as the one policy no operator may
soften**. The agent must have no door but the gate.

## The gap we are closing

1. **Interposition is advisory, not structural.** The VJS front door enforces at
   *commit/CI*; the PC-15 runtime overlay (`submit_decision`) *returns* a
   decision the subscriber is trusted to honour. Agent kernelB shows the
   discipline: a JIT tool can call only `kernelb.syscall(...)`; authority is
   checked at the **primitive manager**, never in the wrapper
   (`agent_kernelb/runtime/syscall_router.py`, `agent_kernelb/primitives/*`). We want
   the Acmeco kernel chokepoint to be the *sole* syscall surface, so revocation
   takes effect on the next call and there is no path around the check.
2. **Two divergent notions of authority.** VJS *permits* (scope + obligations)
   and the Acmeco kernel's grants are not the same primitive, and neither is a
   true capability. Agent kernelB's record
   (`agent_kernelb/capability/manager.py`, `.../resources.py`) binds
   `{subject, resource, rights, constraints, issuer, lifetime, revocation,
   use-count, delegation-depth}` with one-shot grants and attenuation. We want
   one capability primitive shared by both kernels.
3. **Audit can leak content.** Our subscriber-name-in-logs problem is exactly
   what Agent kernelB's `docs/artifact_anonymity.md` + bounded-observability
   audit (`agent_kernelb/runtime/audit_manager.py`, no raw prompts by default)
   prevent at the source. We want a single audit envelope, no raw subscriber
   content by default, answering the five W's.
4. **No structural separation of seeing / invoking / touching.** Agent kernelB
   separates Visibility, Invocation, and Authority and holds the invariant
   "tool visibility does not imply resource authority." Our overlay conflates
   them. We want the three-decision split explicit in both kernels.

## What "done" looks like (success criteria)

- **D1 Sole door.** An Acmeco-kernel agent provably cannot read/write tenant
  resources except through a kernel verb; a red-team that bypasses the verb
  surface fails closed. Demonstrated in an adversarial harness, not asserted.
- **D2 One capability.** Both kernels issue/check the same capability record;
  revocation is honoured on the next call; one-shot grants are consumed exactly
  once; delegation attenuates. Proven by golden vectors.
- **D3 Three-decision split.** Every governed action runs Visibility then
  Invocation then Authority; visibility never implies authority (test-locked).
- **D4 Clean audit.** Every authority-changing/side-effecting boundary emits the
  shared envelope; no raw subscriber content is stored by default; an anonymity
  test passes over the whole audit estate (the boundary-prose gate extended to
  logs).
- **D5 Irreversible-action gate.** Outward irreversible actions (deploy, send,
  spend) block on a durable approval queue, resume on a Principal one-shot grant,
  and are reversibility-classified (`irreversible | rollbackable | none`).
- **D6 Floor intact.** None of the above narrows the entrenched VJS-ACT 10 floor;
  the assent floor remains the one policy machinery may not soften.

## Keystone (the four court matters that gate the rest)

These are first-impression architectural forks and go to the bench as one
connected reference (not a decisive call, not the Principal):

- **K1** The Visibility / Invocation / Authority split as a kernel invariant.
- **K2** The unified capability primitive (the record shape + one-shot +
  delegation/attenuation + revocation-on-next-call).
- **K3** The sole-syscall-surface mandate for the Acmeco kernel chokepoint
  (advisory -> structural interposition).
- **K4** The one-shot-capability + durable human-approval-queue design for
  irreversible outward actions.

Everything else is a build *under* these ratios, or a decisive call.

## Disposition map

- **[court]** K1-K4 (the keystone reference).
- **[call] start now:** the shared audit envelope (D4), reversibility tagging
  (D5 substrate), VJS permit lifecycle hygiene (auto-expire/close + author-time
  obligation check).
- **[build] under the ratios:** wire VJS `submit_decision` as the policy behind
  the Acmeco primitive boundary (GOV); per-primitive finest-grain audit
  (CC-ACMECO 104); the adversarial threat-suite from Agent kernelB
  `docs/invariants.md`; the unitary-stack SoR migration (per service).
- **[Principal] needs you:** rewrite/rotate the ~26 subscriber-named historical
  decision-logs; stand up GOV-2 (the second VJS server) and GOV-4 (the flip);
  authorize each prod deploy.

## Grounding

`~/Projects/Agent-kernelB` (cloned). Read first for the case file:
`docs/architecture.md`, `docs/capabilities.md`, `docs/invariants.md`,
`docs/artifact_anonymity.md`, `agent_kernelb/runtime/syscall_router.py`,
`agent_kernelb/capability/manager.py`, `agent_kernelb/runtime/audit_manager.py`,
`agent_kernelb/human/manager.py`. The court case file cites their actual
mechanisms, not the paper's abstraction.

## Non-goals (honest, carried from both sources)

No defence against semantic prompt injection (the model may still *request* a
dangerous action; the point is that the request meets a checked boundary, a
policy, an approval, and an audit). No kernel-grade host isolation, no formally
verified access control, no transactional rollback of external effects. The
write-access-recompile residual remains bounded (the enforcement-surface pin +
required CI) not cured; the ultimate backstops are non-machine (the Sovereign's
gate and the duty of care), and are not represented as more.
