//! Kernel internals — **this is the part you write.**
//!
//! Nothing in here is implemented, and nothing is prescribed. The submodules
//! are empty on purpose and only contain questions, specification pointers and
//! Rust pitfalls. Rename, merge or delete them as your design evolves.
//!
//! Suggested order (matches `docs/roadmap.md`, stage 1):
//!
//! 1. [`task`] — the task control block and the state machine, testable on
//!    the host without any hardware.
//! 2. [`scheduler`] — the ready queue and the scheduling decision, also
//!    host-testable.
//! 3. [`arch`] — context switch on Cortex-M; the first part that needs QEMU.
//! 4. Wire the public services in `crate::task` to 1–3, then resources,
//!    events, counters and alarms.
//!
//! # Host tests
//!
//! Everything that does not touch registers can be unit-tested on your Mac:
//!
//! ```text
//! cargo test-host
//! ```
//!
//! Keeping hardware access behind a narrow interface in [`arch`] is what makes
//! that possible — and it is also what will make the later ports (ARMv8-M for
//! the multi-core stage) cheap.

mod arch;
mod scheduler;
mod task;
