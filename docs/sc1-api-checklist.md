# SC1 API checklist

Everything stage 1 has to implement, with its source. Tick a box when the item
is implemented in **both** layers (`rautosa_os::*` and `rautosa_os::compat`)
and has tests.

- **OSEK OS** = OSEK/VDX Operating System Specification 2.2.3.
- **SWS OS** = AUTOSAR CP Specification of Operating System, R25-11.
- **CC** = OSEK conformance classes that must provide the item (OSEK OS §3.2).
- Error codes marked *(ext)* are only checked in extended status
  (OSEK OS §12.2.3).

AUTOSAR requires the OSEK API as-is (`[SWS_Os_00001]`), so for most services
the OSEK section *is* the specification; SWS OS only adds the deltas listed
under "AUTOSAR additions".

## Task management — OSEK OS §13.2

| Done | Rust | Specification name | Source | CC | Errors |
|:----:|------|--------------------|--------|----|--------|
| [ ] | `task::activate_task` | `ActivateTask` | §13.2.3.1 | all | `E_OS_LIMIT`, `E_OS_ID` *(ext)* |
| [ ] | `task::terminate_task` | `TerminateTask` | §13.2.3.2 | all | `E_OS_RESOURCE` *(ext)*, `E_OS_CALLEVEL` *(ext)* |
| [ ] | `task::chain_task` | `ChainTask` | §13.2.3.3 | all | `E_OS_LIMIT`, `E_OS_ID` *(ext)*, `E_OS_RESOURCE` *(ext)*, `E_OS_CALLEVEL` *(ext)* |
| [ ] | `task::schedule` | `Schedule` | §13.2.3.4 | all | `E_OS_CALLEVEL` *(ext)*, `E_OS_RESOURCE` *(ext)* |
| [ ] | `task::get_task_id` | `GetTaskID` | §13.2.3.5 | all | — |
| [ ] | `task::get_task_state` | `GetTaskState` | §13.2.3.6 | all | `E_OS_ID` *(ext)* |

Behaviour behind the services:

- [ ] Task state model: basic (3 states) and extended (4 states) — §4.2
- [ ] Full preemptive scheduling — §4.6.1
- [ ] Non preemptive scheduling and its rescheduling points — §4.6.2
- [ ] Mixed preemptive scheduling, groups of tasks — §4.6.3, §4.6.4
- [ ] Priority queue semantics (FIFO per priority, preempted task first) — §4.5
- [ ] Multiple activation requests of basic tasks (BCC2, ECC2) — §4.3
- [ ] More than one task per priority (BCC2, ECC2) — §3.2

## Interrupt handling — OSEK OS §13.3

| Done | Rust | Specification name | Source | CC |
|:----:|------|--------------------|--------|----|
| [ ] | `interrupt::enable_all_interrupts` | `EnableAllInterrupts` | §13.3.2.1 | all |
| [ ] | `interrupt::disable_all_interrupts` | `DisableAllInterrupts` | §13.3.2.2 | all |
| [ ] | `interrupt::resume_all_interrupts` | `ResumeAllInterrupts` | §13.3.2.3 | all |
| [ ] | `interrupt::suspend_all_interrupts` | `SuspendAllInterrupts` | §13.3.2.4 | all |
| [ ] | `interrupt::resume_os_interrupts` | `ResumeOSInterrupts` | §13.3.2.5 | all |
| [ ] | `interrupt::suspend_os_interrupts` | `SuspendOSInterrupts` | §13.3.2.6 | all |

- [ ] Category 1 ISRs (no OS services, no rescheduling) — §6
- [ ] Category 2 ISRs (OS services allowed, rescheduling on exit) — §6
- [ ] A way to declare an ISR (`ISR(name)` in C) — §13.3.3
- [ ] Nesting of the suspend/resume pairs — §13.3.2.3–§13.3.2.6

## Resource management — OSEK OS §13.4

