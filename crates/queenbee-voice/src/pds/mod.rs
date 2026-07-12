//! Live PDS client — ATProto audit-entry read path.
//!
//! D-009a2: fail-closed read scan. The `find_entry_by_derivation_input`
//! method is a paginated client-side field scan over
//! `social.skaists.alpha.audit.entry` records. Transport errors
//! propagate as `Err(ScanError)` — they are never swallowed into
//! `Ok(None)`. G-Q: indeterminate silence is never permission to speak.

pub mod live_client;
