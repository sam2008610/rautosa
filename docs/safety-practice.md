# Practising ISO 26262 software development on Rautosa — proposal

Status: **proposal, not adopted.** Nothing here is set up yet except the
requirement extractor. Pick what you want to practise and delete the rest.

## What this is and is not

Rautosa is a learning project and a portfolio piece. It can demonstrate that
you know how ISO 26262 part 6 work products are produced and linked. It cannot
be "ISO 26262 compliant": compliance belongs to a safety lifecycle with a
safety plan, confirmation measures performed with independence, and an item
(or assumed item) with a hazard analysis behind it. Say "developed following
selected ISO 26262-6 practices", never "certified" or "compliant".

Two sourcing rules, same spirit as the rest of the project:

- ISO 26262 is a paid, copyrighted standard. Reference clauses by number and
  describe them in your own words; do not paste its text or tables into the
  repository. Clause numbers below refer to the 2018 edition and were written
  from memory — check them against your own copy before publishing.
- Templates, checklists and work products from previous employers stay there.
  Write every document from a blank page.

## Recommended framing: a Safety Element out of Context

An OS has no item and no hazards of its own, so it is developed as an SEooC
(ISO 26262-10, clause 9): you *assume* the requirements an integrator would
place on it and document those assumptions.

For an OS the assumed safety requirements are essentially **freedom from
interference** between software partitions of different ASIL (ISO 26262-6,
Annex D), in three classes:

| Interference class | AUTOSAR OS mechanism | Roadmap stage |
|--------------------|----------------------|---------------|
| Memory | Memory protection, OS-Applications, stack monitoring | 2 (stack), 3 |
| Timing and execution | Priority ceiling (bounded blocking), service protection, timing protection (SC2/SC4, not on the roadmap) | 1, 3 |
| Exchange of information | IOC, spinlocks | 4 |

That table is the bridge between the two specifications: AUTOSAR says *what*
the mechanisms are, ISO 26262 says *why* an integrator needs them and what
evidence makes them trustworthy.

## The practice loop

Do not try to cover the whole standard. Take one thin slice through the
V-model per roadmap stage, and automate the links so the evidence cannot rot.

### 1. Requirements (part 6 clause 6, part 8 clause 6)

- The extracted SWS OS requirements (`tools/sws_extract.py`) are your
  *given* functional requirements, each with a stable ID.
- Add a small set of **assumed software safety requirements** of your own,
  with your own IDs (e.g. `RAU_SSR_001`), each stating: the requirement, the
  assumed ASIL, the interference class, the SWS requirements that implement
  it, and the assumption about the integrator.
- Practise the requirement attributes of part 8 clause 6: unambiguous,
  atomic, verifiable, uniquely identified, with status.

### 2. Traceability, automated

This is the highest-value practice and the cheapest to start.

- Tag code and tests with requirement IDs in a fixed, greppable form, for
  example a doc line `/// Implements: SWS_Os_00399` and, on tests,
  `/// Verifies: SWS_Os_00285`.
- Extend the extractor into a trace tool: for every in-scope requirement,
  report *implemented by* and *verified by*; fail CI if an in-scope
  requirement has no test or a tag refers to an unknown ID.
- The requirement browser already shows "referenced in Rust code"; the step
  from "mentioned" to "implemented / verified" is the exercise.

### 3. Architectural design and safety analysis (part 6 clause 7, Annex E)

- One document with a static view (components, interfaces, which are allowed
  to contain `unsafe`) and a dynamic view (context switch, ISR entry/exit,
  the system-call path from stage 3).
- A software FMEA at component level: for each kernel component, failure
  mode → effect → detection or mitigation. A corrupted ready queue, a lost
  activation, a wrong ceiling priority and a stack overflow are enough to
  make it real.
- From stage 4: a dependent-failure analysis for state shared across cores.

### 4. Unit design and implementation (part 6 clause 8)

The standard asks for a coding guideline covering topics such as low
complexity, a language subset, strong typing and defensive implementation
(part 6, clause 5 and its table of guideline topics). For Rust that means
writing your own, because MISRA does not apply:

