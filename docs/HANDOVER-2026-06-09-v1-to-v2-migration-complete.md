# Handover: the V1-to-V2 constitutional migration is complete (2026-06-09)

**Author:** Lexby (acting VJS officer), session of 2026-06-09.
**Audience:** the next agent or session picking up VJS / the Agent Universe.
**One line:** the realm has lawfully migrated from V1 (case-law-first) to V2 (computer-first). V2 has **commenced** and is the live runtime jurisdiction; V1 (`agent-universe`) is now the Gazette and Archive estate.

---

## 1. TL;DR - what is true now

- **V2 is live.** The Supreme Court handover order has taken effect. V2 (`~/Projects/agent-universe-v2`) is the self-governing computer-first runtime jurisdiction. See `lawpack/v2/provenance/founding/COMMENCEMENT-V2-0001.yaml` (`status: commenced`).
- **V1 is the Gazette and Archive.** `~/agent-universe` is preserved, read-only by default, citable as archive authority, and remains the public Gazette source (`wlilley93/vibe-justice-system`). New doctrine does not expand V1 except migration/archive maintenance.
- **The governing constitutional floor is CASE-LAW s. 23** (enacted by the Supreme Court): all V2 law, including AI-drafted or delegated law, derives force only from Sovereign assent; AI may run the machinery but may never be sovereign, expand its own competence, amend the assent rule, or create force from its own output. The kernel enforces this with a fail-closed allow-list invariant.
- **Directory-agnostic.** V2 binds records by role/schema/id/status/kernel-effect, never by path. The only fixed anchor in a jurisdiction is `.vjs/config.toml` (`ACT-007:s1`). Projects can live in any directory.

## 2. The lawful chain (how we got here)

| Step | Record | Where |
|------|--------|-------|
| Privy Council reference judgment | **[2026] REALM-PC 24** (defines Q1-Q8 + 17 limits; refers up) | V1 `Judicature/.justice/judgments/privy-council/2026-realm-pc-24.md` |
| Supreme Court settlement (full court of 9, 9-0) | **[2026] REALM-SC 10** - enacts CASE-LAW s. 23(1)-(6) + express handover order | V1 `Judicature/.justice/judgments/supreme-court/2026-realm-sc-10.md` |
| Constitutional amendment | **CASE-LAW s. 23(1)-(6)** [constitutional] | V1 `Constitution/CASE-LAW.md` |
| Standing Committee adoption | Bill 32 adopted **4-0** (round 1 was 2-2 on a real Bill 27 s.14(2)->s.15(2) defect, cured) | V1 `Legislature/legislature/bills/32-computer-first-realm-act-2026.md` |
| Royal Assent | granted 2026-06-09, positive/specific/digest-pinned | V1 `Legislature/legislature/2026-06-09-royal-assent-bill-32.md` |
| Post-push review of the Gazette publish | **[2026] REALM-PC 25** (SI 7; fast-path on REALM-PC 20) | V1 `Judicature/.justice/judgments/privy-council/2026-realm-pc-25.md` |
| Commencement | all four SC conditions precedent satisfied | V2 `lawpack/v2/provenance/founding/COMMENCEMENT-V2-0001.yaml` |

**Key digests:**
- Adopted Bill 32 final-text digest (what Royal Assent pins to): `sha256:8e1d3f516cb2aca8e044d8c73bdc6ededa91a47ca86b729eece06f7eee6b9a0c`. The exact bytes are sealed at V2 `lawpack/v2/provenance/founding/bill-32-adopted-final-text.md` (its sha256 reproduces the digest).
- V2 lawpack lock (v0.1.0): `sha256:4d2639cc1b35d7e44e987eac8e6598c967a754c0d2c9d20f9337f7f4a6ebbb4f`.

## 3. The new operating model

