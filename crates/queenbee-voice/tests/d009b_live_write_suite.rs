//! D-009b — Live write transport suite (offline, mock HTTP boundary)
//!
//! The five PdsClient write methods mapped to real XRPC shapes,
//! proven against a mock transport. No network in any test.
//!
//! Marquee red (commit A):
//! 1. submit_post 2xx-missing-uri → Err (no fabricated success)
//!
//! Also: remove_entry reachability — run() never calls it.

#![forbid(unsafe_code)]

use queenbee_voice::adapter::tree_landing::CommitFacts;
use queenbee_voice::heartbeat::HeartbeatState;
use queenbee_voice::pds::live_client::{
    AuditRecord, AuditRecordSource, LivePdsClient, RecordsPage, XrpcTransport,
};
use queenbee_voice::pipeline::{
    Clock, Hasher, PendingEntry, PdsClient, Pipeline, PipelineResult,
};
use queenbee_voice::wrapper::DailyCounter;
use std::cell::RefCell;

// ═══════════════════════════════════════════════════════════════
//  Mock XrpcTransport — records calls, returns canned responses
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
enum XrpcCall {
    PutRecord(serde_json::Value),
    CreateRecord(serde_json::Value),
    #[allow(dead_code)]
    DeleteRecord(serde_json::Value),
}

struct MockXrpcTransport {
    /// Canned responses, one per call. Each is either a success body
    /// or an error message.
    responses: Vec<Result<serde_json::Value, String>>,
    /// All calls made, in order.
    calls: RefCell<Vec<XrpcCall>>,
}

impl MockXrpcTransport {
    fn new(responses: Vec<Result<serde_json::Value, String>>) -> Self {
        Self {
            responses,
            calls: RefCell::new(Vec::new()),
        }
    }

    fn calls_made(&self) -> Vec<XrpcCall> {
        self.calls.borrow().clone()
    }
}

impl XrpcTransport for MockXrpcTransport {
    fn put_record(&mut self, body: serde_json::Value) -> Result<serde_json::Value, String> {
        let idx = self.calls.borrow().len();
        self.calls.borrow_mut().push(XrpcCall::PutRecord(body));
        if idx >= self.responses.len() {
            return Err("MockXrpcTransport: no canned response".to_string());
        }
        self.responses[idx].clone()
    }

    fn create_record(&mut self, body: serde_json::Value) -> Result<serde_json::Value, String> {
        let idx = self.calls.borrow().len();
        self.calls.borrow_mut().push(XrpcCall::CreateRecord(body));
        if idx >= self.responses.len() {
            return Err("MockXrpcTransport: no canned response".to_string());
        }
        self.responses[idx].clone()
    }

    fn delete_record(&mut self, body: serde_json::Value) -> Result<serde_json::Value, String> {
        let idx = self.calls.borrow().len();
        self.calls.borrow_mut().push(XrpcCall::DeleteRecord(body));
        if idx >= self.responses.len() {
            return Err("MockXrpcTransport: no canned response".to_string());
        }
        self.responses[idx].clone()
    }
}

// ═══════════════════════════════════════════════════════════════
//  Empty read source (write tests don't need audit records)
// ═══════════════════════════════════════════════════════════════

struct EmptySource;

impl AuditRecordSource for EmptySource {
    fn list_audit_records(&self, _cursor: Option<String>) -> Result<RecordsPage, String> {
        Ok(RecordsPage {
            records: vec![],
            cursor: None,
        })
    }
}

// ═══════════════════════════════════════════════════════════════
//  Pipeline-level test doubles (for integration test)
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

fn make_pending() -> PendingEntry {
    PendingEntry {
        derivation_input: "skaists/LOVErnment-DAO@884b2bce".to_string(),
        input_digest: "test_digest".to_string(),
        adapter_class: "TreeLanding".to_string(),
        adapter_digest: "adapter_d".to_string(),
        model_digest: "model_d".to_string(),
        prompt_digest: "prompt_d".to_string(),
        created_at: "2026-07-12T00:00:00Z".to_string(),
    }
}

// ═══════════════════════════════════════════════════════════════
//  1. create_pending_entry happy → correct putRecord shape
// ═══════════════════════════════════════════════════════════════

