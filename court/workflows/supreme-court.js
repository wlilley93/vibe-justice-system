export const meta = {
  name: 'vjs-supreme-court',
  description: 'Vibe Justice System - Supreme Court. The apex court. Reaches a constitutional/foundational question by progression (with permission to appeal granted by an independent leave-judge) or by the Principal\'s express leapfrog certificate. Seats 9 justices for constitutional questions, 5 for ordinary appeals. Each justice writes an independent opinion; Hallam CJ delivers the leading judgment; Lexby translates. The only court that can enact SPEC-LAW.',
  phases: [
    { title: 'Law Load', detail: 'Read SPEC-LAW.md and .justice/INDEX.md from the repo - always bound to the current law, never a stale copy' },
    { title: 'Permission to appeal', detail: 'An independent leave-judge (randomised, never one who sat below) decides permission on a Sonnet-class model; bypassed only by the Principal\'s leapfrog certificate (VPR 4)' },
    { title: 'Hard research - both sides', detail: 'The mandatory first leg of the trial: claimant and defendant each research the law hard and build their strongest case (with the full procedural arsenal); at the Supreme Court an independent observer (amicus) also joins' },
    { title: 'Justices - Independent Opinions', detail: 'Each justice on the panel writes an independent opinion in parallel, on a fully-argued symmetric record' },
    { title: 'Leading Judgment', detail: 'Hallam CJ synthesises the majority into the binding judgment of the Court' },
    { title: 'Lexby Translation', detail: 'Lexby translates the judgment into plain English for the principal' },
    { title: 'Structured Extract', detail: 'A clerk agent extracts the citation, ratio, disposition, enactments and overrulings as structured fields for filing' },
    { title: 'PDF Render', detail: 'Deterministically render the judgment as a PDF into .justice/pdfs/ via the court/renderer engine (cwd-independent: locates the repo root)' },
  ],
}

// --------------------------------------------------------------------------
// args may arrive as a JSON-encoded string depending on the host runtime; coerce
// to an object before any field access.
// --------------------------------------------------------------------------
if (typeof args === 'string') { try { args = JSON.parse(args) } catch (_) {} }
if (!args || typeof args !== 'object') args = {}

const YEAR = args.year || 2026
// Constitutional / foundational questions seat the full court of 9. Default true:
// a matter that reaches the Supreme Court is presumptively foundational unless told otherwise.
const isConstitutional = args.is_constitutional !== false

const BENCH = [
  { name: 'Hallam CJ', brief: 'You are Hallam CJ: principled and precise. You anchor every analysis to statute first, work outward to case law, and articulate your ratio with surgical economy. You are the senior voice of the Court and your leading judgment must be authoritative and complete.' },
  { name: 'Goffe J', brief: 'You are Goffe J: the pragmatist. You test every proposed ruling for real-world workability. Does it actually function in the day-to-day life of the principal? You call out any ruling that sounds elegant in legalese but fails on delivery.' },
  { name: 'Blackmere J', brief: 'You are Blackmere J: the textualist. You hold hard to the literal words of SPEC-LAW. Where the text is plain, no purposive reasoning overrides it. Where it is ambiguous, you consult the enacted history before any extrinsic aid.' },
  { name: 'Sumberly J', brief: 'You are Sumberly J: the formalist. Procedural correctness is a substantive guarantee, not a technicality. Standing, permission, and the gate requirements matter as much as the merits. You note any procedural irregularity in the record.' },
  { name: 'Elden J', brief: 'You are Elden J: historically minded. You draw on precedent and tradition to test whether a proposed rule is consistent with the long run of authority. You are suspicious of novel departures unless the existing law genuinely cannot accommodate the case.' },
  { name: 'Coade J', brief: 'You are Coade J: restrained and cautious. You prefer the narrowest ruling that resolves the case. You resist dicta that reach beyond what is strictly necessary, and write separately to trim overreach.' },
  { name: 'Steyne J', brief: 'You are Steyne J: bold and expansive. Where the law is unclear and the principal\'s position is sound, you are willing to extend the law to serve it, provided the reasoning is honest and the landing is firm.' },
  { name: 'Bowan J', brief: 'You are Bowan J: the dissenter. Your function is to test every proposition to destruction. You probe the majority\'s reasoning for hidden contradictions, unexamined assumptions, and slippery extensions.' },
  { name: 'Aldermere J', brief: 'You are Aldermere J: balanced and synthetic. You hold the tension between competing positions and find the formulation the whole bench can stand behind. You are integrative, not unprincipled.' },
]
const BENCH_5_NAMES = ['Hallam CJ', 'Goffe J', 'Blackmere J', 'Sumberly J', 'Aldermere J']

