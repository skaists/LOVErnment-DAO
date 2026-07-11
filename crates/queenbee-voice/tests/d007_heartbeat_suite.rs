//! D-007 — Heartbeat suite
//!
//! Q-5 / VOICE-1 §6 G-B: 21-day dead-man's switch. Suspends posting on
//! a missed beat; resumes on the next. Suspension and resumption each
//! generate an audit payload exactly once per transition.

#![forbid(unsafe_code)]

use queenbee_voice::heartbeat::{
    check_transition, is_alive, HeartbeatAuditPayload, HeartbeatState, HEARTBEAT_INTERVAL,
};
use std::time::Duration;

const DAY: Duration = Duration::from_secs(24 * 60 * 60);
const SEC: Duration = Duration::from_secs(1);

// ═══════════════════════════════════════════════════════════════
//  Interval constant
// ═══════════════════════════════════════════════════════════════

#[test]
fn heartbeat_interval_is_21_days() {
    assert_eq!(
        HEARTBEAT_INTERVAL,
        Duration::from_secs(21 * 24 * 60 * 60),
        "VOICE-1 §6 G-B: heartbeat interval is 21 days"
    );
}

// ═══════════════════════════════════════════════════════════════
//  is_alive boundary
// ═══════════════════════════════════════════════════════════════

#[test]
fn fresh_beat_is_alive() {
    assert!(is_alive(Duration::ZERO, Duration::ZERO));
    assert!(is_alive(Duration::ZERO, DAY));
}

#[test]
fn twenty_one_days_minus_one_second_alive() {
    let beat = Duration::ZERO;
    let now = HEARTBEAT_INTERVAL - SEC;
    assert!(
        is_alive(beat, now),
        "21d − 1s must be alive"
    );
}

#[test]
fn twenty_one_days_plus_one_second_stale() {
    let beat = Duration::ZERO;
    let now = HEARTBEAT_INTERVAL + SEC;
    assert!(
        !is_alive(beat, now),
        "21d + 1s must be stale"
    );
}

// ═══════════════════════════════════════════════════════════════
//  Transition payloads — suspension and resumption
// ═══════════════════════════════════════════════════════════════

#[test]
fn suspension_payload_on_transition_to_suspended() {
    // Was alive, now stale → suspension payload
    let last_beat = Duration::ZERO;
    let now = HEARTBEAT_INTERVAL + SEC;

    let (state, payload) = check_transition(HeartbeatState::Alive, last_beat, now);

    assert_eq!(state, HeartbeatState::Suspended);
    assert!(
        payload.is_some(),
        "suspension transition must generate exactly one audit payload"
    );
    let p = payload.unwrap();
    assert_eq!(p.adapter_class, "system.heartbeat");
    assert!(
        p.event.contains("suspend"),
        "payload event must indicate suspension"
    );
}

#[test]
fn no_payload_when_already_suspended() {
    // Already suspended, still stale → no new payload
    let last_beat = Duration::ZERO;
    let now = HEARTBEAT_INTERVAL + DAY;

    let (state, payload) = check_transition(HeartbeatState::Suspended, last_beat, now);

    assert_eq!(state, HeartbeatState::Suspended);
    assert!(
        payload.is_none(),
        "no transition → no payload (not per check, per transition)"
    );
}

#[test]
fn resumption_payload_on_new_beat() {
    // Was suspended, new beat arrived → resumption payload
    let now = HEARTBEAT_INTERVAL + DAY; // well past stale
    let new_beat = now; // fresh beat right now

    let (state, payload) = check_transition(HeartbeatState::Suspended, new_beat, now);

    assert_eq!(state, HeartbeatState::Alive);
    assert!(
        payload.is_some(),
        "resumption transition must generate exactly one audit payload"
    );
    let p = payload.unwrap();
    assert_eq!(p.adapter_class, "system.heartbeat");
    assert!(
        p.event.contains("resume"),
        "payload event must indicate resumption"
    );
}

#[test]
fn no_payload_when_already_alive() {
    // Already alive, still alive → no new payload
    let beat = Duration::ZERO;
    let now = DAY;

    let (state, payload) = check_transition(HeartbeatState::Alive, beat, now);

    assert_eq!(state, HeartbeatState::Alive);
    assert!(
        payload.is_none(),
        "no transition → no payload"
    );
}
