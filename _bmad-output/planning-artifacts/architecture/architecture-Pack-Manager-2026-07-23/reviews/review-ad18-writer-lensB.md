# Reviewer Gate — AD-18 writer identity / record cardinality — Lens B

**Intent:** Validate. **Angle:** invariant-enforceability and runtime consequence —
what a *compliant* implementation could actually do, and what the durable bytes
look like afterward. Report only; no file outside this one was modified.

**Claim under review:** "AD-18 covers the plan-attempt journal's home, format and
durability but names **no writer** and **no record cardinality**."

**Verdict: CONFIRMED.** Closing it is a **NEW invariant**, not an amendment to
AD-18's existing rules — with a hard enforceability rider stated in §5.

---

## 0. Absence proof: AD-18 in full

The claim is an absence claim, so here is every line of AD-18, quoted, so the
absence is visible rather than asserted. `sed -n '620,649p' ARCHITECTURE-SPINE.md
| grep -c '\*\*Rule:\*\*'` → **6**.

`ARCHITECTURE-SPINE.md:620` `"### AD-18 — [ADOPTED] Confirmed plan attempts have their own durable store"`

`ARCHITECTURE-SPINE.md:622` `"- **Binds:** UX-PB.2c, UX-PB.2d, UX-PB.2f, UX-PB.3d, UX-PB.4a, UX-PB.4b, UX-PB.4e; Story 6.5"`

`ARCHITECTURE-SPINE.md:623-625`
```
- **Prevents:** the writer, the History reader, and the diagnostics exporter each
  choosing a different home for attempt records — an extended `operations.jsonl`,
  a private sidecar file, or memory only
```

Rule 1 — `ARCHITECTURE-SPINE.md:626-629`
```
- **Rule:** Confirmed attempts persist to their own append-only NDJSON journal in
  the same Application Support directory as `operations.jsonl`, under the same
  discipline: an append failure is nonfatal to package operations, and compaction
  is temp file + fsync + rename, never truncate-in-place.
```

Rule 2 — `ARCHITECTURE-SPINE.md:630-632`
```
- **Rule:** `operations.jsonl` keeps its record shape and carries `planAttemptId`
  only where one exists. A record without one stays an individually labeled
  legacy Operation.
```

Rule 3 — `ARCHITECTURE-SPINE.md:633-635`
```
- **Rule:** Diagnostics export carries both journals as distinct entries
  alongside `report.json`, the newest three app logs, and the newest 25
  transcripts. Existing retention bounds are unchanged.
```

Rule 4 — `ARCHITECTURE-SPINE.md:636-641`
```
- **Rule:** Widening the export does not widen disclosure. Plan-attempt records
  enter the archive under the same allowlist the export already applies —
  inherited environment values are excluded, and a record carries the reviewed
  intent and the exact argv Pack-Manager constructed, never ambient environment
  or user paths beyond what the existing entries already disclose. A story that
  adds a field to the attempt record owns its disclosure review.
```

Rule 5 — `ARCHITECTURE-SPINE.md:642-646`
```
- **Rule:** The two journals share a retention policy. Compacting the Operation
  journal may not orphan an attempt whose Operations it drops, and compacting the
  attempt journal may not leave Operations pointing at an attempt that no longer
  resolves. A record that loses its counterpart reads as legacy, never as
  corrupt.
```

Rule 6 — `ARCHITECTURE-SPINE.md:647-649`
```
- **Rule:** Operation status, output and stall events, transcript metadata, and
  journal start/finish records carry `planAttemptId` when one exists, so
  correlation never depends on reconstructing it from timing or membership.
```

Subject of each rule: 1 = location + format + failure mode + compaction
mechanism; 2 = the *other* journal's shape; 3 = export entries; 4 = disclosure
allowlist; 5 = cross-journal retention; 6 = field presence. **Six rules about the
file. Zero about who may open it for append, and zero about how many lines one
attempt puts in it.**

Grep counts over the whole spine:

- `grep -c "cardinality"` → the word appears **only** at `:1051`, the OPEN row
  itself. Not inside AD-18.
- `grep -c "start/finish"` → **1**, at `:648` (Rule 6), and it names the *Operation*
  journal's records — see §2.
