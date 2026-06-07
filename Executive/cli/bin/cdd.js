#!/usr/bin/env node
'use strict';
// Vibe Justice System CLI. Commands: init, next-citation, submit-request, submit-breach.
// No runtime dependencies (pure Node).

const fs = require('fs');
const path = require('path');
const { nextCitation, seriesCode } = require('../lib/citation');
const { auditCitator } = require('../lib/citator-audit');
const { scanBenchNames } = require('../lib/bench-name-scan');

const CLI_ROOT = path.resolve(__dirname, '..');
const REPO_ROOT = path.resolve(__dirname, '..', '..', '..'); // the vibe-justice-system repo root
const VERSION = require('../package.json').version;

function findCitator(dir) {
  for (const rel of ['Judicature/.justice/INDEX.md', 'Judicature/caselaw/INDEX.md', '.justice/INDEX.md', 'caselaw/INDEX.md']) {
    const p = path.join(dir, rel);
    if (fs.existsSync(p)) return p;
  }
  return null;
}

// Statutory instruments are numbered from their own register, not the judgments citator
// ([2026] REALM-PC 11: one flat REALM-SI ordinal). Walk up to find the SI register.
function findSIRegister(dir) {
  for (const rel of ['Legislature/statutes/instruments/INDEX.md', 'statutes/instruments/INDEX.md']) {
    const p = path.join(dir, rel);
    if (fs.existsSync(p)) return p;
  }
  return null;
}

function die(msg, code = 1) { process.stderr.write(msg + '\n'); process.exit(code); }

function cmdNextCitation(args) {
  const tier = args._[0];
  if (!tier) die('usage: cdd next-citation <supreme-court|court-of-appeal|privy-council|high-court|county-court> [--division D] [--repo R] [--year YYYY] [--citator PATH] [--json]');
  const opts = {};
  if (args.division) opts.division = args.division;
  if (args.repo) opts.repo = args.repo;
  if (args.year) opts.year = parseInt(args.year, 10);
  try { seriesCode(tier, opts); } catch (e) { die('next-citation: ' + e.message); } // validate early
  // SIs number from the SI register; everything else from the judgments citator.
  const isSI = /^(si|statutory[-_]instrument)$/i.test(tier);
  const citatorPath = args.citator || (isSI ? findSIRegister(process.cwd()) : findCitator(process.cwd()));
  const text = citatorPath && fs.existsSync(citatorPath) ? fs.readFileSync(citatorPath, 'utf8') : '';
  if (!citatorPath) process.stderr.write(`note: no ${isSI ? 'SI register' : '.justice/INDEX.md'} found; numbering from empty (this will be N=1)\n`);
  const r = nextCitation(text, tier, opts);
  if (args.json) process.stdout.write(JSON.stringify(r) + '\n');
  else process.stdout.write(r.citation + '\n');
}

function cmdCheckCitator() {
  const res = auditCitator(process.cwd());
  if (res.ok) { process.stdout.write('citator OK: no collisions, every ruling filed and indexed.\n'); return; }
  for (const p of res.problems) process.stderr.write('FAIL [' + p.type + ']: ' + p.message + '\n');
  process.stderr.write(`\n${res.problems.length} problem(s). The record is inconsistent; fix the citator/ruling files.\n`);
  process.exit(1);
}

function cmdCheckBenchNames(args) {
  if (args['source-only'] && args['corpus-only']) die('check-bench-names: choose at most one of --source-only or --corpus-only');
  const res = scanBenchNames(process.cwd(), {
    root: args.root,
    sources: !args['corpus-only'],
    corpus: !args['source-only'],
  });
  if (args.json) {
    process.stdout.write(JSON.stringify(res, null, 2) + '\n');
    if (!res.ok) process.exit(1);
    return;
  }
  if (res.ok) {
    process.stdout.write(`bench-name scan OK: ${res.scanned.length} target(s), no prohibited real jurist labels.\n`);
    return;
  }
  for (const f of res.findings) {
    if (f.type === 'prohibited-bench-name') {
      const loc = f.line ? `${f.path}:${f.line}:${f.column}` : `${f.path}${f.pointer ? '#' + f.pointer : ''}`;
      const where = f.pointer ? ` (${f.pointer}${f.citation ? ', ' + f.citation : ''})` : '';
      process.stderr.write(`FAIL [bench-name]: ${loc}${where} matched "${f.match}" (${f.jurist})\n`);
      if (f.context) process.stderr.write(`  ${f.context}\n`);
    } else {
      process.stderr.write(`FAIL [${f.type}]: ${f.path || '.'}${f.message ? ': ' + f.message : ''}\n`);
    }
  }
  process.stderr.write(`\n${res.findings.length} problem(s). Replace real jurist labels with invented VJS bench names.\n`);
  process.exit(1);
}

