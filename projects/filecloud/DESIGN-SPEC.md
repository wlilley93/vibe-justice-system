# filecloud - Design Specification

Status: Revised 2026-06-26. Completely rewritten against actual Onyx globals.css token values.

---

## 1. Design language

Filecloud uses the Onyx design system as its visual foundation. Onyx is multi-theme: light (default), dark, neon, ember, ocean. Filecloud inherits all five by adding the same theme class (`dark`, `neon`, `ember`, `ocean`) to the `<html>` element and loading globals.css. No custom colour overrides are needed.

---

## 2. CSS token reference

These are the canonical tokens filecloud uses, sourced directly from globals.css. All values are from the `:root` (light) block; each theme class overrides these in place.

### 2.1 Surface tokens

| Token | Light value | Purpose |
|-------|-------------|---------|
| `--bg-primary` | `#ffffff` | Page canvas, card, modal background |
| `--bg-secondary` | `#f8fafc` | Table headers, search inputs, sidebar muted bg |
| `--bg-tertiary` | `#f1f5f9` | Hover tint on rows, selected row fill |
| `--app-canvas` | `#eef0f2` | Outer page body (between sidebar and viewport edge) |
| `--border` | `#cbd5e1` | All hairline borders, dividers |
| `--border-hover` | `#94a3b8` | Border on hover |

Dark overrides (`.dark` class):

| Token | Dark value |
|-------|------------|
| `--bg-primary` | `#1a1a1a` |
| `--bg-secondary` | `#141414` |
| `--bg-tertiary` | `#262626` |
| `--app-canvas` | `#111111` |
| `--border` | `#2a2a2a` |

### 2.2 Ink (text) tokens

Defined per-theme in globals.css at line 4283+. These are the ink tier tokens:

| Token | Light | Dark | Neon | Ember | Ocean |
|-------|-------|------|------|-------|-------|
| `--ink` | `#0f172a` | `#f1f5f9` | `#e6edf3` | `#f5e6d0` | `#e0ecf4` |
| `--ink-sub` | `#334155` | `#94a3b8` | `#8b949e` | `#c49a6c` | `#7fa5c4` |
| `--ink-mute` | `#475569` | `#475569` | `#545d68` | `#8b6843` | `#4a7499` |
| `--ink-hair` | `var(--border)` | `#1e293b` | `var(--border)` | `var(--border)` | `var(--border)` |

Usage: `--ink` for primary text, `--ink-sub` for secondary/metadata, `--ink-mute` for placeholder/disabled, `--ink-hair` for hairline borders.

### 2.3 Accent tokens

| Token | Light | Dark | Neon | Ember | Ocean |
|-------|-------|------|------|-------|-------|
| `--accent` | `#0066ff` | `#0066ff` | `#00d4ff` | `#f59e0b` | `#06d6a0` |
| `--accent-soft` | `#eef5ff` | `rgba(0,102,255,0.18)` | `rgba(0,212,255,0.16)` | `rgba(245,158,11,0.16)` | `rgba(6,214,160,0.16)` |
| `--accent-fg` | `#ffffff` | `#ffffff` | `#0a0e12` | `#1c1208` | `#091521` |

Accent is used only for: focus rings, active links, selected-state indicators, and inline text links. CTAs use `--ink` (ink-black on paper-white), not accent. This matches Onyx convention.

### 2.4 Semantic tokens

| Token | Light value | Purpose |
|-------|-------------|---------|
| `--success` | `#059669` | Success state |
| `--warning` | `#d97706` | Warning state |
| `--error` | `#dc2626` | Error/danger state |
| `--filter-active` | `rgba(0,102,255,0.1)` | Active filter chip bg |
| `--filter-active-border` | `rgba(0,102,255,0.5)` | Active filter chip border |
| `--filter-active-text` | `#0048bb` | Active filter chip text |

### 2.5 Sidebar layout tokens

From globals.css (these are already set; filecloud reads them directly):

```css
--app-sidebar-width: 264px;
--app-sidebar-inset: 8px;        /* left inset from viewport edge */
--app-sidebar-inset-top: 16px;   /* top inset */
--app-sidebar-inset-bottom: 16px; /* bottom inset */
--app-page-gutter-x: 40px;       /* content area horizontal padding */
```

