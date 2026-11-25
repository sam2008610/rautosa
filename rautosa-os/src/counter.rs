//! Counters.
//!
//! OSEK only describes counters as a concept (OSEK OS §9.1) and leaves their
//! API to the implementation. AUTOSAR standardises it:
//!
//! * SWS OS `[SWS_Os_00399]` `IncrementCounter`
//! * SWS OS `[SWS_Os_00383]` `GetCounterValue`
//! * SWS OS `[SWS_Os_00392]` `GetElapsedValue`
//! * SWS OS §7.2 "Software Free Running Timer" (`[SWS_Os_00374]`,
//!   `[SWS_Os_00384]`), `[SWS_Os_00301]` (alarm action "increment counter").
//!
//! All three services are available in every scalability class
//! (`[SWS_Os_00530]`, `[SWS_Os_00532]`, `[SWS_Os_00534]`).

use crate::Result;

/// A counter value or a duration, in ticks (`TickType`, OSEK OS §13.6.1).
pub type Tick = u32;

/// Identifies a counter (`CounterType`, SWS OS `[SWS_Os_00786]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CounterId(u8);

impl CounterId {
    /// Creates a counter ID from its raw index.
    pub const fn new(raw: u8) -> Self {
        Self(raw)
    }

    /// The raw index of this counter.
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// Advances the software counter `counter` by one tick and processes the
/// alarms that expire.
///
/// # Errors
///
/// [`Error::Id`](crate::Error::Id) — `counter` is invalid or is a hardware
/// counter (`[SWS_Os_00285]`, extended status).
///
/// An error raised by an alarm *action* (e.g. `E_OS_LIMIT` from a task
/// activation) is reported to the error hook, but this service still returns
/// `Ok` (`[SWS_Os_00321]`).
///
/// # Specification
///
/// SWS OS `[SWS_Os_00399]`, `[SWS_Os_00286]`, `[SWS_Os_00529]`.
#[doc(alias = "IncrementCounter")]
pub fn increment_counter(counter: CounterId) -> Result {
    // What to do
    // ----------
    // * A counter wraps to 0 after `max_allowed_value` (OSEK OS §9.1), which is
    //   generally *not* `Tick::MAX`.
    // * Find the alarms attached to this counter that expire on the new value
    //   and run their actions (see `alarm`). Cyclic alarms are re-armed.
    //
    // Rust notes
    // ----------
    // * Modular arithmetic with an arbitrary modulus: `wrapping_add` alone is
    //   wrong, and `(a + b) % m` can overflow. Look at `checked_*` and think in
    //   terms of "distance to expiry".
    // * How do you iterate over alarms while their actions mutate kernel state
    //   (activate a task, set an event, increment *another* counter)? This is
    //   where the borrow checker will first push back on your data layout.
    todo!("IncrementCounter: SWS OS [SWS_Os_00399]")
}

/// Returns the current value of `counter`.
///
/// # Errors
///
/// [`Error::Id`](crate::Error::Id) — `counter` is invalid (`[SWS_Os_00376]`,
/// extended status).
///
/// # Specification
///
/// SWS OS `[SWS_Os_00383]`, `[SWS_Os_00377]`, `[SWS_Os_00531]`.
#[doc(alias = "GetCounterValue")]
pub fn get_counter_value(counter: CounterId) -> Result<Tick> {
    todo!("GetCounterValue: SWS OS [SWS_Os_00383]")
}

/// Returns the number of ticks elapsed since `previous`, and updates
/// `previous` to the current counter value.
///
/// # Errors
///
/// * [`Error::Id`](crate::Error::Id) — `counter` is invalid (`[SWS_Os_00381]`).
/// * [`Error::Value`](crate::Error::Value) — `previous` is larger than the
///   counter's maximum allowed value (`[SWS_Os_00391]`).
///
/// Both are extended status.
///
/// # Specification
///
/// SWS OS `[SWS_Os_00392]`, `[SWS_Os_00382]`, `[SWS_Os_00460]`,
/// `[SWS_Os_00533]`.
#[doc(alias = "GetElapsedValue")]
pub fn get_elapsed_value(counter: CounterId, previous: &mut Tick) -> Result<Tick> {
    todo!("GetElapsedValue: SWS OS [SWS_Os_00392]")
}
