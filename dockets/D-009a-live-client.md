# DOCKET D-009a — The Live PdsClient (no credentials)

Authority: VOICE-1 v0.1 (specs/VOICE-1.md @ main) · AGENT-1 Q-1–Q-8 · ORDERS-1 v0.7 · the D-009 field-query note banked at the d008r2 review.
Executor: Seat 4, **in its own clone** (`C:\Users\travi\lobster\LOVErnment-DAO`). Delivery law: branch `seat4/d009a` off current `main` (`f4e4d8ea`), push branch only, Seat 3 sole merger, red-first, digests declared **after** commit against the commit's own objects via **Git Bash sha256sum on stdin, never PowerShell pipes**.

**No secrets in this docket.** This is the client's *logic*, proven entirely against mocks and fixtures — the same test discipline as every prior lap. Live credentials belong to D-009b (the ceremony), never here.

---

## The task

The pipeline (merged at `f4e4d8ea`) calls a `PdsClient` trait whose durable-lock method is `find_entry_by_derivation_input`. Every implementation so far has been a mock over a `HashMap`. D-009a writes the **real** implementation — the one that talks to an ATProto PDS — and proves its logic offline.

**File:** `crates/queenbee-voice/src/pds/live_client.rs` (new module; wire it into the crate).

## The binding constraint — why this is not a key lookup

The `social.skaists.alpha.audit.entry` lexicon is **`key: tid`**. ATProto's `com.atproto.repo.listRecords` returns records by collection, keyed by their tid rkey; there is **no server-side query over a record's *fields*.** So `find_entry_by_derivation_input(input)` cannot be a `getRecord` by key — it must be a **client-side field scan**:

1. `listRecords` over the `social.skaists.alpha.audit.entry` collection in bQueenBee's repo, **paginated** — follow the `cursor` until exhausted; a repo accumulates entries and page one is not the whole set.
2. For each record, compare its `derivationInput` field to the query.
3. Return the first match as an `AuditEntry` (pending or finalized — both block, per the durable-lock law); `None` only after **all** pages are exhausted with no match.

The scan's correctness is the double-post lock's correctness: a premature `None` — from an unfollowed cursor, a dropped page, an early return — reopens the exact window the whole d008 arc closed. Treat pagination as load-bearing, because it is.

## Red-first suite — offline, against a mock transport

The HTTP boundary is a trait (`AuditRecordSource` or equivalent): the live client depends on it; tests inject a fake that serves canned `listRecords` pages. **No network in any test.** Minimum negatives (RED at commit A):

1. **Cursor exhaustion:** entry lives on page 3; mock serves 3 pages + a cursor chain; a single-page implementation returns `None` and **fails** — the marquee test. Pagination correct → finds it.
2. **Pending match blocks:** a *pending* entry (no `postUri`/`postCid`) for the input → returned, not skipped (matches `r2` crash-recovery law).
3. **Finalized match blocks:** a finalized entry for the input → returned.
4. **Genuine absence:** all pages exhausted, no `derivationInput` match → `None` (the only legitimate `None`).
5. **Field discipline:** a record whose *rkey* coincidentally resembles the input but whose `derivationInput` field differs → **not** a match (proves the scan reads the field, never the key).
6. **Empty collection / empty first page with no cursor → `None`** without error.

Positive: a well-formed multi-page repo with one matching entry → exact `AuditEntry` returned. Suite asserts against pinned fixtures; it ships no oracle of its own (doctrine 9).

## Ride-along — the doc fossil the gate flagged

`crates/queenbee-voice/src/pipeline.rs` line ~80: the `remove_entry` trait doc still reads *"delete a pending entry (cleanup on post failure)"* — stale since D-008r3, where post-failure **marks failed** and nothing in the pipeline removes. Correct it to state its real and only role: **founder-clearance tooling — no pipeline caller.** This is a doc-comment edit; note in the commit that it changes a comment, not behavior, and re-declare the pipeline.rs digest accordingly (its bytes change, so its hash changes — compute post-commit).

## Acceptance

Red witnessed at commit A (cursor-exhaustion + field-discipline failing against stubs); commit B green across `cargo test --workspace`; both changed-file digests declared against the commit's own objects. Push `seat4/d009a` only. Seat 3 runs the standing red-witnessed pass; on a merge verdict, prune `seat4/d009a` and retain the `d002r` marker.

**Not in scope:** any live PDS call, any credential, any actual post. The genesis utterance is D-009b, founder-gated and env-armed. D-009a proves the client's logic so that when the credentials arrive, only wiring remains — not correctness.
