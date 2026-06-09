# The Raft - six statutory instruments made under the Framework Act s.7

**status:** void first drafts, made by the Standing Committee under the parent authority of the Realm Consolidation and Reconciliation Framework Act s.7 (subordinate law; `assent_source: standing_bounded_assent`; no fresh Royal Assent required, only parent authority). Held pending the Framework Act's lodgement as a runtime statute and the staged migration; each commences on the lapse of its objection window (Bill 14/26) and on the Schedule-3 predicates it depends on. None weakens the protective floor (Framework Act s.21(2A)); none amends the Act or the assent rule (anti-Henry-VIII).
**Drafting standard:** lean and legible (the forward duty). Machine detail here; constitutional commitment in the Act.

When lodged, each becomes `lawpack/v2/regulations/REG-*.yaml` with its parent `authority: ACT-CONSOLIDATION-FRAMEWORK:s7`, a deterministic `[YEAR] VJS-REG N` citation minted at lodgement, and the `kernel_effect` shown.

---

## SI 1 - Migration and Incorporation Regulation

**Parent:** Framework Act s.7, s.20-21. **Gives effect to:** SC-1 Q5; the migration ledger; the incorporation-validity invariant.

- **Incorporation record form** (the s.8 fields, mandatory): `v1_source`, `v2_destination`, `operative_rule`, `kernel_effect`, `supersession_or_variation`, `validity_of_original`.
- **Migration ledger:** a deterministic, append-only, **provenance-scoped** record (`lawpack/v2/provenance/migration-ledger/`), one row per crossing, the single source of truth for what crossed; **not loaded into runtime context**.
- **`INV-INCORPORATION-VALIDITY-001`** (fatal, fail-closed, allow-list): rejects any incorporation missing an s.8 field or the validity finding, breaching the **rank-floor** (destination rank >= source rank), missing a ledger row, or **weakening any of the four protective-floor limbs** (s.21(2A)).
- **General vs repo-specific:** decided by a declared `scope` field - `general` crosses by the central lawmaking route into the canonical lawpack; `repo_specific` uses the [2026] VJS-PC 1 continuity election (local-scope, `.vjs/config.toml`); doubt resolves to general.
- **Archival-status / graph rule:** a REALM/V1 record carries `estate: v1_archive`, `status: archival_source_only`, `live_authority: false`; a citation to it is a **source edge, not an authority edge**.
- **Contested validity:** a genuinely contested V1-source validity is found by the V1 Court ([2026] VJS-PC 3); an uncontested source is certified by the adopting organ on the record.
- **No bulk import:** one record per crossing; propose-by-any-agent, adopt-by-organ-only (REALM-SC 8).

## SI 2 - Federation Coordination Regulation

**Parent:** Framework Act s.7, s.14; ACT-007. **Gives effect to:** SC-1 Q3.

- **Bright-line (kernel `must_not`, fail-closed):** a coordinating/super-repo root act is void if it (i) asserts an apex or final-court function; (ii) writes/binds/gates a peer's local law or `.vjs/config.toml` without that peer's recorded adoption; (iii) overrides the canonical lawpack without the s.6 route; or (iv) forecloses a peer's amend / pin-or-decline / fork-with-lineage / exit.
- **Practical-subjection standard (court-applied, NOT a kernel gate):** leveraging an infrastructural chokepoint to make a peer's compliance practically unavoidable, or conditioning essential access/Gazette publication on central terms beyond the canonical lawpack such that exit is paper-only, is sovereignty; justiciable on a pleaded record (s.11 kernel never decides this).
- **Default subscription (ACT-007 s.2)** is lawful coordination because genuinely revocable; decline/pin/fork/exit must remain real, not sham.

## SI 3 - Transition-Court and Continuity Regulation

**Parent:** Framework Act s.7, s.11-12; ACT-007. **Gives effect to:** VJS-PC 1; SC-1 Q4. (Completes REG-TRANSITION-CONTINUITY-001.)

- The **Privy Council is the transition court**: it classifies each pending V1 matter `continue | reframe | moot`; default is continue-at-current-point; reframe/moot are reasoned findings on the record (no relitigation).
- The **narrow V1-Court referral gate** (fail-closed flag): the V1 Court acts iff the matter was fully heard in V1 before commencement, only perfection-in-V1-form + Gazette publication remains, no new doctrine is made, and nothing is imported into V2 (s.9; s.16).
- Perfected V1 orders and accrued rights preserved; the Court of Appeal persists (convened on need; abolition needs express s.10 amendment).

## SI 4 - Canonicalisation and Migration Regulation (the lock-preserving runbook)

