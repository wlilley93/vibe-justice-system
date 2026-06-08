'use strict';

const fs = require('fs');
const path = require('path');

const INDEX_OPTIONS = {
  fields: ['citation', 'title', 'ratio', 'body', 'court', 'status', 'panel', 'cites'],
  storeFields: [
    'kind', 'citation', 'title', 'series', 'court', 'status', 'ratio',
    'date', 'p_slug', 'p_source', 'p_pdf', 'p_court', 'p_jur', 'p_stage', 'p_no',
    'p_submission_kind', 'p_filed_by', 'p_route',
  ],
  searchOptions: { boost: { citation: 5, title: 4, ratio: 3 }, prefix: true, fuzzy: 0.2 },
};

const DEFAULT_AUTHORITY_RANK = {
  case: 4,
  si: 3,
  bill: 2,
  submission: 1,
};

function findLawSiteRoot(start) {
  let dir = path.resolve(start || process.cwd());
  for (let i = 0; i < 64; i++) {
    const site = path.join(dir, 'Judicature', 'law-reports', 'site');
    if (
      fs.existsSync(path.join(site, 'search-index.json')) &&
      fs.existsSync(path.join(site, 'citator-graph.json'))
    ) return dir;
    const up = path.dirname(dir);
    if (up === dir) break;
    dir = up;
  }
  return null;
}

function sitePath(root, file) {
  return path.join(root, 'Judicature', 'law-reports', 'site', file);
}

function readJson(file) {
  return JSON.parse(fs.readFileSync(file, 'utf8'));
}

function loadSearchIndex(root) {
  return readJson(sitePath(root, 'search-index.json'));
}

function loadGraph(root) {
  return readJson(sitePath(root, 'citator-graph.json'));
}

function loadMiniSearch(root) {
  const cjs = path.join(root, 'Judicature', 'law-reports', 'node_modules', 'minisearch', 'dist', 'cjs', 'index.cjs');
  if (!fs.existsSync(cjs)) return null;
  try {
    const MiniSearch = require(cjs);
    return typeof MiniSearch.loadJS === 'function' ? MiniSearch : null;
  } catch (_) {
    return null;
  }
}

function normalize(s) {
  return String(s || '').toLowerCase().replace(/\s+/g, ' ').trim();
}

function tokenize(query) {
  return normalize(query).split(/[^a-z0-9[\]-]+/).filter(Boolean);
}

function compactText(text, max = 360) {
  const s = String(text || '').replace(/\s+/g, ' ').trim();
  if (!s) return null;
  return s.length > max ? s.slice(0, max - 3) + '...' : s;
}

function externalIds(index) {
  const out = new Map();
  for (const [internalId, externalId] of Object.entries(index.documentIds || {})) {
    out.set(String(internalId), externalId);
  }
  return out;
}

function pointerRecord(id, fields, score, match) {
  return {
    id,
    kind: fields.kind || null,
    citation: fields.citation || id,
    title: fields.title || fields.citation || id,
    series: fields.series || null,
    court: fields.court || null,
    status: fields.status || null,
    date: fields.date || null,
    summary: compactText(fields.ratio),
    sourcePath: fields.p_source || null,
    pdfPath: fields.p_pdf || null,
    slug: fields.p_slug || null,
    score: score === undefined ? undefined : score,
    matchedTerms: match ? Object.keys(match).sort() : undefined,
    note: 'retrieval aid only; not legal force',
  };
}

function allPointerRecords(index) {
  const ids = externalIds(index);
  return Object.entries(index.storedFields || {}).map(([internalId, fields]) => {
    return pointerRecord(ids.get(String(internalId)) || fields.citation || internalId, fields);
  });
}

function applyFilters(records, opts = {}) {
  return records.filter((record) => {
    if (opts.kind && opts.kind !== 'all' && record.kind !== opts.kind) return false;
    if (opts.court && normalize(record.court) !== normalize(opts.court)) return false;
    if (opts.status && normalize(record.status) !== normalize(opts.status)) return false;
    return true;
  });
}

function authorityRank(record, opts = {}) {
  if (opts.kind && opts.kind !== 'all') return 0;
  return DEFAULT_AUTHORITY_RANK[record.kind] || 0;
}

function sortSearchResults(records, opts = {}) {
  return records.sort((a, b) =>
    authorityRank(b, opts) - authorityRank(a, opts)
    || (b.score || 0) - (a.score || 0)
    || String(a.id).localeCompare(String(b.id)));
}

function fallbackSearch(index, query, opts = {}) {
  const terms = tokenize(query);
  const raw = normalize(query);
  return applyFilters(allPointerRecords(index), opts)
    .map((record) => {
      const id = normalize(record.id);
      const citation = normalize(record.citation);
      const title = normalize(record.title);
      const summary = normalize(record.summary);
      const court = normalize(record.court);
      let score = 0;
      if (id === raw || citation === raw) score += 1000;
      if (id.includes(raw) || citation.includes(raw)) score += 250;
      if (title.includes(raw)) score += 160;
      if (summary.includes(raw)) score += 50;
      for (const term of terms) {
        if (id.includes(term) || citation.includes(term)) score += 80;
        if (title.includes(term)) score += 30;
        if (summary.includes(term)) score += 10;
        if (court.includes(term)) score += 5;
      }
      return { ...record, score };
    })
    .filter((record) => record.score > 0)
    .sort((a, b) => b.score - a.score || String(a.id).localeCompare(String(b.id)));
}

