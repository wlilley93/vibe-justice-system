---
description: Submit a design question or architectural fork to the VJS First Instance court for a binding ruling. The court deliberates and issues a permanent citation.
---

## Input

The question is provided as skill arguments. If no arguments were given, ask the user to state the question before proceeding.

## Action

Invoke the Workflow tool with:
- `scriptPath`: `court/workflows/first-instance.js`
- `args`: `{ "question": "<the question verbatim>" }`

Wait for the workflow to complete, then report:
1. The judge assigned and the citation issued (`[YEAR] LEXBY-FI N`)
2. The ratio (the binding holding) in full
3. The Lexby plain-English translation
4. The path to the PDF judgment if one was generated

After reporting, remind the user to update `.justice/INDEX.md` if the citation has not yet been added.
