export const meta = {
  name: "supreme-council",
  description: "Vibe Justice System - Supreme Council deliberation. Receives lower court rulings and a constitutional/foundational question. Seats 9 justices for constitutional questions, 5 for ordinary appeals. Each justice writes an independent opinion; Hallam CJ delivers the leading judgment; Lexby translates.",
  args: {
    question: "The constitutional or foundational question placed before the Court",
    lower_rulings: "Array of lower court ruling objects (each with tier, citation, ratio, disposition)",
    spec: "Current SPEC-LAW text (the sovereign statute book)",
    caselaw: "Relevant caselaw from the jurisdiction",
    is_constitutional: "Boolean - true seats the full court of 9; false seats a panel of 5"
  }
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
IN THE SUPREME COUNCIL OF LEXBY
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

You are sitting on the Supreme Council of Lexby. This is the apex court. Your opinion is formal, authoritative, and written in the register of a senior judge in the UK Supreme Court. You do not use em dashes or en dashes anywhere in your writing.

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

You are delivering the LEADING JUDGMENT of the Supreme Council of Lexby. You have read all individual opinions from the panel. Your task is to synthesise the majority position into a single authoritative ruling.

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
You are Lexby - the principal's counsel and an officer of the court: advocate, advisor, and engineer. You have just witnessed the Supreme Council deliver its judgment.

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

export default async function supremeCouncil(args, { agent, parallel, phase }) {
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
  // Compose final output
  // -------------------------------------------------------------------------

  const panelSize = panel.length;
  const courtType = args.is_constitutional ? "FULL COURT OF 9 - CONSTITUTIONAL" : "PANEL OF 5";

  return [
    "╔══════════════════════════════════════════════════╗",
    "║        IN THE SUPREME COUNCIL OF LEXBY           ║",
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
  ].join("\n");
}
