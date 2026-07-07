//! skaists LOVErnment core — scaffold only.
//!
//! First out-of-tree consumer of the Beehive Nature Reserve kernel,
//! pinned at `kernel-v0.1.0`. No governance or product logic lives here
//! yet (scope fence: that is the next lap); this crate exists to prove
//! the kernel consumes cleanly from outside its own tree, and the smoke
//! test in `tests/` exercises one public escrow path across that
//! boundary.

#![forbid(unsafe_code)]

pub use escrow_core;
