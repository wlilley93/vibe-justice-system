# VJS Kernel - Kernel Specification (brownfield)

Brownfield reverse-engineering of `~/Projects/Vibe Justice System` (Rust, 9 crates, ~16.9k
LOC, edition 2024), captured 2026-06-25. One of three sibling specs; see `agent-libos.md`,
`opbox-kernel.md`, and the synthesis.

## 1. IDENTITY & SCOPE

A governance/law kernel: a deterministic, model-free Rust engine that governs which writes
may enter a repository's law-and-record corpus, by what authority, recorded how, with an
entrenched immutability floor and a court/precedent model on top. NOT a runtime substrate:
it does not sit between an agent and a syscall/tool/file/socket at execution time. It sits
at the git write chokepoint (staged-diff / CI time) and the record-creation verbs, and
answers one question deterministically: may this staged change become canon, and if not,
which instrument forbids it. It never executes the governed action, never calls a model,
never reaches the network (`deny.toml` bans net/model/vector crates from the kernel closure).
Positioned in its own statutes as clerk, not court (REG-KERNEL-001).

## 2. ARCHITECTURE

Crates: `vjs-core` (types + gate predicates: route.rs, governance.rs, bench.rs, front_door.rs,
enforcement.rs, hook.rs, scope.rs, spec.rs, report.rs); `vjs-lawpack` (loader, refs.rs PC-17
grounding, overlay.rs two-tier loader); `vjs-engine` (the chokepoint orchestrator: lib.rs
validate() pipeline, assent.rs floor, runtime.rs submit_decision); `vjs-store`; `vjs-git`;
`vjs-redact`; `vjs-cli`; `vjs-mcp` (9 verbs); `vjs-testkit`. Data model = a YAML lawpack
(statutes, regulations, orders=precedent, invariants, decisions, obligations, specs, provenance).
Three chokepoints, all commit/CI-time: the git pre-commit hook (`vjs validate --staged`), the
advisory pre-write hook, and the required CI job (canon-enforce.yml) re-running the identical gate.

```
 SOVEREIGN ASSENT (non-machine: Will) -- entrenches -->
   lawpack/v2/*.yaml (statutes, regs, ORDERS=precedent, invariants, provenance)
        | load + pin (sha256 lawpack.lock)
   -- vjs-engine::validate() --
   lawpack validate -> referential integrity -> citation uniqueness -> lock-drift
   staged_gates: apex-routing | 28 invariant predicates | PERMIT gate | canon-write boundary |
   media | destructive-delete | cross-repo | PC-17 citation grounding | bench-integrity |
   enforcement-surface drift  -- then -- ASSENT-RESOLUTION FLOOR (downgrade) --
        ^ pre-write hook (advisory)   ^ pre-commit hook (the local wall)
   REQUIRED CI (canon-enforce.yml) = TRUST ROOT (re-runs the SAME gate; --no-verify can't reach it)
   [separate ADVISORY layer] runtime::submit_decision(overlay, env) -> Grant|Deny|RouteForCorrection
        -> returned to a subscriber TRUSTED to honour it
```

## 3. AUTHORITY & ENFORCEMENT MODEL

route -> permit -> log -> validate -> commit. `route()` mints a `permit_id` only when
Allowed/AllowedWithConditions; a CourtRequired matter walks to court with no permit;
obligations (OBL-LOG-001, OBL-BOUNDARY-001, OBL-VALIDATE-001, all due BeforeCommit) attach.
The `Permit{id, route_id, actor, scope: Option<Scope>, obligations, expires_at, status,
self_issued, meaning, intent_digest}` is an agent-routed SELF-ISSUE, non-repudiable via
`intent_digest`. Enforcement is COMMIT-TIME, not runtime: the wall is the pre-commit hook
(`vjs validate --staged`), fail-closed if the gate binary is stale, bypassable with
`--no-verify`, consciously backstopped by REQUIRED CI. The pre-write hook is advisory.

Permits are NOT capabilities: no use-count/one-shot (an Active in-scope permit covers unlimited
writes until expiry), no delegation/attenuation, no revocation-on-next-call (Revoked is read at
the next validate; there is no call). Scope = path-globs + optional jurisdictions/action-kinds,
not `{subject, resource, rights, issuer, lifetime, use-count, delegation-depth}`.

The entrenched assent-resolution floor: after all gates, every blocking finding on a staged
governed record whose declared `assent_source` RESOLVES is downgraded Fatal->Warning and tagged
ASSENTED_ROUTE_FOR_CORRECTION (never voided), giving teeth to ACT-ASSENTED-RECORD-PROTECTION /
VJS-ACT 10. Resolution is model-free (`assent_resolves`): declare an allow-listed value
(column-zero only) AND trace via the three-limb route-class check ([2026] VJS-SC 5): established-
at-HEAD, or a regulation whose parent authority resolves to a defined statute, or an order
declaring a non-empty bench. CONSTITUTIVE_CODES (BENCH_*, TIER_NOT_CONSTITUTED, CITATION_COLLISION,
APEX_RECORD_IN_SUBSCRIBING_JURISDICTION, CANON_BOUNDARY_VIOLATION) never downgrade. Enforcement-
surface digest-pin (enforcement.rs + .vjs/enforcement-surface.lock) makes any gate edit non-silent.

PC-15 overlay `submit_decision` is ADVISORY: a pure function returning Grant|Deny|RouteForCorrection
with law_source[]; no interposition, the subscriber is trusted to honour it. Visibility/Invocation/
Authority separation: ABSENT (the overlay answers one authority question).

## 4. INVARIANTS

