//! D-001 — First Compiler Contact: performance-set putRecord against the LIVE PDS.
//!
//! Docket: dockets/D-001.md (Owner: Seat 3). Spec (source of truth):
//!   specs/SPEC-performance-set.md @ 60ebd9cd9dca951e3ea5564c053dbda259e685e0
//!   lexicon: social.skaists.alpha.performance.set (+ .performance.defs)
//!
//! Steps 1-5 of the docket, self-cleaning:
//!   1. construct the minimal valid record (required fields only) + `cueTime`,
//!      the single optional field, included so SET-4 is asserted against real
//!      data rather than vacuously (SET-4 only bites when cueTime is present);
//!   2. putRecord to the live PDS under the skaists DID (key: tid -> we mint the
//!      rkey as a TID);
//!   3. getRecord round-trip by the returned at-uri; assert equivalence;
//!   4. assert SET-4 exactly as written (clause quoted verbatim below);
//!   5. deleteRecord — self-cleaning. Delete is attempted even if an assertion
//!      panics, so a failed run does not leave a record behind.
//!
//! Env contract (env-only, never committed, never logged): SKAISTS_PDS_URL,
//! SKAISTS_HANDLE, SKAISTS_APP_PASSWORD. Missing env -> SKIPPED-ENV (see note in
//! `env_trio`). The app password and the session JWT are NEVER printed.

use serde_json::{json, Value};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

const COLLECTION: &str = "social.skaists.alpha.performance.set";

/// The env trio, or None if any is absent/empty.
fn env_trio() -> Option<(String, String, String)> {
    let get = |k: &str| env::var(k).ok().filter(|v| !v.trim().is_empty());
    Some((
        get("SKAISTS_PDS_URL")?,
        get("SKAISTS_HANDLE")?,
        get("SKAISTS_APP_PASSWORD")?,
    ))
}

/// base32-sortable ("234567abcdefghijklmnopqrstuvwxyz"), 13 chars, MSB first.
fn s32_encode(mut n: u64) -> String {
    const ALPHABET: &[u8; 32] = b"234567abcdefghijklmnopqrstuvwxyz";
    let mut buf = [b'2'; 13];
    for i in (0..13).rev() {
        buf[i] = ALPHABET[(n & 31) as usize];
        n >>= 5;
    }
    String::from_utf8(buf.to_vec()).unwrap()
}

/// A TID rkey: 53-bit microsecond clock << 10 | 10-bit clock id, high bit 0.
fn generate_tid() -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let micros = (now.as_micros() as u64) & 0x1F_FFFF_FFFF_FFFF; // 53 bits
    let clock = (now.subsec_nanos() as u64) & 0x3FF; // 10 bits
    s32_encode((micros << 10) | clock)
}

/// Post JSON with a bearer token; panics with status only on error (never the
/// request body or token). Returns the parsed JSON response.
fn post_auth(url: &str, jwt: &str, body: Value) -> Value {
    ureq::post(url)
        .set("Authorization", &format!("Bearer {jwt}"))
        .send_json(body)
        .unwrap_or_else(|e| panic!("POST {url} failed: {}", describe(e)))
        .into_json()
        .expect("response was not JSON")
}

/// ureq error -> safe one-liner: status code / transport kind only. Never
/// includes headers (the JWT lives there) or the request body (the password).
fn describe(e: ureq::Error) -> String {
    match e {
        ureq::Error::Status(code, _resp) => format!("HTTP {code}"),
        ureq::Error::Transport(t) => format!("transport error: {}", t.kind()),
    }
}