#[test]
fn create_pending_entry_happy_correct_put_record_shape() {
    let transport = MockXrpcTransport::new(vec![Ok(serde_json::json!({}))]);
    let mut client = LivePdsClient::new(EmptySource, transport);
    let pending = make_pending();

    let result = client.create_pending_entry("test-key", &pending);
    assert!(result.is_ok(), "should succeed on happy putRecord");

    let calls = client.transport().calls_made();
    assert_eq!(calls.len(), 1, "exactly one putRecord call");

    match &calls[0] {
        XrpcCall::PutRecord(body) => {
            assert_eq!(
                body["collection"],
                "social.skaists.alpha.audit.entry",
                "collection must be the audit-entry NSID"
            );
            assert_eq!(body["rkey"], "test-key", "rkey must be the entry key");
            let record = &body["record"];
            assert_eq!(
                record["derivationInput"],
                "skaists/LOVErnment-DAO@884b2bce"
            );
            assert_eq!(record["inputDigest"], "test_digest");
            assert_eq!(record["adapterClass"], "TreeLanding");
            assert!(record["postUri"].is_null(), "pending must not have postUri");
            assert!(
                record["postCid"].is_null(),
                "pending must not have postCid"
            );
            assert!(
                record["failureError"].is_null(),
                "pending must not have failureError"
            );
        }
        other => panic!("expected PutRecord, got {other:?}"),
    }
}

// ═══════════════════════════════════════════════════════════════
//  2. submit_post happy → parses (uri, cid) from createRecord response
// ═══════════════════════════════════════════════════════════════

#[test]
fn submit_post_happy_parses_uri_cid() {
    let transport = MockXrpcTransport::new(vec![Ok(serde_json::json!({
        "uri": "at://did:plc:test/app.bsky.feed.post/tid123",
        "cid": "bafyrei_abc123",
    }))]);
    let mut client = LivePdsClient::new(EmptySource, transport);

    let result = client.submit_post("hello world");
    assert!(result.is_ok(), "should succeed on happy createRecord");

    let (uri, cid) = result.unwrap();
    assert_eq!(uri, "at://did:plc:test/app.bsky.feed.post/tid123");
    assert_eq!(cid, "bafyrei_abc123");

    let calls = client.transport().calls_made();
    assert_eq!(calls.len(), 1);
    match &calls[0] {
        XrpcCall::CreateRecord(body) => {
            assert_eq!(body["collection"], "app.bsky.feed.post");
            assert_eq!(body["record"]["text"], "hello world");
        }
        other => panic!("expected CreateRecord, got {other:?}"),
    }
}

// ═══════════════════════════════════════════════════════════════
//  3. MARQUEE RED — submit_post 2xx-missing-uri → Err
//     A 2xx response missing uri/cid must NOT be fabricated success.
// ═══════════════════════════════════════════════════════════════

#[test]
fn submit_post_2xx_missing_uri_returns_err() {
    let transport = MockXrpcTransport::new(vec![Ok(serde_json::json!({
        // 2xx but no uri/cid — server bug or malformed response
        "someOtherField": "value",
    }))]);
    let mut client = LivePdsClient::new(EmptySource, transport);

    let result = client.submit_post("hello world");

    assert!(
        result.is_err(),
        "MUST return Err on 2xx response missing uri — \
         no fabricated success, no empty-string placeholders. \
         (The 'sha256:0'-class finding stays dead.)"
    );
}

// ═══════════════════════════════════════════════════════════════
//  4. submit_post timeout/5xx → Err (ambiguous-write-is-failed-write)
// ═══════════════════════════════════════════════════════════════

#[test]
fn submit_post_timeout_returns_err() {
    let transport = MockXrpcTransport::new(vec![Err("timeout".to_string())]);
    let mut client = LivePdsClient::new(EmptySource, transport);

    let result = client.submit_post("hello world");

    assert!(
        result.is_err(),
        "MUST return Err on timeout — ambiguous write is a failed write. \
         The pipeline marks the entry failed; the durable lock survives."
    );
}

