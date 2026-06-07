# Public VJS Publish Runbook

This runbook is for the final outward publication of the canonical public VJS repository to
`wlilley93/vibe-justice-system`.

## Status

The public publish is held at the Founder checkpoint. The private `agent-universe` backup may be
pushed to `origin` as a reversible private/dev push. A push to `upstream` / `vibe-justice-system`
is blocked by the VJS pre-push checkpoint gate unless an express authorisation record exists.

## Order

1. Verify the current tree is system data only.
2. Verify citator and judgment filing integrity.
3. Verify deterministic rendering/lodgement is clean.
4. Push the private backup branch to `origin`.
5. Prepare the public VJS publication from a clean canonical tree.
6. Record the Founder checkpoint.
7. Push to the public `vibe-justice-system` remote.

## Checkpoint Record

The pre-push gate accepts either a tracked reasons-ledger record:

```text
Judicature/ministry-of-justice/reasons-ledger/outward-act-authorisations/public-vjs-publish.md
```

or a local operational checkpoint:

```text
.vjs/checkpoints/public-vjs-publish-authorisation.env
```

Minimum content:

```text
AUTHORISED_OUTWARD_ACT=public-vjs-publish
AUTHORISED_BY=Sovereign Founder
AUTHORISED_AT=YYYY-MM-DDTHH:MM:SSZ
```

Optional scoping fields:

```text
AUTHORISED_REMOTE_URL=https://github.com/wlilley93/vibe-justice-system.git
AUTHORISED_REMOTE_REF=refs/heads/main
AUTHORISED_LOCAL_SHA=<exact sha being pushed>
```

If an optional field is present, the gate requires it to match the attempted push.

## History Rule

Do not publish the private development history wholesale. Commit `ead143e` once carried server-estate
facts before those facts were moved into the private operational registry. The current tree may be
public-safe, but the public VJS publication should be a clean canonical publication or a scrubbed
history, not a raw push of the private branch history.

Recommended public publication mode:

1. Build the public canonical tree from the current checked-out system-data files.
2. Exclude private/dev-only branches and ignored operational stores.
3. Publish as a clean public history, or use a verified history scrub before pushing.
4. Keep the private `agent-universe` repository as the full internal provenance and development
   record.
