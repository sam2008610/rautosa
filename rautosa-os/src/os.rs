//! Operating system execution control.
//!
//! * OSEK OS §5 "Application modes".
//! * OSEK OS §11.3 "System start-up", §11.4 "System shutdown".
//! * OSEK OS §13.7 "Operating system execution control": the services in this
//!   module.
//! * SWS OS `[SWS_Os_00424]`: the first call to `StartOS` does not return.
//! * SWS OS `[SWS_Os_00425]`: if `ShutdownHook` returns, the OS disables all
//!   interrupts and enters an endless loop.
//! * SWS OS `[SWS_Os_00071]`: `PostTaskHook` is not called during shutdown.

use crate::Result;

/// An application mode (`AppModeType`, OSEK OS §13.7.1).
///
/// The mode passed to [`start_os`] selects which tasks and alarms are started
/// automatically.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppMode(u8);

impl AppMode {
    /// `OSDEFAULTAPPMODE` — always a valid argument for [`start_os`]
    /// (OSEK OS §13.7.3).
    pub const DEFAULT: Self = Self(0);

    /// Creates an application mode from its raw value.
    pub const fn new(raw: u8) -> Self {
        Self(raw)
    }

    /// The raw value of this mode.
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// Returns the application mode the OS was started in.
///
/// # Specification
///
/// OSEK OS §13.7.2.1.
#[doc(alias = "GetActiveApplicationMode")]
pub fn get_active_application_mode() -> AppMode {
    todo!("GetActiveApplicationMode: OSEK OS §13.7.2.1")
}

/// Starts the operating system in application mode `mode`. Does not return.
///
/// # Specification
///
/// OSEK OS §13.7.2.2, §11.3; SWS OS `[SWS_Os_00424]`.
#[doc(alias = "StartOS")]
pub fn start_os(mode: AppMode) -> ! {
    // What to do (OSEK OS §11.3, Figure 11-2)
    // ----------
    // 1. Initialise kernel state.
    // 2. Call `StartupHook` with interrupts still disabled.
    // 3. Activate autostart tasks and arm autostart alarms for `mode`.
    // 4. Enable interrupts and start scheduling. From here on the caller's
    //    stack (the reset/`main` stack) is never used by this context again.
    //    What runs when no task is ready?
    //
    // Rust notes
    // ----------
    // * `-> !` makes the compiler enforce [SWS_Os_00424] for you.
    // * `cortex_m::Peripherals::take()` hands out the core peripherals exactly
    //   once. Decide who owns SysTick, the NVIC and the SCB: the application
    //   (which then passes them in) or the kernel.
    todo!("StartOS: OSEK OS §13.7.2.2")
}

/// Shuts the operating system down. Does not return.
///
/// `status` is passed on to `ShutdownHook`. `Ok(())` corresponds to `E_OK`;
/// the OS itself only ever shuts down with an error.
///
/// # Specification
///
/// OSEK OS §13.7.2.3, §11.4; SWS OS `[SWS_Os_00425]`, `[SWS_Os_00071]`.
#[doc(alias = "ShutdownOS")]
pub fn shutdown_os(status: Result) -> ! {
    // Hint: on QEMU, the example application's ShutdownHook is a natural place
    // to call `cortex_m_semihosting::debug::exit`, which turns "OS shut down
    // with status X" into a process exit code that a test script can check.
    todo!("ShutdownOS: OSEK OS §13.7.2.3")
}