#[test]
fn submit_post_5xx_returns_err() {
    let transport =
        MockXrpcTransport::new(vec![Err("HTTP 503: service unavailable".to_string())]);
    let mut client = LivePdsClient::new(EmptySource, transport);

    let result = client.submit_post("hello world");

    assert!(
        result.is_err(),
        "5xx must return Err — same fail-closed-write rule as timeout"
    );
}

// ═══════════════════════════════════════════════════════════════
//  5. finalize_entry happy → putRecord overwrites same rkey
// ═══════════════════════════════════════════════════════════════

#[test]
fn finalize_entry_happy_correct_put_record() {
    let transport = MockXrpcTransport::new(vec![Ok(serde_json::json!({}))]);
    let mut client = LivePdsClient::new(EmptySource, transport);
    let pending = make_pending();

    let result = client.finalize_entry(
        "test-key",
        &pending,
        "at://did:plc:test/app.bsky.feed.post/abc",
        "bafyrei_cid",
    );
    assert!(result.is_ok());

    let calls = client.transport().calls_made();
    assert_eq!(calls.len(), 1);
    match &calls[0] {
        XrpcCall::PutRecord(body) => {
            assert_eq!(body["rkey"], "test-key");
            let record = &body["record"];
            // D-009b2: ALL pending fields must survive finalize (carry-forward).
            assert_eq!(
                record["derivationInput"],
                "skaists/LOVErnment-DAO@884b2bce",
                "derivationInput must be preserved on finalize"
            );
            assert_eq!(record["inputDigest"], "test_digest");
            assert_eq!(record["adapterClass"], "TreeLanding");
            assert_eq!(record["adapterDigest"], "adapter_d");
            assert_eq!(record["modelDigest"], "model_d");
            assert_eq!(record["promptDigest"], "prompt_d");
            assert_eq!(record["createdAt"], "2026-07-12T00:00:00Z");
            assert_eq!(
                record["postUri"],
                "at://did:plc:test/app.bsky.feed.post/abc"
            );
            assert_eq!(record["postCid"], "bafyrei_cid");
            assert!(record["failureError"].is_null(), "failureError must be null on finalize");
        }
        other => panic!("expected PutRecord, got {other:?}"),
    }
}

// ═══════════════════════════════════════════════════════════════
//  6. mark_entry_failed happy → putRecord sets failureError, no delete
// ═══════════════════════════════════════════════════════════════

#[test]
fn mark_entry_failed_happy_sets_error_no_delete() {
    let transport = MockXrpcTransport::new(vec![Ok(serde_json::json!({}))]);
    let mut client = LivePdsClient::new(EmptySource, transport);

    let result = client.mark_entry_failed("test-key", "timeout");
    assert!(result.is_ok());

    let calls = client.transport().calls_made();
    assert_eq!(calls.len(), 1, "exactly one call");
    match &calls[0] {
        XrpcCall::PutRecord(body) => {
            assert_eq!(body["rkey"], "test-key");
            assert_eq!(body["record"]["failureError"], "timeout");
        }
        XrpcCall::DeleteRecord(_) => {
            panic!("mark_entry_failed must NOT call deleteRecord — entry survives")
        }
        other => panic!("expected PutRecord, got {other:?}"),
    }
}

// ═══════════════════════════════════════════════════════════════
//  7. XRPC 4xx → Err, mapped not panicked
// ═══════════════════════════════════════════════════════════════

#[test]
fn xrpc_4xx_returns_err_not_panic() {
    let transport =
        MockXrpcTransport::new(vec![Err("HTTP 400: Bad Request — invalid record".to_string())]);
    let mut client = LivePdsClient::new(EmptySource, transport);
    let pending = make_pending();

    let result = client.create_pending_entry("test-key", &pending);
    assert!(
        result.is_err(),
        "4xx must return Err, not panic or silently succeed"
    );
}

// ═══════════════════════════════════════════════════════════════
//  8. remove_entry reachability — run() never invokes it
// ═══════════════════════════════════════════════════════════════

/// Pipeline-level PdsClient wrapper that tracks whether remove_entry is ever called.
struct RemoveTracker {
    create_pending_ok: bool,
    submit_ok: bool,
    finalize_ok: bool,
    remove_called: RefCell<bool>,
}

