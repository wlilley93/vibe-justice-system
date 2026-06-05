export const meta = {
  name: 'vjs-first-instance',
  description: 'Vibe Justice System - First Instance court. Adjudicates any Request for Ruling (design fork) or Breach (negligence charge) for any project. Single judge, standing check, fast-path on binding precedent, full deliberation for genuine first-impression matters. Lexby translates.',
  phases: [
    { title: 'Law Load', detail: 'Read SPEC-LAW.md and .justice/INDEX.md from the repo - the court is always bound to the current law, never a stale copy' },
    { title: 'Intake', detail: 'Assign a judge from the permanent bench and check standing (VPR 1 / s. 11)' },
    { title: 'Fast-path screen', detail: 'Check whether a binding ratio on all fours disposes the matter without a sitting (VPR 2 / s. 11)' },
    { title: 'Deliberation', detail: 'If no fast-path: the assigned judge deliberates and renders a ruling in formal legalese (skipped on fast-path)' },
    { title: 'Translation', detail: 'Lexby translates the ruling into plain English for the record' },
    { title: 'PDF Render', detail: 'Render the judgment as a PDF using the court/renderer engine' },
  ],
}

// --------------------------------------------------------------------------
// Bench roster (permanent seats, temperaments per DESIGN-NOTES.md)
// --------------------------------------------------------------------------
const BENCH = [
  {
    name: 'Hallam CJ',
    temperament: 'Principled and precise. Always anchors to statute first. Opens with the governing provision before examining any facts.',
  },
  {
    name: 'Goffe J',
    temperament: 'Pragmatist. Tests every ruling for real-world workability before it is recorded. Will not leave a ratio that produces absurd results in the next case.',
  },
  {
    name: 'Blackmere J',
    temperament: 'Textualist. Holds hard to the literal words of SPEC-LAW. Resists implications that stretch beyond the plain text.',
  },
  {
    name: 'Sumberly J',
    temperament: 'Formalist. Insists on procedural correctness above all. Will dismiss on standing or fast-path grounds before touching the merits.',
  },
  {
    name: 'Elden J',
    temperament: 'Historically-minded. Draws on precedent and tradition. Anchors the ratio to what prior courts have held wherever possible.',
  },
  {
    name: 'Coade J',
    temperament: 'Restrained and cautious. Prefers the narrowest ruling that decides the case. Never extends the law further than the facts require.',
  },
  {
    name: 'Steyne J',
    temperament: 'Bold and willing to extend the law where it serves the principal. Looks to the purpose of the provision, not merely its words.',
  },
  {
    name: 'Bowan J',
    temperament: 'The dissenter. Tests every proposition to destruction. If the argument holds even under Bowan, it is sound.',
  },
  {
    name: 'Aldermere J',
    temperament: 'Balanced and synthetic. Often writes the final synthesis when other temperaments pull apart. Seeks the position that honours every legitimate competing concern.',
  },
]

// --------------------------------------------------------------------------
// Ruling artefact schema
// --------------------------------------------------------------------------
const RULING_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  required: [
    'citation_id',
    'tier',
    'judge',
    'kind',
    'question_or_charge',
    'standing',
    'fast_path',
    'fast_path_cite',
    'ratio',
    'obiter',
    'per_incuriam',
    'remedy',
    'status',
  ],
  properties: {
    citation_id: {
      type: 'string',
      description: 'Neutral citation in the form [YEAR] LEXBY n, e.g. [2026] LEXBY 1. Assign a sequential integer. Use the current year.',
    },
    tier: {
      type: 'string',
      enum: ['first-instance'],
    },
    judge: {
      type: 'string',
      description: 'The presiding judge, exactly as named on the bench roster.',
    },
    kind: {
      type: 'string',
      enum: ['request_for_ruling', 'breach'],
      description: 'request_for_ruling for a forward-looking fork; breach for a backward-looking negligence charge.',
    },
    question_or_charge: {
      type: 'string',
      description: 'The matter as filed: verbatim or close paraphrase of args.question / args.charge.',
    },
    standing: {
      type: 'boolean',
      description: 'True if the party has standing to bring this matter (a real party with a real question or genuine charge). False triggers dismissal.',
    },
    fast_path: {
      type: 'boolean',
      description: 'True if a binding ratio on all fours was found in the caselaw provided, disposing the matter without full deliberation.',
    },
    fast_path_cite: {
      type: ['string', 'null'],
      description: 'The neutral citation of the governing precedent if fast_path is true; null otherwise.',
    },
    ratio: {
      type: 'string',
      description: 'The one clear, binding holding of the court. This is what binds future courts. Must be a single decisive statement. Null only when standing is false.',
    },
    obiter: {
      type: ['string', 'null'],
      description: 'Observations made in passing that are persuasive but not binding. Null if nothing material to record.',
    },
    per_incuriam: {
      type: 'boolean',
      description: 'True only if the ruling was made in ignorance of binding statute or precedent. Should be false in almost every case when the court has been provided the relevant law.',
    },
    remedy: {
      type: ['string', 'null'],
      description: 'For a breach: the specific remediation and restitution ordered (never punishment). For a request_for_ruling: null unless the court finds prior deviation requiring remedy.',
    },
    status: {
      type: 'string',
      enum: ['good-law', 'per-incuriam'],
      description: 'good-law unless per_incuriam is true.',
    },
  },
}

