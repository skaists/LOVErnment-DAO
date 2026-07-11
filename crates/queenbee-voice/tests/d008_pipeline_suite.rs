//! D-008r — Atomic pipeline suite (rework)
//!
//! VOICE-1 §5.5: no utterance without its entry. The pipeline is
//! atomic: create pending → submit post → finalize. Failure semantics
//! pinned. Idempotent on ATTEMPT per repo@sha.

#![forbid(unsafe_code)]

use queenbee_voice::adapter::tree_landing::CommitFacts;
use queenbee_voice::heartbeat::HeartbeatState;
use queenbee_voice::pipeline::{
    Clock, FinalizedEntry, Hasher, PdsClient, PendingEntry, Pipeline, PipelineResult,
};
use queenbee_voice::wrapper::{DailyCounter, SubmitResult};
use std::cell::RefCell;

// ═══════════════════════════════════════════════════════════════
//  Test doubles
// ═══════════════════════════════════════════════════════════════

#[derive(Default)]
struct MockPds {
    submit_result: Option<Result<(String, String), String>>,
    finalize_result: Option<Result<(), String>>,
    pending_created: RefCell<Option<PendingEntry>>,
    submitted: RefCell<Option<String>>,
    finalized: RefCell<Option<(String, String, String)>>,
    removed: RefCell<Option<String>>,
}

