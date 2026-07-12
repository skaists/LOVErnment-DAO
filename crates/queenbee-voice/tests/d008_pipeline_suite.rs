//! D-008r3 — Atomic pipeline suite (the lock survives failure)
//!
//! VOICE-1 §5.5: no utterance without its entry. The pipeline is
//! atomic: create pending → submit post → finalize. Durable
//! idempotence: the audit trail is queried before the in-process mark.
//! No non-success terminal removes the audit entry. On submit_post Err
//! the entry is marked failed-pending-founder-review — it SURVIVES.

#![forbid(unsafe_code)]

use queenbee_voice::adapter::tree_landing::CommitFacts;
use queenbee_voice::heartbeat::HeartbeatState;
use queenbee_voice::pipeline::{
    AuditEntry, Clock, Hasher, PdsClient, PendingEntry, Pipeline, PipelineResult, ScanError,
};
use queenbee_voice::wrapper::{DailyCounter, SubmitResult};
use std::cell::RefCell;
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════
//  Test doubles
// ═══════════════════════════════════════════════════════════════

/// PDS mock with a durable store: entries survive pipeline destruction.
/// Store maps derivation_input → (PendingEntry, Option<post_uri>, Option<post_cid>, Option<failure_error>).
/// When failure_error is Some the entry is marked failed-pending-founder-review.
type StoredEntry = (PendingEntry, Option<String>, Option<String>, Option<String>);

struct DurableMockPds {
    store: RefCell<HashMap<String, StoredEntry>>,
    submit_result: Option<Result<(String, String), String>>,
    finalize_result: Option<Result<(), String>>,
    submitted: RefCell<Option<String>>,
    removed: RefCell<Option<String>>,
}

impl DurableMockPds {
    fn success() -> Self {
        Self {
            store: RefCell::new(HashMap::new()),
            submit_result: Some(Ok((
                "at://did:plc:test/app.bsky.feed.post/tid1".to_string(),
                "bafyrei_fake_cid".to_string(),
            ))),
            finalize_result: Some(Ok(())),
            submitted: RefCell::new(None),
            removed: RefCell::new(None),
        }
    }

    fn submit_fails() -> Self {
        Self {
            submit_result: Some(Err("PDS error".to_string())),
            ..Self::success()
        }
    }

    fn finalize_fails() -> Self {
        Self {
            finalize_result: Some(Err("finalize error".to_string())),
            ..Self::success()
        }
    }

    /// Construct from a pre-existing store — for crash-recovery tests.
    fn from_store(store: HashMap<String, StoredEntry>) -> Self {
        Self {
            store: RefCell::new(store),
            ..Self::success()
        }
    }
}

impl PdsClient for DurableMockPds {
    fn find_entry_by_derivation_input(
        &self,
        derivation_input: &str,
    ) -> Result<Option<AuditEntry>, ScanError> {
        // Ruling B (D-009c): scan by the derivationInput FIELD — the store is
        // keyed by tid rkey, never by derivationInput. This mirrors the real
        // client's field scan and keeps the durable lock intact.
        Ok(self.store.borrow().values().find_map(
            |(pending, uri, cid, failure_error)| {
                if pending.derivation_input == derivation_input {
                    Some(AuditEntry {
                        pending: pending.clone(),
                        post_uri: uri.clone(),
                        post_cid: cid.clone(),
                        failure_error: failure_error.clone(),
                    })
                } else {
                    None
                }
            },
        ))
    }

    fn create_pending_entry(&mut self, key: &str, entry: &PendingEntry) -> Result<(), String> {
        // Ruling B (D-009c): the rkey must be a tid, never the derivationInput.
        assert!(
            !key.contains('/') && !key.contains('@'),
            "invalid audit rkey (Ruling B): {key}"
        );
        self.store
            .borrow_mut()
            .insert(key.to_string(), (entry.clone(), None, None, None));
        Ok(())
    }

    fn submit_post(&mut self, text: &str) -> Result<(String, String), String> {
        *self.submitted.borrow_mut() = Some(text.to_string());
        self.submit_result.clone().unwrap_or(Ok((
            "at://did:plc:test/app.bsky.feed.post/tid1".to_string(),
            "bafyrei_fake_cid".to_string(),
        )))
    }

