//! Smoke test: boots on QEMU, prints a greeting over semihosting and exits.
//!
//! Run with `cargo run --example hello`.

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use panic_semihosting as _;
use rautosa_os as _;

#[entry]
fn main() -> ! {
    hprintln!("Hello from Rautosa");

    // Ask QEMU to terminate with exit code 0. On real hardware this is a no-op,
    // so fall through to an idle loop.
    debug::exit(debug::EXIT_SUCCESS);
    loop {
        cortex_m::asm::wfi();
    }
}