| Done | Rust | Specification name | Source | CC | Errors |
|:----:|------|--------------------|--------|----|--------|
| [ ] | `resource::get_resource` | `GetResource` | §13.4.3.1 | all | `E_OS_ID` *(ext)*, `E_OS_ACCESS` *(ext)* |
| [ ] | `resource::release_resource` | `ReleaseResource` | §13.4.3.2 | all | `E_OS_ID` *(ext)*, `E_OS_NOFUNC` *(ext)*, `E_OS_ACCESS` *(ext)* |

- [ ] OSEK priority ceiling protocol — §8.5
- [ ] Ceiling priority derived from the configuration — §8.5
- [ ] Nested resources, LIFO release — §8.2
- [ ] Internal resources — §8.7
- [ ] `RES_SCHEDULER` as an ordinary configured resource — §8.3, SWS OS §7.1.2.1
- [ ] *(optional)* Resources shared with ISRs — §8.6

## Event control — OSEK OS §13.5

| Done | Rust | Specification name | Source | CC | Errors |
|:----:|------|--------------------|--------|----|--------|
| [ ] | `event::set_event` | `SetEvent` | §13.5.3.1 | ECC1, ECC2 | `E_OS_ID` *(ext)*, `E_OS_ACCESS` *(ext)*, `E_OS_STATE` *(ext)* |
| [ ] | `event::clear_event` | `ClearEvent` | §13.5.3.2 | ECC1, ECC2 | `E_OS_ACCESS` *(ext)*, `E_OS_CALLEVEL` *(ext)* |
| [ ] | `event::get_event` | `GetEvent` | §13.5.3.3 | ECC1, ECC2 | `E_OS_ID` *(ext)*, `E_OS_ACCESS` *(ext)*, `E_OS_STATE` *(ext)* |
| [ ] | `event::wait_event` | `WaitEvent` | §13.5.3.4 | ECC1, ECC2 | `E_OS_ACCESS` *(ext)*, `E_OS_RESOURCE` *(ext)*, `E_OS_CALLEVEL` *(ext)* |

- [ ] Events are cleared when an extended task is activated — §13.2.3.1, §7
- [ ] `WaitEvent` returns immediately if an awaited event is already set — §13.5.3.4

## Counters — SWS OS

| Done | Rust | Specification name | Source | Errors |
|:----:|------|--------------------|--------|--------|
| [ ] | `counter::increment_counter` | `IncrementCounter` | `[SWS_Os_00399]`, `[SWS_Os_00286]`, `[SWS_Os_00321]`, `[SWS_Os_00529]` | `E_OS_ID` *(ext)* `[SWS_Os_00285]` |
| [ ] | `counter::get_counter_value` | `GetCounterValue` | `[SWS_Os_00383]`, `[SWS_Os_00377]`, `[SWS_Os_00531]` | `E_OS_ID` *(ext)* `[SWS_Os_00376]` |
| [ ] | `counter::get_elapsed_value` | `GetElapsedValue` | `[SWS_Os_00392]`, `[SWS_Os_00382]`, `[SWS_Os_00460]`, `[SWS_Os_00533]` | `E_OS_ID` *(ext)* `[SWS_Os_00381]`, `E_OS_VALUE` *(ext)* `[SWS_Os_00391]` |

- [ ] Counter attributes `maxallowedvalue`, `ticksperbase`, `mincycle` — OSEK OS §9.1, §13.6.1
- [ ] At least one counter driven by a hardware timer (system counter) — OSEK OS §9.1
- [ ] Hardware counter values start at zero and increase until the modulus — `[SWS_Os_00384]`
- [ ] The OS owns initialisation of the timers it uses — `[SWS_Os_00374]`

## Alarms — OSEK OS §13.6