    fn finalize_entry(&mut self, key: &str, _entry: &PendingEntry, uri: &str, cid: &str) -> Result<(), String> {
        let result = self.finalize_result.clone().unwrap_or(Ok(()));
        if result.is_ok() {
            if let Some(entry) = self.store.borrow_mut().get_mut(key) {
                entry.1 = Some(uri.to_string());
                entry.2 = Some(cid.to_string());
            }
        }
        result
    }

    fn remove_entry(&mut self, key: &str) -> Result<(), String> {
        *self.removed.borrow_mut() = Some(key.to_string());
        self.store.borrow_mut().remove(key);
        Ok(())
    }

    fn mark_entry_failed(&mut self, key: &str, error: &str) -> Result<(), String> {
        if let Some(entry) = self.store.borrow_mut().get_mut(key) {
            entry.3 = Some(error.to_string());
        }
        Ok(())
    }
}

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
        "2026-07-11T00:00:00Z".to_string()
    }
    fn now_micros(&self) -> u64 {
        1_752_192_000_000_000
    }
}

/// Test hash function: FNV-1a (non-cryptographic, deterministic).
/// Not sha256 — labeled truthfully. Used only for test assertions
/// against known values computed from fixed input.
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

const DERIVATION_KEY: &str = "skaists/LOVErnment-DAO@884b2bce";

// ═══════════════════════════════════════════════════════════════
//  Canonical serialization test vector
// ═══════════════════════════════════════════════════════════════

#[test]
fn canonical_facts_json_pinned() {
    let facts = clean_facts();
    let canonical = Pipeline::canonical_facts_json(&facts);
    assert_eq!(
        canonical,
        r#"{"repo":"skaists/LOVErnment-DAO","sha":"884b2bce","ref_name":"main","subject":"test commit","body":"","signature_verified":true}"#,
        "canonical serialization must be deterministic and pinned"
    );
}

// ═══════════════════════════════════════════════════════════════
//  Happy path — post live, entry finalized, real audit fields
// ═══════════════════════════════════════════════════════════════

#[test]
fn happy_path_writes_both_entry_finalized() {
    let mut pipeline = make_pipeline();
    let mut pds = DurableMockPds::success();
    let mut counter = MockCounter { count: 0 };

    let result = pipeline.run(
        &clean_facts(),
        &mut pds,
        &mut counter,
        HeartbeatState::Alive,
        &FixedClock,
        &Fnv1aHasher,
    );

    match result {
        PipelineResult::Success { entry } => {
            assert_eq!(entry.post_uri, "at://did:plc:test/app.bsky.feed.post/tid1");
            assert_eq!(entry.post_cid, "bafyrei_fake_cid");

            assert_eq!(
                entry.pending.derivation_input,
                "skaists/LOVErnment-DAO@884b2bce"
            );
            assert_eq!(entry.pending.adapter_class, "TreeLanding");
            assert_eq!(entry.pending.adapter_digest, "adapter_digest_hash");
            assert_eq!(entry.pending.model_digest, "model_digest_hash");
            assert_eq!(entry.pending.prompt_digest, "prompt_digest_hash");
            assert_eq!(entry.pending.created_at, "2026-07-11T00:00:00Z");

            let canonical = Pipeline::canonical_facts_json(&clean_facts());
            let expected_digest = Fnv1aHasher.sha256_hex(canonical.as_bytes());
            assert_eq!(entry.pending.input_digest, expected_digest);
        }
        other => panic!("expected Success, got {other:?}"),
    }

    // Explicit assertions: pending was created AND entry was finalized.
    assert!(
        pds.submitted.borrow().is_some(),
        "post must have been submitted"
    );
    let store = pds.store.borrow();
    let entry = store
        .values()
        .find(|e| e.0.derivation_input == DERIVATION_KEY)
        .expect("pending entry must have been created in the store");
    assert!(
        entry.1.is_some(),
        "entry must have post_uri after finalization"
    );
    assert!(
        entry.2.is_some(),
        "entry must have post_cid after finalization"
    );
}

