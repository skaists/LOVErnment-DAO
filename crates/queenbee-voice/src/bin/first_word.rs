//! THE FIRST WORD — bQueenBee's genesis utterance (D-009c).
//!
//! Run ONCE from the armed shell (`QUEENBEE_*` set). Reads the credentials
//! from the environment, connects to the live PDS (verifying the DID),
//! builds the genesis `CommitFacts` from the REAL `884b2bce` commit on the
//! DAO tree, and runs the merged pipeline a single time. The fail-closed
//! durable lock refuses any second run.
//!
//! The app password is read from the env and handed to `LiveXrpc::connect`;
//! it is never printed. Do not run twice — one utterance is the ceremony.

use queenbee_voice::adapter::tree_landing::{provenance_verified, CommitFacts};
use queenbee_voice::ceremony::{run_first_word, RealSha256, SystemClock};
use queenbee_voice::pds::live_client::LivePdsClient;
use queenbee_voice::pds::live_transport::LiveXrpc;
use queenbee_voice::pipeline::Hasher;
use std::process::Command;

/// The genesis derivation input (VOICE-1 §3): VOICE-1's own landing commit.
const GENESIS: &str = "884b2bce";
/// Audit-tuple labels (founder Ruling / D-009c docket).
const MODEL_DIGEST: &str = "opus-4-8";
const PROMPT_DIGEST: &str =
    "0c44a15d4aa9d1811751f1cd04c4cb4bb9aa56b925f68a942f4036ff358402cf";
/// adapterDigest = sha256 of the merged TreeLanding adapter source, embedded
/// at compile time and hashed at runtime (computed + reported below).
const TREE_LANDING_SRC: &[u8] = include_bytes!("../adapter/tree_landing.rs");

fn env_or_halt(key: &str) -> String {
    match std::env::var(key) {
        Ok(v) if !v.is_empty() => v,
        _ => {
            eprintln!("HALT: {key} not set — not launched from the armed shell.");
            std::process::exit(2);
        }
    }
}

fn git_out(args: &[&str]) -> Result<String, String> {
    let out = Command::new("git")
        .args(args)
        .output()
        .map_err(|e| format!("git failed to launch: {e}"))?;
    if !out.status.success() {
        return Err(format!("git {args:?} exited non-zero"));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn halt(msg: String, code: i32) -> ! {
    eprintln!("HALT: {msg}");
    std::process::exit(code);
}

fn main() {
    // 1. Credentials — env only, HALT if any missing.
    let pds_url = env_or_halt("QUEENBEE_PDS_URL");
    let handle = env_or_halt("QUEENBEE_HANDLE");
    let app_password = env_or_halt("QUEENBEE_APP_PASSWORD");

    // 2. Connect (verifies the session DID == bQueenBee before any write).
    let xrpc = LiveXrpc::connect(&pds_url, &handle, &app_password)
        .unwrap_or_else(|e| halt(format!("connect failed: {e}"), 3));

    // 3. Genesis CommitFacts from the REAL 884b2bce commit — do not fabricate.
    let sha = git_out(&["rev-parse", GENESIS]).unwrap_or_else(|e| halt(e, 4));
    let subject = git_out(&["log", "-1", "--format=%s", GENESIS]).unwrap_or_else(|e| halt(e, 4));
    let body = git_out(&["log", "-1", "--format=%b", GENESIS]).unwrap_or_default();

    // Provenance per Ruling A (DCO tree): reachable-on-main AND DCO present.
    let reachable = Command::new("git")
        .args(["merge-base", "--is-ancestor", GENESIS, "origin/main"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    let full_msg = git_out(&["log", "-1", "--format=%B", GENESIS]).unwrap_or_default();
    let dco = full_msg
        .lines()
        .any(|l| l.trim_start().starts_with("Signed-off-by:"));
    let provenance = provenance_verified(reachable, dco);
    if !provenance {
        halt(
            format!("genesis {GENESIS} not provenance-verified (reachable={reachable}, dco={dco})"),
            5,
        );
    }

    let facts = CommitFacts {
        repo: "skaists/LOVErnment-DAO".to_string(),
        sha,
        ref_name: "main".to_string(),
        subject,
        body,
        signature_verified: provenance,
    };

    // 4. Audit tuple: adapterDigest computed from the merged source.
    let adapter_digest = RealSha256.sha256_hex(TREE_LANDING_SRC);
    println!("adapterDigest (sha256 of merged TreeLanding source): {adapter_digest}");

    // 5. Run the pipeline ONCE.
    let mut client = LivePdsClient::new(xrpc.clone(), xrpc);
    let report = run_first_word(
        &mut client,
        &facts,
        adapter_digest,
        MODEL_DIGEST.to_string(),
        PROMPT_DIGEST.to_string(),
        &SystemClock,
        &RealSha256,
    )
    .unwrap_or_else(|e| halt(format!("ceremony did not post: {e}"), 6));

    // 6. Report the matched pair.
    println!("\nTHE FIRST WORD — bQueenBee has spoken.");
    println!("post URI:         {}", report.post_uri);
    println!("audit entry URI:  {}", report.audit_uri);
    println!("derivationInput:  {}", report.derivation_input);
    println!("inputDigest:      {}", report.input_digest);
    println!(
        "cross-reference:  {}",
        if report.cross_references {
            "CONFIRMED — the audit entry's postUri equals the post uri (Q-6)"
        } else {
            "NOT CONFIRMED — investigate before trusting"
        }
    );
}
