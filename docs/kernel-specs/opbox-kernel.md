# Opbox Kernel - Kernel Specification (brownfield)

Brownfield reverse-engineering of `~/Projects/Opbox/kernel` (Rust, `opbox-kernel` -> bin
`opbox`; 489 verbs, 109 migrations, ~66k LOC, ~821 test fns), captured 2026-06-25. One of
three sibling specs; see `agent-libos.md`, `vjs-kernel.md`, and the synthesis.

## 1. IDENTITY & SCOPE

A governed action substrate: a single Rust process exposing every state change to a
professional-services product (matters, bills, parties, documents, forms, files, equity,
signing, agents) as a verb behind ONE dispatch chokepoint over ONE Postgres. Contract:
route -> gate -> RLS-bind -> permit -> run-handler -> audit -> commit. Layer: primarily a
runtime substrate (action/enforcement plane) with a bolted-on, mostly-inert governance-law
overlay (the vendored VJS engine + the `governance/` permit gate). Relationship to
frontend-v2: they share one Postgres; intended unitary stack = frontend reads kernel tables
directly (RLS-scoped Prisma `$extends`) and writes only via verbs. Reality: ~25% migrated
by write-site (~148 verb dispatches vs ~417 still-direct Prisma writes), so the frontend is
a trusted second writer and "one write path" is aspirational at the system level.

## 2. ARCHITECTURE

