//! Heartbeat — the dead-man's switch.
//!
//! Q-5 / VOICE-1 §6 G-B: authority expires after 21 days without a
//! founder heartbeat. `is_alive(last_beat, now) = now < last_beat + 21d`.
//! A missed beat **suspends** posting; the next beat **resumes** it.
//! Suspension and resumption each generate an audit-entry payload
//! (`adapterClass: "system.heartbeat"`) so silence itself is ledgered.
//! Clock injected; no wall-time in tests.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// The heartbeat interval: 21 days. VOICE-1 §6 G-B.
pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(21 * 24 * 60 * 60);

/// An audit-entry payload generated on a heartbeat state transition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeartbeatAuditPayload {
    pub adapter_class: String,
    pub event: String,
}

/// The heartbeat state — alive or suspended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeartbeatState {
    Alive,
    Suspended,
}

impl HeartbeatState {
    pub fn is_alive(self) -> bool {
        matches!(self, HeartbeatState::Alive)
    }
}

/// Check whether the voice is currently alive.
///
/// Q-5: `now < last_beat + 21 days` → alive.
pub fn is_alive(last_beat: Duration, now: Duration) -> bool {
    now < last_beat + HEARTBEAT_INTERVAL
}

/// Evaluate a heartbeat check, producing audit payloads on state transitions.
///
/// Returns the current state and any transition payload (exactly one per
/// transition, not per check).
pub fn check_transition(
    prev_state: HeartbeatState,
    last_beat: Duration,
    now: Duration,
) -> (HeartbeatState, Option<HeartbeatAuditPayload>) {
    // STUB — always alive, never transitions. Commit A: red-first baseline.
    let _ = (last_beat, now);
    let _ = prev_state;
    (HeartbeatState::Alive, None)
}
