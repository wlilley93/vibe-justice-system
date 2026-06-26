# filecloud - Decision Ledger

All founding decisions and revisions. Max 60 words for each decision, 120 for rationale.

---

## DEC-FC-001: Replace FilesBrowserPage with filecloud

**Date:** 2026-06-26 (revised 2026-06-26)
**Status:** Accepted - corrects founding framing

**Decision:** Build filecloud as a cleanroom React component to replace Opbox's `FilesBrowserPage`, embedded at `/files`. The original framing ("replace Nextcloud with a standalone server") is superseded. The correct context is replacing a weak internal page within the Opbox frontend.

**Rationale:** Nextcloud was the prior tool, but the immediate replacement target is `FilesBrowserPage` in the Opbox frontend. Filecloud is not a standalone server - it is an Opbox frontend component. Building it as a component means it inherits Opbox auth, theme, routing, and API access without duplication. The cleanroom approach is still correct: no Nextcloud code, no generic file-server frameworks.

---

## DEC-FC-002: Stack - React component within the Opbox frontend (no standalone server)

**Date:** 2026-06-26 (revised 2026-06-26)
**Status:** Accepted - replaces founding decision

**Decision:** Filecloud is a React component tree built with the Opbox frontend build system. No Hono server, no Vite standalone build, no SQLite, no port. The original decision (Node.js + Hono + React + Vite as a standalone app) is superseded.

**Rationale:** Filecloud reads and writes files via the Opbox kernel's verb API. The kernel, the session, the theme system, and the Postgres data layer are all already in Opbox. Building a separate server would duplicate all of that and create a second authentication surface. A React component embedded at `/files` gets everything for free and ships as part of the normal Opbox deploy.

---

## DEC-FC-003: No separate SQLite - Opbox Postgres is the data layer

**Date:** 2026-06-26 (revised 2026-06-26)
**Status:** Superseded - original decision no longer applies

Original decision: use SQLite via `better-sqlite3` for tags, share links, recent files, starred files, trash metadata, and activity log.

Superseded because: filecloud is a React component inside Opbox. All persistent state (tags, share links, recent, starred, activity) goes into Opbox Postgres via the kernel or existing API routes. No local SQLite database.

---

## DEC-FC-004: Filename/key search uses Opbox file search API

**Date:** 2026-06-26 (revised 2026-06-26)
**Status:** Accepted - scope updated

**Decision:** Search covers file keys and names only, using the existing Opbox file search API. Full-text content search inside files is explicitly deferred.

**Rationale:** The original decision (SQLite FTS5 index over 184k local files) is superseded by the component architecture. Filecloud uses whatever search endpoint the Opbox kernel or `/api/files` layer exposes. Full-text content search remains out of scope for v1 regardless of architecture.

---

## DEC-FC-005: Theme inherited from Opbox - no separate theme switcher

**Date:** 2026-06-26 (revised 2026-06-26)
**Status:** Accepted - corrects founding decision

**Decision:** Filecloud inherits Opbox's active theme via the `<html>` class set by the Opbox theme system. No separate theme switcher inside filecloud. All five Onyx themes (default/dark/neon/ember/ocean) are supported automatically.

**Rationale:** As an embedded Opbox component, filecloud shares the `<html>` element with the rest of the app. The theme is already applied. Adding a second theme control would fight the Opbox system and create divergent state. The user sets the theme in Opbox settings; filecloud just responds.

---

## DEC-FC-006: No port - filecloud is a component, not a server

**Date:** 2026-06-26 (revised 2026-06-26)
**Status:** Superseded - original decision no longer applies

Original decision: run on port 3000 by default, configurable via `PORT` env var.

Superseded because: filecloud is a React component embedded in the Opbox frontend. It does not run a server, does not bind a port, and is not accessed via a separate URL. It is served as part of the Opbox frontend at `/files`.

---

## DEC-FC-007: Auth is Opbox session (NextAuth) - no separate auth

**Date:** 2026-06-26 (revised 2026-06-26)
**Status:** Accepted - replaces founding decision

**Decision:** Filecloud uses Opbox's existing NextAuth session. No separate login page, no password hash, no session cookie managed by filecloud. If the session is invalid, the component redirects to the Opbox login page.

