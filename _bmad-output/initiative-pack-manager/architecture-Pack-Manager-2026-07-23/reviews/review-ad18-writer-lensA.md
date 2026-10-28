# Reviewer Gate — Lens A (story-contract / builder's-eye)

**Question:** Does AD-18 name a writer and a record cardinality, and does `epics.md`
UX-PB.3d currently carry the single-terminal-writer rule in its own text with a note
that it belongs in AD-18?

**Intent:** Validate. Report only. No file edited except this one.

**Files Read this session:**
`_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/ARCHITECTURE-SPINE.md`
(lines 1-60, 160-189, 600-680, 795-816, 945-960, 1035-1053),
`_bmad-output/planning-artifacts/epics.md` (lines 681-712, 822-853, 911-942),
`_bmad-output/planning-artifacts/architecture/architecture-Pack-Manager-2026-07-23/reviews/review-divergence-v9.md`
(lines 315-359).

**VERDICT: CONFIRMED.**

---

## 0. AD-18's exact extent, established by command

`awk 'NR>=615 && NR<=655 && /^### AD-/{print NR": "$0}' ARCHITECTURE-SPINE.md`:

```
620: ### AD-18 — [ADOPTED] Confirmed plan attempts have their own durable store
651: ### AD-19 — [ADOPTED] Persisted schemas tolerate their own history
```

So AD-18 is lines 620-649 inclusive (650 blank). Rule count,
`sed -n '620,650p' ARCHITECTURE-SPINE.md | grep -c '^- \*\*Rule:\*\*'` → **6**.
All six are quoted in full in §1 below, so absence is proven by exhibition, not
by assertion.

---

## 1. The closest AD-18 text to naming a writer — and the whole AD, quoted

`sed -n '620,650p' ARCHITECTURE-SPINE.md | grep -c 'writer'` → **1**. The single
occurrence is not in a Rule. It is in the `Prevents` line:

> `ARCHITECTURE-SPINE.md:623-625` `"- **Prevents:** the writer, the History reader, and the diagnostics exporter each\n  choosing a different home for attempt records — an extended `operations.jsonl`,\n  a private sidecar file, or memory only"`

### Judgment on the `Prevents` line: *home* only.

Three grounds, all textual.

1. **Grammatical object.** The three subjects — "the writer, the History reader,
   and the diagnostics exporter" — govern one participle, "choosing", whose object
   is "**a different home** for attempt records". The line's enumerated failure
   modes are all homes: "an extended `operations.jsonl`, a private sidecar file, or
   memory only". Not one of them is a writer-identity failure. Every listed harm is
   cured by naming a location, which the first Rule does.
2. **Position in the AD's own grammar.** The spine's ADs separate `Prevents`
   (motivation) from `Rule` (normative content) — the shape is uniform from AD-1
   onward, e.g. `ARCHITECTURE-SPINE.md:164-170`
   `"- **Binds:** all"` / `"- **Prevents:** a test harness, CI lane, or release tool becoming load-bearing"` /
   `"- **Rule:** Product code never imports, branches on, or requires test"`.
   A builder is bound by the Rules. "the writer" appears only where nothing is
   mandated.
3. **Presupposition is not prescription.** "**the** writer" — definite article,
   singular — *presupposes* one writer exists. It never says there must be exactly
   one, never says which layer or which story it is, and never forbids a second.
   A presupposition in a motivation clause is unimplementable and untestable.

### The six Rules, in full — none names a writer

> `ARCHITECTURE-SPINE.md:626-629` `"- **Rule:** Confirmed attempts persist to their own append-only NDJSON journal in\n  the same Application Support directory as `operations.jsonl`, under the same\n  discipline: an append failure is nonfatal to package operations, and compaction\n  is temp file + fsync + rename, never truncate-in-place."`

> `ARCHITECTURE-SPINE.md:630-632` `"- **Rule:** `operations.jsonl` keeps its record shape and carries `planAttemptId`\n  only where one exists. A record without one stays an individually labeled\n  legacy Operation."`

> `ARCHITECTURE-SPINE.md:633-635` `"- **Rule:** Diagnostics export carries both journals as distinct entries\n  alongside `report.json`, the newest three app logs, and the newest 25\n  transcripts. Existing retention bounds are unchanged."`

> `ARCHITECTURE-SPINE.md:636-641` `"- **Rule:** Widening the export does not widen disclosure. Plan-attempt records\n  enter the archive under the same allowlist the export already applies —\n  inherited environment values are excluded, and a record carries the reviewed\n  intent and the exact argv Pack-Manager constructed, never ambient environment\n  or user paths beyond what the existing entries already disclose. A story that\n  adds a field to the attempt record owns its disclosure review."`

