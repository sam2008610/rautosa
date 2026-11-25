//! Resource management (OSEK priority ceiling protocol).
//!
//! * OSEK OS §8 "Resource management": behaviour (§8.1), restrictions (§8.2),
//!   priority inversion and deadlocks (§8.4), the OSEK priority ceiling
//!   protocol (§8.5), its extension to interrupt levels (§8.6), internal
//!   resources (§8.7).
//! * OSEK OS §13.4 "Resource management": the services in this module.
//! * SWS OS §7.1.2.1: unlike OSEK (§8.3), AUTOSAR does not create
//!   `RES_SCHEDULER` automatically; it is configured like any other resource.
//!
//! # Design question for you
//!
//! Resources must be released in LIFO order and before the task terminates or
//! waits. In Rust that shape is usually an RAII guard or a closure
//! (`with_resource(res, || ...)`). The spec-shaped pair below is the base
//! layer; whether to put a safe wrapper on top — and how it interacts with a
//! `terminate_task` that never runs destructors — is discussed in
//! `docs/design-notes.md`.

use crate::Result;

/// Identifies a resource (`ResourceType`, OSEK OS §13.4.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceId(u8);

impl ResourceId {
    /// Creates a resource ID from its raw index.
    pub const fn new(raw: u8) -> Self {
        Self(raw)
    }

    /// The raw index of this resource.
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// Enters the critical section guarded by `resource`.
///
/// # Errors
///
/// * [`Error::Id`](crate::Error::Id) — `resource` is invalid (extended status).
/// * [`Error::Access`](crate::Error::Access) — the resource is already occupied
///   by a task or ISR, or the caller's statically assigned priority is higher
///   than the ceiling priority of `resource` (extended status).
///
/// # Specification
///
/// OSEK OS §13.4.3.1, §8.5. Callable from tasks and category 2 ISRs.
#[doc(alias = "GetResource")]
pub fn get_resource(resource: ResourceId) -> Result {
    // What to do
    // ----------
    // * Raise the caller's *current* priority to the resource's ceiling
    //   priority. The ceiling is known at configuration time: the highest
    //   priority of all tasks that access the resource (§8.5).
    // * Remember enough to restore the previous priority on release, with
    //   nested resources in mind.
    // * Nothing ever blocks here. Work out why a task can never find a
    //   resource occupied if the protocol is implemented correctly.
    // * Resources shared with ISRs also mask interrupts (§8.6) — optional,
    //   leave it for later.
    //
    // Rust notes
    // ----------
    // * Ceiling priorities are a pure function of the configuration: a good
    //   candidate for `const fn` evaluation at compile time.
    todo!("GetResource: OSEK OS §13.4.3.1")
}

/// Leaves the critical section guarded by `resource`.
///
/// # Errors
///
/// * [`Error::Id`](crate::Error::Id) — `resource` is invalid (extended status).
/// * [`Error::NoFunc`](crate::Error::NoFunc) — `resource` is not occupied by
///   the caller, or another resource has to be released first (extended
///   status).
/// * [`Error::Access`](crate::Error::Access) — the caller's statically assigned
///   priority is higher than the ceiling priority (extended status).
///
/// # Specification
///
/// OSEK OS §13.4.3.2, §8.2, §8.5.
#[doc(alias = "ReleaseResource")]
pub fn release_resource(resource: ResourceId) -> Result {
    // Hint: lowering the priority can make a higher-priority *ready* task
    // eligible, so this is a rescheduling point for preemptable tasks (§8.5).
    todo!("ReleaseResource: OSEK OS §13.4.3.2")
}
