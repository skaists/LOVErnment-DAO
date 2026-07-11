//! Atomic pipeline — no utterance without its entry.
//!
//! VOICE-1 §5.5: run(facts) executes: adapter → wrapper gate →
//! create pending audit entry (all fields except postUri/postCid) →
//! submit post → finalize entry with postUri/postCid.
//!
//! Failure semantics (pinned):
//! - adapter None or wrapper refusal → nothing persisted
//! - post submission fails → pending entry removed, no retry storm
//! - finalization fails after a live post → pending entry remains,
//!   visibly incomplete (detectable honesty, never silent success)
//! - exactly one post attempt per input, ever
//! - idempotence on ATTEMPT, not success: any prior attempt — Success,
//!   FinalizeFailed, or PostFailed — refuses rerun. The mark is set
//!   before submit. Clearance is a founder act, out of code's reach.

use crate::adapter::tree_landing::{derive_tree_landing, CommitFacts};
use crate::heartbeat::HeartbeatState;
use crate::wrapper::{submit_post, DailyCounter, SubmitResult};
use std::collections::HashSet;

/// Injected clock — returns RFC 3339 UTC timestamp. No wall-time in tests.
pub trait Clock {
    fn now_rfc3339(&self) -> String;
}

/// Injected SHA-256 hasher — returns hex digest of input bytes.
pub trait Hasher {
    fn sha256_hex(&self, input: &[u8]) -> String;
}

/// The audit entry's pending state — all fields except postUri/postCid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingEntry {
    pub derivation_input: String,
    pub input_digest: String,
    pub adapter_class: String,
    pub adapter_digest: String,
    pub model_digest: String,
    pub prompt_digest: String,
    pub created_at: String,
}

/// The finalized entry — pending fields plus the pinned post reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinalizedEntry {
    pub pending: PendingEntry,
    pub post_uri: String,
    pub post_cid: String,
}

/// Mock PDS client trait — the pipeline never touches the network directly.
///
/// Operations on `social.skaists.alpha.audit.entry` records:
/// - `create_pending_entry`: write the entry with all fields except
///   postUri/postCid. Returns Ok if the record was written.
/// - `submit_post`: submit the post text. Returns (uri, cid) on success.
/// - `finalize_entry`: update the entry with postUri/postCid. Returns
///   Ok if the finalization succeeded.
/// - `remove_entry`: delete a pending entry (cleanup on post failure).
pub trait PdsClient {
    fn create_pending_entry(&mut self, key: &str, entry: &PendingEntry) -> Result<(), String>;
    fn submit_post(&mut self, text: &str) -> Result<(String, String), String>;
    fn finalize_entry(&mut self, key: &str, uri: &str, cid: &str) -> Result<(), String>;
    fn remove_entry(&mut self, key: &str) -> Result<(), String>;
}

/// Result of a pipeline run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineResult {
    /// Post live, entry finalized.
    Success { entry: FinalizedEntry },
    /// Wrapper refused (rate, stale, off-allowlist). Nothing persisted.
    Refused { reason: SubmitResult },
    /// Adapter produced no candidate. Nothing persisted.
    NoCandidate,
    /// Post submission failed. Pending entry removed.
    PostFailed { error: String },
    /// Post live but entry finalization failed. Pending entry remains,
    /// visibly incomplete — detectable honesty.
    FinalizeFailed { post_uri: String, post_cid: String, error: String },
    /// Same repo@sha already attempted — idempotence on attempt.
    /// Clearance is a founder act, out of code's reach.
    Duplicate,
}

/// The pipeline — holds idempotence state and adapter/prompt digests.
pub struct Pipeline {
    seen: HashSet<String>,
    adapter_digest: String,
    model_digest: String,
    prompt_digest: String,
}

impl Pipeline {
    pub fn new(
        adapter_digest: String,
        model_digest: String,
        prompt_digest: String,
    ) -> Self {
        Self {
            seen: HashSet::new(),
            adapter_digest,
            model_digest,
            prompt_digest,
        }
    }

    /// Canonical serialization of CommitFacts for input_digest.
    ///
    /// The serialization is serde_json::to_string of the CommitFacts
    /// struct, which is deterministic: field order matches declaration
    /// order in the struct (repo, sha, ref_name, subject, body,
    /// signature_verified). The same facts always produce the same bytes.
    /// Pinned by a test vector in the suite.
    pub fn canonical_facts_json(facts: &CommitFacts) -> String {
        serde_json::to_string(facts)
            .expect("CommitFacts is always serializable")
    }

    /// Run the pipeline on one commit's facts.
    ///
    /// VOICE-1 §5.5 order: adapter → wrapper gate → create pending →
    /// submit post → finalize entry. Idempotence mark is set BEFORE
    /// submit — any prior attempt refuses rerun.
    pub fn run<C: PdsClient>(
        &mut self,
        facts: &CommitFacts,
        pds: &mut C,
        counter: &mut dyn DailyCounter,
        heartbeat_state: HeartbeatState,
        clock: &dyn Clock,
        hasher: &dyn Hasher,
    ) -> PipelineResult {
        // STUB — returns NoCandidate for all inputs. Commit A: red-first.
        let _ = (facts, pds, counter, heartbeat_state, clock, hasher);
        PipelineResult::NoCandidate
    }
}