### 2.6 Typography tokens

```css
--font-page:    "PP Pangaia", "PP Neue York", Georgia, serif;    /* page/folder titles */
--font-subhead: "PP Neue York", Georgia, serif;                  /* card titles, modal headers */
--font-body:    "Bricolage Grotesque", system-ui, sans-serif;    /* all UI text */
--font-mono:    "JetBrains Mono", ui-monospace, monospace;       /* paths, code, filenames */
--font-logo:    "JetBrains Mono", ui-monospace, monospace;       /* "filecloud" wordmark */
```

Type scale (from globals.css):

```css
--text-display: 28px;   /* folder name in hero */
--text-h1:      20px;   /* section headers */
--text-h2:      16px;   /* card/panel titles */
--text-body-lg: 14px;   /* file list primary text */
--text-body:    13px;   /* canonical UI body */
--text-body-sm: 12px;   /* metadata: size, date */
--text-meta:    11px;   /* column headers, eyebrows */
--text-pill:    10px;   /* status tags, badges */
```

Letter-spacing:
```css
--track-section: 0.14em;   /* column header labels (uppercase) */
--track-editorial: 0.06em; /* section eyebrows */
```

### 2.7 Status tint tokens (used for file status, share state)

```css
--status-success-bg: rgba(34,197,94,0.1)
--status-success-border: rgba(34,197,94,0.25)
--status-success-fg: #047857

--status-error-bg: rgba(239,68,68,0.1)
--status-error-fg: #b91c1c

--status-warning-bg: rgba(245,158,11,0.1)
--status-warning-fg: #b45309

--status-info-bg: rgba(59,130,246,0.1)
--status-info-fg: #0052cc
```

---

## 3. Typography detail

### 3.1 Font roles

- **PP Pangaia** (`--font-page`): Folder/page title at `--text-display`, PP Neue York is the licensed display. Used for the current directory name displayed as the content area header.
- **PP Neue York** (`--font-subhead`): Modal titles, panel headers, card titles.
- **Bricolage Grotesque** (`--font-body`): Everything else - sidebar labels, list rows, metadata, buttons, inputs, breadcrumbs.
- **JetBrains Mono** (`--font-mono` / `--font-logo`): File paths, the "filecloud" wordmark, breadcrumb path segments, file extension badges, code preview.

### 3.2 Font loading strategy

PP Pangaia and PP Neue York are licensed Pangram Pangram fonts. They are not freely available via Google Fonts. Two options:

**Option A (preferred if fonts are available):** Self-host the woff2 files in `/public/fonts/` copied from the Opbox bundle. Opbox's public/fonts/blank-slate/ directory contains these files. No external request needed at runtime.

**Option B (fallback):** Georgia covers PP Neue York's role acceptably. System-ui covers Bricolage. JetBrains Mono is loaded from Google Fonts in globals.css (available offline via the existing import). This matches the globals.css fallback chain exactly.

See DEC-FC-011 in the decision ledger.

### 3.3 No topbar

There is no fixed topbar. Navigation context (breadcrumb, search, upload) lives in the content area header, which scrolls with the content until the first 64px, then sticks. This matches the Opbox DualRailLayout pattern.

---

## 4. Theme support

Filecloud supports all five Onyx themes out of the box. The theme is applied as a class on `<html>`. No additional CSS is required.

| Class | Scheme | Accent |
|-------|--------|--------|
| (none) | Light, white paper | Blue `#0066ff` |
| `.dark` | Dark, near-black | Blue `#0066ff` |
| `.neon` | Dark, cyberpunk | Cyan `#00d4ff` |
| `.ember` | Dark, warm amber | Amber `#f59e0b` |
| `.ocean` | Dark, deep blue | Teal `#06d6a0` |

The user's selected theme is stored in localStorage (`filecloud-theme`) and applied before first paint to avoid flash. The settings panel exposes a theme switcher. Default for first-time users is `.dark`.

---

## 5. Shell layout

### 5.1 Desktop ASCII diagram