- A lint policy in `Cargo.toml` (`[workspace.lints]`): start from clippy's
  defaults and opt in to the restriction lints that match guideline topics —
  `undocumented_unsafe_blocks`, `arithmetic_side_effects`,
  `indexing_slicing`, `unwrap_used`, `expect_used`, `panic`. Justify every
  `#[allow]` in a comment.
- An `unsafe` policy: where it may appear, what a `// SAFETY:` comment must
  contain, who (which test or analysis) checks the claim.
- A panic policy: a kernel that panics has failed. Decide which panics are
  acceptable (none in service paths?) and how you would show their absence.
- Integer overflow: `overflow-checks = true` in release, or explicit
  `checked_*` / `wrapping_*` everywhere arithmetic can overflow.
- Follow the public work on safety-critical Rust: the Ferrocene Language
  Specification, the Ferrocene qualified toolchain, and the Rust Foundation's
  Safety-Critical Rust Consortium coding guidelines.

### 5. Unit verification (part 6 clause 9)

- Requirements-based tests, plus the derivation methods the standard lists:
  equivalence classes, boundary values (`maxallowedvalue`, counter wrap,
  activation limit), error guessing.
- Structural coverage as a *measure of test completeness*, not a goal:
  statement, branch, and MC/DC for the highest ASILs. `cargo-llvm-cov` gives
  line and region coverage on stable for the host tests; branch and MC/DC
  coverage are still nightly-only in rustc — measuring what the tooling can
  and cannot yet show is itself a good article.
- Static analysis beyond lints: Miri on the host tests to check `unsafe`
  code for undefined behaviour; a model checker such as Kani to prove small
  invariants (the ready queue never returns a lower priority than one it
  holds; ceiling arithmetic never overflows).

### 6. Integration and embedded testing (part 6 clauses 10 and 11)

- The QEMU scenario tests from the roadmap, run in CI, each tagged with the
  requirements it verifies.
- **Fault injection**: deliberately violate the assumptions and show the
  safety mechanism reacts — a wild write (stage 3), a stack overflow
  (stage 2), a task that never terminates, a missing `TerminateTask`.
- Resource usage evidence: worst-case stack depth per task, interrupt-masked
  time of each kernel path.

### 7. Supporting processes (part 8)

- Configuration and change management: already git. Add a pull-request
  template with an impact analysis section, and use it even when working
  alone — the habit is the practice.
- Confidence in the use of software tools (part 8 clause 11): write a short
  tool evaluation for `rustc`, `clippy`, QEMU and your own trace tool —
  can a tool malfunction introduce or mask an error (tool impact), and
  would you notice (tool error detection)? QEMU is the interesting one:
  what do tests on an emulator *not* prove?
- A **safety manual**: the assumptions of use an integrator must satisfy
  (configuration constraints, which hooks must be provided, what the OS does
  not detect). This is the SEooC's main deliverable and a natural README
  section.

## Suggested increments per stage

| Stage | Add this | Evidence produced |
|-------|----------|-------------------|
| 1 | Lint and `unsafe` policy; requirement tags; trace report in CI; host-test coverage | Coding guideline, trace matrix, coverage report |
| 2 | Compile-time configuration checks as "error avoidance"; stack-overflow fault injection | Verification report for one safety mechanism |
| 3 | Assumed safety requirements for memory FFI; architecture document; software FMEA; fault-injection suite; safety manual draft | The complete slice: requirement → design → analysis → code → test |
| 4 | Dependent-failure analysis; lock-order argument; tool confidence evaluation | Analysis report |

Stage 3 is where this pays off: memory protection *is* a safety mechanism, so
every work product has real content there. Stage 1 is where the habits
(tags, lints, tests per requirement) are cheapest to form.

## Smallest useful start

1. Decide the tag syntax and tag the stage 1 services as you implement them.
2. Add `[workspace.lints]` with a first, short restriction set.
3. Make "every implemented requirement has a test that names it" a CI check.

Everything else can wait until there is a kernel to analyse.
