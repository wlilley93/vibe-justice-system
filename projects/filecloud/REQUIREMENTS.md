# filecloud - Requirements

Status: Revised 2026-06-26. Architecture corrected: filecloud is a React component embedded in the Opbox frontend, not a standalone server.

---

## 1. Scope

Filecloud replaces `FilesBrowserPage` in the Opbox frontend. It is a React component embedded at `/files`. Files are stored in Opbox's S3 bucket. All file I/O goes through the Opbox kernel's verb API (`file.put`, `file.get`, `file.list`, `file.delete`, `file.replace_bytes`). The kernel handles AEAD encryption before bytes reach S3 - filecloud never handles ciphertext. Decrypted bytes are served via `/api/stored-files/[key]`.

The S3 backend (`ReservedBackend`) is fail-closed until a VJS court record unlocks it (see DEC-FC-017). That unlock is a build prerequisite for any file I/O.

Filecloud does not replicate sync clients, calendar, contacts, or standalone auth.

---

## Scope boundary

Filecloud is a file browser surface. It does not include:
- Document editing (handled by Tiptap at `/documents/[id]`)
- Calendar (handled by Opbox calendar)
- Contacts, mail, video calls, or any Nextcloud plugin equivalent

Filecloud hands off to those surfaces via routing. It does not build its own versions.

The storage backend is abstracted via a StorageAdapter interface. The first implementation is Opbox S3 via the kernel verb API. Future adapters (SharePoint, local disk, Dropbox) are swappable without changing the browser UI.

---

## 2. Browse and navigate

- **FR-BROWSE-001** - Display directory contents as a sortable list: name, size, modified date, type.
- **FR-BROWSE-002** - Display directory contents as a thumbnail grid for image-heavy folders.
- **FR-BROWSE-003** - Toggle between list and grid view; preference persisted per-session.
- **FR-BROWSE-004** - Breadcrumb showing full path from root, each segment clickable.
- **FR-BROWSE-005** - Click-through navigation into subdirectories without full page reload (client-side routing within the component).
- **FR-BROWSE-006** - Show file count and total size for each directory in the listing (from `file.list` response).
- **FR-BROWSE-007** - Sort by name (A-Z, Z-A), modified date (newest, oldest), size (largest, smallest), type.
- **FR-BROWSE-008** - Folder tree in a collapsible floating sidebar for quick deep navigation (desktop only).
- **FR-BROWSE-009** - Browser back/forward history works correctly; URL reflects the current path.
- **FR-BROWSE-010** - Quick access bar at the top of the content area showing recent and starred files as horizontal tiles.
- **FR-BROWSE-011** - "Up one level" button in breadcrumb (or keyboard shortcut Backspace when not focused on an input).

---

## 3. Search

- **FR-SEARCH-001** - Filename/key search across the file store, using the existing Opbox file search API.
- **FR-SEARCH-002** - Search results show filename, path, size, and modified date.
- **FR-SEARCH-003** - Search triggered by keyboard shortcut (Cmd+K / Ctrl+K) and by clicking the sidebar search trigger.
- **FR-SEARCH-004** - Search filters: by file type (folder, image, video, audio, PDF, document, code, archive, other).
- **FR-SEARCH-005** - Search filters: by modified date range (today, this week, this month, custom range).
- **FR-SEARCH-006** - Search filters: by size range (< 1MB, 1-10MB, > 10MB).
- **FR-SEARCH-007** - Search results ranked by relevance: exact filename match first, then partial match, then path match.
- **FR-SEARCH-008** - Recent searches shown when the search overlay opens with an empty input.
- **FR-SEARCH-009** - Keyboard navigation in results: arrow keys to move, Enter to open preview, Escape to close.

---

## 4. Preview

- **FR-PREVIEW-001** - In-browser preview opens in a right-rail panel (400px, desktop) or full-screen sheet (mobile) without leaving the current directory.
- **FR-PREVIEW-002** - PDF preview via pdf.js: page navigation, zoom (50%-200%), text selection, keyboard shortcuts (arrow keys for pages). Bytes fetched from `/api/stored-files/[key]`.
- **FR-PREVIEW-003** - Image preview (JPEG, PNG, GIF, WebP, AVIF, SVG): zoom (scroll wheel / pinch) and pan (drag / touch drag). Bytes from `/api/stored-files/[key]`.
- **FR-PREVIEW-004** - Markdown preview rendered as HTML with code blocks syntax-highlighted (Prism.js).
- **FR-PREVIEW-005** - Plain text and code preview with syntax highlighting and line numbers. Language auto-detected from file extension.
- **FR-PREVIEW-006** - Video preview (MP4, WebM, MOV): native `<video>` pointing at `/api/stored-files/[key]` with range-request support. Non-native formats (MKV etc.) show a download prompt.
- **FR-PREVIEW-007** - Audio preview (MP3, FLAC, OGG, M4A, WAV, AIFF): custom audio player UI with progress bar, time display, play/pause. Bytes from `/api/stored-files/[key]`.
- **FR-PREVIEW-008** - DOCX files open in the existing Tiptap editor at `/documents/[id]` - no separate preview panel for DOCX.
- **FR-PREVIEW-009** - Non-DOCX, non-previewable files show: type icon + "No preview available" + Download button.
- **FR-PREVIEW-010** - Previous/next file navigation within the current directory while preview is open. Keyboard shortcuts: Left/Right arrow.
- **FR-PREVIEW-011** - Preview panel can be dismissed by pressing Escape or clicking outside it on desktop.
- **FR-PREVIEW-012** - Download button always present in preview panel regardless of file type.