```
+--viewport-----------------------------------------------------------------+
|                                                                           |
|  +--sidebar (264px, inset 8px left, 16px top/bottom, radius 12px)-----+  |
|  |                                                                     |  |
|  |  filecloud                  (JetBrains Mono, --ink, --text-body)   |  |
|  |  ─────────────────────────────────────────────────────────────      |  |
|  |  [search]                                                           |  |
|  |  ─────────────────────────────────────────────────────────────      |  |
|  |  ▶ The-Atrophied-Mind                                               |  |
|  |    ▶ AI                                                             |  |
|  |    ▶ Archives                                                       |  |
|  |    ▷ Audio                                                          |  |
|  |                                                                     |  |
|  |  ─────────────────────────────────────────────────────────────      |  |
|  |  ★ Starred                                                          |  |
|  |  ⏱ Recent                                                           |  |
|  |  🗑 Trash                                                            |  |
|  +---------------------------------------------------------------------+  |
|                                                                           |
|  +--content area (flex-1, left margin = sidebar + gap)------------------+  |
|  |  [ Quick access bar - recent/starred tiles ]                         |  |
|  |  ─────────────────────────────────────────────────────────────────   |  |
|  |  AI /                                        [list] [grid] [upload]  |  |
|  |  ─────────────────────────────────────────────────────────────────   |  |
|  |  NAME ↑         SIZE      MODIFIED      TYPE     TAGS    STATUS      |  |
|  |  ─────────────────────────────────────────────────────────────────   |  |
|  |  📁 Consulting    —        2 days ago    folder   work           …   |  |
|  |  📄 notes.md      12 KB    1 Jun 2026    md       ideas          …   |  |
|  +-----------------------------------------------------------------------+  |
+-----------------------------------------------------------------------------------+
```

### 5.2 Sidebar geometry

- Width: `var(--app-sidebar-width)` = 264px
- Inset from left viewport edge: `var(--app-sidebar-inset)` = 8px
- Inset top: `var(--app-sidebar-inset-top)` = 16px
- Inset bottom: `var(--app-sidebar-inset-bottom)` = 16px
- Border radius: 12px (matches Opbox ResizableSidebar)
- Background: `var(--sidebar-bg)` = `var(--bg-primary)`
- Border: 1px solid `var(--sidebar-border)` = `var(--border)`
- The sidebar floats - it does not span the full viewport height. The area behind it (and behind the content gap) shows `var(--app-canvas)`.
- The sidebar is position: fixed, not sticky. Content area has padding-left: calc(264px + 8px + 8px) = 280px plus --app-page-gutter-x.

### 5.3 No topbar

There is no topbar. The sidebar contains the wordmark, search trigger, and navigation. The content area header row contains: breadcrumb, view toggle, upload button.

### 5.4 Content area

- Background: `var(--bg-primary)` (white or equivalent per theme)
- Outer page (behind sidebar): `var(--app-canvas)`
- Horizontal padding: `var(--app-page-gutter-x)` = 40px
- Content area is a full-height scrollable column with the sticky content header at top.

---

## 6. Component specifications

### 6.1 Sidebar

Structure (top to bottom):
1. Wordmark: "filecloud" in `--font-logo`, `--text-body`, `--ink`. Left-aligned, 16px padding.
2. Horizontal divider: 1px `--ink-hair`, inset 12px from sidebar edges.
3. Search trigger: 32px tall, `--bg-secondary` fill, `--border` border, rounded 6px. Placeholder "Search..." in `--ink-mute`. Keyboard hint "⌘K" right-aligned in `--ink-mute`.
4. Horizontal divider.
5. Folder tree: recursive tree of directories. No files shown in sidebar.
6. Horizontal divider.
7. Bottom links: Starred, Recent, Trash. Each a 32px row, icon + label, `--ink-sub` text, hover `--bg-tertiary`.

Folder tree rows:
- Height: 28px
- Icon: Lucide `Folder` / `FolderOpen` at 14px, 1.5px stroke, `--ink-mute`
- Label: `--font-body`, `--text-body` (13px), `--ink-sub`
- Indent: 16px per level
- Current directory: `--bg-tertiary` background, `--ink` text
- Hover: `--bg-secondary` background
- Chevron: `ChevronRight` rotates 90deg when expanded, 80ms ease-out

### 6.2 Quick access bar

Horizontal scrollable strip at the top of the content area, below the content header.