> `ARCHITECTURE-SPINE.md:642-646` `"- **Rule:** The two journals share a retention policy. Compacting the Operation\n  journal may not orphan an attempt whose Operations it drops, and compacting the\n  attempt journal may not leave Operations pointing at an attempt that no longer\n  resolves. A record that loses its counterpart reads as legacy, never as\n  corrupt."`

> `ARCHITECTURE-SPINE.md:647-649` `"- **Rule:** Operation status, output and stall events, transcript metadata, and\n  journal start/finish records carry `planAttemptId` when one exists, so\n  correlation never depends on reconstructing it from timing or membership."`

Rule 1 fixes home + format + durability + compaction. Rule 2 fixes the *other*
journal's shape. Rule 3 fixes export membership. Rule 4 fixes disclosure. Rule 5
fixes cross-journal retention. Rule 6 fixes correlation keys. **Zero** of the six
contains a verb whose subject is a component that appends. The nearest thing to an
assignment of ownership anywhere in AD-18 is
`ARCHITECTURE-SPINE.md:640-641` `"A story that\n  adds a field to the attempt record owns its disclosure review."` — ownership of a
*disclosure review*, not of a write.

The two projections of AD-18 elsewhere in the spine repeat the same silence:

> `ARCHITECTURE-SPINE.md:956` `"| Persistence | Application Support holds `settings.json` (atomic replace) plus append-only NDJSON journals compacted by temp file + fsync + rename. Unknown and retired fields are tolerated on read (AD-18, AD-19). |"`

> `ARCHITECTURE-SPINE.md:998` `"  <plan-attempts>.jsonl  # confirmed attempts; own append-only journal (AD-18)"`

**The spine demonstrably knows how to write a single-writer invariant** — it did so
for a different concern in the same revision:

> `ARCHITECTURE-SPINE.md:801-805` `"- **Rule:** The one persistent draft has exactly one author: a user staging or\n  removal action resolved through the Rust canonical rebuild. Admission of the\n  draft's own preview empties it as custody transfer (AD-17) and a canonical\n  rebuild may narrow it (AD-16); no other path adds, replaces, or clears\n  membership."`

AD-24 is the control case. "exactly one author … no other path adds, replaces, or
clears" is exactly the shape AD-18 lacks. Its heading even carries it:
`ARCHITECTURE-SPINE.md:795` `"### AD-24 — The draft has exactly one author; a derived intent never routes through it"`.
The absence in AD-18 is therefore a gap in this spine's own idiom, not a stylistic
choice it applies uniformly.

---

## 2. The single-terminal-writer rule, and the note — both live in UX-PB.3d

The rule, in a story's acceptance-criterion prose:

> `epics.md:849` `"**Then** the failure to persist is surfaced honestly, the visible Results are not presented as durably recorded, and no fabricated success is shown. This story renders and announces Results; it never writes a durable record itself. An attempt accumulates several append-only records — UX-PB.2c writes the admission record at mint — but exactly one **terminal** record exists per `planAttemptId` and UX-PB.4a writes it, folding the attempt's records into the single immutable History row. A second terminal write here would append a duplicate with no rule for which record is authoritative."`

Its `Given`/`When` frame:

> `epics.md:847-848` `"**Given** an attempt reaching terminal state (Results persistence failure)\n**When** the single durable terminal write owned by UX-PB.4a fails"`

The note that it belongs in AD-18, embedded inside a Dependencies line:

> `epics.md:827` `"**Dependencies:** UX-PB.3c; D29-D30; AD-16 (verification-gated success; post-exit fresh acquisition); AD-18 (the plan-attempt journal's home, format and durability — note AD-18 does not itself name a writer or a record cardinality, so the terminal-write ownership below is stated here and belongs in AD-18 when it is next amended); UX-PB.4a owns the single durable terminal write and this story never writes one; AD-25 (a failed verification refresh leaves the Last-good Snapshot in place); AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)"`

`epics.md` itself concedes the claim in its own words: "**note AD-18 does not itself
name a writer or a record cardinality**". Lens A's job is to check whether that
concession is true rather than to take it, and §1 and §3 establish it independently
by exhibiting all six Rules.

### Uniqueness, by count

```
grep -c 'exactly one \*\*terminal\*\* record' epics.md   → 1   (line 849)
grep -c 'UX-PB.4a writes it'                 epics.md   → 1   (line 849)
grep -c 'never writes one'                   epics.md   → 1   (line 827)
grep -c 'belongs in AD-18'                   epics.md   → 1   (line 827)
```

