# Vendored assets

The Gazette is a record: it must render offline, from an archive crawl, and
without trusting a third-party CDN (unpkg serves unpinned latest - silent code
substitution). Presentation libraries are therefore vendored and pinned.

| asset | version | license | upstream | sha256 (upstream dist) |
|---|---|---|---|---|
| force-graph.min.js | 1.49.5 | MIT | https://unpkg.com/force-graph@1.49.5/dist/force-graph.min.js | c778f2efccd7b18e9a4030f53d81d319f179a8ab8fbc6f10001dc6c904d203a1 |

Re-vendor: `curl -sL -o assets/vendor/force-graph.min.js https://unpkg.com/force-graph@<ver>/dist/force-graph.min.js`,
re-add the provenance header comment, update this table with the new sha256.
