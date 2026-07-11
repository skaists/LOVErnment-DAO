//! Performance-set validation — product oracle for SPEC-performance-set v0.1.
//!
//! Source of truth: `specs/SPEC-performance-set.md` @ 60ebd9cd…
//! Lexicon: `social.skaists.alpha.performance.set` (+ `.defs`, `.setStatus`)
//!
//! Every invariant in §4 of the spec that is machine-decidable from the
//! record's own fields is enforced here. Rules that require auth context,
//! repo state, or consumer behavior are documented as excluded.
//!
//! ## Enforced via validate_set (12 rules)
//!
//! SET-1 (position total & dense) · SET-2 (time monotonic) · SET-3 (set
//! contains its tracks) · SET-4 (cueTime ±1s) · SET-5 (embed, never
//! reference fm.teal.*) · SET-6 (no floats — bpmMilli integer) · SET-11
//! (disclosure affirmative) · SET-12 (mixed is derived, not stored) ·
//! SET-13 (venue name required) · §5 (playedTime required at ingest) ·
//! §5 (artists required, minLength 1) · §5 (artists non-empty array).
//!
//! ## Enforced via validate_set_status (1 rule)
//!
//! STATUS-1 (setStatus is state, never append-only — `supersedes` rejected).
//! The remaining STATUS rules (STATUS-2 stale-render, STATUS-3
//! status-to-set mutation) are consumer/process rules, not decidable from
//! the record's own fields, and are documented in the excluded list below.
//!
//! ## Excluded (not decidable from record fields)
//!
//! - SET-7 (authorship follows performance) — requires ATProto auth context.
//! - SET-8 (eventUri is enrichment) — no negative case; dangling is valid.
//! - SET-9 (supersession confined to one repo) — requires the author's DID,
//!   which is not a field on the record (it is the repo host, not the record).
//! - SET-10 (superseded record not deleted) — repo operation, not a record
//!   property.
//! - STATUS-2 (stale never renders as live) — consumer rendering rule.
//! - STATUS-3 (status never promoted to set by mutation) — process rule.

#![forbid(unsafe_code)]

use serde_json::Value;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

/// A named validation rule from the spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rule {
    Set1,
    Set2,
    Set3,
    Set4,
    Set5,
    Set6,
    Set11,
    Set12,
    Set13,
    /// §5 ingest: playRef missing playedTime.
    IngestPlayedTime,
    /// §5 ingest: playRef missing artists.
    IngestArtistsMissing,
    /// §5 ingest: playRef artists array empty (minLength 1).
    IngestArtistsEmpty,
    /// STATUS-1: setStatus carrying supersedes.
    Status1,
}

impl Rule {
    pub fn as_str(self) -> &'static str {
        match self {
            Rule::Set1 => "SET-1",
            Rule::Set2 => "SET-2",
            Rule::Set3 => "SET-3",
            Rule::Set4 => "SET-4",
            Rule::Set5 => "SET-5",
            Rule::Set6 => "SET-6",
            Rule::Set11 => "SET-11",
            Rule::Set12 => "SET-12",
            Rule::Set13 => "SET-13",
            Rule::IngestPlayedTime => "INGEST-playedTime",
            Rule::IngestArtistsMissing => "INGEST-artists-missing",
            Rule::IngestArtistsEmpty => "INGEST-artists-empty",
            Rule::Status1 => "STATUS-1",
        }
    }
}