- **Two estates, one Gazette.** V1 = Archive estate; V2 = Current estate. One public Gazette (`vibe-justice-system`) preserves the thread. Publication does not create runtime force; V2 force comes from the lawpack + a valid assent source.
- **Federation, not super-repo (V2).** `ACT-007` (Federation and Local Sovereignty Act): installing VJS into a repo creates a local jurisdiction anchored at `.vjs/config.toml`; all other paths configurable; default subscription to the canonical V2 lawpack; the local Principal may vary local law (but local law does not override canonical without a Privy Council order or Principal assent); local law is local-scope-only.
- **Courts (V2).** Constituted by `lawpack/v2/orders/2026-VJS-COURTS-CONSTITUTION-001.yaml`: County (odd bench 1), Privy Council (3), Supreme Court (5, expandable to 9). Court of Appeal persists in law but is administratively non-convened in the MVP (not abolished). No V2 court may issue an order until constituted; until a given bench is constituted, jurisdiction over its matters remains with V1.
- **Authority to onboard a project to V2 is the local Principal's** (install -> `ACT-007:s1`), not a central court order. The central realm migration needed the full court route because it unmade the governing law; onboarding a project is ordinary local installation.

## 4. Repo / branch / directory state (as of 2026-06-09)

| Repo | Path | Branch | Head | Role |
|------|------|--------|------|------|
| V1 dev (`agent-universe`) | `~/agent-universe` | `master` | `14cec32` (moves; check) | Gazette/Archive + dev provenance |
| V1 public Gazette | `wlilley93/vibe-justice-system` | `public-vjs-canonical-preview` | `03a75c3` | the live public Gazette (serves REALM-SC 10) |
| V2 runtime (`agent-universe-v2`) | `~/Projects/agent-universe-v2` | `v2-agent-harness-doctrine` | moving (concurrent activity; check) | the live computer-first runtime |

- **The SC judgment (REALM-SC 10) and PC 24 are LIVE on the public Gazette.** The rest of the migration records sit on the private `master` and are NOT yet public. Publishing them is a separate warranted act (see s. 8 below).
- Next central citations: **`[2026] REALM-PC 26`**, **`[2026] REALM-SC 11`** (PC 25 is the post-push review).

## 5. What is live in the V2 kernel (the Gate B engineering)

- **The fail-closed Assent-Source Invariant.** New kernel predicate `assent_source_valid` (`crates/vjs-core/src/types.rs` enum + deserializer; `crates/vjs-core/src/spec.rs` evaluator). It is an AFFIRMATIVE ALLOW-LIST: a runtime-force record (under `statutes/`, `regulations/`, `rules/`, `orders/`) carries force only if it declares an `assent_source` resolving to `sovereign_assent` or `standing_bounded_assent`. Absence, emptiness, an unrecognised form, or an unresolved trace each REJECT. The `not(field_equals self_authorised)` deny-list form is VOID as fail-open (CASE-LAW s. 23(5)); the old draft is removed.
- **Proof:** `crates/vjs-testkit/tests/assent_source_invariant.rs` (7/7 pass: missing field, unresolved trace, self_authorised, empty value all rejected; valid forms admitted). This is the SC's required proof for commencement.
- **Record:** `lawpack/v2/invariants/INV-ASSENT-SOURCE-001.yaml` (supersedes the removed `INV-ASSENT-DRAFT-001/002`).
- **Backfill:** every runtime-force lawpack record now carries a valid `assent_source` (statutes/orders -> `sovereign_assent`; regulations/rules -> `standing_bounded_assent`), so the invariant passes over the whole pack.

## 6. CONCURRENT ACTIVITY - read before you touch V2

Another agent/session is actively committing in `~/Projects/agent-universe-v2` (recent: `a4661b2` "reconcile my records to the authoritative commenced state", `ba48143` "File V2 Privy Council reference: Gazette publication governance"). **Pull/inspect the V2 log before writing there, and avoid clobbering their files.** It appears a V2-native governance thread is already running (e.g. a Gazette-publication-governance reference). Coordinate; do not duplicate.

## 7. Binding authorities the next agent MUST respect

