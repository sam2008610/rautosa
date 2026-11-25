//! Specification-named API.
//!
//! The services of OSEK OS §13 and the AUTOSAR counter services under their
//! original names and signatures. Every function is a thin wrapper around the
//! idiomatic API and contains no kernel logic:
//!
//! ```ignore
//! use rautosa_os::compat::*;
//!
//! let status: StatusType = ActivateTask(TASK_A);
//! if status != E_OK { /* ... */ }
//! ```
//!
//! Differences from the C API that remain:
//!
//! * Out-parameters (`TaskRefType`, `TickRefType`, ...) are `&mut T` instead
//!   of raw pointers, so `E_OS_ILLEGAL_ADDRESS` cannot occur
//!   (`[SWS_Os_00566]`). A true C ABI (`extern "C"`, raw pointers) would be a
//!   third layer on top of this one.
//! * `StartOS` and `ShutdownOS` return `!` instead of `void`.
//! * The `Declare*` macros of OSEK OS §13.x.2 are not provided; AUTOSAR makes
//!   them no-ops anyway (SWS OS §7.1.2.1).

#![allow(non_snake_case)]

use crate::{alarm, counter, event, interrupt, os, resource, task};

/// OSEK OS §13.1.
pub type StatusType = u8;

/// OSEK OS §13.1: no error.
pub const E_OK: StatusType = 0;
/// OSEK OS §13.1.
pub const E_OS_ACCESS: StatusType = crate::Error::Access.status();
/// OSEK OS §13.1.
pub const E_OS_CALLEVEL: StatusType = crate::Error::CallLevel.status();
/// OSEK OS §13.1.
pub const E_OS_ID: StatusType = crate::Error::Id.status();
/// OSEK OS §13.1.
pub const E_OS_LIMIT: StatusType = crate::Error::Limit.status();
/// OSEK OS §13.1.
pub const E_OS_NOFUNC: StatusType = crate::Error::NoFunc.status();
/// OSEK OS §13.1.
pub const E_OS_RESOURCE: StatusType = crate::Error::Resource.status();
/// OSEK OS §13.1.
pub const E_OS_STATE: StatusType = crate::Error::State.status();
/// OSEK OS §13.1.
pub const E_OS_VALUE: StatusType = crate::Error::Value.status();
/// SWS OS `[SWS_Os_91025]`.
pub const E_OS_SERVICEID: StatusType = crate::Error::ServiceId.status();
/// SWS OS `[SWS_Os_91025]`.
pub const E_OS_ILLEGAL_ADDRESS: StatusType = crate::Error::IllegalAddress.status();
/// SWS OS `[SWS_Os_91025]`.
pub const E_OS_MISSINGEND: StatusType = crate::Error::MissingEnd.status();
/// SWS OS `[SWS_Os_91025]`.
pub const E_OS_DISABLEDINT: StatusType = crate::Error::DisabledInt.status();
/// SWS OS `[SWS_Os_91025]`.
pub const E_OS_STACKFAULT: StatusType = crate::Error::StackFault.status();

/// OSEK OS §13.2.1.
pub type TaskType = task::TaskId;
/// OSEK OS §13.2.1.
pub type TaskRefType<'a> = &'a mut TaskType;
/// OSEK OS §13.2.1.
pub type TaskStateType = task::TaskState;
/// OSEK OS §13.2.1.
pub type TaskStateRefType<'a> = &'a mut TaskStateType;
/// OSEK OS §13.4.1.
pub type ResourceType = resource::ResourceId;
/// OSEK OS §13.5.1.
pub type EventMaskType = event::EventMask;
/// OSEK OS §13.5.1.
pub type EventMaskRefType<'a> = &'a mut EventMaskType;
/// OSEK OS §13.6.1.
pub type TickType = counter::Tick;
/// OSEK OS §13.6.1.
pub type TickRefType<'a> = &'a mut TickType;
/// OSEK OS §13.6.1. The fields are spelled `max_allowed_value`,
/// `ticks_per_base` and `min_cycle`.
pub type AlarmBaseType = alarm::AlarmBase;
/// OSEK OS §13.6.1.
pub type AlarmBaseRefType<'a> = &'a mut AlarmBaseType;
/// OSEK OS §13.6.1.
pub type AlarmType = alarm::AlarmId;
/// SWS OS `[SWS_Os_00786]`.
pub type CounterType = counter::CounterId;
/// OSEK OS §13.7.1.
pub type AppModeType = os::AppMode;
/// OSEK OS §13.8.1.
pub type OSServiceIdType = crate::hooks::ServiceId;

