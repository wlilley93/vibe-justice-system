export const meta = {
  name: 'vjs-court-of-appeal',
  description: 'Vibe Justice System - Court of Appeal (3-judge panel). Reviews a First Instance ruling on an arguable point of law or binding-precedent conflict. Permission to appeal is decided by an independent leave-judge who did not sit below (s. 19(3)); the matter then reaches the panel on a symmetric researched record (s. 19(1)). The judgment is authored by one of the three (s. 18). General-purpose; runs on any repo.',
  phases: [
    { title: 'Law Load', detail: 'Read SPEC-LAW.md and .justice/INDEX.md from the repo - always bound to the current law' },
    { title: 'Permission to appeal', detail: 'An independent leave-judge (randomised, never the trial judge or a panel member) decides leave on a Sonnet-class model (s. 19(3))' },
    { title: 'Hard research - both sides', detail: 'Researched intake (s. 19(1)): appellant and respondent each file a brief so the panel sits on a symmetric record' },
    { title: 'Appeal - Three Independent Opinions', detail: 'The three-judge panel (Blackmere J strict-construction, Goffe J pragmatist, Elden J precedent-hawk) delivers independent opinions' },
    { title: 'Ruling - authored from within the panel', detail: 'Elden J, one of the three, authors the judgment of the Court (s. 18: no fourth synthesising seat)' },
    { title: 'Lexby Translation', detail: 'Lexby translates the judgment into plain English' },
    { title: 'PDF Render', detail: 'Deterministically render the judgment as a PDF into .justice/pdfs/ (cwd-independent)' },
  ],
}

// args may arrive as a JSON-encoded string depending on the host runtime; coerce first.
if (typeof args === 'string') { try { args = JSON.parse(args) } catch (_) {} }
if (!args || typeof args !== 'object') args = {}

const YEAR = args.year || 2026

// The permanent bench. The leave-judge is drawn from here (excluding anyone who sat below
// or sits on this panel). The panel itself is the fixed three posture-seats.
const BENCH = [
  { name: 'Hallam CJ', brief: 'Principled and precise; anchors to statute first.' },
  { name: 'Goffe J', brief: 'Pragmatist; tests every ruling for real-world workability.' },
  { name: 'Blackmere J', brief: 'Textualist; holds hard to the literal words of SPEC-LAW.' },
  { name: 'Sumberly J', brief: 'Formalist; procedural correctness as a substantive guarantee.' },
  { name: 'Elden J', brief: 'Historically minded; draws on precedent and tradition.' },
  { name: 'Coade J', brief: 'Restrained; prefers the narrowest ruling that resolves the case.' },
  { name: 'Steyne J', brief: 'Bold; willing to extend the law where the purpose requires.' },
  { name: 'Bowan J', brief: 'The dissenter; tests every proposition to destruction.' },
  { name: 'Aldermere J', brief: 'Balanced and synthetic; honours every legitimate competing concern.' },
]

// The fixed three-judge panel (s. 18: the bench is THREE; Elden J authors from within).
const PANEL = [
  { name: 'Blackmere J', posture: 'strict-construction', brief: 'You are Blackmere J, the textualist of the panel. You hold hard to the literal words of SPEC-LAW; where the text is plain no purposive reasoning overrides it; where SPEC-LAW is silent you apply the s. 7 default precisely. Your question: does the ratio below hold under the letter of the statute?' },
  { name: 'Goffe J', posture: 'pragmatist', brief: 'You are Goffe J, the pragmatist of the panel. You read SPEC-LAW purposively and test the outcome for real-world workability and proportionate remedy (s. 6). Your question: does the ruling below serve the principal and produce a workable result?' },
  { name: 'Elden J', posture: 'precedent-hawk (presiding, authoring)', brief: 'You are Elden J, the precedent-hawk and presiding member. Consistency of the case law is itself a value; a ruling that departs from precedent without distinguishing or overruling it is an error. Your question: is the ruling below consistent with all binding and persuasive precedent, cited and uncited? As presiding member you will also author the judgment of the Court from within the panel (s. 18).' },
]

// --------------------------------------------------------------------------
// Law Load
// --------------------------------------------------------------------------
phase('Law Load')
const lawLoad = await parallel([
  () => agent('Read the file .justice/SPEC-LAW.md in the current working directory. If it does not exist, read SPEC-LAW.md instead. Return the complete text verbatim with no commentary.', { label: 'load SPEC-LAW', phase: 'Law Load', agentType: 'Explore' }),
  () => agent('Read the file .justice/INDEX.md in the current working directory. If it does not exist, try caselaw/INDEX.md (legacy). Return the complete text verbatim with no commentary.', { label: 'load .justice/INDEX.md', phase: 'Law Load', agentType: 'Explore' }),
])
const liveSpec = (lawLoad[0] && lawLoad[0].trim()) || null
const liveIndex = (lawLoad[1] && lawLoad[1].trim()) || null
const specBlock = liveSpec || args.spec || '(SPEC-LAW not available)'
const caselawBlock = liveIndex || args.caselaw || '(no caselaw available)'
if (liveSpec) log('SPEC-LAW loaded from repo.')
if (liveIndex) log('.justice/INDEX.md loaded from repo.')

