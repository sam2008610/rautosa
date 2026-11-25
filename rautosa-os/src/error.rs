//! Status and error codes.
//!
//! * OSEK OS §13.1 "Common data types": `StatusType`, `E_OK`, `E_OS_*`.
//! * OSEK OS §12.2.3 "Error characteristics": standard vs. extended status.
//! * SWS OS §7.15 `[SWS_Os_91025]`: additional AUTOSAR error codes.
//!
//! The specification models the outcome of a service as a `StatusType` integer
//! where `E_OK == 0`. In Rust that is `Result<T, Error>`: `Ok` *is* `E_OK`, and
//! [`Error`] only has to enumerate the failures. The numeric encoding is still
//! available through [`Error::status`] for the [`compat`](crate::compat) layer
//! and for trace tools.

use core::fmt;

/// Result type of all OS services. `Ok` corresponds to `E_OK`.
pub type Result<T = ()> = core::result::Result<T, Error>;

/// An OS service error (every `StatusType` value except `E_OK`).
///
/// Discriminants 1–8 are fixed by OSEK OS §13.1. The AUTOSAR codes are
/// "assigned by implementation" (`[SWS_Os_91025]`); the values below are
/// Rautosa's choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[non_exhaustive]
pub enum Error {
    /// `E_OS_ACCESS` — the caller is not allowed to use the object this way.
    Access = 1,
    /// `E_OS_CALLEVEL` — service called from a context that must not call it.
    CallLevel = 2,
    /// `E_OS_ID` — invalid object identifier.
    Id = 3,
    /// `E_OS_LIMIT` — too many task activations.
    Limit = 4,
    /// `E_OS_NOFUNC` — the object is not in use / nothing to do.
    NoFunc = 5,
    /// `E_OS_RESOURCE` — the caller still occupies a resource.
    Resource = 6,
    /// `E_OS_STATE` — the object is in the wrong state.
    State = 7,
    /// `E_OS_VALUE` — parameter value out of range.
    Value = 8,

    /// `E_OS_SERVICEID` — the service cannot be called (AUTOSAR).
    ServiceId = 9,
    /// `E_OS_ILLEGAL_ADDRESS` — invalid address passed to a service
    /// (AUTOSAR, `[SWS_Os_00566]`). The idiomatic API takes references and
    /// returns values, so only an FFI layer can ever produce this.
    IllegalAddress = 10,
    /// `E_OS_MISSINGEND` — a task returned without `TerminateTask`/`ChainTask`
    /// (AUTOSAR, `[SWS_Os_00069]`).
    MissingEnd = 11,
    /// `E_OS_DISABLEDINT` — service called while interrupts are disabled by the
    /// caller (AUTOSAR, `[SWS_Os_00093]`).
    DisabledInt = 12,
    /// `E_OS_STACKFAULT` — stack monitoring detected a fault
    /// (AUTOSAR, `[SWS_Os_00068]`).
    StackFault = 13,
}

impl Error {
    /// Every error, in discriminant order.
    pub const ALL: [Error; 13] = [
        Error::Access,
        Error::CallLevel,
        Error::Id,
        Error::Limit,
        Error::NoFunc,
        Error::Resource,
        Error::State,
        Error::Value,
        Error::ServiceId,
        Error::IllegalAddress,
        Error::MissingEnd,
        Error::DisabledInt,
        Error::StackFault,
    ];

    /// The numeric `StatusType` value of this error. Never 0 (`E_OK`).
    pub const fn status(self) -> u8 {
        self as u8
    }

    /// Converts a numeric `StatusType` back into an error.
    ///
    /// Returns `None` for 0 (`E_OK`) and for unknown codes.
    pub const fn from_status(status: u8) -> Option<Self> {
        let mut i = 0;
        while i < Self::ALL.len() {
            if Self::ALL[i] as u8 == status {
                return Some(Self::ALL[i]);
            }
            i += 1;
        }
        None
    }

    /// The specification name of this error, e.g. `"E_OS_LIMIT"`.
    pub const fn name(self) -> &'static str {
        match self {
            Error::Access => "E_OS_ACCESS",
            Error::CallLevel => "E_OS_CALLEVEL",
            Error::Id => "E_OS_ID",
            Error::Limit => "E_OS_LIMIT",
            Error::NoFunc => "E_OS_NOFUNC",
            Error::Resource => "E_OS_RESOURCE",
            Error::State => "E_OS_STATE",
            Error::Value => "E_OS_VALUE",
            Error::ServiceId => "E_OS_SERVICEID",
            Error::IllegalAddress => "E_OS_ILLEGAL_ADDRESS",
            Error::MissingEnd => "E_OS_MISSINGEND",
            Error::DisabledInt => "E_OS_DISABLEDINT",
            Error::StackFault => "E_OS_STACKFAULT",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl core::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn osek_codes_have_the_values_from_the_specification() {
        // OSEK OS §13.1
        assert_eq!(Error::Access.status(), 1);
        assert_eq!(Error::CallLevel.status(), 2);
        assert_eq!(Error::Id.status(), 3);
        assert_eq!(Error::Limit.status(), 4);
        assert_eq!(Error::NoFunc.status(), 5);
        assert_eq!(Error::Resource.status(), 6);
        assert_eq!(Error::State.status(), 7);
        assert_eq!(Error::Value.status(), 8);
    }

    #[test]
    fn status_round_trips() {
        for error in Error::ALL {
            assert_eq!(Error::from_status(error.status()), Some(error));
        }
    }

    #[test]
    fn e_ok_and_unknown_codes_are_not_errors() {
        assert_eq!(Error::from_status(0), None);
        assert_eq!(Error::from_status(200), None);
    }

    #[test]
    fn display_uses_the_specification_name() {
        assert_eq!(Error::Limit.to_string(), "E_OS_LIMIT");
    }
}
