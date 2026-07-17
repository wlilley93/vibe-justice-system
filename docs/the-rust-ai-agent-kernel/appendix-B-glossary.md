# Appendix B - Glossary

*This is the volume's controlled vocabulary: one precise sense per term, alphabetized, so that "capability", "chokepoint", "advisory", and "fail-closed" mean exactly one thing across every chapter. Entries cite the invariant they serve (K-n, defined in full in Appendix A) and the chapter that motivates it; where a property has a cleanest reference exemplar among the three kernels it is tagged [L] Agent libOS, [O] Opbox, [V] VJS.*

---

**AAD (Additional Authenticated Data).** Bytes fed into an authenticated-encryption operation (e.g. AES-256-GCM) that are authenticated but not encrypted, binding the ciphertext to a context. A kernel sets the owning row's identity as AAD so a stolen ciphertext cannot be replayed into a different row, and decryption fails closed if the context does not match. See K-21; exemplar [O].

**actor.** The authenticated principal on whose behalf an action is attempted (workspace, subject, agent, or human), as distinct from the resource it acts upon. The kernel resolves the actor by construction at the door and never from a request-asserted field. See identity-by-construction, K-3.

**advisory-as-enforcement.** The volume's cardinal anti-pattern: a check that returns a verdict but is not the only path to the resource, so a caller can ignore it and still reach the resource. Advisory-as-law (a policy a real boundary consults) is legitimate; advisory-as-enforcement-boundary is the defect. The lesson is drawn from [V], whose flagship runtime overlay (`submit_decision`) is advisory and therefore not a boundary. See chokepoint, sole mediated path, K-1.

**apex / federation routing.** In a federated kernel, the deterministic bright-line that a subscriber instance may seat only first-instance matters and must refer anything above its jurisdiction upward (anonymised) to the canonical authority, never recording an apex order itself. A forged or self-asserted apex record stays fatal through the whole pipeline. Exemplar [V] (`apex_routing`, REG-FEDERATION-COORDINATION-001). See front door.

**assented record.** A record entrenched by an out-of-band human-assent act (e.g. Sovereign assent in [V]). Machinery may never void, block, or downgrade it, only route it for correction; the capability lifecycle governs prospective authorization only and may not touch an assented record. See entrenched floor, route-for-correction, K-15, K-16.

**attenuating delegation.** Re-grant that can only narrow: a delegated capability must have a covering delegable parent and may reduce, never widen, resource/rights/expiry/constraints/depth. Finite-use capabilities cannot be delegated; a child cannot outlive its parent, and revoking the parent kills the child. See K-7; exemplar [L].

**binding debt.** The count of in-scope invariants that have no bound, collectible, failing-by-default test. A kernel publishes this number and ratchets it monotonically downward, so safety coverage can only improve and never silently regress. See invariant-to-test binding, K-29, K-30, and Ch. 9 (The Binding Discipline).

**binding quality.** A measure beyond the mere existence of a test: whether the bound test actually fails when the property is broken (an adversarial or ablation test) rather than only asserting the invariant is cited somewhere. The lesson that [O]'s invariant gate "proves citation not property" is what forces quality to be measured, not just presence. See Ch. 9 (The Binding Discipline).

**capability.** The unified authority record `{subject, resource(typed), rights, effect(allow/deny/ask), issuer, parent_cap_id, delegation_depth, expires_at, uses_remaining, status, constraints}` that is the sole token of authority. Possession of the record, not knowledge of a name, authorizes; a VJS permit and an Opbox bearer/grant are both profiles of this one record. See K-4 and the capability-primitive chapter (K-4..K-11).

**chokepoint.** The single in-process locus where every action on a governed resource is mediated and decided. "One chokepoint" means there is exactly one and that no path reaches the resource around it. See sole mediated path, K-1, K-2.

