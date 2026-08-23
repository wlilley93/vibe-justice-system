//! Citation allocation (#14).
//!
//! The live allocator is `vjs_lawpack::LawpackValidator::live_citation_max` +
//! `parse_citation`. The live persisted register PC-13 D2 requires it to read is the
//! body of governed records itself: every record under `lawpack/v2`, `.vjs/orders` and
//! `.vjs/court` carries its own top-level `citation:`, and the allocator takes the
//! highest allocated N off them (PC-13 D2 / ACT-004:s8). Not a citator index, and not
//! the lawpack alone. The former in-memory `CitationRegistry` /
//! `CitationSeries` / `Citation` types always started a series at its genesis (they
//! never loaded the register) and were superseded by that allocator; they have been
//! removed to leave one source of truth for citation numbers.
