//! bQueenBee publishing pipeline.
//!
//! Adapters, tool wrapper, heartbeat, and the atomic pipeline that
//! guarantees no utterance without its audit entry (VOICE-1 §5.5).
//! The pipeline and its logic are exercised entirely offline against
//! mocks; only `pds::live_transport` (HTTPS) and the `first_word` binary
//! (D-009c ceremony) touch a real PDS, and never during `cargo test`.

#![forbid(unsafe_code)]

pub mod adapter;
pub mod ceremony;
pub mod wrapper;
pub mod heartbeat;
pub mod pipeline;
pub mod pds;
