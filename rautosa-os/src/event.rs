//! Event control (extended tasks only).
//!
//! * OSEK OS §7 "Event mechanism".
//! * OSEK OS §4.2.1: the *waiting* state of extended tasks.
//! * OSEK OS §13.5 "Event control": the services in this module.

use core::ops::BitOr;

use crate::Result;
use crate::task::TaskId;

/// A set of events (`EventMaskType`, OSEK OS §13.5.1).
///
/// Events are owned by an extended task; each bit is one event of that task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct EventMask(u32);

impl EventMask {
    /// The empty event set.
    pub const EMPTY: Self = Self(0);

    /// Creates a mask from raw bits.
    pub const fn new(bits: u32) -> Self {
        Self(bits)
    }

    /// The raw bits of this mask.
    pub const fn bits(self) -> u32 {
        self.0
    }
}

impl BitOr for EventMask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

// TODO: the kernel will also want intersection / difference / emptiness tests.
// Add the operator traits or named methods you actually need.

/// Sets the events `mask` of the extended task `task`.
///
/// If `task` is waiting for at least one of them it becomes *ready*.
///
/// # Errors
///
/// * [`Error::Id`](crate::Error::Id) — `task` is invalid.
/// * [`Error::Access`](crate::Error::Access) — `task` is not an extended task.
/// * [`Error::State`](crate::Error::State) — `task` is *suspended*.
///
/// All of them are extended status.
///
/// # Specification
///
/// OSEK OS §13.5.3.1. Callable from tasks and category 2 ISRs.
#[doc(alias = "SetEvent")]
pub fn set_event(task: TaskId, mask: EventMask) -> Result {
    // Hint: any task or ISR may *set* events of another task, but only the
    // owner may clear or wait for them. The waiting -> ready transition is a
    // rescheduling point, with the same caller-dependent rules as
    // `activate_task`.
    todo!("SetEvent: OSEK OS §13.5.3.1")
}

/// Clears the events `mask` of the calling extended task.
///
/// # Errors
///
/// [`Error::Access`](crate::Error::Access) — caller is not an extended task;
/// [`Error::CallLevel`](crate::Error::CallLevel) — called at interrupt level.
/// Both are extended status.
///
/// # Specification
///
/// OSEK OS §13.5.3.2.
#[doc(alias = "ClearEvent")]
pub fn clear_event(mask: EventMask) -> Result {
    todo!("ClearEvent: OSEK OS §13.5.3.2")
}

/// Returns the events currently set for the extended task `task`.
///
/// # Errors
///
/// [`Error::Id`](crate::Error::Id), [`Error::Access`](crate::Error::Access)
/// (not an extended task), [`Error::State`](crate::Error::State) (*suspended*).
/// All extended status.
///
/// # Specification
///
/// OSEK OS §13.5.3.3.
#[doc(alias = "GetEvent")]
pub fn get_event(task: TaskId) -> Result<EventMask> {
    todo!("GetEvent: OSEK OS §13.5.3.3")
}

/// Puts the calling extended task into the *waiting* state until at least one
/// of the events in `mask` is set.
///
/// Returns immediately if one of them is already set.
///
/// # Errors
///
/// [`Error::Access`](crate::Error::Access) — caller is not an extended task;
/// [`Error::Resource`](crate::Error::Resource) — caller occupies a resource;
/// [`Error::CallLevel`](crate::Error::CallLevel) — called at interrupt level.
/// All extended status.
///
/// # Specification
///
/// OSEK OS §13.5.3.4, §7.
#[doc(alias = "WaitEvent")]
pub fn wait_event(mask: EventMask) -> Result {
    // What to do
    // ----------
    // * running -> waiting, unless an awaited event is already set. An internal
    //   resource is released while waiting (§8.7).
    // * This is the one service that needs a *per-task stack*: the task is
    //   switched out in the middle of a function and must resume right here.
    //   Basic tasks never do that, which is why OSEK lets them share a stack.
    //
    // Rust notes
    // ----------
    // * From the compiler's point of view this is an ordinary function that
    //   returns `Result`. The suspension happens "inside" it, in your context
    //   switch. Anything the caller holds across the call (borrows, guards)
    //   stays alive while other tasks run — make sure your API cannot hand out
    //   something that must not survive that.
    todo!("WaitEvent: OSEK OS §13.5.3.4")
}