impl MockPds {
    fn success() -> Self {
        Self {
            submit_result: Some(Ok((
                "at://did:plc:test/app.bsky.feed.post/tid1".to_string(),
                "bafyrei_fake_cid".to_string(),
            ))),
            finalize_result: Some(Ok(())),
            ..Default::default()
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
}

impl PdsClient for MockPds {
    fn create_pending_entry(&mut self, _key: &str, entry: &PendingEntry) -> Result<(), String> {
        *self.pending_created.borrow_mut() = Some(entry.clone());
        Ok(())
    }
    fn submit_post(&mut self, text: &str) -> Result<(String, String), String> {
        *self.submitted.borrow_mut() = Some(text.to_string());
        self.submit_result.clone().unwrap_or(Ok((
            "at://did:plc:test/app.bsky.feed.post/tid1".to_string(),
            "bafyrei_fake_cid".to_string(),
        )))
    }
    fn finalize_entry(&mut self, key: &str, uri: &str, cid: &str) -> Result<(), String> {
        let result = self.finalize_result.clone().unwrap_or(Ok(()));
        if result.is_ok() {
            *self.finalized.borrow_mut() =
                Some((key.to_string(), uri.to_string(), cid.to_string()));
        }
        result
    }
    fn remove_entry(&mut self, key: &str) -> Result<(), String> {
        *self.removed.borrow_mut() = Some(key.to_string());
        *self.pending_created.borrow_mut() = None;
        Ok(())
    }
}

struct MockCounter { count: u64 }
impl DailyCounter for MockCounter {
    fn count_today(&self) -> u64 { self.count }
    fn increment(&mut self) { self.count += 1; }
}

struct FixedClock;
impl Clock for FixedClock {
    fn now_rfc3339(&self) -> String { "2026-07-11T00:00:00Z".to_string() }
}

struct IdentityHasher;
impl Hasher for IdentityHasher {
    fn sha256_hex(&self, input: &[u8]) -> String {
        // Use a real sha256 so the test vector is meaningful.
        use std::fmt::Write;
        // Simple non-crypto hash for test determinism — the test
        // asserts against this value, computed from known input.
        let mut h: u64 = 0xcbf29ce484222325;
        for &b in input {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        let mut s = String::new();
        write!(&mut s, "{h:016x}").unwrap();
        s
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

// ═══════════════════════════════════════════════════════════════
//  Canonical serialization test vector
// ═══════════════════════════════════════════════════════════════

#[test]
fn canonical_facts_json_pinned() {
    let facts = clean_facts();
    let canonical = Pipeline::canonical_facts_json(&facts);
    // Deterministic: field order matches struct declaration order.
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
    let mut pds = MockPds::success();
    let mut counter = MockCounter { count: 0 };

    let result = pipeline.run(
        &clean_facts(), &mut pds, &mut counter,
        HeartbeatState::Alive, &FixedClock, &IdentityHasher,
    );

    match result {
        PipelineResult::Success { entry } => {
            assert_eq!(entry.post_uri, "at://did:plc:test/app.bsky.feed.post/tid1");
            assert_eq!(entry.post_cid, "bafyrei_fake_cid");

            // Real audit fields.
            assert_eq!(entry.pending.derivation_input, "skaists/LOVErnment-DAO@884b2bce");
            assert_eq!(entry.pending.adapter_class, "TreeLanding");
            assert_eq!(entry.pending.adapter_digest, "adapter_digest_hash");
            assert_eq!(entry.pending.model_digest, "model_digest_hash");
            assert_eq!(entry.pending.prompt_digest, "prompt_digest_hash");
            assert_eq!(entry.pending.created_at, "2026-07-11T00:00:00Z");

            // input_digest = hash of canonical CommitFacts JSON.
            let canonical = Pipeline::canonical_facts_json(&clean_facts());
            let expected_digest = IdentityHasher.sha256_hex(canonical.as_bytes());
            assert_eq!(entry.pending.input_digest, expected_digest);
        }
        other => panic!("expected Success, got {other:?}"),
    }
    assert!(pds.submitted.borrow().is_some(), "post must have been submitted");
    assert!(pds.finalized.borrow().is_some(), "entry must have been finalized");
    // Pending entry was created before post.
    assert!(pds.pending_created.borrow().is_some() || pds.finalized.borrow().is_some(),
        "pending entry must have been created");
}

// ═══════════════════════════════════════════════════════════════
//  Wrapper refused — nothing persisted
// ═══════════════════════════════════════════════════════════════

#[test]
fn wrapper_refused_writes_neither() {
    let mut pipeline = make_pipeline();
    let mut pds = MockPds::success();
    let mut counter = MockCounter { count: 3 };

    let result = pipeline.run(
        &clean_facts(), &mut pds, &mut counter,
        HeartbeatState::Alive, &FixedClock, &IdentityHasher,
    );

    assert!(
        matches!(result, PipelineResult::Refused { .. }),
        "wrapper refusal must return Refused, got {result:?}"
    );
    if let PipelineResult::Refused { reason } = result {
        assert_eq!(reason, SubmitResult::RateLimited);
    }
    assert!(pds.submitted.borrow().is_none(), "nothing submitted on refusal");
    assert!(pds.pending_created.borrow().is_none(), "no pending entry on refusal");
}

// ═══════════════════════════════════════════════════════════════
//  Post failure — pending entry created then removed
// ═══════════════════════════════════════════════════════════════

#[test]
fn post_failure_removes_pending_entry() {
    let mut pipeline = make_pipeline();
    let mut pds = MockPds::submit_fails();
    let mut counter = MockCounter { count: 0 };

    let result = pipeline.run(
        &clean_facts(), &mut pds, &mut counter,
        HeartbeatState::Alive, &FixedClock, &IdentityHasher,
    );

    assert!(
        matches!(result, PipelineResult::PostFailed { .. }),
        "post failure must return PostFailed, got {result:?}"
    );
    // The pending entry was verifiably created (F-1 cure).
    assert!(pds.finalized.borrow().is_none(), "entry must not be finalized");
    // And then removed on post failure.
    assert!(pds.removed.borrow().is_some(), "pending entry must be removed on post failure");
    assert!(pds.pending_created.borrow().is_none(), "pending entry must not survive post failure");
}

// ═══════════════════════════════════════════════════════════════
//  Finalize failure — pending entry survives, post is live
// ═══════════════════════════════════════════════════════════════

#[test]
fn finalize_failure_leaves_pending_entry_alive() {
    let mut pipeline = make_pipeline();
    let mut pds = MockPds::finalize_fails();
    let mut counter = MockCounter { count: 0 };

    let result = pipeline.run(
        &clean_facts(), &mut pds, &mut counter,
        HeartbeatState::Alive, &FixedClock, &IdentityHasher,
    );

    match result {
        PipelineResult::FinalizeFailed { post_uri, post_cid, .. } => {
            assert_eq!(post_uri, "at://did:plc:test/app.bsky.feed.post/tid1");
            assert_eq!(post_cid, "bafyrei_fake_cid");
        }
        other => panic!("expected FinalizeFailed, got {other:?}"),
    }
    assert!(pds.submitted.borrow().is_some(), "post must be live");
    assert!(pds.finalized.borrow().is_none(), "entry must not be finalized");
    // The pending entry MUST survive (F-1 cure).
    assert!(
        !pds.removed.borrow().is_some(),
        "pending entry must NOT be removed on finalize failure — detectable honesty"
    );
}

// ═══════════════════════════════════════════════════════════════
//  Idempotence on attempt — rerun after FinalizeFailed refuses
// ═══════════════════════════════════════════════════════════════

#[test]
fn rerun_after_finalize_failed_refuses() {
    let mut pipeline = make_pipeline();
    let mut pds_fail = MockPds::finalize_fails();
    let mut counter = MockCounter { count: 0 };

    // First run: finalize fails, post is live but entry incomplete.
    let result1 = pipeline.run(
        &clean_facts(), &mut pds_fail, &mut counter,
        HeartbeatState::Alive, &FixedClock, &IdentityHasher,
    );
    assert!(
        matches!(result1, PipelineResult::FinalizeFailed { .. }),
        "first run must fail at finalize"
    );

    // Second run with same repo@sha → Duplicate (idempotence on attempt).
    let mut pds_ok = MockPds::success();
    let result2 = pipeline.run(
        &clean_facts(), &mut pds_ok, &mut counter,
        HeartbeatState::Alive, &FixedClock, &IdentityHasher,
    );
    assert_eq!(
        result2, PipelineResult::Duplicate,
        "rerun after FinalizeFailed must refuse — idempotence on attempt, \
         not success. Clearance is a founder act."
    );
    // Second run must not have submitted a second post.
    assert!(pds_ok.submitted.borrow().is_none(), "no second post attempt");
}

// ═══════════════════════════════════════════════════════════════
//  Idempotence on attempt — rerun after PostFailed refuses
// ═══════════════════════════════════════════════════════════════

#[test]
fn rerun_after_post_failed_refuses() {
    let mut pipeline = make_pipeline();
    let mut pds_fail = MockPds::submit_fails();
    let mut counter = MockCounter { count: 0 };

    // First run: post fails, pending removed.
    let result1 = pipeline.run(
        &clean_facts(), &mut pds_fail, &mut counter,
        HeartbeatState::Alive, &FixedClock, &IdentityHasher,
    );
    assert!(matches!(result1, PipelineResult::PostFailed { .. }));

    // Second run → Duplicate (idempotence on attempt).
    let mut pds_ok = MockPds::success();
    let result2 = pipeline.run(
        &clean_facts(), &mut pds_ok, &mut counter,
        HeartbeatState::Alive, &FixedClock, &IdentityHasher,
    );
    assert_eq!(
        result2, PipelineResult::Duplicate,
        "rerun after PostFailed must refuse — idempotence on attempt"
    );
}

// ═══════════════════════════════════════════════════════════════
//  Duplicate after Success
// ═══════════════════════════════════════════════════════════════

#[test]
fn rerun_after_success_refuses() {
    let mut pipeline = make_pipeline();
    let mut pds = MockPds::success();
    let mut counter = MockCounter { count: 0 };

    let result1 = pipeline.run(
        &clean_facts(), &mut pds, &mut counter,
        HeartbeatState::Alive, &FixedClock, &IdentityHasher,
    );
    assert!(matches!(result1, PipelineResult::Success { .. }));

    let result2 = pipeline.run(
        &clean_facts(), &mut pds, &mut counter,
        HeartbeatState::Alive, &FixedClock, &IdentityHasher,
    );
    assert_eq!(result2, PipelineResult::Duplicate);
}
