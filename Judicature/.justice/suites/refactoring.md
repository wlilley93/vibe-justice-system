# VJS Refactoring Suite

> **Legal status.** The durable PRINCIPLES of this suite are law: **The Refactoring Suite Instrument 2026**
> ([2026] REALM-SI 3, under Bill 5 s. 18), made by the Standing Committee, owned by the Ministry of Business,
> Engineering and Skills (MBES) under Bill 27 s. 5B. This file is the **operational expression** of that
> Instrument: the concrete checks the Ministry maintains and amends as engineering. The Instrument governs;
> where this file and the Instrument diverge on a principle, the Instrument prevails. A court ruling that
> identifies a refactoring-practice gap updates this suite (and, where it states a new durable principle, the
> Instrument by amendment) carrying the ruling's citation.

## When to invoke

Invoke this suite when:
- A court ruling's remedy order includes a refactoring obligation
- A breach finding identifies code quality as a contributing factor
- You are executing a remediation order for work found below standard
- The principal explicitly requests a supervised refactoring pass under VJS

Do NOT invoke for routine feature work, new additions, or cosmetic changes with no ruling obligation.

## Checks

Apply checks scoped to the extent stated in the ruling's remedy. Do not extend the refactoring beyond that
scope - surface further issues discovered in passing as a new Request for Ruling.

### 1. Respect the ruling scope

Apply changes exactly to the scope identified in the ruling. If broader issues are found, file a new Request
for Ruling - do not silently fix them.

### 2. Naming and clarity

- Public functions, classes, and variables named for what they do, not how they do it
- No single-letter variables outside tight loops
- Boolean-returning functions named with `is`, `has`, or `can` prefix

### 3. Single responsibility

- Each function does one thing; side effects are explicit in the name or signature
- No function over ~40 lines without a clear structural reason
- No file handling unrelated concerns in the same module

### 4. Dead code and duplication

- Unused exports, unreachable branches, and dead variables removed
- Repeated logic extracted only when three or more sites share it (rule of three)
- No backwards-compatibility stubs for callers that do not exist

### 5. Test coverage at the changed surface

- Existing tests still pass after refactoring
- Any behaviour change (not just a rename) is covered by at least one test

### 6. Atomicity

- Each logical change committed separately with a clear commit message
- Refactoring commits separate from behaviour-change commits

## Updating this suite

A court ruling that identifies a gap in refactoring practice updates this suite as part of its remedy. The
executing agent adds the new check with the ruling citation. Any project member may propose additions via PR.
