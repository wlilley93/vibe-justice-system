#!/usr/bin/env node
'use strict';
// Vibe Justice System CLI. Commands: init, next-citation, submit-request, submit-breach.
// No runtime dependencies (pure Node).

const fs = require('fs');
const path = require('path');
const { nextCitation, seriesCode } = require('../lib/citation');
const { auditCitator } = require('../lib/citator-audit');

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

// Install the VJS hooks into a target repo: the watchdog Stop hook, the settings wiring, and the
// deterministic pre-commit hard gate. Idempotent.
function installHooks(target) {
  const srcHooks = path.join(REPO_ROOT, 'Executive', 'plugin', 'hooks');
  if (!fs.existsSync(srcHooks)) { process.stderr.write('skip hooks: plugin/hooks not found in package\n'); return; }
  const dstHooks = path.join(target, '.claude', 'hooks');
  fs.mkdirSync(dstHooks, { recursive: true });
  for (const f of fs.readdirSync(srcHooks)) {
    if (!f.endsWith('.sh')) continue;
    const dst = path.join(dstHooks, f);
    fs.copyFileSync(path.join(srcHooks, f), dst);
    try { fs.chmodSync(dst, 0o755); } catch (_) {}
    process.stdout.write('installed hook .claude/hooks/' + f + '\n');
  }
  // Merge the Stop-hook wiring into .claude/settings.json, idempotently.
  const settingsSrc = path.join(REPO_ROOT, 'Executive', 'plugin', 'settings.json');
  if (fs.existsSync(settingsSrc)) {
    let incoming = {};
    try { incoming = JSON.parse(fs.readFileSync(settingsSrc, 'utf8')); } catch (_) { incoming = {}; }
    const dstSettings = path.join(target, '.claude', 'settings.json');
    let cur = {};
    if (fs.existsSync(dstSettings)) { try { cur = JSON.parse(fs.readFileSync(dstSettings, 'utf8')); } catch (_) { cur = {}; } }
    cur.hooks = cur.hooks || {};
    let added = false;
    for (const event of Object.keys(incoming.hooks || {})) {
      const arr = Array.isArray(cur.hooks[event]) ? cur.hooks[event] : (cur.hooks[event] = []);
      if (!JSON.stringify(arr).includes('vjs-watchdog')) { arr.push(...incoming.hooks[event]); added = true; }
    }
    fs.writeFileSync(dstSettings, JSON.stringify(cur, null, 2) + '\n');
    process.stdout.write(added ? 'merged VJS hooks into .claude/settings.json\n' : '.claude/settings.json already has VJS hooks, left as-is\n');
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
      const src = path.join(dstHooks, scriptName);
      if (!fs.existsSync(src)) continue;
      const dst = path.join(ghooks, hookName);
      if (fs.existsSync(dst)) {
        process.stdout.write(`note: .git/hooks/${hookName} exists; chain it to .claude/hooks/${scriptName} manually\n`);
      } else {
        const rel = path.relative(ghooks, src);
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
  // Append the binding plugin block to CLAUDE.md, idempotently.
  const MARK = '<!-- vjs:plugin -->';
  const block = `\n${MARK}\n` + fs.readFileSync(path.join(REPO_ROOT, 'Executive', 'plugin', 'CLAUDE.md'), 'utf8') + `\n<!-- /vjs:plugin -->\n`;
  const claudePath = path.join(target, 'CLAUDE.md');
  const existing = fs.existsSync(claudePath) ? fs.readFileSync(claudePath, 'utf8') : '';
  if (existing.includes(MARK)) process.stdout.write('CLAUDE.md already has the VJS plugin block, left as-is\n');
  else { fs.writeFileSync(claudePath, existing + block); process.stdout.write('appended the VJS plugin block to CLAUDE.md\n'); }
  // Install the watchdog Stop hook + the deterministic pre-commit hard gate.
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
  return `Run the court in Claude Code via the Workflow tool:\n\n` +
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
  init [dir]                       Install VJS into a repo (vendor CASE-LAW/VPR/CDD, scaffold .justice/, inject plugin block into CLAUDE.md)
  next-citation <tier> [--year Y]  Deterministic next neutral citation from the citator (.justice/INDEX.md). tier = privy-council|court-of-appeal|supreme-court|high-court|county-court|si. --json for full object.
  check-citator                    Deterministic citator audit (the hard gate): fails closed on citation collisions and on ruling-file/citator-row mismatches. Exit 1 on any problem.
  lodge-judgment [--check-only]    Render-and-lodge a judgment ([2026] REALM-SI 2): render PDFs (idempotent, fail-open), rebuild the corpus/index/ledger projections in lockstep (fail-open), and verify the citation layer (fail-closed). --no-render to skip the PDF.
  submit-request "<question>"      Print the Workflow invocation to file a Request for Ruling
  submit-breach "<charge>"         Print the Workflow invocation to file a Breach
  --version                        Print version

Spec is law. Rulings are precedent. Lexby is your lawyer.\n`);
    default: die(`unknown command: ${cmd}\nrun 'cdd --help'`);
  }
}

main();
