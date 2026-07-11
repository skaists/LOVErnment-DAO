//! Adapter class 1 — TreeLanding.
//!
//! Announces merges to `main` on allowlisted trees. Input: a commit on
//! an allowlisted repo's main branch, signature-verified. Output: one
//! candidate post carrying the repo name, the short sha, the commit
//! subject **quoted as data**, and a link to the commit.
//!
//! Q-8: law-bearing product code, born red-first. Q-1: autonomy bounded
//! by derivation — she posts what she can derive from signed, verifiable
//! state. Signed proves provenance, never benignity.

use serde::{Deserialize, Serialize};

/// The allowlist of repos that class-1 (TreeLanding) may announce.
/// Adding a repo here is a founder gate per Q-8 / VOICE-1 §6 G-C.
// PUBLIC-CONSTANT
pub const CLASS1_ALLOWLIST: &[&str] = &[
    "skaists/LOVErnment-DAO",
    "beehive-nature/beehive-nature",
];

/// The fixed character limit for the subject in the post template.
const SUBJECT_LIMIT: usize = 120;

/// Verified facts about a commit, populated honestly by the pipeline.
/// The adapter's law is total over this input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitFacts {
    pub repo: String,
    pub sha: String,
    pub ref_name: String,
    pub subject: String,
    pub body: String,
    pub signature_verified: bool,
}

/// A candidate post produced by an adapter. The pipeline submits this
/// via the tool wrapper; it never reaches the network directly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CandidatePost {
    pub text: String,
}

/// Adapter class 1 — TreeLanding.
///
/// Pure function: no network, no git, no clock. Returns `None` when the
/// input does not meet the class-1 criteria (unsigned, off-main, off-
/// allowlist). Returns `Some(post)` when the input is a clean signed
/// commit on an allowlisted repo's main branch, with the subject quoted
/// verbatim as inert data — never obeyed, never interpolated into any
/// prompt context.
///
/// VOICE-1 §1: "one candidate post carrying the repo name, the short
/// sha, the commit subject quoted as data, and a link to the commit.
/// Nothing else, ever."
pub fn derive_tree_landing(facts: &CommitFacts) -> Option<CandidatePost> {
    // STUB — accept-all. Commit A lands this as red-first baseline.
    // Commit B replaces this with the real enforcement logic.
    let _ = facts;
    Some(CandidatePost {
        text: String::new(),
    })
}

/// Truncate a subject to the template's fixed limit, marking with `…`.
fn truncate_subject(subject: &str) -> String {
    if subject.chars().count() <= SUBJECT_LIMIT {
        subject.to_string()
    } else {
        let truncated: String = subject.chars().take(SUBJECT_LIMIT).collect();
        format!("{truncated}…")
    }
}

/// Build the commit URL for an allowlisted repo.
fn commit_url(repo: &str, sha: &str) -> String {
    format!("https://github.com/{repo}/commit/{sha}")
}

/// Short sha (first 7 chars).
fn short_sha(sha: &str) -> &str {
    if sha.len() >= 7 {
        &sha[..7]
    } else {
        sha
    }
}
