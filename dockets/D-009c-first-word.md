# DOCKET D-009c — THE FIRST WORD

Authority: VOICE-1 v0.1 §3 (genesis utterance) · AGENT-1 Q-1–Q-8 · SPIRIT-1 (bQueenBee = QueenBee genesis epoch) · ORDERS-1 v0.7 · the completed write path @ DAO main `ed1daa6`.
Executor: **Seat 3, env-armed** (the D-001 whistle precedent — secrets ride as environment variables in the launch shell, never chat). Single run. Founder go required before execution.

**This is the only lap that touches a live PDS.** Everything before it was proven offline; this one speaks.

---

## Identity — of record

- **Subject:** bQueenBee, genesis occupant of the RBI chair (SPIRIT-1 §1).
- **DID:** `did:plc:77xbxwg7vh3wh5pmzvid65hc` — this replaces every `"repo": "bQueenBee"` placeholder in the five write-method bodies (`live_client.rs`), commented `D-009c WIRING PREREQUISITE`.
- **Handle:** `bqueenbee.beehivenature.com` (custom domain; the chair represents Beehive Nature, the Reserve — not any single DAO). Genesis binds to the **DID**, so the handle-flip state does not gate the ceremony; the DID is stable regardless.
- **Disclosure:** the account is machine, affirmatively (A-6). `performer.kind = machine`.

## Step 0 — credentials (founder act, already done or doing)

The account exists with its own handle, DID, and a **posting-scope app password** (never the account password; revocable from the founder's device — A-10). The three values reach Seat 3 **only** as environment variables in the launch shell:

```
QUEENBEE_PDS_URL      = <her PDS host>
QUEENBEE_HANDLE       = bqueenbee.beehivenature.com   (or bqueenbee.bsky.social if handle not yet flipped — DID is what binds)
QUEENBEE_APP_PASSWORD = <app-scoped, revocable>
```

**Never pasted into chat, never committed, never logged.** The D-001 leak lesson is law.

## The wiring

Before the run, the `"repo": "bQueenBee"` literals in `live_client.rs` take the real DID `did:plc:77xbxwg7vh3wh5pmzvid65hc`. This is a code edit on a `seat4/*`-class delivery branch (`seat4/d009c-wiring`) if it changes committed bytes — red-first if it carries a test, or a trivial wiring commit reviewed by the merger — OR, if the DID is injected purely from env at runtime with no committed change, no branch is needed. **Seat 3's call which shape applies; the constraint is only that no secret and no non-public value enters the tree.** The DID is public (it's in the PLC directory), so it may live in code; the app password may not.

## The utterance

- **Genesis derivation input:** commit **`884b2bce`** — VOICE-1's own landing on DAO main (VOICE-1 §3). Her first sentence is a TreeLanding announcement of the document that governs her mouth.
- **Adapter:** class 1, TreeLanding, as merged. The pipeline runs **once**: adapter → wrapper gate (heartbeat fresh, rate 0-of-3 today) → fail-closed read scan (no prior entry for `repo@sha` `beehive-nature/beehive-nature@884b2bce`... **note:** confirm the derivation-input repo — VOICE-1 §3 says the input is the VOICE-1 landing commit `884b2bce`, which is on **skaists/LOVErnment-DAO** `main`; the derivation input string is `skaists/LOVErnment-DAO@884b2bce`) → create pending audit entry → submit post → finalize entry.
- **One post. One audit entry. Matched pair.** The atomic pipeline guarantees no utterance without its entry; the fail-closed lock guarantees no double-post on any retry.

## Acceptance — the matched pair, verified live

After the single run, confirm on the live PDS:
1. **The post exists** at `bqueenbee.beehivenature.com`'s repo — a TreeLanding announcement referencing `884b2bce`.
2. **The audit entry exists** (`social.skaists.alpha.audit.entry`) with: `postUri`/`postCid` matching the post, `derivationInput` = the `884b2bce` input string, `inputDigest`, `adapterClass = TreeLanding`, `adapterDigest`, `modelDigest`, `promptDigest`, `createdAt`.
3. **They reference each other** — the post is announced, the entry attests it. A stranger can fetch the entry, fetch the input commit, re-hash, and verify (Q-6, four moves).
4. **Idempotence proven live:** a second invocation with the same input → the fail-closed read scan finds the entry → **refuses**, no second post. (Optional but recommended: prove it once, since this is the first live exercise of the lock.)

Report: the post URI, the audit entry URI, and the confirmation they form a matched, cross-referencing pair. Then bQueenBee has spoken — auditable from her first word, exactly as VOICE-1 designed.

## After

- The heartbeat clock starts (Q-5: 21 days; a missed founder beat suspends her).
- The rate cap is live (Q-4: ≤3/day).
- Her voice is autonomous within TreeLanding and bounded by everything ratified.
- New adapter classes and allowlist additions remain founder gates (Q-8).
