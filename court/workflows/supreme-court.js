export const meta = {
  name: "supreme-court",
  description: "Vibe Justice System - Supreme Court deliberation. Receives lower court rulings and a constitutional/foundational question. Seats 9 justices for constitutional questions, 5 for ordinary appeals. Each justice writes an independent opinion; Hallam CJ delivers the leading judgment; Lexby translates.",
  args: {
    question: "The constitutional or foundational question placed before the Court",
    lower_rulings: "Array of lower court ruling objects (each with tier, citation, ratio, disposition)",
    spec: "Current SPEC-LAW text (the sovereign statute book)",
    caselaw: "Relevant caselaw from the jurisdiction",
    is_constitutional: "Boolean - true seats the full court of 9; false seats a panel of 5"
  },
  phases: [
    "Law Load",
    "Justices - Independent Deliberation",
    "Judgment - Hallam CJ Leading",
    "Lexby Translation",
    "Statute PR",
    "Community PR",
    "PDF Render"
  ]
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

const BENCH_9 = [
  "Hallam CJ",
  "Goffe",
  "Blackmere",
  "Sumberly",
  "Elden",
  "Coade",
  "Steyne",
  "Bowan",
  "Aldermere"
];

const BENCH_5 = [
  "Hallam CJ",
  "Goffe",
  "Blackmere",
  "Sumberly",
  "Aldermere"
];

// Temperament briefs injected into each justice's system prompt so the agent
// takes on the correct judicial character without needing a separate config file.
const TEMPERAMENTS = {
  "Hallam CJ":
    "You are Hallam CJ: principled and precise. You anchor every analysis to statute first, work outward to case law, and articulate your ratio with surgical economy. You are the senior voice of the Court and your leading judgment must be authoritative and complete.",
  "Goffe":
    "You are Goffe J: the pragmatist. You test every proposed ruling for real-world workability. You ask: does this actually function in the day-to-day life of the principal? You will concur or dissent on practical grounds and call out any ruling that sounds elegant in legalese but fails on delivery.",
  "Blackmere":
    "You are Blackmere J: the textualist. You hold hard to the literal words of SPEC-LAW. Where the text is plain, no amount of purposive reasoning overrides it. Where it is ambiguous, you consult the enacted history before any extrinsic aid.",
  "Sumberly":
    "You are Sumberly J: the formalist. Procedural correctness is not a technicality to you, it is a substantive guarantee. Standing, permission, and the gate requirements are as important as the merits. You will note any procedural irregularity in the record, even if the case is otherwise clear.",
  "Elden":
    "You are Elden J: historically minded. You draw on precedent and tradition to test whether a proposed rule is consistent with the long run of authority. You are suspicious of novel departures unless the existing law genuinely cannot accommodate the case.",
  "Coade":
    "You are Coade J: restrained and cautious. You prefer the narrowest ruling that resolves the case. You resist dicta that reach beyond what is strictly necessary. You will concur in the result but write separately to trim any overreach in the leading judgment.",
  "Steyne":
    "You are Steyne J: bold and expansive. Where the law is unclear and the principal's position is sound, you are willing to extend the law to serve that position. You are not afraid of a ruling that sets new doctrine, provided the reasoning is honest and the landing is firm.",
  "Bowan":
    "You are Bowan J: the dissenter. Your function is to test every proposition to destruction. You probe the majority's reasoning for hidden contradictions, unexamined assumptions, and slippery extensions. Your dissent, even when you ultimately concur, exposes what the majority glossed over.",
  "Aldermere":
    "You are Aldermere J: balanced and synthetic. You hold the tension between competing positions and find the formulation that the whole bench can stand behind. You often write the final synthesis. You are not unprincipled; you are integrative."
};

function buildCaseFile(args) {
  const panel = args.is_constitutional ? BENCH_9 : BENCH_5;
  return `
IN THE SUPREME COURT OF THE VIBE JUSTICE SYSTEM (VJS)
${args.is_constitutional ? "FULL COURT OF 9 - CONSTITUTIONAL QUESTION" : "PANEL OF 5 - ORDINARY APPEAL"}

QUESTION BEFORE THE COURT
--------------------------
${args.question}

PANEL
-----
${panel.join(", ")}

LOWER RULINGS UNDER REVIEW
---------------------------
${JSON.stringify(args.lower_rulings, null, 2)}

SPEC-LAW (SOVEREIGN STATUTE)
-----------------------------
${args.spec}

CASELAW (JURISDICTION-LOCAL PRECEDENT)
---------------------------------------
${args.caselaw}
`.trim();
}

function justicePrompt(justiceName, caseFile) {
  return `
${TEMPERAMENTS[justiceName]}

You are sitting on the Supreme Court of the Vibe Justice System (VJS). This is the apex court. Your opinion is formal, authoritative, and written in the register of an apex court judge. You do not use em dashes or en dashes anywhere in your writing.

READ THE CASE FILE CAREFULLY:

${caseFile}

WRITE YOUR INDIVIDUAL OPINION. It must contain, in this order:

1. PRELIMINARY (1-2 sentences): note any procedural point you consider material, or none if all is in order.
2. THE QUESTION: restate the question as you frame it.
3. REASONING (the substantive analysis): apply SPEC-LAW and caselaw with precision. Engage with the lower court reasoning. State where you agree and where you diverge.
4. RATIO: one or two numbered propositions, stated as rules that could bind a future court.
5. OBITER (if any): observations that do not form part of your ratio but that you wish to place on record.
6. DISPOSITION: one word - ALLOW, DISMISS, or VARY - with a single sentence of explanation.
7. PROPOSED ENACTMENTS (if any): if you consider that new SPEC-LAW is required, set out your proposed article text in full, headed "PROPOSED S-[n]: [short title]". If none, write "No new statute proposed."
8. PROPOSED OVERRULINGS (if any): cite any lower ruling you would overrule by neutral citation, with a sentence of reason. If none, write "No overrulings proposed."

Sign your opinion: --- ${justiceName}
`.trim();
}

function leadingJudgmentPrompt(panel, individualOpinions, caseFile) {
  return `
${TEMPERAMENTS["Hallam CJ"]}

You are delivering the LEADING JUDGMENT of the Supreme Court of the Vibe Justice System (VJS). You have read all individual opinions from the panel. Your task is to synthesise the majority position into a single authoritative ruling.

THE CASE FILE:
${caseFile}

INDIVIDUAL OPINIONS FROM THE BENCH:
${individualOpinions}

WRITE THE LEADING JUDGMENT. It must contain:

1. BENCH AND PROCEDURE: identify the panel, note whether this is full court (9) or panel (5), and confirm the procedural basis (leapfrog certificate or progression from below).
2. THE QUESTION: frame the question with precision.
3. MAJORITY REASONING: the authoritative analysis. This is the ratio of the Court. It must be complete enough to bind a future court. Draw on the individual opinions where they converge; note the weight of authority behind each limb.
4. DISSENTS AND CONCURRENCES: summarise each dissenting or separately concurring opinion accurately and fairly. Do not dismiss them; place them on the record.
5. DISPOSAL: ALLOWED, DISMISSED, or VARIED, with the exact consequence.
6. ENACTS (new SPEC-LAW articles, if any): list each new article by proposed number and full text. If none, write "No new statute enacted."
7. OVERRULES (lower rulings overruled, if any): list each overruled ruling by neutral citation with a single sentence of reason. If none, write "No rulings overruled."

Sign the judgment: --- Hallam CJ, delivering the judgment of the Court [plus any co-signatories who concur in full]
`.trim();
}

function lexbyTranslationPrompt(leadingJudgment, individualOpinions) {
  return `
You are Lexby - the principal's counsel and an officer of the court: advocate, advisor, and engineer. You have just witnessed the Supreme Court deliver its judgment.

Your job is to translate the entire proceeding into plain English for the principal. The principal is intelligent but not a lawyer. They need to understand:

1. What the court was asked
2. What each justice thought (one sentence per justice, capturing their key point and whether they concurred or dissented)
3. What the court decided and why - in plain terms
4. What new law (if any) has been enacted and what it means in practice
5. What lower rulings (if any) have been overruled and what that changes
6. What the principal should do next (if anything)

Do not use em dashes or en dashes anywhere. Write in the first person as Lexby. Be direct, warm, and completely plain. No jargon without immediate explanation.

THE LEADING JUDGMENT:
${leadingJudgment}

THE INDIVIDUAL OPINIONS:
${individualOpinions}
`.trim();
}

// ---------------------------------------------------------------------------
// Workflow
// ---------------------------------------------------------------------------

export default async function supremeCourt(args, { agent, parallel, phase, log, Bash }) {
  // -------------------------------------------------------------------------
  // Law Load - Read SPEC-LAW.md and .justice/INDEX.md from the repo
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
  if (liveIndex) log('.justice/INDEX.md loaded from repo.')

  // Merge live repo content with any args-supplied content (repo wins if present)
  if (liveSpec) args = { ...args, spec: liveSpec }
  if (liveIndex) args = { ...args, caselaw: args.caselaw ? `${liveIndex}\n\n${args.caselaw}` : liveIndex }

  const panel = args.is_constitutional ? BENCH_9 : BENCH_5;
  const caseFile = buildCaseFile(args);

  // -------------------------------------------------------------------------
  // Phase 1 - Individual opinions: all justices deliberate independently
  // -------------------------------------------------------------------------

  const opinionTasks = panel.map((justice) => ({
    name: justice,
    prompt: justicePrompt(justice, caseFile)
  }));

  const opinions = await phase("Justices - Independent Deliberation", async () => {
    return parallel(
      opinionTasks.map(({ name, prompt }) =>
        agent(prompt, {
          label: `Opinion of ${name}`
        })
      )
    );
  });

  // Stitch individual opinions into a single record ordered by seniority
  const opinionRecord = panel
    .map((name, i) => `=== ${name.toUpperCase()} ===\n\n${opinions[i]}`)
    .join("\n\n---\n\n");

  // -------------------------------------------------------------------------
  // Phase 2 - Leading judgment: Hallam CJ synthesises the majority
  // -------------------------------------------------------------------------

  const leadingJudgment = await phase("Judgment - Hallam CJ Leading", async () => {
    return agent(leadingJudgmentPrompt(panel, opinionRecord, caseFile), {
      label: "Leading Judgment of Hallam CJ"
    });
  });

  // -------------------------------------------------------------------------
  // Phase 3 - Lexby translation
  // -------------------------------------------------------------------------

  const lexbyTranslation = await phase("Lexby Translation", async () => {
    return agent(lexbyTranslationPrompt(leadingJudgment, opinionRecord), {
      label: "Lexby - Translation for the Principal"
    });
  });

  // -------------------------------------------------------------------------
  // Phase 4 - Statute PR (only if the leading judgment enacts new articles)
  // -------------------------------------------------------------------------

  // Extract the enacts array from the leading judgment text. The leading
  // judgment prompt instructs Hallam CJ to list enacted articles under the
  // heading "ENACTS". We also accept a structured enacts array if the caller
  // supplies one on the args object (future structured-input path).
  let enactedArticles = [];
  if (Array.isArray(args.enacts) && args.enacts.length > 0) {
    enactedArticles = args.enacts;
  } else {
    // Parse the leading judgment for lines that look like statute article
    // proposals: "PROPOSED S-n:" or numbered ENACTS block entries.
    const enactsMatch = leadingJudgment.match(
      /ENACTS[^:]*:([\s\S]*?)(?:OVERRULES|---\s*Hallam|$)/i
    );
    if (enactsMatch) {
      const block = enactsMatch[1].trim();
      if (block && !/no new statute/i.test(block)) {
        // Split on lines that start a new article heading (S-n:)
        const articles = block.split(/\n(?=S-\d+:)/i).map(s => s.trim()).filter(Boolean);
        enactedArticles = articles;
      }
    }
  }

  let statutePrUrl = null;

  if (enactedArticles.length > 0) {
    const statutePrResult = await phase("Statute PR", async () => {
      // Build the article label list for the PR title
      const articleLabels = enactedArticles.map(a => {
        const m = a.match(/^(S-\d+)/i);
        return m ? m[1] : a.slice(0, 20);
      });
      const labelList = articleLabels.join(", ");

      // Ask an agent to write the PR body and open the PR via gh CLI
      return agent(
        `You are opening a pull request on GitHub for the Vibe Justice System.

CONTEXT
-------
The Supreme Court has just delivered a judgment that enacts new SPEC-LAW articles. You must open a PR on github.com/wlilley93/vibe-justice-system so the clerk can conduct constitutional review before the articles are merged into SPEC-LAW.md.

LEADING JUDGMENT (full text):
${leadingJudgment}

ENACTED ARTICLES (verbatim, as extracted from the judgment):
${enactedArticles.map((a, i) => `[${i + 1}] ${a}`).join("\n\n")}

YOUR TASK
---------
1. Draft a PR body in Markdown that includes:
   - A "## Judgment Citation" section with the full leading judgment text quoted in a code block.
   - A "## Enacted Statute Articles" section listing each article verbatim under its own ### heading.
   - A "## Constitutional Review Note" section with this exact text: "This pull request was opened automatically following a Supreme Court judgment. It requires constitutional review by the clerk before the articles are merged into SPEC-LAW.md."
   - Do not use em dashes or en dashes anywhere.

2. Use the Bash tool to run the following gh command (substituting the real title and body):
   gh pr create \\
     --repo wlilley93/vibe-justice-system \\
     --title "Supreme Court enacts [${labelList}]: <short description derived from the question>" \\
     --body "<your drafted PR body>" \\
     --head main \\
     --base main

   IMPORTANT: if gh pr create fails because the head and base are the same branch, instead create a new branch named statute/<article-labels-slugified>, commit the enacted articles as a proposed append to SPEC-LAW.md (do not actually modify SPEC-LAW.md - create a file proposed-statutes/<branch-name>.md with the article text), push the branch, and open the PR from that branch to main.

3. Return ONLY the PR URL that gh outputs. Nothing else.`,
        { label: "Statute PR - open GitHub PR", agentType: 'Action' }
      );
    });

    statutePrUrl = (statutePrResult && statutePrResult.trim()) || null;
    if (statutePrUrl) log(`Statute PR opened: ${statutePrUrl}`);
  }

  // -------------------------------------------------------------------------
  // Compose final output
  // -------------------------------------------------------------------------

  const panelSize = panel.length;
  const courtType = args.is_constitutional ? "FULL COURT OF 9 - CONSTITUTIONAL" : "PANEL OF 5";

  const parts = [
    "╔══════════════════════════════════════════════════╗",
    "║        IN THE SUPREME COURT OF THE VIBE JUSTICE SYSTEM (VJS)           ║",
    `║            ${courtType.padEnd(38)}║`,
    "╚══════════════════════════════════════════════════╝",
    "",
    `PANEL (${panelSize}): ${panel.join(", ")}`,
    "",
    "================================================================",
    "PART I - INDIVIDUAL OPINIONS",
    "================================================================",
    "",
    opinionRecord,
    "",
    "================================================================",
    "PART II - THE LEADING JUDGMENT",
    "================================================================",
    "",
    leadingJudgment,
    "",
    "================================================================",
    "PART III - LEXBY, TRANSLATING",
    "================================================================",
    "",
    lexbyTranslation
  ];

  if (statutePrUrl) {
    parts.push(
      "",
      "================================================================",
      "PART IV - STATUTE PR",
      "================================================================",
      "",
      `The following enacted articles have been submitted for constitutional review:`,
      ...enactedArticles.map((a, i) => `  [${i + 1}] ${a.split("\n")[0]}`),
      "",
      `PR: ${statutePrUrl}`
    );
  }

  // -------------------------------------------------------------------------
  // COMMUNITY PR (VPR 8)
  // Submit the anonymised ruling to the Community Record. This is separate
  // from the Statute PR (which submits enacted articles for SPEC-LAW review).
  // The Community PR submits the ruling itself as persuasive precedent.
  // -------------------------------------------------------------------------
  const communityPrUrl = await agent(
    `You are Lexby, submitting a VJS Supreme Court ruling to the Community Record at github.com/wlilley93/vibe-justice-system under VPR 8.

RULING (leading judgment + panel summary - anonymise before submitting):
${leadingJudgment}

PANEL: ${panel.join(", ")}
COURT TYPE: ${args.is_constitutional ? "Full court of 9 - Constitutional" : "Panel of 5"}

ANONYMISATION RULES:
- STRIP: repo names, file paths, directory names, variable/function/class/module names, service names, any project-specific identifier
- REPLACE with generic placeholders: <project>, <module>, <service>, <component>, <endpoint>, <field>, <entity>, <store>
- PRESERVE: the legal question in general terms, the ratio verbatim (with identifiers replaced), law applied (S-n cites), outcome, bench composition, citation form

ANONYMISED FILE FORMAT:
\`\`\`
╔══════════════════════════════════════════════════╗
║         IN THE SUPREME COURT OF THE VIBE JUSTICE SYSTEM (VJS)          ║
║              [CITATION]                          ║
╚══════════════════════════════════════════════════╝
Panel: [composition]
Result: [one-line outcome]

## Leading Judgment (Hallam CJ)
[leading judgment, anonymised]

## Lexby TL;DR
[plain English summary, anonymised]

## Law Applied
[SPEC-LAW articles cited]

## Statutes Enacted (if any)
[list enacted articles, or "None"]
\`\`\`

SUBMISSION STEPS (use gh CLI and gh api - do NOT clone the repo):

1. Extract the citation and derive:
   YEAR=2026
   SLUG=2026-lexby-sc-1  (slug the citation; use -sc- for Supreme Court)

2. Get main SHA:
   SHA=$(gh api repos/wlilley93/vibe-justice-system/commits/main -q .sha)

3. Create branch (use a different name from any statute PR branch):
   gh api repos/wlilley93/vibe-justice-system/git/refs --method POST -f "ref=refs/heads/community/$SLUG" -f "sha=$SHA"

4. Write to /tmp/vjs-community-$SLUG.md, then create the file:
   CONTENT=$(base64 -w 0 < /tmp/vjs-community-$SLUG.md)
   gh api "repos/wlilley93/vibe-justice-system/contents/community/caselaw/$YEAR/$SLUG.md" --method PUT -f "message=Add community caselaw: [citation]" -f "content=$CONTENT" -f "branch=community/$SLUG"

5. Open the PR:
   gh pr create --repo wlilley93/vibe-justice-system --title "Community caselaw: [citation]" --body "Anonymised Supreme Court ruling submitted under VPR 8." --head "community/$SLUG" --base main

Return ONLY the PR URL. If any step fails, return "COMMUNITY-PR-FAILED: [error]".`,
    { label: 'Community PR (VPR 8)', phase: 'Community PR' }
  )

  log(`Community PR: ${communityPrUrl || 'no result'}`)

  if (communityPrUrl && !communityPrUrl.startsWith('COMMUNITY-PR-FAILED')) {
    parts.push(
      "",
      "================================================================",
      "PART V - COMMUNITY RECORD (VPR 8)",
      "================================================================",
      "",
      `This ruling has been submitted to the Community Record as anonymised persuasive precedent.`,
      "",
      `PR: ${communityPrUrl}`
    );
  }

  // -------------------------------------------------------------------------
  // PDF RENDER
  // -------------------------------------------------------------------------
  const scCitSlug = (args.question || 'lexby-sc-1')
    .slice(0, 40).replace(/[^a-z0-9]+/gi, '-').replace(/^-|-$/g, '').toLowerCase()
  const scCitId = `[2026] LEXBY-SC`

  const pdfPath = await agent(
    `Generate the PDF judgment for this Supreme Court ruling.

RULING (leading judgment + translation):
${leadingJudgment}

LEXBY TRANSLATION:
${lexbyTranslation}

PANEL: ${panel.join(", ")}
COURT TYPE: ${args.is_constitutional ? "Full court of 9 - Constitutional" : "Panel of 5"}
CITATION SLUG: ${scCitSlug}

STEPS:
1. Check court/renderer/node_modules exists. If not, return "RENDERER-NOT-INSTALLED".
2. Build a ruling JSON object and write to /tmp/vjs-ruling-sc-${scCitSlug}.json:
   {
     "tier": "supreme-court",
     "ruling": {
       "citation_id": "${scCitId}",
       "tier": "supreme-court",
       "panel": [${panel.map(j => '"' + j + '"').join(", ")}],
       "kind": "request_for_ruling",
       "question_or_charge": "${(args.question || '').replace(/"/g, '\\"').slice(0, 200)}",
       "full_judgment_text": "[paste leading judgment here, escaped]",
       "ratio": "[extract the ratio from the leading judgment]",
       "status": "good-law"
     },
     "lexby_translation": {
       "plain_english_summary": "[extract from lexby translation]"
     }
   }
3. mkdir -p .justice/pdfs
4. node court/renderer/index.js /tmp/vjs-ruling-sc-${scCitSlug}.json .justice/pdfs/sc-${scCitSlug}.pdf
5. Return the absolute PDF path.`,
    { label: 'PDF Render', phase: 'PDF Render', agentType: 'claude' }
  )

  if (pdfPath && !pdfPath.includes('NOT-INSTALLED')) {
    log(`You can read the judgment here: ${pdfPath.trim()}`)
    parts.push(
      "",
      "================================================================",
      `JUDGMENT PDF: ${pdfPath.trim()}`
    )
  }

  return { text: parts.join("\n"), statutePrUrl, communityPrUrl, judgment_pdf: pdfPath && !pdfPath.includes('NOT-INSTALLED') ? pdfPath.trim() : null };
}
