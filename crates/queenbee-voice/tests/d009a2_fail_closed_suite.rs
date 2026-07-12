//! D-009a2 — Fail-closed read scan suite (offline, against a mock transport)
//!
//! G-Q (founder ruling 2026-07-11): a double-post lock must fail
//! **closed**. When the audit scan cannot determine whether an entry
//! exists — transport error, indeterminate listing — the pipeline
//! does not post. Indeterminate silence is never permission to speak.
//!
//! Marquee reds (commit A):
//! 1. Transport Err mid-scan → scan returns Err (not Ok(None))
//! 2. Matching-but-partial record (missing createdAt) → blocks
//!
//! Carried forward from d009a: cursor-exhaustion, field-discipline,
//! pending/finalized/genuine-absence/empty/positive.

#![forbid(unsafe_code)]

use queenbee_voice::pds::live_client::{
    AuditRecord, AuditRecordSource, LivePdsClient, NoopXrpcTransport, RecordsPage,
};
use queenbee_voice::pipeline::{
    AuditEntry, Clock, Hasher, PdsClient, PendingEntry, Pipeline, PipelineResult, ScanError,
};
use queenbee_voice::adapter::tree_landing::CommitFacts;
use queenbee_voice::heartbeat::HeartbeatState;
use queenbee_voice::wrapper::DailyCounter;
use std::cell::RefCell;

// ═══════════════════════════════════════════════════════════════
//  Constants and helpers
// ═══════════════════════════════════════════════════════════════

const QUERY: &str = "skaists/LOVErnment-DAO@884b2bce";

/// Build a complete record value matching the audit-entry lexicon.
fn record_value(
    derivation_input: &str,
    post_uri: Option<&str>,
    post_cid: Option<&str>,
    failure_error: Option<&str>,
) -> serde_json::Value {
    serde_json::json!({
        "$type": "social.skaists.alpha.audit.entry",
        "derivationInput": derivation_input,
        "inputDigest": "test_digest_hex",
        "adapterClass": "TreeLanding",
        "adapterDigest": "adapter_d_hash",
        "modelDigest": "model_d_hash",
        "promptDigest": "prompt_d_hash",
        "createdAt": "2026-07-11T00:00:00Z",
        "postUri": post_uri,
        "postCid": post_cid,
        "failureError": failure_error,
    })
}

/// Build a PARTIAL record value — matching derivationInput but
/// missing `createdAt`. Simulates a crash-mid-flight before the
/// last field was written.
fn partial_record_value_missing_created_at(derivation_input: &str) -> serde_json::Value {
    serde_json::json!({
        "$type": "social.skaists.alpha.audit.entry",
        "derivationInput": derivation_input,
        "inputDigest": "test_digest_hex",
        "adapterClass": "TreeLanding",
        "adapterDigest": "adapter_d_hash",
        "modelDigest": "model_d_hash",
        "promptDigest": "prompt_d_hash",
        // createdAt MISSING — crash-mid-flight
        "postUri": null,
        "postCid": null,
        "failureError": null,
    })
}

fn rec(rkey: &str, value: serde_json::Value) -> AuditRecord {
    AuditRecord {
        rkey: rkey.to_string(),
        value,
    }
}

fn expected(
    derivation_input: &str,
    post_uri: Option<&str>,
    post_cid: Option<&str>,
    failure_error: Option<&str>,
) -> AuditEntry {
    AuditEntry {
        pending: PendingEntry {
            derivation_input: derivation_input.to_string(),
            input_digest: "test_digest_hex".to_string(),
            adapter_class: "TreeLanding".to_string(),
            adapter_digest: "adapter_d_hash".to_string(),
            model_digest: "model_d_hash".to_string(),
            prompt_digest: "prompt_d_hash".to_string(),
            created_at: "2026-07-11T00:00:00Z".to_string(),
        },
        post_uri: post_uri.map(String::from),
        post_cid: post_cid.map(String::from),
        failure_error: failure_error.map(String::from),
    }
}

