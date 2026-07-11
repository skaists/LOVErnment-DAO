//! D-006 — Tool wrapper suite
//!
//! Q-4: rate enforced at the tool layer (hard cap 3/day).
//! Q-3/A-8: public surface is exactly {submit_post} — nothing else.
//! Heartbeat injected as a trait (D-007 wires the real check).

#![forbid(unsafe_code)]

use queenbee_voice::wrapper::{
    public_verbs, submit_post, DailyCounter, HeartbeatCheck, SubmitResult, Verb, DAILY_CAP,
};
use std::cell::RefCell;

// ═══════════════════════════════════════════════════════════════
//  Test doubles
// ═══════════════════════════════════════════════════════════════

struct MockCounter {
    count: u64,
}

impl MockCounter {
    fn new(count: u64) -> Self {
        Self { count }
    }
}

impl DailyCounter for MockCounter {
    fn count_today(&self) -> u64 {
        self.count
    }
    fn increment(&mut self) {
        self.count += 1;
    }
}

struct AliveHeartbeat;
impl HeartbeatCheck for AliveHeartbeat {
    fn is_alive(&self) -> bool {
        true
    }
}

struct StaleHeartbeat;
impl HeartbeatCheck for StaleHeartbeat {
    fn is_alive(&self) -> bool {
        false
    }
}

// ═══════════════════════════════════════════════════════════════
//  API surface — exactly one verb
// ═══════════════════════════════════════════════════════════════

#[test]
fn api_surface_is_exactly_submit_post() {
    assert_eq!(
        public_verbs(),
        vec![Verb::SubmitPost],
        "Q-3/A-8: the only reachable verb is submit_post"
    );
}

#[test]
fn daily_cap_is_three() {
    assert_eq!(DAILY_CAP, 3, "VOICE-1 §6 G-A: ≤ 3 posts per day");
}

// ═══════════════════════════════════════════════════════════════
//  Rate limiting — 3rd accepted, 4th refused
// ═══════════════════════════════════════════════════════════════

#[test]
fn third_post_of_day_accepted() {
    let mut counter = MockCounter::new(2);
    let heartbeat = AliveHeartbeat;
    assert_eq!(
        submit_post("skaists/LOVErnment-DAO", &mut counter, &heartbeat),
        SubmitResult::Accepted,
        "3rd post (count=2) must be accepted — cap is 3"
    );
    assert_eq!(counter.count, 3, "counter must be incremented on accept");
}

#[test]
fn fourth_post_of_day_refused() {
    let mut counter = MockCounter::new(3);
    let heartbeat = AliveHeartbeat;
    assert_eq!(
        submit_post("skaists/LOVErnment-DAO", &mut counter, &heartbeat),
        SubmitResult::RateLimited,
        "4th post (count=3) must be refused — hard cap"
    );
    assert_eq!(counter.count, 3, "counter must NOT increment on refusal");
}

// ═══════════════════════════════════════════════════════════════
//  Day rollover resets counter
// ═══════════════════════════════════════════════════════════════

struct RolloverCounter {
    counts: RefCell<Vec<u64>>,
}

impl DailyCounter for RolloverCounter {
    fn count_today(&self) -> u64 {
        self.counts.borrow().last().copied().unwrap_or(0)
    }
    fn increment(&mut self) {
        let mut b = self.counts.borrow_mut();
        if let Some(last) = b.last_mut() {
            *last += 1;
        }
    }
}

impl RolloverCounter {
    fn rollover(&self) {
        self.counts.borrow_mut().push(0);
    }
}

#[test]
fn day_rollover_resets_counter() {
    let mut counter = RolloverCounter {
        counts: RefCell::new(vec![3]), // day 1: capped
    };
    let heartbeat = AliveHeartbeat;

    // Day 1: 4th post refused
    assert_eq!(
        submit_post("skaists/LOVErnment-DAO", &mut counter, &heartbeat),
        SubmitResult::RateLimited,
    );

    // Rollover to day 2
    counter.rollover();

    // Day 2: first post accepted
    assert_eq!(
        submit_post("skaists/LOVErnment-DAO", &mut counter, &heartbeat),
        SubmitResult::Accepted,
        "day rollover must reset the counter"
    );
}

// ═══════════════════════════════════════════════════════════════
//  Off-allowlist refusal
// ═══════════════════════════════════════════════════════════════

#[test]
fn off_allowlist_refused() {
    let mut counter = MockCounter::new(0);
    let heartbeat = AliveHeartbeat;
    assert_eq!(
        submit_post("evil/not-allowlisted", &mut counter, &heartbeat),
        SubmitResult::OffAllowlist,
        "off-allowlist repo must be refused"
    );
}

// ═══════════════════════════════════════════════════════════════
//  Stale heartbeat refusal
// ═══════════════════════════════════════════════════════════════

#[test]
fn stale_heartbeat_refused() {
    let mut counter = MockCounter::new(0);
    let heartbeat = StaleHeartbeat;
    assert_eq!(
        submit_post("skaists/LOVErnment-DAO", &mut counter, &heartbeat),
        SubmitResult::Stale,
        "stale heartbeat must suspend posting"
    );
    assert_eq!(counter.count, 0, "counter must NOT increment on stale refusal");
}
