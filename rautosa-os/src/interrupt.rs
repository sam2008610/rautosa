//! Interrupt handling.
//!
//! * OSEK OS §6 "Interrupt processing": ISR category 1 (the OS does not know
//!   about them, they must not call OS services) and category 2 (run under OS
//!   control, may call a subset of services, may cause rescheduling on exit).
//! * OSEK OS §13.3 "Interrupt handling": the services in this module.
//! * SWS OS `[SWS_Os_00299]`: the `*_all_interrupts` services must also work
//!   before `StartOS` and after `ShutdownOS`.
//! * SWS OS `[SWS_Os_00092]`: an enable/resume without a matching
//!   disable/suspend is ignored.
//! * SWS OS `[SWS_Os_00093]`: other OS services called between a
//!   disable/suspend and its enable/resume are ignored and return
//!   `E_OS_DISABLEDINT` (service protection; mandatory from SC3).
//!
//! None of these services returns a status, so none of them raises the error
//! hook (`[SWS_Os_00367]`).
//!
//! # Cortex-M notes (public ARMv7-M architecture)
//!
//! * `PRIMASK` masks everything maskable → natural fit for "all interrupts".
//! * `BASEPRI` (ARMv7-M, not ARMv6-M) masks priorities at or below a
//!   threshold → natural fit for "OS interrupts", provided category 1 ISRs
//!   are configured at more urgent priorities than every category 2 ISR.
//!
//! # Design question for you
//!
//! The specification's shape is "call A, later call B". Rust code would
//! normally express that as a guard (`let _g = suspend_all();`) or a closure
//! (`cortex_m::interrupt::free`, `critical_section::with`). Also decide how
//! this module relates to the `critical-section` crate: today the examples use
//! `cortex-m`'s `critical-section-single-core` implementation; a kernel can
//! provide its own via `critical_section::set_impl!`.

/// Disables all interrupts. Not nestable.
///
/// Must be paired with [`enable_all_interrupts`]; no OS service may be called
/// in between.
///
/// # Specification
///
/// OSEK OS §13.3.2.2.
#[doc(alias = "DisableAllInterrupts")]
pub fn disable_all_interrupts() {
    todo!("DisableAllInterrupts: OSEK OS §13.3.2.2")
}

/// Restores the interrupt state saved by [`disable_all_interrupts`].
///
/// # Specification
///
/// OSEK OS §13.3.2.1.
#[doc(alias = "EnableAllInterrupts")]
pub fn enable_all_interrupts() {
    todo!("EnableAllInterrupts: OSEK OS §13.3.2.1")
}

/// Saves the interrupt state and disables all interrupts. Nestable.
///
/// # Specification
///
/// OSEK OS §13.3.2.4.
#[doc(alias = "SuspendAllInterrupts")]
pub fn suspend_all_interrupts() {
    // Hint: "nestable" means only the outermost suspend saves, and only the
    // matching outermost resume restores. Where does the nesting counter live,
    // and who may touch it concurrently?
    todo!("SuspendAllInterrupts: OSEK OS §13.3.2.4")
}

/// Restores the interrupt state saved by the matching
/// [`suspend_all_interrupts`].
///
/// # Specification
///
/// OSEK OS §13.3.2.3.
#[doc(alias = "ResumeAllInterrupts")]
pub fn resume_all_interrupts() {
    todo!("ResumeAllInterrupts: OSEK OS §13.3.2.3")
}

/// Saves the interrupt state and disables category 2 interrupts. Nestable.
///
/// # Specification
///
/// OSEK OS §13.3.2.6.
#[doc(alias = "SuspendOSInterrupts")]
pub fn suspend_os_interrupts() {
    todo!("SuspendOSInterrupts: OSEK OS §13.3.2.6")
}

/// Restores the interrupt state saved by the matching
/// [`suspend_os_interrupts`].
///
/// # Specification
///
/// OSEK OS §13.3.2.5.
#[doc(alias = "ResumeOSInterrupts")]
pub fn resume_os_interrupts() {
    todo!("ResumeOSInterrupts: OSEK OS §13.3.2.5")
}