// --------------------------------------------------------------------------
// Law Load
// --------------------------------------------------------------------------
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
const spec = liveSpec || args.spec || '(SPEC-LAW not available to the court)'
const caselaw = liveIndex || args.caselaw || '(no caselaw available)'
if (liveSpec) log('SPEC-LAW loaded from repo.')
if (liveIndex) log('.justice/INDEX.md loaded from repo.')

// Clerk: deterministic SC citation from the citator (mirror of cli/lib/citation.js).
function nextCitation(citatorText, code, year) {
  const re = new RegExp('\\[' + year + '\\]\\s*LEXBY-' + code + '\\s+(\\d+)', 'gi')
  let max = 0, m
  while ((m = re.exec(citatorText || '')) !== null) { const n = parseInt(m[1], 10); if (n > max) max = n }
  return '[' + year + '] LEXBY-' + code + ' ' + (max + 1)
}
const assignedCitation = nextCitation(caselaw, 'SC', YEAR)
log('Clerk assigned citation: ' + assignedCitation)

// --------------------------------------------------------------------------
// Permission to appeal (the leave gate)
// Permission is NOT vested in the principal at large (that would make the gate a
// formality) and never in a judge who sat on the decision under challenge. It is
// granted, if at all, by an independent leave-judge drawn at random from the bench,
// excluding everyone who sat below, applying the arguable-point-of-law /
// binding-precedent-conflict test on a Sonnet-class model. The sole bypass is the
// Principal's express leapfrog certificate (SPEC-LAW s.13, VPR 4).
// --------------------------------------------------------------------------
phase('Permission to appeal')

const LEAVE_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  required: ['granted', 'basis', 'reason', 'admit_observer', 'observer_brief'],
  properties: {
    granted: { type: 'boolean', description: 'True only if the grounds disclose an arguable point of law or a binding-precedent conflict. Mere dissatisfaction with the outcome, or a wish to re-argue the facts, is not a ground.' },
    basis: { type: 'string', enum: ['arguable_point_of_law', 'binding_precedent_conflict', 'refused'], description: 'The ground on which leave is granted, or refused.' },
    reason: { type: 'string', description: '1-3 sentences, formal, explaining the leave decision.' },
    admit_observer: { type: 'boolean', description: 'As the judge who reviews and admits this appeal to the Supreme Court, you hold the power to introduce an observer (amicus curiae) to the proceedings. Set true only where an independent perspective would materially assist the Court (e.g. a realm-wide / systemic interest the two parties will not adequately represent). Default false.' },
    observer_brief: { type: ['string', 'null'], description: 'If admit_observer is true, one or two sentences directing what perspective the observer should bring. Null otherwise.' },
  },
}

const belowPanel = Array.isArray(args.appeal_panel) ? args.appeal_panel
  : (args.lower_ruling && args.lower_ruling.judge ? [args.lower_ruling.judge] : [])