One primary crate plus three vendored governance crates (`vjs-core/lawpack/engine`) and the
VPS ledger crate. `registry.rs` (14,396 lines) holds the one verb registry (489 `Verb`
rows) and the one `dispatch`. `Verb{name, capability, risk_class, authz, idempotent, tier,
handler}`; egress/budget/mfa/legal-basis are DERIVED name-keyed methods (can't drift).
`src/verbs/*.rs` (56 modules) are the handlers; `auth.rs` the identity provider; `authz.rs`
the fail-closed error taxonomy; `governance/` the (flag-gated) permit gate; `events.rs` the
SHA-256 audit chain; `kern_bridge.rs` the HMAC'd VPS ledger. ~115 singular kernel-owned
tables; workspace is the RLS scope-root.

```
        CLI   HTTP /v/:verb   MCP /mcp   Scheduler   Agent loop
         └────────── IdentityProvider.resolve (bearer->AuthContext) ──────────┘
                                   v
   ── registry::dispatch (ONE chokepoint) ──
   1 unmapped-verb deny  2 tier gate  3 autonomy gate  4 cap-scope
   5 BEGIN tx  6 RLS bind + SET LOCAL ROLE opbox_app  7 workspace-active
   8 egress  9 AI-budget  10 MFA step-up  11 VJS permit (flag-off)
   12 handler  13 legal-basis receipt  14 append_event  15 COMMIT
                                   v
   Postgres (single DB) - FORCE RLS, fail-closed-by-NULL-GUC, role opbox_app (NOSUPERUSER NOBYPASSRLS)
                                   ^
   frontend-v2 (Prisma): READS direct (RLS-scoped)  ·  WRITES via /v/:verb (~25% migrated)
```

## 3. AUTHORITY & ENFORCEMENT MODEL

End to end: a bearer arrives; `IdentityProvider::resolve` hashes it, looks up its home
workspace, RLS-binds the validation tx, runs `consume_token` (expiry / single-use spend /
session_version staleness / scope-fan-version / bound-actor / require-browser), loads the
bound actor, door-stamps `source` immutably (MCP=AGENT-only; AGENT denied over HTTP/CLI),
derives tier from the actor row, computes resolved autonomy = `min(actor.autonomy_level,
token.autonomy_ceiling)`. Workspace is read off the actor row, never request-asserted, so
cross-tenant IDOR is unreachable by construction. Then `dispatch` runs the 15-step pipeline
above, fail-closed at every axis, denying before handler effect.

Single chokepoint, honest two-part answer: inside the kernel process it is STRUCTURAL (one
dispatch; agent/MCP/CLI provably reuse it; RLS is the DB backstop). At the system level it
is PARTLY ADVISORY: the kernel cannot stop a caller reaching Postgres around it; the
frontend writes ~417 kernel-owned rows directly. The structural cure (the BD-5 `via_verb`
BEFORE-INSERT/UPDATE/DELETE trigger that RAISEs unless `app.via_verb='1'`) is DEFINED but
attached to NO table. RLS is structural only because the dispatch code remembers to demote
to `opbox_app`; the connecting role is the superuser `opbox`, which bypasses RLS. A direct
connection as `opbox` defeats both RLS and the unattached writeguard.

Two layered authorization models: (a) token/bearer (`token_hash, kind, status, single_use,
scope_ref, scope_version, autonomy_ceiling, capability_scope (per-bearer verb allow-list),
session_version, expires_at, bound_actor_id, require_browser`); revocation via single-use
spend, session_version bump, scope-fan version bump; delegation via `agent.convene` minting
a subset seat (scope subset, autonomy <= convener, depth-bounded `MAX_AGENT_TASK_DEPTH=5`).
(b) the VJS `Permit{id, law_source[], scope, ttl, law_version, signature, outcome}` for the
runtime-interposition gate - inert by default.

Visibility/Invocation/Authority present but not formally named: Visibility = VerbTier
filtering of the MCP surface (presentation only; dispatch enforces independently);
Invocation authority = tier+autonomy+capability_scope+bound_scope; Data authority = RLS.
Three independent mechanisms, no single unifying capability object.

## 4. INVARIANTS

17 canonical invariants (INV-1..16 + INV-EPI, plus INV-17) in `docs/spec/03-domain-model.md`.
Key: INV-1 one canonical write path (in-process enforced; NOT structural vs direct writers -
the BD-5 guard is unattached); INV-3 door-stamped immutable source / anti-masquerade
(structural); INV-7 encrypt-before-write PII (AES-256-GCM, row-bound AAD); INV-8 immutable
audit, SHA-256 hash-chained per workspace; INV-11 fail-closed authz at the verb (structural);
INV-12 AI spend fail-closed (PARTIALLY DEFERRED/waived - metered path stubbed); INV-13 no
direct LLM calls (deferred); INV-15 convened agent subset of convener (min() ceiling, re-
checked every call); INV-16 convening depth bounded; INV-17 per-principal confidentiality
(DB conjunction RLS). Caveat: `invariant_gate.rs` proves each INV is CITED in a test, not
that the property is fully proven; INV-12/13 explicitly waived.

## 5. TEST & PROOF DISCIPLINE

821 test fns across ~99 files; 86 require a live Postgres (integration-first against the real
dispatch + DB + RLS). Adversarial suites + meta-gates as CI tests: invariant_gate (every INV
cited or waived), nfr_coverage_gate, realness_gate (stub-marker budget ratchet),
schema_consolidation_gate, ledger_integrity_gate, verb_docs_gate, closure_gate (duplication +
too_many_lines deny), egress_gate, rls.rs (cross-tenant denial, unbound->0 rows, opbox_app
non-superuser). Not covered/thin: INV-12/13 (waived); the GOV permit gate's integration into a
real dispatch envelope; the BD-5 writeguard (unattached -> no DB write-bypass test); no formal
map proving every write path reaches dispatch; no fuzz/property tests of gate ordering; the
frontend's ~417 direct writes are outside the test boundary.

## 6. BUILT vs STUBBED vs ADVISORY

Built & enforced: the 489-verb registry + single dispatch; four-axis pre-tx gate; RLS
isolation + per-actor ACL conjunction (FORCE, non-superuser, NULL-GUC->0 rows); token/session
model with full revocation surface; INV-3 source-stamping & AGENT-MCP confinement; AES-GCM PII
at rest with row-bound AAD; SHA-256 audit chain + HMAC'd VPS ledger with boot law-pin; egress
allow-list for real rails; idempotency replay; agent loop wired to dispatch; compile-time fail-
closed identity. Stubbed/flagged-off: the GOV permit gate (INERT by default; even when on, the
envelope hard-codes `assent=Unassented`/`paths=[]`/`matter=None` and the law overlay is empty
-> grants all - a tested seam, not a control); the BD-5 writeguard (unattached); gate_requirement
(dormant, returns None for every verb); the live agent engine (NoLlm fail-closed by default);
the unitary-stack cutover (~25% migrated, ~150 sites fenced pending a ruling, dual-SoR mirror in
place). Advisory/trusted: "writes only via verbs" at the system boundary; role-demotion
discipline; the frontend as second writer; the model engine.

## 7. DEPTH SIGNALS (STRENGTHS)

Genuine single chokepoint with derived drift-proof attributes + CI gates that fail on undeclared
egress/stub/schema; compile-time fail-closed identity (release binary cannot name the dev path);
workspace authenticated-by-construction (RLS GUC from the resolved actor row, never the request)
-> cross-tenant IDOR closed structurally; RLS done correctly (FORCE + non-superuser + NULL-GUC->0
+ per-actor ACL conjunction); real revocation surface (generation counter + scope-fan + single-
use + expiry, re-checked every dispatch); tamper-evident audit in the write tx + HMAC'd
governance ledger; heavy adversarial + meta-gate test culture.

## 8. GAPS, SOFTNESS & RISKS

The flagship governance layer doesn't govern yet (off by default, impoverished envelope, empty
grant-all overlay; the legal-basis receipt is attribution-only, cannot refuse); "one write path"
is not structurally true (BD-5 trigger unattached; frontend writes ~417 rows directly); RLS
depends on code discipline for role demotion (connecting role is a superuser); dual-SoR migration
debt (~75% of writes still direct from the frontend; the syncMatterFromKernel mirror is a stale-
read hazard); dormant gate machinery + deferred invariants ride on the unbuilt cage; over-
consolidation smell (a 14k-line registry/dispatch with a hand-sequenced in-line gate chain);
audit is tamper-evident not tamper-proof (main chain unsigned SHA-256; only the VPS ledger HMAC'd).

## 9. THREAT MODEL

Defends: cross-tenant access (RLS + authenticated workspace); per-principal over-read (ACL
conjunction); unmapped-verb invocation; agent privilege escalation (min() ceiling, scope subset);
agent masquerade (INV-3 door-stamping); stolen/stale bearer reuse; PII at rest; audit tampering;
unbounded egress (built rails); over-spend (post-hoc); retried writes (idempotency); a release
build with dev identity or live engine (compile-time excision). Does NOT defend: a direct Postgres
connection as superuser `opbox` (bypasses RLS + unattached writeguard); the frontend's honesty as a
co-equal writer (~417 sites); any governance breach the GOV layer should catch (off, empty law); a
handler issuing raw SQL re-promoting its role; model-output threats (engine deferred); a forgotten
`bind_workspace_rls` on a new fresh-tx path.

## 10. TRANSFERABLE IDEAS

Adopt: one in-process dispatch chokepoint with door-stamped immutable source and identity resolved
at the door (workspace authenticated-by-construction, never request-asserted); derived single-source
attributes + CI gates that fail on undeclared egress/stub/schema; compile-time fail-closed via cfg-
excision of dev/no-engine paths; capability bearer = (verb allow-list intersect resource bound_scope
intersect autonomy ceiling), all min()'d and re-checked every call with a full revocation surface;
RLS as the real backstop (FORCE + non-superuser + NULL-GUC->0 + per-actor ACL conjunction); the BD-5
`via_verb` DB-trigger DESIGN (and actually attach it + connect as non-superuser); audit in the same
tx as the write, HMAC'd; the meta-gate discipline. Avoid: governance-by-flag-and-TODO (don't ship a
permit gate whose envelope hard-codes Unassented/empty and whose overlay grants all while presenting
it as enforcement - make the envelope complete or absent); a trusted second writer reaching around the
verb surface; enforcement that depends on remembering to demote the role; a 14k-line registry with a
hand-sequenced in-line gate chain (make gates composable middleware).

## BOTTOM LINE

A mature, production-grade runtime substrate: one real chokepoint, 489 verbs, 821 tests, genuinely
strong fail-closed identity/RLS/capability/revocation engineering, real depth in compile-time fail-
closed properties and anti-drift CI gates - ahead of an advisory or library-only kernel on action-
plane enforcement. But its governance-law layer (the GOV permit gate) is inert, placeholder-fed, and
grants-all (a tested seam, not a control), and its headline "one write path" invariant is not
structurally enforced (BD-5 unattached, frontend a trusted second writer for ~417 sites, role-demotion
by discipline). Verdict: a strong, battle-tested action substrate with best-in-class capability/RLS
primitives, but a thin/advisory governance overlay and real dual-SoR + structural-write-path debt to
close (attach the writeguard, connect as non-superuser, finish the cutover, give the permit gate a real
envelope) before its "governed substrate" claim is structurally true.
