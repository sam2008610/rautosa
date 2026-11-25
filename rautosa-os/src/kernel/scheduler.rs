//! Ready queue and scheduling decision.
//!
//! # What to build
//!
//! * A ready queue that returns the highest-priority ready task, FIFO among
//!   tasks of equal priority.
//! * The scheduling decision: given the running task and the ready queue,
//!   should a switch happen *now*?
//!
//! # Specification
//!
//! * OSEK OS §4.5 "Task priority", Figure 4-3: the scheduler's queue model. A
//!   preempted task is the *first* of its priority's queue; a task released
//!   from *waiting* is the *last*.
//! * OSEK OS §4.6: full preemptive (§4.6.1), non preemptive (§4.6.2), groups
//!   of tasks (§4.6.3), mixed (§4.6.4). §4.6.2 lists the exact rescheduling
//!   points of a non-preemptable task.
//! * OSEK OS §8.5: the *current* priority of a task changes while it holds a
//!   resource; the scheduler must compare against that, not the static one.
//! * OSEK OS §8.7: internal resources — this is how non-preemptable tasks and
//!   task groups are modelled.
//!
//! # Rust questions to think about
//!
//! * No allocator: the queue has a fixed capacity known from the
//!   configuration. Const generics (`[T; N]`), or a crate such as `heapless`?
//!   Writing one ring buffer yourself is a good exercise in `MaybeUninit`
//!   and `const fn`; you can still swap in a crate later.
//! * A priority bitmap plus `leading_zeros()` gives an O(1) "highest ready
//!   priority". Check what that compiles to on thumbv7m (`CLZ`).
//! * Keep this module free of hardware access so it stays host-testable; the
//!   decision ("switch to task X") and the mechanism (pend PendSV) are
//!   separate concerns.