// ═══════════════════════════════════════════════════════════════
//  Mock AuditRecordSource — supports error injection
// ═══════════════════════════════════════════════════════════════

struct MockSource {
    /// Each element is either a page or a transport error.
    responses: Vec<Result<RecordsPage, String>>,
    calls: RefCell<Vec<Option<String>>>,
}

impl MockSource {
    fn new(responses: Vec<Result<RecordsPage, String>>) -> Self {
        Self {
            responses,
            calls: RefCell::new(Vec::new()),
        }
    }

    fn call_count(&self) -> usize {
        self.calls.borrow().len()
    }
}

impl AuditRecordSource for MockSource {
    fn list_audit_records(&self, cursor: Option<String>) -> Result<RecordsPage, String> {
        let idx = self.calls.borrow().len();
        self.calls.borrow_mut().push(cursor);
        if idx >= self.responses.len() {
            // Past the scripted responses — return empty terminal page.
            return Ok(RecordsPage {
                records: vec![],
                cursor: None,
            });
        }
        self.responses[idx].clone()
    }
}

// ═══════════════════════════════════════════════════════════════
//  Pipeline-level test doubles
// ═══════════════════════════════════════════════════════════════

struct MockCounter {
    count: u64,
}
impl DailyCounter for MockCounter {
    fn count_today(&self) -> u64 {
        self.count
    }
    fn increment(&mut self) {
        self.count += 1;
    }
}

struct FixedClock;
impl Clock for FixedClock {
    fn now_rfc3339(&self) -> String {
        "2026-07-12T00:00:00Z".to_string()
    }
}

struct Fnv1aHasher;
impl Hasher for Fnv1aHasher {
    fn sha256_hex(&self, input: &[u8]) -> String {
        let mut h: u64 = 0xcbf29ce484222325;
        for &b in input {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        format!("{h:016x}")
    }
}

fn clean_facts() -> CommitFacts {
    CommitFacts {
        repo: "skaists/LOVErnment-DAO".to_string(),
        sha: "884b2bce".to_string(),
        ref_name: "main".to_string(),
        subject: "test commit".to_string(),
        body: String::new(),
        signature_verified: true,
    }
}

fn make_pipeline() -> Pipeline {
    Pipeline::new(
        "adapter_digest_hash".to_string(),
        "model_digest_hash".to_string(),
        "prompt_digest_hash".to_string(),
    )
}

/// Mock PdsClient that returns a canned scan result and tracks writes.
struct ScanTestPds {
    scan_result: Result<Option<AuditEntry>, ScanError>,
    submitted: RefCell<Option<String>>,
    created: RefCell<Option<String>>,
}

impl ScanTestPds {
    fn success() -> Self {
        Self {
            scan_result: Ok(None),
            submitted: RefCell::new(None),
            created: RefCell::new(None),
        }
    }

    fn scan_err(error: &str) -> Self {
        Self {
            scan_result: Err(ScanError::Transport(error.to_string())),
            ..Self::success()
        }
    }
}

impl PdsClient for ScanTestPds {
    fn find_entry_by_derivation_input(
        &self,
        _derivation_input: &str,
    ) -> Result<Option<AuditEntry>, ScanError> {
        self.scan_result.clone()
    }

    fn create_pending_entry(
        &mut self,
        key: &str,
        _entry: &PendingEntry,
    ) -> Result<(), String> {
        *self.created.borrow_mut() = Some(key.to_string());
        Ok(())
    }

    fn submit_post(&mut self, text: &str) -> Result<(String, String), String> {
        *self.submitted.borrow_mut() = Some(text.to_string());
        Ok((
            "at://did:plc:test/app.bsky.feed.post/tid1".to_string(),
            "bafyrei_fake_cid".to_string(),
        ))
    }

