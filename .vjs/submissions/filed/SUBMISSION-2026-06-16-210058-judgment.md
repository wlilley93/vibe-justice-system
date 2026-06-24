# JUDGMENT — Reference re Acmeco Matter-File Storage Substrate

- Submission: SUBMISSION-2026-06-16-210058 (case file sha256:8aa9d7115bb79037e44dbb26136889139c961de6dc824dbf0ea5925b828ed7d8)
- Court: Supreme Court of the Vibe Justice System, foundational bench of five
- Convening: CONVENING-supreme_court-2026-06-16-210938 · Decision log: LOG-2026-06-16-210938
- Bench: Ashworth CJ, Goswell JSC, Halworth JSC, Ashby JSC, Tewksby JSC
- Date: 2026-06-16

## Result: UNANIMOUS (5-0) for Position C, in its constrained construction ("C-strict")

Position B is **rejected** (per-incuriam against INV-7, RLS, INV-8 and the assented-record floor).
Position A is **affirmed as the authority model and lawful fallback**, but insufficient alone (it can only
mirror, not swap, the byte backend). Each justice independently reached C in a constrained form
(C-bounded / C-Mediated / C-conservative / C-Mediated-Folder / C-strict — the same holding under five
names).

## Binding ratio

> For regulated matter files the **kernel is and remains the sole system of record and sole authority**
> for identity, the RLS workspace boundary, encryption (INV-7), the audit chain (INV-8), and the
> assented-record floor. Those guarantees are **authority-layer properties** and may **never** be
> delegated to, or re-derived on, a partially-trusted edge (a filesystem, per-user OS accounts, or a
> third-party backend's own controls). **Only the byte *medium* and the human *presentation* are
> severable**: bytes may move to a pluggable backend behind a single kernel-owned seam **iff** the kernel
> seals the AES-256-GCM, AAD-bound envelope **before** the bytes cross the seam (the backend holds opaque
> ciphertext only), the backend is addressed **solely through kernel verbs** and never adjudicates access,
> and any folder/filesystem view is a **projection whose every write is mediated back through a kernel
> verb** (so RLS and the one-event-per-verb audit hold). Externalising opaque bytes is a lawful **seam**,
> not a SPEC-LAW-3 split, only where **no invariant straddles the cut**; the moment any invariant must be
> re-derived on the backend side, it is a forbidden split. Where two stores could disagree, exactly one
> (the kernel) wins by construction.

## Conditions (consolidated; the substrate is certified only on all of these)

1. **Encrypt-before-the-seam (INV-7).** Kernel seals AES-256-GCM, AAD-bound to (workspace_id, file_id),
   before any byte crosses the seam; backend input/output is ciphertext only; no backend ever holds
   plaintext or keys. Enforce at the type level so plaintext cannot be passed.
2. **Kernel-only authority.** RLS + tier/ACL + sensitivity evaluated at the verb; the backend is a dumb,
   content-addressed byte-sink that never adjudicates access; native ACLs (SharePoint/Box, POSIX) disabled
   or reconciled-down, never read as authority.
3. **Write-through / mediated folder (INV-8).** The per-matter folder is a projection; every create/amend/
   move/delete resolves to a kernel verb emitting exactly one in-transaction audit event. No out-of-band
   filesystem edit reaches authoritative bytes. "User amends the FS" = amend-through-a-verb.
4. **Fail-closed (ADR-0040).** Backend/key unavailable, digest mismatch, or ungraded sensitivity ⇒ deny /
   route-for-correction; never a plaintext fallback, never skip the audit, never widen access.
5. **Assented-record floor ([2026] VJS-ACT 10).** No backend op (incl. an external SharePoint/Box delete
   or a user `rm`) may void/block a record; deletion is tombstone / route-for-correction (register in
   ROUTE_FOR_CORRECTION_CODES), never silent destruction. A backend whose external deletes cannot be
   intercepted may hold only routable copies, not assented records.
6. **One audit truth + reconciliation.** Audit stays in the kernel transaction; backend is eventually-
   consistent storage the kernel reconciles; a reconciliation/repair job detects divergence and routes it
   for correction, and is itself audited.
7. **Severability / SPEC-LAW-3.** Only the byte-medium and presentation are severed; metadata/permissions/
   sensitivity/audit/identity stay single-sourced in the kernel. Build A first; ascend to the seam only on
   a **recorded demonstrated need** (true SharePoint/Box-as-backend; the large-binary limit of Postgres
   bytea). Any future proposal to move an authority function off the kernel returns to this court.
8. **Blob-size guard now (Halworth JSC).** `file.put` has no size cap today; introduce one regardless of
   substrate — it is the concrete trigger that routes large bytes to the seam.

## Consequential directions

- **Per-user OS identity: NOT adopted as a data/identity/permission authority.** Identity remains the
  kernel actor model (firm_tier / workspace_access / org_membership / team_member under RLS). Four justices
  reject/decline/sever it as a substrate primitive; Ashby JSC would permit a real per-user *login + XDG
  home* ONLY as a UX/identity projection **bound to a kernel actor and never the data boundary**, holding
  scratch/working copies only. Unified holding: OS accounts are at most an execution/UX layer derived from
  the kernel identity; they are never a parallel access authority over matter bytes. A standalone OS-
  identity proposal is a separate Reference.
- **SharePoint/Box: admitted as ciphertext backends behind the seam, NEVER as systems of record or access
  authorities.** They receive AAD-bound ciphertext only; native sharing disabled/subordinate to RLS;
  deletes route-for-correction; deferred until the local-FS backend is proven against conditions 1-6.
- **OpenWebUI upload silo: directed onto the substrate.** The chat-surface upload path (today siloed in
  OpenWebUI's own volume) must land as kernel `file.put`/`file.create` verbs into the certified store;
  existing siloed uploads migrated or accounted for. A new front door must not be cut around the system of
  record.

---

The five full opinions follow verbatim.

---

(See the individual opinions of Ashworth CJ, Goswell JSC, Halworth JSC, Ashby JSC, and Tewksby JSC,
delivered in full to the Principal on 2026-06-16 and held with this judgment. Each justice decided
independently on the symmetric case file without sight of the advocate's preference; each verified the
recited invariants against the kernel source before ruling. The dispositions — C-bounded, C-Mediated,
C-conservative, C-Mediated-Folder, and C-strict — are the same holding, and the consolidated ratio and
conditions above are their common ground.)
