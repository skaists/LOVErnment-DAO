//! Tool wrapper — the single permitted verb.
//!
//! Q-3/Q-4/A-8: the wrapper's public surface exposes exactly one verb —
//! `submit_post` — and nothing else. No like/repost/follow/reply/delete.
//! `submit_post` refuses when: the UTC-day post count would exceed 3,
//! the target repo context is off-allowlist, or the heartbeat is stale.
//!
//! Rate counter and heartbeat are injected traits so tests control them.

use crate::adapter::tree_landing::CLASS1_ALLOWLIST;

/// UTC day for rate-counting (injected, no wall-time in tests).
pub trait DailyCounter {
    /// Returns the count of posts already made in the current UTC day.
    fn count_today(&self) -> u64;
    /// Increments the counter after a successful post.
    fn increment(&mut self);
}

/// Heartbeat check (injected — D-007 provides the real implementation).
pub trait HeartbeatCheck {
    /// Returns true if the voice is currently alive (heartbeat not stale).
    fn is_alive(&self) -> bool;
}

/// The one permitted verb on the pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verb {
    SubmitPost,
}

/// The public API surface — exactly this set, nothing else.
pub fn public_verbs() -> Vec<Verb> {
    vec![Verb::SubmitPost]
}

/// The daily post cap. VOICE-1 §6 G-A: ≤ 3 posts per day, hard cap.
pub const DAILY_CAP: u64 = 3;

/// Result of a `submit_post` attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubmitResult {
    /// Post accepted; counter was incremented.
    Accepted,
    /// Hard daily cap exceeded.
    RateLimited,
    /// Target repo is off the allowlist.
    OffAllowlist,
    /// Heartbeat is stale — posting suspended.
    Stale,
}

/// Submit a post through the wrapper's gates.
///
/// Q-4: rate enforced at the tool layer, not the prompt.
/// Q-3/A-8: only `submit_post` is reachable; no like/repost/follow/reply/delete.
pub fn submit_post(
    repo: &str,
    counter: &mut dyn DailyCounter,
    heartbeat: &dyn HeartbeatCheck,
) -> SubmitResult {
    // STUB — always accepts. Commit A lands this as red-first baseline.
    let _ = (repo, counter, heartbeat);
    SubmitResult::Accepted
}
