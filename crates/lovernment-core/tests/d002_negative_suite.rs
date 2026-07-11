//! D-002r — Negative Suite: records that MUST fail
//!
//! Spec: `social.skaists.alpha.performance.set`, v0.1 (APPROVED 2026-07-09)
//! Source: `specs/SPEC-performance-set.md` @ 60ebd9cd…
//! Docket: `dockets/D-002.md` sha256 766a7b78…
//!
//! Targets the PRODUCT validators:
//!   - `lovernment_core::performance::validate_set`
//!   - `lovernment_core::performance::validate_set_status`
//!
//! ## Coverage (13 negative cases)
//!
//! validate_set (12):
//! - **SET-1** — position gap [0, 1, 3]
//! - **SET-2** — playedTime decreases between consecutive items
//! - **SET-3** — startedAt after first item's playedTime
//! - **SET-4** — cueTime disagrees with floor(playedTime − startedAt) beyond ±1s
//! - **SET-5** — at-uri to fm.teal.* collection (NSID authority check, not substring)
//! - **SET-6** — bpmMilli as string instead of integer
//! - **SET-11** — performer.kind absent
//! - **SET-12** — "mixed" agency stored as field on set
//! - **SET-13** — venue missing required name
//! - **§5** — playRef missing playedTime (rejected at ingest)
//! - **§5** — playRef missing artists (required, minLength 1)
//! - **§5** — playRef with empty artists array
//!
//! validate_set_status (1):
//! - **STATUS-1** — setStatus carrying `supersedes` → reject
//!
//! ## Excluded (not machine-decidable from record fields)
//!
//! - **SET-7** (authorship follows performance) — requires ATProto auth context.
//! - **SET-8** (eventUri is enrichment) — no negative case; dangling eventUri
//!   is explicitly a valid set.
//! - **SET-9** (supersession confined to one repo) — requires the author's DID,
//!   which is not a field on the record; it is the repo host identity.
//! - **SET-10** (superseded record not deleted) — repo operation, not a
//!   property of a single record.
//! - **STATUS-2** (stale must not render as live) — consumer rendering rule.
//! - **STATUS-3** (status never promoted to set by mutation) — process rule.

#![forbid(unsafe_code)]

use lovernment_core::performance::{validate_set, validate_set_status, Rule};
use serde_json::{json, Value};

// ═══════════════════════════════════════════════════════════════
//  Base records
// ═══════════════════════════════════════════════════════════════

/// Minimal valid record — same shape as D-001's putRecord payload.
/// Required fields only + cueTime (so SET-4 is exercised, not vacuous).
fn minimal_valid_set() -> Value {
    json!({
        "$type": "social.skaists.alpha.performance.set",
        "setName": "D-002r minimal valid set",
        "startedAt": "2026-07-11T00:00:00Z",
        "items": [
            {
                "position": 0,
                "play": {
                    "trackName": "Base Track",
                    "artists": [{"artistName": "skaists"}],
                    "playedTime": "2026-07-11T00:00:05Z"
                },
                "cueTime": 5
            },
            {
                "position": 1,
                "play": {
                    "trackName": "Second Track",
                    "artists": [{"artistName": "Artist B"}],
                    "playedTime": "2026-07-11T00:00:10Z"
                },
                "cueTime": 10
            }
        ]
    })
}

/// Minimal valid setStatus — required fields only.
fn minimal_valid_set_status() -> Value {
    json!({
        "$type": "social.skaists.alpha.performance.setStatus",
        "updatedAt": "2026-07-11T00:00:00Z",
        "startedAt": "2026-07-11T00:00:00Z"
    })
}

// ═══════════════════════════════════════════════════════════════
//  Positive cases — minimal valid records MUST pass
// ═══════════════════════════════════════════════════════════════

#[test]
fn positive_minimal_valid_set_passes() {
    let violations = validate_set(&minimal_valid_set());
    assert!(
        violations.is_empty(),
        "minimal valid set must pass; got: {:?}",
        violations.iter().map(|r| r.as_str()).collect::<Vec<_>>()
    );
}

// ═══════════════════════════════════════════════════════════════
//  SET-1 — position is total and dense
// ═══════════════════════════════════════════════════════════════

#[test]
fn set1_position_gap() {
    // SET-1: "items[i].position strictly increasing, beginning at 0,
    // no gaps, no duplicates."
    //
    // Violation: positions [0, 1, 3] — gap at position 2.

    let mut record = minimal_valid_set();
    record["items"][1]["position"] = json!(3);

    let violations = validate_set(&record);
    assert!(
        violations.contains(&Rule::Set1),
        "position gap must be rejected; got: {:?}",
        violations.iter().map(|r| r.as_str()).collect::<Vec<_>>()
    );
}

// ═══════════════════════════════════════════════════════════════
//  SET-2 — time is monotonic
// ═══════════════════════════════════════════════════════════════