**constitutive vs correctable.** The split between "is this a valid object of its kind at all" (constitutive, never waivable) and "is this otherwise-valid object defective in some fixable way" (correctable, waivable under authority). No authority claim or assent may launder a constitutive defect: a forged object with valid assent stays blocked on constitutive grounds. Exemplar [V]; see K-16, the entrenched-floor chapter.

**content-driven gate.** A gate whose verdict derives from hashing or scanning the actual content rather than from a declared flag, so it cannot be evaded by mislabelling. Its characteristic failure mode is the false positive (a publication denylist keyed on a content hash blocking a legitimate public term), which is why such gates need an assent-backed override path, not a softening of the gate. See read-policy-by-reference, K-18, K-28.

**dependency fence.** A compile-time and CI constraint on the kernel's dependency closure that bans whole crate classes (network, model/LLM) from the decision path, proving determinism structurally instead of by code review. Tested by failing the build if a banned crate appears in the lockfile. See K-12; exemplar [V] (`the_kernel_closure_bans_network_and_model_crates`).

**deny-dominance.** The precedence rule that an unconstrained deny overrides every overlapping allow, with no hidden ordering. Changing a deny requires an explicit revoke-and-reissue, never a competing allow that races it. See K-5; exemplar [L].

**digest-pinned surface.** The set of enforcement-relevant files pinned by a digest stored outside the witnessed code, so any edit to a gate trips a drift check and cannot be silent; re-locking the pin is a deliberate, reviewable act. See K-25; exemplar [V] (`check_drift_flags_an_edited_gate`).

**entrenched floor.** The small set of protections that no policy edit, refactor, feature flag, or the floor's own machinery may soften (protected from itself), changeable only by an out-of-band human-assent act, with the guarding rule citing itself by number. See K-15; exemplar [V]; the entrenched-floor chapter.

**fail-closed.** The default that any uncertainty denies: an unmapped verb, missing classifier, unparseable expiry, null scope, unknown profile, over-budget condition, or unreachable engine all resolve to deny, never to allow. See K-13 and the deterministic-policy chapter (K-12..K-14).

**front door.** The single authenticated entry surface (e.g. an MCP server-of-law, or the verb-dispatch entry) through which all callers must pass to reach the chokepoint. It stamps identity and routes record kinds, but authority is still checked at the primitive: passing the door grants nothing on its own. See chokepoint, identity-by-construction, K-1, K-3.

**governance-law plane.** The topmost of the three planes: the entrenched floor, precedent, and the law a decision cites. It is the deterministic, model-free policy ENGINE that lower planes consult but never bypass. Exemplar [V]. See plane, substrate plane.

**grant-is-transfer.** The rule that granting is moving rights one already holds, not minting new authority. An actor cannot create a deny/ask it lacks, nor widen a grant beyond its own capability. See K-11; exemplar [L].

**hash-chained audit.** An append-only audit in which each entry carries a hash (HMAC) over its predecessor and is written in the same transaction as the state change it records, so any edit, reorder, or drop is detectable and the chain cannot be reforged without the key. See K-19; exemplar [O].

**honest remainder.** The discipline of explicitly recording what the kernel does NOT prove or defend (for example a direct superuser DB connection, prompt injection, or a sandbox escape) instead of implying total coverage. The honesty-to-teeth ratio is itself a signal of a sound kernel. See universal negative, paper claim.

**identity-by-construction.** Resolving the principal from the authenticated bearer at the door and stamping an immutable `source`, never trusting a request-asserted workspace or identity field. A request claiming a different principal is ignored, closing cross-tenant impersonation structurally. See K-3; exemplar [O].

**invariant-to-test binding.** The keystone meta-gate: every safety invariant maps to at least one collectible, deterministic, failing-by-default test, and the build fails if any in-scope invariant is unbound. This converts "we believe this is enforced" into "the build fails if it is not." See K-29; Ch. 9 (The Binding Discipline); exemplar [L] (`check_test_invariants.py`).