let leave
if (args.leapfrog_certificate) {
  // On a leapfrog there is no leave-judge reviewing an appeal, so the power to admit an
  // observer vests in the certificate itself (the Principal stands in the leave role).
  leave = {
    granted: true,
    basis: 'leapfrog_certificate',
    reason: 'The Principal, acting as Sovereign, has issued an express leapfrog certificate (VPR 4). Permission to appeal is not required; the certificate is the authority to sit.',
    admit_observer: !!args.admit_observer,
    observer_brief: args.admit_observer ? (args.observer_brief || null) : null,
  }
  log('Leapfrog certificate present - the leave gate is bypassed (VPR 4).' + (leave.admit_observer ? ' Certificate admits an observer.' : ''))
} else {
  // Independent leave-judge: randomised, never one who sat below. Deterministic draw seeded
  // on the matter text so the same matter draws the same independent judge, but the judge is
  // always disjoint from the bench under challenge.
  const eligible = BENCH.filter(j => !belowPanel.includes(j.name))
  const pool = eligible.length ? eligible : BENCH
  const seedStr = String(args.question || '') + '|leave'
  let h = 0
  for (let i = 0; i < seedStr.length; i++) h = ((h << 5) - h + seedStr.charCodeAt(i)) >>> 0
  const leaveJudge = pool[h % pool.length]
  log('Leave-judge (independent, did not sit below): ' + leaveJudge.name + ' [Sonnet]')

  leave = await agent(
    `${leaveJudge.brief}

You are ${leaveJudge.name}, sitting ALONE as the independent leave-judge of the Vibe Justice System. You did NOT sit on the decision under challenge. Your office is to decide PERMISSION TO APPEAL to the Supreme Court, and nothing else. You do not decide the merits.

THE TEST (SPEC-LAW s.11(a), VPR 3) - grant leave only if the grounds disclose at least one of:
  - an arguable point of law (a question of legal principle arguably decided wrongly below, not mere disagreement with the outcome), or
  - a binding-precedent conflict (the ruling below is arguably irreconcilable with a binding precedent).
Mere dissatisfaction with the result, a preference for a different approach on the same facts, or a wish to re-argue findings of fact are NOT grounds: refuse leave.

THE RULING(S) BELOW:
${JSON.stringify(args.lower_rulings || args.lower_ruling || 'none on the record', null, 2)}

THE GROUNDS ADVANCED:
${args.grounds || args.question || '(no separate grounds supplied; treat the question below as the grounds)'}

THE QUESTION SOUGHT TO BE TAKEN UP:
${args.question || '(none stated)'}

SPEC-LAW:
${spec}

CASELAW:
${caselaw}

As the judge who reviews and admits this appeal, you ALSO hold the power to introduce an observer (amicus curiae) to the Supreme Court proceedings. Exercise it sparingly: admit an observer only where an independent perspective would materially assist the Court on a realm-wide or systemic interest the two parties will not adequately represent. If you admit one, give a one or two sentence brief for what perspective it should bring.

Decide permission and the observer question. Be strict: the gate exists to keep the apex court for genuine points of law.`,
    { label: `${leaveJudge.name} - permission to appeal`, phase: 'Permission to appeal', model: 'sonnet', schema: LEAVE_SCHEMA }
  )
  log('Permission to appeal: ' + (leave.granted ? 'GRANTED (' + leave.basis + ')' : 'REFUSED'))
}

if (!leave.granted) {
  return {
    citation: null,
    disposition: 'permission_refused',
    leave,
    ratio: 'Permission to appeal to the Supreme Court refused: the grounds disclose no arguable point of law or binding-precedent conflict. The ruling below stands.',
    lexby_translation: `Your appeal was stopped at the gate by an independent judge who did not sit on the original decision. ${leave.reason} The ruling below stands. You can still take it forward only if you can frame a genuine point of law, not just a disagreement with the result.`,
  }
}

// --------------------------------------------------------------------------
// The panel
// --------------------------------------------------------------------------
const panel = isConstitutional ? BENCH : BENCH.filter(j => BENCH_5_NAMES.includes(j.name))
const courtType = isConstitutional ? 'FULL COURT OF 9 (constitutional / foundational question)' : 'PANEL OF 5 (ordinary appeal)'