#[test]
fn set2_played_time_regression() {
    // SET-2: "items[i].play.playedTime is non-decreasing in i."
    //
    // Violation: second item's playedTime (00:00:03) precedes the
    // first item's (00:00:05). cueTime updated to match so SET-4
    // does not co-fire.

    let mut record = minimal_valid_set();
    record["items"][1]["play"]["playedTime"] = json!("2026-07-11T00:00:03Z");
    record["items"][1]["cueTime"] = json!(3);

    let violations = validate_set(&record);
    assert!(
        violations.contains(&Rule::Set2),
        "playedTime regression must be rejected; got: {:?}",
        violations.iter().map(|r| r.as_str()).collect::<Vec<_>>()
    );
}

// ═══════════════════════════════════════════════════════════════
//  SET-3 — set contains its tracks
// ═══════════════════════════════════════════════════════════════

#[test]
fn set3_started_at_after_first_play() {
    // SET-3: "startedAt ≤ items[0].play.playedTime."
    //
    // Violation: startedAt (00:00:06) is after the first item's
    // playedTime (00:00:05). cueTime removed so SET-4 does not co-fire.

    let mut record = minimal_valid_set();
    record["startedAt"] = json!("2026-07-11T00:00:06Z");
    for item in record["items"].as_array_mut().unwrap() {
        item.as_object_mut().unwrap().remove("cueTime");
    }

    let violations = validate_set(&record);
    assert!(
        violations.contains(&Rule::Set3),
        "startedAt after first playedTime must be rejected; got: {:?}",
        violations.iter().map(|r| r.as_str()).collect::<Vec<_>>()
    );
}

// ═══════════════════════════════════════════════════════════════
//  SET-4 — redundancy is deliberate; disagreement is a MISMATCH
// ═══════════════════════════════════════════════════════════════

#[test]
fn set4_cuetime_mismatch() {
    // SET-4: "Where cueTime is present, cueTime == floor(playedTime −
    // startedAt) in seconds, tolerance ±1s for clock granularity."
    //
    // Violation: item[0] cueTime is 999; expected 5 (floor of
    // 00:00:05 − 00:00:00 = 5s). Difference 994 ≫ 1s tolerance.

    let mut record = minimal_valid_set();
    record["items"][0]["cueTime"] = json!(999);

    let violations = validate_set(&record);
    assert!(
        violations.contains(&Rule::Set4),
        "cueTime mismatch must be rejected; got: {:?}",
        violations.iter().map(|r| r.as_str()).collect::<Vec<_>>()
    );
}

// ═══════════════════════════════════════════════════════════════
//  SET-5 — embed, never reference
// ═══════════════════════════════════════════════════════════════

#[test]
fn set5_fm_teal_at_uri() {
    // SET-5: "A set record MUST NOT contain an at-uri to any
    // fm.teal.* record. Tracks are carried by value."
    //
    // Violation: extra field carrying an at-uri whose collection NSID
    // is in the fm.teal namespace: at://did:plc:abc/fm.teal.alpha.feed.play/tid
    // The NSID authority is "fm.teal" — checked by parsing the at-uri
    // structure, not by substring search.

    let mut record = minimal_valid_set();
    record["tealPlayRef"] =
        json!("at://did:plc:abc/fm.teal.alpha.feed.play/tidXYZ");

    let violations = validate_set(&record);
    assert!(
        violations.contains(&Rule::Set5),
        "fm.teal at-uri must be rejected; got: {:?}",
        violations.iter().map(|r| r.as_str()).collect::<Vec<_>>()
    );
}

// ═══════════════════════════════════════════════════════════════
//  SET-6 — no floats exist
// ═══════════════════════════════════════════════════════════════

#[test]
fn set6_bpm_milli_as_string() {
    // SET-6: "Tempo is bpmMilli, an integer in milli-BPM
    // (174000 == 174.000). Do not substitute a string."
    //
    // Violation: item[0] bpmMilli is the string "174000".

    let mut record = minimal_valid_set();
    record["items"][0]["bpmMilli"] = json!("174000");

    let violations = validate_set(&record);
    assert!(
        violations.contains(&Rule::Set6),
        "bpmMilli as string must be rejected; got: {:?}",
        violations.iter().map(|r| r.as_str()).collect::<Vec<_>>()
    );
}

// ═══════════════════════════════════════════════════════════════
//  SET-11 — disclosure is affirmative
// ═══════════════════════════════════════════════════════════════

#[test]
fn set11_performer_kind_missing() {
    // SET-11: "#performer.kind is required, and 'undisclosed' is a
    // sayable value. Silence is not."
    //
    // Violation: performer object without a kind field.

    let mut record = minimal_valid_set();
    record["performers"] = json!([{"performerName": "DJ Test"}]);

    let violations = validate_set(&record);
    assert!(
        violations.contains(&Rule::Set11),
        "missing performer.kind must be rejected; got: {:?}",
        violations.iter().map(|r| r.as_str()).collect::<Vec<_>>()
    );
}