| Done | Rust | Specification name | Source | CC | Errors |
|:----:|------|--------------------|--------|----|--------|
| [ ] | `alarm::get_alarm_base` | `GetAlarmBase` | §13.6.3.1 | all | `E_OS_ID` *(ext)* |
| [ ] | `alarm::get_alarm` | `GetAlarm` | §13.6.3.2 | all | `E_OS_NOFUNC`, `E_OS_ID` *(ext)* |
| [ ] | `alarm::set_rel_alarm` | `SetRelAlarm` | §13.6.3.3 | all | `E_OS_STATE`, `E_OS_ID` *(ext)*, `E_OS_VALUE` *(ext)* |
| [ ] | `alarm::set_abs_alarm` | `SetAbsAlarm` | §13.6.3.4 | all | `E_OS_STATE`, `E_OS_ID` *(ext)*, `E_OS_VALUE` *(ext)* |
| [ ] | `alarm::cancel_alarm` | `CancelAlarm` | §13.6.3.5 | all | `E_OS_NOFUNC`, `E_OS_ID` *(ext)* |

- [ ] Alarm action: activate a task — §9.2
- [ ] Alarm action: set an event (ECC1, ECC2) — §9.2
- [ ] Alarm action: alarm-callback routine — §9.3; SC1 only `[SWS_Os_00242]`
- [ ] Single and cyclic alarms — §9.2
- [ ] Autostart of alarms depending on the application mode — §5.2, §11.3

## Execution control — OSEK OS §13.7

| Done | Rust | Specification name | Source | CC |
|:----:|------|--------------------|--------|----|
| [ ] | `os::get_active_application_mode` | `GetActiveApplicationMode` | §13.7.2.1 | all |
| [ ] | `os::start_os` | `StartOS` | §13.7.2.2 | all |
| [ ] | `os::shutdown_os` | `ShutdownOS` | §13.7.2.3 | all |

- [ ] Start-up sequence — §11.3
- [ ] Shutdown sequence — §11.4
- [ ] Autostart of tasks depending on the application mode — §5.2, §11.3
- [ ] `OSDEFAULTAPPMODE` — §13.7.3

## Hooks and error handling — OSEK OS §13.8

| Done | Rust | Specification name | Source |
|:----:|------|--------------------|--------|
| [ ] | `hooks::Hooks::error` | `ErrorHook` | §13.8.2.1, §11.2 |
| [ ] | `hooks::Hooks::pre_task` | `PreTaskHook` | §13.8.2.2 |
| [ ] | `hooks::Hooks::post_task` | `PostTaskHook` | §13.8.2.3 |
| [ ] | `hooks::Hooks::startup` | `StartupHook` | §13.8.2.4 |
| [ ] | `hooks::Hooks::shutdown` | `ShutdownHook` | §13.8.2.5 |
| [ ] | `hooks::error_service_id` | `OSErrorGetServiceId` | §13.8.4, `[SWS_Os_00439]` |

- [ ] Standard and extended status as a build-time choice — §12.2.3
- [ ] Error hook is not called recursively — §11.2
- [ ] Calling-context restrictions of all services — §12, Figure 12-1; SWS OS Table 7.1
- [ ] *(optional)* Parameter access in the error hook (`OSError_x1_x2`) — §13.8.4

## Data types and constants

