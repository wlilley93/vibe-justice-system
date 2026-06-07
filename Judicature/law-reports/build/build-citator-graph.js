'use strict';
// Build a deterministic, pointer-only Gazette lineage graph from the public corpus/citator data.
// The graph is a derived orientation aid only: nodes point to public Gazette items, and edges are
// deterministic citation/reference links. It does not create, amend, overrule, or store law.
//
//   node build/build-citator-graph.js   ->   law-reports/site/citator-graph.json

const fs = require('fs');
const path = require('path');
const { ROOT } = require('./corpus');

const corpusPath = path.join(ROOT, 'Judicature', 'law-reports', 'corpus.json');
const corpus = JSON.parse(fs.readFileSync(corpusPath, 'utf8'));

const CASE_RE = /\[(\d{4})\]\s+(REALM-(?:SC|PC|CA))\s+(\d+)/g;
const SI_RE = /\[(\d{4})\]\s+REALM-SI\s+(\d+)/g;
const BARE_CASE_RE = /(?<!\]\s)\b(REALM-(?:SC|PC|CA))\s+(\d+)\b/g;
const BARE_SI_RE = /(?<!\]\s)\bREALM-SI\s+(\d+)\b/g;
const BILL_RE = /\bBill\s+(\d{1,3})\b/g;

const RELATION_LABELS = {
  affirms: 'affirms',
  'affirmed-by': 'affirmed by',
  applies: 'applies',
  clarifies: 'clarifies',
  confirms: 'confirms',
  'consistent-with': 'consistent with',
  cites: 'cites',
  directs: 'directs',
  distinguishes: 'distinguishes',
  extends: 'extends',
  references: 'references',
  'referred-to': 'referred to',
  'superseded-by': 'superseded by',
  supersedes: 'supersedes',
  amends: 'amends',
  commences: 'commences',
  'depends-on': 'depends on',
  implements: 'implements',
  repeals: 'repeals',
};

const GRAPH_REPORT_REL = path.join('Judicature', 'law-reports', 'site', 'citator-graph-validation.json');
const ALLOWED_EDGE_TYPES = new Set(Object.keys(RELATION_LABELS));
const ALLOWED_EDGE_SOURCES = new Set(['backfilled-derived']);
const ALLOWED_EDGE_STATUSES = new Set(['derived']);
const ALLOWED_SOURCE_FIELDS = new Set(['cites', 'publicText']);
const FORBIDDEN_EDGE_KEYS = new Set([
  'body',
  'holding',
  'lawText',
  'legalForce',
  'ratio',
  'remedy',
  'searchBody',
  'statutoryText',
  'text',
]);

function briefWhy(type, token, field) {
  const label = RELATION_LABELS[type] || type;
  const basis = field === 'cites' ? 'the public Cites field' : 'public Act/SI text';
  return `Derived because ${basis} contains ${token} in language classified as "${label}".`;
}

function nodeId(record) {
  if (record.type === 'case') return `case:${record.citation}`;
  if (record.type === 'bill') return `bill:${record.no}`;
  if (record.type === 'instrument') return `si:${record.citation}`;
  return '';
}

function nodeLabel(record) {
  if (record.type === 'case') return record.citation;
  if (record.type === 'bill') return `Bill ${record.no}: ${record.shortTitle}`;
  if (record.type === 'instrument') return `${record.citation}: ${record.shortTitle}`;
  return '';
}

function nodeDate(record) {
  return record.date || record.royalAssent || record.made || '';
}

function normalizeCitation(citation) {
  return String(citation || '').replace(/[[\]]/g, '').replace(/\s+/g, ' ').trim().toUpperCase();
}

function sourceYear(record) {
  const d = nodeDate(record);
  const m = String(d).match(/^(\d{4})/);
  return m ? m[1] : '2026';
}

function textWindow(text, index, length) {
  const s = String(text || '');
  const start = Math.max(0, index - 90);
  const end = Math.min(s.length, index + length + 90);
  return s.slice(start, end).replace(/\s+/g, ' ').trim();
}

