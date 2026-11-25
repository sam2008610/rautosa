# Design notes

Open design decisions, in roughly the order they will block you. Each entry
states the question, the constraints from the specification, and options to
compare — not an answer. When you decide, record the decision and the reason
here; these entries are raw material for the articles.

Ground rule: the design follows the public specifications and idiomatic Rust.
It does not need to resemble any existing C implementation.

---

## 1. Static configuration — **open, blocks stage 1**

**Question.** How does an application declare its tasks, resources, events,
counters and alarms?

**Constraints.** All OS objects are known at build time (OSEK OS §12.1). No
allocator. Derived data — ceiling priorities (OSEK OS §8.5), queue sizes,
validity checks — should be computed at build time, as a C generator would.

**Options.**

- `const` tables plus `const fn` validation, written by hand in the
  application. No tooling; errors appear as compile errors via `const` panics.
- A declarative macro (`macro_rules!`) that expands to those tables.
- A procedural macro or a `build.rs` generator reading a description file
  (TOML? OIL? ARXML is the real-world format and far too large for now).
- Generic kernel type `Os<const TASKS: usize, ...>` vs. link-time symbols the
  kernel refers to (`extern` statics) vs. a trait implemented by the
  application.

**Things to weigh.** Compile-error quality; how IDs become typed constants
(`TaskId` values the application can name); whether an invalid ID can exist at
all; rust-analyzer support; how the examples stay readable.

## 2. Where kernel state lives — **open, blocks stage 1**

**Question.** The kernel is a singleton reached from free functions
(`activate_task(id)`) called by tasks *and* ISRs. What holds its mutable
state?

**Options.** One `static` behind a critical-section mutex; several statics
with finer-grained protection; per-object atomics where possible; a
`static` `UnsafeCell` wrapper with a documented "interrupts are masked"
access contract.

**Things to weigh.** Exercise 01 is this question in miniature. Borrows must
not be held across a context switch. Stage 4 turns "mask interrupts" into
"mask interrupts and take a lock", so keep the access path in one place.

## 3. Task entry signature and termination — **open, blocks stage 1**

**Question.** What type is a task body, and how does it end?

**Constraints.** `TerminateTask` does not return (OSEK OS §13.2.3.2). A task
that returns without calling it is an error that the OS may clean up
(`[SWS_Os_00052]`, `[SWS_Os_00069]`).

**Options.** `fn()` whose return *is* termination; `fn() -> !`;
`fn() -> Result<Infallible, Error>` ending in `terminate_task()?`;
`extern "C" fn()` to simplify the first stack frame.

**Things to weigh.** Abandoning a stack skips destructors. Which RAII guards
could be alive at that point, and is skipping each of them merely a leak or a
real bug?

## 4. Paired services vs. guards — open

**Question.** `GetResource`/`ReleaseResource` and the interrupt
suspend/resume services are call pairs with LIFO rules. Offer guards or
closures on top?

**Options.** Spec-shaped functions only; guard types with `Drop`; closure
API (`with_resource(id, |…| …)`); both, with the guard built on the pair.

**Things to weigh.** A guard cannot be held across `wait_event` or
`terminate_task` (the specification forbids holding a resource there:
`E_OS_RESOURCE`). Can the type system express that, or is the run-time check
the honest solution? `mem::forget` on a guard is safe Rust — what happens
then?

## 5. Hook registration — open

See `rautosa_os::hooks`. Trait object, generic parameter, function-pointer
table in the configuration, or `extern "Rust"` symbols? Tied to decision 1.

## 6. Standard vs. extended status — open

OSEK OS §12.2.3 defines two error-checking levels. A Cargo feature
(`extended-status`)? A `const` in the configuration? Always extended (SC3 and
SC4 require it anyway, `[SWS_Os_00327]`)? Note that Cargo features must be
additive.

## 7. Architecture boundary — open, blocks the context switch

See `rautosa-os/src/kernel/arch/mod.rs`. What crosses the line between
portable kernel and CPU port, and by which mechanism (trait, `cfg` modules,
link-time symbols)? Stage 4 brings a second architecture (ARMv8-M), stage 3
needs the MPU behind the same boundary.

## 8. Who owns the core peripherals — open

`cortex_m::Peripherals::take()` succeeds once. Does the application take them
and pass SysTick/SCB to `start_os`, or does the kernel take them? What does
the answer mean for an application that wants its own use of the NVIC?

## 9. Relationship to `critical-section` — open

The examples enable `cortex-m`'s `critical-section-single-core`. Should the
kernel provide its own `critical_section::Impl` (so that libraries using
`critical-section` cooperate with OS interrupt masking), and should the OSEK
interrupt services be built on it or the other way round?

## 10. Smaller questions

- ID width: `u8` today. Enough? Does `TaskId` need a niche for
  `Option<TaskId>` to stay one byte (`NonZero`, or an enum generated by the
  configuration)?
- `set_rel_alarm(.., cycle: Tick)` with `0` meaning single-shot, or
  `Option<NonZero<Tick>>`?
- `shutdown_os` takes `Result<(), Error>`; OSEK allows application-defined
  codes (see the TODO in `compat::ShutdownOS`).
- `EventMask` width: `u32` today; OSEK leaves it to the implementation.
- Error context for the error hook: global query functions as in the
  specification (`OSErrorGetServiceId`), or a context argument?

---

## Decisions made

| # | Decision | Reason |
|---|----------|--------|
| D1 | QEMU `mps2-an385`, Cortex-M3, `thumbv7m-none-eabi` for stages 1–3 | Has an MPU (8 regions, verified with a probe that takes a MemManage fault on a write to a read-only region). No FPU, so the first context switch has one frame layout. Same board family as `mps2-an500` (M7) and `mps2-an521` (dual M33). |
| D2 | Semihosting for console output and for ending QEMU | One mechanism for printing and for exit codes; no driver code before the kernel exists. Not for real hardware. |
| D3 | Two API layers: idiomatic modules plus `compat` | Rust callers get `Result`, newtypes and return values; readers who know AUTOSAR find every service under its specification name. |
| D4 | Toolchain pinned in `rust-toolchain.toml` | Reproducible builds; `naked_asm!`, `core::error::Error` and edition 2024 are available. |
| D5 | Specification text stays out of the repository | AUTOSAR allows reproduction for informational purposes only. The extractor is committed, its output is git-ignored. |