- `grep -n "single writer\|one writer\|sole writer\|owns the write"` → **only**
  `:1051`. AD-18's sole use of the word "writer" is `:623`, in **Prevents**, where
  it is a presupposed actor, not a designated one, and what it is prevented from
  choosing is `"a different home for attempt records"` — a location, not a record set.

The spine already books this itself. `ARCHITECTURE-SPINE.md:1037`:
`"| Plan-attempt file name and serde shape | **Deferred** | AD-18 fixes ownership, location, durability, and failure mode; the exact filename and field list belong to UX-PB.2c. |"`
— note that "ownership" there means *ownership of the home* (which journal owns
attempt records), consistent with `:623`; nothing in the six rules assigns
ownership of the append.

---

## 1. Two compliant implementations, one divergent replay

Both implementations below satisfy every one of the six rules literally: same
Application Support directory, append-only NDJSON, append failure nonfatal,
compaction by temp file + fsync + rename, `operations.jsonl` untouched, both
journals exported as distinct entries, allowlist honored, cross-journal
retention honored, `planAttemptId` on every correlated record.

First, note that AD-18's own format choice *forces* multiple records. Rule 1
says `"append-only NDJSON"` (`:626`). `epics.md:693` requires a record at mint:
`"**Then** the append-only record stores the reviewed Manager/Package scope, Manager self-update identities, exact command snapshot, version evidence, timestamps, and result/verification state as immutable plan-admission metadata"`.
`epics.md:925` requires a terminal outcome per attempt:
`"**Then** exactly one immutable History row is created for that `planAttemptId`"`.
And the attempt's own `state` field must read differently at those two moments —
`ARCHITECTURE-SPINE.md:482` `"  state: admitted | running | verifying | terminal"`.
Append-only means the mint-time line cannot be edited later. So **≥2 records per
attempt is mandatory, not optional** — and no AD-18 rule says how a reader collapses
them.

### Implementation A — transition log (delta records)

Writes one thin line per state transition, each carrying `planAttemptId` and the
new `state`, with payload only on the record that introduces it:

```
{"planAttemptId":"pa-7","state":"admitted","reviewedIntent":{…},"reviewedCommandSnapshot":[…],"at":"…"}
{"planAttemptId":"pa-7","state":"running","at":"…"}
{"planAttemptId":"pa-7","state":"verifying","at":"…"}
{"planAttemptId":"pa-7","state":"terminal","verificationResults":{…},"resultSummary":"10 of 12 verified · 2 failed","at":"…"}
```

Four lines. The attempt's truth is the **last** record plus the payload
accumulated from earlier ones. Reading it correctly requires a last-wins fold
keyed on `planAttemptId` and a rule for merging payload forward.

### Implementation B — snapshot log (self-contained records)

Writes the whole `PlanAttempt` from `ARCHITECTURE-SPINE.md:477-484` at each
transition — every line complete, mirroring the normative shape verbatim:

```
{"planAttemptId":"pa-7","state":"admitted","reviewedIntent":{…},"reviewedCommandSnapshot":[…],"operationIds":["op-a","op-b"],"verificationResults":null,"resultSummary":null}
{"planAttemptId":"pa-7","state":"running","reviewedIntent":{…},"reviewedCommandSnapshot":[…],"operationIds":["op-a","op-b"],"verificationResults":null,"resultSummary":null}
{"planAttemptId":"pa-7","state":"verifying",…}
{"planAttemptId":"pa-7","state":"terminal",…,"resultSummary":"10 of 12 verified · 2 failed"}
```

Four lines again — but each is a **complete attempt**, indistinguishable in shape
from a whole attempt record.

### What replay then shows

The divergence is not hypothetical arithmetic; it follows the shipping reader's
own fold precedent, which AD-18 Rule 1 invites by saying `"under the same discipline"`.
`src-tauri/src/journal.rs:131-134`:

```rust
            Ok(Line::Start(s)) => {
                index.insert(s.op_id.clone(), entries.len());
                entries.push((s, None));
            }
```

**That fold is not idempotent.** Every complete-looking record pushes a *new*
entry; only the finish half is deduplicated, and only against the most recent
index write. Trace two identically-keyed complete records: first → `index["A"]=0`,
`entries=[(A,None)]`; second → `index["A"]=1`, `entries=[(A,None),(A,None)]`. A
later terminal line attaches to index 1 only. `src-tauri/src/journal.rs:152-155`
then supplies the default for the unattached one:

```rust
            let (status, exit_code, finished_at) = match finish {
                Some(f) => (f.outcome, f.exit_code, Some(f.finished_at)),
                None => (OpStatus::Interrupted, None, None),
            };
```

So a reader built on the shipping precedent yields, for **Implementation B**,
four History entries for `pa-7`: three reading `Interrupted` and one reading
terminal. A reader built for **Implementation A** — which must fold last-wins,
because a `{"state":"running"}` delta record is not a viable standalone row —
yields one. Same six rules obeyed, same journal, two different Histories:

| | Records for `pa-7` | Replay under a paired/push fold (shipping precedent) | Replay under last-wins fold |
| --- | --- | --- | --- |
| Impl A | 4 delta lines | 1 terminal row + 3 rows with no payload (reads `Interrupted`) | 1 correct row |
| Impl B | 4 snapshot lines | **4 rows for one attempt** — 3 `Interrupted`, 1 terminal | 1 correct row |

Impl B under the shipping fold violates `epics.md:926` verbatim:
`"**And** no attempt ever yields more than one row or a per-Package or per-command row."`
And it violates it *while obeying every AD-18 rule*, which is the definition of
an unenforceable invariant.

### The second, worse consequence: a superseded record read as current

`ARCHITECTURE-SPINE.md:486-488`:
```
The active-attempt lookup, cancel command, History query, Activity replay, and
diagnostic export address `planAttemptId`. Operation detail continues to address
`opId` within that attempt.
```

Under Impl A, `{"planAttemptId":"pa-7","state":"running"}` is a durable assertion
that `pa-7` is running. Crash between `running` and `terminal`, relaunch, and the
active-attempt lookup — which AD-18 gives no fold rule — can resolve `pa-7` as
live. Then `ARCHITECTURE-SPINE.md:394`:
`"- **Rule:** Exactly one confirmed attempt may be active. A second confirmation"` /
`:395 "  fails closed with a typed already-active result."`
A stale `running` record read as current makes every subsequent confirmation fail
closed permanently. That is not a cosmetic replay defect; it wedges admission,
and it is reachable by an implementation that broke no rule.

Compaction makes a third case. Rule 5 (`:642-646`) constrains only *cross*-journal
orphaning — `"may not orphan an attempt whose Operations it drops"`,
`"may not leave Operations pointing at an attempt that no longer resolves"`.
Nothing constrains dropping part of **one attempt's own** record set. The shipping
compactor keeps newest-N by *operation*, per `src-tauri/src/journal.rs:175-177`:
`"/// Startup compaction: keep the newest `keep` operations (file order = age;"`.
An attempt-journal compactor written as "keep newest N *records*" is equally
compliant, and can retain `{"state":"terminal"}` while dropping
`{"state":"admitted"}` — leaving an attempt with an outcome and no reviewed
intent. `epics.md:936-939` governs only the opposite direction:
`"**Given** an attempt whose admission record is present but whose terminal record is absent, unparseable, or skipped by the read (AD-19)"`.
The terminal-present / admission-absent case is unaddressed on both sides.

---

## 2. Does Rule 6 amount to a cardinality rule? No.

`ARCHITECTURE-SPINE.md:647-649`:
```
- **Rule:** Operation status, output and stall events, transcript metadata, and
  journal start/finish records carry `planAttemptId` when one exists, so
  correlation never depends on reconstructing it from timing or membership.
```

**Judgment: it is a field-presence rule, not a record-count rule.** Its predicate
is `"carry `planAttemptId`"` — a statement about what a record contains, which is
orthogonal to how many records exist. `"when one exists"` makes even that presence
conditional (the legacy allowance, Rule 2 at `:631-632` and `ARCHITECTURE-SPINE.md:538-539`
`"- **Rule:** **Legacy honesty.** Journal and History records without a recorded / `planAttemptId` stay individually labeled legacy Operations."`).
A cardinality invariant cannot be conditional in that way.