- Height: 80px
- Each tile: 120px wide, rounded 8px, `--bg-secondary` background, `--border` border
- Hover: `--border-hover` border, `--bg-tertiary` background
- Content: file type icon (20px) + filename (2-line clamp, `--text-body-sm`, `--ink-sub`)
- Two sections: "Recent" (clock icon label) and "Starred" (star icon label), separated by a 1px vertical divider
- Scrolls horizontally if items overflow; no scrollbar visible

### 6.3 File list (list view)

Column grid:

```
[checkbox] [icon] [NAME          ] [SIZE  ] [MODIFIED   ] [TYPE] [TAGS   ] [...]
```

Column headers:
- Height: 32px
- Text: `--text-meta` (11px), `--track-section` (0.14em), uppercase, `--ink-mute`
- Background: `--bg-secondary`
- Border-bottom: 1px `--ink-hair`
- Sorted column: accent-coloured bottom indicator (2px, `--accent`)

Row spec:
- Height: 36px
- Border-bottom: 1px `--ink-hair`
- Hover background: `--bg-tertiary`
- Selected background: `--accent-soft`, left border 2px `--accent`
- Transition: background 80ms ease-out

Column widths:
- Checkbox: 36px (appears on hover or when any row selected)
- Icon: 32px - Lucide at 14px, 1.5px stroke, `--ink-mute`
- Name: flex-1 (fills remaining space). Font: `--font-body`, `--text-body-lg` (14px), `--ink`. Truncate with ellipsis.
- Size: 72px, right-aligned, `--text-body-sm` (12px), `--ink-sub`
- Modified: 96px, right-aligned, `--text-body-sm` (12px), `--ink-sub`. "2 days ago" for < 7 days, "12 Jun 2026" for older.
- Type: 56px, `--text-pill` (10px), `--ink-mute`
- Tags: 120px, up to 3 pills. Pill: `--bg-tertiary` bg, `--border` border, `--ink-sub` text, `--text-pill` (10px), rounded 999px, 4px horizontal padding.
- Actions (...): 32px, `MoreHorizontal` icon, appears on row hover

File type icons (Lucide, 14px, 1.5px stroke):
- Folder: `Folder` / `FolderOpen`
- PDF: `FileText` in `--status-error-fg`
- Image: `FileImage` in `--status-info-fg`
- Video: `FileVideo` in `--status-warning-fg`
- Audio: `Music` in `--accent`
- Code: `FileCode` in `--status-success-fg`
- Archive: `Archive` in `--ink-sub`
- Text/Markdown: `FileText` in `--ink-sub`
- Other: `File` in `--ink-mute`

Multi-select bulk action bar:
- Fixed at bottom of content area
- Background: `--ink` (ink-black), text: `--bg-primary` (white)
- "N selected" + Download ZIP / Move / Delete buttons
- Dismiss: `X` on right

### 6.4 File grid (grid view)

- Column count: 2 at < 640px, 3 at 640-1024px, 4 at 1024-1280px, 5 at 1280-1536px, 6 at > 1536px
- Card size: fill grid column, aspect ratio 3:4 (portrait card)
- Card background: `--bg-secondary`, border: 1px `--border`, radius: 8px
- Hover: border `--border-hover`, shadow `0 2px 8px rgba(0,0,0,0.08)`
- Selected: border 2px `--accent`, background `--accent-soft`
- Thumbnail area (top 60% of card): image, video poster, PDF first page, or type icon on `--bg-tertiary`
- Card footer (bottom 40%): filename (2-line clamp, `--text-body`, `--ink`), size (`--text-body-sm`, `--ink-mute`)
- Checkbox appears top-left on hover or when any card selected

### 6.5 Preview panel

- Slides in from the right, 400px wide on desktop
- Overlay: does not push content; sits on top
- Background: `--bg-primary`, border-left: 1px `--border`
- Shadow: `-4px 0 24px rgba(0,0,0,0.12)`
- Slide-in animation: 200ms ease-out translateX(0) from translateX(400px)
- Backdrop: none (panel is transparent-edge, not dimming)

Panel header (48px):
- Filename in `--font-body`, `--text-body-lg`, `--ink`
- Close button (X): right side, `--ink-mute`, hover `--ink`
- Download button: icon + label, secondary style
- Share button: icon + label, secondary style