function appendMarkedInstruction(target, src, mark, closeMark, label) {
  if (!fs.existsSync(src)) { process.stderr.write(`skip ${label}: source not found in package\n`); return; }
  const existing = fs.existsSync(target) ? fs.readFileSync(target, 'utf8') : '';
  if (existing.includes(mark)) {
    process.stdout.write(`${label} already present, left as-is\n`);
    return;
  }
  const block = `\n${mark}\n` + fs.readFileSync(src, 'utf8') + `\n${closeMark}\n`;
  fs.writeFileSync(target, existing + block);
  process.stdout.write(`appended ${label}\n`);
}

function copyHookScripts(srcHooks, dstHooks, label, target) {
  fs.mkdirSync(dstHooks, { recursive: true });
  for (const f of fs.readdirSync(srcHooks)) {
    if (!f.endsWith('.sh')) continue;
    const dst = path.join(dstHooks, f);
    fs.copyFileSync(path.join(srcHooks, f), dst);
    try { fs.chmodSync(dst, 0o755); } catch (_) {}
    process.stdout.write(`installed ${label} hook ${path.relative(target, dst)}\n`);
  }
}

// Install the VJS hooks into a target repo: portable hook scripts, adapter wiring, and deterministic
// git hard gates. Claude remains the bundled adapter, not the contract itself. Idempotent.
function installHooks(target) {
  const srcHooks = path.join(REPO_ROOT, 'Executive', 'plugin', 'hooks');
  if (!fs.existsSync(srcHooks)) { process.stderr.write('skip hooks: plugin/hooks not found in package\n'); return; }
  const genericHooks = path.join(target, '.vjs', 'hooks');
  const claudeHooks = path.join(target, '.claude', 'hooks');
  copyHookScripts(srcHooks, genericHooks, 'generic VJS', target);
  copyHookScripts(srcHooks, claudeHooks, 'Claude adapter', target);

  // Merge the Claude adapter hook wiring into .claude/settings.json, idempotently.
  const settingsSrc = path.join(REPO_ROOT, 'Executive', 'plugin', 'settings.json');
  if (fs.existsSync(settingsSrc)) {
    let incoming = {};
    try { incoming = JSON.parse(fs.readFileSync(settingsSrc, 'utf8')); } catch (_) { incoming = {}; }
    const dstSettings = path.join(target, '.claude', 'settings.json');
    let cur = {};
    if (fs.existsSync(dstSettings)) { try { cur = JSON.parse(fs.readFileSync(dstSettings, 'utf8')); } catch (_) { cur = {}; } }
    cur.hooks = cur.hooks || {};
    let added = false;
    const hookKey = (hook) => hook && hook.command ? `command:${hook.command}` : JSON.stringify(hook);
    const collectKeys = (entries) => {
      const keys = new Set();
      for (const entry of entries || []) {
        for (const hook of entry.hooks || []) keys.add(hookKey(hook));
      }
      return keys;
    };
    for (const event of Object.keys(incoming.hooks || {})) {
      const arr = Array.isArray(cur.hooks[event]) ? cur.hooks[event] : (cur.hooks[event] = []);
      const existing = collectKeys(arr);
      for (const entry of incoming.hooks[event] || []) {
        const hooks = (entry.hooks || []).filter((hook) => !existing.has(hookKey(hook)));
        if (!hooks.length) continue;
        arr.push({ ...entry, hooks });
        for (const hook of hooks) existing.add(hookKey(hook));
        added = true;
      }
    }
    fs.writeFileSync(dstSettings, JSON.stringify(cur, null, 2) + '\n');
    process.stdout.write(added ? 'merged Claude adapter hooks into .claude/settings.json\n' : '.claude/settings.json already has VJS hooks, left as-is\n');
  }
  // Lay down the deterministic git hard gates.
  const gitDir = path.join(target, '.git');
  if (fs.existsSync(gitDir)) {
    const ghooks = path.join(gitDir, 'hooks');
    fs.mkdirSync(ghooks, { recursive: true });
    for (const [hookName, scriptName, label] of [
      ['pre-commit', 'vjs-pre-commit.sh', 'pre-commit hard gate'],
      ['pre-push', 'vjs-pre-push.sh', 'pre-push checkpoint gate'],
    ]) {
      const src = path.join(genericHooks, scriptName);
      if (!fs.existsSync(src)) continue;
      const dst = path.join(ghooks, hookName);
      const rel = path.relative(ghooks, src);
      if (fs.existsSync(dst)) {
        let installed = false;
        try { installed = fs.lstatSync(dst).isSymbolicLink() && fs.readlinkSync(dst) === rel; } catch (_) {}
        if (installed) process.stdout.write(`git ${label} already installed\n`);
        else process.stdout.write(`note: .git/hooks/${hookName} exists; chain it to .vjs/hooks/${scriptName} manually\n`);
      } else {
        try { fs.symlinkSync(rel, dst); } catch (_) { fs.copyFileSync(src, dst); }
        try { fs.chmodSync(dst, 0o755); } catch (_) {}
        process.stdout.write(`installed git ${label}\n`);
      }
    }
  }
}