| Done | Rust | Specification name | Source |
|:----:|------|--------------------|--------|
| [x] | `Error`, `Result` | `StatusType`, `E_OK`, `E_OS_*` | OSEK OS §13.1; `[SWS_Os_91025]` |
| [x] | `task::TaskId` | `TaskType`, `TaskRefType` | OSEK OS §13.2.1 |
| [x] | `task::TaskState` | `TaskStateType`, `RUNNING`, `WAITING`, `READY`, `SUSPENDED` | OSEK OS §13.2.1, §13.2.4 |
| [x] | `Option<TaskId>` / `compat::INVALID_TASK` | `INVALID_TASK` | OSEK OS §13.2.4 |
| [x] | `resource::ResourceId` | `ResourceType` | OSEK OS §13.4.1 |
| [x] | `event::EventMask` | `EventMaskType`, `EventMaskRefType` | OSEK OS §13.5.1 |
| [x] | `counter::Tick` | `TickType`, `TickRefType` | OSEK OS §13.6.1 |
| [x] | `alarm::AlarmBase` | `AlarmBaseType`, `AlarmBaseRefType` | OSEK OS §13.6.1 |
| [x] | `alarm::AlarmId` | `AlarmType` | OSEK OS §13.6.1 |
| [x] | `counter::CounterId` | `CounterType` | `[SWS_Os_00786]` |
| [x] | `os::AppMode`, `AppMode::DEFAULT` | `AppModeType`, `OSDEFAULTAPPMODE` | OSEK OS §13.7.1, §13.7.3 |
| [x] | `hooks::ServiceId` | `OSServiceIdType`, `OSServiceId_xx` | OSEK OS §13.8.1, §13.8.3 |
| [ ] | per-counter constants | `OSMAXALLOWEDVALUE_x`, `OSTICKSPERBASE_x`, `OSMINCYCLE_x`, `OSTICKDURATION` | OSEK OS §13.6.4 |
| [ ] | object declaration | `DeclareTask`, `DeclareResource`, `DeclareEvent`, `DeclareAlarm`, `TASK()`, `ALARMCALLBACK()` | OSEK OS §13.2.2, §13.2.5, §13.4.2, §13.5.2, §13.6.2, §13.6.5 |

The types marked done are only *defined*; their representation may still
change with the configuration design.

## AUTOSAR additions to the OSEK behaviour — SWS OS §7.1.2

- [ ] `[SWS_Os_00001]` API backward compatible with OSEK OS
- [ ] `[SWS_Os_00242]` Alarm callbacks only in SC1
- [ ] `[SWS_Os_00304]` `SetRelAlarm` with `increment == 0` returns `E_OS_VALUE` in standard and extended status
- [ ] `[SWS_Os_00424]` The first call to `StartOS` does not return
- [ ] `[SWS_Os_00425]` If `ShutdownHook` returns: disable all interrupts, loop forever
- [ ] `[SWS_Os_00299]` `*AllInterrupts` services usable before `StartOS` and after `ShutdownOS`
- [ ] `[SWS_Os_00301]` Alarm action "increment a software counter"
- [ ] `[SWS_Os_00476]` Autostart of absolute alarms
- [ ] `[SWS_Os_00367]` Services without `StatusType` do not raise the error hook
- [ ] `[SWS_Os_00071]` `PostTaskHook` is not called during `ShutdownOS`
- [x] `[SWS_Os_00566]` Null-pointer checks — not applicable to the Rust API (references and return values); revisit if a C ABI is added

## Recommended in SC1, mandatory from SC3 — SWS OS §7.7.3.4

Service protection is formally an SC3/SC4 feature (`[SWS_Os_00241]`), but these
make an SC1 kernel far easier to debug. Decide per item.

- [ ] `[SWS_Os_00052]` A task returning without `TerminateTask` is terminated
- [ ] `[SWS_Os_00069]` ... and the error hook is called with `E_OS_MISSINGEND`
- [ ] `[SWS_Os_00070]` ... and its resources are released
- [ ] `[SWS_Os_00239]` ... and interrupts are re-enabled
- [ ] `[SWS_Os_00092]` Unmatched enable/resume calls are ignored
- [ ] `[SWS_Os_00093]` OS services called with interrupts disabled return `E_OS_DISABLEDINT`
- [ ] `[SWS_Os_00088]` Services called from the wrong context return `E_OS_CALLEVEL`

## Part of SC1, scheduled for stage 2

- [ ] Schedule tables — SWS OS §7.3 (`[SWS_Os_00241]`: minimum 2 tables in SC1)
- [ ] Stack monitoring — SWS OS §7.5: `[SWS_Os_00067]`, `[SWS_Os_00068]`, `[SWS_Os_00396]`