Panel body (flex-1, scrollable):
- PDF: pdf.js embed with page nav, zoom, text selection
- Image: `<img>` with CSS pan/zoom (mouse wheel + drag on desktop, pinch on mobile)
- Markdown: rendered HTML, `--font-body` for body text, `--font-mono` for code blocks, `--ink` foreground throughout
- Code/text: Prism.js with auto-language detection from extension. Line numbers. `--font-mono`, `--text-body-sm`.
- Video: `<video>` with native controls styled to Onyx (custom CSS overlay)
- Audio: `<audio>` with custom progress bar, `--accent` fill, `--bg-secondary` track
- Unsupported: centered icon + "No preview available" in `--ink-mute` + Download button

Panel footer (40px):
- Previous / Next arrows (ChevronLeft / ChevronRight), `--ink-sub`
- "3 of 47" label, `--text-body-sm`, `--ink-mute`

### 6.6 Search overlay

Triggered by Cmd+K / Ctrl+K or clicking the sidebar search trigger.

- Full-screen overlay, `--bg-primary` at 96% opacity with `backdrop-filter: blur(8px)`
- Spotlight glass style using `--spotlight-glass` and `--spotlight-glass-border` tokens
- Input: 56px tall, `--font-body`, `--text-h1` (20px), `--ink`. Auto-focused.
- Filter chips below input: All / Folders / Images / Videos / Audio / PDFs / Documents / Code. Active chip uses `--filter-active` / `--filter-active-border` / `--filter-active-text` tokens.
- Results list: same row style as FileList, keyboard navigable (arrow keys + Enter)
- Empty state: "Type to search filenames..." in `--ink-mute`
- Recent searches shown when input is empty
- Escape to close

### 6.7 Upload zone

- Drag target: entire content area (not sidebar)
- Drag-over state: content area covered by `--bg-secondary` at 90% opacity, dashed 2px border `--accent`, centered label "Drop files here" in `--font-body`, `--text-h1`, `--ink`
- Progress panel: slides up from bottom-right corner, 320px wide, `--bg-primary` bg, `--border` border, 8px radius
  - Each file: filename (`--text-body`, `--ink`) + progress bar (`--accent` fill, `--bg-secondary` track) + percentage
  - On complete: tick icon in `--success`

### 6.8 Share modal

- Centered modal, 480px wide, `--bg-primary` bg, `--border` border, 12px radius
- Shadow: `0 8px 32px rgba(0,0,0,0.16)`
- Backdrop: `rgba(0,0,0,0.4)` overlay

Contents:
1. Title: "Share link" in `--font-subhead`, `--text-h1`, `--ink`
2. Generated URL: monospace input, read-only, with Copy button. Font: `--font-mono`, `--text-body`.
3. Expiry picker: segmented control (1 hour / 1 day / 7 days / No expiry). Active segment: `--ink` background, `--bg-primary` text.
4. Password toggle: checkbox + optional password input. `--bg-secondary` fill input, `--border` border.
5. Active links list (if any): each shows path, expiry date, Revoke button.

CTA button: "Copy link" - full-width, `--ink` background, `--bg-primary` text, 36px height, 6px radius.

### 6.9 Shared directory view (unauthenticated)

Stripped-down FileList with:
- No sidebar
- No upload
- No file operations (rename, move, delete)
- Header: "Shared: [foldername]" + expiry notice in `--status-warning-bg` / `--status-warning-fg`
- Download and preview only
- Same Onyx theme tokens - uses the default (light) theme unless a cookie carries a preference

---

## 7. Mobile layout (iPhone over Tailscale)

Breakpoint: < 768px.

- Sidebar hidden by default; triggered via a hamburger icon (Menu, 20px) top-left of content
- Sidebar slides in as a bottom-anchored sheet (80vh, rounded top corners 12px) over the content
- Sheet backdrop: `rgba(0,0,0,0.4)`, tap to close
- Content area: full width, padding-left reduces to 16px
- Upload button: becomes a floating action button (bottom-right, 56px, rounded 50%, `--accent` background, `--accent-fg` icon)
- Touch targets: minimum 44px height for all interactive rows
- Swipe right on a file row: reveals Download action (green strip, `--success`)
- Swipe left on a file row: reveals Delete action (red strip, `--error`)
- Preview panel: full-screen when on mobile (100vw, 100dvh), close via top-right X
- Pinch-to-zoom on images in preview

