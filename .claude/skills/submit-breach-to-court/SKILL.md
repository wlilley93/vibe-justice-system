---
description: File a breach charge with the VJS First Instance court. Use this when your own work or a prior decision fell below the applicable standard of care.
---

## Input

The breach description is provided as skill arguments. It should state:
- What fell below standard
- Why it fell below standard (what the standard was and how the work deviated from it)

If no arguments were given, ask the user to describe the breach before proceeding. Remind them: self-submission is correct, expected, and carries no punishment - the only outcome is remediation.

## Action

Invoke the Workflow tool with:
- `scriptPath`: `court/workflows/first-instance.js`
- `args`: `{ "charge": "<the breach description verbatim>" }`

Wait for the workflow to complete, then report:
1. The judge assigned and the citation issued (`[YEAR] LEXBY-FI N`)
2. Whether breach was made out and the ratio
3. The remedy ordered (if any) - always remediation only, never punishment
4. The Lexby plain-English translation
5. The path to the PDF judgment if one was generated

After reporting, remind the user to update `.justice/INDEX.md` if the citation has not yet been added.
