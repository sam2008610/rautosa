# Exercise 00 — A ring buffer, for someone who knows C but not Rust

The ready queue of the scheduler is, per priority, a small fixed-capacity
double-ended queue. This exercise builds exactly that one piece, on your Mac,
with no hardware, no interrupts and no `unsafe`. By the end you will have
written every Rust construct the scheduler's data structures need.

Do this before exercise 01.

## How to work

The file is `exercises/src/ex00_ring.rs`. It contains a `Ring` type whose
functions are all `todo!()`, and eleven tests that describe what they must do.

```sh
cargo test-host            # run all host tests
cargo test-host a03        # run only tests whose name contains "a03"
```

At the start all eleven fail. Make them pass **in order** (`a01`, `a02`, ...);
each one needs only a little more than the previous. Do not edit the tests.

Read compiler errors from the top; the first one is usually the only real
one. `rustc --explain E0502` (with the code from the message) gives a longer
explanation. Install the rust-analyzer extension in your editor — inline
types and errors make the first weeks far easier.

## Read first

*The Rust Programming Language* ("the Book"), free at
<https://doc.rust-lang.org/book/>. For this exercise you need only:

| Chapter | Why |
|---------|-----|
| 3 — Common Programming Concepts | variables, `mut`, integer types, functions, `if`, loops |
| 4.1, 4.2 — Ownership, References and Borrowing | what `&self` and `&mut self` mean |
| 5 — Structs, Method Syntax | `struct`, `impl`, `Self` |
| 6 — Enums and Pattern Matching | `Option`, `match`, `if let` |
| 9.2 — Recoverable Errors with `Result` | `Result`, `Ok`, `Err`, `?` |

Skip everything else for now, including strings, collections and lifetimes.

## From C to Rust, for this exercise

| In C you would write | In Rust |
|----------------------|---------|
| `struct ring { uint8_t buf[4]; size_t head, len; };` | `struct Ring { buf: [u8; 4], head: usize, len: usize }` |
| `void ring_init(struct ring *r)` | `fn new() -> Self` — a function that *returns* an initialised value; there is no uninitialised struct |
| `size_t ring_len(const struct ring *r)` | `fn len(&self) -> usize` |
| `int ring_push(struct ring *r, uint8_t v)` returning `0` or `-1` | `fn push_back(&mut self, value: u8) -> Result<(), Full>` |
| `int ring_pop(struct ring *r, uint8_t *out)` | `fn pop_front(&mut self) -> Option<u8>` — the value *is* the return value, "nothing" is `None` |
| `return x;` as the last statement | just `x` with no semicolon |
| `r->len++` | `self.len += 1` (there is no `++`) |
| a variable | immutable unless declared `let mut` |
| `size_t i = a - 1;` wrapping silently when `a == 0` | **panics** in debug builds; you have to say what you mean |
| reading `buf[7]` of a 4-element array | **panics**; never undefined behaviour |
| implicit integer conversions | none; `x as usize` when you need one |

The last three rows are where C habits produce the first surprises.

## Part A — make the tests pass

Fill in the eight functions of `Ring`. Capacity is the constant `CAPACITY`.

Acceptance:

- [ ] `cargo test-host` reports 11 passed.
- [ ] The line `#![allow(unused_variables, dead_code)]` at the top of the file
      is deleted and `cargo lint` is still clean.
- [ ] No `unsafe`, no `unwrap()` outside the tests.

## Part B — any capacity

A kernel needs queues of different sizes, known at compile time.

Change the type so that the capacity is a parameter of the type:
`Ring<4>` and `Ring<16>` are different types, each with its own array size.
Remove the `CAPACITY` constant. Adapt the existing tests and add one that
uses two rings of different capacity.

Keyword to look up: **const generics** (Book chapter 10.1 mentions them; the
Rust Reference has the details).

Question to answer in a comment: what is `size_of::<Ring<4>>()` and
`size_of::<Ring<16>>()`, and why? Write a test that checks your prediction.

## Part C — queue task IDs, not bytes

