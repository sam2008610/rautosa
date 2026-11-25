//! Rautosa — an AUTOSAR Classic Platform OS written in Rust.
//!
//! This crate is the OS kernel. It is being built in stages (see
//! `docs/roadmap.md`); at the moment it only contains the **API shell** of
//! Scalability Class 1, i.e. the OSEK/VDX OS service set plus the AUTOSAR
//! counter services. Every service body is still `todo!()`.
//!
//! # Two API layers
//!
//! * The modules [`task`], [`resource`], [`event`], [`counter`], [`alarm`],
//!   [`interrupt`] and [`os`] are the idiomatic Rust API: snake_case names,
//!   newtype IDs, `Result<T, Error>` instead of status codes, return values
//!   instead of out-parameters.
//! * [`compat`] is a thin wrapper that exposes the same services under their
//!   specification names and signatures (`ActivateTask`, `StatusType`, ...).
//!
//! Every idiomatic service carries a `#[doc(alias = "...")]` with its
//! specification name, so searching rustdoc for `ActivateTask` finds
//! [`task::activate_task`].
//!
//! # Specifications
//!
//! The design is derived only from public documents:
//!
//! * **OSEK OS** — OSEK/VDX Operating System Specification 2.2.3 (2005).
//!   Cited as "OSEK OS §x.y".
//! * **SWS OS** — AUTOSAR CP "Specification of Operating System", release
//!   R25-11 (document ID 34). Cited by requirement ID, e.g. `[SWS_Os_00399]`.
//!   The IDs are stable across releases, section numbers are not.

#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]
// The service stubs do not use their parameters yet.
// TODO: delete this line once the stubs have real bodies.
#![allow(unused_variables)]

pub mod alarm;
pub mod compat;
pub mod counter;
pub mod error;
pub mod event;
pub mod hooks;
pub mod interrupt;
pub mod os;
pub mod resource;
pub mod task;

mod kernel;

pub use error::{Error, Result};
