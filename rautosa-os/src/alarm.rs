//! Alarms.
//!
//! * OSEK OS §9 "Alarms": counters (§9.1), alarm management (§9.2),
//!   alarm-callback routines (§9.3).
//! * OSEK OS §13.6 "Alarms": the services in this module.
//! * SWS OS `[SWS_Os_00304]`: `SetRelAlarm` with increment 0 is `E_OS_VALUE`.
//! * SWS OS `[SWS_Os_00242]`: alarm callbacks are only allowed in SC1.
//! * SWS OS `[SWS_Os_00301]`: "increment a software counter" as alarm action.
//! * SWS OS `[SWS_Os_00476]`: autostart of absolute alarms.
//!
//! An alarm is statically bound to one counter and one action: activate a
//! task, set an event, call a callback, or increment a counter.

use crate::Result;
use crate::counter::Tick;

/// Identifies an alarm (`AlarmType`, OSEK OS §13.6.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AlarmId(u8);

impl AlarmId {
    /// Creates an alarm ID from its raw index.
    pub const fn new(raw: u8) -> Self {
        Self(raw)
    }

    /// The raw index of this alarm.
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// Characteristics of the counter behind an alarm (`AlarmBaseType`,
/// OSEK OS §13.6.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AlarmBase {
    /// `maxallowedvalue` — the counter wraps to 0 after this value.
    pub max_allowed_value: Tick,
    /// `ticksperbase` — ticks required to reach a counter-specific unit.
    pub ticks_per_base: Tick,
    /// `mincycle` — smallest allowed cycle of a cyclic alarm (extended status).
    pub min_cycle: Tick,
}

/// Returns the characteristics of the counter that `alarm` is bound to.
///
/// # Errors
///
/// [`Error::Id`](crate::Error::Id) — `alarm` is invalid (extended status).
///
/// # Specification
///
/// OSEK OS §13.6.3.1.
#[doc(alias = "GetAlarmBase")]
pub fn get_alarm_base(alarm: AlarmId) -> Result<AlarmBase> {
    todo!("GetAlarmBase: OSEK OS §13.6.3.1")
}

/// Returns the number of ticks until `alarm` expires.
///
/// # Errors
///
/// * [`Error::NoFunc`](crate::Error::NoFunc) — `alarm` is not in use (standard
///   status).
/// * [`Error::Id`](crate::Error::Id) — `alarm` is invalid (extended status).
///
/// # Specification
///
/// OSEK OS §13.6.3.2.
#[doc(alias = "GetAlarm")]
pub fn get_alarm(alarm: AlarmId) -> Result<Tick> {
    todo!("GetAlarm: OSEK OS §13.6.3.2")
}

/// Arms `alarm` to expire `increment` ticks from now, then every `cycle`
/// ticks (`cycle == 0` means single-shot).
///
/// # Errors
///
/// * [`Error::State`](crate::Error::State) — `alarm` is already in use
///   (standard status).
/// * [`Error::Value`](crate::Error::Value) — `increment == 0` (standard and
///   extended status, `[SWS_Os_00304]`).
/// * [`Error::Id`](crate::Error::Id) — `alarm` is invalid (extended status).
/// * [`Error::Value`](crate::Error::Value) — `increment` or `cycle` outside the
///   limits of the counter (extended status).
///
/// # Specification
///
/// OSEK OS §13.6.3.3, §9.2; SWS OS `[SWS_Os_00304]`.
#[doc(alias = "SetRelAlarm")]
pub fn set_rel_alarm(alarm: AlarmId, increment: Tick, cycle: Tick) -> Result {
    // Design question: `cycle == 0` as "single-shot" is a C idiom. Would
    // `Option<NonZero<Tick>>` — or two separate functions — serve Rust callers
    // better? Whatever you choose, `compat::SetRelAlarm` keeps the spec shape.
    todo!("SetRelAlarm: OSEK OS §13.6.3.3")
}

/// Arms `alarm` to expire when its counter reaches `start`, then every `cycle`
/// ticks (`cycle == 0` means single-shot).
///
/// # Errors
///
/// As [`set_rel_alarm`], except that `start == 0` is a valid value.
///
/// # Specification
///
/// OSEK OS §13.6.3.4, §9.2.
#[doc(alias = "SetAbsAlarm")]
pub fn set_abs_alarm(alarm: AlarmId, start: Tick, cycle: Tick) -> Result {
    // Hint: read the note in §13.6.3.4 about a `start` value that is very
    // close to the current counter value, and decide what your implementation
    // does when `start` equals the current value.
    todo!("SetAbsAlarm: OSEK OS §13.6.3.4")
}

/// Cancels `alarm`.
///
/// # Errors
///
/// * [`Error::NoFunc`](crate::Error::NoFunc) — `alarm` is not in use (standard
///   status).
/// * [`Error::Id`](crate::Error::Id) — `alarm` is invalid (extended status).
///
/// # Specification
///
/// OSEK OS §13.6.3.5.
#[doc(alias = "CancelAlarm")]
pub fn cancel_alarm(alarm: AlarmId) -> Result {
    todo!("CancelAlarm: OSEK OS §13.6.3.5")
}