/// Validate a `performance.set` record. Returns the list of violated rules;
/// empty means the record passes all machine-decidable checks.
pub fn validate_set(record: &Value) -> Vec<Rule> {
    let mut v = Vec::new();

    let items = match record.get("items").and_then(|i| i.as_array()) {
        Some(a) if !a.is_empty() => a,
        _ => return v,
    };

    // ── SET-1: position is total and dense ──
    // "items[i].position strictly increasing, beginning at 0,
    //  no gaps, no duplicates."
    let mut positions: Vec<i64> = items
        .iter()
        .filter_map(|item| item.get("position").and_then(|p| p.as_i64()))
        .collect();
    positions.sort_unstable();
    let set1_ok = !positions.is_empty()
        && positions.iter().enumerate().all(|(i, &p)| p == i as i64);
    if !set1_ok {
        v.push(Rule::Set1);
    }

    // ── SET-2: time is monotonic ──
    // "items[i].play.playedTime is non-decreasing in i."
    let times: Vec<Option<i64>> = items
        .iter()
        .map(|item| {
            item.get("play")
                .and_then(|p| p.as_object())
                .and_then(|o| o.get("playedTime"))
                .and_then(|t| t.as_str())
                .and_then(parse_ts)
        })
        .collect();
    let set2_ok = times.windows(2).all(|w| match (w[0], w[1]) {
        (Some(a), Some(b)) => a <= b,
        _ => true,
    });
    if !set2_ok {
        v.push(Rule::Set2);
    }

    // ── SET-3: set contains its tracks ──
    // "startedAt ≤ items[0].play.playedTime. If endedAt is present,
    //  items[n-1].play.playedTime ≤ endedAt."
    let started_at = record
        .get("startedAt")
        .and_then(|s| s.as_str())
        .and_then(parse_ts);
    let ended_at = record
        .get("endedAt")
        .and_then(|s| s.as_str())
        .and_then(parse_ts);
    let first_play = times.first().copied().flatten();
    let last_play = times.last().copied().flatten();
    if let (Some(s), Some(f)) = (started_at, first_play) {
        if s > f {
            v.push(Rule::Set3);
        }
    }
    if let (Some(e), Some(l)) = (ended_at, last_play) {
        if l > e {
            v.push(Rule::Set3);
        }
    }

    // ── SET-4: cueTime matches floor(playedTime − startedAt) ±1s ──
    // "Where cueTime is present, cueTime == floor(playedTime − startedAt)
    //  in seconds, tolerance ±1s for clock granularity."
    if let Some(start) = started_at {
        for item in items {
            let cue = item.get("cueTime").and_then(|c| c.as_i64());
            let play_ts = item
                .get("play")
                .and_then(|p| p.as_object())
                .and_then(|o| o.get("playedTime"))
                .and_then(|t| t.as_str())
                .and_then(parse_ts);
            if let (Some(cue_time), Some(play_time)) = (cue, play_ts) {
                let expected = play_time - start;
                if (cue_time - expected).abs() > 1 {
                    v.push(Rule::Set4);
                    break;
                }
            }
        }
    }

    // ── SET-5: embed, never reference ──
    // "A set record MUST NOT contain an at-uri to any fm.teal.* record."
    if contains_teal_at_uri(record) {
        v.push(Rule::Set5);
    }

    // ── SET-6: no floats exist ──
    // "Tempo is bpmMilli, an integer in milli-BPM. Do not substitute
    //  a string. Do not round to integer BPM."
    for item in items {
        if let Some(bpm) = item.get("bpmMilli") {
            if bpm.is_string() || bpm.is_f64() {
                v.push(Rule::Set6);
                break;
            }
        }
    }

    // ── SET-11: disclosure is affirmative ──
    // "#performer.kind is required, and 'undisclosed' is a sayable
    //  value. Silence is not."
    if let Some(perfs) = record.get("performers").and_then(|p| p.as_array()) {
        if perfs.iter().any(|p| p.get("kind").is_none()) {
            v.push(Rule::Set11);
        }
    }

    // ── SET-12: agency is per-performer; "mixed" is derived ──
    // "Two humans, two machines, or any blend is a property of the
    //  performers array, not a field on the set. Do not store it."
    if record.get("performerAgency").is_some()
        || record.get("agency").is_some()
        || record.get("mixed").is_some()
    {
        v.push(Rule::Set12);
    }

    // ── SET-13: venue is a strict profile, name required ──
    // "name is required here and optional there."
    if let Some(venue) = record.get("venue") {
        let name_missing = venue.get("name").is_none()
            || venue
                .get("name")
                .and_then(|n| n.as_str())
                .map_or(false, |s| s.is_empty());
        if name_missing {
            v.push(Rule::Set13);
        }
    }

    // ── §5 ingest: playRef requires trackName, artists, playedTime ──
    // "A playView missing playedTime is rejected at ingest."
    // "artists ... required, minLength: 1"
    for item in items {
        if let Some(play) = item.get("play").and_then(|p| p.as_object()) {
            if play.get("playedTime").is_none() {
                v.push(Rule::IngestPlayedTime);
            }
            match play.get("artists") {
                None => v.push(Rule::IngestArtistsMissing),
                Some(a) => {
                    if a.as_array().map_or(false, |arr| arr.is_empty()) {
                        v.push(Rule::IngestArtistsEmpty);
                    }
                }
            }
        }
    }

    v
}

/// Validate a `performance.setStatus` record.
///
/// Currently enforces STATUS-1 only (setStatus must not carry `supersedes`).
/// STATUS-2 (stale-render) and STATUS-3 (status-to-set mutation) are consumer
/// and process rules — see the module-level excluded list.
pub fn validate_set_status(record: &Value) -> Vec<Rule> {
    let mut v = Vec::new();

    // STATUS-1: "setStatus is the sole record in this namespace that
    // is mutated in place. It is never append-only, never superseded,
    // never corrected."
    if record.get("supersedes").is_some() {
        v.push(Rule::Status1);
    }

    v
}

// ── internal helpers (used by the real implementation in commit B) ──

/// Parse an RFC 3339 timestamp to Unix epoch seconds.
fn parse_ts(s: &str) -> Option<i64> {
    OffsetDateTime::parse(s, &Rfc3339)
        .ok()
        .map(|dt| dt.unix_timestamp())
}

/// Check whether any at-uri string value in the JSON tree points at a
/// collection in the `fm.teal.*` namespace.
///
/// An at-uri is `at://authority/collection/rkey`. The authority may be a
/// DID (`did:plc:…`) or a handle. The collection is an NSID
/// (`ns.authority.name`). We check whether the collection component's
/// authority portion is `fm.teal` — i.e. the NSID starts with `fm.teal.`.
fn contains_teal_at_uri(value: &Value) -> bool {
    match value {
        Value::String(s) => is_teal_at_uri(s),
        Value::Array(a) => a.iter().any(contains_teal_at_uri),
        Value::Object(o) => o.values().any(contains_teal_at_uri),
        _ => false,
    }
}

/// True if `s` is an at-uri whose collection NSID is in the fm.teal namespace.
fn is_teal_at_uri(s: &str) -> bool {
    let rest = match s.strip_prefix("at://") {
        Some(r) => r,
        None => return false,
    };
    // authority is everything up to the first '/'
    let after_authority = match rest.find('/') {
        Some(i) => &rest[i + 1..],
        None => return false,
    };
    // collection is everything up to the next '/' (or end of string)
    let collection = match after_authority.find('/') {
        Some(i) => &after_authority[..i],
        None => after_authority,
    };
    // NSID structure: ns.authority.name — check if it starts with "fm.teal."
    collection.starts_with("fm.teal.")
}