/// OSEK OS §13.2.4.
pub const RUNNING: TaskStateType = task::TaskState::Running;
/// OSEK OS §13.2.4.
pub const WAITING: TaskStateType = task::TaskState::Waiting;
/// OSEK OS §13.2.4.
pub const READY: TaskStateType = task::TaskState::Ready;
/// OSEK OS §13.2.4.
pub const SUSPENDED: TaskStateType = task::TaskState::Suspended;
/// OSEK OS §13.2.4: "no task". The configuration must never assign this ID.
pub const INVALID_TASK: TaskType = task::TaskId::new(u8::MAX);
/// OSEK OS §13.7.3.
pub const OSDEFAULTAPPMODE: AppModeType = os::AppMode::DEFAULT;

fn status_of(result: crate::Result) -> StatusType {
    match result {
        Ok(()) => E_OK,
        Err(error) => error.status(),
    }
}

/// See [`task::activate_task`]. OSEK OS §13.2.3.1.
pub fn ActivateTask(TaskID: TaskType) -> StatusType {
    status_of(task::activate_task(TaskID))
}

/// See [`task::terminate_task`]. OSEK OS §13.2.3.2.
pub fn TerminateTask() -> StatusType {
    match task::terminate_task() {
        Err(error) => error.status(),
    }
}

/// See [`task::chain_task`]. OSEK OS §13.2.3.3.
pub fn ChainTask(TaskID: TaskType) -> StatusType {
    match task::chain_task(TaskID) {
        Err(error) => error.status(),
    }
}

/// See [`task::schedule`]. OSEK OS §13.2.3.4.
pub fn Schedule() -> StatusType {
    status_of(task::schedule())
}

/// See [`task::get_task_id`]. OSEK OS §13.2.3.5.
pub fn GetTaskID(TaskID: TaskRefType) -> StatusType {
    *TaskID = task::get_task_id().unwrap_or(INVALID_TASK);
    E_OK
}

/// See [`task::get_task_state`]. OSEK OS §13.2.3.6.
pub fn GetTaskState(TaskID: TaskType, State: TaskStateRefType) -> StatusType {
    status_of(task::get_task_state(TaskID).map(|state| *State = state))
}

/// See [`interrupt::enable_all_interrupts`]. OSEK OS §13.3.2.1.
pub fn EnableAllInterrupts() {
    interrupt::enable_all_interrupts();
}

/// See [`interrupt::disable_all_interrupts`]. OSEK OS §13.3.2.2.
pub fn DisableAllInterrupts() {
    interrupt::disable_all_interrupts();
}

/// See [`interrupt::resume_all_interrupts`]. OSEK OS §13.3.2.3.
pub fn ResumeAllInterrupts() {
    interrupt::resume_all_interrupts();
}

/// See [`interrupt::suspend_all_interrupts`]. OSEK OS §13.3.2.4.
pub fn SuspendAllInterrupts() {
    interrupt::suspend_all_interrupts();
}

/// See [`interrupt::resume_os_interrupts`]. OSEK OS §13.3.2.5.
pub fn ResumeOSInterrupts() {
    interrupt::resume_os_interrupts();
}

/// See [`interrupt::suspend_os_interrupts`]. OSEK OS §13.3.2.6.
pub fn SuspendOSInterrupts() {
    interrupt::suspend_os_interrupts();
}

/// See [`resource::get_resource`]. OSEK OS §13.4.3.1.
pub fn GetResource(ResID: ResourceType) -> StatusType {
    status_of(resource::get_resource(ResID))
}

/// See [`resource::release_resource`]. OSEK OS §13.4.3.2.
pub fn ReleaseResource(ResID: ResourceType) -> StatusType {
    status_of(resource::release_resource(ResID))
}

/// See [`event::set_event`]. OSEK OS §13.5.3.1.
pub fn SetEvent(TaskID: TaskType, Mask: EventMaskType) -> StatusType {
    status_of(event::set_event(TaskID, Mask))
}

/// See [`event::clear_event`]. OSEK OS §13.5.3.2.
pub fn ClearEvent(Mask: EventMaskType) -> StatusType {
    status_of(event::clear_event(Mask))
}

