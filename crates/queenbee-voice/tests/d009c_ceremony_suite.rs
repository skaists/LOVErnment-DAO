//! D-009c — ceremony runner + Rulings A/B, offline against a mock PdsClient.
//! No network, ever. Proves: the tid rkey generator (Ruling B), the DCO
//! provenance check (Ruling A), the real sha256, and that run_first_word
//! wires them into the pipeline to produce a matched (post, audit) pair.

#![forbid(unsafe_code)]

use queenbee_voice::adapter::tree_landing::{provenance_verified, CommitFacts};
use queenbee_voice::ceremony::{run_first_word, RealSha256};
use queenbee_voice::pipeline::{
    tid_from_micros, AuditEntry, Clock, Hasher, PdsClient, PendingEntry, ScanError,
};
use std::cell::RefCell;
use std::collections::HashMap;

// ─── Ruling B: tid rkey generator ──────────────────────────────────

const TID_ALPHABET: &str = "234567abcdefghijklmnopqrstuvwxyz";

#[test]
fn tid_is_valid_rkey_shape() {
    let tid = tid_from_micros(1_752_192_000_000_000, 0);
    assert_eq!(tid.len(), 13, "a tid is 13 chars");
    assert!(!tid.contains('/'), "a tid rkey must never contain '/'");
    assert!(!tid.contains('@'), "a tid rkey must never contain '@'");
    assert!(
        tid.chars().all(|c| TID_ALPHABET.contains(c)),
        "every tid char must be in the base32-sortable alphabet"
    );
}

#[test]
fn tid_encodes_micros_distinctly_and_deterministically() {
    // Distinct timestamps -> distinct tids (the stub, a constant, fails this).
    assert_ne!(
        tid_from_micros(1_000_000, 0),
        tid_from_micros(2_000_000, 0),
        "different micros must produce different tids"
    );
    // Same input -> same tid.
    assert_eq!(
        tid_from_micros(1_752_192_000_000_000, 0),
        tid_from_micros(1_752_192_000_000_000, 0),
        "tid generation must be deterministic"
    );
    // Sortability: later micros sort >= earlier (base32-sortable).
    assert!(
        tid_from_micros(2_000_000, 0) > tid_from_micros(1_000_000, 0),
        "later timestamps must sort after earlier ones"
    );
}

// ─── Ruling A: DCO provenance ──────────────────────────────────────

#[test]
fn provenance_requires_reachable_and_dco() {
    assert!(
        provenance_verified(true, true),
        "reachable-on-main + DCO present -> verified (DCO tree, Ruling A)"
    );
    assert!(
        !provenance_verified(false, true),
        "a random off-tree sha (not reachable) -> refused"
    );
    assert!(
        !provenance_verified(true, false),
        "no DCO Signed-off-by -> refused"
    );
}

// ─── Real sha256 ───────────────────────────────────────────────────

#[test]
fn real_sha256_matches_known_vector() {
    // The canonical sha256("abc") test vector.
    assert_eq!(
        RealSha256.sha256_hex(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

// ─── Ceremony wiring: run_first_word -> matched pair ───────────────

/// A durable mock: create keyed by tid rkey, submit returns a post uri/cid,
/// finalize fills postUri/postCid, find scans by the derivationInput FIELD.
#[derive(Default)]
struct CeremonyMockPds {
    // rkey -> (pending, Option<post_uri>, Option<post_cid>)
    store: RefCell<HashMap<String, (PendingEntry, Option<String>, Option<String>)>>,
}

impl PdsClient for CeremonyMockPds {
    fn find_entry_by_derivation_input(
        &self,
        derivation_input: &str,
    ) -> Result<Option<AuditEntry>, ScanError> {
        Ok(self.store.borrow().values().find_map(|(p, uri, cid)| {
            if p.derivation_input == derivation_input {
                Some(AuditEntry {
                    pending: p.clone(),
                    post_uri: uri.clone(),
                    post_cid: cid.clone(),
                    failure_error: None,
                })
            } else {
                None
            }
        }))
    }
    fn create_pending_entry(&mut self, key: &str, entry: &PendingEntry) -> Result<(), String> {
        assert!(
            !key.contains('/') && !key.contains('@'),
            "invalid audit rkey (Ruling B): {key}"
        );
        self.store
            .borrow_mut()
            .insert(key.to_string(), (entry.clone(), None, None));
        Ok(())
    }
    fn submit_post(&mut self, _text: &str) -> Result<(String, String), String> {
        Ok((
            "at://did:plc:77xbxwg7vh3wh5pmzvid65hc/app.bsky.feed.post/genesis".to_string(),
            "bafyrei_genesis_cid".to_string(),
        ))
    }
    fn finalize_entry(
        &mut self,
        key: &str,
        _entry: &PendingEntry,
        uri: &str,
        cid: &str,
    ) -> Result<(), String> {
        if let Some(e) = self.store.borrow_mut().get_mut(key) {
            e.1 = Some(uri.to_string());
            e.2 = Some(cid.to_string());
        }
        Ok(())
    }
    fn remove_entry(&mut self, _key: &str) -> Result<(), String> {
        Ok(())
    }
    fn mark_entry_failed(&mut self, _key: &str, _error: &str) -> Result<(), String> {
        Ok(())
    }
}

struct TestClock;
impl Clock for TestClock {
    fn now_rfc3339(&self) -> String {
        "2026-07-12T00:00:00Z".to_string()
    }
    fn now_micros(&self) -> u64 {
        1_752_192_000_000_000
    }
}

fn genesis_facts() -> CommitFacts {
    CommitFacts {
        repo: "skaists/LOVErnment-DAO".to_string(),
        sha: "884b2bce08bc0c8b8d6fea0c449690eaaeee57d2".to_string(),
        ref_name: "main".to_string(),
        subject: "VOICE-1 v0.1 APPROVED: the first voice".to_string(),
        body: String::new(),
        // Provenance per Ruling A (the runner computes this from git).
        signature_verified: provenance_verified(true, true),
    }
}

#[test]
fn ceremony_produces_matched_cross_referencing_pair() {
    let mut client = CeremonyMockPds::default();
    let report = run_first_word(
        &mut client,
        &genesis_facts(),
        "adapter_digest".to_string(),
        "opus-4-8".to_string(),
        "prompt_digest".to_string(),
        &TestClock,
        &RealSha256,
    )
    .expect("the ceremony must produce a post and its audit entry");

    // The post exists.
    assert!(report.post_uri.starts_with("at://"), "post uri present");
    // The audit entry uri is the tid rkey under bQueenBee's audit collection.
    let expected_rkey = tid_from_micros(1_752_192_000_000_000, 0);
    assert_eq!(
        report.audit_uri,
        format!(
            "at://did:plc:77xbxwg7vh3wh5pmzvid65hc/social.skaists.alpha.audit.entry/{expected_rkey}"
        ),
        "audit uri = at://did/collection/<tid>, never the derivationInput"
    );
    assert!(!report.audit_uri.contains('@'), "audit rkey is a tid, not repo@sha");
    // The derivationInput is the canonical repo@sha.
    assert_eq!(
        report.derivation_input,
        "skaists/LOVErnment-DAO@884b2bce08bc0c8b8d6fea0c449690eaaeee57d2"
    );
    // Q-6: they cross-reference.
    assert!(
        report.cross_references,
        "the audit entry's postUri must equal the post uri (Q-6 matched pair)"
    );
}
