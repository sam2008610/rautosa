# Roadmap

Rautosa is built in four stages. Each stage ends with something that runs on
QEMU, a set of tests, and one article.

Sources are cited as "OSEK OS §x" (OSEK/VDX OS 2.2.3) and `[SWS_Os_nnnnn]`
(AUTOSAR CP SWS OS, R25-11). To read the full text of any requirement, generate
the local requirement browser (see `README.md`, "Specification browser") and
search for the ID.

| Stage | Theme | QEMU machine | Target |
|-------|-------|--------------|--------|
| 0 | Toolchain, workspace, API shell | `mps2-an385` (Cortex-M3) | `thumbv7m-none-eabi` |
| 1 | SC1 / OSEK core | `mps2-an385` | `thumbv7m-none-eabi` |
| 2 | Schedule tables | `mps2-an385` | `thumbv7m-none-eabi` |
| 3 | SC3 memory protection | `mps2-an385` (8 MPU regions) | `thumbv7m-none-eabi` |
| 4 | Multi-core | `mps2-an521` (dual Cortex-M33) | `thumbv8m.main-none-eabi` |

> **A note on scalability classes.** `[SWS_Os_00241]` puts schedule tables and
> stack monitoring into *every* scalability class, including SC1. So "SC1" in
> the strict AUTOSAR sense is complete only after stage 2. Stage 1 below is
> "OSEK OS plus the AUTOSAR counter API", which is the natural first milestone.

---

## Stage 0 — Foundations (done)

- [x] Cargo workspace, pinned toolchain, QEMU runner.
- [x] `cargo run --example hello` prints `Hello from Rautosa` and exits 0.
- [x] `rautosa-os` API shell: idiomatic layer + `compat` layer, all `todo!()`.
- [ ] `docs/exercise-00.md` solved (Rust basics: a ring buffer, host-tested). **← you are here**
- [ ] `docs/exercise-01.md` solved (SysTick and shared state).

---

## Stage 1 — SC1 / OSEK core

**Goal.** A preemptive, statically configured OSEK kernel: basic and extended
tasks, events, resources with priority ceiling, counters and alarms.

**Scope.**

| Feature | Source |
|---------|--------|
| Basic tasks, state model, activation, termination | OSEK OS §4.2.2, §4.3, §4.7, §13.2 |
| Full / non / mixed preemptive scheduling | OSEK OS §4.6 |
| Multiple activations, several tasks per priority (BCC2/ECC2) | OSEK OS §3.2, §4.3, §4.5 |
| Extended tasks and events | OSEK OS §4.2.1, §7, §13.5 |
| Resources, priority ceiling protocol, internal resources | OSEK OS §8, §13.4 |
| Category 1 and 2 ISRs, interrupt services | OSEK OS §6, §13.3; `[SWS_Os_00299]` |
| Counters | OSEK OS §9.1; `[SWS_Os_00399]`, `[SWS_Os_00383]`, `[SWS_Os_00392]` |
| Alarms (activate task, set event, callback, increment counter) | OSEK OS §9, §13.6; `[SWS_Os_00304]`, `[SWS_Os_00242]`, `[SWS_Os_00301]`, `[SWS_Os_00476]` |
| StartOS / ShutdownOS, application modes, autostart | OSEK OS §5, §11.3, §11.4, §13.7; `[SWS_Os_00424]`, `[SWS_Os_00425]` |
| Hooks and error handling, standard vs. extended status | OSEK OS §11.1, §11.2, §12.2.3, §13.8; `[SWS_Os_00439]`, `[SWS_Os_00367]` |

Suggested order of work (each step is a mergeable increment):

1. Static configuration: decide how tasks are declared (`docs/design-notes.md`).
2. Task state machine and ready queue, host-tested only.
3. Context switch on Cortex-M; two basic tasks ping-pong via `ActivateTask` /
   `TerminateTask`.
4. Preemption from an ISR; non-preemptive tasks and `Schedule`.
5. Resources and the priority ceiling protocol.
6. Extended tasks and events (first use of per-task stacks that survive a
   block).
7. Counters and alarms driven by SysTick.
8. Hooks, extended status checks, `ShutdownOS`.

**Definition of Done.**

- [ ] Every box in `docs/sc1-api-checklist.md` is ticked.
- [ ] Every service has at least one test for its success path and one per
      error code listed in the specification (standard and extended status).
- [ ] Host tests cover the state machine, ready queue, ceiling computation and
      counter arithmetic: `cargo test -p rautosa-os --target <host>`.
- [ ] QEMU scenario tests exist as examples that exit 0 on pass and non-zero
      on fail, covering at least:
      preemption by a higher-priority task; FIFO order within one priority;
      a classic priority-inversion scenario that the ceiling protocol prevents;
      an extended task blocking on and being released by an event set from an
      ISR; a cyclic alarm activating a task; `E_OS_LIMIT` on over-activation.
