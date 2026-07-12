//! Live ATProto XRPC transport — the real network boundary.
//!
//! D-009c: implements `AuditRecordSource` (read) and `XrpcTransport`
//! (write) with authenticated HTTPS calls to a real PDS, sitting behind
//! the same trait boundary the offline laps proved against mocks. The
//! unit tests here exercise only the PURE response parsers; nothing in
//! `cargo test` opens a socket. The network paths run only at THE FIRST
//! WORD, invoked explicitly from the armed shell.
//!
//! Credential discipline (the D-001 leak lesson is law):
//! - The app password is used exactly ONCE — in `connect`, as the
//!   createSession request body — and is NEVER stored, logged, or echoed.
//!   Only the short-lived `accessJwt` and the public DID are retained.
//! - Errors surface a status/transport summary via [`status_only`],
//!   never a response body and never the request body.
//! - Before any write, the authenticated session's DID is verified to
//!   equal [`BQUEENBEE_DID`] — fail-closed against posting as the wrong
//!   account.

use super::live_client::{
    AuditRecord, AuditRecordSource, RecordsPage, XrpcTransport, AUDIT_COLLECTION, BQUEENBEE_DID,
};
use serde_json::Value;

/// A verified session: the bearer token and the DID it authenticates.
pub(crate) struct Session {
    pub access_jwt: String,
    pub did: String,
}

/// Parse a `com.atproto.server.createSession` response into a [`Session`].
/// Fail-closed: a response missing `accessJwt` or `did` is an error, never
/// a session with empty fields.
pub(crate) fn parse_session_response(v: &Value) -> Result<Session, String> {
    // COMMIT A STUB — implemented in commit B.
    let _ = v;
    Err("parse_session_response: not implemented (commit A stub)".to_string())
}

/// Verify the authenticated session belongs to bQueenBee. Fail-closed:
/// any other DID is refused before a single write is attempted.
pub(crate) fn verify_did(did: &str) -> Result<(), String> {
    if did == BQUEENBEE_DID {
        Ok(())
    } else {
        Err(format!(
            "session DID '{did}' does not match expected bQueenBee DID — refusing to write"
        ))
    }
}

/// Extract the rkey (final path segment) from an at-uri of the form
/// `at://<did>/<collection>/<rkey>`. Returns `None` for an empty or
/// trailing-slash uri.
pub(crate) fn rkey_from_uri(uri: &str) -> Option<String> {
    // COMMIT A STUB — implemented in commit B.
    let _ = uri;
    None
}

/// Parse a `com.atproto.repo.listRecords` response into a [`RecordsPage`].
/// Each record maps to an [`AuditRecord`] (rkey from its uri, value carried
/// verbatim for the field scan). `cursor` is carried when present. A
/// response whose `records` is not an array is a malformed-listing error.
pub(crate) fn parse_list_response(v: &Value) -> Result<RecordsPage, String> {
    // COMMIT A STUB — implemented in commit B.
    let _ = v;
    Err("parse_list_response: not implemented (commit A stub)".to_string())
}

/// The live XRPC transport. Holds a verified session and implements both
/// trait boundaries. Cheap to clone (`ureq::Agent` is `Arc`-backed), so a
/// single session serves both the read and write sides of `LivePdsClient`.
#[derive(Clone)]
pub struct LiveXrpc {
    pds_url: String,
    access_jwt: String,
    agent: ureq::Agent,
}

impl LiveXrpc {
    /// Authenticate: `createSession` with `handle` + `app_password`, verify
    /// the returned DID is bQueenBee's, and retain only the `accessJwt`.
    /// The app password does not survive this call. Fail-closed on any
    /// transport error, parse error, or DID mismatch.
    pub fn connect(pds_url: &str, handle: &str, app_password: &str) -> Result<Self, String> {
        let pds_url = pds_url.trim_end_matches('/').to_string();
        let agent = ureq::AgentBuilder::new().build();
        let resp = agent
            .post(&format!("{pds_url}/xrpc/com.atproto.server.createSession"))
            .send_json(serde_json::json!({
                "identifier": handle,
                "password": app_password,
            }))
            .map_err(|e| format!("createSession failed: {}", status_only(&e)))?;
        let body: Value = resp
            .into_json()
            .map_err(|e| format!("createSession response parse: {e}"))?;
        let session = parse_session_response(&body)?;
        verify_did(&session.did)?;
        Ok(Self {
            pds_url,
            access_jwt: session.access_jwt,
            agent,
        })
    }

    /// POST an authenticated XRPC procedure, returning the parsed JSON body.
    fn authed_post(&self, nsid: &str, body: Value) -> Result<Value, String> {
        let resp = self
            .agent
            .post(&format!("{}/xrpc/{nsid}", self.pds_url))
            .set("Authorization", &format!("Bearer {}", self.access_jwt))
            .send_json(body)
            .map_err(|e| format!("{nsid} failed: {}", status_only(&e)))?;
        resp.into_json()
            .map_err(|e| format!("{nsid} response parse: {e}"))
    }
}

/// Reduce a ureq error to a status/transport summary. NEVER includes a
/// request body (which for createSession holds the app password) or a
/// response body.
fn status_only(e: &ureq::Error) -> String {
    match e {
        ureq::Error::Status(code, _response) => format!("HTTP {code}"),
        ureq::Error::Transport(_transport) => "transport error".to_string(),
    }
}

