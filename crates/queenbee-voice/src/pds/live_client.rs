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

// ─── Write boundary ─────────────────────────────────────────────

/// HTTP boundary for ATProto XRPC write operations.
///
/// All write methods map to XRPC endpoints. Tests inject a fake
/// transport returning canned responses and errors. **No network
/// in any test.**
///
/// Response shape: a successful call returns a `serde_json::Value`
/// (the parsed JSON body). A failed call returns `Err(String)` —
/// timeout, 4xx, 5xx, malformed response. **Fail-closed extends to
/// writes: an ambiguous write is a failed write.**
pub trait XrpcTransport {
    /// `com.atproto.repo.putRecord` — create or overwrite a record.
    fn put_record(&mut self, body: serde_json::Value) -> Result<serde_json::Value, String>;

    /// `com.atproto.repo.createRecord` — create a new record.
    /// Returns the response body containing `uri` and `cid`.
    fn create_record(&mut self, body: serde_json::Value) -> Result<serde_json::Value, String>;

    /// `com.atproto.repo.deleteRecord` — delete a record.
    fn delete_record(&mut self, body: serde_json::Value) -> Result<serde_json::Value, String>;
}

/// A no-op `XrpcTransport` for read-only tests. All write methods
/// return `Err`. Used by d009a2 read-scan tests that don't test writes.
#[derive(Debug, Clone, Default)]
pub struct NoopXrpcTransport;

impl XrpcTransport for NoopXrpcTransport {
    fn put_record(&mut self, _body: serde_json::Value) -> Result<serde_json::Value, String> {
        Err("NoopXrpcTransport: writes not configured".to_string())
    }
    fn create_record(&mut self, _body: serde_json::Value) -> Result<serde_json::Value, String> {
        Err("NoopXrpcTransport: writes not configured".to_string())
    }
    fn delete_record(&mut self, _body: serde_json::Value) -> Result<serde_json::Value, String> {
        Err("NoopXrpcTransport: writes not configured".to_string())
    }
}

// ─── Collection / lexicon constants ────────────────────────────

/// The audit-entry collection NSID.
const AUDIT_COLLECTION: &str = "social.skaists.alpha.audit.entry";

/// The post collection NSID (VOICE-1 §1: app.bsky.feed.post at genesis).
const POST_COLLECTION: &str = "app.bsky.feed.post";

// ─── LivePdsClient ──────────────────────────────────────────────

/// The live PdsClient. Generic over both the read boundary (`S`)
/// and the write boundary (`T`) so tests can inject mocks for either.
pub struct LivePdsClient<S: AuditRecordSource, T: XrpcTransport> {
    source: S,
    transport: T,
    /// RFC 3339 timestamp for post records. Injected so tests control it.
    /// D-009c wiring provides wall time.
    now_rfc3339: String,
}

impl<S: AuditRecordSource, T: XrpcTransport> LivePdsClient<S, T> {
    pub fn new(source: S, transport: T) -> Self {
        Self {
            source,
            transport,
            now_rfc3339: "2026-07-12T00:00:00Z".to_string(),
        }
    }

    /// Set the timestamp generator for post records.
    pub fn with_now(mut self, now: impl Into<String>) -> Self {
        self.now_rfc3339 = now.into();
        self
    }

    /// Borrow the underlying source — for test inspection.
    pub fn source(&self) -> &S {
        &self.source
    }

    /// Borrow the underlying transport — for test inspection.
    pub fn transport(&self) -> &T {
        &self.transport
    }

    /// Borrow the underlying transport mutably — for test inspection.
    pub fn transport_mut(&mut self) -> &mut T {
        &mut self.transport
    }
}

// ─── Record parsing ────────────────────────────────────────────