function cmdInit(args) {
  const target = path.resolve(args._[0] || process.cwd());
  const copy = [
    ['Constitution/CASE-LAW.md', 'CASE-LAW.md'],
    ['Constitution/VPR.md', 'VPR.md'],
    ['Constitution/CDD.md', 'CDD.md'],
  ];
  for (const [srcRel, dstRel] of copy) {
    const src = path.join(REPO_ROOT, srcRel);
    if (!fs.existsSync(src)) { process.stderr.write(`skip (missing in package): ${srcRel}\n`); continue; }
    fs.copyFileSync(src, path.join(target, dstRel));
    process.stdout.write(`vendored ${dstRel}\n`);
  }
  // .justice scaffold. A downloaded repo starts as a local jurisdiction subscribed to the
  // canonical VJS law, with its own local citator and judgment store.
  const jdir = path.join(target, '.justice');
  for (const d of [
    '',
    'caselaw',
    'pdfs',
    'suites',
    'judgments',
    path.join('judgments', 'privy-council'),
    path.join('judgments', 'court-of-appeal'),
    path.join('judgments', 'supreme-court'),
  ]) fs.mkdirSync(path.join(jdir, d), { recursive: true });
  const suitesSrc = path.join(REPO_ROOT, 'Judicature', '.justice', 'suites');
  if (fs.existsSync(suitesSrc)) {
    for (const f of fs.readdirSync(suitesSrc)) {
      if (!f.endsWith('.md')) continue;
      fs.copyFileSync(path.join(suitesSrc, f), path.join(jdir, 'suites', f));
      process.stdout.write(`vendored .justice/suites/${f}\n`);
    }
  }
  const indexPath = path.join(jdir, 'INDEX.md');
  if (!fs.existsSync(indexPath)) {
    // Seed an EMPTY citator template - a fresh jurisdiction starts with no rulings of its own.
    fs.copyFileSync(path.join(CLI_ROOT, 'templates', 'INDEX.md'), indexPath);
    process.stdout.write('created .justice/INDEX.md (empty citator)\n');
  } else process.stdout.write('.justice/INDEX.md already present, left as-is\n');
  // Append the portable agent contract and the Claude adapter instruction block, idempotently.
  appendMarkedInstruction(
    path.join(target, 'AGENTS.md'),
    path.join(REPO_ROOT, 'Executive', 'plugin', 'AGENTS.md'),
    '<!-- vjs:agent-contract -->',
    '<!-- /vjs:agent-contract -->',
    'the VJS agent contract to AGENTS.md'
  );
  appendMarkedInstruction(
    path.join(target, 'CLAUDE.md'),
    path.join(REPO_ROOT, 'Executive', 'plugin', 'CLAUDE.md'),
    '<!-- vjs:plugin -->',
    '<!-- /vjs:plugin -->',
    'the VJS Claude adapter block to CLAUDE.md'
  );
  // Install portable hooks, the bundled Claude adapter, and deterministic git hard gates.
  installHooks(target);
  process.stdout.write('\nVJS installed. The court is in session.\n');
}