// ═══════════════════════════════════════════════════════════════
//  Wrapper refused — nothing persisted
// ═══════════════════════════════════════════════════════════════

#[test]
fn wrapper_refused_writes_neither() {
    let mut pipeline = make_pipeline();
    let mut pds = DurableMockPds::success();
    let mut counter = MockCounter { count: 3 };

    let result = pipeline.run(
        &clean_facts(),
        &mut pds,
        &mut counter,
        HeartbeatState::Alive,
        &FixedClock,
        &Fnv1aHasher,
    );

    assert!(
        matches!(result, PipelineResult::Refused { .. }),
        "wrapper refusal must return Refused, got {result:?}"
    );
    if let PipelineResult::Refused { reason } = result {
        assert_eq!(reason, SubmitResult::RateLimited);
    }
    assert!(
        pds.submitted.borrow().is_none(),
        "nothing submitted on refusal"
    );
    assert!(
        pds.store.borrow().is_empty(),
        "no pending entry on refusal"
    );
}

// ═══════════════════════════════════════════════════════════════
//  Post failure — entry SURVIVES, marked failed-pending-founder-review
//  (INVERTED from d008r2: was "removes_pending_entry")
// ═══════════════════════════════════════════════════════════════

#[test]
fn post_failure_marks_entry_failed_survives() {
    let mut pipeline = make_pipeline();
    let mut pds = DurableMockPds::submit_fails();
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
        "post failure must return PostFailed, got {result:?}"
    );
    // The entry MUST survive — the lock outlives any non-success terminal.
    assert!(
        pds.store.borrow().values().any(|e| e.0.derivation_input == DERIVATION_KEY),
        "entry must SURVIVE post failure — the durable lock"
    );
    // The entry MUST be marked failed.
    let binding = pds.store.borrow();
    let entry = binding
        .values()
        .find(|e| e.0.derivation_input == DERIVATION_KEY)
        .expect("entry must exist");
    assert!(
        entry.3.is_some(),
        "entry must be marked failed-pending-founder-review"
    );
    // The entry MUST NOT have been removed.
    assert!(
        pds.removed.borrow().is_none(),
        "remove_entry must NOT be called on post failure"
    );
}

// ═══════════════════════════════════════════════════════════════
//  Finalize failure — pending entry survives, post is live
// ═══════════════════════════════════════════════════════════════

#[test]
fn finalize_failure_leaves_pending_entry_alive() {
    let mut pipeline = make_pipeline();
    let mut pds = DurableMockPds::finalize_fails();
    let mut counter = MockCounter { count: 0 };

    let result = pipeline.run(
        &clean_facts(),
        &mut pds,
        &mut counter,
        HeartbeatState::Alive,
        &FixedClock,
        &Fnv1aHasher,
    );

    match result {
        PipelineResult::FinalizeFailed { post_uri, post_cid, .. } => {
            assert_eq!(post_uri, "at://did:plc:test/app.bsky.feed.post/tid1");
            assert_eq!(post_cid, "bafyrei_fake_cid");
        }
        other => panic!("expected FinalizeFailed, got {other:?}"),
    }
    assert!(
        pds.submitted.borrow().is_some(),
        "post must be live"
    );
    // The pending entry MUST survive — detectable honesty.
    assert!(
        pds.store.borrow().values().any(|e| e.0.derivation_input == DERIVATION_KEY),
        "pending entry must survive finalize failure"
    );
    let binding = pds.store.borrow();
    let entry = binding
        .values()
        .find(|e| e.0.derivation_input == DERIVATION_KEY)
        .unwrap();
    assert!(
        entry.1.is_none(),
        "entry must not have post_uri after finalize failure"
    );
}

// ═══════════════════════════════════════════════════════════════
//  Idempotence on attempt — rerun after FinalizeFailed refuses
// ═══════════════════════════════════════════════════════════════