- [ ] Every `unsafe` block has a `// SAFETY:` comment; `unsafe` exists only in
      the architecture layer and in clearly marked kernel-state accessors.
- [ ] `cargo build`, `cargo lint`, `cargo fmt --check` are clean.
- [ ] No design decision in `docs/design-notes.md` relevant to stage 1 is
      still "open".

**Article.** *"Priority ceiling in Rust: an OSEK kernel where the borrow
checker reviews your critical sections"* — task model and scheduler in a
statically configured `no_std` kernel; what the ceiling protocol guarantees
(OSEK OS §8.5) and how much of it can be encoded in types; what `unsafe` was
unavoidable in the context switch and why.

Smaller pieces along the way: the Cortex-M context switch in `naked_asm!`;
static configuration with `const fn`; why `TerminateTask` and RAII do not mix.

---

## Stage 2 — Schedule tables

**Goal.** Statically defined activation patterns driven by a counter
(SWS OS §7.3), completing SC1 in the AUTOSAR sense.

**Scope.**

| Feature | Source |
|---------|--------|
| Structure: expiry points, offsets, actions | `[SWS_Os_00401]`–`[SWS_Os_00404]` |
| Configuration constraints | `[SWS_Os_00407]`, `[SWS_Os_00442]`, `[SWS_Os_00443]`, `[SWS_Os_00408]` |
| Processing, one counter per table, several tables concurrently | `[SWS_Os_00002]`, `[SWS_Os_00007]`, `[SWS_Os_00409]`–`[SWS_Os_00412]` |
| Single-shot and repeating tables, final delay | `[SWS_Os_00413]`, `[SWS_Os_00009]`, `[SWS_Os_00427]`, `[SWS_Os_00444]`, `[SWS_Os_00194]` |
| Start / stop / switch, autostart | `[SWS_Os_00428]`, `[SWS_Os_00414]`, `[SWS_Os_00510]` |
| Services | `StartScheduleTableRel` `[SWS_Os_00347]`, `StartScheduleTableAbs` `[SWS_Os_00358]`, `StopScheduleTable` `[SWS_Os_00006]`, `NextScheduleTable` `[SWS_Os_00191]`, `GetScheduleTableStatus` `[SWS_Os_00227]` |
| Stack monitoring | `[SWS_Os_00067]`, `[SWS_Os_00068]`, `[SWS_Os_00396]` |

Out of scope: schedule table synchronisation (SWS OS §7.4) — that is SC2/SC4.

**Definition of Done.**

- [ ] All five services implemented in both API layers, with tests per error
      code.
- [ ] The configuration constraints above are rejected **at compile time**
      (`const` evaluation), each with a compile-fail test or a documented
      example.
- [ ] QEMU scenarios: a repeating table whose expiry points fire at the exact
      configured ticks over several rounds, including one counter wrap-around;
      `NextScheduleTable` switching at the end of a round; two tables on one
      counter.
- [ ] Alarms and schedule tables share one counter-expiry mechanism (no
      duplicated "what expires next" logic).
- [ ] Stack monitoring detects an overflowing task and ends in
      `ShutdownOS(E_OS_STACKFAULT)`.
- [ ] The stage 1 tests still pass.

**Article.** *"Making invalid schedule tables unrepresentable: compile-time
validation of AUTOSAR OS configuration with `const fn`"* — what a C generator
checks at generation time and how far Rust's const evaluation can replace it.

---

## Stage 3 — SC3: memory protection

**Goal.** OS-Applications isolated by the MPU; non-trusted code runs
unprivileged and cannot corrupt the kernel or other applications.

**Scope.**

| Feature | Source |
|---------|--------|
| OS-Applications, trusted / non-trusted, object ownership | SWS OS §7.6; `[SWS_Os_00445]`, `[SWS_Os_00446]`, `[SWS_Os_00448]` |
| Memory protection of OS data, stacks, application data | SWS OS §7.7.1; `[SWS_Os_00198]`, `[SWS_Os_00207]`, `[SWS_Os_00196]`, `[SWS_Os_00355]`, `[SWS_Os_00356]`, `[SWS_Os_00044]` |
| Service protection: wrong context, invalid objects, undefined behaviour | SWS OS §7.7.3; `[SWS_Os_00088]`, `[SWS_Os_00093]`, `[SWS_Os_00052]`, `[SWS_Os_00069]`, `[SWS_Os_00070]` |
| Protecting OS hardware, privileged / unprivileged mode | SWS OS §7.7.4 |
| Trusted functions | SWS OS §7.7.5; `CallTrustedFunction` `[SWS_Os_00097]` |
| Protection hook and reactions | SWS OS §7.8; `[SWS_Os_00107]`, `[SWS_Os_00553]`, `[SWS_Os_00554]`, `[SWS_Os_00556]` |
| Access-check and application services | `[SWS_Os_00016]`, `[SWS_Os_00512]`, `[SWS_Os_00513]`, `[SWS_Os_00256]`, `[SWS_Os_00017]`, `[SWS_Os_00258]` |
| Extended status always on | `[SWS_Os_00327]` |

