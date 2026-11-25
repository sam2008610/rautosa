//! Task management.
//!
//! * OSEK OS §4 "Task management": task concept, state model (§4.2), activation
//!   (§4.3), priorities (§4.5), scheduling policies (§4.6), termination (§4.7).
//! * OSEK OS §3.2 "Conformance classes": BCC1/BCC2/ECC1/ECC2 decide whether
//!   multiple activations and several tasks per priority must be supported.
//! * OSEK OS §13.2 "Task management": the services in this module.
//! * SWS OS `[SWS_Os_00001]`: AUTOSAR OS keeps this API backward compatible.
//! * SWS OS §7.7.3.3, Table 7.1: allowed calling context of every service.

use core::convert::Infallible;

use crate::Result;

/// Identifies a task (`TaskType`, OSEK OS §13.2.1).
///
/// The OS is statically configured: the set of tasks is fixed at build time and
/// an ID is an index into that set. How the configuration is expressed in Rust
/// is an open design decision, see `docs/design-notes.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TaskId(u8);

impl TaskId {
    /// Creates a task ID from its raw index.
    pub const fn new(raw: u8) -> Self {
        Self(raw)
    }

    /// The raw index of this task.
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// State of a task (`TaskStateType`, OSEK OS §13.2.1 and §4.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskState {
    /// The task owns the CPU. At most one task per core is running.
    Running,
    /// Extended tasks only: waiting for at least one event (OSEK OS §4.2.1).
    Waiting,
    /// All prerequisites to run are fulfilled; waiting for the CPU.
    Ready,
    /// The task is passive and can be activated.
    Suspended,
}

/// Transfers `task` from *suspended* to *ready*.
///
/// # Errors
///
/// * [`Error::Limit`](crate::Error::Limit) — too many activations of `task`
///   (standard status).
/// * [`Error::Id`](crate::Error::Id) — `task` is invalid (extended status).
///
/// # Specification
///
/// OSEK OS §13.2.3.1, §4.3. Callable from tasks and category 2 ISRs.
#[doc(alias = "ActivateTask")]
pub fn activate_task(task: TaskId) -> Result {
    // What to do
    // ----------
    // * suspended -> ready. For an extended task, all its events are cleared
    //   on this transition (OSEK OS §13.2.3.1 "Particularities", §7).
    // * A task that is not suspended: BCC2/ECC2 allow *queued* activations of
    //   basic tasks up to a configured limit, otherwise E_OS_LIMIT (§4.3, §3.2).
    //   Queued activations of equal priority run in FIFO order (§4.5).
    // * Rescheduling depends on the caller (§13.2.3.1): a full-preemptive task
    //   may be preempted right here, a non-preemptive task keeps running, and
    //   from an ISR the switch is deferred until the last nested ISR returns
    //   (§6).
    //
    // Rust notes
    // ----------
    // * Kernel state lives in `static`s that both tasks and ISRs touch. Think
    //   about *which* interior-mutability wrapper makes that sound before you
    //   reach for `static mut` (references to a `static mut` are denied by the
    //   `static_mut_refs` lint in edition 2024 — for good reasons).
    // * If this call ends in a context switch, the function "returns" much
    //   later, on this task's stack. No `&mut` borrow of kernel state and no
    //   critical-section token may be alive across that point.
    todo!("ActivateTask: OSEK OS §13.2.3.1")
}

/// Terminates the calling task (*running* → *suspended*).
///
/// On success this function does not return, which is why the success type is
/// [`Infallible`].
///
/// # Errors
///
/// * [`Error::Resource`](crate::Error::Resource) — the task still occupies a
///   resource (extended status).
/// * [`Error::CallLevel`](crate::Error::CallLevel) — called at interrupt level
///   (extended status).
///
/// # Specification
///
/// OSEK OS §13.2.3.2, §4.7. See also SWS OS `[SWS_Os_00052]`,
/// `[SWS_Os_00069]`, `[SWS_Os_00070]`, `[SWS_Os_00239]` for a task that returns
/// from its entry function *without* calling this service (service protection,
/// mandatory from SC3, optional below).
#[doc(alias = "TerminateTask")]
pub fn terminate_task() -> Result<Infallible> {
    // What to do
    // ----------
    // * running -> suspended, release an internal resource (§8.7), then let the
    //   scheduler pick the next task. A queued activation (BCC2) makes the task
    //   ready again immediately.
    // * The task's stack is abandoned, not unwound.
    //
    // Rust notes
    // ----------
    // * "Does not return and abandons the stack" means destructors of the
    //   caller's live locals never run. Leaking is *safe* in Rust, but decide
    //   what that means for RAII guards your own API hands out (a resource
    //   guard? an interrupt guard?).
    // * How does the task body reach this call ergonomically? Compare
    //   `terminate_task()?` at the end of an entry function returning
    //   `Result<Infallible>`, an entry function of type `fn() -> !`, and an
    //   entry function that simply returns and lets the kernel terminate it.
    todo!("TerminateTask: OSEK OS §13.2.3.2")
}

/// Terminates the calling task and activates `task` in one step.
///
/// On success this function does not return. `task` may be the calling task
/// itself.
///
/// # Errors
///
/// * [`Error::Limit`](crate::Error::Limit) — too many activations of `task`
///   (standard status). The calling task is *not* terminated in this case.
/// * [`Error::Id`](crate::Error::Id), [`Error::Resource`](crate::Error::Resource),
///   [`Error::CallLevel`](crate::Error::CallLevel) — extended status.
///
/// # Specification
///
/// OSEK OS §13.2.3.3.
#[doc(alias = "ChainTask")]
pub fn chain_task(task: TaskId) -> Result<Infallible> {
    // Hint: this is not `activate_task` followed by `terminate_task`. Read the
    // "Particularities" of §13.2.3.3 and work out what differs when a task
    // chains to itself, and when the activation limit of `task` is reached.
    todo!("ChainTask: OSEK OS §13.2.3.3")
}

/// Voluntarily yields the CPU to a higher-priority ready task, if any.
///
/// Only has an effect for tasks that hold an internal resource, which includes
/// non-preemptable tasks.
///
/// # Errors
///
/// [`Error::CallLevel`](crate::Error::CallLevel) and
/// [`Error::Resource`](crate::Error::Resource) in extended status.
///
/// # Specification
///
/// OSEK OS §13.2.3.4, §4.6.2–§4.6.4, §8.7.
#[doc(alias = "Schedule")]
pub fn schedule() -> Result {
    todo!("Schedule: OSEK OS §13.2.3.4")
}

/// Returns the task that is currently *running*, or `None` if no task is
/// running (the specification returns `INVALID_TASK` in that case).
///
/// # Specification
///
/// OSEK OS §13.2.3.5. Callable from tasks, category 2 ISRs and several hooks.
#[doc(alias = "GetTaskID")]
pub fn get_task_id() -> Option<TaskId> {
    // Hint: from an ISR this reports the *interrupted* task, not "the ISR".
    todo!("GetTaskID: OSEK OS §13.2.3.5")
}

/// Returns the state of `task` at the time of the call.
///
/// # Errors
///
/// [`Error::Id`](crate::Error::Id) — `task` is invalid (extended status).
///
/// # Specification
///
/// OSEK OS §13.2.3.6.
#[doc(alias = "GetTaskState")]
pub fn get_task_state(task: TaskId) -> Result<TaskState> {
    todo!("GetTaskState: OSEK OS §13.2.3.6")
}