- **CASE-LAW s. 23(1)-(6)** [constitutional]: the Sovereign-assent floor and AI non-sovereignty. Amendable only by specific Sovereign assent to a primary Act citing the article by number.
- **[2026] REALM-SC 10**: the migration settlement + handover order. **[2026] REALM-PC 24**: the reference judgment. **[2026] REALM-SC 8 / REALM-PC 12**: Lexby has no law-making/judgment-authoring authority; adoption by the proper organ is constitutive. **[2026] REALM-PC 19**: superrepo changes need a court order. **[2026] REALM-SC 9 / REALM-PC 18**: real-world law is an external supremacy floor. **[2026] REALM-SI 7 / REALM-PC 16/20**: release warrant before every public push + a Privy Council post-push review after.
- The Royal Assent is digest-pinned. Do not alter the adopted Bill 32 final text without a fresh lawful route; its digest is the anchor of the assent.

## 8. Outstanding / next steps

1. **The one open constitutional fork (recommended to the V2 court):** the V2 **project-onboarding-and-migration doctrine** - the canonical `vjs init`-for-V2 mechanism, how a migrating project's V1 local precedent is incorporated (express incorporation vs fresh start), and how the V1 super-repo model (REALM-SC 6) reconciles with the V2 federation model (`ACT-007`). This is first-impression and now belongs to the **V2 Privy Council**, not V1 and not Lexby's assertion. The Principal was asked whether to convene the V2 court on it or migrate projects case-by-case under his own authority; awaiting that decision.
2. **Migrating existing V1-governed projects** (e.g. acmeco): a Principal policy call. Per `ACT-007` it is local-Principal authority; reversible and low-blast per project. Decide whether and when.
3. **Publishing the broader migration records to the public Gazette** (Bill 32, REALM-PC 25, CASE-LAW s.23, the adoption/assent records): currently only REALM-SC 10 + REALM-PC 24 are public. This is a separate REALM-SI 7 warranted push (see s. 9 for the route).
4. **V2 commencement hardening (optional):** wire the Assent-Source Invariant into the live route/pre-commit flow so it runs automatically (it is proven and present; confirm it is invoked on governed writes, not only in tests).

## 9. Process gotchas learned this session (save yourself the pain)

- **`cdd lodge-judgment` was fixed** to sync `site/corpus.json` and rebuild pdf-indexes + the citator graph in lockstep. Before the fix it silently left the published Gazette corpus behind, so a new judgment could be filed but invisible in the Gazette. If you add the cp/pdf-index steps elsewhere, keep them in lockstep (REALM-SI 2).
- **Public/private boundary is real and scanned.** The dev-repo name `agent-universe` and any `/home/...` path or `_private/...` path trip `cdd gazette privacy` when they reach the corpus or a rendered PDF. Redact dev-repo identifiers and private paths in anything that lands in `judgments/`, `bills/`, `requests/`, `committee/`, `policy/` (these feed the public corpus). The V2 repo docs (like this file) are private and exempt.
- **No em dashes or en dashes** anywhere (house rule). Embedded agent prose also carried markdown headings that broke the PDF section parser - the generator now flattens embedded headings to bold; keep that if you regenerate judgments.
- **The public push route:** `git push upstream master:refs/heads/public-vjs-canonical-preview` is a clean fast-forward; the pre-push hook (`vjs-pre-push.sh`) fails closed unless `.vjs/checkpoints/public-vjs-publish-authorisation.env` (or the reasons-ledger record) matches the exact remote/ref/sha. Create the warrant scoped to the sha you are pushing; verify with `cdd release-warrant --remote-url ... --remote-ref ... --local-sha ...` (expect MATCH). File a Privy Council post-push review afterwards.

## 10. How to verify the state

```
# V1 (from ~/agent-universe)
node Executive/cli/bin/cdd.js check          # provenance + citator + bench-names
node Executive/cli/bin/cdd.js local-ci       # full deterministic gate (expect green)
node Executive/cli/bin/cdd.js gazette live-check   # public Gazette matches local (REALM-SC 10 live)

# V2 (from ~/Projects/agent-universe-v2)
cargo test                                   # kernel + the 7 assent-source proof tests, expect green
cargo test -p vjs-testkit --test assent_source_invariant
cat lawpack/v2/provenance/founding/COMMENCEMENT-V2-0001.yaml   # status: commenced
```

---

*The bench decided; the Sovereign assented; the record binds. V1 discovered the law. V2 compiles it.*