/// See [`event::get_event`]. OSEK OS §13.5.3.3.
pub fn GetEvent(TaskID: TaskType, Event: EventMaskRefType) -> StatusType {
    status_of(event::get_event(TaskID).map(|mask| *Event = mask))
}

/// See [`event::wait_event`]. OSEK OS §13.5.3.4.
pub fn WaitEvent(Mask: EventMaskType) -> StatusType {
    status_of(event::wait_event(Mask))
}

/// See [`alarm::get_alarm_base`]. OSEK OS §13.6.3.1.
pub fn GetAlarmBase(AlarmID: AlarmType, Info: AlarmBaseRefType) -> StatusType {
    status_of(alarm::get_alarm_base(AlarmID).map(|base| *Info = base))
}

/// See [`alarm::get_alarm`]. OSEK OS §13.6.3.2.
pub fn GetAlarm(AlarmID: AlarmType, Tick: TickRefType) -> StatusType {
    status_of(alarm::get_alarm(AlarmID).map(|ticks| *Tick = ticks))
}

/// See [`alarm::set_rel_alarm`]. OSEK OS §13.6.3.3.
pub fn SetRelAlarm(AlarmID: AlarmType, increment: TickType, cycle: TickType) -> StatusType {
    status_of(alarm::set_rel_alarm(AlarmID, increment, cycle))
}

/// See [`alarm::set_abs_alarm`]. OSEK OS §13.6.3.4.
pub fn SetAbsAlarm(AlarmID: AlarmType, start: TickType, cycle: TickType) -> StatusType {
    status_of(alarm::set_abs_alarm(AlarmID, start, cycle))
}

/// See [`alarm::cancel_alarm`]. OSEK OS §13.6.3.5.
pub fn CancelAlarm(AlarmID: AlarmType) -> StatusType {
    status_of(alarm::cancel_alarm(AlarmID))
}

/// See [`counter::increment_counter`]. SWS OS `[SWS_Os_00399]`.
pub fn IncrementCounter(CounterID: CounterType) -> StatusType {
    status_of(counter::increment_counter(CounterID))
}

/// See [`counter::get_counter_value`]. SWS OS `[SWS_Os_00383]`.
pub fn GetCounterValue(CounterID: CounterType, Value: TickRefType) -> StatusType {
    status_of(counter::get_counter_value(CounterID).map(|value| *Value = value))
}

/// See [`counter::get_elapsed_value`]. SWS OS `[SWS_Os_00392]`.
pub fn GetElapsedValue(
    CounterID: CounterType,
    Value: TickRefType,
    ElapsedValue: TickRefType,
) -> StatusType {
    status_of(counter::get_elapsed_value(CounterID, Value).map(|elapsed| *ElapsedValue = elapsed))
}

/// See [`os::get_active_application_mode`]. OSEK OS §13.7.2.1.
pub fn GetActiveApplicationMode() -> AppModeType {
    os::get_active_application_mode()
}

/// See [`os::start_os`]. OSEK OS §13.7.2.2.
pub fn StartOS(Mode: AppModeType) -> ! {
    os::start_os(Mode)
}

/// See [`os::shutdown_os`]. OSEK OS §13.7.2.3.
pub fn ShutdownOS(Error: StatusType) -> ! {
    // TODO(design): OSEK lets the application pass its own, non-OS status
    // codes to ShutdownOS. `os::shutdown_os` takes `Result<(), Error>` and
    // cannot represent them, so unknown codes collapse to `E_OS_VALUE` here.
    // Decide whether the idiomatic signature should change instead.
    let status = match Error {
        E_OK => Ok(()),
        code => Err(crate::Error::from_status(code).unwrap_or(crate::Error::Value)),
    };
    os::shutdown_os(status)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_constants_match_osek_13_1() {
        assert_eq!(
            [
                E_OK,
                E_OS_ACCESS,
                E_OS_CALLEVEL,
                E_OS_ID,
                E_OS_LIMIT,
                E_OS_NOFUNC,
                E_OS_RESOURCE,
                E_OS_STATE,
                E_OS_VALUE
            ],
            [0, 1, 2, 3, 4, 5, 6, 7, 8]
        );
    }

    #[test]
    fn status_of_maps_ok_to_e_ok() {
        assert_eq!(status_of(Ok(())), E_OK);
        assert_eq!(status_of(Err(crate::Error::Limit)), E_OS_LIMIT);
    }
}