// --------------------------------------------------------------------------
// Translation schema
// --------------------------------------------------------------------------
const TRANSLATION_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  required: ['plain_english_summary', 'what_it_means_in_practice', 'can_it_be_appealed'],
  properties: {
    plain_english_summary: {
      type: 'string',
      description: 'Plain English version of the ruling: what was asked or charged, whether it had standing, whether it was decided on precedent or fresh deliberation, and what the court held. 3-6 sentences. No legalese.',
    },
    what_it_means_in_practice: {
      type: 'string',
      description: 'Concrete, actionable translation: what the party must do (or is now permitted to do) as a result of this ruling. 1-3 sentences.',
    },
    can_it_be_appealed: {
      type: 'string',
      description: 'Brief note on appeal rights: this is a First Instance ruling, so permission to appeal to the Court of Appeal is available on an arguable point of law or a binding-precedent conflict (VPR 3 / s. 10).',
    },
  },
}

// --------------------------------------------------------------------------
// Deterministic judge selection
// Seeds on the matter text so the same question always draws the same judge,
// but different questions rotate the bench.
// --------------------------------------------------------------------------
function selectJudge(matter) {
  let hash = 0
  for (let i = 0; i < matter.length; i++) {
    hash = ((hash << 5) - hash + matter.charCodeAt(i)) >>> 0
  }
  return BENCH[hash % BENCH.length]
}

// --------------------------------------------------------------------------
// Build the SPEC-LAW block handed to every agent
// --------------------------------------------------------------------------
function buildSpecBlock(specSummary) {
  return `
SPEC-LAW (the sovereign statute, supreme throughout):
${specSummary || `
s. 1: Two sources of law: SPEC-LAW (statute, supreme) and case law (interprets statute where silent; void to the extent it conflicts with statute).
s. 2: The principal holds two offices: Sovereign/Parliament (may make or unmake any law by due process) and Prime Minister (must act lawfully as executive). Ultra vires demands must be pushed back.
s. 3: Lexby is advocate, advisor, and engineer. The bench decides; Lexby advocates; the record binds both.
s. 4: Breach is tortious (not criminal). Lexby owes a continuing duty of reasonable skill and care to every principal who relies on the work (the neighbour principle). No jurisdiction-first gate.
s. 5: Standard and breach: graded endeavours hierarchy (reasonable skill and care / all reasonable endeavours / best endeavours), pleaded and found per engagement. Bolam: conduct a responsible body of competent practice would endorse is not breach.
s. 6: Remedy = remediation and restitution only, proportionate. Punishment, fine, and sanction are unavailable in every instance.
s. 7: No-statute case: silence in SPEC-LAW fixes the standard at reasonable care; the matter is justiciable from the first act.
s. 8: One continuous standard (first/second-time rule is repealed). A genuinely novel first failure ordinarily founds no breach and triggers a forward duty to spec and remediate; recurrence of a logged hazard is breach.
s. 9: Unitary sovereignty: one global SPEC-LAW, jurisdiction-local case law, no competing sovereigns.
s. 10: Court structure: First Instance (1 judge) -> Court of Appeal (3) -> Supreme Court (5, or full 9 for constitutional/foundational questions). Permission to appeal is mandatory between tiers.
s. 11: Procedural gates: (a) standing at intake; (b) permission to appeal between tiers; (c) precedent fast-path on all-fours binding ratio; (d) neutral citation [YEAR] LEXBY n; (e) ratio binds, obiter persuades, per incuriam voids; (f) declaration of incompatibility refers case law conflicting with SPEC-LAW upward.
s. 12: Anti-bloat: no juries, no costs, no interlocutory.
s. 13: Rule-based progression (no leap-frogging). Every matter commences at First Instance. Sole exception: Principal's express leapfrog certificate.
`}`.trim()
}

