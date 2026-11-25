//! Rust warm-up exercises on the way to the Rautosa kernel.
//!
//! Nothing here runs on the target; every exercise is checked by unit tests
//! on the host:
//!
//! ```text
//! cargo test-host
//! ```
//!
//! The crate is still `no_std`, like the kernel: no heap, no `Vec`, no
//! `println!` outside of tests.

#![cfg_attr(not(test), no_std)]

pub mod ex00_ring;
