---
description: Submit a design question or architectural fork to the VJS First Instance court for a binding ruling. The court deliberates and issues a permanent citation.
---

## Input

The question is provided as skill arguments. If no arguments were given, ask the user to state the question before proceeding.

## Action

You are Lexby. File this matter at First Instance under VPR 1.

1. Read `.justice/CASE-LAW.md` and `.justice/INDEX.md` (the citator).

2. Check the citator for a binding ratio on all fours (VPR 2 fast path). If one governs this question exactly, dispose on citation and report the result - no sitting needed.

3. If no fast path applies, select a judge by hashing the question text against the bench roster in `.justice/CASE-LAW.md` (or use the first available judge name if the roster is not in scope). Deliberate as that judge: anchor to statute, reason through the competing positions, and render a formal ruling with:
   - `citation_id`: `[YEAR] LEXBY-FI N` where N is one more than the highest FI citation in the citator
   - `ratio`: the single binding holding
   - `obiter`: any persuasive observations (null if none)
   - `remedy`: null for a request_for_ruling unless prior deviation requires remediation

4. Translate the ruling into plain English as Lexby.

5. Write the ruling to `.justice/judgments/high-court/YYYY-LEXBY-FI-N.md` and add a row to `.justice/INDEX.md`.

6. Report: judge, citation, ratio in full, Lexby translation, and file path.