// --------------------------------------------------------------------------
// Workflow
// --------------------------------------------------------------------------

// args may arrive as a JSON-encoded string depending on the host runtime; coerce to an object
// before any field access (otherwise args.question is undefined and selectJudge crashes).
if (typeof args === 'string') { try { args = JSON.parse(args) } catch (_) {} }
if (!args || typeof args !== 'object') args = {}

// Law Load - always read the current law from the repo first.
// args.spec and args.caselaw are optional fallbacks for headless/test runs.
phase('Law Load')
const lawLoad = await parallel([
  () => agent(
    'Read the file .justice/SPEC-LAW.md in the current working directory. If that file does not exist, read SPEC-LAW.md instead. Return the complete text verbatim with no commentary or summary.',
    { label: 'load SPEC-LAW', phase: 'Law Load', agentType: 'Explore' }
  ),
  () => agent(
    'Read the file .justice/INDEX.md in the current working directory. If that file does not exist, try caselaw/INDEX.md instead (legacy fallback). Return the complete text verbatim with no commentary or summary.',
    { label: 'load .justice/INDEX.md', phase: 'Law Load', agentType: 'Explore' }
  ),
])
const liveSpec = (lawLoad[0] && lawLoad[0].trim()) || null
const liveIndex = (lawLoad[1] && lawLoad[1].trim()) || null
if (liveSpec) log('SPEC-LAW loaded from repo.')
else log('SPEC-LAW not found in repo - using built-in fallback.')
if (liveIndex) log('.justice/INDEX.md loaded from repo.')
else log('.justice/INDEX.md not found - no precedents available.')

// Clerk: deterministic citation numbering (mirror of cli/lib/citation.js; the Workflow sandbox has no require).
const VJS_YEAR = (typeof args !== 'undefined' && args && args.year) || 2026
function vjsAssignCitation(citatorText, code, year) {
  const re = new RegExp('\\[' + year + '\\]\\s*LEXBY-' + code + '\\s+(\\d+)', 'gi')
  let max = 0, m
  while ((m = re.exec(citatorText || '')) !== null) { const n = parseInt(m[1], 10); if (n > max) max = n }
  return '[' + year + '] LEXBY-' + code + ' ' + (max + 1)
}
const assignedCitation = vjsAssignCitation(liveIndex, 'FI', VJS_YEAR)
log('Clerk assigned citation: ' + assignedCitation)

const matter = args.question ?? args.charge
const kind = args.question ? 'request_for_ruling' : 'breach'
const judge = selectJudge(matter)
const specBlock = buildSpecBlock(liveSpec || args.spec)
const caselawBlock = liveIndex
  ? `CASELAW INDEX (current, read directly from repo):\n${liveIndex}`
  : args.caselaw
    ? `CASELAW PROVIDED (fallback from caller):\n${args.caselaw}`
    : 'CASELAW: none available. This is either a new repo or INDEX.md has not been created yet.'

// --------------------------------------------------------------------------
// Phase 1: Intake - standing check + judge assignment (fast; no schema needed)
// --------------------------------------------------------------------------
phase('Intake')
const intakeNote = `Judge assigned by deterministic draw: ${judge.name}. Temperament: ${judge.temperament}.`

// --------------------------------------------------------------------------
// Phase 2: Fast-path screen
// --------------------------------------------------------------------------
phase('Fast-path screen')

const FAST_PATH_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  required: ['standing', 'standing_reason', 'fast_path', 'fast_path_cite', 'fast_path_reason'],
  properties: {
    standing: {
      type: 'boolean',
      description: 'Does the filing party have standing? There must be a real party with a real question (for a request_for_ruling) or a genuine, particularised charge in negligence (for a breach). A hypothetical, academic, or abstract matter lacks standing.',
    },
    standing_reason: {
      type: 'string',
      description: '1-2 sentences explaining the standing finding.',
    },
    fast_path: {
      type: 'boolean',
      description: 'Is there a binding ratio on all fours in the caselaw provided that governs this precise question, leaving no genuine room for distinction? If yes, the matter is disposed of on citation without a sitting.',
    },
    fast_path_cite: {
      type: ['string', 'null'],
      description: 'The neutral citation of the governing precedent if fast_path is true. Null otherwise.',
    },
    fast_path_reason: {
      type: 'string',
      description: 'Explain why the fast path does or does not apply. If it does, state exactly how the ratio of the cited case governs this matter on all fours.',
    },
  },
}

