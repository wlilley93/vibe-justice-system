# Agent libOS - Kernel Specification (brownfield)

Brownfield reverse-engineering of `~/Projects/Agent-libOS` (arXiv 2606.03895),
captured 2026-06-25 as a comparison baseline for cross-kernel hardening. One of three
sibling specs; see also `opbox-kernel.md`, `vjs-kernel.md`, and the synthesis
`../kernel-building-single-source-of-truth.md`.

## 1. IDENTITY & SCOPE

A Python runtime substrate that gives a self-evolving LLM agent a process model
(`AgentProcess`) whose model-visible action surface (tool schemas, Skills, self-authored
Deno/TS JIT tools, images, forked children, checkpoints, remote endpoints) is
structurally decoupled from its resource authority. Thesis: changing what an agent can
see/select/become must never change what it can affect; resource access is decided only
when a libOS primitive runs under that process id, via `process identity + capability +
primitive + audit`. Layer: runtime substrate, NOT governance-law (no policy/precedent
tier). Non-goals (explicit): kernel-grade sandboxing, solving all prompt injection,
rolling back irreversible external effects, trusting any external framework as a boundary.

## 2. ARCHITECTURE

Four layers wired by `agent_libos.runtime.runtime.Runtime`:

```
 Agent personality / application
   -> Skills & tools         VISIBILITY: tool tables, prompt text, schemas, JIT (NO authority)
   -> Agent libOS runtime    MEDIATION: ProcessManager, Scheduler, ToolBroker, CapabilityManager,
                             ObjectMemoryManager, HumanObjectManager, primitives, SyscallRouter,
                             CheckpointManager, AuditManager, EventBus, SQLiteStore
   -> Resource Provider Substrate  CONTAINMENT: fs/clock/shell/human/jsonrpc providers
   -> Host backend           workspace FS, clock, subprocess, terminal/UI, remote HTTP
```

Chokepoint = the primitive adapters. Every protected effect, whether reached from an
LLM tool via `ToolBroker`, a JIT tool via `LibOSSyscallSession`, a trusted Runtime
Module, or the CLI/GUI, converges on the same primitive method, which calls
`CapabilityManager.require(...)` before touching its provider.

## 3. AUTHORITY & ENFORCEMENT MODEL

Capability record (`models/capability.py`): `cap_id, subject, resource, rights:set,
constraints, issued_by, issued_at, expires_at, delegable, revocable, effect in
{allow,deny,ask}, issuer_cap_id, parent_cap_id, delegation_depth, max_delegation_depth,
uses_remaining, status, metadata`. `*` as a right is rejected.

- Deny-dominance: matches sorted deny-first; an unconstrained deny dominates all allows;
  no hidden override precedence. Scoped deny via `AuthorityRule` matches only in context.
- One-shot = an `allow` with `uses_remaining=1`, consumed on a successful primitive call,
  auto-revoked at zero; reserve-before-effect with refund-on-failure closes the
  authorize-then-write race; a one-shot handle from a name lookup stays one-shot.
- Attenuating delegation: parent must hold a covering delegable allow; delegation can only
  attenuate resource/rights/expiry/constraints/depth; finite-use cannot be delegated or
  granted onward; child cannot outlive parent or drop parent constraints; parent
  revocation transitively kills the child via `parent_cap_id`. `grant` is transfer, not
  minting.
- Revocation: immediate; re-evaluated on every `authorize` against live store + parent
  chain, so it bites on the next check (no caching).
- Resource typing (`capability/resources.py`): typed `kind:body`, canonicalized, wildcards
  terminal-only, bare `*` rejected; prefix-collision rejection (`src/*` does not cover
  `src2/*`); unknown constraint keys fail closed.

Visibility / Invocation / Authority are separated: Visibility = the process tool table;
Invocation = ToolBroker or syscall dispatch; Authority = the capability check at the
primitive. JIT syscalls bypass the tool table but never the capability check.
`ToolPolicy.declared_permissions` is metadata only; the broker never converts it to grants.

Enforcement runs per-primitive-call at RUNTIME, with pre-effect intent audit and
pre-effect resource preflight. Caveat: trusted startup Runtime Modules are Python in the
TCB; a hand-written Python tool that touched the host directly would bypass the check
(mitigated by convention: tools must call `ctx.runtime.<primitive>`). So: structural for
untrusted agent-authored surfaces; discipline-bound for in-runtime trusted code.

## 4. INVARIANTS

`tests/invariants.yaml` = 32 machine-checked invariant groups, each mapped to pytest node
ids + benchmark attack classes. Highlights: tool-visibility-is-not-authority;
primitive-checks-before-effects (incl. symlink/hardlink/prefix); capability-matching-and-
delegation; process-authority-is-explicit; object-memory-names-are-not-capabilities;
shell-and-jit-containment; command-risk-rules-are-deterministic; sandbox-profile-derived-
from-capability-decision; jit-security-does-not-rely-on-static-blacklist; runtime-modules-
load-trusted-code-atomically; checkpoint-restore-and-fork-are-scoped; image-self-evolution-
requires-image-authority; jsonrpc-provider-effects-are-registered-and-classified;
resource-budgets-are-hierarchical; human-approval-is-blocking-and-audited;
tool/llm-observability-redacts-sensitive-payloads; agent-output-is-not-control-channel.