impl RemoveTracker {
    fn success() -> Self {
        Self {
            create_pending_ok: true,
            submit_ok: true,
            finalize_ok: true,
            remove_called: RefCell::new(false),
        }
    }

    fn submit_fails() -> Self {
        Self {
            submit_ok: false,
            ..Self::success()
        }
    }
}

impl PdsClient for RemoveTracker {
    fn find_entry_by_derivation_input(
        &self,
        _derivation_input: &str,
    ) -> Result<Option<queenbee_voice::pipeline::AuditEntry>, queenbee_voice::pipeline::ScanError>
    {
        Ok(None) // clear to proceed
    }

    fn create_pending_entry(
        &mut self,
        _key: &str,
        _entry: &PendingEntry,
    ) -> Result<(), String> {
        if self.create_pending_ok {
            Ok(())
        } else {
            Err("create_pending failed".to_string())
        }
    }

    fn submit_post(&mut self, _text: &str) -> Result<(String, String), String> {
        if self.submit_ok {
            Ok((
                "at://did:plc:test/app.bsky.feed.post/tid1".to_string(),
                "bafyrei_fake_cid".to_string(),
            ))
        } else {
            Err("submit_post failed".to_string())
        }
    }

    fn finalize_entry(&mut self, _key: &str, _entry: &PendingEntry, _uri: &str, _cid: &str) -> Result<(), String> {
        if self.finalize_ok {
            Ok(())
        } else {
            Err("finalize failed".to_string())
        }
    }

    fn remove_entry(&mut self, _key: &str) -> Result<(), String> {
        *self.remove_called.borrow_mut() = true;
        Ok(())
    }

    fn mark_entry_failed(&mut self, _key: &str, _error: &str) -> Result<(), String> {
        Ok(())
    }
}

#[test]
fn remove_entry_never_called_by_pipeline_on_success() {
    let mut pipeline = make_pipeline();
    let mut pds = RemoveTracker::success();
    let mut counter = MockCounter { count: 0 };

    let result = pipeline.run(
        &clean_facts(),
        &mut pds,
        &mut counter,
        HeartbeatState::Alive,
        &FixedClock,
        &Fnv1aHasher,
    );

    assert!(matches!(result, PipelineResult::Success { .. }));
    assert!(
        !*pds.remove_called.borrow(),
        "remove_entry must NEVER be called by the pipeline on success"
    );
}

#[test]
fn remove_entry_never_called_by_pipeline_on_post_failure() {
    let mut pipeline = make_pipeline();
    let mut pds = RemoveTracker::submit_fails();
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
        matches!(result, PipelineResult::PostFailed { .. }),
        "post failure must return PostFailed"
    );
    assert!(
        !*pds.remove_called.borrow(),
        "remove_entry must NEVER be called on post failure — \
         D-008r3 law: entry SURVIVES, marked failed, not removed"
    );
}

// ═══════════════════════════════════════════════════════════════
//  9. Positive integration — full run() over mock transport
//     Writes the expected XRPC sequence in order.
// ═══════════════════════════════════════════════════════════════

/// Integration source: empty read (no existing entries → clear to post).
struct IntegrationSource;

impl AuditRecordSource for IntegrationSource {
    fn list_audit_records(&self, _cursor: Option<String>) -> Result<RecordsPage, String> {
        Ok(RecordsPage {
            records: vec![],
            cursor: None,
        })
    }
}