// Walk up from a starting dir to the repo root (central Judicature/.justice or local .justice).
function findRepoRoot(start) {
  let dir = start;
  for (let i = 0; i < 64; i++) {
    if (fs.existsSync(path.join(dir, 'Judicature', '.justice'))) return dir;
    if (fs.existsSync(path.join(dir, '.justice'))) return dir;
    const up = path.dirname(dir);
    if (up === dir) break;
    dir = up;
  }
  return null;
}

// cdd lodge-judgment - the first-class deterministic render-and-lodge verb mandated by
// [2026] REALM-SI 2 (the Judgment Rendering and Lodgement Instrument). On delivery of a judgment it
// (1) renders every judgment to PDF idempotently (fail-OPEN convenience layer), (2) lodges the
// judgment by rebuilding the derived projections in lockstep - corpus, search index, rulings ledger
// (fail-OPEN), and (3) verifies the citation layer with the fail-CLOSED citator audit (s.19(5)).
// Flags: --check-only (verify only, no render/lodge); --no-render (lodge + verify, skip the PDF).
function cmdLodgeJudgment(args) {
  const { spawnSync } = require('child_process');
  const root = findRepoRoot(process.cwd());
  if (!root) die('lodge-judgment: no .justice found above ' + process.cwd());
  if (!fs.existsSync(path.join(root, 'Judicature', '.justice'))) {
    const res = auditCitator(root);
    if (res.ok) { process.stdout.write('lodge-judgment: local citation layer OK (fail-closed verify passed).\n'); return; }
    for (const p of res.problems) process.stderr.write('FAIL [' + p.type + ']: ' + p.message + '\n');
    die('\nlodge-judgment: the local citation layer failed (fail-closed).');
  }
  const run = (cmd, argv) => spawnSync(cmd, argv, { cwd: root, stdio: 'pipe', encoding: 'utf8' });
  const have = (cmd) => spawnSync(cmd, ['--version'], { stdio: 'ignore' }).status === 0;
  const checkOnly = !!args['check-only'];
  const noRender = !!args['no-render'];

  if (!checkOnly) {
    // (1) RENDER (fail-open): idempotent render of every judgment to PDF.
    const renderer = path.join(root, 'Judicature/court/scripts/render-all-judgments.sh');
    if (!noRender && fs.existsSync(renderer) && have('node') && have('python3')) {
      const r = run('bash', [renderer]);
      if (r.status === 0) process.stdout.write('lodge-judgment: judgments rendered to PDF (idempotent).\n');
      else process.stderr.write('lodge-judgment: WARNING (fail-open) - judgment render failed; lodging without PDF refresh.\n');
    } else if (!noRender) {
      process.stderr.write('lodge-judgment: WARNING (fail-open) - renderer or node/python3 missing; PDF not refreshed.\n');
    }
    // (2) LODGE (fail-open): rebuild the derived projections in lockstep.
    const projections = [
      ['node', ['Judicature/law-reports/build/ingest.js'], 'law-site corpus'],
      ['node', ['Judicature/law-reports/build/build-search-index.js'], 'search index'],
      ['python3', ['Judicature/ministry-of-justice/ledger/build-ledger.py'], 'rulings ledger'],
    ];
    for (const [cmd, a, label] of projections) {
      if (!fs.existsSync(path.join(root, a[0]))) continue;
      const r = run(cmd, a);
      if (r.status === 0) process.stdout.write('lodge-judgment: ' + label + ' rebuilt in lockstep.\n');
      else process.stderr.write('lodge-judgment: WARNING (fail-open) - ' + label + ' rebuild failed.\n');
    }
  }
  // (3) VERIFY (fail-closed): the citation layer. Exit 1 on any citator inconsistency (s.19(5)).
  const res = auditCitator(root);
  if (res.ok) { process.stdout.write('lodge-judgment: citation layer OK (fail-closed verify passed).\n'); return; }
  for (const p of res.problems) process.stderr.write('FAIL [' + p.type + ']: ' + p.message + '\n');
  die('\nlodge-judgment: the citation layer failed (fail-closed). The judgment is not lodged; fix the citator.');
}