Architecture work this stage forces: a system-call boundary (SVC) between
unprivileged tasks and the kernel; MPU reprogramming on context switch; a
linker script that places each application's data in its own region.

**Definition of Done.**

- [ ] A non-trusted task writing to kernel data, to another application's
      data, and below its own stack each ends in `ProtectionHook` with
      `E_OS_PROTECTION_MEMORY` — three QEMU tests.
- [ ] Each `ProtectionHook` return value (`PRO_TERMINATETASKISR`,
      `PRO_TERMINATEAPPL`, `PRO_SHUTDOWN`) has a test showing the system
      continues or stops as specified.
- [ ] Non-trusted tasks run unprivileged; every OS service is reached through
      the system-call path and validates object IDs and ownership.
- [ ] A document lists every MPU region, who may access it, and how the 8
      regions of the Cortex-M3 are budgeted.
- [ ] A task that returns without `TerminateTask` is cleaned up with
      `E_OS_MISSINGEND`.
- [ ] Stage 1 and 2 scenarios pass when run as non-trusted applications.

**Article.** *"Two kinds of memory safety: where Rust's guarantees end and the
MPU's begin"* — what the compiler proves at build time, what the MPU enforces
at run time, and why a safety-relevant RTOS wants both (freedom from
interference between applications).

---

## Stage 4 — Multi-core

**Goal.** Two cores under one OS: cross-core task activation and events,
spinlocks, and the IOC.

**Platform change.** No QEMU ARMv7-M machine has two cores. This stage moves
to `mps2-an521` (dual Cortex-M33, ARMv8-M Mainline). ARMv8-M has a different
MPU programming model (base/limit instead of size-aligned regions) and
TrustZone — expect to touch the architecture layer written in stages 1 and 3.
Verify the machine and its second-core start-up behaviour in QEMU before
planning details.

**Scope.**

| Feature | Source |
|---------|--------|
| Per-core scheduling, locatable entities | SWS OS §7.9.2, §7.9.3 |
| Start-up and shutdown of cores | SWS OS §7.9.4–§7.9.6; `StartCore` `[SWS_Os_00676]`, `GetCoreID` `[SWS_Os_00674]`, `ShutdownAllCores` `[SWS_Os_00713]` |
| Cross-core `ActivateTask`, `SetEvent`, alarms | SWS OS §7.9.10, §7.9.12, §7.9.26; `[SWS_Os_00589]` |
| Spinlocks, nesting order, deadlock prevention | SWS OS §7.9.28; `[SWS_Os_00648]`, `[SWS_Os_00649]`, `GetSpinlock` `[SWS_Os_00686]`, `ReleaseSpinlock` `[SWS_Os_00695]`, `TryToGetSpinlock` `[SWS_Os_00703]` |
| IOC: queued and last-is-best, 1:1 and N:1 | SWS OS §7.10, §8.5; `IocSend` `[SWS_Os_00718]`, `IocReceive` `[SWS_Os_00738]` |

**Definition of Done.**

- [ ] Both cores boot, run their own autostart tasks and schedule
      independently.
- [ ] A task on core 0 activates a task and sets an event on core 1.
- [ ] Spinlock stress test: both cores increment a shared counter N times; the
      final value is exact. Wrong nesting order is rejected (`[SWS_Os_00691]`) with
      `E_OS_NESTING_DEADLOCK`.
- [ ] IOC: queued and last-is-best channels across cores, including the
      queue-full and no-data cases.
- [ ] Kernel state is either per-core or protected by a documented lock; the
      document states the lock order.
- [ ] `Send` / `Sync` bounds on kernel types are justified in comments — no
      blanket `unsafe impl Sync`.
- [ ] All single-core tests still pass on core 0.

**Article.** *"`Send`, `Sync` and spinlocks: what Rust's concurrency model
buys an AUTOSAR multi-core OS"* — mapping locatable entities, spinlocks and
the IOC onto ownership; which data races the compiler rules out and which
deadlocks it cannot.

---

## Parallel track — functional safety practice

See `docs/safety-practice.md` for a proposal on practising ISO 26262 part 6
activities alongside these stages (requirements traceability, coding
guidelines, verification evidence).
