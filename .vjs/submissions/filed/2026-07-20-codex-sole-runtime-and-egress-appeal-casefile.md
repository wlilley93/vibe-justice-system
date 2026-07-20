# Case file: appeal of [2026] VJS-CC-VJS 8 (orchestration locus) + joined egress question

**Court sought:** Privy Council (3), as V2 appellate + constitutional-machinery court.
**Appealed order:** [2026] VJS-CC-VJS 8 (County), "orchestration locus".
**Joined first-impression question:** model-native internet egress vs kernel-mediated search.
**Posture:** symmetric record. Both positions are put at their strongest. The bench has no access to
the advocate's (Lexby's) preference and must decide on this record and the binding law alone.

---

## 1. What binds and is NOT open to be moved on this record

- **VJS-CC-VJS 5 (fact):** two cells sharing one uid leak a cross-tenant bearer, no race, no defect in
  the attestation logic - held shut only by refusing to start a second cell.
- **VJS-CC-VJS 7 (grant on conditions):** CAP_SETUID/SETGID at container uid 0, privilege-separated
  spawner, per-slot tmpfs, cells at distinct uids asserted unprivileged from /proc before trust. The
  kernel forks and uid-isolates every cell.
- **VJS-CC-VJS 8 RATIO (binding):** the locus of cross-cell orchestration (subagents-as-cells, phases,
  teams, workflows, automations) is the kernel's chokepoint; Codex is a governed leaf; intra-cell Codex
  reasoning is permitted; Codex spawning its OWN kernel-invisible siblings is PR8, court-gated.

The ratio of VJS-CC-VJS 8, and the fact-findings of 5 and 7, are **not** put in issue here. No party
asks the bench to let a leaf spawn a kernel-invisible sibling. The appeal reaches only the INCIDENTAL
holdings below.

## 2. What is under appeal (VJS-CC-VJS 8, incidental / K2)

VJS-CC-VJS 8 K2 held, as incidental reasoning: "keep Codex as one governed leaf **among several**, the
org's **default_runtime remaining pi**, and never reframe the product as a thin wrapper around Codex."
The forbidden list bars "collapsing the manifest's multi-runtime routing to a single vendor."

## 3. The Principal's re-direction (stated faithfully, S-2 executive act)

Across a live exchange the Principal (Sovereign/Parliament + PM) directed:

1. **Codex replaces every interim runtime** - opencode, herdr, mastra, **and pi**. Codex is open-source,
   "basically already finished," and "resolves all of them at once." One runtime, not several.
2. **Submit to Codex's own model** of subagent profiles and skills, rather than maintain a parallel
   Boltrig orchestration vocabulary.
3. **A thin adapter for any Anthropic-API model** behind Codex (Codex's provider is already the loopback
   model-proxy to bifrost, so any model - GLM, Claude - is a routing choice at the chokepoint).
4. Boltrig's additive value over raw Codex is **custom UI, memory, bifrost, and kernel governance.**

The Principal further **raised, without deciding**, the joined question (§6).

## 4. POSITION A - GRANT the appeal (move K2; Codex as sole governed leaf)

- The ratio survives untouched: the kernel still forks, uid-isolates and audits **every** cell. Making
  Codex the **only** leaf runtime changes *which* binary each cell runs, not *who* forks it. K2's
  "several runtimes / default pi" is incidental description of a transitional posture, not doctrine.
- "Submit to Codex's model" is safe **if read as interface-not-executor**: Codex emits a
  subagent/profile/skill *request*; the kernel intercepts it at the chokepoint and forks a governed,
  uid-isolated, audited cell to fulfil it. Codex supplies the *vocabulary and the reasoning*; the kernel
  remains the *executor*. This gives the Principal "one vendor's model" without a kernel-invisible
  sibling, so it does not touch the VJS-CC-VJS 8 ratio or PR8.
- Consolidation is a first-order good already recorded in the estate (consolidation-over-fragmentation):
  four interim runtimes (opencode, herdr, mastra, pi) are maintenance surface, each its own bug and
  security surface. One open-source, well-resourced runtime reduces the attack surface the kernel must
  reason about and the number of adapters bifrost must carry.
- Runtime-agnosticism was a means, not an end. Its purpose was never "always keep N vendors"; it was to
  avoid lock-in to an *ungoverned* runtime. A leaf whose provider, tools and egress are all mediated at
  the kernel chokepoint is not lock-in in the sense the doctrine feared.

