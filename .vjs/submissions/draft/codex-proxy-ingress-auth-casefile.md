# Codex per-cell model proxy: ingress authentication (symmetric case file)

## Question
How must the per-cell Codex model proxy authenticate a Codex process that connects over
TCP HTTP loopback, given that secure model-proxy peer-attestation (`ModelProxyCellScope` via
`SO_PEERCRED`) requires a Unix-socket peer (`AcceptedUnixPeer`), and
`TrustedModelProxyRequestObservation` records "production authentication stays off until
Unix-socket ingress creates this from SO_PEERCRED"?

## Fixed constraints (not in dispute)
- Codex 0.144.3 pins `base_url = http://127.0.0.1:{port}/v1`, `wire_api = responses`, and an
  `auth.command` helper (`helper --cell-id X`, refresh 30s).
- `SO_PEERCRED` yields unforgeable peer identity only on `AF_UNIX` sockets.
- The grant store exposes `find_active_for_trusted_observation` (peer-attested) and
  `find_active_by_id` (bearer-only).
- The upstream key never enters the cell env. Scope: read-only cutover; write phase (PR8) gated.

## Option A - Unix-socket ingress for model calls
Provider `base_url` becomes a Unix socket; `SO_PEERCRED` attests every call.
- For: strongest continuous per-request attestation; no replay window.
- Against: depends on Codex's HTTP client supporting a unix-socket base_url (non-standard;
  likely infeasible without patching Codex); the pinned config is TCP.

## Option B - TCP + /proc/net/tcp peer resolution
Resolve the peer PID from the connection's source port via `/proc/net/tcp`, then attest.
- For: no Codex config change; a per-request peer identity of a sort.
- Against: `/proc/net/tcp` resolution is TOCTOU-racy and weaker than `SO_PEERCRED`; the
  codebase demands `AcceptedUnixPeer` and states "Caller JSON is never acceptable provenance",
  so this degrades the stated model.

## Option C - Two-channel: Unix-socket bearer issuance + bearer-authenticated TCP proxy
The auth-helper connects to a Boltrig Unix socket; `SO_PEERCRED` attests the helper into a
`ModelProxyCellScope` and issues a short-TTL, single-cell bearer at issuance. Codex presents
that bearer to the loopback TCP proxy per call; the proxy authenticates by bearer alone
(`find_active_by_id`), injects the kernel-only key, forwards to bifrost.
- For: reconciles SO_PEERCRED-needs-Unix with Codex's TCP reality by attesting the peer at
  bearer issuance, not per call; matches the pinned config's two-component shape (helper +
  TCP proxy); real attestation (unlike B); feasible without patching Codex (unlike A);
  bearer short-TTL, single-cell, loopback-only.
- Against: the TCP channel trusts the bearer, so a bearer stolen within its TTL by a local
  process could be replayed on loopback; bounded by short TTL, single-cell binding, loopback bind.

## At stake
Whether `SO_PEERCRED` must gate every model call (A/B) or may gate bearer issuance only, with
the model-call channel trusting the issued short-TTL capability (C).