**Does the attempt journal inherit a start/finish pair? No, and nothing says so.**
Every subject Rule 6 enumerates is Operation-side: `"Operation status"`,
`"output and stall events"`, `"transcript metadata"`, `"journal start/finish records"`.
The last names the shipping Operation journal's two concrete types —
`src-tauri/src/journal.rs:26` `"pub struct StartRecord {"` and
`src-tauri/src/journal.rs:43` `"pub struct FinishRecord {"` — described at
`docs/SPEC.md:455` as
`"one line at start `{opId, kind, executor, subject, commandLine, pgid, startedAt}`, one at finish `{opId, outcome, exitCode, finishedAt}`, flushed each write"`.
`grep -c "start/finish"` over the spine returns **1**. There is exactly one
mention of a start/finish pair in the entire spine and it is about the other file.

The asymmetry is the finding: **the one sentence in AD-18 that names a per-record
cardinality names it for `operations.jsonl` and is silent for the journal AD-18
exists to create.** A builder reading Rule 6 would be *entitled* to infer a
start/finish pair by analogy — which is precisely Implementation B's shape, the
one that replays four rows.

AD-18's prose does lean singular once, Rule 4 at `:641`:
`"adds a field to the attempt record owns its disclosure review."` — "the attempt
record", definite singular. But that is a disclosure-review assignment, and it
directly contradicts Rule 1's append-only format, which cannot store one mutable
record. A singular noun in a disclosure rule is not a cardinality invariant; it is
evidence the gap was never noticed.

---

## 3. Is the shipping `operations.jsonl` writer single-owner? Yes in code, no in contract.

**In code: single-owner, provably.** `grep -rn "record_start\|record_finish" src-tauri/src/`
returns 17 hits; **exactly two are production call sites**, both in `queue.rs`,
both on the same handle:

`src-tauri/src/queue.rs:1357` (inside `fn start`, declared at `queue.rs:1323`
`"    fn start(&mut self, pending: Pending, permit: OwnedSemaphorePermit) {"`):
```rust
        self.deps.journal.record_start(&StartRecord {
```

`src-tauri/src/queue.rs:1247` (inside `fn handle_finished`, declared at
`queue.rs:1201` `"    fn handle_finished("`):
```rust
        self.deps.journal.record_finish(&FinishRecord {
```

`grep -rn "\.journal" src-tauri/src/ | grep -v "journal_records\|journal_path\|journal::"`
returns those two lines and nothing else.

Both live on one owner, and that owner is a single-task actor.
`src-tauri/src/queue.rs:920-931`:
```rust
struct Sched {
    tx: mpsc::UnboundedSender<Msg>,
    deps: QueueDeps,
```
`src-tauri/src/queue.rs:941` `"    let mut sched = Sched {"` — owned **by value** inside
`scheduler_task`, driven by `src-tauri/src/queue.rs:952`
`"    while let Some(msg) = rx.recv().await {"`. One task, serialized messages, one
`&mut` path to the writer. That is a real single-writer discipline, and the state
graph funnels it deliberately: `src-tauri/src/state.rs:247`
`"        let journal = Arc::new(Journal::new(journal_path));"` and the handle's only
destination is `src-tauri/src/state.rs:299` `"            journal,"` inside
`Queue::new(QueueDeps { … })`. `AppState` itself has **no `journal` field** —
`src-tauri/src/state.rs:194-213` lists it, and what it gets instead is the read
snapshot at `src-tauri/src/state.rs:317`:
`"            journal_records: Arc::new(RwLock::new(loaded)),"`, documented at
`src-tauri/src/state.rs:201-203`:
```
    /// Records loaded from the journal at startup (previous sessions;
    /// start-without-finish already marked Interrupted).
    pub journal_records: Arc<RwLock<Vec<crate::ipc::OperationRecord>>>,
```
The exporter is likewise a reader-by-path, not a handle holder —
`src-tauri/src/diagnostics.rs:143-144`:
```rust
    if is_regular_file(journal_path) {
        add_file(&mut zip, "operations.jsonl", journal_path)?;
```
with the path resolved independently at `src-tauri/src/diagnostics.rs:161`
`"        &Settings::app_support_dir().join(\"operations.jsonl\"),"`.