---

## 5. File operations

- **FR-OPS-001** - Rename a file or directory via inline edit (double-click or F2) or context menu. Uses kernel verb (or `/api/files` route).
- **FR-OPS-002** - Delete file with confirmation prompt. Soft-delete via kernel verb or Postgres flag - no `.filecloud-trash/` directory.
- **FR-OPS-003** - Move file or directory via drag-and-drop or a Move modal with path picker.
- **FR-OPS-004** - Copy file or directory via context menu. Destination selected via path picker modal.
- **FR-OPS-005** - Create new directory via button in the content header or context menu.
- **FR-OPS-006** - Multi-select via checkbox (click checkbox, or Shift+click range, or Cmd+A for all).
- **FR-OPS-007** - Bulk operations on multi-select: download as zip, move, copy, delete, tag.
- **FR-OPS-008** - Context menu on right-click (desktop) or long-press (mobile): Preview, Download, Rename, Move, Copy, Share, Tag, Star, Delete.

---

## 6. Upload

- **FR-UPLOAD-001** - Drag-and-drop files or folders onto any directory view.
- **FR-UPLOAD-002** - Click-to-select upload via a file picker button (single and multi-file).
- **FR-UPLOAD-003** - Per-file upload progress shown in a panel (slides up from bottom-right).
- **FR-UPLOAD-004** - Upload via existing `/api/files/upload` route or `file.put` kernel verb.
- **FR-UPLOAD-005** - Upload destination is the currently browsed directory (path prefix).
- **FR-UPLOAD-006** - Upload conflict handling: prompt to overwrite, rename (auto-suffix), or skip per file.
- **FR-UPLOAD-007** - Upload can be cancelled mid-flight per file.
- **FR-UPLOAD-008** - `file.put` has a 50MiB cap per call (U-17). Files above that threshold show a warning until a chunked upload path is confirmed or added.

---

## 7. Download

- **FR-DOWNLOAD-001** - Download any single file via a Download button in the listing row, context menu, and preview panel. Fetches from `/api/stored-files/[key]`.
- **FR-DOWNLOAD-002** - Download a directory as a ZIP archive. Uses bulk download route if it exists in Opbox (U-16), or client-assembles from individual file fetches.
- **FR-DOWNLOAD-003** - Multi-select files and download as a ZIP archive (same as FR-DOWNLOAD-002).
- **FR-DOWNLOAD-004** - Download of large files uses HTTP range requests to allow resume (served by `/api/stored-files/[key]`).

---

## 8. Share links

- **FR-SHARE-001** - Generate a time-limited share link for any file from the context menu or preview panel.
- **FR-SHARE-002** - Share links are managed via Opbox's existing data layer (Postgres via kernel or existing share-link API).
- **FR-SHARE-003** - Expiry options: 1 hour, 1 day, 7 days, 30 days, no expiry.
- **FR-SHARE-004** - List all active share links with target and expiry in the Share modal.
- **FR-SHARE-005** - Revoke any share link before its expiry.
- **FR-SHARE-006** - Share links include a "copy to clipboard" button in the modal and a toast confirmation.

---

## 9. Tagging

- **FR-TAG-001** - Add one or more tags to any file from the row context menu, multi-select bulk action, or preview panel.
- **FR-TAG-002** - Tags are stored in Opbox Postgres via kernel (or a dedicated tagging API route - see U-13).
- **FR-TAG-003** - Browse by tag: clicking a tag pill navigates to a filtered view.
- **FR-TAG-004** - Tag autocomplete when typing, showing existing tags sorted by frequency.
- **FR-TAG-005** - Tag management: rename tag, delete tag (removes from all associated files).
- **FR-TAG-006** - Tags shown as pills in the file listing row (max 3 visible, +N more on hover).

---

## 10. Starred / favourites

- **FR-STAR-001** - Star any file from the context menu, preview panel, or by clicking a star icon on row hover.
- **FR-STAR-002** - Starred items appear in the quick access bar and in the Starred view (accessible from the sidebar).
- **FR-STAR-003** - Starred state is stored in Opbox Postgres.
- **FR-STAR-004** - Unstar from the same locations. Confirmation not required.