function inferCaseRelation(context) {
  const c = context.toLowerCase();
  if (/\bsuperseded by\b/.test(c)) return 'superseded-by';
  if (/\baffirmed by\b/.test(c)) return 'affirmed-by';
  if (/\breferred to\b/.test(c)) return 'referred-to';
  if (/\bapplies?\b/.test(c)) return 'applies';
  if (/\baffirms?\b/.test(c)) return 'affirms';
  if (/\bclarifies?\b/.test(c)) return 'clarifies';
  if (/\bconfirms?\b/.test(c)) return 'confirms';
  if (/\bdistinguishes?\b/.test(c)) return 'distinguishes';
  if (/\bextends?\b/.test(c)) return 'extends';
  if (/\bsupersedes?\b/.test(c)) return 'supersedes';
  if (/\bconsistent with\b/.test(c)) return 'consistent-with';
  if (/\bdirects?\b/.test(c)) return 'directs';
  if (/\breferences?\b/.test(c)) return 'references';
  return 'cites';
}

function inferGazetteRelation(context) {
  const c = context.toLowerCase();
  if (/\b(amend|amends|amended|amending|insert|inserted|substitut|omit|omitted)\b/.test(c)) return 'amends';
  if (/\b(repeal|repeals|repealed|revoke|revokes|revoked)\b/.test(c)) return 'repeals';
  if (/\b(commence|commences|commencement|coming into force|appointed day)\b/.test(c)) return 'commences';
  if (/\b(implement|implements|implemented|operationalise|operationalises|operationalizes|give effect|gives effect|codif|codify)\b/.test(c)) return 'implements';
  if (/\b(conferred|power|powers|under|read with|authority|parent authority|exercise of|pursuant to)\b/.test(c)) return 'depends-on';
  return 'cites';
}

function addRef(refs, ref) {
  const key = `${ref.kind}:${ref.key}:${ref.index}`;
  if (!refs.has(key)) refs.set(key, ref);
}

function findReferences(text, source) {
  const refs = new Map();
  const year = sourceYear(source);
  const s = String(text || '');
  for (const m of s.matchAll(CASE_RE)) {
    addRef(refs, { kind: 'case', key: normalizeCitation(`[${m[1]}] ${m[2]} ${m[3]}`), token: m[0], index: m.index, length: m[0].length });
  }
  for (const m of s.matchAll(SI_RE)) {
    addRef(refs, { kind: 'si', key: normalizeCitation(`[${m[1]}] REALM-SI ${m[2]}`), token: m[0], index: m.index, length: m[0].length });
  }
  for (const m of s.matchAll(BARE_CASE_RE)) {
    addRef(refs, { kind: 'case', key: normalizeCitation(`[${year}] ${m[1]} ${m[2]}`), token: m[0], index: m.index, length: m[0].length });
  }
  for (const m of s.matchAll(BARE_SI_RE)) {
    addRef(refs, { kind: 'si', key: normalizeCitation(`[${year}] REALM-SI ${m[1]}`), token: m[0], index: m.index, length: m[0].length });
  }
  for (const m of s.matchAll(BILL_RE)) {
    addRef(refs, { kind: 'bill', key: String(parseInt(m[1], 10)), token: m[0], index: m.index, length: m[0].length });
  }
  return [...refs.values()].sort((a, b) => a.index - b.index || a.token.localeCompare(b.token));
}

function publicNode(record) {
  return {
    id: nodeId(record),
    kind: record.type === 'instrument' ? 'si' : record.type,
    label: nodeLabel(record),
    citation: record.citation || `Bill ${record.no}`,
    title: record.shortTitle || record.citation,
    status: record.status || '',
    date: nodeDate(record),
    sourcePath: record.sourcePath,
    pdfPath: record.pdfPath || null,
  };
}

function readSource(record) {
  if (!record.sourcePath) return '';
  try {
    return fs.readFileSync(path.join(ROOT, record.sourcePath), 'utf8');
  } catch {
    return '';
  }
}

function noEdgeDeclaration(record) {
  const raw = readSource(record);
  if (!raw) return null;
  const declarations = [
    ['html-comment', /<!--\s*gazette-graph\s*:\s*no-edge(?:\s+declaration)?(?:\s*:[\s\S]*?)?\s*-->/i],
    ['markdown-field', /^\s*(?:\*\*)?Gazette graph no-edge declaration(?:\*\*)?\s*:\s*\S.+$/im],
    ['markdown-field', /^\s*(?:\*\*)?No graph edges(?:\*\*)?\s*:\s*\S.+$/im],
  ];
  const found = declarations.find(([, re]) => re.test(raw));
  if (!found) return null;
  return {
    nodeId: nodeId(record),
    kind: record.type === 'instrument' ? 'si' : record.type,
    label: nodeLabel(record),
    sourcePath: record.sourcePath,
    declarationSource: found[0],
  };
}