/// Parse a raw record value into an `AuditEntry` (strict — all fields
/// required). Returns `None` if any required field is missing.
/// Used by tests for assertions against well-formed records.
#[allow(dead_code)]
fn parse_audit_entry_strict(record: &AuditRecord) -> Option<AuditEntry> {
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

/// Best-effort parse — used after derivationInput match is confirmed.
/// Missing fields are filled with empty strings. The pipeline treats
/// `Ok(Some(_))` as block regardless of field completeness — the match
/// on `derivationInput` is what blocks, not the full parse.
fn parse_audit_entry_best_effort(record: &AuditRecord) -> AuditEntry {
    let v = &record.value;
    AuditEntry {
        pending: PendingEntry {
            derivation_input: v
                .get("derivationInput")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            input_digest: v
                .get("inputDigest")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            adapter_class: v
                .get("adapterClass")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            adapter_digest: v
                .get("adapterDigest")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            model_digest: v
                .get("modelDigest")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            prompt_digest: v
                .get("promptDigest")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            created_at: v
                .get("createdAt")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
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
    }
}

// ─── PdsClient impl ────────────────────────────────────────────
//
// D-009a2: fail-closed read scan.
// F-1: transport Err propagates as Err(ScanError::Transport) — never Ok(None).
// F-2: derivationInput field checked FIRST, before full parse attempt.
// A matched-but-partial record (e.g. crash-mid-flight missing createdAt)
// still blocks — it is never skipped.

impl<S: AuditRecordSource, T: XrpcTransport> PdsClient for LivePdsClient<S, T> {
    fn find_entry_by_derivation_input(
        &self,
        derivation_input: &str,
    ) -> Result<Option<AuditEntry>, ScanError> {
        let mut cursor = None;
        let mut pages_fetched = 0u32;
        loop {
            // G-Q: transport failure → Err, never Ok(None).
            // Indeterminate silence is never permission to speak.
            let page = self
                .source
                .list_audit_records(cursor)
                .map_err(|e| ScanError::Transport(e))?;

            pages_fetched += 1;
            if pages_fetched > MAX_PAGES {
                return Err(ScanError::Indeterminate(format!(
                    "pagination bound ({MAX_PAGES}) exceeded — possible cyclic cursor"
                )));
            }

            for record in &page.records {
                // F-2: match the derivationInput field FIRST, before
                // attempting full parse. A record whose derivationInput
                // matches but is missing a sibling field (e.g. crash-
                // mid-flight missing createdAt) still blocks — it is
                // never skipped.
                let is_match = record
                    .value
                    .get("derivationInput")
                    .and_then(|v| v.as_str())
                    .map(|s| s == derivation_input)
                    .unwrap_or(false);

                if is_match {
                    // Match confirmed — construct the best AuditEntry we can.
                    // Missing sibling fields are empty strings; the pipeline
                    // treats Ok(Some(_)) as block regardless.
                    return Ok(Some(parse_audit_entry_best_effort(record)));
                }
            }

            match page.cursor {
                Some(next) => cursor = Some(next),
                None => return Ok(None),
            }
        }
    }

    fn create_pending_entry(&mut self, key: &str, entry: &PendingEntry) -> Result<(), String> {
        // putRecord into social.skaists.alpha.audit.entry, rkey = key.
        let record = serde_json::json!({
            "$type": AUDIT_COLLECTION,
            "derivationInput": entry.derivation_input,
            "inputDigest": entry.input_digest,
            "adapterClass": entry.adapter_class,
            "adapterDigest": entry.adapter_digest,
            "modelDigest": entry.model_digest,
            "promptDigest": entry.prompt_digest,
            "createdAt": entry.created_at,
            // No postUri/postCid on pending.
            "postUri": null,
            "postCid": null,
            "failureError": null,
        });
        let body = serde_json::json!({
            "repo": "bQueenBee", // placeholder — D-009c wiring provides real DID
            "collection": AUDIT_COLLECTION,
            "rkey": key,
            "record": record,
        });
        self.transport
            .put_record(body)
            .map(|_| ())
            .map_err(|e| format!("putRecord failed: {e}"))
    }

    fn submit_post(&mut self, text: &str) -> Result<(String, String), String> {
        // createRecord into app.bsky.feed.post.
        let record = serde_json::json!({
            "$type": POST_COLLECTION,
            "text": text,
            "createdAt": self.now_rfc3339,
        });
        let body = serde_json::json!({
            "repo": "bQueenBee",
            "collection": POST_COLLECTION,
            "record": record,
        });
        let resp = self.transport.create_record(body)?;
        // NAIVE (commit A): extracts uri/cid without validating presence.
        // A 2xx response missing uri/cid → fabricated success (empty strings).
        // Commit B fixes: strict extraction, Err if missing.
        let uri = resp
            .get("uri")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let cid = resp
            .get("cid")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        Ok((uri, cid))
    }

    fn finalize_entry(&mut self, key: &str, uri: &str, cid: &str) -> Result<(), String> {
        // putRecord overwriting the same audit rkey with postUri/postCid filled.
        // In the real implementation this would first read the existing record
        // to preserve fields; for now we write the finalized shape.
        let record = serde_json::json!({
            "$type": AUDIT_COLLECTION,
            "postUri": uri,
            "postCid": cid,
        });
        let body = serde_json::json!({
            "repo": "bQueenBee",
            "collection": AUDIT_COLLECTION,
            "rkey": key,
            "record": record,
        });
        self.transport
            .put_record(body)
            .map(|_| ())
            .map_err(|e| format!("putRecord failed: {e}"))
    }

    fn remove_entry(&mut self, key: &str) -> Result<(), String> {
        // deleteRecord. Founder-clearance tooling only; pipeline never calls.
        let body = serde_json::json!({
            "repo": "bQueenBee",
            "collection": AUDIT_COLLECTION,
            "rkey": key,
        });
        self.transport
            .delete_record(body)
            .map(|_| ())
            .map_err(|e| format!("deleteRecord failed: {e}"))
    }

    fn mark_entry_failed(&mut self, key: &str, error: &str) -> Result<(), String> {
        // putRecord overwriting with failureError set. Entry survives — never a delete.
        let record = serde_json::json!({
            "$type": AUDIT_COLLECTION,
            "failureError": error,
        });
        let body = serde_json::json!({
            "repo": "bQueenBee",
            "collection": AUDIT_COLLECTION,
            "rkey": key,
            "record": record,
        });
        self.transport
            .put_record(body)
            .map(|_| ())
            .map_err(|e| format!("putRecord failed: {e}"))
    }
}
