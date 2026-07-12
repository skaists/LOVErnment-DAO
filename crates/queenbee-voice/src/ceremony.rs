//! THE FIRST WORD ceremony — the runner logic (D-009c).
//!
//! Wires the REAL sha256 hasher and RFC3339/micros wall clock into the merged
//! pipeline, runs it exactly once, and returns the matched (post, audit-entry)
//! pair. Env-reading, the live transport, and git are in the `first_word`
//! binary; this module is offline-testable with a mock `PdsClient`.

use crate::adapter::tree_landing::CommitFacts;
use crate::heartbeat::HeartbeatState;
use crate::pds::live_client::{AUDIT_COLLECTION, BQUEENBEE_DID};
use crate::pipeline::{Clock, Hasher, PdsClient, Pipeline, PipelineResult};
use crate::wrapper::DailyCounter;
use sha2::{Digest, Sha256};

/// Real SHA-256 for the audit `inputDigest` — replaces the test doubles.
pub struct RealSha256;

impl Hasher for RealSha256 {
    fn sha256_hex(&self, input: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input);
        hasher.finalize().iter().map(|b| format!("{b:02x}")).collect()
    }
}

/// Wall clock: RFC 3339 (seconds precision, `Z`) for `createdAt`, and micros
/// since the Unix epoch for the tid rkey.
pub struct SystemClock;

impl Clock for SystemClock {
    fn now_rfc3339(&self) -> String {
        chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
    }
    fn now_micros(&self) -> u64 {
        chrono::Utc::now().timestamp_micros().max(0) as u64
    }
}

/// A single-shot daily counter for the one ceremony post (starts at 0-of-3).
pub struct ZeroCounter(pub u64);

impl DailyCounter for ZeroCounter {
    fn count_today(&self) -> u64 {
        self.0
    }
    fn increment(&mut self) {
        self.0 += 1;
    }
}

/// The matched pair the ceremony reports.
#[derive(Debug, Clone)]
pub struct CeremonyReport {
    pub post_uri: String,
    pub post_cid: String,
    pub audit_uri: String,
    pub derivation_input: String,
    pub input_digest: String,
    /// Cross-reference proof (Q-6): the audit entry, re-read by its
    /// derivationInput field, carries a `postUri` equal to the live post uri.
    pub cross_references: bool,
}

/// Run the pipeline once against `client` for the genesis `facts` and return
/// the matched pair. Any non-Success outcome is an error — the ceremony
/// produces exactly one post and its audit entry, or nothing.
pub fn run_first_word<C: PdsClient>(
    client: &mut C,
    facts: &CommitFacts,
    adapter_digest: String,
    model_digest: String,
    prompt_digest: String,
    clock: &dyn Clock,
    hasher: &dyn Hasher,
) -> Result<CeremonyReport, String> {
    let mut pipeline = Pipeline::new(adapter_digest, model_digest, prompt_digest);
    let mut counter = ZeroCounter(0);
    match pipeline.run(facts, client, &mut counter, HeartbeatState::Alive, clock, hasher) {
        PipelineResult::Success { entry } => {
            let audit_uri = format!("at://{BQUEENBEE_DID}/{AUDIT_COLLECTION}/{}", entry.rkey);
            // Q-6: re-read the entry by its derivationInput FIELD and confirm
            // it cross-references the post.
            let cross_references =
                match client.find_entry_by_derivation_input(&entry.pending.derivation_input) {
                    Ok(Some(found)) => {
                        found.post_uri.as_deref() == Some(entry.post_uri.as_str())
                    }
                    _ => false,
                };
            Ok(CeremonyReport {
                post_uri: entry.post_uri,
                post_cid: entry.post_cid,
                audit_uri,
                derivation_input: entry.pending.derivation_input,
                input_digest: entry.pending.input_digest,
                cross_references,
            })
        }
        other => Err(format!("ceremony did not produce a post: {other:?}")),
    }
}