28 YAML invariants in `lawpack/v2/invariants/`, each a deterministic predicate tree (`PredicateExpr`,
~40 variants, evaluated with no LLM). E.g. INV-ENTRENCHED-GATE-001 = if statute-08 modified then the
s.14 entrenchment clause must survive (raw-byte check). Plus the code-level constitutive codes. Bound
by `predicate_teeth.rs`, but NOT a complete invariant->test map: the conformance map (docs/conformance-
map.md) is 43/281 duties wired (15%); 238 unwired "side doors" (many declarative/agent-duties, but the
ratio is the honesty signal). The single-front-door instrument to triage them (D12) is reserved, not
built.

## 5. TEST & PROOF DISCIPLINE

~116 testkit `#[test]` (~169 workspace-wide), model-free, network-free, deterministic; toolchain pinned
in CI. `e2e_gate_harness.rs` stands up an ephemeral git repo carrying the real canon and runs the full
validate exactly as a commit would (the forged-apex-order and constitutive-non-downgrade cases run
through the real pipeline - genuinely strong). But binding is uneven: the e2e harness proves a handful of
the highest-value vectors; the other 238 unwired duties have no gate and no gate-test. No fuzz/property
testing of permit/scope algebra; no adversarial red-team for the runtime overlay (a goal, not built).

## 6. BUILT vs STUBBED vs ADVISORY

Real & enforced (commit-time, blocking, test-bound): the validate pipeline + all staged gates; the
assent-resolution floor + constitutive codes; the enforcement-surface pin; install-completeness; the
apex-routing bright-line; the required-CI trust root. Advisory: runtime::submit_decision (the whole
overlay); the pre-write hook; subject_tier_advisory + ORDER_CITATION_NOT_IN_FORCE (Warning-only); the
MCP server is a door, not the wall. Governance-ceremony vs enforcement: a large fraction is record
(gazette ~0.9MB, precedent, provenance, opinions); the enforced core is a few hundred lines across
validate + assent.rs + bench.rs + enforcement.rs.

## 7. DEPTH SIGNALS (STRENGTHS)

An entrenched constitutional floor machinery may not soften (fail-OPEN for assented records, fail-CLOSED
for everyone else; changeable only by Sovereign assent, protected from itself); the constitutive-vs-
correctable distinction (defeats "type two magic words to launder a Fatal", proven through the real
pipeline); the digest-pinned enforcement surface held OUTSIDE the witnessed code; read-policy-by-reference
(bench sizes parsed from the constitution, not hard-coded); "every denial names its instrument" + "every
grant carries law_source[]" as first-class fields; required CI as the trust root (SHA-pinned actions,
pinned toolchain, fail-closed-on-stale-binary); deterministic by construction (no model/net in the closure).

## 8. GAPS, SOFTNESS & RISKS

The runtime overlay is advisory not structural (the single biggest gap vs a runtime substrate); permits
are not capabilities (no use-count/delegation/revocation-on-call); the conformance gap (43/281 wired,
15%); legal-ceremony-to-enforcement ratio is high; THREE+ recently-shipped real bugs in the enforcement
path - permit-closed bypass (a Closed permit excused new writes + skipped obligations), in_force citation
mislabel (a binding order falsely read NOT_IN_FORCE), prose-boundary leak (a subscriber name escaped the
field-only gate), plus the silent-seats false-positive - evidence of velocity outrunning depth on a fast-
moving gate surface; the write-access-recompile residual (bounded not cured); the MCP `record` verb's
fail-open history; no Visibility/Invocation/Authority split.

## 9. THREAT MODEL

Defends (commit/CI-time): forged governed records (forged assent traces to nothing -> stays Fatal);
laundering a constitutive defect; rogue canon inserts / boundary leaks (RedactScanner); unauthorised
governed writes (PermitGate, fail-closed); silent gate-weakening (ENFORCEMENT_SURFACE_DRIFT); a subscriber
asserting apex authority (apex_routing); --no-verify local bypass (CI trust root); hallucinated citations
(PC-17). Does NOT defend: runtime actions (anything the agent DOES is invisible); a subscriber ignoring
submit_decision (advisory); semantic/prompt injection; a determined write-access author editing a gate
(bounded not cured); most of the statute book (238 unwired duties); host isolation / verified access
control / transactional rollback.

## 10. TRANSFERABLE IDEAS

Adopt from VJS: the entrenched machinery-can't-soften floor, protected from itself, changeable only by an
out-of-band human-assent act; the constitutive-vs-correctable split; the digest-pinned enforcement surface
held outside the witnessed code; "every denial names its instrument / every grant carries law_source";
read-policy-by-reference (never hard-code a threshold); a required-CI trust root re-running the SAME
deterministic gate; the precedent/citator discipline; a published wired-vs-unwired conformance map. Avoid
(lessons from VJS's softness): advisory enforcement masquerading as a boundary (make the kernel hold the
resource and be the sole syscall surface); a permit/grant that is not a real capability; over-ceremony
(keep the ceremony-to-teeth ratio honest); loose test binding to a sprawling duty surface (bind to
generated invariant->test->duty coverage as a build gate); conflating Visibility/Invocation/Authority.

## BOTTOM LINE

A mature, genuinely principled governance/law kernel with a small but real enforced core and several
best-in-class ideas (entrenched floor, constitutive codes, digest-pinned surface, required-CI trust root)
other kernels should steal. But it is a commit-time/CI-time records gate, not a runtime substrate: its
flagship runtime overlay is advisory, its permits are not capabilities, ~15% of stated duties are wired.
Recent shipped bugs in the enforcement path signal velocity outrunning depth. Its honesty about its own
residuals is itself a maturity signal. It sits one clear layer ABOVE a runtime kernel: excellent at
policing what becomes law, structurally unable to police what an agent does at runtime.