---

## 11. Recent files

- **FR-RECENT-001** - Track the last 50 files accessed (previewed or downloaded) by the authenticated user, stored in Opbox Postgres.
- **FR-RECENT-002** - Recent files displayed in the quick access bar and in the Recent view (sidebar).
- **FR-RECENT-003** - Recent view shows: filename, path, last accessed timestamp, and file type.
- **FR-RECENT-004** - Clear recent history from the component's settings panel.

---

## 12. Activity log

- **FR-ACTIVITY-001** - Log all write operations (upload, delete, rename, move) with timestamp, operation type, and key/path.
- **FR-ACTIVITY-002** - Log all share link creations and revocations.
- **FR-ACTIVITY-003** - Activity log accessible from the component's settings panel, showing recent events in reverse-chronological order.
- **FR-ACTIVITY-004** - Activity log stored in Opbox Postgres (kernel audit trail or dedicated table).

---

## 13. Settings panel

- **FR-SETTINGS-001** - Default view mode: list or grid.
- **FR-SETTINGS-002** - Active share links: list and revoke.
- **FR-SETTINGS-003** - Tag management: rename and delete tags.
- **FR-SETTINGS-004** - Activity log: view and clear.

Note: theme is inherited from Opbox's existing theme system (html class set by Opbox). No separate theme switcher in filecloud.

---

## 14. Authentication

Auth is Opbox's existing session (NextAuth). No separate login page, no separate password, no bcrypt env var, no session cookie managed by filecloud.

- **FR-AUTH-001** - All file API calls include the Opbox session cookie automatically (same-origin).
- **FR-AUTH-002** - If the session is invalid or expired, the component redirects to the Opbox login page.
- **FR-AUTH-003** - Share links use the existing Opbox share-link mechanism (no separate auth bypass logic in filecloud).

---

## 15. Mobile

- **FR-MOB-001** - Fully usable on iPhone Safari: touch targets minimum 44px, no hover-only interactions for core functions.
- **FR-MOB-002** - Sidebar collapses to a bottom sheet triggered by a hamburger icon.
- **FR-MOB-003** - Preview panel is full-screen on mobile (100vw, 100dvh), dismissed by swipe down or X button.
- **FR-MOB-004** - Pinch-to-zoom on images in preview.
- **FR-MOB-005** - Upload via the floating action button (bottom-right): opens file picker or camera roll on iOS.

---

## 16. Performance

- **NFR-PERF-001** - Directory listings load in under 500ms (depends on `file.list` or `/api/files/browse` response time from the kernel).
- **NFR-PERF-002** - File list uses windowed virtualisation (tanstack-virtual) for directories with more than 200 items.
- **NFR-PERF-003** - Video streaming uses HTTP range requests so the browser can seek without full download (handled by `/api/stored-files/[key]`).
- **NFR-PERF-004** - Thumbnail generation is async and non-blocking; the directory listing renders before thumbnails load.

---

## 17. Security

- **NFR-SEC-001** - Filecloud never reads or writes S3 directly. All byte access goes via the kernel, which handles AEAD encryption/decryption.
- **NFR-SEC-002** - Share link tokens are managed by the Opbox share-link layer (cryptographic entropy requirements are Opbox's invariants).
- **NFR-SEC-003** - No external network calls from the filecloud component at runtime beyond the existing Opbox API.
- **NFR-SEC-004** - Upload size limit is determined by the kernel's `file.put` 50MiB cap (U-17) pending confirmation of a chunked upload path.

---

## 18. Maintainability

- **NFR-MAINT-001** - TypeScript throughout, strict mode.
- **NFR-MAINT-002** - Filecloud is a React component tree. No Hono server, no Vite standalone build, no SQLite, no port config. It is built with the Opbox frontend build system.
- **NFR-MAINT-003** - No telemetry, no analytics, no SaaS integrations.

---

- **NFR-ADAPTER-001** - All file I/O must go through the StorageAdapter interface. No direct S3 SDK calls, no direct kernel verb calls, from UI components. The adapter is the only boundary.
- **NFR-ADAPTER-002** - Switching the storage adapter must not require changes to any filecloud UI component.

---

## 19. Out of scope

- Standalone server or separate deployment
- Port 3000, systemd unit, Tailscale-served URL for filecloud itself
- Separate authentication (bcrypt, session cookies)
- SQLite database
- LibreOffice headless (DOCX opens in Tiptap instead)
- ffmpeg transcoding (video served as-is via stored-files API)
- `.filecloud-trash/` directory (soft-delete in Postgres)
- Sync clients (desktop or mobile apps that mirror the folder locally)
- Multi-user support or ACLs
- File versioning / history
- Calendar, contacts, or any Nextcloud feature beyond files
- Full-text content search inside files (v2 if ever)
- WebDAV endpoint
