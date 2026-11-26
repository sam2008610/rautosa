# Writing requirements from OSEK OS

AUTOSAR numbers every requirement (`[SWS_Os_00285]`), so code and tests can
point at one. OSEK OS does not: it is prose, tables and figures. AUTOSAR then
pulls all of it in with a single sentence, `[SWS_Os_00001]`.

To trace the OSEK half of the kernel, Rautosa writes its own numbered
requirements from the OSEK text. The specification is the input; the list in
[`requirements/`](../requirements) is our work product.

The IDs are Rautosa's own. `OSEK_ActivateTask_04` means "our fourth
requirement about ActivateTask, derived from OSEK OS"; nobody outside this
repository knows that name.

## Where things are

| Path | What |
|------|------|
| `requirements/osek/document.toml` | Title, ID prefix and the section headings a requirement may cite |
| `requirements/osek/*.toml` | The requirements, one `[[requirement]]` table each |
| `tools/reqs.py` | Checks the files: `python3 tools/reqs.py` (add `--list` to print them) |
| `spec/sws-os-requirements.html` | The browser, which shows these next to the AUTOSAR ones (see the README) |

One file per topic:

| File | OSEK OS sections |
|------|------------------|
| `task.toml` | 4 Task management, 13.2 |
| `interrupt.toml` | 3.1, 6 Interrupt processing, 13.3 |
| `event.toml` | 7 Event mechanism, 13.5 |
| `resource.toml` | 8 Resource management, 13.4 |
| `alarm.toml` | 9 Alarms, 13.6 |
| `os.toml` | 3.2, 5, 11, 12, 13.1, 13.7, 13.8: conformance, application modes, start-up, shutdown, hooks, error handling |

Chapter 10 (messages) belongs to OSEK COM and chapter 14 says of itself that
it is not normative, so neither has requirements here. Statements about a
system where OSEK OS coexists with OSEKtime OS are left out as well.

## Terms

The requirements use these words with one meaning each.

| Term | Meaning |
|------|---------|
| Configured priority | The priority given to a task in the static configuration. It never changes. |
| Current priority | The priority a task has right now: its configured priority, or the ceiling priority it was raised to while it holds a resource. |
| Activation limit | The largest number of activations of one task that exist at the same time, counting the instance that is ready or running. |
| Oldest, newest | The order of the ready tasks of one priority. The oldest is the next to run. |
| Rescheduling | The scheduler makes its selection (highest current priority, then oldest) among the ready and running tasks, and switches tasks if the selected one is not running. |
| Task level, interrupt level | Code running in a task, or in an interrupt service routine. |
| In use | An alarm that has been set and has neither expired as a single alarm nor been cancelled. |

## What one requirement is

One sentence, in your own words, that promises **one thing you can observe**,
so that a single test can pass or fail it.

```toml
[[requirement]]
id = "OSEK_ActivateTask_04"
text = "If ActivateTask is called for a task that has already reached its configured activation limit, then ActivateTask shall return E_OS_LIMIT."
source = "13.2.3.1"
page = 50
where = "Status"
level = "standard"
```

| Key | Meaning |
|-----|---------|
| `id` | `OSEK_<Topic>_<NN>`. The topic is the service name, or a short word for a behaviour chapter (`Activation`, `Preemption`, `Ceiling`). |
| `text` | The requirement. |
| `source`, `page` | Section number and printed page it comes from. |
| `where` | Optional. The part of the section: `Description`, `Particularities`, `Status`, `Figure 4-5`, `Footnote 14`. |
| `level` | Optional, for error returns. `standard`: checked always. `extended`: checked only in extended status. |
| `kind` | Optional. `constraint` marks a duty of the application, see step 2. |
| `verify` | Optional. `test` (default), `review` or `analysis`. |
| `note` | Optional. Your interpretation where the text is unclear, or why you worded it this way. |
| `see` | Optional. Related requirement IDs, ours or `SWS_Os_...`. |
| `withdrawn` | Optional. The reason an entry is no longer valid. Keep the entry. |

## Sentence patterns

Pick the pattern from what triggers the behaviour. These are the EARS
patterns, a common way to keep requirements uniform.

| Situation | Pattern |
|-----------|---------|
| Always true | The OS shall *response*. |
| Something happens | **When** *trigger*, the OS shall *response*. |
| As long as a state lasts | **While** *state*, the OS shall *response*. |
| Misuse or error | **If** *condition*, **then** *service* shall return *status*. |
| Only in some configurations | **Where** *feature is configured*, the OS shall *response*. |

Always name who acts ("the OS", "ActivateTask") and use "shall" exactly once.

## How to split a section

### Step 1: label every sentence

Go through the section sentence by sentence and decide which of these it is.

| Label | How to recognise it |
|-------|---------------------|
| Behaviour | The OS does something you could observe from outside. |
| Error return | A line in the Status list, or "returns E_OS_…". |
| Duty of the caller | "The application must…", "shall only be called…". The OS is not the one acting. |
| Pointer | "Depends on…", "see chapter…". It promises nothing itself. |
| Explanation | Why it is designed this way, an example, advice to the user. |
| Freedom | "Implementation specific", "not specified". |

### Step 2: turn each label into the right thing

- **Behaviour** becomes one requirement per observable outcome.
- **Error return** becomes one requirement per status line, with `level`.
  `E_OK` gets one too.