function nextCitation(citatorText, code, year) {
  const re = new RegExp('\\[' + year + '\\]\\s*LEXBY-' + code + '\\s+(\\d+)', 'gi')
  let max = 0, m
  while ((m = re.exec(citatorText || '')) !== null) { const n = parseInt(m[1], 10); if (n > max) max = n }
  return '[' + year + '] LEXBY-' + code + ' ' + (max + 1)
}
const assignedCitation = nextCitation(caselawBlock, 'CA', YEAR)
log('Clerk assigned citation: ' + assignedCitation)

const lowerRuling = args.lower_ruling || args.lower_rulings || null
const grounds = args.grounds || args.question || '(no grounds supplied)'
const trialJudge = (lowerRuling && lowerRuling.judge) || null

// --------------------------------------------------------------------------
// Permission to appeal (s. 19(3)): an independent leave-judge, randomised, never the
// trial judge and never a member of this panel. Sonnet-class model.
// --------------------------------------------------------------------------
phase('Permission to appeal')
const LEAVE_SCHEMA = {
  type: 'object', additionalProperties: false,
  required: ['granted', 'basis', 'reason'],
  properties: {
    granted: { type: 'boolean', description: 'True only if the grounds disclose an arguable point of law or a binding-precedent conflict. Mere dissatisfaction with the outcome, or a wish to re-argue the facts, is not a ground.' },
    basis: { type: 'string', enum: ['arguable_point_of_law', 'binding_precedent_conflict', 'refused'] },
    reason: { type: 'string', description: '1-3 sentences, formal, explaining the leave decision.' },
  },
}
const panelNames = PANEL.map(p => p.name)
const eligible = BENCH.filter(j => j.name !== trialJudge && !panelNames.includes(j.name))
const leavePool = eligible.length ? eligible : BENCH.filter(j => j.name !== trialJudge)
let lh = 0
const leaveSeed = String(grounds) + '|ca-leave'
for (let i = 0; i < leaveSeed.length; i++) lh = ((lh << 5) - lh + leaveSeed.charCodeAt(i)) >>> 0
const leaveJudge = leavePool[lh % leavePool.length]
log('Leave-judge (independent, did not sit below): ' + leaveJudge.name + ' [Sonnet]')

const leave = await agent(
  `${leaveJudge.brief}

You are ${leaveJudge.name}, sitting ALONE as the independent leave-judge of the Court of Appeal in the Vibe Justice System. You did NOT sit at First Instance on this matter and you are not on the appeal panel. Your office is to decide PERMISSION TO APPEAL only; you do not decide the merits.

THE TEST (s. 11(a), s. 19(3), VPR 3) - grant leave only if the grounds disclose at least one of: an arguable point of law (a question of principle arguably decided wrongly below, not mere disagreement), or a binding-precedent conflict. Mere dissatisfaction with the result, or a wish to re-argue the facts, is refused.

THE FIRST INSTANCE RULING UNDER CHALLENGE:
${JSON.stringify(lowerRuling, null, 2)}

THE GROUNDS OF APPEAL:
${grounds}

SPEC-LAW:
${specBlock}

CASELAW:
${caselawBlock}

Decide permission, on the papers. Be strict.`,
  { label: `${leaveJudge.name} - permission to appeal`, phase: 'Permission to appeal', model: 'sonnet', schema: LEAVE_SCHEMA }
)
log('Permission to appeal: ' + (leave.granted ? 'GRANTED (' + leave.basis + ')' : 'REFUSED'))

if (!leave.granted) {
  return {
    citation: null,
    disposition: 'permission_refused',
    leave,
    panel: [leaveJudge.name + ' (leave-judge)'],
    ratio: 'Permission to appeal refused: the grounds disclose no arguable point of law or binding-precedent conflict. The First Instance ruling stands.',
    lower_ruling: lowerRuling,
    lexby_translation: `An independent judge who did not sit on your original case refused permission to appeal. ${leave.reason} The First Instance ruling stands. You could only take it further by framing a genuine point of law, not a disagreement with the result.`,
  }
}