#[test]
fn positive_integration_full_run_xrpc_sequence() {
    // The pipeline will make 3 XRPC calls in order:
    // 1. putRecord (create_pending_entry)
    // 2. createRecord (submit_post)
    // 3. putRecord (finalize_entry)
    let transport = MockXrpcTransport::new(vec![
        // 1. putRecord for pending entry
        Ok(serde_json::json!({})),
        // 2. createRecord for post
        Ok(serde_json::json!({
            "uri": "at://did:plc:test/app.bsky.feed.post/int1",
            "cid": "bafyrei_int_cid",
        })),
        // 3. putRecord for finalize
        Ok(serde_json::json!({})),
    ]);

    let mut client = LivePdsClient::new(IntegrationSource, transport);

    let mut pipeline = make_pipeline();
    let mut counter = MockCounter { count: 0 };

    let result = pipeline.run(
        &clean_facts(),
        &mut client,
        &mut counter,
        HeartbeatState::Alive,
        &FixedClock,
        &Fnv1aHasher,
    );

    assert!(
        matches!(result, PipelineResult::Success { .. }),
        "full run must succeed, got {result:?}"
    );

    // Verify the XRPC call sequence.
    let calls = client.transport().calls_made();
    assert_eq!(
        calls.len(),
        3,
        "must make exactly 3 XRPC calls: pending, post, finalize"
    );

    // Call 1: putRecord for pending entry.
    match &calls[0] {
        XrpcCall::PutRecord(body) => {
            assert_eq!(body["collection"], "social.skaists.alpha.audit.entry");
            assert!(body["record"]["postUri"].is_null());
            assert_eq!(
                body["record"]["derivationInput"],
                "skaists/LOVErnment-DAO@884b2bce"
            );
        }
        other => panic!("call 1 must be PutRecord, got {other:?}"),
    }

    // Call 2: createRecord for post.
    match &calls[1] {
        XrpcCall::CreateRecord(body) => {
            assert_eq!(body["collection"], "app.bsky.feed.post");
            assert!(body["record"]["text"]
                .as_str()
                .unwrap()
                .contains("LOVErnment-DAO"));
        }
        other => panic!("call 2 must be CreateRecord, got {other:?}"),
    }

    // Call 3: putRecord for finalize.
    match &calls[2] {
        XrpcCall::PutRecord(body) => {
            assert_eq!(body["collection"], "social.skaists.alpha.audit.entry");
            assert_eq!(
                body["record"]["postUri"],
                "at://did:plc:test/app.bsky.feed.post/int1"
            );
        }
        other => panic!("call 3 must be PutRecord, got {other:?}"),
    }

    // No DeleteRecord in the sequence.
    for call in &calls {
        assert!(
            !matches!(call, XrpcCall::DeleteRecord(_)),
            "DeleteRecord must never appear in a successful run"
        );
    }
}

// ═══════════════════════════════════════════════════════════════
//  D-009b2 — Round-trip harness + marquee reds
//
//  The defect: finalize_entry writes only {$type, postUri, postCid},
//  stripping derivationInput and every other field. After any
//  successful post the audit record loses the field the durable
//  lock scans on → restart re-posts.
//
//  These tests exercise the REAL LivePdsClient (not a mock
//  PdsClient) with a shared read/write store, proving the defect
//  and its cure at the XRPC boundary where it lives.
// ═══════════════════════════════════════════════════════════════

use std::rc::Rc;
use std::collections::HashMap as StdHashMap;

/// Shared backing store: records keyed by rkey. Both the read
/// source and write transport reference the same Rc<RefCell<...>>,
/// so a putRecord is immediately visible to listAuditRecords.
#[derive(Clone, Default)]
struct RoundTripStore {
    records: Rc<RefCell<StdHashMap<String, serde_json::Value>>>,
}

struct RoundTripSource {
    store: RoundTripStore,
}

impl AuditRecordSource for RoundTripSource {
    fn list_audit_records(&self, _cursor: Option<String>) -> Result<RecordsPage, String> {
        let records: Vec<AuditRecord> = self
            .store
            .records
            .borrow()
            .iter()
            .map(|(rkey, value)| AuditRecord {
                rkey: rkey.clone(),
                value: value.clone(),
            })
            .collect();
        Ok(RecordsPage {
            records,
            cursor: None,
        })
    }
}

struct RoundTripTransport {
    store: RoundTripStore,
}

impl XrpcTransport for RoundTripTransport {
    fn put_record(&mut self, body: serde_json::Value) -> Result<serde_json::Value, String> {
        let rkey = body["rkey"]
            .as_str()
            .ok_or("putRecord missing rkey")?
            .to_string();
        let record = body["record"].clone();
        self.store.records.borrow_mut().insert(rkey, record);
        Ok(serde_json::json!({}))
    }