The single-terminal-writer rule exists in exactly **one** place in the entire
`epics.md`, and that place is inside UX-PB.3d.

### It was placed there instead of in AD-18, in the current HEAD commit

`git blame -L 848,849 --porcelain _bmad-output/planning-artifacts/epics.md`:

```
5972109f46efe730d8b02c69cff3273aa80adfaa 848 848 2
summary Clear the epics.md residual pile (#36)
```

`git blame -L 620,649 --line-porcelain … ARCHITECTURE-SPINE.md | grep "^summary" | sort -u`:

```
summary Reconcile the planning spine with reality, and fix what that surfaced (#35)
```

AD-18's block dates to `c8c1f9a` (#35). UX-PB.3d's rule dates to `5972109` (#36,
HEAD). The rule was written **after** the AD's last touch and did not go into the
AD. The spine's own ledger says so too, and marks it out of scope:

> `ARCHITECTURE-SPINE.md:1051` `"| Plan-attempt journal: writer identity and record cardinality | **OPEN — new architecture, not this run's scope** | Surfaced by `reviews/review-divergence-v9.md` C-6. AD-18 fixes the journal's *home*, format, and durability discipline but names no single writer and no record cardinality per attempt. UX-PB.3d and UX-PB.4a both own a terminal durable write, and append-only NDJSON guarantees at least two records for one attempt with no stated fold rule, so History could replay an attempt twice or replay a superseded record as current. Same disposition as the row above: it needs a new invariant, not a reworded one. |"`

---

## 3. Record cardinality: AD-18 fixes none of it

`sed -n '620,650p' ARCHITECTURE-SPINE.md | grep -cEi 'cardinal|exactly one|fold|supersede|duplicate|at most one|one per'` → **0**.

The rule that makes cardinality unavoidable is Rule 1, which mandates append-only
and thereby forbids updating a record in place:

> `ARCHITECTURE-SPINE.md:626-627` `"- **Rule:** Confirmed attempts persist to their own append-only NDJSON journal in\n  the same Application Support directory as `operations.jsonl`, under the same"`

Append-only + a durable admission at mint + a durable terminal outcome = at least
two records per attempt, mechanically. AD-18 then states no fold rule, no
supersede rule, and no authority ordering. Worse, its own prose reads as if there
were one record per attempt — singular, twice:

> `ARCHITECTURE-SPINE.md:640-641` `"A story that\n  adds a field to the attempt record owns its disclosure review."`

> `ARCHITECTURE-SPINE.md:645-646` `"A record that loses its counterpart reads as legacy, never as\n  corrupt."`

Line 645-646 is the only place AD-18 discusses a record's relationship to another
record, and it is strictly *cross-journal* — the counterpart is an `operations.jsonl`
record, per the sentence it continues: `ARCHITECTURE-SPINE.md:642-645` `"The two journals share a retention policy. Compacting the Operation\n  journal may not orphan an attempt whose Operations it drops, and compacting the\n  attempt journal may not leave Operations pointing at an attempt that no longer\n  resolves."`
Nothing in AD-18 addresses two records *within* the attempt journal bearing the same
`planAttemptId`. Rule 6 hands out the correlation key —
`ARCHITECTURE-SPINE.md:647-649` `"Operation status, output and stall events, transcript metadata, and\n  journal start/finish records carry `planAttemptId` when one exists"` — and even
names "journal start/finish records" in the plural, which is the AD's one
acknowledgement that an attempt produces more than one record. It draws no
consequence from it.

Every cardinality statement that exists lives in story text:

> `epics.md:849` (quoted in full in §2) `"…exactly one **terminal** record exists per `planAttemptId` and UX-PB.4a writes it, folding the attempt's records into the single immutable History row."`

> `epics.md:925-926` `"**Then** exactly one immutable History row is created for that `planAttemptId`, its Operation-level evidence is nested inside that row, and its summary uses verified-outcome wording such as `10 of 12 verified · 2 failed` rather than a generic completion ratio\n**And** no attempt ever yields more than one row or a per-Package or per-command row."`

Note what UX-PB.4a actually constrains: **rows**, not records. "no attempt ever
yields more than one **row**" is a History-surface guarantee. The record-level
guarantee — one terminal *record* — appears only at `epics.md:849`, in the other
story.

---

## 4. Verdict

**CONFIRMED**, on all three limbs, with one refinement worth recording.

1. *AD-18 covers home, format, durability.* True. `ARCHITECTURE-SPINE.md:626-629`
   fixes directory, NDJSON, append-only, nonfatal append failure, and
   temp+fsync+rename compaction.
