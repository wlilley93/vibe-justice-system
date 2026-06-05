export const meta = {
  name: "court-of-appeal",
  description: "Vibe Justice System - Court of Appeal (3-judge panel). Reviews First Instance rulings on an arguable point of law. General-purpose; operates on any project repo.",
  version: "1.0.0",
  phases: [
    { name: 'Law Load', detail: 'Read SPEC-LAW.md and .justice/INDEX.md from the repo - the court is always bound to the current law, never a stale copy' },
    { name: 'Standing Gate', detail: 'Assess whether grounds of appeal disclose an arguable point of law or binding-precedent conflict' },
    { name: 'Appeal - Three Independent Opinions', detail: 'Three-judge panel delivers independent opinions (Blackmere J, Goffe J, Elden J)' },
    { name: 'Ruling - Presiding Judge Synthesis', detail: 'Elden J (presiding, one of the three) synthesises the panel into the Court of Appeal ruling artefact (s. 18: author is a counted member, not a fourth seat)' },
    { name: 'Lexby Translates', detail: 'Lexby translates the judgment into plain language for the principal' },
    { name: 'PDF Render', detail: 'Render the judgment as a PDF using the court/renderer engine' },
  ],
};

// ---------------------------------------------------------------------------
// STANDING GATE
// ---------------------------------------------------------------------------
// The Court of Appeal does not convene for mere disagreement with the outcome.
// Grounds must disclose an arguable point of law or a binding-precedent conflict
// (SPEC-LAW-11(a), VPR 3). If they do not, the court disposes without sitting.
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// BENCH POOL
// Draws three from the nine permanent bench members per SPEC-LAW-10 / VPR 5.
// For this Court of Appeal the three posture-seats map to distinct temperaments:
//   - strict-construction seat  -> Blackmere (textualist, holds hard to the literal words)
//   - pragmatist seat           -> Goffe (real-world workability)
//   - precedent-hawk seat       -> Elden (historically-minded, draws on precedent)
// The judgment of the Court is authored by ONE of the three (the presiding member,
// Elden J), synthesising the panel from within - per SPEC-LAW s. 18 ([2026] LEXBY-SC 3):
// every bench is odd, the size is the TOTAL deciding membership, and no synthesiser may
// be added on top of the sized panel. The bench is THREE; there is no fourth seat.
// ---------------------------------------------------------------------------