const caseFileBase = `
IN THE COURT OF APPEAL OF THE VIBE JUSTICE SYSTEM (VJS)
PANEL OF THREE: ${panelNames.join(', ')}
Permission to appeal granted by ${leaveJudge.name} (independent leave-judge): ${leave.basis}. ${leave.reason}

GROUNDS OF APPEAL:
${grounds}

FIRST INSTANCE RULING UNDER REVIEW:
${JSON.stringify(lowerRuling, null, 2)}

PROPOSED CITATION (clerk, deterministic): ${assignedCitation}

SPEC-LAW:
${specBlock}

CASELAW:
${caselawBlock}
`.trim()

// --------------------------------------------------------------------------
// Researched intake (s. 19(1)): appellant + respondent briefs.
// --------------------------------------------------------------------------
phase('Hard research - both sides')
const BRIEF_SCHEMA = {
  type: 'object', additionalProperties: false,
  required: ['role', 'thesis', 'best_arguments', 'statutes_relied', 'precedents_relied', 'procedural_motions', 'strongest_opposing_point'],
  properties: {
    role: { type: 'string' },
    thesis: { type: 'string' },
    best_arguments: { type: 'array', items: { type: 'string' } },
    statutes_relied: { type: 'array', items: { type: 'string' } },
    precedents_relied: { type: 'array', items: { type: 'string' } },
    procedural_motions: { type: 'array', items: { type: 'string' } },
    strongest_opposing_point: { type: 'string' },
  },
}
const arsenal = 'Deploy whatever genuinely helps (cite the article): per incuriam (s. 11(e)); distinguishing; binding-precedent conflict; the s. 11(c) fast-path; declaration of incompatibility (s. 11(f)); the Bolam responsible-body defence (s. 5); the s. 15 threshold; the s. 16 candour scope.'
const briefs = await parallel([
  { role: 'appellant', stance: 'Argue FOR the appeal: the First Instance ruling should be reversed or varied. Identify each error of law in the ruling below.' },
  { role: 'respondent', stance: 'Argue AGAINST the appeal: the First Instance ruling should be affirmed. Defend its reasoning and answer each ground.' },
].map(({ role, stance }) => () => agent(
  `You are COUNSEL FOR THE ${role.toUpperCase()} in the Court of Appeal of the Vibe Justice System. ${stance}

This is the mandatory hard-research first leg (s. 19(1)). Research the law HARD: you may READ the full text of any ruling under .justice/judgments/ and re-read SPEC-LAW.md. Ground every argument in a cited article (s. n) or neutral citation. No em dashes or en dashes.

${arsenal}

${caseFileBase}

Produce your brief: adversarial and thorough for your side, never misstating the law.`,
  { label: `${role} brief`, phase: 'Hard research - both sides', agentType: 'Explore', schema: BRIEF_SCHEMA }
)))
const briefsSection = ['appellant', 'respondent'].map((role, i) => {
  const b = briefs[i]
  if (!b) return `=== ${role.toUpperCase()} BRIEF ===\n(no brief returned)`
  return `=== ${role.toUpperCase()} BRIEF ===
Thesis: ${b.thesis}
Best arguments:
${(b.best_arguments || []).map(a => '  - ' + a).join('\n')}
Statutes relied: ${(b.statutes_relied || []).join('; ')}
Precedents relied: ${(b.precedents_relied || []).join('; ')}
Procedural motions: ${(b.procedural_motions || []).length ? b.procedural_motions.join('; ') : 'none'}
Strongest point against this side (conceded): ${b.strongest_opposing_point || ''}`
}).join('\n\n')

const caseFile = `${caseFileBase}

================================================================
ADVERSARIAL BRIEFS (researched intake, s. 19(1))
================================================================
${briefsSection}`

// --------------------------------------------------------------------------
// Three independent opinions.
// --------------------------------------------------------------------------
phase('Appeal - Three Independent Opinions')
const opinions = await parallel(PANEL.map(j => () => agent(
  `${j.brief}

You are sitting in the Court of Appeal of the Vibe Justice System on a three-judge panel. Write your independent opinion in formal legalese. Do NOT use em dashes or en dashes. You have a symmetric, two-sided researched record; weigh both briefs but rule on the law.

${caseFile}

Structure your opinion: (1) the provisions/precedents in play; (2) whether the ruling below is sound under your posture; (3) your provisional disposition - AFFIRM, AFFIRM WITH MODIFICATIONS, or REVERSE - with reasons. Sign: --- ${j.name}`,
  { label: `Opinion of ${j.name}`, phase: 'Appeal - Three Independent Opinions' }
)))
const opinionRecord = PANEL.map((j, i) => `=== ${j.name.toUpperCase()} (${j.posture}) ===\n\n${opinions[i]}`).join('\n\n---\n\n')

