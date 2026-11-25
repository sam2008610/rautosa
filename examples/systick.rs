//! Exercise 01 — count SysTick interrupts.
//!
//! The task description, hints and acceptance criteria are in
//! `docs/exercise-01.md`. Run with `cargo run --example systick`.
//!
//! As shipped, this program panics with "not yet implemented" and QEMU exits
//! with a non-zero code. That is the starting point, not a bug.

#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception};
use panic_semihosting as _;

#[entry]
fn main() -> ! {
    // Step 1: take the core peripherals and configure SysTick for 100 Hz.
    // Step 3: observe the tick count from here, print each new value, and exit
    //         QEMU successfully after the tenth tick.
    todo!("exercise 01: configure SysTick and report ten ticks")
}

#[exception]
fn SysTick() {
    // Step 2: count this tick in state that `main` can read.
    todo!("exercise 01: count the tick")
}