**Rationale:** The original decision (single-password session auth with bcrypt) was for a standalone server. As an embedded component, filecloud is inside the Opbox auth boundary. Same-origin API calls carry the existing session cookie automatically. Adding a second auth layer is redundant and creates two passwords to manage.

---

## DEC-FC-008: Tagging included in v1 scope, last to build

**Date:** 2026-06-26
**Status:** Accepted

**Decision:** Tagging (add/remove/filter/manage) is in v1 scope but is the last feature to build and the first to cut if the build runs long.

**Rationale:** Tags add navigation value. The data layer is Opbox Postgres (via a tagging verb or API route - see U-13). The UI is a pill-input component. The main cost is the tag-browse view. These are manageable but are the lowest-priority v1 feature relative to browse, search, preview, and share.

---

## DEC-FC-009: Lucide React for all iconography

**Date:** 2026-06-26
**Status:** Accepted

**Decision:** Use Lucide React for all icons. Size: 14px/1.5px stroke in list rows and sidebar; 20px/1.5px in modals and FAB. No other icon library.

**Rationale:** Lucide is MIT licensed, has comprehensive file-type and action icons, is tree-shakeable, and is the Opbox visual convention. Consistency with Opbox means no visual register mismatch when moving between the files view and the rest of the app.

---

## DEC-FC-010: No sync client, no WebDAV, web component only

**Date:** 2026-06-26
**Status:** Accepted

**Decision:** Filecloud is a web component only. No sync client, no mobile app, no WebDAV endpoint, no CalDAV.

**Rationale:** The use case is browsing and retrieving files from a browser - Mac and iPhone over Tailscale. A sync client requires OS-level software on every device and conflict resolution logic. WebDAV adds significant server complexity and is out of scope for a file-browser replacement.

---

## DEC-FC-011: Fonts loaded by Opbox - no separate font strategy

**Date:** 2026-06-26 (revised 2026-06-26)
**Status:** Accepted - replaces founding decision

**Decision:** Filecloud uses whatever fonts Opbox has already loaded. No separate font imports, no self-hosting decisions, no new license purchases.

**Rationale:** As an embedded Opbox component, filecloud shares the document with the rest of the app. Opbox already loads Bricolage Grotesque, PP Pangaia, PP Neue York, and JetBrains Mono. No separate font strategy is needed or wanted - adding one would either duplicate loads or create conflicts.

---

## DEC-FC-012: DOCX files open in Tiptap editor - no LibreOffice headless

**Date:** 2026-06-26 (revised 2026-06-26)
**Status:** Accepted - replaces founding decision

**Decision:** DOCX files open in the existing Tiptap editor at `/documents/[id]`. No LibreOffice headless conversion, no PDF conversion cache. Other Office formats (XLSX, PPTX) show a download prompt.

**Rationale:** Opbox already has a Tiptap-based document editor that handles DOCX. Using it is the right call: no new dependency, no conversion latency, no preview cache to manage. LibreOffice headless is not available in the Opbox runtime environment (it was a Beelink-local tool from the superseded standalone architecture).

---

## DEC-FC-013: No ffmpeg transcoding - video served via stored-files API

**Date:** 2026-06-26 (revised 2026-06-26)
**Status:** Superseded - original decision no longer applies

Original decision: no video transcoding in v1; MKV is download-only; FFmpeg is deferred to v2.

Superseded because: the stored-files API (`/api/stored-files/[key]`) serves decrypted bytes with range-request support. The browser plays natively supported formats (MP4, WebM) from that endpoint. MKV and other non-native formats show a download prompt. FFmpeg transcoding is not in scope at all - it was only relevant for a Beelink-local server.

---

## DEC-FC-014: No .filecloud-trash/ directory - soft-delete in Postgres

**Date:** 2026-06-26 (revised 2026-06-26)
**Status:** Superseded - original decision no longer applies

Original decision: deleted files moved to `.filecloud-trash/` in the served root; metadata in SQLite.

Superseded because: filecloud does not have filesystem access. Trash is implemented as a soft-delete flag in Opbox Postgres (via a kernel verb or `/api/files` route), or via a `file.delete` kernel verb if the kernel tracks deletion state. No directory is created on any disk.

---

## DEC-FC-015: Floating sidebar, no topbar

**Date:** 2026-06-26
**Status:** Accepted