function searchLaw(root, query, opts = {}) {
  if (!tokenize(query).length) return [];
  const index = loadSearchIndex(root);
  const limit = Math.max(1, Number(opts.limit || 10));
  const MiniSearch = loadMiniSearch(root);
  let records;
  if (MiniSearch) {
    const ms = MiniSearch.loadJS(index, INDEX_OPTIONS);
    records = ms.search(query, INDEX_OPTIONS.searchOptions).map((result) => {
      return pointerRecord(result.id, result, result.score, result.match);
    });
  } else {
    records = fallbackSearch(index, query, opts);
  }
  return sortSearchResults(applyFilters(records, opts), opts).slice(0, limit);
}

function citationAliases(needle) {
  const raw = String(needle || '').trim();
  const key = normalize(raw);
  const aliases = new Set([key]);
  const si = raw.match(/(?:si:)?(?:\[2026\]\s*)?REALM-SI\s+(\d+)/i) || raw.match(/^si:(\d+)$/i);
  if (si) {
    aliases.add(`si:${si[1]}`);
    aliases.add(`[2026] realm-si ${si[1]}`);
    aliases.add(`si:[2026] realm-si ${si[1]}`);
  }
  const shortSi = raw.match(/^si:(\d+)$/i);
  if (shortSi) {
    aliases.add(`[2026] realm-si ${shortSi[1]}`);
    aliases.add(`si:[2026] realm-si ${shortSi[1]}`);
  }
  const bill = raw.match(/(?:bill:|bill\s+)(\d+)/i);
  if (bill) aliases.add(`bill:${bill[1]}`);
  return aliases;
}

function getLawRecord(root, needle, opts = {}) {
  const index = loadSearchIndex(root);
  const aliases = citationAliases(needle);
  const records = allPointerRecords(index);
  let record = records.find((candidate) => {
    return [candidate.id, candidate.citation, candidate.title].filter(Boolean)
      .some((value) => aliases.has(normalize(value)));
  }) || null;
  if (!record) {
    record = fallbackSearch(index, needle, { ...opts, limit: 1 })[0] || null;
  }
  if (!record) return null;
  const out = { ...record };
  if (opts.includeSource) {
    if (!out.sourcePath) throw new Error('record has no sourcePath');
    const source = path.join(root, out.sourcePath);
    if (!fs.existsSync(source)) throw new Error(`source file not found: ${out.sourcePath}`);
    const maxChars = Math.max(1, Number(opts.maxChars || 4000));
    const text = fs.readFileSync(source, 'utf8');
    out.source = {
      path: out.sourcePath,
      truncated: text.length > maxChars,
      text: text.slice(0, maxChars),
    };
  }
  return out;
}

function graphNodeKey(s) {
  return normalize(s);
}

function resolveGraphNode(graph, needle) {
  const aliases = citationAliases(needle);
  const key = graphNodeKey(needle);
  return (graph.nodes || []).find((node) => {
    return [node.id, node.citation, node.label, node.title]
      .filter(Boolean)
      .some((value) => aliases.has(graphNodeKey(value)) || graphNodeKey(value) === key);
  }) || (graph.nodes || []).find((node) => {
    return [node.id, node.citation, node.label, node.title]
      .filter(Boolean)
      .some((value) => graphNodeKey(value).includes(key));
  }) || null;
}

function compactNode(node) {
  if (!node) return null;
  return {
    id: node.id,
    kind: node.kind,
    citation: node.citation || null,
    label: node.label || node.title || node.id,
    title: node.title || null,
    status: node.status || null,
    date: node.date || null,
    sourcePath: node.sourcePath || null,
    pdfPath: node.pdfPath || null,
  };
}

function graphNode(root, needle) {
  const graph = loadGraph(root);
  const node = resolveGraphNode(graph, needle);
  if (!node) return null;
  return {
    node: compactNode(node),
    counts: {
      incoming: (graph.edges || []).filter((edge) => edge.target === node.id).length,
      outgoing: (graph.edges || []).filter((edge) => edge.source === node.id).length,
    },
    graph: {
      validation: graph.validation || null,
      boundary: graph.boundary || null,
    },
    note: 'graph node is a retrieval aid only; not legal force',
  };
}

function graphEdges(root, needle, opts = {}) {
  const graph = loadGraph(root);
  const node = resolveGraphNode(graph, needle);
  if (!node) return null;
  const dir = opts.dir || 'both';
  const type = opts.type || null;
  const limit = Math.max(1, Number(opts.limit || 20));
  const nodeById = new Map((graph.nodes || []).map((n) => [n.id, n]));
  const edges = (graph.edges || []).filter((edge) => {
    if (dir === 'in' || dir === 'incoming') {
      if (edge.target !== node.id) return false;
    } else if (dir === 'out' || dir === 'outgoing') {
      if (edge.source !== node.id) return false;
    } else if (edge.source !== node.id && edge.target !== node.id) {
      return false;
    }
    return !type || edge.type === type;
  }).slice(0, limit).map((edge) => ({
    id: edge.id,
    source: compactNode(nodeById.get(edge.source)),
    target: compactNode(nodeById.get(edge.target)),
    type: edge.type,
    label: edge.label || edge.type,
    pinpoint: edge.pinpoint || null,
    briefWhy: edge.briefWhy || null,
    edgeSource: edge.edgeSource || null,
    edgeStatus: edge.edgeStatus || null,
    evidenceToken: edge.evidenceToken || null,
    sourcePath: edge.sourcePath || null,
    targetPath: edge.targetPath || null,
  }));
  return {
    node: compactNode(node),
    dir,
    type,
    limit,
    edges,
    note: 'graph edges are retrieval evidence only; not legal force',
  };
}

module.exports = {
  findLawSiteRoot,
  loadSearchIndex,
  loadGraph,
  searchLaw,
  getLawRecord,
  graphNode,
  graphEdges,
};
