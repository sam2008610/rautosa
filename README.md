# Rautosa — an AUTOSAR Classic Platform OS written in Rust

Rautosa is a from-scratch implementation of the AUTOSAR Classic Platform
operating system (OSEK/VDX OS plus the AUTOSAR extensions) in `no_std` Rust.
It is a learning project: the goal is an idiomatic Rust design derived from
the public specifications, developed in the open and written up as a series
of articles.

**Status: skeleton.** The workspace builds and boots on QEMU, and the public
API of Scalability Class 1 is defined, but every OS service is still
`todo!()`. See [`docs/roadmap.md`](docs/roadmap.md).

## Quick start

Prerequisites (macOS):

```sh
brew install qemu
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh   # if rustup is missing
```

The Rust version, the `thumbv7m-none-eabi` target, `clippy` and `rustfmt` are
pinned in [`rust-toolchain.toml`](rust-toolchain.toml); `rustup` installs
them on first use.

```sh
cargo run --example hello
```

```text
Hello from Rautosa
```

This builds the example for a Cortex-M3, boots it in QEMU's `mps2-an385`
machine and exits with code 0. If QEMU ever hangs, quit with `Ctrl-A`, `X`.

## Everyday commands

| Command | What it does |
|---------|--------------|
| `cargo build` | Build the kernel crate for `thumbv7m-none-eabi` |
| `cargo run --example <name>` | Run an example on QEMU |
| `cargo lint` | Clippy on the kernel and all examples, warnings are errors |
| `cargo fmt --all` | Format |
| `cargo test -p rautosa-os --target "$(rustc -vV \| sed -n 's/^host: //p')"` | Run the kernel's unit tests on the host |
| `cargo doc -p rautosa-os --no-deps --open` | API documentation; search for a specification name such as `ActivateTask` |

## Layout

```text
rautosa/
├── rautosa-os/        OS kernel crate (no_std)
│   └── src/
│       ├── task.rs, resource.rs, event.rs,      idiomatic Rust API
│       │   counter.rs, alarm.rs, interrupt.rs,
│       │   os.rs, hooks.rs, error.rs
│       ├── compat.rs                            the same services under their
│       │                                        specification names
│       └── kernel/                              scheduler, task state machine,
│                                                context switch (empty)
├── examples/          programs that run on QEMU (hello, systick)
├── docs/              roadmap, API checklist, design notes, exercises
├── tools/             specification requirement extractor
└── spec/              specification PDFs and generated files (git-ignored)
```

## Platform

| | |
|---|---|
| Machine | QEMU `mps2-an385` (ARM MPS2 board, AN385 FPGA image) |
| CPU | Cortex-M3, ARMv7-M, 8 MPU regions, no FPU |
| Rust target | `thumbv7m-none-eabi` |
| Console | Semihosting (`cortex-m-semihosting`) |

The machine was chosen because it has an MPU (needed for Scalability Class 3),
no FPU (a simpler first context switch), and siblings in the same board family
for later stages: `mps2-an500` (Cortex-M7) and `mps2-an521` (dual Cortex-M33,
for multi-core).

## Specifications and sources

Rautosa is designed **only** from publicly available documents:

- OSEK/VDX Operating System Specification 2.2.3
- AUTOSAR Classic Platform, *Specification of Operating System* (SWS OS),
  release R25-11, a free download from <https://www.autosar.org>
- ARM architecture and device documentation

It is not derived from, and does not try to resemble, any existing OSEK or
AUTOSAR OS implementation. Source comments cite OSEK sections and
`[SWS_Os_nnnnn]` requirement IDs so that every behaviour can be traced to its
public source.

### Specification browser

`tools/sws_extract.py` turns the SWS OS PDF into a searchable offline page
listing every requirement with its section, upstream requirements,
cross-references, roadmap stage and the places in this repository that cite
it.

```sh
mkdir -p spec    # put AUTOSAR_CP_SWS_OS.pdf here
python3 -m venv tools/.venv && tools/.venv/bin/pip install pypdf
tools/.venv/bin/python tools/sws_extract.py spec/AUTOSAR_CP_SWS_OS.pdf
open spec/sws-os-requirements.html
```

The generated files contain specification text, which AUTOSAR allows to be
reproduced for informational purposes only. They are git-ignored on purpose;
do not publish them.

## Documentation

- [`docs/roadmap.md`](docs/roadmap.md) — the four stages and their definitions of done
- [`docs/sc1-api-checklist.md`](docs/sc1-api-checklist.md) — every SC1 service with its source
- [`docs/design-notes.md`](docs/design-notes.md) — open design questions and decisions made
- [`docs/exercise-01.md`](docs/exercise-01.md) — warm-up: SysTick and shared state
- [`docs/safety-practice.md`](docs/safety-practice.md) — proposal for practising ISO 26262-6 activities

## Disclaimer

This is an independent, educational project. It is not affiliated with or
endorsed by the AUTOSAR partnership, and it makes no claim of conformance to
any AUTOSAR release or to ISO 26262. AUTOSAR is a registered trademark of
AUTOSAR GbR. Do not use this software in a vehicle.

## License

**TODO: choose a license.** Until one is added, no license is granted.

Candidates to consider:

- `MIT OR Apache-2.0` — the Rust ecosystem default; permissive, with an
  explicit patent grant from the Apache side.
- `MPL-2.0` — file-level copyleft: changes to these files stay open, linking
  into proprietary firmware is allowed.
- `GPL-3.0-or-later` — strong copyleft.

After choosing, add the license file(s) and set `license = "..."` under
`[workspace.package]` in `Cargo.toml`.
