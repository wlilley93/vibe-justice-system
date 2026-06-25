//! Citation allocation (#14).
//!
//! The live allocator is `vjs_lawpack::LawpackValidator::live_citation_max` +
//! `parse_citation`, which reads the LIVE persisted register from the lawpack
//! (PC-13 D2 / ACT-004:s8). The former in-memory `CitationRegistry` /
//! `CitationSeries` / `Citation` types always started a series at its genesis (they
//! never loaded the register) and were superseded by that allocator; they have been
//! removed to leave one source of truth for citation numbers.