const screen = await agent(
  `You are ${judge.name}, sitting at First Instance in the Vibe Justice System.

Your judicial temperament: ${judge.temperament}

THE MATTER FILED:
Kind: ${kind === 'request_for_ruling' ? 'Request for Ruling (forward-looking fork)' : 'Breach (charge in negligence)'}
Question/Charge: ${matter}

${specBlock}

${caselawBlock}

VPR RULES GOVERNING THIS SCREEN:
- VPR 1: Check standing at intake. A non-party cannot conjure a sitting. For a request_for_ruling, there must be a real decision to make with real consequences. For a breach, there must be a genuine, particularised allegation of duty / standard / falling-below.
- VPR 2: If a point is governed by binding ratio on all fours in the caselaw above, dispose of it on citation. Do NOT convene a sitting if the precedent is clear and squarely on point.

Conduct the intake screen. Assess standing first. If standing fails, the matter is dismissed (fast_path is irrelevant: set it to false and fast_path_cite to null). If standing is established, search the provided caselaw for a binding ratio on all fours. Report your findings.`,
  { label: `${judge.name} - intake screen`, phase: 'Fast-path screen', schema: FAST_PATH_SCHEMA }
)

// --------------------------------------------------------------------------
// Phase 3: Deliberation (conditional: only if standing AND no fast-path)
// --------------------------------------------------------------------------
let ruling = null

if (screen && screen.standing && !screen.fast_path) {
  phase('Deliberation')

  ruling = await agent(
    `You are ${judge.name}, sitting at First Instance in the Vibe Justice System.

Your judicial temperament: ${judge.temperament}

THE MATTER:
Kind: ${kind === 'request_for_ruling' ? 'Request for Ruling (forward-looking fork)' : 'Breach (charge in negligence)'}
Question/Charge: ${matter}

INTAKE FINDINGS:
- Standing: established. Reason: ${screen.standing_reason}
- Fast-path: not available. ${screen.fast_path_reason}

${specBlock}

${caselawBlock}

CHARGE TO THE BENCH:
This is a matter of first impression (or genuine distinction from existing precedent). You must deliberate and render a ruling. Speak in formal legalese. The style is dense, precise, and judicial: cite your statutory authority (S-n) for every proposition; reason through the elements before reaching the ratio; record any obiter observations separately.

FOR A REQUEST FOR RULING:
- State the question precisely as filed.
- Identify the governing statutory provisions.
- Deliberate on the competing approaches or positions the question presents.
- Render a declaratory ruling: which approach is permitted / required / forbidden and why.
- The ratio must be a single clear holding that binds future courts facing the same fork.

FOR A BREACH:
- Identify the duty (s. 4, the neighbour principle).
- Identify the applicable standard of endeavours (s. 5): reasonable skill and care is the default; state if a higher standard applies.
- Find the facts of the alleged falling-below.
- Determine whether the falling-below constitutes breach (s. 5: is it conduct a responsible body of competent practice would endorse? If yes: no breach).
- If breach is made out: order remedy under s. 6 (remediation and restitution only; no punishment).
- If no breach: dismiss the charge with reasons.

STRUCTURAL REQUIREMENTS:
- Use the neutral citation the clerk has assigned deterministically from the citator: ${assignedCitation}
- The citation_id field must equal that exactly (tiered form [YEAR] LEXBY-FI n).
- tier must be "first-instance".
- judge must be your name exactly: ${judge.name}
- kind must be "${kind}"
- standing must be true (already found at intake).
- fast_path must be false (already found at intake).
- fast_path_cite must be null.
- per_incuriam must be false (you have been given the governing statute and all available precedent).
- status must be "good-law".
- remedy: for a breach finding, state the specific remediation ordered. For a request_for_ruling, set to null unless prior deviation has been admitted and requires remedy.
- No em dashes or en dashes anywhere in the ruling text.`,
    { label: `${judge.name} - deliberation`, phase: 'Deliberation', schema: RULING_SCHEMA }
  )
} else if (screen && screen.standing && screen.fast_path) {
  // Fast-path disposal: construct the ruling artefact from the screen findings
  // without a full deliberation phase (VPR 2 compliant).
  ruling = {
    citation_id: '[2026] LEXBY 1',
    tier: 'first-instance',
    judge: judge.name,
    kind,
    question_or_charge: matter,
    standing: true,
    fast_path: true,
    fast_path_cite: screen.fast_path_cite,
    ratio: `Disposed on citation: the matter is governed on all fours by ${screen.fast_path_cite}. ${screen.fast_path_reason}`,
    obiter: null,
    per_incuriam: false,
    remedy: null,
    status: 'good-law',
  }
} else if (screen && !screen.standing) {
  // No standing: dismissal ruling
  ruling = {
    citation_id: '[2026] LEXBY 1',
    tier: 'first-instance',
    judge: judge.name,
    kind,
    question_or_charge: matter,
    standing: false,
    fast_path: false,
    fast_path_cite: null,
    ratio: `Matter dismissed at intake for want of standing. ${screen.standing_reason}`,
    obiter: null,
    per_incuriam: false,
    remedy: null,
    status: 'good-law',
  }
}