function summarizeNode(node, degree) {
  return {
    id: node.id,
    kind: node.kind,
    label: node.label,
    citation: node.citation,
    sourcePath: node.sourcePath,
    pdfPath: node.pdfPath,
    degree,
  };
}

function buildValidationReport(graph, sourceRecords) {
  const nodeMap = new Map();
  const edgeIds = new Set();
  const errors = [];
  const degree = new Map();

  if (!graph || typeof graph !== 'object') {
    errors.push('Graph root must be an object.');
  }
  if (!Array.isArray(graph.nodes)) errors.push('Graph nodes must be an array.');
  if (!Array.isArray(graph.edges)) errors.push('Graph edges must be an array.');

  for (const node of graph.nodes || []) {
    if (!node || typeof node !== 'object') {
      errors.push('Node entry must be an object.');
      continue;
    }
    if (!node.id) errors.push('Node is missing id.');
    if (node.id && nodeMap.has(node.id)) errors.push(`Duplicate node id ${node.id}.`);
    if (node.id) {
      nodeMap.set(node.id, node);
      degree.set(node.id, 0);
    }
    if (!node.kind || !['case', 'bill', 'si'].includes(node.kind)) errors.push(`Node ${node.id || '(missing id)'} has invalid kind.`);
    if (!node.label) errors.push(`Node ${node.id || '(missing id)'} is missing label.`);
    if (!node.sourcePath) errors.push(`Node ${node.id || '(missing id)'} is missing sourcePath.`);
  }

  for (const edge of graph.edges || []) {
    if (!edge || typeof edge !== 'object') {
      errors.push('Edge entry must be an object.');
      continue;
    }
    const id = edge.id || '(missing id)';
    if (!edge.id) errors.push('Edge is missing id.');
    if (edge.id && !/^edge:\d{5}$/.test(edge.id)) errors.push(`Edge ${id} id must match edge:00001 format.`);
    if (edge.id && edgeIds.has(edge.id)) errors.push(`Duplicate edge id ${edge.id}.`);
    if (edge.id) edgeIds.add(edge.id);
    for (const field of ['source', 'target', 'type', 'label', 'direction', 'briefWhy', 'edgeSource', 'edgeStatus', 'sourceField', 'evidenceToken', 'sourcePath', 'targetPath']) {
      if (edge[field] === undefined || edge[field] === null || edge[field] === '') errors.push(`Edge ${id} is missing ${field}.`);
    }
    for (const key of Object.keys(edge)) {
      if (FORBIDDEN_EDGE_KEYS.has(key)) errors.push(`Edge ${id} contains non-pointer/non-lineage field ${key}.`);
    }
    if (edge.source && !nodeMap.has(edge.source)) errors.push(`Edge ${id} source ${edge.source} is not a graph node.`);
    if (edge.target && !nodeMap.has(edge.target)) errors.push(`Edge ${id} target ${edge.target} is not a graph node.`);
    if (edge.source && edge.target && edge.source === edge.target) errors.push(`Edge ${id} is a self-edge.`);
    if (edge.type && !ALLOWED_EDGE_TYPES.has(edge.type)) errors.push(`Edge ${id} has invalid relation type ${edge.type}.`);
    if (edge.type && edge.label !== RELATION_LABELS[edge.type]) errors.push(`Edge ${id} label does not match relation type ${edge.type}.`);
    if (edge.direction !== 'outgoing') errors.push(`Edge ${id} direction must be outgoing.`);
    if (edge.edgeSource && !ALLOWED_EDGE_SOURCES.has(edge.edgeSource)) errors.push(`Edge ${id} has invalid edgeSource ${edge.edgeSource}.`);
    if (edge.edgeStatus && !ALLOWED_EDGE_STATUSES.has(edge.edgeStatus)) errors.push(`Edge ${id} has invalid edgeStatus ${edge.edgeStatus}.`);
    if (edge.sourceField && !ALLOWED_SOURCE_FIELDS.has(edge.sourceField)) errors.push(`Edge ${id} has invalid sourceField ${edge.sourceField}.`);

    const source = nodeMap.get(edge.source);
    const target = nodeMap.get(edge.target);
    if (source && edge.sourcePath !== source.sourcePath) errors.push(`Edge ${id} sourcePath does not match source node.`);
    if (target && edge.targetPath !== target.sourcePath) errors.push(`Edge ${id} targetPath does not match target node.`);
    if (source && target && edge.source !== edge.target) {
      degree.set(edge.source, (degree.get(edge.source) || 0) + 1);
      degree.set(edge.target, (degree.get(edge.target) || 0) + 1);
    }
  }

  const noEdgeDeclarations = sourceRecords
    .map(noEdgeDeclaration)
    .filter(Boolean)
    .sort((a, b) => a.nodeId.localeCompare(b.nodeId));
  const declarationIds = new Set(noEdgeDeclarations.map(d => d.nodeId));
  const isolatedNodes = [...nodeMap.values()]
    .filter(n => (degree.get(n.id) || 0) === 0)
    .map(n => summarizeNode(n, degree.get(n.id) || 0))
    .sort((a, b) => a.id.localeCompare(b.id));
  const isolatedNodesWithoutNoEdgeDeclaration = isolatedNodes
    .filter(n => !declarationIds.has(n.id));
  const noEdgeDeclarationsWithEdges = noEdgeDeclarations
    .filter(d => (degree.get(d.nodeId) || 0) > 0)
    .map(d => ({ ...d, degree: degree.get(d.nodeId) || 0 }));

  return {
    schemaVersion: 1,
    title: 'Gazette Citator Graph Validation Report',
    note: 'Deterministic report for the derived, pointer-only Gazette graph. Malformed edge entries fail the build; isolated nodes and no-edge declarations are report-only for the current static projection.',
    boundary: graph.boundary,
    status: errors.length ? 'fail' : 'pass',
    malformedEdgesFailBuild: true,
    forwardFilingReportStatus: isolatedNodesWithoutNoEdgeDeclaration.length ? 'review-needed' : 'complete',
    counts: {
      nodes: nodeMap.size,
      edges: (graph.edges || []).length,
      malformedEdgeErrors: errors.length,
      isolatedNodes: isolatedNodes.length,
      noEdgeDeclarations: noEdgeDeclarations.length,
      isolatedNodesWithoutNoEdgeDeclaration: isolatedNodesWithoutNoEdgeDeclaration.length,
      noEdgeDeclarationsWithEdges: noEdgeDeclarationsWithEdges.length,
    },
    malformedEdgeErrors: errors,
    isolatedNodes,
    noEdgeDeclarations,
    isolatedNodesWithoutNoEdgeDeclaration,
    noEdgeDeclarationsWithEdges,
  };
}