**Parent:** Framework Act s.7, s.17; PC-2; SC-1 Q6. **Gives effect to:** the staged, governed merge onto one canonical line. This is the executable runbook; the **actual push/rename/re-licence remain the Principal's warranted acts**.

**Invariants (fail-closed):** the assented digests (`8e1d3f51`, Framework Act `90e843c1`), the lawpack lock (`4d2639cc`), the `[YEAR] VJS-ACT / VJS-REG / REALM-*` series, and existing record ids are **immutable**; an **unstaged mass edit is void** (it would break the lock and un-scope path-bound invariants fail-open).

**The migration procedure (ordered; halt-and-return on any failure):**
1. **Pre-flight:** enumerate every record with a `lawpack/v2/` path in `scope.paths`/`basis`/`evidence`; snapshot the current lock `4d2639cc`; record the set of fatal invariants and prove each binds a non-empty set today (baseline).
2. **Destination:** the established public repo (`vibe-justice-system`), canon on `main`; the V1 archive becomes a protected, read-only estate anchored by an immutable signed tag + a distinct Archive directory.
3. **Path continuity:** either **retain the `lawpack/v2/` prefix** in the destination (a path is not an estate; the prefix need not track the rename), or install a **compatibility/redirect** so every `scope.paths` glob still resolves. Do NOT `sed` the prefix.
4. **Invariant re-check:** after the move, re-evaluate every fatal invariant and prove each still binds a non-empty set; **any that does not -> halt and return** ("better two repos than a fail-open lawpack").
5. **Lock:** record a **fresh lock under a commencement addendum** pinning the new tree digest and referencing the old; the old lock `4d2639cc` and `COMMENCEMENT-V2-0001` stay pinned, never overwritten.
6. **Name:** drop the spent "v2" from the repo/canon name **prospectively**; install an inbound redirect; do NOT retro-edit any digest/citation/id.
7. **Estate markers:** every V1 record renders with `estate: v1_archive` / `archival_source_only` / `live_authority: false`; the single Gazette builds both estates on the canonical line with the graph resolving in one place.
8. **Warranted gate:** the actual `git` move, push, branch-protection, tag-signing, and any AGPL re-licence are performed only under a recorded release warrant ([2026] REALM-SI 7) by the Principal; this Regulation supplies the runbook, not the warrant.

## SI 5 - Gazette-Continuity Regulation

**Parent:** Framework Act s.7, s.18; ACT-COMPUTER-FIRST-REALM s.16. **Gives effect to:** the single Gazette of two estates; the still-open Gazette publication-governance reference (PC).

- **One Gazette, two estates** on the canonical line; **publication is constitutively inert** (force from the lawpack/kernel status, never from publication).
- **Publication packet:** manifest, validated records, rendered views, redaction report, validation report, lawpack digest; **export fails unless** the lawpack validates, the public/private scan passes, statuses are valid, citations are unique, lineage resolves, and the manifest includes the digest.
- **Estate labelling operative-by-substance**; typed lineage edges (`supersedes_runtime`, `preserves_archive`, `incorporates`, `derived_from`, `published_in`, `explains`); cites the canonicalisation estate-label rule (SI 4).
- **Transitional interface:** until a V2-controlled publication route exists, a push to the V1-hosted Gazette repo follows the V1 estate's preserved SI 7 / PC 19; this is not s.9 revival.

## SI 6 - Repos Register and Subscription Regulation

**Parent:** Framework Act s.7, s.15-16; ACT-007. **Gives effect to:** the mandatory transition-subscription + the repos register.

- **Subscription period:** 90 days from Framework Act commencement (the s.15(2) default), or such period as this Regulation later fixes.
- **Subscription act (each repo's duty):** create `.vjs/config.toml`, declare lawpack lineage (ACT-007 s.6), default-subscribe; record the row in the repos register.
- **Repos register schema** (mandated centrally, maintained per-repo): `repo`, `lawpack_lineage`, `trust_status` (AUTHORITATIVE-SOURCE / CONFORMANT-PROJECTION / DERIVED-INDEX / ARCHIVED / QUARANTINED), `subscribed_at`. Derived, pointer-only, rebuildable; never a source of law.
- **Local sovereignty preserved:** a subscribed repo may amend local law, pin/decline a version, fork, or exit (REALM-PC 17); the mandate is to subscribe, not to surrender sovereignty.

---

## Made-by note

Made by the Standing Committee under the Framework Act s.7 (the four counsel reconciled; clerk-drafter). Each is a void first draft until lodged into `lawpack/v2/regulations/` (on the Framework Act's lodgement + the staged migration) and commenced on its objection-window lapse. None amends the Act or the assent rule; none weakens the protective floor. The actual repo migration, push, rename, re-licence, and each repo's subscription are warranted/outward acts reserved to the Principal and the subscribing repos.