export default async function courtOfAppeal({ agent, parallel, phase, log }, args) {

  // -------------------------------------------------------------------------
  // LAW LOAD - read SPEC-LAW.md and .justice/INDEX.md from the repo
  // -------------------------------------------------------------------------
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

  const specBlock = liveSpec || args.spec
  const caselawBlock = liveIndex || args.caselaw

  // Clerk: deterministic citation numbering (mirror of cli/lib/citation.js; the Workflow sandbox has no require).
  const VJS_YEAR = (args && args.year) || 2026
  const vjsAssignCitation = (citatorText, code, year) => {
    const re = new RegExp('\\[' + year + '\\]\\s*LEXBY-' + code + '\\s+(\\d+)', 'gi')
    let max = 0, m
    while ((m = re.exec(citatorText || '')) !== null) { const n = parseInt(m[1], 10); if (n > max) max = n }
    return '[' + year + '] LEXBY-' + code + ' ' + (max + 1)
  }
  const assignedCitation = vjsAssignCitation(liveIndex, 'CA', VJS_YEAR)
  log('Clerk assigned citation: ' + assignedCitation)

  // -------------------------------------------------------------------------
  // PHASE 0 - STANDING GATE
  // -------------------------------------------------------------------------
  const standingPhase = await phase("Standing Gate", async () => {
    return await agent("Standing Officer (Sumberly J)", {
      description: "Assess whether the grounds of appeal disclose an arguable point of law or a binding-precedent conflict. Dispose without convening if standing is not made out.",
      prompt: `
You are SUMBERLY J, sitting as Standing Officer for the Court of Appeal of the Vibe Justice System.
Your office is entirely procedural. You assess one question only: whether the stated grounds of appeal
disclose an arguable point of law or a binding-precedent conflict sufficient to grant permission to appeal
under SPEC-LAW-11(a) and VPR 3. You do not form any view on the merits.

STANDING TEST (apply strictly):
- Arguable point of law: a question of legal principle that was arguably decided wrongly below, not merely
  that the applicant disagrees with the outcome or wishes a different answer on the same facts.
- Binding-precedent conflict: the lower ruling is arguably irreconcilable with a cited or uncited
  binding precedent.
- Mere dissatisfaction with the outcome, preference for a different approach, or re-argument of
  facts already found below are NOT grounds. They warrant disposal as "appeal not granted."

INPUTS:
Lower ruling (First Instance):
${JSON.stringify(args.lower_ruling, null, 2)}

Grounds of appeal:
${args.grounds}

SPEC-LAW (binding statute):
${specBlock}

Relevant case law:
${caselawBlock}

TASK:
Assess standing. Output a JSON object with exactly these fields:
{
  "standing": true | false,
  "disposal_reason": "<if standing false: the reason, in formal legalese, one paragraph>",
  "arguable_points": ["<point 1>", ...],
  "standing_note": "<brief formal note on the standing decision>"
}

Speak in formal legalese as becomes your office. Do not form any view on the merits.
`,
    });
  });

  // Parse standing result
  let standingResult;
  try {
    const raw = typeof standingPhase === "string" ? standingPhase : JSON.stringify(standingPhase);
    const match = raw.match(/\{[\s\S]*\}/);
    standingResult = match ? JSON.parse(match[0]) : { standing: false, disposal_reason: "Standing result could not be parsed.", arguable_points: [], standing_note: "" };
  } catch (_) {
    standingResult = { standing: false, disposal_reason: "Standing result could not be parsed.", arguable_points: [], standing_note: "" };
  }

  if (!standingResult.standing) {
    return {
      citation: null,
      disposition: "appeal_not_granted",
      standing_gate: standingResult,
      panel: ["Sumberly J (Standing Officer)"],
      ratio: "Permission to appeal refused. The grounds do not disclose an arguable point of law or a binding-precedent conflict. The lower ruling stands.",
      lower_ruling: args.lower_ruling,
      amendments: null,
      lexby_translation: `The appeal was stopped at the gate. The reason: ${standingResult.disposal_reason} The original ruling stands.`,
    };
  }

  // -------------------------------------------------------------------------
  // PHASE 1 - THREE INDEPENDENT OPINIONS
  // -------------------------------------------------------------------------
  const opinionsPhase = await phase("Appeal - Three Independent Opinions", async () => {
    return await parallel("Three-Judge Panel", [

      // (a) Strict-construction opinion - BLACKMERE J
      agent("Blackmere J (Strict Construction)", {
        description: "Strict-construction opinion: does the ratio hold under the letter of SPEC-LAW?",
        prompt: `
You are BLACKMERE J, sitting in the Court of Appeal of the Vibe Justice System.
Your judicial temperament: textualist. You hold hard to the literal words of SPEC-LAW. You do not
supplement, extend, or gloss the statute; you apply it word by word. Where SPEC-LAW is silent, you
apply the no-statute default at s. 7 precisely. You distrust purposive construction and resist
implication.

Your posture in this appeal: STRICT CONSTRUCTION.
Your sole question is: does the ratio of the lower ruling hold under the letter of SPEC-LAW as
enacted? Apply every relevant article word by word. Note any place where the lower court read in
words that are not there, ignored words that are, or applied an article to facts outside its
literal scope.

INPUTS:
Lower ruling:
${JSON.stringify(args.lower_ruling, null, 2)}

Grounds of appeal (arguable points):
${args.grounds}

SPEC-LAW:
${specBlock}

Case law:
${caselawBlock}

TASK:
Write your opinion in formal legalese. Structure:
1. The articles in play (identify each SPEC-LAW provision engaged, quote the operative words).
2. Whether the lower court applied those words correctly on the facts found.
3. Whether the ratio is sound, unsound, or sound-with-modification under strict construction.
4. Your provisional conclusion: affirm / affirm with modifications / reverse, with reasons.

End with a JSON block:
\`\`\`json
{
  "justice": "Blackmere J",
  "posture": "strict-construction",
  "provisional_disposition": "affirm | affirm_with_modifications | reverse",
  "key_findings": ["<finding 1>", ...],
  "proposed_amendments": ["<amendment if any, else null>"],
  "opinion_summary": "<one paragraph summary of your opinion>"
}
\`\`\`
`,
      }),

      // (b) Pragmatist opinion - GOFFE J
      agent("Goffe J (Pragmatist)", {
        description: "Pragmatist opinion: does the outcome serve the principal?",
        prompt: `
You are GOFFE J, sitting in the Court of Appeal of the Vibe Justice System.
Your judicial temperament: pragmatist. You test every ruling for real-world workability. A ruling
that is technically correct but produces an absurd, unworkable, or harmful outcome for the principal
should be re-examined. You respect the text of SPEC-LAW but you read it purposively, asking what
mischief it was designed to remedy and whether this ruling remedies or creates that mischief.

Your posture in this appeal: PRAGMATIST.
Your question is: does the outcome of the lower ruling serve the principal as Sovereign and PM?
Does it produce a workable, proportionate result? Does the remedy (remediation + restitution, s. 6)
actually make good the harm, or does it over- or under-shoot? Would a competent practitioner
recognise this ruling as sensible?

INPUTS:
Lower ruling:
${JSON.stringify(args.lower_ruling, null, 2)}

Grounds of appeal (arguable points):
${args.grounds}

SPEC-LAW:
${specBlock}

Case law:
${caselawBlock}

TASK:
Write your opinion in formal legalese. Structure:
1. The practical question raised by the lower ruling.
2. Whether the ruling produces a workable, proportionate outcome for the principal.
3. Whether the remedy is calibrated correctly under s. 6.
4. Your provisional conclusion: affirm / affirm with modifications / reverse, with reasons.

End with a JSON block:
\`\`\`json
{
  "justice": "Goffe J",
  "posture": "pragmatist",
  "provisional_disposition": "affirm | affirm_with_modifications | reverse",
  "key_findings": ["<finding 1>", ...],
  "proposed_amendments": ["<amendment if any, else null>"],
  "opinion_summary": "<one paragraph summary of your opinion>"
}
\`\`\`
`,
      }),

      // (c) Precedent-hawk opinion - ELDEN J
      agent("Elden J (Precedent Hawk)", {
        description: "Precedent-hawk opinion: is the ruling consistent with all cited and uncited precedents?",
        prompt: `
You are ELDEN J, sitting in the Court of Appeal of the Vibe Justice System.
Your judicial temperament: historically-minded. You draw on precedent and tradition. Consistency of
the common law is itself a value; a ruling that departs from established precedent without
distinguishing it or overruling it is a jurisprudential error, even if the outcome feels just on the
day. You examine not only the precedents the lower court cited but the ones it should have cited.

Your posture in this appeal: PRECEDENT HAWK.
Your question is: is the lower ruling consistent with all binding and persuasive precedents, cited
and uncited? Did the lower court distinguish, follow, or overrule precedents correctly? Is any
departure adequately reasoned? Does the lower ratio create a tension with the existing body of
case law that will produce inconsistency downstream?

INPUTS:
Lower ruling:
${JSON.stringify(args.lower_ruling, null, 2)}

Grounds of appeal (arguable points):
${args.grounds}

SPEC-LAW:
${specBlock}

Case law (supplied; but also consider what is not cited that should be):
${caselawBlock}

TASK:
Write your opinion in formal legalese. Structure:
1. The precedents engaged (both cited by the lower court and any you identify as omitted).
2. Whether the lower court correctly applied, distinguished, or departed from each.
3. Whether any departure is adequately reasoned or amounts to per incuriam (s. 11(e)).
4. Your provisional conclusion: affirm / affirm with modifications / reverse, with reasons.

End with a JSON block:
\`\`\`json
{
  "justice": "Elden J",
  "posture": "precedent-hawk",
  "provisional_disposition": "affirm | affirm_with_modifications | reverse",
  "key_findings": ["<finding 1>", ...],
  "proposed_amendments": ["<amendment if any, else null>"],
  "opinion_summary": "<one paragraph summary of your opinion>"
}
\`\`\`
`,
      }),

    ]);
  });

  // -------------------------------------------------------------------------
  // PHASE 2 - RULING BY THE PRESIDING JUDGE (ELDEN J, ONE OF THE THREE)
  // Per SPEC-LAW s. 18 ([2026] LEXBY-SC 3): the judgment is authored by a counted
  // member of the sized panel, never a synthesiser added on top. The bench is THREE.
  // -------------------------------------------------------------------------
  const rulingPhase = await phase("Ruling - Presiding Judge Synthesis", async () => {
    return await agent("Elden J (Presiding - Synthesis)", {
      description: "The presiding member of the three (Elden J) synthesises the panel into the Court of Appeal ruling artefact, writing from within the sized bench.",
      prompt: `
You are ELDEN J, the presiding member of this three-judge Court of Appeal of the Vibe Justice System,
and one of the three who sat and opined. You have read all three opinions on this panel (including your
own). You now deliver the judgment of the Court from WITHIN the sized bench of three: you hold no vote or
authority beyond your single seat, and you record the majority of the three as the ratio (SPEC-LAW s. 18).
Your judicial temperament: historically-minded; here writing for the Court in a balanced, synthetic voice.

THE THREE OPINIONS FROM THE PANEL:
${JSON.stringify(opinionsPhase, null, 2)}

ORIGINAL INPUTS:
Lower ruling:
${JSON.stringify(args.lower_ruling, null, 2)}

Grounds of appeal:
${args.grounds}

SPEC-LAW:
${specBlock}

Case law:
${caselawBlock}

YOUR TASK:
Deliver the judgment of the Court of Appeal. Synthesise the three opinions. Where the panel
converges, record that as the Court's ratio. Where the panel diverges, give reasons for the
majority position and record the dissent as obiter. Apply the standard of: can the lower ruling
stand, must it be modified, or must it be reversed?

Your judgment must:
1. State the disposition: affirm, affirm_with_modifications, or reverse.
2. Where affirm_with_modifications or reverse: state precisely what is changed (the amendments).
3. Identify the ratio of this Court (the binding holding, stated as a rule of law).
4. Record any obiter dicta (persuasive but not binding observations).
5. Record any dissenting position from the panel opinions, attributed.
6. Produce the full ruling artefact as JSON at the end.

RULING ARTEFACT SCHEMA (produce this exactly at the end, in a JSON block):
\`\`\`json
{
  "citation": "${assignedCitation}",
  "tier": "court_of_appeal",
  "panel": ["Blackmere J", "Goffe J", "Elden J (presiding)"],
  "disposition": "affirm | affirm_with_modifications | reverse",
  "lower_ruling": <the lower ruling object verbatim>,
  "ratio": "<the binding ratio of this Court, stated as a rule of law>",
  "obiter": ["<obiter dictum 1>", ...],
  "dissent": "<dissenting position if any, or null>",
  "amendments": [
    {
      "field": "<field in lower ruling being changed>",
      "from": "<original value>",
      "to": "<new value>",
      "reason": "<brief reason>"
    }
  ],
  "panel_opinions": {
    "blackmere": "<one-sentence summary of Blackmere J's position>",
    "goffe": "<one-sentence summary of Goffe J's position>",
    "elden": "<one-sentence summary of Elden J's position>"
  },
  "standing_grounds_accepted": true,
  "judgment_text": "<full judgment text in formal legalese, multi-paragraph>"
}
\`\`\`

Write the full judgment before the JSON block. Speak in formal legalese as becomes the presiding
judge of the Court of Appeal.
`,
    });
  });

  // -------------------------------------------------------------------------
  // PHASE 3 - LEXBY TRANSLATES
  // -------------------------------------------------------------------------
  const lexbyPhase = await phase("Lexby Translates", async () => {
    return await agent("Lexby (Translation)", {
      description: "Lexby translates the Court of Appeal judgment into plain language for the principal.",
      prompt: `
You are LEXBY - the principal's counsel, officer of the court, and translator (per SPEC-LAW s. 3).
The Court of Appeal has delivered its judgment. Your job is to translate it for the principal
in plain, clear language. No jargon. No em dashes. No en dashes. Short sentences.

THE COURT OF APPEAL JUDGMENT:
${JSON.stringify(rulingPhase, null, 2)}

THE LOWER RULING IT REVIEWED:
${JSON.stringify(args.lower_ruling, null, 2)}

TASK:
Write a plain-language translation. Cover:
1. What the appeal was about (one sentence).
2. Whether the appeal was allowed or dismissed, and the bottom line.
3. What changes (if any) were made to the original ruling.
4. What it means practically for the principal and the project.
5. What happens next (e.g. if reversed, what must be remediated; if affirmed, the lower ruling stands).

Keep it short: 150-250 words. Plain English. No legalese. No em or en dashes.

End with a JSON block:
\`\`\`json
{
  "translation": "<the full plain-language translation as a single string>",
  "bottom_line": "<one sentence: what happened and what it means>",
  "next_steps": ["<step 1>", ...]
}
\`\`\`
`,
    });
  });

  // -------------------------------------------------------------------------
  // ASSEMBLE FINAL OUTPUT
  // -------------------------------------------------------------------------

  // Extract the ruling artefact JSON from Aldermere's opinion
  let rulingArtefact = null;
  try {
    const raw = typeof rulingPhase === "string" ? rulingPhase : JSON.stringify(rulingPhase);
    const match = raw.match(/```json\s*(\{[\s\S]*?\})\s*```/);
    if (match) {
      rulingArtefact = JSON.parse(match[1]);
    }
  } catch (_) {
    rulingArtefact = { raw_judgment: rulingPhase };
  }

  // Extract Lexby's translation JSON
  let lexbyResult = null;
  try {
    const raw = typeof lexbyPhase === "string" ? lexbyPhase : JSON.stringify(lexbyPhase);
    const match = raw.match(/```json\s*(\{[\s\S]*?\})\s*```/);
    if (match) {
      lexbyResult = JSON.parse(match[1]);
    }
  } catch (_) {
    lexbyResult = { translation: String(lexbyPhase), bottom_line: "", next_steps: [] };
  }

  // -------------------------------------------------------------------------
  // PDF RENDER
  // -------------------------------------------------------------------------
  phase('PDF Render')

  // Clerk binds the deterministic citation onto the artefact (schema emits "citation"; downstream reads "citation_id").
  if (rulingArtefact) { rulingArtefact.citation_id = assignedCitation; rulingArtefact.citation = assignedCitation }

  const citSlug = ((rulingArtefact && rulingArtefact.citation_id) || assignedCitation)
    .replace(/[\[\]\s]+/g, '-').replace(/^-|-$/g, '').toLowerCase()

  const pdfPath = await agent(
    `Generate the PDF judgment for this Court of Appeal ruling.

RULING JSON:
${JSON.stringify({ tier: 'court-of-appeal', ruling: rulingArtefact, lexby: lexbyResult }, null, 2)}

CITATION SLUG: ${citSlug}

STEPS:
1. Check court/renderer/node_modules exists (ls court/renderer/node_modules 2>/dev/null | head -1). If not, return "RENDERER-NOT-INSTALLED".
2. mkdir -p .justice/pdfs
3. Write the ruling JSON to /tmp/vjs-ruling-${citSlug}.json
4. node court/renderer/index.js /tmp/vjs-ruling-${citSlug}.json .justice/pdfs/${citSlug}.pdf
5. Return the absolute PDF path (use pwd to construct it).`,
    { label: 'PDF Render', phase: 'PDF Render', agentType: 'claude' }
  )

  if (pdfPath && !pdfPath.includes('NOT-INSTALLED')) {
    log(`You can read the judgment here: ${pdfPath.trim()}`)
  }

  return {
    standing: standingResult,
    ruling: rulingArtefact,
    lexby: lexbyResult,
    judgment_pdf: pdfPath && !pdfPath.includes('NOT-INSTALLED') ? pdfPath.trim() : null,
    raw: {
      standing_phase: standingPhase,
      opinions_phase: opinionsPhase,
      ruling_phase: rulingPhase,
      lexby_phase: lexbyPhase,
    },
  };
}
