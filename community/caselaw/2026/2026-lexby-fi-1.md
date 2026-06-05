╔══════════════════════════════════════════════════╗
║       IN THE FIRST INSTANCE COURT OF LEXBY       ║
║           [2026] LEXBY-FI 1                      ║
╚══════════════════════════════════════════════════╝
Judge: Bowan J
Result: Request for Ruling allowed - <project> is fit for alpha release under the standard of reasonable skill and care

## Ratio

A software system implementing a justice or governance framework is fit for alpha release under the standard of reasonable skill and care (S-4, S-5) where: (a) the core legal or logical model is demonstrably instantiated and internally coherent; (b) the governing rules, procedure, and founding caselaw are committed and self-consistent; (c) constitutional enforcement automation is in place; (d) the known gaps are disclosed, do not undermine the system's logical coherence, and are appropriate to remediate before v1. Convenience-layer tooling (CLI commands, packaging, renderers) is not a prerequisite for alpha. Deterministic citation or identifier numbering is the highest-priority gap before v1 on the grounds that it is a prerequisite for citator integrity under S-11(d).

## Obiter

Before v1 (non-alpha), the minimum closure set in order of legal priority is: (1) deterministic citation/identifier numbering (S-11(d) integrity - necessary condition for v1); (2) user-facing submit commands for requests and breach filings (practitioner surface - necessary condition for v1); (3) cite/lookup command for the citator (practitioner tooling - strongly advisable); (4) ruling card renderer (output legibility - strongly advisable); (5) CLI init command (onboarding - advisable); (6) package distribution (distribution - advisable). Items 1 and 2 are necessary conditions for v1; items 3 through 6 are strongly advisable but their absence alone would not defeat v1 readiness.

## Lexby TL;DR

The <project> governance framework at the named commit has a complete statute book, complete procedure rules, runnable court workflows, constitutional review automation, and founding caselaw. That is enough for alpha: the concept is built, the logic is coherent, the gaps are named and do not break what is there. The missing pieces - user commands, citation numbering, packaging - are the remaining work before a production (v1) release, with citation numbering being the most legally critical because without it the case record can become ambiguous.

## Law Applied

S-1 (two sources, supremacy), S-4 (tortious duty, neighbour principle), S-5 (reasonable skill and care standard, Bolam), S-7 (no-statute case), S-8 (novel first failure, forward duty to spec), S-11(d) (neutral citation form, citator integrity), S-14 (subject matter jurisdiction)