// Clerk assigns the binding citation deterministically from the citator (overrides any model-supplied value).
if (ruling) ruling.citation_id = assignedCitation

// --------------------------------------------------------------------------
// Phase 4: Translation (Lexby plain-English render)
// --------------------------------------------------------------------------
phase('Translation')

const translation = await agent(
  `You are Lexby: the principal's counsel, advocate, advisor, and officer of the court.

The bench has spoken. Your job is to translate the ruling into plain English for the record. You are NOT the judge; you do not re-decide anything. You translate faithfully, in full, and you give the principal a clear sense of what this means for their project.

THE RULING ARTEFACT:
${JSON.stringify(ruling, null, 2)}

TRANSLATION RULES:
- No legalese. Write as if explaining to an intelligent non-lawyer who needs to act on this.
- No em dashes or en dashes.
- plain_english_summary: what was filed, whether it had standing, whether it was disposed on precedent or fresh deliberation, and what the court held. 3-6 sentences.
- what_it_means_in_practice: concrete and actionable. What must the party do, or what are they now permitted to do? 1-3 sentences.
- can_it_be_appealed: brief, factual note on the appeal route available (permission to appeal to the Court of Appeal, VPR 3 / s. 10), including the grounds required (arguable point of law or binding-precedent conflict).`,
  { label: 'Lexby - translation', phase: 'Translation', schema: TRANSLATION_SCHEMA }
)

// --------------------------------------------------------------------------
// Phase 5: PDF Render
// Generate a PDF judgment via court/renderer. Requires npm packages installed
// in court/renderer/. Skipped gracefully if not available.
// --------------------------------------------------------------------------
phase('PDF Render')

const citationSlug = (ruling.citation_id || 'lexby-1')
  .replace(/[\[\]\s]+/g, '-')
  .replace(/^-|-$/g, '')
  .toLowerCase()

const pdfPath = await agent(
  `You are generating the PDF judgment for this VJS ruling using the court/renderer engine.

RULING JSON (will be written to a temp file for the renderer):
${JSON.stringify({ tier: ruling.tier, ruling, lexby_translation: translation }, null, 2)}

CITATION SLUG: ${citationSlug}

STEPS:
0. Locate the VJS repo root: the nearest directory containing BOTH court/renderer/index.js AND .justice/ (do not assume the current working directory is the repo). cd into it before the steps below.

1. Check that court/renderer/node_modules exists:
   ls court/renderer/node_modules 2>/dev/null | head -1

2. If it does NOT exist, print "RENDERER-NOT-INSTALLED" and stop.

3. If it does exist:
   a. Create the output directory: mkdir -p .justice/pdfs
   b. Write the ruling JSON to a temp file:
      cat > /tmp/vjs-ruling-${citationSlug}.json << 'ENDJSON'
      [paste the ruling JSON here]
      ENDJSON
   c. Run the renderer:
      node court/renderer/index.js /tmp/vjs-ruling-${citationSlug}.json .justice/pdfs/${citationSlug}.pdf
   d. Check the file was created:
      ls -lh .justice/pdfs/${citationSlug}.pdf
   e. Return ONLY the absolute path to the PDF, e.g.: /home/user/project/.justice/pdfs/${citationSlug}.pdf
      Use pwd to get the current directory and construct the full path.

Return the PDF path on success, or "RENDERER-NOT-INSTALLED" if step 2 applies.`,
  { label: 'PDF Render', phase: 'PDF Render', agentType: 'claude' }
)

if (pdfPath && !pdfPath.includes('NOT-INSTALLED')) {
  log(`You can read the judgment here: ${pdfPath.trim()}`)
}

// --------------------------------------------------------------------------
// Return
// --------------------------------------------------------------------------
return {
  intake: intakeNote,
  screen,
  ruling,
  lexby_translation: translation,
  judgment_pdf: pdfPath && !pdfPath.includes('NOT-INSTALLED') ? pdfPath.trim() : null,
}
