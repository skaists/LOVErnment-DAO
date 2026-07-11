//! skaists LOVErnment core.
//!
//! First out-of-tree consumer of the Beehive Nature Reserve kernel,
//! pinned at `kernel-v0.1.0` — the smoke test in `tests/` exercises one
//! public escrow path across that boundary. The `cascade` module models
//! the fractal-consensus geometry (demo-grade, source-pinned, no
//! emission); governance logic beyond geometry remains gated in the
//! kernel quarantine.

#![forbid(unsafe_code)]

pub mod cascade;
pub mod performance;

pub use escrow_core;
