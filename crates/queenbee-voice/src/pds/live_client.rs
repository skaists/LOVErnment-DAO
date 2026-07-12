//! The live PdsClient — talks to an ATProto PDS.
//!
//! `social.skaists.alpha.audit.entry` is `key: tid`. ATProto's
//! `com.atproto.repo.listRecords` returns records by collection,
//! keyed by their tid rkey; there is **no server-side query over a
//! record's fields.** So `find_entry_by_derivation_input` is a
//! **client-side field scan**: paginate `listRecords`, compare each
//! record's `derivationInput` field to the query, return the first
//! match (pending or finalized — both block, per the durable-lock
//! law). `None` only after **all pages are exhausted** with no match.
//!
//! A premature `None` — from an unfollowed cursor, a dropped page,
//! an early return, or a **swallowed transport error** — reopens the
//! exact window the d008 arc closed. G-Q: fail closed, always.
//!
//! **Canonical `derivationInput` form** (F-4): `repo@sha` where `repo`
//! is `owner/name` (GitHub canonical) and `sha` is the full 40-char
//! lowercase hex git object ID. One form, enforced at the write
//! boundary (pipeline `run()` constructs it). The read path uses
//! exact-string `==` with no normalization — no guessing at equivalence.

use crate::pipeline::{AuditEntry, PendingEntry, PdsClient, ScanError};

// ─── HTTP boundary ──────────────────────────────────────────────

/// HTTP boundary — abstracts `com.atproto.repo.listRecords` for the
/// `social.skaists.alpha.audit.entry` collection.
///
/// The live client depends on this trait; tests inject a fake that
/// serves canned pages. **No network in any test.**
pub trait AuditRecordSource {
    /// Fetch one page of audit-entry records.
    ///
    /// `cursor: None` starts from the beginning; `Some(cursor)`
    /// continues from the given cursor. Returns `Err` on transport
    /// failure.
    fn list_audit_records(&self, cursor: Option<String>) -> Result<RecordsPage, String>;
}

/// One page of results from `listRecords`.
#[derive(Debug, Clone)]
pub struct RecordsPage {
    /// Records on this page.
    pub records: Vec<AuditRecord>,
    /// Cursor for the next page; `None` means no more pages.
    pub cursor: Option<String>,
}

/// One record from the audit-entry collection.
#[derive(Debug, Clone)]
pub struct AuditRecord {
    /// The record's key (tid rkey). **NOT the match field** — the scan
    /// reads `derivationInput` from the record's value, never the rkey.
    /// A record whose rkey coincidentally resembles the query must not
    /// be treated as a match.
    pub rkey: String,
    /// The raw record value. The live client parses fields from this.
    pub value: serde_json::Value,
}

// ─── Pagination bound (F-5) ────────────────────────────────────

/// Maximum pages fetched in a single scan before returning
/// `Err(ScanError::Indeterminate)`. Guards against cyclic/repeating
/// cursors. 1000 pages × ~100 records/page = 100k entries — far beyond
/// any reasonable audit trail.
const MAX_PAGES: u32 = 1000;

// ─── LivePdsClient ──────────────────────────────────────────────

/// The live PdsClient. Generic over the HTTP boundary so tests can
/// inject a mock transport.
pub struct LivePdsClient<S: AuditRecordSource> {
    source: S,
}

impl<S: AuditRecordSource> LivePdsClient<S> {
    pub fn new(source: S) -> Self {
        Self { source }
    }

    /// Borrow the underlying source — for test inspection.
    pub fn source(&self) -> &S {
        &self.source
    }
}

// ─── Record parsing ────────────────────────────────────────────

/// Parse a raw record value into an `AuditEntry` (strict — all fields
/// required). Returns `None` if any required field is missing.
fn parse_audit_entry(record: &AuditRecord) -> Option<AuditEntry> {
    let v = &record.value;
    Some(AuditEntry {
        pending: PendingEntry {
            derivation_input: v.get("derivationInput")?.as_str()?.to_string(),
            input_digest: v.get("inputDigest")?.as_str()?.to_string(),
            adapter_class: v.get("adapterClass")?.as_str()?.to_string(),
            adapter_digest: v.get("adapterDigest")?.as_str()?.to_string(),
            model_digest: v.get("modelDigest")?.as_str()?.to_string(),
            prompt_digest: v.get("promptDigest")?.as_str()?.to_string(),
            created_at: v.get("createdAt")?.as_str()?.to_string(),
        },
        post_uri: v
            .get("postUri")
            .and_then(|v| v.as_str())
            .map(String::from),
        post_cid: v
            .get("postCid")
            .and_then(|v| v.as_str())
            .map(String::from),
        failure_error: v
            .get("failureError")
            .and_then(|v| v.as_str())
            .map(String::from),
    })
}

// ─── PdsClient impl ────────────────────────────────────────────
//
// D-009a2 commit A: NAIVE implementation with TWO deliberate defects
// the test suite catches:
//   F-1: transport Err swallowed into Ok(None) — fail-open
//   F-2: full parse required before field match — partial records skipped
// Commit B fixes both: Err propagates, field match checked first.

impl<S: AuditRecordSource> PdsClient for LivePdsClient<S> {
    fn find_entry_by_derivation_input(
        &self,
        derivation_input: &str,
    ) -> Result<Option<AuditEntry>, ScanError> {
        let mut cursor = None;
        let mut pages_fetched = 0u32;
        loop {
            // NAIVE (commit A): swallows transport error into Ok(None).
            // G-Q says this must be Err(ScanError::Transport(..)).
            let page = match self.source.list_audit_records(cursor) {
                Ok(p) => p,
                Err(_) => return Ok(None), // BUG: fail-open
            };

            pages_fetched += 1;
            if pages_fetched > MAX_PAGES {
                return Err(ScanError::Indeterminate(format!(
                    "pagination bound ({MAX_PAGES}) exceeded — possible cyclic cursor"
                )));
            }

            for record in &page.records {
                // NAIVE (commit A): requires full parse (all 7 fields)
                // before checking derivationInput match. A record whose
                // derivationInput matches but is missing a sibling field
                // (e.g. crash-mid-flight missing createdAt) parses to
                // None and is silently skipped — invisible to the lock.
                // Commit B: check derivationInput field FIRST.
                if let Some(entry) = parse_audit_entry(record) {
                    if entry.pending.derivation_input == derivation_input {
                        return Ok(Some(entry));
                    }
                }
            }

            match page.cursor {
                Some(next) => cursor = Some(next),
                None => return Ok(None),
            }
        }
    }

    fn create_pending_entry(&mut self, _key: &str, _entry: &PendingEntry) -> Result<(), String> {
        Err(
            "D-009a2: write operations not implemented — live credentials are D-009b territory"
                .to_string(),
        )
    }

    fn submit_post(&mut self, _text: &str) -> Result<(String, String), String> {
        Err(
            "D-009a2: write operations not implemented — live credentials are D-009b territory"
                .to_string(),
        )
    }

    fn finalize_entry(
        &mut self,
        _key: &str,
        _uri: &str,
        _cid: &str,
    ) -> Result<(), String> {
        Err(
            "D-009a2: write operations not implemented — live credentials are D-009b territory"
                .to_string(),
        )
    }

    fn remove_entry(&mut self, _key: &str) -> Result<(), String> {
        Err(
            "D-009a2: write operations not implemented — live credentials are D-009b territory"
                .to_string(),
        )
    }

    fn mark_entry_failed(&mut self, _key: &str, _error: &str) -> Result<(), String> {
        Err(
            "D-009a2: write operations not implemented — live credentials are D-009b territory"
                .to_string(),
        )
    }
}