// --------------------------------------------------------------------------
// Judgment of the Court - authored by Elden J from WITHIN the three (s. 18).
// --------------------------------------------------------------------------
phase('Ruling - authored from within the panel')
const RULING_SCHEMA = {
  type: 'object', additionalProperties: false,
  required: ['citation_id', 'tier', 'panel', 'disposition', 'ratio', 'obiter', 'dissent', 'amendments', 'status', 'full_judgment_text'],
  properties: {
    citation_id: { type: 'string', description: 'Exactly: ' + assignedCitation },
    tier: { type: 'string', enum: ['court-of-appeal'] },
    panel: { type: 'array', items: { type: 'string' } },
    disposition: { type: 'string', enum: ['affirm', 'affirm_with_modifications', 'reverse'] },
    ratio: { type: 'string', description: 'The binding holding of the Court, stated as a rule of law.' },
    obiter: { type: ['string', 'null'] },
    dissent: { type: ['string', 'null'], description: 'Any dissenting position from the panel, attributed; null if unanimous.' },
    amendments: { type: 'array', items: { type: 'string' }, description: 'For affirm_with_modifications/reverse: precisely what changes in the ruling below. Empty if affirmed.' },
    status: { type: 'string', enum: ['good-law'] },
    full_judgment_text: { type: 'string', description: 'The full judgment of the Court in formal legalese, multi-paragraph.' },
  },
}
const ruling = await agent(
  `${PANEL[2].brief}

You are Elden J, the presiding member of the three. Author the JUDGMENT OF THE COURT of Appeal from within the panel: you are one of the three counted members, not a fourth seat, and you may record as the ratio only a position the majority of the three in fact commands (s. 18). Do NOT use em dashes or en dashes.

THE THREE OPINIONS:
${opinionRecord}

${caseFile}

Deliver the judgment. Where the panel converges, that is the Court's ratio; where it diverges, give the majority's reasons and record the dissent. State the disposition and, if affirm-with-modifications or reverse, exactly what changes. Use the citation ${assignedCitation}; tier court-of-appeal; panel ${JSON.stringify(panelNames)}; status good-law.`,
  { label: 'Elden J - judgment of the Court', phase: 'Ruling - authored from within the panel', schema: RULING_SCHEMA }
)
ruling.citation_id = assignedCitation
ruling.panel = panelNames

// --------------------------------------------------------------------------
// Lexby translation.
// --------------------------------------------------------------------------
phase('Lexby Translation')
const lexbyTranslation = await agent(
  `You are Lexby, the principal's counsel (s. 3). The Court of Appeal has ruled. Translate it into plain English: what the appeal was about, whether it was allowed/dismissed, what changed, what it means in practice, and what happens next. 150-250 words, first person, no jargon, no em dashes or en dashes.

JUDGMENT:
${ruling.full_judgment_text}

DISPOSITION: ${ruling.disposition}. RATIO: ${ruling.ratio}`,
  { label: 'Lexby - translation', phase: 'Lexby Translation' }
)

// --------------------------------------------------------------------------
// PDF render (deterministic, cwd-independent).
// --------------------------------------------------------------------------
phase('PDF Render')
const caSlug = assignedCitation.replace(/[\[\]\s]+/g, '-').replace(/^-|-$/g, '').toLowerCase()
const rulingForPdf = {
  tier: 'court-of-appeal',
  ruling: { citation_id: assignedCitation, tier: 'court-of-appeal', panel: panelNames, kind: 'request_for_ruling', question_or_charge: String(grounds).slice(0, 400), full_judgment_text: ruling.full_judgment_text, ratio: ruling.ratio, status: 'good-law' },
  lexby_translation: { plain_english_summary: String(lexbyTranslation || '').slice(0, 4000) },
}
const pdfPath = await agent(
  `You are rendering the Court of Appeal judgment as a PDF. Be deterministic.
1. Locate the VJS repo root: the nearest directory containing BOTH court/renderer/index.js AND .justice/ (do not assume the working directory is the repo).
2. If <root>/court/renderer/node_modules does not exist, print exactly RENDERER-NOT-INSTALLED and stop.
3. Write this JSON verbatim to /tmp/vjs-ruling-${caSlug}.json:
${JSON.stringify(rulingForPdf)}
4. Run: cd <root> && mkdir -p .justice/pdfs && node court/renderer/index.js /tmp/vjs-ruling-${caSlug}.json .justice/pdfs/${caSlug}.pdf
5. Return the absolute path to the PDF, or RENDERER-NOT-INSTALLED.`,
  { label: 'PDF Render', phase: 'PDF Render', agentType: 'claude' }
)
const judgmentPdf = pdfPath && !pdfPath.includes('NOT-INSTALLED') ? pdfPath.trim() : null
if (judgmentPdf) log('Judgment PDF: ' + judgmentPdf)

return {
  citation: assignedCitation,
  leave,
  panel: panelNames,
  opinions: PANEL.map((j, i) => ({ justice: j.name, opinion: opinions[i] })),
  ruling,
  lexby_translation: lexbyTranslation,
  judgment_pdf: judgmentPdf,
}