// ═══════════════════════════════════════════════════════════════
//  SET-12 — agency is per-performer; "mixed" is derived
// ═══════════════════════════════════════════════════════════════

#[test]
fn set12_mixed_agency_stored() {
    // SET-12: "Two humans, two machines, or any blend is a property
    // of the performers array, not a field on the set. Do not store it."
    //
    // Violation: set record carries a performerAgency field.

    let mut record = minimal_valid_set();
    record["performerAgency"] = json!("mixed");

    let violations = validate_set(&record);
    assert!(
        violations.contains(&Rule::Set12),
        "stored mixed-agency field must be rejected; got: {:?}",
        violations.iter().map(|r| r.as_str()).collect::<Vec<_>>()
    );
}

// ═══════════════════════════════════════════════════════════════
//  SET-13 — venue is a strict profile, name required
// ═══════════════════════════════════════════════════════════════

#[test]
fn set13_venue_missing_name() {
    // SET-13: "name is required here and optional there."
    //
    // Violation: venue object present without a name field.

    let mut record = minimal_valid_set();
    record["venue"] = json!({"locality": "Berlin"});

    let violations = validate_set(&record);
    assert!(
        violations.contains(&Rule::Set13),
        "venue without name must be rejected; got: {:?}",
        violations.iter().map(|r| r.as_str()).collect::<Vec<_>>()
    );
}

// ═══════════════════════════════════════════════════════════════
//  §5 ingest — playRef missing playedTime
// ═══════════════════════════════════════════════════════════════

#[test]
fn ingest_play_ref_missing_played_time() {
    // §5: "A playView missing playedTime is rejected at ingest. It is
    // not repaired. It is not backfilled from now(), from rkey TID,
    // or from position."
    //
    // Violation: item[0] play object has no playedTime field.
    // cueTime removed so SET-4 does not co-fire.

    let mut record = minimal_valid_set();
    record["items"][0]["play"]
        .as_object_mut()
        .unwrap()
        .remove("playedTime");
    record["items"][0]
        .as_object_mut()
        .unwrap()
        .remove("cueTime");

    let violations = validate_set(&record);
    assert!(
        violations.contains(&Rule::IngestPlayedTime),
        "playRef missing playedTime must be rejected; got: {:?}",
        violations.iter().map(|r| r.as_str()).collect::<Vec<_>>()
    );
}

// ═══════════════════════════════════════════════════════════════
//  §5 ingest — playRef missing artists
// ═══════════════════════════════════════════════════════════════

#[test]
fn ingest_play_ref_missing_artists() {
    // §5: playRef "artists" is "required, minLength: 1" (stricter
    // than teal's playView).
    //
    // Violation: item[0] play object has no artists field.

    let mut record = minimal_valid_set();
    record["items"][0]["play"]
        .as_object_mut()
        .unwrap()
        .remove("artists");

    let violations = validate_set(&record);
    assert!(
        violations.contains(&Rule::IngestArtistsMissing),
        "playRef missing artists must be rejected; got: {:?}",
        violations.iter().map(|r| r.as_str()).collect::<Vec<_>>()
    );
}

// ═══════════════════════════════════════════════════════════════
//  §5 ingest — playRef with empty artists array
// ═══════════════════════════════════════════════════════════════

#[test]
fn ingest_play_ref_empty_artists() {
    // §5: playRef "artists" is "required, minLength: 1."
    //
    // Violation: item[0] play has artists: [] (present but empty,
    // violating minLength: 1).

    let mut record = minimal_valid_set();
    record["items"][0]["play"]["artists"] = json!([]);

    let violations = validate_set(&record);
    assert!(
        violations.contains(&Rule::IngestArtistsEmpty),
        "playRef with empty artists must be rejected; got: {:?}",
        violations.iter().map(|r| r.as_str()).collect::<Vec<_>>()
    );
}

// ═══════════════════════════════════════════════════════════════
//  STATUS-1 — status is state; never append-only
// ═══════════════════════════════════════════════════════════════

#[test]
fn status1_set_status_with_supersedes() {
    // STATUS-1: "setStatus is the sole record in this namespace that
    // is mutated in place. It is never append-only, never superseded,
    // never corrected."
    //
    // Violation: setStatus record carrying a supersedes field.

    let mut record = minimal_valid_set_status();
    record["supersedes"] = json!({
        "uri": "at://did:plc:abc/social.skaists.alpha.performance.setStatus/self",
        "cid": "bafyrei_fake"
    });

    let violations = validate_set_status(&record);
    assert!(
        violations.contains(&Rule::Status1),
        "setStatus with supersedes must be rejected; got: {:?}",
        violations.iter().map(|r| r.as_str()).collect::<Vec<_>>()
    );
}