const caseFile = `
IN THE SUPREME COURT OF THE VIBE JUSTICE SYSTEM (VJS)
${courtType}

HOW THE COURT IS SEISED OF THIS MATTER:
${leave.basis === 'leapfrog_certificate' ? 'By the Principal\'s express leapfrog certificate (VPR 4).' : 'By permission to appeal granted by an independent leave-judge (' + leave.basis + '): ' + leave.reason}

THE QUESTION BEFORE THE COURT
-----------------------------
${args.question || '(no question supplied)'}

LOWER RULINGS UNDER REVIEW
--------------------------
${JSON.stringify(args.lower_rulings || args.lower_ruling || 'none (matter reached the apex court directly)', null, 2)}

PROPOSED CITATION (clerk, deterministic): ${assignedCitation}

SPEC-LAW (SOVEREIGN STATUTE)
----------------------------
${spec}

CASELAW (JURISDICTION-LOCAL PRECEDENT)
--------------------------------------
${caselaw}
`.trim()

// --------------------------------------------------------------------------
// Hard research - both sides (the mandatory first leg of the trial)
// Before the bench deliberates, each side researches the law hard and builds its
// strongest case, armed with the full procedural arsenal. The bench then rules on a
// fully-argued, symmetric record (SPEC-LAW s.3). At the Supreme Court an independent
// observer (amicus curiae) also joins, owing no allegiance to either party.
// --------------------------------------------------------------------------
phase('Hard research - both sides')

const ARSENAL = `THE PROCEDURAL ARSENAL (research and deploy whatever genuinely helps your side; cite the article):
  - Standing / strike-out: the matter may be struck out at the threshold for want of standing (s.11(b)) or for falling outside subject-matter jurisdiction (s.14). If your side is the respondent and the claim is bad in limine, MOVE TO STRIKE OUT and state the grounds.
  - Precedent fast-path (s.11(c), VPR 2): if a binding ratio on all fours already governs, the matter is disposed on citation with no sitting. Identify it if it exists.
  - Per incuriam (s.11(e)): a ruling made in ignorance of binding statute or precedent is void. Argue it if a lower or cited ruling missed binding law.
  - Distinguishing: show the facts here are materially different from a precedent that would otherwise bind.
  - Declaration of incompatibility (s.11(f)): if case law is irreconcilable with SPEC-LAW, it is referred up, never used to strike the spec.
  - Bolam responsible-body defence (s.5): conduct a responsible body of competent practice would endorse is not breach.
  - Threshold of duty (s.15): de minimis / disposable / sequenced work may not cross the breach threshold.
  - Candour scope (s.16): the candour duty attaches to representations of delivered scope, not to forward-looking proposals.
  - Anti-bloat (s.12): procedure that adds tokens without decisional or screening value is excluded; use this to attack or defend a proposed procedure.`

const BRIEF_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  required: ['role', 'thesis', 'best_arguments', 'statutes_relied', 'precedents_relied', 'procedural_motions', 'strongest_opposing_point'],
  properties: {
    role: { type: 'string' },
    thesis: { type: 'string', description: 'The one-paragraph position this brief argues for.' },
    best_arguments: { type: 'array', items: { type: 'string' }, description: 'The strongest legal arguments for this side, each grounded in a cited article (s. n) or precedent ([YEAR] LEXBY-...).' },
    statutes_relied: { type: 'array', items: { type: 'string' }, description: 'SPEC-LAW sections relied on, with a clause on how each supports the side.' },
    precedents_relied: { type: 'array', items: { type: 'string' }, description: 'Neutral citations relied on or distinguished, with the point taken from each.' },
    procedural_motions: { type: 'array', items: { type: 'string' }, description: 'Any motion made (e.g. strike-out for want of standing/jurisdiction, per incuriam, fast-path disposal), each with its grounds. Empty if none.' },
    strongest_opposing_point: { type: 'string', description: 'The single strongest point AGAINST this side, stated honestly, and the best answer to it.' },
  },
}

