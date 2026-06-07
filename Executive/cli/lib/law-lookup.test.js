'use strict';

const assert = require('assert');
const path = require('path');
const {
  findLawSiteRoot,
  loadSearchIndex,
  searchLaw,
  graphNode,
  graphEdges,
  getLawRecord,
} = require('./law-lookup');

const root = path.resolve(__dirname, '..', '..', '..');

assert.strictEqual(findLawSiteRoot(root), root);
const index = loadSearchIndex(root);
assert.ok(index.documentCount > 0, 'search index should load');

const lawResults = searchLaw(root, 'superrepo court order', { limit: 5 });
assert.ok(lawResults.length > 0, 'searchLaw should return results');
assert.ok(lawResults.some((r) => String(r.id).includes('REALM-PC 19') || /superrepo/i.test(r.summary)), 'searchLaw should find superrepo material');
assert.ok(lawResults.every((r) => r.source === undefined), 'searchLaw must not include source text');

const si = getLawRecord(root, '[2026] REALM-SI 7');
assert.ok(si && si.id === 'si:7', 'getLawRecord should resolve SI citation');
assert.ok(si.source === undefined, 'getLawRecord should omit source text by default');

const withSource = getLawRecord(root, '[2026] REALM-SI 7', { includeSource: true, maxChars: 200 });
assert.ok(withSource.source && withSource.source.text.length <= 200, 'includeSource should be explicit and bounded');

const node = graphNode(root, 'si:7');
assert.ok(node && node.node && node.node.id === 'si:[2026] REALM-SI 7', 'graphNode should resolve si:7 alias');
assert.ok(node.counts.incoming >= 0 && node.counts.outgoing >= 0, 'graphNode should include edge counts');

const edges = graphEdges(root, 'si:7', { dir: 'both', limit: 5 });
assert.ok(edges && Array.isArray(edges.edges), 'graphEdges should return an edge list');
assert.ok(edges.edges.length <= 5, 'graphEdges should honour limit');
assert.ok(edges.edges.every((edge) => edge.source && edge.target), 'graphEdges should include node summaries');

console.log('law-lookup tests OK');
