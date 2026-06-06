-- Neutral annotation + review store for The Realm Law Reports & Gazette.
--
-- PORTED from acmeco (the concepts, not the server), NEUTRALISED so it runs standalone and can
-- later fold into acmeco (schema is a strict SUBSET of acmeco's):
--   pdf_annotation  <- acmeco kernel/migrations/0015_exhibit.sql  (the [0..1] geometry model)
--   review_item     <- acmeco kernel/migrations/0005_control_primitives.sql + verbs/review.rs semantics
-- Dropped: workspace_id / RLS, the file_record FK (binary/encryption), matter_id, gate_id.
-- The [0..1] normalized-geometry contract + CHECK backstops are kept EXACTLY (acmeco INV-6:
-- geometry stored normalized; pixel positions computed at render, never stored).

CREATE TABLE IF NOT EXISTS pdf_annotation (
  id            TEXT PRIMARY KEY,                 -- 'anno_<n>'
  doc_ref       TEXT NOT NULL,                    -- logical corpus key: 'judgment:2026-realm-sc-1' | 'bill:11'
  type          TEXT NOT NULL CHECK (type IN ('BOUNDING_BOX','ARROW','COMMENT')),
  page_number   INTEGER NOT NULL CHECK (page_number >= 1),
  x REAL NOT NULL CHECK (x >= 0 AND x <= 1),
  y REAL NOT NULL CHECK (y >= 0 AND y <= 1),
  w REAL NOT NULL CHECK (w >= 0 AND w <= 1),
  h REAL NOT NULL CHECK (h >= 0 AND h <= 1),
  end_x REAL CHECK (end_x IS NULL OR (end_x >= 0 AND end_x <= 1)),
  end_y REAL CHECK (end_y IS NULL OR (end_y >= 0 AND end_y <= 1)),
  color       TEXT,
  label       TEXT,
  criterion   TEXT,                               -- first-class facet (here: section / paragraph anchor)
  comment     TEXT,
  created_by  TEXT NOT NULL,
  author_kind TEXT NOT NULL DEFAULT 'HUMAN' CHECK (author_kind IN ('HUMAN','AGENT','SYSTEM')),
  created_at  TEXT NOT NULL,
  review_id   TEXT                                -- the review raised by this annotation (if any)
);
CREATE INDEX IF NOT EXISTS ix_annotation_doc ON pdf_annotation (doc_ref, page_number);

CREATE TABLE IF NOT EXISTS review_item (
  id            TEXT PRIMARY KEY,                 -- 'rev_<n>'
  lane          TEXT NOT NULL CHECK (lane IN ('JUDGMENT_COMMENT','BILL_COMMENT')),
  status        TEXT NOT NULL DEFAULT 'OPEN' CHECK (status IN ('OPEN','CLAIMED','RESOLVED','DISMISSED','EXPIRED')),
  opened_by     TEXT NOT NULL,
  opened_by_source TEXT NOT NULL DEFAULT 'web',   -- 'web' | 'cli' | 'agent'
  subject_type  TEXT NOT NULL DEFAULT 'pdf_annotation',
  subject_id    TEXT NOT NULL,                    -- the annotation id
  doc_ref       TEXT NOT NULL,                    -- denormalised for grouping the queue by document
  payload       TEXT NOT NULL DEFAULT '{}',       -- JSON: {commentText, anchor:{page,x,y,w,h}, criterion}
  sla_due_at    TEXT,
  claimed_by    TEXT,
  claimed_at    TEXT,
  resolved_by   TEXT,
  resolution    TEXT,
  resolution_note TEXT,
  resolved_at   TEXT,
  created_at    TEXT NOT NULL,
  updated_at    TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS ix_review_lane_status ON review_item (lane, status);
-- Idempotent open: at most ONE non-terminal review per (lane, subject) - acmeco US-REVIEW-01.
CREATE UNIQUE INDEX IF NOT EXISTS uq_review_one_open_per_subject
  ON review_item (lane, subject_type, subject_id) WHERE status IN ('OPEN','CLAIMED');
