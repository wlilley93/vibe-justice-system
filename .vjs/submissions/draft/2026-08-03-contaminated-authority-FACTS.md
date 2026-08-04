# Contaminated frame authority and a silently truncating puller

Full case file: `.vjs/submissions/draft/2026-08-03-opbox-contaminated-authority-and-a-truncating-puller.md`

## Measured facts

Eleven of Opbox's 167 registered frames carry a `LEGACY UNDERLAY` layer. All eleven are
`named_layer` and signed. None of the 122 refused frames carries one.

In each, the frame's own content, including its 56px rail and body, was renamed
`LEGACY UNDERLAY` and set invisible, and a new layer was given a recognised authority
marker. Six of those new layers name themselves `cloned from <node>`. Four are named
`CURRENT CODE`, meaning traced from the shipped application.

The demoted content is intact: 2,376 nodes, hidden not deleted. On `/pipelines/[id]` the
demoted layer holds 336 nodes and 111 texts against the authority layer's 86 and 58.

The Principal states the `/pipelines` family is not his work and was "signed in error",
that his frames draw a thin rail and are not named "current". Geometry corroborates: 134
frames draw a 56px rail; six draw a 220px expanded sidebar, four being the signed
`/pipelines` family.

Separately, `vds-figma/src/pull.rs` carries an uncommitted fix documenting that
`GET /v1/files/:key` returns HTTP 200 with a silently truncated body: one 136 MB file
fetched twice returned 102 MB and 62 MB, truncating at different points, no error.
`authority_root` inspects direct children only, so a lost marker falls silently through to
`frame_own_children`. The ledger producing 19/122/26 predates that fix.

## The case for relief

Order 16 holds a machine verdict creates no authority. An agent that clones a node, labels
the clone `SOURCE AUTHORITY`, and renames the Principal's drawing `LEGACY UNDERLAY` has
manufactured authority. The correlation is perfect and inverted: every overwritten frame is
registered, every untouched frame refused. CC-OPBOX 6 forbids treating inspection failure
as absence, and the partition is an inspection result from a defective inspector.

## The case against relief

The evidence comes from a capture taken with the impugned puller, so every figure may be
short. Richness and alignment are not legal tests; nothing makes craft a condition of
authority. The truncation ground proves possibility, not occurrence: no frame is shown to
have lost a marker, and re-derivation may return the partition unchanged. The `/pipelines`
sign-offs may rest on a direction not pleaded. Vacatur alone would cut registrable frames
from nineteen to eight and worsen the estate.

## Relief sought

Freeze the demoted layers; restore and re-sign in one operation per frame so authority
never dips; retire `CURRENT CODE` from the marker vocabulary as a narrowing, not a
widening; impose a prospective clean-hands condition; audit the remaining eight; re-derive
with the corrected puller before scheduling remediation; and confirm that the 122 untouched
frames may be cured by the second cure CC-OPBOX 6 already names, an express verified
hash-bound Principal label-resolution act.
