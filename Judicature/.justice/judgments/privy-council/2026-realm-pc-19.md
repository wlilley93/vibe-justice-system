---
citation_id: "[2026] REALM-PC 19"
tier: privy-council
kind: request_for_ruling
status: good-law
per_incuriam: false
date: 2026-06-07
panel: ["Coade J", "Goffe J", "Sumberly J"]
seised_by: "Sovereign Founder reference: whether all superrepo changes require court order before implementation"
cause_title: "In the matter of superrepo edit control, court orders, and public-entrypoint discipline"
registrar_note: "Authored by the bench (Coade J for the Court, Goffe J and Sumberly J concurring); reduced to the filed record by Lexby as s.18(4) registrar, the decision pre-existing the prose ([2026] REALM-SC 8). Bench-name conformance only: invented VJS names replace non-VJS real jurist labels; no change to ratio, status, citation, vote, or legal force."
---

# [2026] REALM-PC 19

| Field | Value |
|-------|-------|
| **Citation** | [2026] REALM-PC 19 |
| **Tier** | Privy Council (constitutional first instance, bench of three) |
| **Before** | Coade J (judgment of the Court), Goffe J, Sumberly J |
| **Kind** | Request for ruling |
| **Status** | good-law |
| **Cites** | CASE-LAW s. 1; s. 2; s. 3; s. 5; s. 6; s. 8; s. 9; s. 13; s. 18; s. 19(1)/(5); Bill 8; Bill 16; Bill 20; Bill 22; Bill 27; Bill 30; [2026] REALM-PC 16; [2026] REALM-PC 17; [2026] REALM-PC 18; [2026] REALM-SC 9 |

> The Court orders a superrepo-change gate. Changes to the canonical VJS superrepo must be referred to
> court for an order before implementation, and any public VJS push must continue to use the public-release
> warrant. Agent-facing files must carry a reminder. Unanimous (3-0).

## Question

Given that this repository is the canonical VJS superrepo, must all changes to it be referred to court for an
order before implementation, and must that reminder appear in both the agent and Claude binding instructions?

## Ratio (binding)

1. The canonical VJS superrepo is system-governing infrastructure. A change to it can alter public law,
   public case law, install-time agent duties, public release machinery, or the public entrypoint. It is
   therefore not an ordinary project edit.

2. Before implementing any change to the canonical VJS superrepo, the agent must identify the intended change,
   check the citator, and obtain a court order unless the change is already expressly ordered by binding
   precedent or statute. Where an existing order covers the change on all fours, the agent cites that order and
   may proceed by fast path.

3. The required order may be a request-for-ruling, breach order, referral order, or release/order conformance
   ruling appropriate to the change. The order must state what is authorised and the legal basis for it.

4. This does not make the court a ceremony for trivial local typing. Pure execution steps that are necessary
   to implement an existing order, including formatting, rendering PDFs, rebuilding derived indexes, or pushing
   the authorised change, proceed under the cited order. A new load-bearing choice, new public-law statement,
   new public-boundary change, new agent duty, new Pages/publication route, or new superrepo governance rule
   needs its own order.

5. The rule must be written into the agent-facing instructions in this repository: `Constitution/AGENTS.md`
   and `Executive/plugin/CLAUDE.md`. The reminder must be near the intake/convening instructions so agents see
   it before editing.

6. `agent-universe` is the private development superrepo. The public entrypoint is `vibe-justice-system`.
   Public-facing links and release language should direct users to the public VJS repository and its GitHub
   Pages law site, not to the private development repository.

## Reasons

The superrepo is different from a normal project repo. It contains the constitutional record, the public court
record, the legislature, the installer, hooks, law-report site, and public-release gate. A small change can
become a change to how every downstream installation understands correctness.

The answer is not to freeze the repo. The answer is to force a reminder at the moment of action: check the
citator, identify whether an existing order authorises the work, and obtain a court order where it does not.
That keeps the system practical while preventing silent constitutional edits.

The public-entrypoint point follows from [2026] REALM-PC 17 and Bill 30. Canonical VJS can be public; the
development superrepo can remain private. Users should enter through VJS, not through the private development
repo.

## Disposal

Order made. The current work to add the GitHub Pages law site, make law and case law easier to find, conform
the README to PDF-first law links, add the real-world-law provision, and add this superrepo reminder is
authorised under this order and [2026] REALM-SC 9. The repo is to be conformed by updating
`Constitution/AGENTS.md` and `Executive/plugin/CLAUDE.md`.