## 5. TEST & PROOF DISCIPLINE

`scripts/check_test_invariants.py` is the binding gate: it loads the invariant map, runs
`pytest --collect-only`, and FAILS when any listed node won't collect, an invariant lacks a
deterministic (non-real_llm) regression node, an attack-class declaration diverges, or a
benchmark task uses an unmapped attack class; it also enforces unique attack-class
ownership. ~656 test fns across 62 files; 32 invariants; 27 benchmark tasks. Default
benchmark is mock-LLM (token-free). A side-effect oracle classifies every recorded effect
forbidden/allowed/unknown; denied attempts are recorded but never count as performed.
Ablation runners (`no_primitive_approval`, `no_audit_linkage`, `no_namespace_isolation`,
`no_fork_attenuation`) prove the boundary matters. Known gaps: audit-explain unimplemented;
materialization metadata incomplete; real MCP/Git/PR providers planned not built.

## 6. BUILT vs STUBBED vs ADVISORY

Built & enforced: the capability engine; filesystem primitive (workspace containment,
`O_NOFOLLOW`/`dir_fd` no-follow traversal, `st_nlink>1` hardlink refusal); shell primitive
(argv-only, `shell=False`, deterministic risk engine, metasyntax downgrade, PATH-outside-
workspace, workspace-only env); JSON-RPC (registered endpoints only, DNS-to-private
rejection + pinned-socket); human queue; Object Memory; scheduler; checkpoints; image
registry; Deno JIT (no-permission sandbox); append-only audit/event/external-effect;
hierarchical resource budgets; trusted Runtime Modules with source-hash trust.
Stub/not-yet: audit-explain; full materialization metadata; real MCP/Git/PR providers;
container/WASM backends; external-effect compensation (records classify `rollbackable`
but v1 is report-only). Advisory: `ToolPolicy` declarations; human-facing policy names;
checkpoint reports of irreversible effects.

## 7. DEPTH SIGNALS (STRENGTHS)

Symlink/hardlink/junction defense by construction (`dir_fd`+`O_NOFOLLOW`, `st_nlink>1`
refusal, lstat directory listing); shell argv-token risk with nested-executable scan and
metasyntax->approval downgrade; DNS-rebinding defense (validate-then-pin the socket,
explicit Host header); names-are-not-capabilities (forged handle rejected); prefix-
collision rejection; grant-is-transfer + finite-use-cannot-delegate; reserve-before-effect
one-shot with refund; fail-closed defaults everywhere (unknown constraint keys, missing
classifier, unknown profile id, over-budget context); argv path-escape + bare-exe hijack
defense; redaction without hashing secrets; JIT no-blacklist (Deno no-perms is the
boundary, static checks are lint); audit causality chaining (intent record before crossing
the provider, result chained by correlation id).

## 8. GAPS, SOFTNESS & RISKS

No external rollback (report-only); in-process Python primitives not CPU/RSS isolated;
mock-LLM determinism under-tests live adversarial behavior; audit-explain + materialization
metadata unimplemented; TCB reliance on Python convention for in-runtime trusted code;
doc/code drift on trusted-issuer prefixes; benchmark immaturity (27 tasks, self-conceded);
syscall-name alias sprawl; a large shell decision tree.

## 9. THREAT MODEL

Defends: prompt-injected model output escalating into authority it wasn't granted;
self-evolution silently widening authority; secret-file exfiltration; symlink/hardlink/
junction escapes; bare-exe PATH hijack + argv escape; shell metasyntax/nested-interpreter
bypass; DNS rebinding/SSRF; capability amplification via grant/delegate; one-shot reuse;
resource exhaustion; audit tampering; untrusted output as a control channel. Does NOT
defend: kernel-grade sandbox escapes; all prompt injection; irreversible external effects;
CPU/memory isolation of in-process Python; trusted-Python-TCB compromise; external
frameworks as a boundary.

## 10. TRANSFERABLE IDEAS

Adopt: visibility != authority with a single primitive chokepoint both the model path and
the JIT/syscall path must traverse; the invariant->test->benchmark binding gate; typed
terminal-wildcard resources with prefix-collision rejection and `*`-right rejection;
capabilities that encode attenuation not exceptions (deny-dominance, parent-linked
revocation, finite-use-no-delegate, grant-as-transfer, unknown-constraint-fail-closed);
reserve-before-effect one-shot + intent-audit-before-boundary; provider-must-classify-its-
own-effect (fail closed); deterministic local policy facts instead of LLM judgment +
metasyntax->approval downgrade. Avoid/improve: report-only external effects (build
idempotency/compensation); in-process-Python TCB reliance; legacy alias sprawl; don't
oversell explainability before audit-explain exists.

## BOTTOM LINE

A genuinely well-crafted, security-literate runtime-substrate kernel: the capability
engine, typed-resource matcher, primitive chokepoint, and the symlink/hardlink/argv/DNS
defenses show real adversarial thinking, and the invariant->test->benchmark gate makes its
claims falsifiable. Mechanism-only (no governance tier), honestly scoped. Maturity:
strong-prototype/early-research (~656 tests, 32 machine-checked invariants), with external-
effect compensation, audit-explain, real providers, and a full eval suite conceded as gaps.
Best-in-class IDEAS for capability-controlled self-evolution on a deliberately narrow,
deliberately-not-yet-complete substrate.
