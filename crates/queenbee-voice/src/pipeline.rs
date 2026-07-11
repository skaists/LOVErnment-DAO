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
//! - DURABLE idempotence: the PDS audit trail is queried before the
//!   in-process mark. Any existing entry (pending or finalized) for
//!   this derivation_input → refuse as Duplicate. A pending entry
//!   means a prior attempt crashed mid-flight — refuse and surface it.
//!   The in-memory seen set is the in-process fast path; the audit
//!   trail is the durable lock. Clearance is a founder act.

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

/// Read-back of an audit entry from the PDS store.
/// Used for durable idempotence: if find_entry_by_derivation_input
/// returns Some, the input has already been attempted — whether the
/// entry is pending (crashed mid-flight) or finalized (completed).
/// Clearance of a pending entry is a founder act, never auto-retry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEntry {
    pub pending: PendingEntry,
    pub post_uri: Option<String>,
    pub post_cid: Option<String>,
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
    /// Durable idempotence check — returns the entry if one exists
    /// for this derivation_input, whether pending or finalized.
    fn find_entry_by_derivation_input(&self, derivation_input: &str) -> Option<AuditEntry>;
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
    /// Same derivation_input already in the audit trail or in-process mark.
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
    /// submit post → finalize entry.
    ///
    /// Durable idempotence: the PDS audit trail is queried for any
    /// existing entry (pending or finalized) BEFORE the in-process
    /// mark. A pending entry means a prior attempt crashed mid-flight —
    /// refuse and surface it. The in-memory seen set is the fast path;
    /// the audit trail is the durable lock.
    pub fn run<C: PdsClient>(
        &mut self,
        facts: &CommitFacts,
        pds: &mut C,
        counter: &mut dyn DailyCounter,
        heartbeat_state: HeartbeatState,
        clock: &dyn Clock,
        hasher: &dyn Hasher,
    ) -> PipelineResult {
        let derivation_input = format!("{}@{}", facts.repo, facts.sha);

        // DURABLE idempotence: the audit trail is the lock.
        // Any existing entry (pending or finalized) → refuse.
        if pds.find_entry_by_derivation_input(&derivation_input).is_some() {
            return PipelineResult::Duplicate;
        }

        // In-process fast path.
        if self.seen.contains(&derivation_input) {
            return PipelineResult::Duplicate;
        }

        // Adapter: derive candidate post.
        let candidate = match derive_tree_landing(facts) {
            Some(c) => c,
            None => return PipelineResult::NoCandidate,
        };

        // Wrapper gate: heartbeat, allowlist, rate cap.
        struct HeartbeatBridge(HeartbeatState);
        impl crate::wrapper::HeartbeatCheck for HeartbeatBridge {
            fn is_alive(&self) -> bool {
                self.0.is_alive()
            }
        }
        let hb = HeartbeatBridge(heartbeat_state);

        let result = submit_post(&facts.repo, counter, &hb);
        if result != SubmitResult::Accepted {
            return PipelineResult::Refused { reason: result };
        }

        // MARK: in-process idempotence. Set before submit.
        self.seen.insert(derivation_input.clone());

        // Real audit fields.
        let canonical = Self::canonical_facts_json(facts);
        let input_digest = hasher.sha256_hex(canonical.as_bytes());
        let created_at = clock.now_rfc3339();

        let pending = PendingEntry {
            derivation_input: derivation_input.clone(),
            input_digest,
            adapter_class: "TreeLanding".to_string(),
            adapter_digest: self.adapter_digest.clone(),
            model_digest: self.model_digest.clone(),
            prompt_digest: self.prompt_digest.clone(),
            created_at,
        };

        // Create pending entry (real PDS record).
        if let Err(e) = pds.create_pending_entry(&derivation_input, &pending) {
            return PipelineResult::PostFailed { error: e };
        }

        // Submit post.
        let (post_uri, post_cid) = match pds.submit_post(&candidate.text) {
            Ok(t) => t,
            Err(e) => {
                let _ = pds.remove_entry(&derivation_input);
                return PipelineResult::PostFailed { error: e };
            }
        };

        // Finalize entry with postUri/postCid.
        if let Err(e) = pds.finalize_entry(&derivation_input, &post_uri, &post_cid) {
            return PipelineResult::FinalizeFailed {
                post_uri,
                post_cid,
                error: e,
            };
        }

        PipelineResult::Success {
            entry: FinalizedEntry {
                pending,
                post_uri,
                post_cid,
            },
        }
    }
}