**kernel.** The single, deterministic, unbypassable mediator between an actor and a resource: the only path to the resource, deciding by a pure function of recorded state (no model, no network at the decision point), failing closed, recording why, and protecting an entrenched floor it cannot itself soften. The one clean concept the whole volume serves.

**names-are-not-capabilities.** The rule that knowing an id, handle, or name confers no authority absent the capability itself; a forged or stale handle is rejected even under an otherwise broad grant. See K-10; exemplars [L, O].

**one-shot capability.** A capability with `uses_remaining = 1` that is reserved before the effect, consumed on success, refunded on failure, and auto-revoked at zero. A name lookup cannot re-launder it, and concurrent attempts cross the resource exactly once. See K-6; exemplar [L].

**opaque resource vocabulary.** The discipline that the capability and governance layers are generic over a resource/scope vocabulary they never interpret (a `kind:body` treated as opaque bytes), so the upper planes never learn subscriber-specific terms and the canon boundary stays intact. See typed resource, K-4, K-10; the capability-primitive chapter.

**paper claim.** A safety property asserted in prose, a comment, or a citation but not bound to a test that fails when it is violated. The volume treats every paper claim as unproven until it has a failing-by-default test; "citation, not property" is the failure this names. See invariant-to-test binding, binding quality, Ch. 9 (The Binding Discipline).

**plane.** One of the three separated layers of a kernel: governance-law (decides), action/enforcement (the chokepoint that enforces), and substrate (touches the resource). A higher plane may be consulted by a lower one, but the lower plane must be the only path to the resource; collapsing the planes, or leaving any one of them advisory, is the defect.

**read-policy-by-reference.** Reading thresholds, sizes, and policy from the governing record at runtime rather than hard-coding them in the enforcer, so that changing the record changes enforcement with no code edit. See K-18; exemplar [V] (`reads_sizes_by_reference`).

**reversibility classification.** Requiring the effect provider to classify its own effect as `irreversible | rollbackable | none`, failing closed if it cannot; the classification then drives whether the action must block on human approval. See K-23; exemplar [L]; the effects-and-human-authority chapter.

**route-for-correction.** The disposition reserved for an entrenched or assented record that carries a defect: instead of voiding or blocking it, the kernel downgrades the blocking finding to a warning and tags it for correction (e.g. `ASSENTED_ROUTE_FOR_CORRECTION`), giving the floor teeth without breaching it. Constitutive codes never downgrade this way. See assented record, K-15, K-16; exemplar [V].

**sole mediated path.** Invariant K-1 stated plainly: every action passes through one chokepoint and no path reaches the resource around it, backstopped at the data layer by a non-superuser role plus a via-verb trigger so that even a direct connection cannot write. See chokepoint, K-1.

**substrate plane.** The lowest plane: the primitive boundary that actually touches the resource, holds the capability, classifies the effect, and audits the crossing. It must be the only path to the resource. Exemplar [L]. See plane, governance-law plane.

**trust root (required-CI).** A required status check on the canonical remote that re-runs the SAME deterministic gate the local hooks run, toolchain-pinned and action-SHA-pinned, so a local `--no-verify` bypass cannot reach the merge. The remote, not the developer's machine, is the root of trust. See K-27; exemplar [V] (`required_ci_reruns_the_same_deterministic_gate`).

**typed resource.** A resource named by `(kind, body)` and canonicalized so matching is structural, not substring: terminal wildcards only, `src/*` does not cover `src2/*`, a bare `*` is rejected, and unknown constraint keys fail closed. See K-9; exemplar [L].

**universal negative.** The one claim a kernel cannot prove: that NO path exists around the chokepoint, because proving a negative over all possible paths is unbounded. A sound kernel is honest about this limit and instead maximises structural closure (single dispatch, non-superuser role, via-verb trigger, compile-time excision) and red-team coverage. See honest remainder, sole mediated path, K-1.

---

## Invariants governed

This glossary indexes the entire invariant set: K-1 through K-30. Full normative definitions live in Appendix A; the entries above give the working sense each term carries throughout the volume.