    fn create_record(&mut self, _body: serde_json::Value) -> Result<serde_json::Value, String> {
        Ok(serde_json::json!({
            "uri": "at://did:plc:test/app.bsky.feed.post/rt1",
            "cid": "bafyrei_rt_cid",
        }))
    }

    fn delete_record(&mut self, _body: serde_json::Value) -> Result<serde_json::Value, String> {
        Ok(serde_json::json!({}))
    }
}

// ═══════════════════════════════════════════════════════════════
//  MARQUEE RED #1 — the lock survives success
//
//  create → submit → finalize → find_entry_by_derivation_input
//  must still return the entry with derivationInput intact.
//  Against the current (buggy) finalize, derivationInput is
//  stripped → find returns None → the lock is broken.
// ═══════════════════════════════════════════════════════════════

#[test]
fn d009b2_marquee_red_1_lock_survives_success() {
    let store = RoundTripStore::default();
    let mut client = LivePdsClient::new(
        RoundTripSource { store: store.clone() },
        RoundTripTransport { store: store.clone() },
    );
    let pending = make_pending();

    // 1. Create pending entry — writes full record to store.
    client
        .create_pending_entry("test-key", &pending)
        .expect("create_pending must succeed");

    // 2. Submit post.
    let (uri, cid) = client
        .submit_post("hello world")
        .expect("submit_post must succeed");

    // 3. Finalize — currently strips derivationInput.
    client
        .finalize_entry("test-key", &pending, &uri, &cid)
        .expect("finalize must succeed");

    // 4. THE LOCK: find_entry_by_derivation_input must still find it.
    let result = client
        .find_entry_by_derivation_input(&pending.derivation_input)
        .expect("scan must not error");

    let entry = result.expect(
        "MUST find the entry after finalize — \
         derivationInput must survive every terminal state. \
         If this is None, the lock is broken on success.",
    );

    assert_eq!(
        entry.pending.derivation_input,
        pending.derivation_input,
        "derivationInput must be intact after finalize"
    );
    assert_eq!(entry.post_uri.as_deref(), Some(uri.as_str()), "postUri must be set");
    assert_eq!(entry.post_cid.as_deref(), Some(cid.as_str()), "postCid must be set");
}

// ═══════════════════════════════════════════════════════════════
//  MARQUEE RED #2 — no re-post after successful finalize
//
//  Run to success, then a FRESH pipeline (empty seen) over the
//  same store → must get Duplicate. Against the current (buggy)
//  finalize, derivationInput is stripped → the fresh scan returns
//  Ok(None) → the pipeline posts again. This is the restart-
//  durability proof, now covering the post-finalize state.
// ═══════════════════════════════════════════════════════════════

#[test]
fn d009b2_marquee_red_2_no_repost_after_finalize_restart() {
    let store = RoundTripStore::default();

    // --- Pipeline A: run to success ---
    {
        let mut client = LivePdsClient::new(
            RoundTripSource { store: store.clone() },
            RoundTripTransport { store: store.clone() },
        );
        let mut pipeline = make_pipeline();
        let mut counter = MockCounter { count: 0 };

        let result = pipeline.run(
            &clean_facts(),
            &mut client,
            &mut counter,
            HeartbeatState::Alive,
            &FixedClock,
            &Fnv1aHasher,
        );

        assert!(
            matches!(result, PipelineResult::Success { .. }),
            "first run must succeed, got {result:?}"
        );
    }

    // --- Pipeline B: fresh pipeline, same store, empty seen ---
    {
        let mut client = LivePdsClient::new(
            RoundTripSource { store: store.clone() },
            RoundTripTransport { store: store.clone() },
        );
        let mut fresh_pipeline = make_pipeline();
        let mut counter_b = MockCounter { count: 0 };

        let result = fresh_pipeline.run(
            &clean_facts(),
            &mut client,
            &mut counter_b,
            HeartbeatState::Alive,
            &FixedClock,
            &Fnv1aHasher,
        );

        assert_eq!(
            result,
            PipelineResult::Duplicate,
            "fresh pipeline over the same store must refuse — \
             derivationInput must survive finalize for the durable \
             lock to hold across restarts. If this is Success, the \
             lock is broken on success and the restart double-post \
             window is open."
        );
    }
}