2. *AD-18 names no writer.* True and proven by exhaustion: all six Rules are quoted
   in §1; `grep -c 'writer'` over AD-18 → 1, and that hit is in the `Prevents`
   clause at `ARCHITECTURE-SPINE.md:623-625`, whose object is "a different **home**
   for attempt records". The definite singular "the writer" presupposes one writer
   without identifying it, mandating it, or forbidding a second.
3. *AD-18 names no record cardinality.* True and proven by count: zero
   cardinality/fold/supersede vocabulary in lines 620-650, while
   `ARCHITECTURE-SPINE.md:626` mandates "append-only", which guarantees ≥2 records
   per attempt.
4. *UX-PB.3d carries the rule with a note that it belongs in AD-18.* True and
   unique: `epics.md:849` carries "exactly one **terminal** record exists per
   `planAttemptId` and UX-PB.4a writes it", `epics.md:827` carries "belongs in AD-18
   when it is next amended", each `grep -c` → 1.

**Refinement (a narrowing, not a hedge):** the same HEAD commit that wrote the rule
into UX-PB.3d also gave UX-PB.4a a *read-side* fold criterion, so the divergence
`review-divergence-v9.md` C-6 described is now narrower than C-6's text.
`git blame -L 913,939 epics.md` attributes lines 916 and 936-939 to `5972109f`.
That criterion reads:

> `epics.md:936-939` `"**Given** an attempt whose admission record is present but whose terminal record is absent, unparseable, or skipped by the read (AD-19)\n**When** History folds that `planAttemptId`'s records into its row\n**Then** the row is presented as `Interrupted` **only when the absence is genuine** — a terminal record that exists but failed to parse is reported as unreadable evidence rather than silently reclassifying a finished attempt as unfinished, and the fold states which it was.\n**And** the direction holds both ways: a missing terminal record never fabricates a completed outcome, and an unreadable one never erases a completed attempt."`

So UX-PB.4a's builder *does* learn from her own story that records are plural, that
one of them is terminal, and that the row is a fold — the C-6 replay-twice scenario
is partly closed on the read side. What she does **not** learn from her own story or
from AD-18 is the *write-side exclusivity*: that she is the only writer of a terminal
record and that UX-PB.3d will not write one. That is why the verdict stays CONFIRMED
rather than partial: the claim is about AD-18's silence and about where the rule
lives, and both hold exactly. The refinement changes the blast radius, not the
finding.

---

## 5. Architectural consequence: is a rule in one story's text enforceable on a second story?

**No.** Three independent reasons, each with a citation.

### (a) UX-PB.4a's builder is never pointed at UX-PB.3d's text

> `epics.md:916` `"**Dependencies:** D29; AD-16 (durable `planAttemptId` identity; atomic all-or-none admission); AD-18; UX-PB.3 complete (PB.3a-g); AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)"`

Contrast the AD-18 citation here — **bare, no parenthetical** — with UX-PB.3d's
citation of the same AD at `epics.md:827`, which carries a 40-word gloss. The
builder of UX-PB.4a follows `AD-18` to `ARCHITECTURE-SPINE.md:620-649` and finds six
Rules, none of which mentions a writer. Her only pointer toward UX-PB.3d is
"UX-PB.3 complete (PB.3a-g)" — a *sequencing* dependency on seven stories being
finished, not a notice that one of them defines a rule binding her implementation.
Nothing on that line distinguishes "3d must ship first" from "3d contains the rule
you must obey". A rule reachable only by reading a sibling story's fourth
acceptance criterion in full, with no forward pointer, is not a contract; it is
folklore.

### (b) The rule is stated in the *negative voice of the wrong story*

`epics.md:849` says "**This story** renders and announces Results; it never writes a
durable record itself" — a constraint on 3d — and then asserts a positive obligation
on a *third party*: "UX-PB.4a writes it". A story's acceptance criteria are the
contract for that story's code and that story's tests. UX-PB.3d can be built, tested
and accepted in full while UX-PB.4a violates the sentence, because nothing in
UX-PB.3d's deliverable can fail when UX-PB.4a appends a second terminal record.
The clause has no test owner. Compare AD-24, which puts the same class of rule where
it binds every party at once: `ARCHITECTURE-SPINE.md:801-805` `"The one persistent draft has exactly one author … no other path adds, replaces, or clears\n  membership."`

### (c) The AD-18 binding set is wider than its citation set — including the writer that isn't governed

