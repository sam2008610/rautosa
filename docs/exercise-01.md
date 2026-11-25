# Exercise 01 — Count SysTick interrupts

A warm-up before the kernel: one interrupt, one piece of state shared between
the interrupt handler and `main`. This is the smallest version of the problem
the whole kernel has — tasks and ISRs touching the same data — and the goal is
to see how Rust makes you state *why* each access is sound.

No answers here. Hints are at the end; try without them first.

## Starting point

`examples/systick.rs` compiles, boots and immediately panics:

```text
$ cargo run --example systick
panicked at examples/systick.rs:20:5:
not yet implemented: exercise 01: configure SysTick and report ten ticks
```

## Task

### Part A — `Mutex<RefCell<_>>`

Make `cargo run --example systick` print exactly this and exit with code 0:

```text
tick 1
tick 2
...
tick 10
```

Requirements:

1. SysTick fires at 100 Hz. QEMU's `mps2-an385` models a 25 MHz system clock.
2. The `SysTick` exception handler increments a counter.
3. `main` prints each new counter value and, once it reaches 10, terminates
   QEMU with `debug::exit(debug::EXIT_SUCCESS)`.
4. The counter is a `static` protected by a `Mutex<RefCell<u32>>`.
5. **No `unsafe`**, and no `static mut`.
6. `main` sleeps between ticks instead of spinning.
7. `cargo lint` stays clean.

### Part B — atomics

Copy your solution to `examples/systick_atomic.rs` (add an `[[example]]` entry
in `examples/Cargo.toml`) and replace the `Mutex<RefCell<u32>>` with an
`AtomicU32`. Same output, still no `unsafe`.

Then answer in a comment at the top of the file:

- Which `Ordering` did you pick for the handler and for `main`, and why is it
  sufficient on a single core?
- Your handler does "load, add one, store". Is that a data race? Is it a race
  condition? What changes if a *second, higher-priority* interrupt also
  increments the counter?
- This would not compile for `thumbv6m-none-eabi` (Cortex-M0) if you used
  `fetch_add`. Why not, and what do people do about it?

### Part C — hand a peripheral to the handler

Back in the `Mutex` version: make the *handler* stop the timer after the tenth
tick, so that no eleventh interrupt ever fires. `main` configured `SYST`, so
ownership of the peripheral has to move from `main` into something the handler
can reach.

`main` should still print the ten lines and exit. Verify that the handler ran
exactly ten times (how can you tell?).

### Part D — think, don't code

Write your answers in `docs/notes/exercise-01.md` (create it). These are the
questions the kernel design will keep asking.

1. Why does the compiler reject a plain `static COUNTER: RefCell<u32>`? Name
   the trait and explain what it promises.
2. `Mutex<RefCell<u32>>`: which of the two layers prevents the interrupt from
   observing a half-finished update, and which one satisfies the borrow
   checker? Could you drop either?
3. `RefCell` can panic at run time. Construct (on paper) a sequence of events
   in which `borrow_mut()` panics in this program, or argue that none exists.
4. What does a critical section cost here, in terms of interrupt latency?
   What would it cost in a kernel where the protected data is a ready queue?
5. `cortex-m`'s feature `critical-section-single-core` implements critical
   sections by masking interrupts. Why is that implementation unsound on a
   dual-core MCU, and what does that tell you about roadmap stage 4?
6. In a C kernel this counter would be `static volatile uint32_t ticks;`. List
   what the Rust version makes explicit that the C version leaves to the
   reviewer.
7. The OSEK interrupt services (`SuspendAllInterrupts` /
   `ResumeAllInterrupts`, OSEK OS §13.3.2) are a pair of calls rather than a
   closure or a guard. Sketch two Rust API shapes for the same thing and
   list one advantage of each.

## Acceptance criteria

- [ ] `cargo run --example systick` prints `tick 1` … `tick 10`, exit code 0.
- [ ] `cargo run --example systick_atomic` does the same.
- [ ] `grep -n unsafe examples/systick*.rs` prints nothing.
- [ ] `cargo lint` and `cargo fmt --all --check` are clean.
- [ ] Part C: the handler provably runs exactly ten times.
- [ ] `docs/notes/exercise-01.md` answers the seven questions.

## Hints

Read only as far as you need.

<details>
<summary>Hint 1 — where things are</summary>

- `cortex_m::Peripherals::take()` returns the core peripherals once;
  `SYST` is the SysTick timer.
- `cortex_m::peripheral::syst::SystClkSource` selects the clock.
- `#[exception] fn SysTick()` in `cortex-m-rt` installs the handler; the
  skeleton already has it.
- There are two `Mutex` types that work here: `cortex_m::interrupt::Mutex`
  (with `cortex_m::interrupt::free`) and `critical_section::Mutex` (with
  `critical_section::with`). They look alike; find out how they differ and
  which one newer code prefers. For the second one you need to add the
  `critical-section` crate to `examples/Cargo.toml`.
</details>

<details>
<summary>Hint 2 — the reload value</summary>

SysTick counts down from `RELOAD` to 0 and then reloads, so one period is
`RELOAD + 1` clock cycles. It is a 24-bit counter: check that your value fits.
Also look at what `SYST::clear_current` is for.
</details>

<details>
<summary>Hint 3 — sleeping</summary>

`cortex_m::asm::wfi()`. Think about what happens if the interrupt arrives
between "I checked the counter" and "I went to sleep", and whether that
matters for this particular program.
</details>

<details>
<summary>Hint 4 — Part C</summary>

A `static` must be initialised at compile time, but the peripheral only exists
at run time. Which type expresses "nothing yet, something later"? The pattern
you end up with is common enough to have a name in the Rust embedded book:
look for "sharing peripherals" in the *Concurrency* chapter.

`cortex-m-rt` also offers a different tool for handler-private state: a
`static mut` *declared inside* an `#[exception]` function is handed to you as
a safe `&mut`. Find out why that is sound, and why it does not solve Part C.
</details>

<details>
<summary>Hint 5 — when it does not work</summary>

- Nothing printed and QEMU hangs: is the SysTick *interrupt* enabled, not just
  the counter? Quit QEMU with `Ctrl-A` then `X`.
- `already borrowed: BorrowMutError`: you are holding a `RefCell` borrow while
  doing something that borrows again. Shorten the borrow.
- For a debugger: add `-s -S` to the QEMU command line and connect with
  `arm-none-eabi-gdb` or `lldb` (`gdb-remote 1234`).
</details>

## Why this exercise

| You practise here | You will need it for |
|-------------------|----------------------|
| `static` + interior mutability | every piece of kernel state |
| critical sections and their cost | `SuspendAllInterrupts`, scheduler locking |
| moving ownership into a handler | the kernel owning SysTick as the system counter |
| atomics and orderings | lock-free flags, later spinlocks (stage 4) |
| SysTick itself | the counter behind alarms (OSEK OS §9.1) |
