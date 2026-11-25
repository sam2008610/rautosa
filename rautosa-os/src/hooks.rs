//! Hook routines.
//!
//! * OSEK OS §11.1 "Hook routines", §11.2 "Error handling".
//! * OSEK OS §13.8 "Hook routines": `ErrorHook`, `PreTaskHook`,
//!   `PostTaskHook`, `StartupHook`, `ShutdownHook`, `OSServiceIdType`,
//!   `OSErrorGetServiceId`.
//! * SWS OS §7.12 "Hook Functions"; `[SWS_Os_00439]` (error macros),
//!   `[SWS_Os_00367]` (services without `StatusType` do not raise the error
//!   hook).
//!
//! Hooks are implemented by the *application* and called by the OS. Hooks run
//! with category 2 interrupts disabled and may only call a subset of services
//! (OSEK OS §12, Figure 12-1; SWS OS Table 7.1).
//!
//! # Design question for you
//!
//! In C the hooks are functions with fixed names that the linker resolves. In
//! Rust the options include a trait implemented by the application (below), a
//! table of `fn` pointers in the static configuration, or `extern "Rust"`
//! declarations. [`Hooks`] is a starting point, not a decision; how an
//! implementation gets registered is part of the configuration design in
//! `docs/design-notes.md`.

use crate::Result;

/// Identifies an OS service (`OSServiceIdType`, OSEK OS §13.8.1).
///
/// Used by the error hook to find out which service failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[allow(missing_docs)] // The variants are named after the services.
pub enum ServiceId {
    ActivateTask,
    TerminateTask,
    ChainTask,
    Schedule,
    GetTaskId,
    GetTaskState,
    GetResource,
    ReleaseResource,
    SetEvent,
    ClearEvent,
    GetEvent,
    WaitEvent,
    GetAlarmBase,
    GetAlarm,
    SetRelAlarm,
    SetAbsAlarm,
    CancelAlarm,
    IncrementCounter,
    GetCounterValue,
    GetElapsedValue,
    StartOs,
    ShutdownOs,
}

/// Application-provided hook routines. Every hook defaults to doing nothing.
pub trait Hooks {
    /// `StartupHook` — called at the end of OS initialisation, before the
    /// scheduler runs (OSEK OS §13.8.2.4, §11.3).
    fn startup(&self) {}

    /// `ShutdownHook` — called by `ShutdownOS` (OSEK OS §13.8.2.5, §11.4).
    fn shutdown(&self, status: Result) {}

    /// `PreTaskHook` — called after a task enters *running*, in its context
    /// (OSEK OS §13.8.2.2).
    fn pre_task(&self) {}

    /// `PostTaskHook` — called before a task leaves *running*, still in its
    /// context (OSEK OS §13.8.2.3). Not called during shutdown
    /// (`[SWS_Os_00071]`).
    fn post_task(&self) {}

    /// `ErrorHook` — called when a service returns an error
    /// (OSEK OS §13.8.2.1, §11.2). Never called recursively.
    fn error(&self, error: crate::Error) {}
}

/// Returns the service that raised the error currently being handled.
///
/// Only meaningful inside [`Hooks::error`]; `None` elsewhere.
///
/// # Specification
///
/// OSEK OS §13.8.4 (`OSErrorGetServiceId`); SWS OS `[SWS_Os_00439]`.
#[doc(alias = "OSErrorGetServiceId")]
pub fn error_service_id() -> Option<ServiceId> {
    // Design question: the specification also defines parameter-access macros
    // (`OSError_x1_x2`, OSEK OS §13.8.4). A Rust alternative to "global state
    // readable only inside the hook" is to pass a context value *into* the
    // hook, e.g. `fn error(&self, error: Error, cx: &ErrorContext)`.
    todo!("OSErrorGetServiceId: OSEK OS §13.8.4")
}