function assertValidGraph(report) {
  if (report.status === 'pass') return;
  const shown = report.malformedEdgeErrors.slice(0, 20).map(e => ` - ${e}`).join('\n');
  const remaining = report.malformedEdgeErrors.length > 20 ? `\n - ... ${report.malformedEdgeErrors.length - 20} more` : '';
  throw new Error(`Malformed Gazette citator graph:\n${shown}${remaining}`);
}

const records = [
  ...(corpus.cases || []),
  ...(corpus.legislation || []),
  ...(corpus.instruments || []),
];

const nodes = records.map(publicNode).sort((a, b) => a.id.localeCompare(b.id));
const byNodeId = new Map(nodes.map(n => [n.id, n]));
const byCase = new Map((corpus.cases || []).map(c => [normalizeCitation(c.citation), nodeId(c)]));
const bySi = new Map((corpus.instruments || []).map(si => [normalizeCitation(si.citation), nodeId(si)]));
const byBill = new Map((corpus.legislation || []).map(b => [String(b.no), nodeId(b)]));
const edgeMap = new Map();
const omitted = new Map();

function targetIdFor(ref) {
  if (ref.kind === 'case') return byCase.get(ref.key);
  if (ref.kind === 'si') return bySi.get(ref.key);
  if (ref.kind === 'bill') return byBill.get(ref.key);
  return null;
}

