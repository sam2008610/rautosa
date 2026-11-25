//! Cortex-M (ARMv7-M) port: context switch, first task start, tick source.
//!
//! # What to build
//!
//! * A context switch performed in the `PendSV` exception.
//! * Fabrication of the initial stack frame of a task, so that "returning"
//!   from an exception starts the task at its entry function.
//! * The jump from `start_os` into the first task.
//!
//! # Public references
//!
//! * *ARMv7-M Architecture Reference Manual* (ARM DDI 0403), part B1: the
//!   exception model, exception entry/return, `EXC_RETURN`, MSP vs. PSP,
//!   stack alignment (`CCR.STKALIGN`).
//! * *Cortex-M3 Devices Generic User Guide* (ARM DUI 0552): PendSV, SVCall,
//!   SysTick, `SHPR` priority registers.
//! * AAPCS (IHI 0042): which registers are callee-saved — i.e. exactly the
//!   ones exception entry does *not* stack for you.
//!
//! # Questions to answer before writing code
//!
//! * Which eight registers does the CPU stack on exception entry, and which
//!   remain for you to save?
//! * Why is PendSV conventionally given the lowest exception priority, and
//!   what would break if a SysTick ISR switched contexts directly?
//! * Tasks on PSP and the kernel/ISRs on MSP, or everything on MSP? What does
//!   each choice cost now, and what does SC3 (unprivileged tasks) need later?
//! * OSEK basic tasks never block, so they *can* share one stack. That is an
//!   optimisation — get the one-stack-per-task version working first.
//!
//! # Rust pitfalls
//!
//! * The handler must control every register, so it has to be assembly:
//!   `core::arch::naked_asm!` inside a `#[unsafe(naked)]` function, or
//!   `core::arch::global_asm!`. A normal `#[exception] fn PendSV()` gets a
//!   compiler-generated prologue that clobbers what you want to save.
//! * The assembly needs "current TCB" and "next TCB". Symbols referenced from
//!   assembly need a stable name (`#[unsafe(no_mangle)]`) and a layout you
//!   control (`#[repr(C)]`); `core::mem::offset_of!` avoids hard-coded offsets.
//! * A task stack is memory the compiler does not know is being used as a
//!   stack. Decide how it is declared (`static` array? `MaybeUninit`? wrapped
//!   in `UnsafeCell`?), who owns it, and how you guarantee 8-byte alignment.
//! * Write down the safety contract of every `unsafe` block as a `// SAFETY:`
//!   comment. For a context switch that comment is the real design document.
//! * This target has no FPU. If you later move to `mps2-an500` (Cortex-M7,
//!   `thumbv7em-none-eabihf`) the exception frame grows and lazy FP stacking
//!   enters the picture — keep the frame layout in one place.
