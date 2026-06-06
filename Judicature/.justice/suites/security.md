# VJS Security Suite

## When to invoke

Invoke this suite whenever changes touch:
- Authentication or authorisation code (login, session, JWT, RBAC)
- Cryptographic operations (hashing, encryption, key management)
- Secret or credential handling (.env, config loading, API keys)
- Input validation or sanitisation (SQL, HTML, shell commands, file paths)
- File upload, read, or path resolution
- Shell command execution with any user-controlled input
- Dependency additions (npm install, pip install, cargo add)
- Network exposure (new endpoints, CORS config, WebSocket, webhooks)

A court ruling may also explicitly mandate invocation via its remedy order.

## Checks

Work through each check in order. Record findings in your work log. A finding that indicates a breach triggers
mandatory self-submission to court (see plugin/CLAUDE.md).

### 1. Injection vectors

- SQL: parameterised queries everywhere - no string concatenation in query text
- Shell: no user-controlled strings passed to `exec`/`spawn` without sanitisation
- HTML/XSS: output encoding verified for all user-supplied strings rendered to HTML
- Path traversal: file paths resolved with `path.resolve` / normalised; `../` passthrough blocked

### 2. Authentication and authorisation

- Auth checks applied before every protected action (no missing guard)
- Session tokens: `httpOnly`, `Secure`, `SameSite=Strict` where applicable
- JWTs: signature verified; expiry checked; HS256 replaced with RS256 where keys are shared
- RBAC/permissions: privilege checks enforced on the server, not only the client

### 3. Secrets and credentials

- No secrets, passwords, tokens, or keys committed to git (scan with `git grep` / `truffleHog`)
- `.env` files gitignored; no `.env.example` containing real values
- API keys loaded from environment, never hardcoded
- Secrets not logged (check logger calls around auth/config paths)

### 4. Dependency audit

- `npm audit` / `pip-audit` / `cargo audit` run; critical and high severity findings investigated
- No abandoned or unmaintained direct dependencies with known CVEs

### 5. Error handling and information disclosure

- Error responses do not leak stack traces, file paths, or internal identifiers to clients
- Generic error messages for auth failures (no user/password distinguishing in error text)

## Updating this suite

Any project member with security knowledge may update this file directly via a PR. A court ruling that mandates
a new security practice is incorporated at the time the remedy is executed. Record the ruling citation in a
comment next to the check that was added by order.
