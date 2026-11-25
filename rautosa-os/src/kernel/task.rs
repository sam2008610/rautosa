//! Task control block and task state machine.
//!
//! # What to build
//!
//! * A representation of the *static* part of a task (priority, basic or
//!   extended, preemptable or not, activation limit, entry function, stack)
//!   and of the *dynamic* part (state, pending activations, events set /
//!   waited for, current priority).
//! * The state machine as code: which transitions exist, and who triggers
//!   them.
//!
//! # Specification
//!
//! * OSEK OS §4.2, Figures 4-1 and 4-2: the four states and the transitions
//!   *activate*, *start*, *wait*, *release*, *preempt*, *terminate*. Basic
//!   tasks have no *waiting* state.
//! * OSEK OS §4.2.3, §3.2: what differs between basic and extended tasks and
//!   between the conformance classes.
//! * OSEK OS §4.3: multiple activation requests.
//!
//! # Rust questions to think about
//!
//! * Static vs. dynamic data: the static part can live in a `const`/`static`
//!   table in flash, the dynamic part needs RAM and interior mutability. Do
//!   they belong in one struct?
//! * Can the type system rule out illegal transitions (e.g. `wait` on a basic
//!   task), or is a runtime check plus `Error::Access` the honest answer for a
//!   statically *configured* but dynamically *indexed* system? Try the
//!   typestate pattern on paper before deciding.
//! * `enum`s with data make "waiting for mask X" unrepresentable outside the
//!   waiting state. Is that worth the larger TCB?
//! * A task entry is a function pointer. What is its type — `fn()`,
//!   `fn() -> !`, `extern "C" fn()`? What does each choice mean for the
//!   context-switch code that has to fabricate the first stack frame?