#[test]
fn rerun_after_finalize_failed_refuses() {
    let mut pipeline = make_pipeline();
    let mut pds_fail = DurableMockPds::finalize_fails();
    let mut counter = MockCounter { count: 0 };

    // First run: finalize fails, post is live but entry pending.
    let result1 = pipeline.run(
        &clean_facts(),
        &mut pds_fail,
        &mut counter,
        HeartbeatState::Alive,
        &FixedClock,
        &Fnv1aHasher,
    );
    assert!(
        matches!(result1, PipelineResult::FinalizeFailed { .. }),
        "first run must fail at finalize"
    );

    // Second run with same repo@sha → Duplicate (pending in store).
    let store = pds_fail.store.into_inner();
    let mut pds_ok = DurableMockPds::from_store(store);
    let result2 = pipeline.run(
        &clean_facts(),
        &mut pds_ok,
        &mut counter,
        HeartbeatState::Alive,
        &FixedClock,
        &Fnv1aHasher,
    );
    assert_eq!(
        result2,
        PipelineResult::Duplicate,
        "rerun after FinalizeFailed must refuse — pending entry in the audit trail"
    );
    assert!(
        pds_ok.submitted.borrow().is_none(),
        "no second post attempt"
    );
}

// ═══════════════════════════════════════════════════════════════
//  Idempotence on attempt — rerun after PostFailed refuses
// ═══════════════════════════════════════════════════════════════

#[test]
fn rerun_after_post_failed_refuses() {
    let mut pipeline = make_pipeline();
    let mut pds_fail = DurableMockPds::submit_fails();
    let mut counter = MockCounter { count: 0 };

    let result1 = pipeline.run(
        &clean_facts(),
        &mut pds_fail,
        &mut counter,
        HeartbeatState::Alive,
        &FixedClock,
        &Fnv1aHasher,
    );
    assert!(matches!(result1, PipelineResult::PostFailed { .. }));

    // Same pipeline + pds → Duplicate (in-process mark set before submit).
    let result2 = pipeline.run(
        &clean_facts(),
        &mut pds_fail,
        &mut counter,
        HeartbeatState::Alive,
        &FixedClock,
        &Fnv1aHasher,
    );
    assert_eq!(
        result2,
        PipelineResult::Duplicate,
        "rerun after PostFailed must refuse — in-process mark set before submit"
    );
}

// ═══════════════════════════════════════════════════════════════
//  Duplicate after Success
// ═══════════════════════════════════════════════════════════════

#[test]
fn rerun_after_success_refuses() {
    let mut pipeline = make_pipeline();
    let mut pds = DurableMockPds::success();
    let mut counter = MockCounter { count: 0 };

    let result1 = pipeline.run(
        &clean_facts(),
        &mut pds,
        &mut counter,
        HeartbeatState::Alive,
        &FixedClock,
        &Fnv1aHasher,
    );
    assert!(matches!(result1, PipelineResult::Success { .. }));

    let result2 = pipeline.run(
        &clean_facts(),
        &mut pds,
        &mut counter,
        HeartbeatState::Alive,
        &FixedClock,
        &Fnv1aHasher,
    );
    assert_eq!(result2, PipelineResult::Duplicate);
}

// ═══════════════════════════════════════════════════════════════
//  r1: RESTART SIMULATION — fresh Pipeline, same store → Duplicate
// ═══════════════════════════════════════════════════════════════

#[test]
fn r1_restart_fresh_pipeline_same_store_refuses() {
    // First pipeline succeeds — entry finalized in the durable store.
    let mut pipeline_a = make_pipeline();
    let mut pds = DurableMockPds::success();
    let mut counter = MockCounter { count: 0 };

    let result_a = pipeline_a.run(
        &clean_facts(),
        &mut pds,
        &mut counter,
        HeartbeatState::Alive,
        &FixedClock,
        &Fnv1aHasher,
    );
    assert!(matches!(result_a, PipelineResult::Success { .. }));

    // Simulate process restart: DROP pipeline_a, construct a FRESH
    // Pipeline over the SAME durable store. The in-memory seen set
    // is empty, but the audit trail holds the entry.
    let store = pds.store.into_inner();
    let mut pipeline_b = make_pipeline();
    let mut pds_b = DurableMockPds::from_store(store);
    let mut counter_b = MockCounter { count: 0 };

    let result_b = pipeline_b.run(
        &clean_facts(),
        &mut pds_b,
        &mut counter_b,
        HeartbeatState::Alive,
        &FixedClock,
        &Fnv1aHasher,
    );
    assert_eq!(
        result_b,
        PipelineResult::Duplicate,
        "fresh pipeline over the same store must refuse — the audit \
         trail is the durable lock, not the in-process mark"
    );
    assert!(
        pds_b.submitted.borrow().is_none(),
        "no post attempt on restart"
    );
}