#[test]
fn d001_performance_set_putrecord_roundtrip_live() {
    let (pds, handle, password) = match env_trio() {
        Some(t) => t,
        None => {
            // SKIPPED-ENV: a skipped D-001 is NOT the whistle. This test can only
            // go green by completing the live round-trip. Kept as a pass (not a
            // hard fail) so `cargo test --workspace` remains runnable without the
            // secrets, e.g. in CI; the acceptance claim is made only on a run
            // that printed the putRecord/getRecord/deleteRecord lines below.
            eprintln!(
                "SKIPPED-ENV: D-001 requires SKAISTS_PDS_URL / SKAISTS_HANDLE / \
                 SKAISTS_APP_PASSWORD in the environment; not run, not green."
            );
            return;
        }
    };
    let pds = pds.trim_end_matches('/').to_string();

    // (1) Minimal valid record. Required: setName, startedAt, items[>=1] with
    // {position, play{trackName, artists[>=1]{artistName}, playedTime}}.
    // startedAt <= playedTime satisfies SET-3. `cueTime` is the one optional
    // field, set consistently (5s after start) so SET-4 is asserted, not skipped.
    let started_at = "2026-07-11T00:00:00.000Z";
    let played_time = "2026-07-11T00:00:05.000Z";
    let record = json!({
        "$type": COLLECTION,
        "setName": "D-001 First Compiler Contact",
        "startedAt": started_at,
        "items": [
            {
                "position": 0,
                "play": {
                    "trackName": "First Compiler Contact",
                    "artists": [ { "artistName": "skaists" } ],
                    "playedTime": played_time
                },
                "cueTime": 5
            }
        ]
    });

    // (2) createSession -> did + accessJwt. Password used here and nowhere else;
    // never logged.
    let session = ureq::post(&format!("{pds}/xrpc/com.atproto.server.createSession"))
        .send_json(json!({ "identifier": handle, "password": password }))
        .unwrap_or_else(|e| panic!("createSession failed: {} (check handle / app password / PDS URL)", describe(e)))
        .into_json::<Value>()
        .expect("createSession: response was not JSON");
    let did = session["did"].as_str().expect("createSession: no did").to_string();
    let jwt = session["accessJwt"].as_str().expect("createSession: no accessJwt").to_string();

    // (2 cont.) putRecord. key: tid -> mint the rkey. validate=false: the PDS
    // does not carry the skaists lexicon; the assertions below are the validator.
    let rkey = generate_tid();
    let put = post_auth(
        &format!("{pds}/xrpc/com.atproto.repo.putRecord"),
        &jwt,
        json!({
            "repo": did,
            "collection": COLLECTION,
            "rkey": rkey,
            "validate": false,
            "record": record.clone()
        }),
    );
    let uri = put["uri"].as_str().expect("putRecord: no uri").to_string();
    println!("D-001 putRecord  -> {uri}");

    // (3-4) Round-trip + assertions, guarded so (5) deleteRecord runs even on a
    // panic (self-cleaning is not conditional on the record being correct).
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let got = ureq::get(&format!("{pds}/xrpc/com.atproto.repo.getRecord"))
            .query("repo", &did)
            .query("collection", COLLECTION)
            .query("rkey", &rkey)
            .call()
            .unwrap_or_else(|e| panic!("getRecord failed: {}", describe(e)))
            .into_json::<Value>()
            .expect("getRecord: response was not JSON");
        let value = &got["value"];

        // (3) Equivalence per the spec: what the PDS stored round-trips to what
        // we wrote (serde_json object comparison is order-independent).
        assert_eq!(value, &record, "round-trip value diverged from the written record");
        println!("D-001 getRecord  -> equivalence OK");

        // (4) SET-4, quoted verbatim from specs/SPEC-performance-set.md:
        //
        // **SET-4 — Redundancy is deliberate; disagreement is a MISMATCH.**
        // Where `cueTime` is present, `cueTime == floor(playedTime − startedAt)`
        // in seconds, tolerance ±1s for clock granularity. `cueTime` is a
        // derived field carried for cheap seeking. When derived and primary
        // disagree beyond tolerance, the record is rejected. It is **never**
        // repaired by recomputing `cueTime` from `playedTime` — that would
        // launder a corrupt record into a clean-looking one. *(no-reconstruction
        // rule)*
        assert_set4(value);
        println!("D-001 SET-4      -> asserted (cueTime == floor(playedTime − startedAt), ±1s)");
    }));

    // (5) deleteRecord — self-cleaning, best-effort even if (3)/(4) panicked.
    match ureq::post(&format!("{pds}/xrpc/com.atproto.repo.deleteRecord"))
        .set("Authorization", &format!("Bearer {jwt}"))
        .send_json(json!({ "repo": did, "collection": COLLECTION, "rkey": rkey }))
    {
        Ok(_) => println!("D-001 deleteRecord -> {uri} (self-cleaned)"),
        Err(e) => eprintln!("WARNING: deleteRecord failed ({}); record may persist at {uri}", describe(e)),
    }

    if let Err(p) = outcome {
        std::panic::resume_unwind(p);
    }
}

/// SET-4: where `cueTime` is present, `cueTime == floor(playedTime − startedAt)`
/// in seconds, tolerance ±1s. No-reconstruction rule: on disagreement the record
/// is REJECTED — cueTime is never recomputed from playedTime.
fn assert_set4(value: &Value) {
    let started = OffsetDateTime::parse(
        value["startedAt"].as_str().expect("startedAt missing"),
        &Rfc3339,
    )
    .expect("startedAt not RFC3339");

    for item in value["items"].as_array().expect("items missing") {
        let Some(cue) = item.get("cueTime").and_then(Value::as_i64) else {
            continue; // cueTime absent -> SET-4 does not bite for this item.
        };
        let played = OffsetDateTime::parse(
            item["play"]["playedTime"].as_str().expect("playedTime missing"),
            &Rfc3339,
        )
        .expect("playedTime not RFC3339");
        // playedTime >= startedAt by SET-3, so whole_seconds() (truncation toward
        // zero) is floor.
        let derived = (played - started).whole_seconds();
        assert!(
            (cue - derived).abs() <= 1,
            "SET-4 MISMATCH: cueTime={cue}, floor(playedTime − startedAt)={derived} (> ±1s). \
             no-reconstruction rule: reject, never recompute cueTime from playedTime."
        );
    }
}