function researchPrompt(role, file) {
  const roleBrief = role === 'claimant'
    ? 'You are COUNSEL FOR THE CLAIMANT (the moving party): you argue FOR the proposition / relief sought in the question. Build the strongest case that the court should ALLOW / grant / answer the question in the affirmative.'
    : role === 'defendant'
      ? 'You are COUNSEL FOR THE DEFENDANT (the responding party): you argue AGAINST the proposition / relief. Build the strongest case that the court should DISMISS / refuse / answer in the negative, including any motion to strike out the claim at the threshold.'
      : `You are the OBSERVER (amicus curiae) - an independent friend of the Supreme Court, admitted to these proceedings by the judge who reviewed the appeal (or by the Principal's leapfrog certificate). You owe no allegiance to either party. Your duty is to surface the considerations BOTH sides may have missed: the systemic and realm-wide consequences of each possible ruling, any binding law neither side cited, and the formulation that best serves the long-run coherence of SPEC-LAW.${leave.observer_brief ? '\n\nThe judge who admitted you directs you to bring this perspective in particular: ' + leave.observer_brief : ''}`
  return `${roleBrief}

This is the mandatory hard-research first leg of the trial. Research the law HARD before you argue. You have access to the repository: you may READ the full text of any ruling under .justice/judgments/ (do not rely only on the one-line ratios in the index) and re-read SPEC-LAW.md in full. Ground every argument in a cited article (s. n) or a neutral citation. Do NOT use em dashes or en dashes.

${ARSENAL}

THE CASE FILE:
${file}

Produce your brief. Be adversarial and thorough for your side (the observer is balanced, not adversarial), but never misstate the law: a brief that misrepresents a statute or precedent fails the court.`
}

// The observer joins only if admitted by the reviewing leave-judge (or the leapfrog certificate).
const researchRoles = leave.admit_observer ? ['claimant', 'defendant', 'observer'] : ['claimant', 'defendant']
if (leave.admit_observer) log('Observer (amicus) admitted to the proceedings.')
const briefs = await parallel(researchRoles.map(role => () => agent(
  researchPrompt(role, caseFile),
  { label: `${role} brief`, phase: 'Hard research - both sides', agentType: 'Explore', schema: BRIEF_SCHEMA }
)))