- **Duty of the caller** is not an OS requirement. Write it with
  `kind = "constraint"` so that it is not lost: these are the rules a user of
  the OS has to follow. Then look for the error check that catches a caller
  who breaks the rule. That check is an OS requirement.
- **Pointer** becomes nothing here. Follow it, and write the requirement in
  the section it points to. Leave a comment so the next reader knows it was
  not forgotten.
- **Explanation** becomes nothing.
- **Freedom** becomes nothing in this list. It is a decision Rautosa has to
  make: record it in [`design-notes.md`](design-notes.md).

### Step 3: check that it is one requirement

Ask: *can a test fail this in exactly one way?* If a test could fail it in
two different ways, split it. The usual sign is "and" joining two outcomes.

"Return E_OS_LIMIT and ignore the activation" is two: the status code can be
right while the task state is wrong. They became `OSEK_ActivateTask_04` and
`OSEK_ActivateTask_05`.

"And" inside a condition is fine: "not suspended **and** below its limit"
describes one situation.

### Step 4: check the wording

- **Your own words.** Read the paragraph, look away, write what the OS must
  do. Do not copy sentences: specification text stays out of this repository.
- **What, not how.** "The OS shall start the higher-priority task" is a
  requirement. "The OS shall set a bit in the ready bitmap" is a design; it
  belongs in the code.
- **No soft words.** "Should", "may", "if necessary", "quickly",
  "correctly" cannot be tested. "May be called from an ISR" becomes "the OS
  shall accept calls from an ISR".
- **One name per thing.** Once you call it "activation limit", call it that
  everywhere.

`tools/reqs.py` warns about several "shall", soft words and very long
sentences. A warning is a hint, not a verdict: fix the sentence or decide it
is fine.

### Step 5: give it an ID, then leave the ID alone

Number within a topic in the order you write them. Never renumber and never
reuse a number, because code and tests cite it. If a requirement turns out to
be wrong, add `withdrawn = "why"` and write a new one with the next number.

## Worked example: ActivateTask (§13.2.3.1)

Open the section next to [`requirements/osek/task.toml`](../requirements/osek/task.toml).

| Part of the section | Label | Result |
|---------------------|-------|--------|
| Description, first sentence | Behaviour | `_01` suspended becomes ready |
| Description, second sentence | Behaviour | `_02` starts at the first statement |
| Footnote on the description | Behaviour | `_03` activation of a task that is not suspended is recorded |
| Particularities: allowed call levels | Behaviour | `_09` accepted from tasks and category 2 ISRs |
| Particularities: rescheduling depends on the caller | Pointer | nothing here; belongs to §4.6.1 and §4.6.2 |
| Particularities: effect of E_OS_LIMIT | Behaviour | `_05` nothing changes |
| Particularities: events of an extended task | Behaviour | `_06` events are cleared |
| Status, standard: no error | Error return | `_08` returns E_OK |
| Status, standard: too many activations | Error return | `_04` returns E_OS_LIMIT |
| Status, extended: invalid task | Error return | `_07` returns E_OS_ID |

Things worth noticing:

- The footnote carries a real requirement. Read footnotes and figure
  captions too.
- "May be called from interrupt level" alone is too vague to test. The table
  in §12.2.2 says which interrupt category is meant, so `_09` names it.
- The specification says "too many task activations". `_04` says exactly
  when that is, and its `note` records a consequence for extended tasks that
  is easy to miss.
- A rule that holds for every service, such as calling the error hook when a
  status other than E_OK is returned (§12.2.3), is written once in its own
  topic and not repeated for each service.

## Behaviour chapters (4 to 8)

Chapter 13 is half-structured already. Chapters 4 to 8 are prose and need
more reading. Some places to look:

- **State diagrams.** Each arrow is one requirement: from which state, on
  which trigger, to which state.
- **Lists.** A list of "rescheduling takes place when…" is one requirement
  per item.
- **Words like "always", "never", "only", "first", "oldest".** They usually
  hide an ordering rule. Ordering rules are what scheduler tests are made of.
- **Things the text does not say.** If you had to guess, write the guess in
  `note`, and if it is a real choice, in `design-notes.md`.

Where AUTOSAR changes an OSEK behaviour, keep the OSEK requirement, and put
the `SWS_Os_...` ID in `see`.

## Using a requirement

Cite the ID where the behaviour is implemented and where it is tested:

```rust
/// Implements: OSEK_ActivateTask_04
```

```rust
/// Verifies: OSEK_ActivateTask_04
#[test]
fn activate_task_beyond_limit_returns_e_os_limit() { /* ... */ }
```

The browser lists the places in the code that mention each ID, and can filter
for requirements nothing refers to yet. The aim is that every requirement has
at least one test.

## Reviewing a requirement

The list was written from the specification in one pass, and a requirement is
only as good as its last review. Before implementing a service:

1. Open its section of the specification next to its requirements.
2. Label the sentences as in step 1 and check that every behaviour and every
   status line has a requirement.
3. Read each requirement on its own. Could you write its test without
   opening the specification again? If not, the wording is not finished.
4. Pay attention to every `note` that starts with "Interpretation": it marks
   a place where the specification is silent or unclear and a reading was
   chosen. Decide whether you agree.

Fix what you find by the rules of step 5: correct a typo in place, but
withdraw and replace a requirement whose meaning changes.