The ready queue stores task IDs. Define your own type for them in this file:

```rust
pub struct TaskId(u8);
```

and make the ring store `TaskId` instead of `u8`.

You will hit two compiler errors that each teach something:

1. The tests compare values with `assert_eq!`. What does a type need for
   that, and how do you get it without writing the code yourself?
2. `front()` hands out a copy of an element while the ring keeps its own.
   For `u8` that just worked. Why does it not for your struct, and what are
   the two ways to fix it?

Then a design question (answer in a comment, no code needed): the array must
be filled with *something* when the ring is empty. With `u8` you used `0`.
What are your options for `TaskId`, and which would you pick?

## Part D — the ready queue

Now build what the last days of discussion were about. In a new file
`exercises/src/ex00_ready.rs`, write a type that owns one ring per priority
plus a bitmap of non-empty priorities, with the four operations we listed:

| Operation | Meaning |
|-----------|---------|
| push to the back of a priority | a task was activated or released from waiting |
| push to the front of a priority | the running task was preempted |
| pop the highest | choose the next task to run |
| peek the highest priority | compare against the running task |

Write the tests yourself, first. Start from the scenarios we walked through:

- same priority runs in activation order;
- a preempted task runs before queued ones of its priority;
- T1 holds a ceiling-3 resource, is preempted by T5; T2 (priority 2) must
  wait until T1 has run again;
- the capacity counter-example: a full priority-3 queue plus one preempted
  "visitor".

Things you decide here: the type of a priority; what "pop" and "peek" return
when everything is empty; what a push into a full queue does; how the bitmap
finds the highest set bit (look at the methods of `u32`).

When part D is green, it moves into `rautosa-os/src/kernel/scheduler.rs` and
stage 1 has started.

## Hints

Open only what you need.

<details>
<summary>Hint 1 — the first test will not even get to its assertions</summary>

`todo!()` panics when executed, so every test fails at the first call. Start
with `new`: it must return a `Ring { ... }` with all three fields given a
value. An array of four zeros is written `[0; 4]`.
</details>

<details>
<summary>Hint 2 — returning an Option</summary>

`Some(value)` and `None` are the two possible values. An `if ... else ...` is
an expression in Rust and can be the last thing in a function.
</details>

<details>
<summary>Hint 3 — where is the back?</summary>

You store `head` and `len`, not `tail`. The slot after the last element is
`len` positions after `head`, modulo the capacity. Draw the array with
`head = 3, len = 2` and capacity 4.
</details>

<details>
<summary>Hint 4 — test a10 panics with "attempt to subtract with overflow"</summary>

Moving `head` one step back from index 0 is `0 - 1` on an unsigned integer.
C wraps silently; Rust refuses. Two ways out: do the arithmetic so that it
never goes below zero (add the capacity before subtracting), or look at the
`checked_sub` and `wrapping_sub` methods of `usize` and decide which one
states your intent.
</details>

<details>
<summary>Hint 5 — "cannot borrow as mutable"</summary>

In the tests the variable is declared `let mut ring`. In your own code, a
method that changes fields needs `&mut self`, one that only reads needs
`&self`. The signatures in the skeleton are already correct; if you see this
error you are probably calling a `&mut self` method on something you only
have a `&` to.
</details>

<details>
<summary>Hint 6 — the question mark</summary>

`let v = self.front()?;` means: if `front()` is `None`, return `None` from
this function right now; otherwise unwrap it into `v`. It works because the
function itself returns an `Option`. The same operator works with `Result`.
Not required for part A, but it shortens `pop_front` to three lines.
</details>

## What you have learned when this is done

| Part | Rust | Used in the kernel for |
|------|------|------------------------|
| A | structs, methods, `&self` vs `&mut self`, `Option`, `Result`, checked arithmetic, unit tests | every data structure |
| B | const generics | sizes that come from the static configuration |
| C | newtypes, `derive`, `Copy` vs. move | `TaskId`, `Priority`, `ResourceId` |
| D | composing types, bit operations, designing an API from scenarios | the scheduler |
