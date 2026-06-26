# filecloud - Metacognition

What we know, what we don't know, open questions, and assumptions that need validating.

Status: Revised 2026-06-26. Architecture corrected: filecloud is a React component in the Opbox frontend, not a standalone Beelink server.

---

## What we know

### Architecture
- Filecloud is a React component embedded in the Opbox frontend at `/files`. It replaces `FilesBrowserPage`.
- Files live in Opbox's S3 bucket, not on a local Beelink filesystem.
- All file I/O goes through the Opbox kernel's verb API (`file.put`, `file.get`, `file.list`, `file.delete`, `file.replace_bytes`).
- The kernel handles AEAD encryption before bytes reach S3. Filecloud never handles ciphertext.
- Decrypted bytes are served via `/api/stored-files/[key]`, which supports range requests.
- Auth is Opbox's existing NextAuth session - same-origin API calls carry the cookie automatically.
- Persistent state (tags, share links, recent, starred, activity) goes into Opbox Postgres via the kernel or existing API routes.
- The kernel's S3 backend (`ReservedBackend`) is fail-closed until a VJS court record unlocks it (DEC-FC-017).

### Opbox frontend context
- Opbox frontend uses React, TypeScript (strict), and Next.js.
- Tiptap document editor already exists at `/documents/[id]` - DOCX files open there.
- Theme system applies Onyx classes to `<html>` - filecloud inherits the active theme automatically.
- Fonts (Bricolage Grotesque, PP Pangaia, PP Neue York, JetBrains Mono) are already loaded by Opbox.
- Lucide React is the icon library convention in Opbox.
- `/api/stored-files/[key]` already exists and serves decrypted file bytes.

### Onyx design system
- All five themes: default (light), dark, neon, ember, ocean.
- Canonical token set: `--bg-primary`, `--bg-secondary`, `--bg-tertiary`, `--border`, `--ink`, `--ink-sub`, `--ink-mute`, `--ink-hair`, `--accent`, `--accent-soft`, `--accent-fg`, `--paper`, `--soft`, `--font-page`, `--font-subhead`, `--font-body`, `--font-mono`.
- Sidebar layout tokens: `--app-sidebar-width: 264px`, `--app-sidebar-inset: 8px`, `--app-sidebar-inset-top: 16px`, `--app-sidebar-inset-bottom: 16px`.
- Type scale: `--text-display: 28px`, `--text-h1: 20px`, `--text-h2: 16px`, `--text-body-lg: 14px`, `--text-body: 13px`, `--text-body-sm: 12px`, `--text-meta: 11px`, `--text-pill: 10px`.

### What this replaces
- `FilesBrowserPage` in the Opbox frontend (the immediate target).
- Nextcloud (the prior tool, no longer in scope to replace directly).

### What we are deliberately not building
- Standalone server (no Hono, no port, no systemd unit)
- Separate auth (no bcrypt, no session cookie)
- SQLite database
- LibreOffice headless conversion
- ffmpeg transcoding
- Filesystem trash directory
- Sync client, WebDAV

---

## Remaining unknowns

**U-11: Does a `file.list` verb exist in the kernel, or must filecloud use `/api/files/browse`?**
The kernel verb API includes `file.put`, `file.get`, `file.delete`, `file.replace_bytes`. Whether `file.list` is implemented or whether directory listing goes via a separate HTTP route needs to be confirmed in the kernel source before building the browse component.

**U-12: Does the kernel have a `file.delete` verb, and what does it do to the S3 object?**
If `file.delete` exists, does it hard-delete the S3 object immediately, or does it support a soft-delete flag? The trash/recycle behaviour depends on this. If only hard-delete exists, soft-delete must be handled at the Postgres layer separately.

**U-13: Does the kernel have a tagging verb, or does that go via a separate API route?**
Tags could go through a kernel verb (if the kernel has a generic metadata/tagging system), a dedicated `/api/files/tags` route, or require a new route. This needs confirming before building the tag UI.

**U-14: What S3 provider and credentials will be used?**
The ReservedBackend is fail-closed pending a VJS court record (DEC-FC-017). The S3 provider (AWS, Cloudflare R2, self-hosted Minio, etc.) and the credential provisioning path are not yet determined. This blocks integration testing.