**Does "the same discipline" transfer it? No — Rule 1 defines what it means, and
the definition excludes both properties.** `:626-629` reads
`"under the same discipline: an append failure is nonfatal to package operations, and compaction is temp file + fsync + rename, never truncate-in-place."`
The colon is an enumeration. Two items, both about the *file*'s failure behavior.
Neither is "one writer" and neither is "one start line plus one finish line" —
even though the shipping journal demonstrably has both, and the second is written
down elsewhere (`docs/SPEC.md:455`; `src-tauri/src/journal.rs:4`
`"//! One line at op start, one at finish, flushed each write. Start-without-"`).
The properties AD-18 *did* carry over are exactly the two it names; the two it
did not name are exactly the two the claim says are missing.

Three further reasons the shipping property will not transfer by itself:

1. **The single-writer property is nowhere stated as an invariant.** `grep -n
   "single writer\|one writer\|sole writer\|owns the write"` over the spine hits
   only `:1051`. It is an emergent consequence of where one `Arc` was placed, not
   a rule any story is bound by.
2. **The type actively permits a second writer.** `src-tauri/src/journal.rs:57-61`:
   ```rust
   /// Append-only, flushed-per-write journal handle.
   pub struct Journal {
       path: PathBuf,
       file: Mutex<Option<std::fs::File>>,
   }
   ```
   The `Mutex` buys thread-safety, not exclusivity — and the handle is held as
   `Arc<Journal>` (`src-tauri/src/queue.rs:687` `"    pub journal: Arc<Journal>,"`),
   so cloning it to a second component is one line that compiles and passes every
   existing test. `src-tauri/src/journal.rs:77` `"    fn append<T: Serialize>(&self, record: &T) {"`
   takes `&self`, not `&mut self`. Nothing fails closed.
3. **The reader survives only because the writer is single.** The
   non-idempotent fold at `journal.rs:131-134` (§1) has no defense against a
   duplicate record; its correctness is entirely a function of exactly one writer
   emitting exactly one start. Transferring the *reader* pattern to a journal
   without transferring the *writer* guarantee is the failure mode.

