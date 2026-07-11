//! bQueenBee publishing pipeline.
//!
//! Adapters, tool wrapper, heartbeat, and the atomic pipeline that
//! guarantees no utterance without its audit entry (VOICE-1 §5.5).
//! All code is offline — fixtures and mocks only; no live PDS.

#![forbid(unsafe_code)]

pub mod adapter;
pub mod wrapper;
pub mod heartbeat;