---

## 8. States for all interactive elements

| Element | Default | Hover | Active/Pressed | Selected | Disabled |
|---------|---------|-------|----------------|----------|---------|
| List row | transparent bg | `--bg-tertiary` bg | `--bg-tertiary` bg + scale 0.998 | `--accent-soft` bg + 2px `--accent` left border | `--ink-mute` text, no hover |
| Grid card | `--bg-secondary` bg, `--border` border | `--border-hover` border, shadow | `--border-hover` border | `--accent-soft` bg, 2px `--accent` border | 50% opacity |
| Button (primary/CTA) | `--ink` bg, `--bg-primary` text | `color-mix(in srgb, --ink 85%, --bg-primary)` bg | scale 0.97 | N/A | `--ink-mute` bg |
| Button (secondary) | transparent bg, `--border` border, `--ink` text | `--bg-secondary` bg | `--bg-tertiary` bg | N/A | `--ink-mute` border and text |
| Link | `--accent` text | underline | underline + scale 0.98 | N/A | `--ink-mute` text |
| Icon button | `--ink-mute` icon | `--ink-sub` icon, `--bg-secondary` bg | `--ink` icon | N/A | `--ink-mute` icon, no hover |
| Input | `--bg-secondary` bg, `--border` border, `--ink` text | `--border-hover` border | focus: 2px `--accent` ring | N/A | `--bg-tertiary` bg, `--ink-mute` text |
| Tag pill | `--bg-tertiary` bg, `--border` border | `--border-hover` border | N/A | `--accent-soft` bg | N/A |
| Checkbox | `--border` border, transparent bg | `--border-hover` border | indeterminate: `--accent` bg | checked: `--accent` bg, white tick | 50% opacity |

---

## 9. Icons

Lucide React throughout (MIT licensed, consistent with Opbox). Icon size: 14px, stroke-width: 1.5px for list rows and sidebar; 20px, 1.5px for modal headers and FAB.

Key icons:

| Use | Icon |
|-----|------|
| Folder | `Folder` / `FolderOpen` |
| PDF | `FileText` |
| Image | `FileImage` |
| Video | `FileVideo` |
| Audio | `Music` |
| Code | `FileCode` |
| Archive | `Archive` |
| Generic file | `File` |
| Download | `Download` |
| Upload | `Upload` |
| Share | `Share2` |
| Tag | `Tag` |
| Delete | `Trash2` |
| Rename | `Pencil` |
| Move | `FolderInput` |
| Copy | `Copy` |
| Actions menu | `MoreHorizontal` |
| Expand | `ChevronRight` (rotates 90deg) |
| Back/Forward | `ArrowLeft` / `ArrowRight` |
| Close | `X` |
| Menu (hamburger) | `Menu` |
| Search | `Search` |
| Starred | `Star` / `StarOff` |
| Recent | `Clock` |
| Trash | `Trash2` |
| Zoom in/out | `ZoomIn` / `ZoomOut` |
| Settings | `Settings` |
| Check | `Check` |
| Link (share URL) | `Link` |

---

## 10. URL structure

| URL | Content |
|-----|---------|
| `/` | Redirect to `/browse/` |
| `/login` | Login page |
| `/browse/*` | Directory listing at path |
| `/preview/*` | Parent listing + triggered preview |
| `/search?q=&type=` | Search results |
| `/share/:token` | Public share view (no auth) |
| `/starred` | Starred files listing |
| `/recent` | Recent files listing |
| `/trash` | Trash listing |
| `/settings` | Settings (theme, view defaults) |
| `/api/*` | Backend API routes (Hono) |

---

## 11. Animations

All from tailwind.config.ts:

| Event | Animation | Duration |
|-------|-----------|---------|
| Sidebar open/close (mobile) | `slide-up` | 200ms ease-out |
| Preview panel slide in/out | translateX | 200ms ease-out |
| Row hover background | background-color | 80ms |
| Modal open | `scale-in` (0.95 to 1.0) + `fade-in` | 150ms ease-out |
| Upload progress panel | `fade-in-up` | 300ms ease-out |
| Search overlay | `fade-in` | 150ms ease-out |
| Folder tree chevron | rotate 90deg | 80ms ease-out |