Also worth flagging for whoever writes the shape: the discriminator is
`src-tauri/src/journal.rs:50-55`:
```rust
#[derive(Deserialize)]
#[serde(untagged)]
enum Line {
    Start(StartRecord),
    Finish(FinishRecord),
}
```
`untagged` discriminates by field shape. It works today because `StartRecord` and
`FinishRecord` share only `opId`. A plan-attempt record set whose kinds overlap
(as Impl B's do — every record carries every field) cannot be discriminated this
way at all. That is `ARCHITECTURE-SPINE.md:1037`'s deferred serde question, but it
is *downstream of* cardinality: you cannot choose a serde shape before you know
how many kinds of record exist.

---

## 4. Does `state: admitted | running | verifying | terminal` imply one mutable record?

`ARCHITECTURE-SPINE.md:477-484`:
```
PlanAttempt
  planAttemptId: durable PlanAttemptId
  retryOfPlanAttemptId?: PlanAttemptId
  reviewedIntent + reviewedCommandSnapshot
  operationIds[]
  state: admitted | running | verifying | terminal
  verificationResults + resultSummary
```

`ARCHITECTURE-SPINE.md:482` `"  state: admitted | running | verifying | terminal"`

**It reads as one record with a mutable field, and append-only NDJSON cannot
store that.** The shape is written as a single aggregate with one `state` slot
holding one of four values — the ordinary reading of which is a row you update.
Rule 1 (`:626`) forbids updating: `"append-only NDJSON journal"`. So the shape and
the format contradict each other on their faces, and no rule resolves it.

That the attempt genuinely traverses all four is not inferable — it is stated.
`ARCHITECTURE-SPINE.md:401-403`:
```
- **Rule:** A mutating attempt is not successful until the required affected
  Manager refreshes complete. The attempt explicitly enters `Verifying`, and
  Results distinguish mutation failure from verification failure while preserving
```
and the transitions are enumerated in the state diagram —
`ARCHITECTURE-SPINE.md:496` `"    Preview --> Admitted: confirmed, planAttemptId minted, draft emptied atomically"`,
`:498` `"    Admitted --> Running: scheduler acquires the lock set"`,
`:499` `"    Running --> Verifying: processes exit, fresh post-exit refreshes required"`,
`:500` `"    Verifying --> Terminal: Results distinguish mutation vs verification failure"`.

And the intermediate states are required to be **durable**, not derived —
`ARCHITECTURE-SPINE.md:412-415`:
```
- **Rule:** `Verifying` and `Skipped` are durable wire-level operation states, not
  presentation states derived in React. They are journaled, exported in
  diagnostics, and replayed from History, so a derived state could not survive a
  crash or a replay. Adding them is one atomic contract change under AD-3.
```
`"They are journaled"` plus `"replayed from History"` plus append-only forces a
record whose `state` reads `verifying`. So:

**Conclusion: a record per transition is the only compliant reading, and it needs
a fold rule that does not exist.** Minimum three records (`admitted`, `verifying`,
`terminal`); four with `running`. The fold must answer, and AD-18 answers none of
these: (a) is the attempt's state the last record's `state`, or the highest state
reached? (b) does a later record's payload replace or merge with an earlier one's?
(c) `epics.md:698` requires that on a failed persist `"no partial attempt record is left behind"`
— under append-only, is a torn final line "left behind", and who removes it?
(d) AD-19 at `ARCHITECTURE-SPINE.md:660-661`
`"A journal must never be defaulted away: an unparseable line / is skipped and counted, the surrounding records stay readable"`
— a skipped `terminal` line silently downgrades a completed attempt, which
`epics.md:938` now forbids
(`"a terminal record that exists but failed to parse is reported as unreadable evidence rather than silently reclassifying a finished attempt as unfinished, and the fold states which it was"`)
but only for UX-PB.4a, and only for that one record kind.

---

## 5. Verdict and disposition

### CONFIRMED

AD-18's six Rules, quoted in full in §0, contain no assignment of write authority
to any component and no statement of how many records one attempt produces or how
a reader collapses them. The one rule that could be mistaken for a cardinality
rule (Rule 6) is a field-presence rule whose only mention of a start/finish pair
is scoped to the *other* journal (§2). The shipping journal really does have both
properties (§3), but Rule 1's `"under the same discipline:"` enumerates exactly
two inherited properties and neither is one of them. And AD-18's own format choice
plus AD-16's own state machine make ≥2 records per attempt mandatory (§4), so the
gap is not theoretical: it is load-bearing on the first line of code.

The two-implementation divergence in §1 is the enforceability proof — two builders
obeying all six rules produce durable bytes that a History replay reads as one
attempt versus four, and a crash-interrupted Impl A can wedge admission
permanently against `ARCHITECTURE-SPINE.md:394`.

**One correction to the OPEN row's stated symptom, which the reviewer gate should
record.** `ARCHITECTURE-SPINE.md:1051` says
`"UX-PB.3d and UX-PB.4a both own a terminal durable write"`. That is **no longer
true of the committed `epics.md`** — commit `5972109` ("Clear the epics.md
residual pile (#36)") landed the assignment. `epics.md:849`:
```
An attempt accumulates several append-only records — UX-PB.2c writes the admission record at mint — but exactly one **terminal** record exists per `planAttemptId` and UX-PB.4a writes it, folding the attempt's records into the single immutable History row. A second terminal write here would append a duplicate with no rule for which record is authoritative.
```
and `epics.md:827` says so explicitly, including that it belongs upstream:
```
AD-18 (the plan-attempt journal's home, format and durability — note AD-18 does not itself name a writer or a record cardinality, so the terminal-write ownership below is stated here and belongs in AD-18 when it is next amended); UX-PB.4a owns the single durable terminal write and this story never writes one
```
So the **story-level** double-write is closed; the **invariant-level** gap the
claim names is not. `epics.md` now testifies to the claim rather than
contradicting it. The OPEN row's symptom sentence is stale and should be
repointed at the invariant, not at the two stories.

### Why NEW invariant, not an amendment — and the enforceability rider

**The substance is new.** Every existing AD-18 rule predicates over *the file*:
where it lives, how it serializes, how it fails, what it exports, what it
discloses, what retention couples it to, what field its records carry. The missing
rule predicates over two different subjects: **who holds write authority** (a
process/ownership rule, the same species as AD-24's `"The one persistent draft has exactly one author"`
at `ARCHITECTURE-SPINE.md:801`) and **how records map to domain state** (a fold
rule over AD-16's normative minimum at `:477-484`). Neither is a reworded version
of any of the six. This matches the spine's own disposition at `:1051`:
`"it needs a new invariant, not a reworded one."`

**The rider — and this is the part a bare amendment would get wrong.** Whichever
id carries the rule, its `Binds` line must reach the writers, and AD-18's does
not. `grep -n "AD-18" epics.md` returns **7** hits. The three stories that
actually write or correlate durable attempt records are among those that **do not
cite AD-18 at all**:

- `epics.md:684` — UX-PB.2c, the story that owns the journal's filename and serde
  shape (`epics.md:316`) and writes the admission record (`epics.md:693`):
  `"**Dependencies:** UX-PB.2b; AD-16; D29; AD-27 (focus is a 2px `outline` in `--color-focus-ring`; …)"`
  — **no AD-18**, despite `ARCHITECTURE-SPINE.md:622` binding UX-PB.2c.
- `epics.md:711` — UX-PB.2d, the story that puts `planAttemptId` on durable records
  (`epics.md:725` `"**Then** crash-journal start/finish records, diagnostics, and verification refreshes carry the same `planAttemptId` where applicable"`):
  `"**Dependencies:** UX-PB.2b; AD-16; D29; AD-27 …"` — **no AD-18**, despite `:622`
  binding UX-PB.2d.
- `epics.md:731` — UX-PB.2e, plan cancellation, which drives an attempt to a
  terminal state (`epics.md:740`) and is therefore a **fourth** candidate terminal
  writer that `epics.md:849`'s sentence does not mention:
  `"**Dependencies:** UX-PB.2b, UX-PB.2d; AD-16; D30; AD-27 …"` — **no AD-18**.
  UX-PB.3g (`epics.md:911`, cancel during the verifying window) has the same
  exposure.

And where AD-18 *is* cited by a writer story it is cited bare —
`epics.md:916` `"**Dependencies:** D29; AD-16 (durable `planAttemptId` identity; atomic all-or-none admission); AD-18; UX-PB.3 complete (PB.3a-g); …"`
— no subject, contrary to the spine's own standing instruction recorded at
`ARCHITECTURE-SPINE.md:1049`:
`"per the spine's own standing instruction the citation is by `AD` id and subject, never by rule ordinal"`.

So: bolting a seventh Rule onto AD-18 would leave the admission writer, the
correlation writer, and both cancel paths unbound by it, and the invariant would
be as unenforceable after the amendment as before. **The close must be a new AD
whose `Binds` line names UX-PB.2c, UX-PB.2d, UX-PB.2e, UX-PB.3d, UX-PB.3g,
UX-PB.4a, UX-PB.4b, UX-PB.4e**, with a one-line cross-reference added to AD-18
Rule 1 pointing at it. The amendment to AD-18 is the pointer; the rule is new.

### What the new invariant must decide (enforceability checklist)

Each item below is one of the questions a compliant implementation currently
answers on its own:

1. **Write authority.** Exactly one Rust component appends to the plan-attempt
   journal; History, Activity replay, and diagnostics are readers. Stated the way
   AD-24 states the draft's, so it fails closed rather than depending on where an
   `Arc` happens to be placed (§3 shows the current property is placement, not
   type).
2. **Record kinds and count.** Which `state` values produce a record. Fixing
   `admitted` and `terminal` is not enough — `ARCHITECTURE-SPINE.md:412-415`
   requires `Verifying` to be journaled and replayable, so the rule must say
   whether the attempt-level `verifying` transition is a record or is derived from
   its Operations.
3. **The fold.** An attempt is a fold over its records keyed on `planAttemptId`,
   with a stated precedence (last-wins vs. highest-state-reached) and a stated
   payload-merge direction — resolving the `:482` single-`state` shape against
   Rule 1's append-only format.
4. **The missing/unreadable rules, both directions and both records.**
   `epics.md:936-939` covers absent-vs-unparseable for the *terminal* record under
   UX-PB.4a only. The invariant owes the same for a missing *admission* record
   (reachable via record-granular compaction, §1) and must state which way an
   attempt with a terminal record and no admission record reads.
5. **Compaction granularity.** Compaction retains or drops **whole attempts**,
   never individual records of one attempt — the intra-attempt analogue of Rule
   5's cross-journal guarantee at `:642-646`.
6. **Replay may not resurrect an active attempt.** A durable non-terminal record
   never satisfies the active-attempt lookup at `:486-488` after a relaunch,
   so a stale `running` record cannot permanently fail closed the
   one-active-attempt rule at `:394`.