    fn finalize_entry(&mut self, _key: &str, _uri: &str, _cid: &str) -> Result<(), String> {
        Ok(())
    }

    fn remove_entry(&mut self, _key: &str) -> Result<(), String> {
        Ok(())
    }

    fn mark_entry_failed(&mut self, _key: &str, _error: &str) -> Result<(), String> {
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════
//  1. MARQUEE RED #1 — Transport Err mid-scan → Err (not Ok(None))
//     Pages 1..k-1 clean, page k fails. The entry is on an
//     unfetched page. A fail-open client returns Ok(None).
// ═══════════════════════════════════════════════════════════════

#[test]
fn transport_err_mid_scan_returns_err() {
    let source = MockSource::new(vec![
        Ok(RecordsPage {
            records: vec![rec(
                "tid-p1-a",
                record_value("other/repo@aaa111", None, None, None),
            )],
            cursor: Some("cursor-to-page-2".to_string()),
        }),
        // Page 2: transport failure. Entry is on page 3 (never reached).
        Err("connection reset".to_string()),
    ]);

    let client = LivePdsClient::new(source, NoopXrpcTransport::default());
    let result = client.find_entry_by_derivation_input(QUERY);

    assert!(
        result.is_err(),
        "MUST return Err on transport failure mid-scan — \
         a swallowed Err becomes Ok(None) = false clear = double-post window open. (G-Q)"
    );
    let err = result.unwrap_err();
    assert!(
        matches!(err, ScanError::Transport(_)),
        "must be Transport error, got {err:?}"
    );

    // The client must have followed the cursor to page 2 (where it failed).
    assert_eq!(
        client.source().call_count(),
        2,
        "must have fetched page 1 cleanly, then failed on page 2"
    );
}

// ═══════════════════════════════════════════════════════════════
//  1b. Transport Err on first page → Err
// ═══════════════════════════════════════════════════════════════

#[test]
fn transport_err_first_page_returns_err() {
    let source = MockSource::new(vec![
        Err("timeout".to_string()),
    ]);

    let client = LivePdsClient::new(source, NoopXrpcTransport::default());
    let result = client.find_entry_by_derivation_input(QUERY);

    assert!(
        result.is_err(),
        "first-page transport failure must return Err, not Ok(None)"
    );
}

// ═══════════════════════════════════════════════════════════════
//  2. MARQUEE RED #2 — Matching-but-partial record (missing
//     createdAt) → blocks (Ok(Some)), not skipped (Ok(None))
// ═══════════════════════════════════════════════════════════════

#[test]
fn matching_but_partial_record_blocks() {
    let source = MockSource::new(vec![
        Ok(RecordsPage {
            records: vec![rec(
                "tid-partial-1",
                partial_record_value_missing_created_at(QUERY),
            )],
            cursor: None,
        }),
    ]);

    let client = LivePdsClient::new(source, NoopXrpcTransport::default());
    let result = client.find_entry_by_derivation_input(QUERY);

    assert!(
        result.is_ok(),
        "scan should succeed — the record is present, just partial"
    );
    assert!(
        result.unwrap().is_some(),
        "MUST return Some — a record with matching derivationInput \
         blocks even if sibling fields are missing. The crash-mid-flight \
         partial pending is the exact case the lock must catch."
    );
}

// ═══════════════════════════════════════════════════════════════
//  3. Pipeline: scan Err → ScanAborted, no post, nothing persisted
// ═══════════════════════════════════════════════════════════════

#[test]
fn pipeline_scan_error_aborts_no_post() {
    let mut pipeline = make_pipeline();
    let mut pds = ScanTestPds::scan_err("connection reset");
    let mut counter = MockCounter { count: 0 };

    let result = pipeline.run(
        &clean_facts(),
        &mut pds,
        &mut counter,
        HeartbeatState::Alive,
        &FixedClock,
        &Fnv1aHasher,
    );

    assert!(
        matches!(result, PipelineResult::ScanAborted { .. }),
        "scan error must abort — G-Q: indeterminate silence is never \
         permission to speak. Got {result:?}"
    );
    assert!(
        pds.submitted.borrow().is_none(),
        "must NOT post on scan error"
    );
    assert!(
        pds.created.borrow().is_none(),
        "must NOT create entry on scan error"
    );
}

// ═══════════════════════════════════════════════════════════════
//  4. Pipeline: scan Ok(None) → proceeds normally
// ═══════════════════════════════════════════════════════════════

#[test]
fn pipeline_scan_clear_proceeds_to_post() {
    let mut pipeline = make_pipeline();
    let mut pds = ScanTestPds::success();
    let mut counter = MockCounter { count: 0 };

    let result = pipeline.run(
        &clean_facts(),
        &mut pds,
        &mut counter,
        HeartbeatState::Alive,
        &FixedClock,
        &Fnv1aHasher,
    );

    assert!(
        matches!(result, PipelineResult::Success { .. }),
        "clear scan must proceed to post. Got {result:?}"
    );
    assert!(
        pds.submitted.borrow().is_some(),
        "must post on clear scan"
    );
}

// ═══════════════════════════════════════════════════════════════
//  5. Cursor exhaustion — carried forward from d009a
// ═══════════════════════════════════════════════════════════════

#[test]
fn cursor_exhaustion_entry_on_page_3() {
    let source = MockSource::new(vec![
        Ok(RecordsPage {
            records: vec![rec(
                "tid-p1-a",
                record_value("other/repo@aaa111", None, None, None),
            )],
            cursor: Some("cursor-to-page-2".to_string()),
        }),
        Ok(RecordsPage {
            records: vec![rec(
                "tid-p2-a",
                record_value("other/repo@bbb222", None, None, None),
            )],
            cursor: Some("cursor-to-page-3".to_string()),
        }),
        Ok(RecordsPage {
            records: vec![rec("tid-p3-match", record_value(QUERY, None, None, None))],
            cursor: None,
        }),
    ]);

    let client = LivePdsClient::new(source, NoopXrpcTransport::default());
    let result = client.find_entry_by_derivation_input(QUERY).unwrap();

    assert!(
        result.is_some(),
        "MUST find entry on page 3 — cursor exhaustion is load-bearing."
    );
    assert_eq!(result.unwrap(), expected(QUERY, None, None, None));
    assert_eq!(
        client.source().call_count(),
        3,
        "must have fetched all 3 pages following the cursor chain"
    );
}

// ═══════════════════════════════════════════════════════════════
//  6. Pending match blocks — carried forward
// ═══════════════════════════════════════════════════════════════

#[test]
fn pending_entry_for_input_is_returned() {
    let source = MockSource::new(vec![
        Ok(RecordsPage {
            records: vec![rec(
                "tid-pending-1",
                record_value(QUERY, None, None, None),
            )],
            cursor: None,
        }),
    ]);

    let client = LivePdsClient::new(source, NoopXrpcTransport::default());
    let result = client.find_entry_by_derivation_input(QUERY).unwrap();

    let entry = result.expect("pending entry must be returned, not skipped");
    assert_eq!(entry, expected(QUERY, None, None, None));
    assert!(entry.post_uri.is_none());
    assert!(entry.post_cid.is_none());
}

// ═══════════════════════════════════════════════════════════════
//  7. Finalized match blocks — carried forward
// ═══════════════════════════════════════════════════════════════

#[test]
fn finalized_entry_for_input_is_returned() {
    let source = MockSource::new(vec![
        Ok(RecordsPage {
            records: vec![rec(
                "tid-finalized-1",
                record_value(
                    QUERY,
                    Some("at://did:plc:test/app.bsky.feed.post/abc"),
                    Some("bafyrei_cid123"),
                    None,
                ),
            )],
            cursor: None,
        }),
    ]);

    let client = LivePdsClient::new(source, NoopXrpcTransport::default());
    let result = client.find_entry_by_derivation_input(QUERY).unwrap();

    let entry = result.expect("finalized entry must be returned");
    assert_eq!(
        entry,
        expected(
            QUERY,
            Some("at://did:plc:test/app.bsky.feed.post/abc"),
            Some("bafyrei_cid123"),
            None,
        )
    );
}

// ═══════════════════════════════════════════════════════════════
//  8. Genuine absence — carried forward
// ═══════════════════════════════════════════════════════════════

#[test]
fn genuine_absence_all_pages_exhausted_returns_ok_none() {
    let source = MockSource::new(vec![
        Ok(RecordsPage {
            records: vec![
                rec("tid-1", record_value("other/repo@sha1", None, None, None)),
                rec("tid-2", record_value("other/repo@sha2", None, None, None)),
            ],
            cursor: Some("page-2".to_string()),
        }),
        Ok(RecordsPage {
            records: vec![rec(
                "tid-3",
                record_value("other/repo@sha3", None, None, None),
            )],
            cursor: None,
        }),
    ]);

    let client = LivePdsClient::new(source, NoopXrpcTransport::default());
    let result = client.find_entry_by_derivation_input(QUERY).unwrap();

    assert!(
        result.is_none(),
        "must return Ok(None) only after ALL pages exhausted with no match"
    );
    assert_eq!(
        client.source().call_count(),
        2,
        "must have fetched both pages before concluding None"
    );
}

// ═══════════════════════════════════════════════════════════════
//  9. Field discipline — carried forward (rkey ≠ field match)
// ═══════════════════════════════════════════════════════════════

#[test]
fn field_discipline_rkey_not_match_field() {
    let source = MockSource::new(vec![
        Ok(RecordsPage {
            records: vec![
                // rkey matches QUERY, but derivationInput does NOT.
                rec(
                    QUERY,
                    record_value("different/repo@differentSha", None, None, None),
                ),
            ],
            cursor: None,
        }),
    ]);

    let client = LivePdsClient::new(source, NoopXrpcTransport::default());
    let result = client.find_entry_by_derivation_input(QUERY).unwrap();

    assert!(
        result.is_none(),
        "must NOT match on rkey — the scan reads derivationInput field, never the key."
    );
}

// ═══════════════════════════════════════════════════════════════
//  10. Empty collection — carried forward
// ═══════════════════════════════════════════════════════════════

#[test]
fn empty_collection_returns_ok_none() {
    let source = MockSource::new(vec![
        Ok(RecordsPage {
            records: vec![],
            cursor: None,
        }),
    ]);

    let client = LivePdsClient::new(source, NoopXrpcTransport::default());
    let result = client.find_entry_by_derivation_input(QUERY).unwrap();

    assert!(result.is_none(), "empty collection must return Ok(None)");
    assert_eq!(client.source().call_count(), 1);
}

// ═══════════════════════════════════════════════════════════════
//  11. Positive multi-page — carried forward
// ═══════════════════════════════════════════════════════════════

#[test]
fn positive_multi_page_finds_exact_entry() {
    let source = MockSource::new(vec![
        Ok(RecordsPage {
            records: vec![
                rec("tid-a", record_value("repo-a@sha-a", None, None, None)),
                rec("tid-b", record_value("repo-b@sha-b", None, None, None)),
            ],
            cursor: Some("page-2".to_string()),
        }),
        Ok(RecordsPage {
            records: vec![rec(
                "tid-c",
                record_value("repo-c@sha-c", None, None, None),
            )],
            cursor: Some("page-3".to_string()),
        }),
        Ok(RecordsPage {
            records: vec![rec(
                "tid-z",
                record_value(
                    QUERY,
                    Some("at://did:plc:test/app.bsky.feed.post/zzz"),
                    Some("bafyrei_final_cid"),
                    None,
                ),
            )],
            cursor: None,
        }),
    ]);

    let client = LivePdsClient::new(source, NoopXrpcTransport::default());
    let result = client.find_entry_by_derivation_input(QUERY).unwrap();

    let entry = result.expect("must find the entry on page 3");
    assert_eq!(
        entry,
        expected(
            QUERY,
            Some("at://did:plc:test/app.bsky.feed.post/zzz"),
            Some("bafyrei_final_cid"),
            None,
        ),
        "returned entry must be exactly correct"
    );
}

// ═══════════════════════════════════════════════════════════════
//  12. F-5: Pagination bound — cyclic cursor → Err(Indeterminate)
// ═══════════════════════════════════════════════════════════════

#[test]
fn pagination_bound_cyclic_cursor_returns_indeterminate() {
    // Create a cyclic cursor: every page returns the same cursor.
    // The bound trips and returns Err(Indeterminate), not Ok(None).
    let cyclic_page = Ok(RecordsPage {
        records: vec![rec(
            "tid-loop",
            record_value("other/repo@loop", None, None, None),
        )],
        cursor: Some("repeating-cursor".to_string()),
    });

    // We need more than MAX_PAGES responses to trigger the bound.
    // MockSource returns terminal empty page past the script.
    // MAX_PAGES is 1000 — too many for a test. Instead, we test
    // the bound mechanism directly: provide exactly enough pages
    // to trip a small bound. Since MAX_PAGES is a const (1000),
    // we verify the behavior via the Indeterminate variant on
    // a more practical scale.
    //
    // Strategy: provide 1001 pages (all cyclic). The 1001st call
    // trips the bound. But that's too many allocations for a test.
    //
    // Alternative: the bound is checked AFTER incrementing
    // pages_fetched. After MAX_PAGES pages, the next iteration
    // returns Err. We can test with a MockSource that always
    // returns the same cursor, and rely on the bound being hit.
    // For test speed, we reduce via a separate test that verifies
    // the Indeterminate path with a smaller bound by testing the
    // mechanism itself.
    //
    // Since we can't change MAX_PAGES for tests easily (it's a const),
    // and 1000 iterations is fast enough, we run it.
    let responses: Vec<Result<RecordsPage, String>> = (0..1001)
        .map(|_| cyclic_page.clone())
        .collect();

    let source = MockSource::new(responses);
    let client = LivePdsClient::new(source, NoopXrpcTransport::default());
    let result = client.find_entry_by_derivation_input(QUERY);

    assert!(
        result.is_err(),
        "cyclic cursor must trip pagination bound → Err, not hang or Ok(None)"
    );
    let err = result.unwrap_err();
    assert!(
        matches!(err, ScanError::Indeterminate(_)),
        "must be Indeterminate, got {err:?}"
    );
}

// ═══════════════════════════════════════════════════════════════
//  13. F-4: Canonical derivationInput — exact match, no normalization
// ═══════════════════════════════════════════════════════════════

#[test]
fn canonical_derivation_input_exact_match_no_normalization() {
    // derivationInput = "repo@sha" — canonical by construction.
    // The scan uses exact-string ==. Case/whitespace skew does not match.
    let source = MockSource::new(vec![
        Ok(RecordsPage {
            records: vec![
                // Same derivationInput but UPPERCASE — must NOT match.
                rec(
                    "tid-upper",
                    record_value("SKAISTS/LOVErnment-DAO@884B2BCE", None, None, None),
                ),
                // Exact match — MUST match.
                rec("tid-exact", record_value(QUERY, None, None, None)),
            ],
            cursor: None,
        }),
    ]);

    let client = LivePdsClient::new(source, NoopXrpcTransport::default());
    let result = client.find_entry_by_derivation_input(QUERY).unwrap();

    let entry = result.expect("exact-string match must find the canonical entry");
    assert_eq!(entry.pending.derivation_input, QUERY);
}