**U-15: Does `file.put` support folder paths/hierarchy, or is it flat?**
If `file.put` is flat (keyed by a single string with no folder semantics), then folder navigation is a UI-layer convention applied to key prefixes. If the kernel understands folder hierarchy natively, the browse component can rely on that. Needs confirming from kernel source.

**U-16: Is there a bulk download (zip) API route in Opbox already?**
FR-DOWNLOAD-002 and FR-DOWNLOAD-003 require downloading multiple files as a ZIP. If Opbox has no existing bulk-download route, this either needs a new API route or a client-side zip assembly (which means decrypting all files client-side via multiple `/api/stored-files/[key]` calls and zipping in the browser via JSZip). The client-side approach is feasible but slower.

**U-17: What is the 50MiB blob cap in `file.put`, and is there a chunked upload path?**
The kernel's `file.put` verb is understood to have a 50MiB cap per call. For files above that size, there must be either a chunked upload verb or a multipart upload route. If neither exists, large-file uploads will fail silently or with a kernel error. This needs confirming before the upload component is built.

**U-18: Does Opbox already have a calendar surface?**
If so, what route is it at and what does it accept (date, event ID)? Filecloud needs the correct route to hand off to it from file previews or context menus that reference calendar events.
---

## Open questions

**OQ-01: Activity log storage**
Where does the activity log live in the new architecture? Options: kernel audit trail (if the kernel logs all verb calls), a dedicated Postgres table, or a client-side session log (poorest option). Needs confirming.

**OQ-02: Share links**
Does Opbox already have a share-link API for files? The `file.put`/`file.get` verbs suggest the kernel manages file access, but time-limited public share links may need a separate mechanism. Needs a check of existing `/api/share` routes before building the share UI.

**OQ-03: StorageAdapter package location**
Does the StorageAdapter interface belong in the Opbox frontend repo or in a shared package? If per-tenant adapters are needed, it may need to be configurable at the workspace level via a setting in Opbox.

**OQ-04: SharePoint adapter - OAuth model**
Does SharePoint adapter require OAuth per-user or a service account? This affects whether the adapter config is per-user or per-workspace.

**OQ-05: Tagging in v1**
Decision: tagging is in v1 scope but is the last feature to build and first to cut if the build runs long (DEC-FC-008 confirmed). Depends on U-13 being resolved first.

**OQ-06: Full-text content search**
Filename/key search only in v1. Full-text content search (inside files) is explicitly v2 and requires kernel-side support for indexing decrypted content.

---

## Assumptions (current status)

| ID | Assumption | Status | Notes |
|----|-----------|--------|-------|
| A-01 | Filecloud is a React component inside Opbox, not a standalone server | Confirmed (architecture corrected 2026-06-26) | |
| A-02 | `/api/stored-files/[key]` exists and serves decrypted bytes with range requests | Assumed from description - verify in Opbox source | |
| A-03 | Opbox uses NextAuth for session management | Accepted | Filecloud relies on this for auth |
| A-04 | Tiptap document editor exists at `/documents/[id]` and can open DOCX | Assumed - verify DOCX support | |
| A-05 | S3 backend is fail-closed until VJS court record | Accepted (DEC-FC-017) | Blocks integration testing |
| A-06 | Opbox Postgres is accessible via the kernel or existing API routes | Accepted | No direct DB writes from filecloud |
| A-07 | `file.put`, `file.get`, `file.delete`, `file.replace_bytes` verbs exist in the kernel | Assumed from description - verify in kernel source | |
| A-08 | The kernel handles AEAD encryption end-to-end | Accepted (DEC-FC-018) | Filecloud never handles ciphertext |

---

## Constraints inherited from context

- Filecloud is a frontend component only. No server process, no port, no separate deployment.
- All file byte access goes through the kernel. No direct S3 reads or writes.
- Auth is Opbox's session. No separate credentials.
- S3 backend unlock (VJS court record) is a prerequisite for any end-to-end file I/O testing.
- The kernel's 50MiB cap on `file.put` applies to uploads until a chunked path is confirmed.