impl XrpcTransport for LiveXrpc {
    fn put_record(&mut self, body: Value) -> Result<Value, String> {
        self.authed_post("com.atproto.repo.putRecord", body)
    }
    fn create_record(&mut self, body: Value) -> Result<Value, String> {
        self.authed_post("com.atproto.repo.createRecord", body)
    }
    fn delete_record(&mut self, body: Value) -> Result<Value, String> {
        self.authed_post("com.atproto.repo.deleteRecord", body)
    }
}

impl AuditRecordSource for LiveXrpc {
    fn list_audit_records(&self, cursor: Option<String>) -> Result<RecordsPage, String> {
        let mut req = self
            .agent
            .get(&format!("{}/xrpc/com.atproto.repo.listRecords", self.pds_url))
            .set("Authorization", &format!("Bearer {}", self.access_jwt))
            .query("repo", BQUEENBEE_DID)
            .query("collection", AUDIT_COLLECTION);
        if let Some(c) = &cursor {
            req = req.query("cursor", c);
        }
        let resp = req
            .call()
            .map_err(|e| format!("listRecords failed: {}", status_only(&e)))?;
        let body: Value = resp
            .into_json()
            .map_err(|e| format!("listRecords response parse: {e}"))?;
        parse_list_response(&body)
    }
}

// ═══════════════════════════════════════════════════════════════════
//  Unit tests — the PURE response parsers only. No network, ever.
//  Red-first: the parsers are commit-A stubs; these fail until commit B.
// ═══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // ─── parse_session_response ────────────────────────────────────

    #[test]
    fn session_valid_extracts_jwt_and_did() {
        let v = json!({
            "accessJwt": "jwt-abc",
            "refreshJwt": "refresh-xyz",
            "did": "did:plc:77xbxwg7vh3wh5pmzvid65hc",
            "handle": "bqueenbee.beehivenature.com"
        });
        let s = parse_session_response(&v).expect("valid session must parse");
        assert_eq!(s.access_jwt, "jwt-abc");
        assert_eq!(s.did, "did:plc:77xbxwg7vh3wh5pmzvid65hc");
    }

    #[test]
    fn session_missing_access_jwt_is_err() {
        let v = json!({ "did": "did:plc:77xbxwg7vh3wh5pmzvid65hc" });
        assert!(
            parse_session_response(&v).is_err(),
            "missing accessJwt must fail-closed, never an empty token"
        );
    }

    #[test]
    fn session_missing_did_is_err() {
        let v = json!({ "accessJwt": "jwt-abc" });
        assert!(
            parse_session_response(&v).is_err(),
            "missing did must fail-closed"
        );
    }

    // ─── verify_did ────────────────────────────────────────────────

    #[test]
    fn verify_did_accepts_bqueenbee() {
        assert!(verify_did("did:plc:77xbxwg7vh3wh5pmzvid65hc").is_ok());
    }

    #[test]
    fn verify_did_rejects_other_account() {
        assert!(
            verify_did("did:plc:someoneelse000000000000000").is_err(),
            "a session for any other DID must be refused before a write"
        );
    }

    // ─── rkey_from_uri ─────────────────────────────────────────────

    #[test]
    fn rkey_from_at_uri_takes_final_segment() {
        assert_eq!(
            rkey_from_uri("at://did:plc:77xbxwg7vh3wh5pmzvid65hc/social.skaists.alpha.audit.entry/3k2aXyZ"),
            Some("3k2aXyZ".to_string())
        );
    }

    #[test]
    fn rkey_from_uri_empty_is_none() {
        assert_eq!(rkey_from_uri(""), None);
        assert_eq!(rkey_from_uri("at://did/coll/"), None);
    }

    // ─── parse_list_response ───────────────────────────────────────

    #[test]
    fn list_maps_records_rkey_value_and_cursor() {
        let v = json!({
            "records": [
                {
                    "uri": "at://did:plc:x/social.skaists.alpha.audit.entry/rk1",
                    "cid": "cid1",
                    "value": { "derivationInput": "skaists/LOVErnment-DAO@884b2bce" }
                },
                {
                    "uri": "at://did:plc:x/social.skaists.alpha.audit.entry/rk2",
                    "cid": "cid2",
                    "value": { "derivationInput": "other@sha" }
                }
            ],
            "cursor": "next-page"
        });
        let page = parse_list_response(&v).expect("valid listRecords must parse");
        assert_eq!(page.records.len(), 2);
        assert_eq!(page.records[0].rkey, "rk1");
        assert_eq!(
            page.records[0].value.get("derivationInput").and_then(|d| d.as_str()),
            Some("skaists/LOVErnment-DAO@884b2bce")
        );
        assert_eq!(page.records[1].rkey, "rk2");
        assert_eq!(page.cursor, Some("next-page".to_string()));
    }

    #[test]
    fn list_empty_is_empty_page_no_cursor() {
        let v = json!({ "records": [] });
        let page = parse_list_response(&v).expect("empty listRecords must parse");
        assert!(page.records.is_empty());
        assert_eq!(page.cursor, None);
    }

    #[test]
    fn list_records_not_array_is_err() {
        let v = json!({ "records": "not-an-array" });
        assert!(
            parse_list_response(&v).is_err(),
            "a malformed listing must be an error, never a silently-empty page (G-Q)"
        );
    }
}