**Decision:** The sidebar uses the Onyx floating sidebar pattern (264px, 8px left inset, 16px top/bottom inset, 12px border-radius, position: fixed within the component container). There is no topbar. Navigation, breadcrumb, and upload controls live in the content area header.

**Rationale:** The Onyx globals.css defines the floating sidebar tokens. Adopting this pattern gives visual consistency with the rest of Opbox without additional design work. Eliminating the topbar reduces chrome and gives more vertical space to the file listing.

---

## DEC-FC-016: Embed-ready from day one

**Date:** 2026-06-26
**Status:** Accepted

**Decision:** Filecloud is built as an embedded Opbox component. There is no standalone delivery mode. Theme class is applied by the parent (`<html>`), not by filecloud. The internal sidebar is collapsible/hideable via props.

**Rationale:** This is no longer just a design goal - it is the architecture. The component must work inside the Opbox router at `/files`. Sidebar toggle must be an accessible prop/state, not hardcoded open.

---

## DEC-FC-017: S3 backend unlock requires VJS court record - build prerequisite

**Date:** 2026-06-26
**Status:** Accepted

**Decision:** The Opbox kernel's S3 backend (`ReservedBackend`) is fail-closed. File I/O is impossible until a VJS court record is issued to unlock it. This is a build prerequisite - filecloud cannot be end-to-end tested until the unlock is in place.

**Rationale:** The kernel enforces this gate. No workaround exists. Development can proceed on the UI and wiring, but integration testing of actual file reads/writes is blocked until the court record is obtained. See U-14 for the S3 credentials question.

---

## DEC-FC-018: Filecloud never handles ciphertext - all byte access via kernel

**Date:** 2026-06-26
**Status:** Accepted

**Decision:** Filecloud never reads or writes S3 directly, and never handles encrypted bytes. All file content is accessed via `/api/stored-files/[key]` (decrypted bytes served by the kernel) or via kernel verbs (`file.get`, `file.put`). The kernel handles AEAD encryption end-to-end.

**Rationale:** The kernel's encryption layer is the security boundary. If filecloud bypassed it, files would be stored unencrypted or require filecloud to manage keys - both are unacceptable. This is a hard architectural invariant, not a preference.

---

## DEC-FC-019: Storage adapter abstraction

**Date:** 2026-06-26
**Status:** Locked

**Decision:** Filecloud separates the browser UI from the storage backend via a StorageAdapter interface. The interface defines: list(path), get(key), put(key, blob), delete(key), move(src, dst), mkdir(path). Opbox S3 is the first implementation. SharePoint, local disk, and other backends are future adapters, swappable per workspace.

**Rationale:** Different tenants may want different storage backends (SharePoint for enterprise, local disk for self-hosted, Dropbox for personal). The browser surface stays identical regardless of where bytes live. This mirrors the kernel's own StorageBackend trait pattern. Adapter is injected at the component root via a React context provider.

---

## DEC-FC-020: Filecloud scope is the browser surface only

**Date:** 2026-06-26
**Status:** Locked

**Decision:** Filecloud implements one thing: a file browser surface. It does not implement a document editor, calendar, contacts, video calls, mail, or any other Nextcloud plugin equivalent. Those already exist in Opbox (Tiptap editor, calendar). Filecloud hands off to them via routing, not by building its own version.

**Rationale:** Nextcloud's value is its plugin ecosystem. Opbox already has those surfaces. Building them again inside filecloud would be duplication. The right model is: filecloud handles browse/upload/share/preview, Opbox handles edit/calendar/collaborate.

---

## DEC-FC-021: MinIO as S3 backend, local to Opbox stack

**Date:** 2026-06-26
**Status:** Locked

**Decision:** The S3 storage backend is MinIO, running as a container in the Opbox docker-compose stack. No external S3 provider. Endpoint is `http://minio:9000` within the stack network.

**Rationale:** MinIO is S3-compatible so the StorageAdapter interface works unchanged for a future migration to AWS S3 or R2. Running it locally avoids egress costs, external auth, and network dependency. The choice was confirmed when the kernel's `ReservedBackend` was replaced with `MinioBackend` under [2026] CC-OPBOX 57. Setting `FILE_STORAGE_BACKEND=minio` activates the backend; default remains `bytea`.