function omit(ref) {
  const key = `${ref.kind}:${ref.key}`;
  omitted.set(key, (omitted.get(key) || 0) + 1);
}

function addEdge(source, targetId, type, field, token) {
  const sourceId = nodeId(source);
  if (!sourceId || !targetId || sourceId === targetId || !byNodeId.has(targetId)) return;
  const key = `${sourceId}|${targetId}|${type}|${field}`;
  if (edgeMap.has(key)) return;
  edgeMap.set(key, {
    source: sourceId,
    target: targetId,
    type,
    label: RELATION_LABELS[type] || type,
    direction: 'outgoing',
    pinpoint: null,
    briefWhy: briefWhy(type, token, field),
    edgeSource: 'backfilled-derived',
    edgeStatus: 'derived',
    sourceField: field,
    evidenceToken: token,
    sourcePath: byNodeId.get(sourceId).sourcePath,
    targetPath: byNodeId.get(targetId).sourcePath,
  });
}

for (const c of corpus.cases || []) {
  const field = 'cites';
  for (const ref of findReferences(c.cites || '', c)) {
    const targetId = targetIdFor(ref);
    if (!targetId) { omit(ref); continue; }
    const context = textWindow(c.cites || '', ref.index, ref.length);
    const relation = ref.kind === 'case' ? inferCaseRelation(context) : 'cites';
    addEdge(c, targetId, relation, field, ref.token);
  }
}

for (const record of [...(corpus.legislation || []), ...(corpus.instruments || [])]) {
  const field = 'publicText';
  const text = [record.longTitle, record.searchBody].filter(Boolean).join('\n');
  for (const ref of findReferences(text, record)) {
    const targetId = targetIdFor(ref);
    if (!targetId) { omit(ref); continue; }
    const context = textWindow(text, ref.index, ref.length);
    addEdge(record, targetId, inferGazetteRelation(context), field, ref.token);
  }
}

const edges = [...edgeMap.values()]
  .sort((a, b) => a.source.localeCompare(b.source) || a.target.localeCompare(b.target) || a.type.localeCompare(b.type))
  .map((edge, index) => ({ id: `edge:${String(index + 1).padStart(5, '0')}`, ...edge }));

const relationCounts = {};
for (const e of edges) relationCounts[e.type] = (relationCounts[e.type] || 0) + 1;

const out = {
  realm: corpus.realm,
  title: 'Gazette Citator Graph',
  note: 'Derived, pointer-only lineage projection from public Gazette corpus/citator data. Edges are deterministic public citation/reference links and are not themselves legal authority.',
  boundary: 'Public Gazette corpus only: central judgments, Acts, and statutory instruments already present in corpus.json. Local/private judgment trees are not scanned.',
  validation: {
    reportPath: GRAPH_REPORT_REL,
    malformedEdgesFailBuild: true,
    noEdgeDeclarationsArePointerOnly: true,
  },
  counts: {
    nodes: nodes.length,
    edges: edges.length,
    omittedReferences: [...omitted.values()].reduce((a, b) => a + b, 0),
    relations: relationCounts,
  },
  nodes,
  edges,
};

const validationReport = buildValidationReport(out, records);
assertValidGraph(validationReport);
out.validation.status = validationReport.status;
out.validation.forwardFilingReportStatus = validationReport.forwardFilingReportStatus;
out.validation.counts = validationReport.counts;

const dest = path.join(ROOT, 'Judicature', 'law-reports', 'site', 'citator-graph.json');
fs.mkdirSync(path.dirname(dest), { recursive: true });
fs.writeFileSync(dest, JSON.stringify(out, null, 2) + '\n');
const reportDest = path.join(ROOT, GRAPH_REPORT_REL);
fs.writeFileSync(reportDest, JSON.stringify(validationReport, null, 2) + '\n');
console.log(`citator-graph.json: ${nodes.length} nodes + ${edges.length} edges -> ${path.relative(ROOT, dest)}`);
console.log(`citator-graph-validation.json: ${validationReport.counts.isolatedNodesWithoutNoEdgeDeclaration} isolated without no-edge declaration -> ${path.relative(ROOT, reportDest)}`);
