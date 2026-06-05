# Court workflows

The three runnable court workflows for the Vibe Justice System. Each is a Claude Code Workflow script
invoked via the `Workflow` tool. They are general-purpose: they work for any project, any fork, any breach.

---

## The three tiers

| File | Tier | Bench | When to use |
|------|------|-------|-------------|
| `workflows/first-instance.js` | First Instance | 1 judge | All matters start here (VPR 1). Standing check + fast-path on binding precedent. Full deliberation only for genuine first-impression questions. |
| `workflows/court-of-appeal.js` | Court of Appeal | 3 judges | After First Instance, with permission to appeal. Three independent postures: strict-construction, pragmatist, precedent-hawk. |
| `workflows/supreme-council.js` | Supreme Council | 5 judges (9 for constitutional) | The apex court. Only reached by progression or the Principal's express leapfrog certificate. The only court that can enact new SPEC-LAW. |

---

## How to invoke

Pass the matter as `args` to the Workflow tool:

```js
// Request for Ruling
Workflow({
  scriptPath: 'court/workflows/first-instance.js',
  args: {
    kind: 'request_for_ruling',
    question: 'Should we use server-side rendering or a SPA for this project?',
    spec: '...contents of SPEC-LAW.md...',
    caselaw: '...contents of caselaw/INDEX.md...',
  }
})

// Breach
Workflow({
  scriptPath: 'court/workflows/first-instance.js',
  args: {
    kind: 'breach',
    charge: 'The authentication layer was implemented without a prior ruling, contrary to an existing precedent on this point.',
    spec: '...contents of SPEC-LAW.md...',
    caselaw: '...contents of caselaw/INDEX.md...',
  }
})
```

In practice, `submit-request-to-court` and `submit-breach-to-court` (via Lexby) handle this automatically.
The workflows are the engine under those commands.

---

## What comes back

Every workflow returns a **ruling artefact** with:
- `citation_id` - the neutral citation to commit to `caselaw/`
- `tier` - which court sat
- `judge` / `panel` - who deliberated
- `ratio` - the binding holding (one precise statement)
- `obiter` - non-binding observations (persuasive only)
- `per_incuriam` - flag if the ruling missed binding law
- `remedy` - for breach matters (make good, restore the position)
- `lexby_translation` - plain English

Commit the artefact to `caselaw/` and add a row to `caselaw/INDEX.md`. The ruling then governs all future
sessions via the fast-path screen (VPR 2).

---

## Progression rules (VPR 3)

You cannot start at the Court of Appeal or Supreme Council directly. Run `first-instance.js` first.
To escalate, pass the prior ruling as `args.lower_ruling` to the next tier's workflow. The leapfrog to
Supreme is only available with the Principal's express certificate, recorded in `args.leapfrog_certificate`.

---

## Community Record (VPR 8)

Every workflow automatically opens a pull request to the canonical VJS repo (`wlilley93/vibe-justice-system`,
path `community/caselaw/YYYY/`) after delivering its ruling. The submission is anonymised: project-specific
identifiers (repo names, file paths, variable names, function names) are replaced by generic placeholders.
The legal question, ratio, law applied, and outcome are preserved.

The clerk reviews each PR for constitutional compliance and subject matter jurisdiction before merging.
Community rulings are persuasive precedent across all VJS jurisdictions.

The `communityPrUrl` field in the return value carries the PR URL when the submission succeeds.