> `ARCHITECTURE-SPINE.md:622` `"- **Binds:** UX-PB.2c, UX-PB.2d, UX-PB.2f, UX-PB.3d, UX-PB.4a, UX-PB.4b, UX-PB.4e; Story 6.5"`

`grep -n "AD-18" epics.md` returns 7 lines — 288, 316, 664, 827, 916, 1004, 1302.
Mapping them (`awk` walk-back to the nearest `### Story` heading): 664 is
**UX-PB.2b** (not in the Binds set at all), 827 is UX-PB.3d, 916 is UX-PB.4a, 1004 is
UX-PB.4e, and 1302 is Story 6.5's governing-invariants line. So of the eight bound
items, **UX-PB.2c, UX-PB.2d, UX-PB.2f and UX-PB.4b cite AD-18 nowhere.**

UX-PB.2c is the decisive case, because it is the *third writer*:

> `epics.md:693-694` `"**Then** the append-only record stores the reviewed Manager/Package scope, Manager self-update identities, exact command snapshot, version evidence, timestamps, and result/verification state as immutable plan-admission metadata\n**And** the stored command snapshot is read back only as evidence and is never converted back into executable input."`

> `epics.md:684` `"**Dependencies:** UX-PB.2b; AD-16; D29; AD-27 (focus is a 2px `outline` in `--color-focus-ring`; never a `ring-*`/box-shadow, which WKWebView drops on native-appearance controls)"`

UX-PB.2c writes to the journal AD-18 governs, is listed in AD-18's `Binds`, and does
not cite AD-18. Its own record stores "**result/verification state**" — a field that,
on its face, a terminal write would also carry. The only place in the planning corpus
that reconciles 2c's write with 4a's is `epics.md:849`, a sentence in a *third*
story that 2c's builder has no reason to open.

### What breaks if UX-PB.4a is built by someone who never reads UX-PB.3d

Reading only `epics.md:913-939` and `ARCHITECTURE-SPINE.md:620-649`, the failure is
not "she omits the terminal write" — `epics.md:929` `"**When** its single immutable History row cannot be persisted"` tells her she persists
something, and `epics.md:936-937` tells her a terminal record exists and that she
folds. The failures are these:

1. **Duplicate terminal records, with no authority rule.** She has no statement that
   she is the *only* terminal writer. If UX-PB.3d's implementer also appends at
   terminal — which UX-PB.3d's own `Given` invites,
   `epics.md:847` `"**Given** an attempt reaching terminal state (Results persistence failure)"` —
   the journal holds two terminal records for one `planAttemptId`. AD-18 forbids
   in-place correction: `ARCHITECTURE-SPINE.md:626` `"append-only NDJSON journal"`.
   `epics.md:849` names precisely this outcome — "A second terminal write here would
   append a duplicate with no rule for which record is authoritative" — from inside
   the one story that cannot enforce it.
2. **The fold is underspecified in the direction she was not warned about.** Her fold
   criterion covers absent and unparseable terminal records
   (`epics.md:936-939`). It says nothing about *two present, parseable, conflicting*
   terminal records, because her story's author knew the rule that made that
   impossible and did not restate it. Last-wins, first-wins, and refuse are all
   compliant with everything she can read.
3. **UX-PB.3d's own failure criterion may have no code to attach to.** `epics.md:848`
   makes 3d's testable behavior conditional on "the single durable terminal write
   **owned by UX-PB.4a**". If 4a's builder implements the History row as a projection
   over records someone else appends, that owned write does not exist as a distinct
   failure site, and 3d's fourth criterion is unverifiable — an accepted story with an
   untestable acceptance criterion.
4. **Cross-journal retention becomes ambiguous.** `ARCHITECTURE-SPINE.md:642-646`
   requires compaction to keep counterparts resolvable. With an unfixed per-attempt
   record count, "compacting the attempt journal may not leave Operations pointing at
   an attempt that no longer resolves" gives no answer to whether dropping one of two
   terminal records for the same attempt still "resolves". A retention rule cannot be
   implemented against an unstated cardinality.

**Where the rule belongs.** The `Prevents` line already gestures at the right shape —
`ARCHITECTURE-SPINE.md:623-625` names three actors, "the writer, the History reader,
and the diagnostics exporter", and then only governs their choice of *home*. Widening
that same sentence's Rules to govern *who appends* and *how many records one attempt
yields* is a one-AD change that binds all eight stories in `ARCHITECTURE-SPINE.md:622`
at once, in the idiom AD-24 already uses at `ARCHITECTURE-SPINE.md:801-805`. Until
that lands, the rule is enforceable against exactly one story — the one that does not
perform the write it governs.