function workflowInvocation(script, kind, text) {
  const q = String(text || '').replace(/'/g, "\\'");
  return `Run the court through your agent's delegable workflow adapter.\n` +
    `Claude Code adapter example:\n\n` +
    `Workflow({\n  scriptPath: 'Judicature/court/workflows/${script}',\n  args: { kind: '${kind}', ${kind === 'breach' ? 'charge' : 'question'}: '${q}' }\n})\n`;
}

function main() {
  const argv = process.argv.slice(2);
  if (argv.includes('--version') || argv.includes('-v')) return process.stdout.write(VERSION + '\n');
  const cmd = argv[0];
  // tiny flag parser
  const args = { _: [] };
  for (let i = 1; i < argv.length; i++) {
    const a = argv[i];
    if (a === '--json') args.json = true;
    else if (a.startsWith('--')) {
      const nxt = argv[i + 1];
      if (nxt === undefined || nxt.startsWith('--')) args[a.slice(2)] = true; // value-less boolean flag
      else { args[a.slice(2)] = nxt; i++; }
    }
    else args._.push(a);
  }
  switch (cmd) {
    case 'next-citation': return cmdNextCitation(args);
    case 'check-citator': return cmdCheckCitator();
    case 'check-bench-names': return cmdCheckBenchNames(args);
    case 'lodge-judgment': return cmdLodgeJudgment(args);
    case 'init': return cmdInit(args);
    case 'submit-request': return process.stdout.write(workflowInvocation('first-instance.js', 'request_for_ruling', args._[0]));
    case 'submit-breach': return process.stdout.write(workflowInvocation('first-instance.js', 'breach', args._[0]));
    case undefined:
    case '--help':
    case '-h':
      return process.stdout.write(
`vjs / cdd - Vibe Justice System CLI (v${VERSION})

Commands:
  init [dir]                       Install VJS into a repo (vendor law, scaffold .justice/, inject AGENTS.md contract, install generic hooks plus Claude adapter)
  next-citation <tier> [--year Y]  Deterministic next neutral citation from the citator (.justice/INDEX.md). tier = privy-council|court-of-appeal|supreme-court|high-court|county-court|si. --json for full object.
  check-citator                    Deterministic citator audit (the hard gate): fails closed on citation collisions and on ruling-file/citator-row mismatches. Exit 1 on any problem.
  check-bench-names                Deterministic scan for prohibited real jurist labels in judgment records and law-report case corpus. Flags include --source-only, --corpus-only, --json.
  lodge-judgment [--check-only]    Render-and-lodge a judgment ([2026] REALM-SI 2): render PDFs (idempotent, fail-open), rebuild the corpus/index/ledger projections in lockstep (fail-open), and verify the citation layer (fail-closed). --no-render to skip the PDF.
  submit-request "<question>"      Print a delegable workflow invocation to file a Request for Ruling
  submit-breach "<charge>"         Print a delegable workflow invocation to file a Breach
  --version                        Print version

Spec is law. Rulings are precedent. Lexby is your lawyer.\n`);
    default: die(`unknown command: ${cmd}\nrun 'cdd --help'`);
  }
}

main();