const briefsSection = researchRoles.map((role, i) => {
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

const fullCaseFile = `${caseFile}

================================================================
ADVERSARIAL BRIEFS (the first leg of the trial - both sides researched the law hard)
================================================================
${briefsSection}`

// --------------------------------------------------------------------------
// Independent opinions
// --------------------------------------------------------------------------
phase('Justices - Independent Opinions')
const opinions = await parallel(panel.map(j => () => agent(
  `${j.brief}

You are sitting on the Supreme Court of the Vibe Justice System (VJS), the apex court. Your opinion is formal, authoritative, and written in the register of an apex judge. Do NOT use em dashes or en dashes anywhere.

You have a fully-argued, symmetric record: both parties researched the law hard and filed briefs, and (in a constitutional sitting) an independent observer filed too. Weigh the briefs; you are not bound by either; rule on the law.

READ THE CASE FILE CAREFULLY:

${fullCaseFile}

WRITE YOUR INDIVIDUAL OPINION, in this order:
1. PRELIMINARY (1-2 sentences): any material procedural point, or none.
2. THE QUESTION: restate the question as you frame it.
3. REASONING: apply SPEC-LAW and caselaw with precision; engage the lower reasoning where there is any; state where you agree and where you diverge.
4. RATIO: one or two numbered propositions, stated as rules that could bind a future court.
5. OBITER (if any): observations not forming part of your ratio.
6. DISPOSITION: one word - ALLOW, DISMISS, or VARY - with a single sentence.
7. PROPOSED ENACTMENTS (if any): if new SPEC-LAW is required, set out the article text in full, headed "PROPOSED S-[n]: [short title]"; else "No new statute proposed."
8. PROPOSED OVERRULINGS (if any): cite any ruling you would overrule by neutral citation with a sentence of reason; else "No overrulings proposed."

Sign: --- ${j.name}`,
  { label: `Opinion of ${j.name}`, phase: 'Justices - Independent Opinions' }
)))

const opinionRecord = panel
  .map((j, i) => `=== ${j.name.toUpperCase()} ===\n\n${opinions[i]}`)
  .join('\n\n---\n\n')

// --------------------------------------------------------------------------
// Leading judgment
// --------------------------------------------------------------------------
phase('Leading Judgment')
const leadingJudgment = await agent(
  `${BENCH[0].brief}

You are delivering the LEADING JUDGMENT of the Supreme Court of the Vibe Justice System (VJS). You have read every individual opinion. Synthesise the majority into a single authoritative ruling. Do NOT use em dashes or en dashes.

THE CASE FILE:
${caseFile}

INDIVIDUAL OPINIONS FROM THE BENCH:
${opinionRecord}

WRITE THE LEADING JUDGMENT, containing:
1. BENCH AND PROCEDURE: identify the panel, whether full court (9) or panel (5), and the procedural basis (leapfrog certificate or permission to appeal).
2. THE QUESTION: frame it with precision.
3. MAJORITY REASONING: the authoritative analysis. This is the ratio of the Court and must be complete enough to bind a future court. Draw on the individual opinions where they converge; note the weight of authority behind each limb.
4. DISSENTS AND CONCURRENCES: summarise each dissenting or separately concurring opinion accurately and fairly. Place them on the record; do not dismiss them.
5. DISPOSAL: ALLOWED, DISMISSED, or VARIED, with the exact consequence.
6. ENACTS (new SPEC-LAW articles, if any): each new article by proposed number and full text, headed "ENACTS S-[n]: [title]". If none, write "No new statute enacted." Mark whether each new article is [constitutional] or ordinary.
7. OVERRULES (lower rulings overruled, if any): each by neutral citation with a sentence of reason. If none, "No rulings overruled."

Sign: --- Hallam CJ, delivering the judgment of the Court [plus any co-signatories who concur in full]`,
  { label: 'Leading Judgment - Hallam CJ', phase: 'Leading Judgment' }
)

// --------------------------------------------------------------------------
// Lexby translation
// --------------------------------------------------------------------------
phase('Lexby Translation')
const lexbyTranslation = await agent(
  `You are Lexby: the principal's counsel and an officer of the court (advocate, advisor, engineer, SPEC-LAW s.3). The Supreme Court has delivered its judgment. Translate the whole proceeding into plain English for the principal, who is intelligent but not a lawyer. Do NOT use em dashes or en dashes. Write in the first person as Lexby: direct, warm, completely plain.

Cover:
1. What the court was asked.
2. What each justice thought (one sentence per justice, capturing their key point and whether they concurred or dissented).
3. What the court decided and why, in plain terms.
4. What new law (if any) was enacted and what it means in practice.
5. What lower rulings (if any) were overruled and what that changes.
6. What the principal should do next.

THE LEADING JUDGMENT:
${leadingJudgment}

THE INDIVIDUAL OPINIONS:
${opinionRecord}`,
  { label: 'Lexby - translation', phase: 'Lexby Translation' }
)

// --------------------------------------------------------------------------
// Structured extract (clean fields for filing the ruling + enacting statute)
// --------------------------------------------------------------------------
phase('Structured Extract')
const SUMMARY_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  required: ['citation_id', 'disposition', 'ratio', 'obiter', 'is_constitutional', 'panel_vote', 'enacts', 'overrules'],
  properties: {
    citation_id: { type: 'string', description: 'The neutral citation, exactly: ' + assignedCitation },
    disposition: { type: 'string', enum: ['ALLOWED', 'DISMISSED', 'VARIED', 'DECLARATORY'], description: 'The disposal. Use DECLARATORY for a foundational ruling that answers a question without an appeal to allow or dismiss.' },
    ratio: { type: 'string', description: 'The binding ratio of the Court, stated as one or more numbered rules of law. This is what binds future courts.' },
    obiter: { type: ['string', 'null'], description: 'Material obiter, or null.' },
    is_constitutional: { type: 'boolean', description: 'True if the matter was treated as constitutional/foundational (full court of 9).' },
    panel_vote: { type: 'string', description: 'The vote, e.g. "unanimous" or "7:2 (Bowan J, Coade J dissenting)".' },
    enacts: {
      type: 'array',
      description: 'Every SPEC-LAW article enacted by this judgment. Empty array if none.',
      items: {
        type: 'object',
        additionalProperties: false,
        required: ['section', 'title', 'entrenchment', 'text'],
        properties: {
          section: { type: 'string', description: 'e.g. "s. 17"' },
          title: { type: 'string' },
          entrenchment: { type: 'string', enum: ['constitutional', 'ordinary'] },
          text: { type: 'string', description: 'The full operative article text, verbatim from the judgment.' },
        },
      },
    },
    overrules: {
      type: 'array',
      description: 'Every lower ruling overruled. Empty array if none.',
      items: {
        type: 'object',
        additionalProperties: false,
        required: ['citation', 'reason'],
        properties: {
          citation: { type: 'string' },
          reason: { type: 'string' },
        },
      },
    },
  },
}
const summary = await agent(
  `You are the VJS clerk extracting structured fields from the Supreme Court's leading judgment for the record. Do not decide anything; extract faithfully. Use the assigned citation ${assignedCitation} for citation_id. Pull the ratio, disposition, vote, every enacted SPEC-LAW article (verbatim), and every overruling from the judgment below.

LEADING JUDGMENT:
${leadingJudgment}`,
  { label: 'clerk - structured extract', phase: 'Structured Extract', schema: SUMMARY_SCHEMA }
)
summary.citation_id = assignedCitation

// --------------------------------------------------------------------------
// PDF Render (deterministic; cwd-independent). Writes the judgment PDF into the
// repo's .justice/pdfs/ so the record is uploaded as a PDF as well as markdown.
// --------------------------------------------------------------------------
phase('PDF Render')
const scSlug = assignedCitation.replace(/[\[\]\s]+/g, '-').replace(/^-|-$/g, '').toLowerCase()
const rulingForPdf = {
  tier: 'supreme-court',
  ruling: {
    citation_id: assignedCitation,
    tier: 'supreme-court',
    panel: panel.map(j => j.name),
    kind: 'request_for_ruling',
    question_or_charge: String(args.question || '').slice(0, 400),
    full_judgment_text: leadingJudgment,
    ratio: summary.ratio,
    status: 'good-law',
  },
  lexby_translation: { plain_english_summary: String(lexbyTranslation || '').slice(0, 4000) },
}
const pdfPath = await agent(
  `You are rendering the Supreme Court judgment as a PDF. Be deterministic; do not editorialise.
1. Locate the VJS repo root: the nearest directory that contains BOTH court/renderer/index.js AND .justice/ (you already know .justice/INDEX.md exists; resolve its repo root). Do not assume the current working directory is the repo.
2. If <root>/court/renderer/node_modules does not exist, print exactly RENDERER-NOT-INSTALLED and stop.
3. Write this JSON verbatim to /tmp/vjs-ruling-${scSlug}.json:
${JSON.stringify(rulingForPdf)}
4. Run: cd <root> && mkdir -p .justice/pdfs && node court/renderer/index.js /tmp/vjs-ruling-${scSlug}.json .justice/pdfs/${scSlug}.pdf
5. Return the absolute path to the PDF, or RENDERER-NOT-INSTALLED.`,
  { label: 'PDF Render', phase: 'PDF Render', agentType: 'claude' }
)
const judgmentPdf = pdfPath && !pdfPath.includes('NOT-INSTALLED') ? pdfPath.trim() : null
if (judgmentPdf) log('Judgment PDF: ' + judgmentPdf)
else log('PDF not rendered (renderer not installed in the repo). Render later with: node court/renderer/index.js <ruling.json> .justice/pdfs/' + scSlug + '.pdf')

return {
  citation: assignedCitation,
  court_type: courtType,
  leave,
  panel: panel.map(j => j.name),
  opinions: panel.map((j, i) => ({ justice: j.name, opinion: opinions[i] })),
  opinion_record: opinionRecord,
  leading_judgment: leadingJudgment,
  lexby_translation: lexbyTranslation,
  summary,
  judgment_pdf: judgmentPdf,
}
