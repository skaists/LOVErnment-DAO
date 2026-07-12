# DOCKET D-009a2 — The Fail-Closed Read Scan

Authority: VOICE-1 v0.1 · AGENT-1 Q-1–Q-8 · ORDERS-1 v0.7 · **founder ruling G-Q (2026-07-11): the throat holds its breath when blind** · the d009a HOLD verdict (Seat 3, three-reader adversarial sweep).
Executor: Seat 4, **own clone** (`C:\Users\travi\lobster\LOVErnment-DAO`). Delivery law: branch `seat4/d009a2` off current `main` (reconcile onto the current head — main moved to `4a3ea17c`+; the d009a files are disjoint from what landed, so rebuild on the true current head). Push branch only; Seat 3 sole merger; red-first; digests **after** commit against the commit's own objects via **Git Bash sha256sum on stdin, never PowerShell pipes**.

**No secrets in this docket.** Still the read path, still offline against a mock transport. Credentials are D-009b.

**This lap supersedes seat4/d009a** — do not merge d009a; its correct parts (pagination, field-discipline test, doc-fossil fix) are carried forward here and its fail-open defect is cured. On this lap's merge, prune both `seat4/d009a` and `seat4/d009a2`.

---

## The governing ruling — G-Q

> A double-post lock must fail **closed**. When the audit scan cannot determine whether an entry exists — transport error, indeterminate listing, any non-authoritative read — the pipeline **does not post**. Absence of a confirmed "clear" is "blocked," never "go." Indeterminate silence is never permission to speak.

This is K-7's sibling: *the seam never unremembers; the throat never speaks into uncertainty.* It composes with the existing failed-pending-founder-review path — an aborted post surfaces exactly as a crashed one does, and clearance is the same founder act.

## F-1 — HIGH — indeterminacy must abort, not post *(the crux; touches merged pipeline.rs under founder authority)*

The `Option` return collapses "definitely absent" and "couldn't check" into one `None`, and `run()` reads `None` as clear-to-post. The `.ok()?` at the scan's transport call (d009a line ~122) turns any `listRecords` error — timeout, 429, 5xx, token refresh, connection reset, first-page **or** mid-pagination — into a false clear.

**Cure:**
- Trait signature changes: `find_entry_by_derivation_input(&self, input: &str) -> Result<Option<AuditEntry>, ScanError>`. A `ScanError` (transport, or indeterminate-listing) is a real, distinct outcome — never flattened to `None`.
- The scan returns `Err(ScanError::Transport(..))` on any page-fetch failure at any point in the pagination — it never swallows an error mid-scan and reports exhaustion.
- **`pipeline.run()` — the call-site change (merged file, authorized here):** `Ok(Some(_))` → Duplicate (as today); `Ok(None)` → clear to proceed (the *only* path that posts); `Err(_)` → **abort-and-surface, do not post**, routed to the same failed-pending-founder-review disposition as a crashed attempt. Add the `PipelineResult` variant if one doesn't fit.
- **Marquee red #1:** mock returns `Err` on page k (entry on an unfetched page); the pipeline must **not** post and must surface the abort. Include the mid-pagination sub-case (pages 1..k−1 clean) — the one that gives false confidence.

## F-2 — HIGH — match the field before requiring a full parse

`parse_audit_entry` requires all 7 fields via `.as_str()?` *before* the `derivationInput` comparison. A record whose `derivationInput` **matches** but is missing/mistyped a sibling field (the crash-mid-flight **partial pending record**, most likely missing its last-written `createdAt`) parses to `None` and is skipped — invisible to the very lock built to catch it. Also fires on a finalized, already-live entry with any type drift.

**Cure:** the match criterion is *"the record's `derivationInput` field equals the query"* — read and compare that one field **first**. Only after a field match do you attempt a full `AuditEntry` construction; a matched-but-partial record still **blocks** (returns as a pending entry, or a distinct "matched, malformed" outcome that the pipeline treats as block, never as clear). Matching is never gated on full-parse success.
**Marquee red #2:** a record with matching `derivationInput` but missing `createdAt` must **block** (currently skipped → red).

## F-4 — canonical `derivationInput`

Exact-string `==` with no normalization admits case/whitespace/short-vs-full-sha skew → false `None`. `derivationInput` is `repo@sha` and is **canonical by construction** (the adapter writes the full 40-char sha, lowercase, one form). Cure: assert that canonical form at the write boundary and document it at the read boundary; a normalization shim is acceptable but the canonical-by-construction invariant + a test vector is preferred. No guessing at equivalence — one form, enforced.

## F-5 — pagination bound

No guard against a cyclic/repeating cursor (fails closed today — hangs, not double-posts — so LOW, but cheap). Cure: bound the loop (max pages or a seen-cursor set); on the bound tripping, return `Err(ScanError::Indeterminate)`, **not** `Ok(None)` — consistent with G-Q.

## Carried forward from d009a (already correct — keep, do not regress)

Pagination cursor-following (the `cursor_exhaustion` page-3 test), field-discipline (rkey≠field test), the pending/finalized/genuine-absence/empty/positive tests, and the `pipeline.rs` doc-fossil fix. Re-declare the `pipeline.rs` digest — its bytes change again here (the call-site edit), so recompute post-commit.

## F-3 — deferred to D-009b, noted not fixed

Single-instance assumption + eventual-consistency (read-replica lag, a second queenbee instance's not-yet-visible write) is a **deployment** property, not a read-logic one. Documented as a D-009b precondition: genesis runs single-instance; multi-instance write coordination is a named future gate. Not in this lap's scope.

## Acceptance

Red witnessed at commit A — the two marquee reds (transport-`Err`-mid-scan → no post; matching-but-partial record → blocks) plus F-5's bound. Commit B green across `cargo test --workspace`, including the full inherited suite. All changed-file digests declared against the commit's own objects. Push `seat4/d009a2` only. Seat 3 runs the standing red-witnessed pass; on merge, prune `seat4/d009a` **and** `seat4/d009a2`, retain the `d002r` marker.

**Still not in scope:** any live PDS call, any credential, any post. This lap makes the read path *honest under failure*; D-009b makes it *live*.