// ═══════════════════════════════════════════════════════════════
//  r2: CRASH RECOVERY — store pre-seeded with PENDING entry → refuses
// ═══════════════════════════════════════════════════════════════

#[test]
fn r2_crash_recovery_pending_entry_refuses() {
    // Simulate: a prior attempt crashed mid-flight after creating the
    // pending entry but before finalizing. The store holds a PENDING entry.
    let mut store = HashMap::new();
    let pending = PendingEntry {
        derivation_input: "skaists/LOVErnment-DAO@884b2bce".to_string(),
        input_digest: "deadbeef".to_string(),
        adapter_class: "TreeLanding".to_string(),
        adapter_digest: "adapter_digest_hash".to_string(),
        model_digest: "model_digest_hash".to_string(),
        prompt_digest: "prompt_digest_hash".to_string(),
        created_at: "2026-07-10T00:00:00Z".to_string(),
    };
    store.insert(
        "skaists/LOVErnment-DAO@884b2bce".to_string(),
        (pending, None, None, None),
    );

    let mut pipeline = make_pipeline();
    let mut pds = DurableMockPds::from_store(store);
    let mut counter = MockCounter { count: 0 };

    let result = pipeline.run(
        &clean_facts(),
        &mut pds,
        &mut counter,
        HeartbeatState::Alive,
        &FixedClock,
        &Fnv1aHasher,
    );
    assert_eq!(
        result,
        PipelineResult::Duplicate,
        "a pending entry in the audit trail must refuse — a prior \
         attempt crashed mid-flight. Recovery/clearance is a founder act."
    );
    assert!(
        pds.submitted.borrow().is_none(),
        "no post attempt when pending entry exists"
    );
}

// ═══════════════════════════════════════════════════════════════
//  r3: AMBIGUOUS TIMEOUT — submit errors, entry survives, rerun refuses
//  The post may have landed server-side; we can't know. The entry
//  surviving is the only protection against a double-post.
// ═══════════════════════════════════════════════════════════════

#[test]
fn r3_submit_error_survives_rerun_refuses() {
    // First pipeline: submit returns Err. In the real world the post
    // may have landed — the error is ambiguous (timeout, network reset).
    let mut pipeline_a = make_pipeline();
    let mut pds = DurableMockPds::submit_fails();
    let mut counter = MockCounter { count: 0 };

    let result_a = pipeline_a.run(
        &clean_facts(),
        &mut pds,
        &mut counter,
        HeartbeatState::Alive,
        &FixedClock,
        &Fnv1aHasher,
    );
    assert!(
        matches!(result_a, PipelineResult::PostFailed { .. }),
        "first run must fail at submit"
    );

    // The entry SURVIVED in the store (marked failed, not removed).
    assert!(
        pds.store.borrow().values().any(|e| e.0.derivation_input == DERIVATION_KEY),
        "entry must survive post failure — the durable lock"
    );

    // Fresh pipeline over the same store — must refuse. The entry
    // is the only thing preventing a double-post.
    let store = pds.store.into_inner();
    let mut pipeline_b = make_pipeline();
    let mut pds_b = DurableMockPds::from_store(store);
    let mut counter_b = MockCounter { count: 0 };

    let result_b = pipeline_b.run(
        &clean_facts(),
        &mut pds_b,
        &mut counter_b,
        HeartbeatState::Alive,
        &FixedClock,
        &Fnv1aHasher,
    );
    assert_eq!(
        result_b,
        PipelineResult::Duplicate,
        "fresh pipeline must refuse — the surviving entry is the only \
         protection against a double-post when the submit error was ambiguous"
    );
    assert!(
        pds_b.submitted.borrow().is_none(),
        "no second post attempt — double-post window closed"
    );
}
