# DOCKET D-009b2 — Finalize Must Not Break the Lock

Authority: VOICE-1 v0.1 · AGENT-1 Q-1–Q-8 · ORDERS-1 v0.7 · founder ruling (Option A, 2026-07-11) on the Seat 3 d009b HOLD · G-Q, extended: **the lock must survive its own success, not only its failures.**
Executor: Seat 4, **own clone** (`C:\Users\travi\lobster\LOVErnment-DAO`) — **only when its eyes are verified working** (read a `sha256sum` back as text first). A docket handed to a blind seat is the exact failure the ELEVATED review existed to catch; do not execute this blind. Delivery law: branch `seat4/d009b2` off `seat4/d009b` @ `26f2d6b` (keep its work; fix only finalize). Push branch only; Seat 3 sole merger; red-first; digests **after** commit against the commit's own objects, **Git Bash sha256sum on stdin, never PowerShell pipes**.

**No secrets in this docket.** Offline against the mock XRPC transport, same as d009b. Credentials and the real DID are D-009c.

---

## The defect (Seat 3, ratified)

`finalize_entry` does `putRecord({$type, postUri, postCid})` at the entry's rkey — a **replace, not an update** — stripping `derivationInput` and every other field. `run()` calls `finalize_entry` on **every** successful post (pipeline.rs:268), so after any success the audit record loses the field the durable lock scans on. Next run in a fresh process (empty in-process `seen`) → `find_entry_by_derivation_input` finds no match → `Ok(None)` → **posts again.** This re-opens the exact restart/multi-instance double-post window d009a2 closed — for the common successful-post case. It is mainline, not an edge, and offline-provable.

## The cure

`finalize_entry` must **preserve `derivationInput` and all pending fields**, adding only `postUri`/`postCid`. Two acceptable shapes:
- **Read-then-merge:** `getRecord` the existing pending entry → set `postUri`/`postCid` on it → `putRecord` the full merged record. Faithful to the ATProto model; the `getRecord` failure is itself fail-closed (an `Err` propagates, the entry stays pending-and-findable — safe).
- **Carry-forward:** thread the full `PendingEntry` (already in hand at the call site) into `finalize_entry` and `putRecord` the complete record with the post fields set — no extra round-trip.

Either is acceptable; carry-forward is simpler and avoids a network hop. Choose one, state which in the commit prose. **No field is lost on finalize. `derivationInput` survives every terminal state — pending, failed, and now finalized.**

Keep everything d009b got right — the fail-closed `submit_post` (strict parse, `2xx`-missing-uri/timeout/5xx → `Err`), `mark_entry_failed` (never deletes), `remove_entry` unreachable from `run()`. Touch only finalize and its test.

## Red-first suite

**Marquee red #1 — the lock survives success:** `create → submit → finalize → find_entry_by_derivation_input(input)` still returns the entry (now finalized, `postUri`/`postCid` set, **`derivationInput` intact**). Against current code this fails (finalize stripped the field). This is the test whose absence let the defect through.

**Marquee red #2 — no re-post after successful finalize:** run to success, then a **fresh pipeline** (empty `seen`) over the same store with the same facts → **Duplicate**, no second post. The restart-durability proof, now covering the post-finalize state (d009a2 only proved it for pending/failed entries).

r1/r2 from d009a2 and the d009b write tests stay green throughout.

## Non-blocking, fold in

The `"repo": "bQueenBee"` literal in the write bodies is a placeholder — comment it as a **hard D-009c wiring prerequisite** (the real DID arrives with credentials at the ceremony). Do not resolve it here; just mark it so D-009c cannot miss it.

## Acceptance

Red witnessed at commit A (both marquee reds failing against the current finalize); commit B green across `cargo test --workspace`; all changed-file digests declared post-commit against the commit's own objects. Push `seat4/d009b2` only. Seat 3 runs the standing red-witnessed pass; on merge, prune **both** `seat4/d009b` and `seat4/d009b2`, retain the `d002r` marker.

**After merge — D-009c, THE FIRST WORD:** Step 0 (bQueenBee account, env-delivered `QUEENBEE_*`, never chat) + founder go; the `"repo"` literal gets the real DID; genesis derivation input is commit `884b2bce` per VOICE-1 §3.
