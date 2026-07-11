//! Performance-set validation — product oracle for SPEC-performance-set v0.1.
//!
//! Source of truth: `specs/SPEC-performance-set.md` @ 60ebd9cd…
//! Lexicon: `social.skaists.alpha.performance.set` (+ `.defs`, `.setStatus`)
//!
//! Every invariant in §4 of the spec that is machine-decidable from the
//! record's own fields is enforced here. Rules that require auth context,
//! repo state, or consumer behavior are documented as excluded.
//!
//! ## Enforced via validate_set (12 rules)
//!
//! SET-1 (position total & dense) · SET-2 (time monotonic) · SET-3 (set
//! contains its tracks) · SET-4 (cueTime ±1s) · SET-5 (embed, never
//! reference fm.teal.*) · SET-6 (no floats — bpmMilli integer) · SET-11
//! (disclosure affirmative) · SET-12 (mixed is derived, not stored) ·
//! SET-13 (venue name required) · §5 (playedTime required at ingest) ·
//! §5 (artists required, minLength 1) · §5 (artists non-empty array).
//!
//! ## Enforced via validate_set_status (1 rule)
//!
//! STATUS-1 (setStatus is state, never append-only — `supersedes` rejected).
//! The remaining STATUS rules (STATUS-2 stale-render, STATUS-3
//! status-to-set mutation) are consumer/process rules, not decidable from
//! the record's own fields, and are documented in the excluded list below.
//!
//! ## Excluded (not decidable from record fields)
//!
//! - SET-7 (authorship follows performance) — requires ATProto auth context.
//! - SET-8 (eventUri is enrichment) — no negative case; dangling is valid.
//! - SET-9 (supersession confined to one repo) — requires the author's DID,
//!   which is not a field on the record (it is the repo host, not the record).
//! - SET-10 (superseded record not deleted) — repo operation, not a record
//!   property.
//! - STATUS-2 (stale never renders as live) — consumer rendering rule.
//! - STATUS-3 (status never promoted to set by mutation) — process rule.

#![forbid(unsafe_code)]

use serde_json::Value;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

/// A named validation rule from the spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rule {
    Set1,
    Set2,
    Set3,
    Set4,
    Set5,
    Set6,
    Set11,
    Set12,
    Set13,
    /// §5 ingest: playRef missing playedTime.
    IngestPlayedTime,
    /// §5 ingest: playRef missing artists.
    IngestArtistsMissing,
    /// §5 ingest: playRef artists array empty (minLength 1).
    IngestArtistsEmpty,
    /// STATUS-1: setStatus carrying supersedes.
    Status1,
}

impl Rule {
    pub fn as_str(self) -> &'static str {
        match self {
            Rule::Set1 => "SET-1",
            Rule::Set2 => "SET-2",
            Rule::Set3 => "SET-3",
            Rule::Set4 => "SET-4",
            Rule::Set5 => "SET-5",
            Rule::Set6 => "SET-6",
            Rule::Set11 => "SET-11",
            Rule::Set12 => "SET-12",
            Rule::Set13 => "SET-13",
            Rule::IngestPlayedTime => "INGEST-playedTime",
            Rule::IngestArtistsMissing => "INGEST-artists-missing",
            Rule::IngestArtistsEmpty => "INGEST-artists-empty",
            Rule::Status1 => "STATUS-1",
        }
    }
}

/// Validate a `performance.set` record. Returns the list of violated rules;
/// empty means the record passes all machine-decidable checks.
pub fn validate_set(record: &Value) -> Vec<Rule> {
    // STUB — accepts everything. Commit A lands this as red-first baseline.
    // Commit B replaces this body with the real enforcement logic.
    let _ = record;
    Vec::new()
}

/// Validate a `performance.setStatus` record.
///
/// Currently enforces STATUS-1 only (setStatus must not carry `supersedes`).
/// STATUS-2 (stale-render) and STATUS-3 (status-to-set mutation) are consumer
/// and process rules — see the module-level excluded list.
pub fn validate_set_status(record: &Value) -> Vec<Rule> {
    // STUB — accepts everything. Commit A lands this as red-first baseline.
    // Commit B replaces this body with the real enforcement logic.
    let _ = record;
    Vec::new()
}

// ── internal helpers (used by the real implementation in commit B) ──

/// Parse an RFC 3339 timestamp to Unix epoch seconds.
fn parse_ts(s: &str) -> Option<i64> {
    OffsetDateTime::parse(s, &Rfc3339)
        .ok()
        .map(|dt| dt.unix_timestamp())
}

/// Check whether any at-uri string value in the JSON tree points at a
/// collection in the `fm.teal.*` namespace.
///
/// An at-uri is `at://authority/collection/rkey`. The authority may be a
/// DID (`did:plc:…`) or a handle. The collection is an NSID
/// (`ns.authority.name`). We check whether the collection component's
/// authority portion is `fm.teal` — i.e. the NSID starts with `fm.teal.`.
fn contains_teal_at_uri(value: &Value) -> bool {
    match value {
        Value::String(s) => is_teal_at_uri(s),
        Value::Array(a) => a.iter().any(contains_teal_at_uri),
        Value::Object(o) => o.values().any(contains_teal_at_uri),
        _ => false,
    }
}

/// True if `s` is an at-uri whose collection NSID is in the fm.teal namespace.
fn is_teal_at_uri(s: &str) -> bool {
    let rest = match s.strip_prefix("at://") {
        Some(r) => r,
        None => return false,
    };
    // authority is everything up to the first '/'
    let after_authority = match rest.find('/') {
        Some(i) => &rest[i + 1..],
        None => return false,
    };
    // collection is everything up to the next '/' (or end of string)
    let collection = match after_authority.find('/') {
        Some(i) => &after_authority[..i],
        None => after_authority,
    };
    // NSID structure: ns.authority.name — check if it starts with "fm.teal."
    collection.starts_with("fm.teal.")
}
