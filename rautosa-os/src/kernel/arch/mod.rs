//! Architecture port layer.
//!
//! The boundary between portable kernel logic and the CPU. Deciding what
//! exactly crosses this boundary is a design task of its own; a useful test is
//! "could the scheduler's unit tests run on the host with a fake of this?".
//!
//! Typical candidates: start the first task, request a context switch,
//! initialise a task's stack, mask/unmask interrupts, "am I in an ISR?".
//!
//! Rust mechanisms to compare: a trait with one implementation selected by
//! `cfg`, plain `cfg`-gated modules exposing the same free functions, or
//! `extern "Rust"` hooks resolved at link time (the approach `critical-section`
//! takes).

#[cfg(all(target_arch = "arm", target_os = "none"))]
mod cortex_m;