## 5. POSITION B - DISMISS the appeal (keep multi-runtime, pi default)

- K2 is not mere description; it is a **guard rail** the court set deliberately, and the forbidden list
  expressly bars "collapsing multi-runtime routing to a single vendor on the strength of the wrapper
  intuition." The Principal's re-direction IS that intuition, restated with more force. Granting the
  appeal is doing precisely the thing VJS-CC-VJS 8 forbade.
- **Single-vendor concentration risk.** Pinning the entire product to one open-source runtime (Codex
  0.144.3) makes every Codex regression, supply-chain event, or upstream direction change a
  whole-product event. Multi-runtime routing is defence in depth: if Codex must be quarantined, Pi still
  answers. Retiring Pi removes the fallback that VJS-CC-VJS 5 relied on ("held shut only by refusing to
  start a second cell" presupposes a runtime that can refuse).
- "Submit to Codex's model" risks **doctrine drift**: even if built as interface-not-executor today, the
  organisational gravity of "we are a Codex shop" pulls toward eventually enabling Codex-native spawning
  (PR8) because it is "the Codex way." The guard rail exists to resist exactly that gravity.
- **production_ready is still Codex-version-blocked** (VJS-CC-VJS 7 J13: two preflight items -
  effective_provider, full_generated_schema_contract - have no method in the 0.144.3 protocol). Betting
  the sole runtime on a binary that cannot self-attest its own provider or schema contract is imprudent
  while that is unresolved.

## 6. JOINED FIRST-IMPRESSION QUESTION - model egress / internet search

Should a leaf model be permitted to perform its **own** internet search / web fetch, OUTSIDE the
kernel's mediation? Facts: OpenAI-hosted models can search server-side; some providers (GLM) may; a
**local** model cannot, so the fallback would be a **browser-CLI tool** the cell invokes.

- **Egress is an effect.** The entire VJS program mediates effect and egress at the chokepoint (bearer
  attested, no key at rest, tool ceiling strips exec_command). A model doing its own search is an
  **uncontrolled egress + data-exfiltration surface**: the cell's context (which may hold another
  tenant's data) leaves the boundary to an endpoint the kernel neither chose nor audited.
- **Two sub-cases the bench must separate:**
  - (a) **Provider-side search** (OpenAI/GLM search *inside the model call*): the egress rides the
    already-governed model-proxy → bifrost wire. The kernel already owns that wire. Question: is
    provider-side search a *new* egress the kernel must additionally gate, or is it inside the model
    call the kernel already permitted?
  - (b) **A browser-CLI tool the cell runs locally**: this is an **effectful tool** = PR8 by
    VJS-CC-VJS 8 K4 (a tool that forks work / reaches outside the cell). It cannot open on the read-only
    lane and is not a mere configuration.
- **The safe general rule the bench is invited to consider:** all egress, including search, is a
  kernel-mediated verb through the chokepoint; provider-side search is permitted only where the kernel
  can see and audit the request/response on the model-proxy wire; a local browser-CLI is an effectful
  tool and is PR8; no model performs unaudited egress with a cell context that may contain another
  tenant's data. (Put for decision, not as a steer.)

## 7. Relief sought / dispositions open to the bench

The bench may, on each limb independently: **grant**, **dismiss**, or **grant-on-conditions**.

- Limb 1 (sole runtime): move K2 to permit Codex as the sole governed leaf, preserving the ratio; or
  dismiss; or grant on conditions (e.g. keep a fallback runtime until production_ready is unblocked).
- Limb 2 (submit to Codex's model): declare the interface-not-executor boundary as the binding reading;
  or dismiss; or condition.
- Limb 3 (egress/search): adopt the §6 rule; or a variant; the browser-CLI limb is PR8 in any event.

## 8. Constraints on any outcome

- Nothing may overrule the VJS-CC-VJS 5/7 fact-findings or the VJS-CC-VJS 8 ratio without confronting
  the cross-tenant bearer leak on the evidence; no party offers evidence to disturb them.
- production_ready stays False regardless (Codex-version-blocked; VJS-CC-VJS 7 J13; fresh application
  under VJS-CC-VJS 4 F9 required to flip).
- Actual write-enable (Codex running effectful tools, spawning siblings, local browser-CLI) is PR8 and
  does not open on this appeal.
- The read-only reasoning lane (VJS-CC-VJS 2) is unaffected and may deploy irrespective of this appeal.
