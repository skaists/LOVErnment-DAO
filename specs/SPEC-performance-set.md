# SPEC — `social.skaists.alpha.performance.set`

Status: **APPROVED — v0.1, 2026-07-09.** Founder gate cleared on §7.1–§7.4, §9a, §9b.
**§7.5 (kernel binding) remains open by explicit decision** and is out of scope for this document. §9b's H3 deferral (`PRIV-1`) is out of scope and unfiled.

Author digest of this spec's subjects, pinned below. Derive once, against the digest.
The digests in §3 are now **frozen**. Any change to those bytes requires a version bump and a re-gate; it does not inherit this approval.

---

## 1. The finding this rests on

Direct read of `teal-fm/teal` at pinned commit. Three facts, each verified against the file, not inferred:

**F-1. `fm.teal.alpha.feed.play` is a record with `"key": "tid"`.** It lives in the *listener's* repository. Its own description states a play is submitted after a user has listened through the track. teal's semantic is **consumption**. A DJ is not consuming; the room is.

**F-2. There is no cross-record reference mechanism anywhere in `fm.teal.alpha`.** No `strongRef`, no `subject`, no `cid`. All 18 lexicon files searched. A set may point at plays; plays can never point back. "Which set did this play belong to" is answerable only from an AppView index, never from the record itself.

**F-3. `playedTime` is optional on `feed.play`.** Required is `trackName` alone. Ordering therefore falls back to rkey TID, which is **write order, not performance order**. These diverge the moment a performer backfills a set after the night.

### Verdict

> **The record does not survive being wrapped. The view object survives being embedded.**

`fm.teal.alpha.feed.defs#playView` is an *object* def, not a record. teal embeds it themselves inside `fm.teal.alpha.actor.status`. It is built for embedding. That is the piece we take.

---

## 2. Vendor pin (author digest)

Nothing downstream may re-derive from a moving target. All structural claims above are made against exactly these bytes.

```
upstream:      github.com/teal-fm/teal
commit:        f661868af2a56199f1b3cae1ad20cd5831ec4212
commit date:   2026-05-30T22:51:33-05:00
license:       MIT
```

| Path | sha256 (author digest) |
|---|---|
| `lexicons/fm.teal.alpha/feed/defs.json`   | `a46be104f27968c35ff12564bc205e7e3bd4b1e2aff2655d51a19b177e0b7ceb` |
| `lexicons/fm.teal.alpha/feed/play.json`   | `5a5f3f23fecc6b821dc3e006cc0487a4d29eb333e5d4226e92c9ca4f33b61448` |
| `lexicons/fm.teal.alpha/actor/status.json`| `af0f5c2e7a6bd5fc8797124ebe2a7579ad857407e80372a2c1bf49217b72f0e6` |

### Second upstream: `lexicon-community/lexicon`

```
upstream:      github.com/lexicon-community/lexicon
commit:        bd2c916e33434b767fa89fa1305ff37e1ff8ffc6
commit date:   2026-06-24T11:28:49-04:00
license:       MIT
```

| Path | sha256 (author digest) |
|---|---|
| `community/lexicon/calendar/event.json`   | `1d612281a347142a061487c2735de0c8ebffded44f33531c9dd71c9a29087d47` |
| `community/lexicon/calendar/rsvp.json`    | `dae48afef85730fdace6b2fb4d5fcba6c9c5bc07c9dbd3469b36d1b4aa9f7c52` |
| `community/lexicon/preference/ai.json`    | `6e62f33e06d4eebf97a7d1a3a31b6f2e110c480d289cd7f6be09c0b60dec04db` |

Unlike teal, these files carry **no "subject to change" banner**. They are community-governed and MIT.

**Standing hazard.** Every one of teal's 18 lexicon files self-declares *"in a not officially released state. It is subject to change."* This is an alpha schema. Pinning is not paperwork; it is the only thing standing between an upstream rename and a silent corruption of every set record we have ever written.

## 3. Artifacts produced

| Path | sha256 |
|---|---|
| `lexicons/social.skaists.alpha/performance/defs.json`      | `b57bdb1e0f8f9caf09d51c463a3361b15fcdc2fef8aaf7c223bc0be1b88c0282` |
| `lexicons/social.skaists.alpha/performance/set.json`       | `33c7a39b8db589cd2938c9d9c4a6304a03826bd25175b1b5535aa1bd50e2381f` |
| `lexicons/social.skaists.alpha/performance/setStatus.json` | `b8e742c6420dbb5586d2f2a1194f91cdfd589a76ce49e0592e670398fed6d929` |

Verifier state at approval: **MATCH** across all three files. Every `ref` resolves against the local def table or the vendored-external allowlist (`com.atproto.repo.strongRef`); no `number` primitive appears anywhere; every required `knownValues` field carries an explicit escape value; every invariant cited by a lexicon is defined in §4. Reproduce with `verify.py`.

---

## 4. Invariants

Each is stated so that a violation is decidable by a machine, not by taste.

**SET-1 — Position is total and dense.**
`items[i].position` strictly increasing, beginning at `0`, no gaps, no duplicates. Array order is advisory; `position` is normative. A consumer that trusts array order is wrong.

**SET-2 — Time is monotonic.**
`items[i].play.playedTime` is non-decreasing in `i`. Two tracks may share a timestamp (a layered acapella); none may precede its predecessor. *This is the same class of defect caught in escrow-core's timestamp non-monotonicity test. It was a real bug there. It will be a real bug here.*

**SET-3 — The set contains its tracks.**
`startedAt ≤ items[0].play.playedTime`. If `endedAt` is present, `items[n-1].play.playedTime ≤ endedAt`.

**SET-4 — Redundancy is deliberate; disagreement is a MISMATCH.**
Where `cueTime` is present, `cueTime == floor(playedTime − startedAt)` in seconds, tolerance ±1s for clock granularity. `cueTime` is a derived field carried for cheap seeking. When derived and primary disagree beyond tolerance, the record is rejected. It is **never** repaired by recomputing `cueTime` from `playedTime` — that would launder a corrupt record into a clean-looking one. *(no-reconstruction rule)*

**SET-5 — Embed, never reference.**
A `set` record MUST NOT contain an `at-uri` to any `fm.teal.*` record. Tracks are carried by value. Consequence: a set is self-contained and verifiable offline; it does not rot when a listener deletes their repo.

**SET-6 — No floats exist.**
AT Protocol lexicon v1 has no float primitive. Tempo is `bpmMilli`, an integer in milli-BPM (`174000 == 174.000`). Do not substitute a string. Do not round to integer BPM — the fractional part is what a pitch fader *is*.

**SET-8 — `eventUri` is enrichment, never dependency.**
A `set` carrying an `eventUri` that is dangling, unresolvable, deleted, or `cancelled` is **still a valid set**. Consumers MUST render it. `venue` is load-bearing; `eventUri` is not. The reference is an unpinned `at-uri` and carries no `cid`: this record attests **presence**, not **description**. A promoter renaming, rescheduling, or cancelling the event does not falsify the performer's claim to have played there, and the record must not be constructed so that it could.

**SET-9 — Sets are append-only.**
A correction is a **new record** carrying `supersedes: {uri, cid}` pointing at the record it replaces. Never an in-place `putRecord`. The chain is single-predecessor, acyclic, and confined to one repository: you may not supersede another performer's set. The **head** is the record no other record supersedes; consumers render the head. This mirrors `did:autonomi`'s append-only signed rotation log, one layer up.

**SET-10 — The superseded record is not deleted.**
Not deleted, not tombstoned. A performer who corrects a tracklist leaves standing evidence that they corrected it. Deleting the predecessor severs the chain and destroys the only thing that made the correction trustworthy.

**SET-11 — Disclosure is affirmative.**
`#performer.kind` is **required**, and `undisclosed` is a sayable value. Silence is not. An absent or unrecognized `kind` MUST render as undisclosed and MUST NEVER default to `human`. Whenever any performer is non-human, `performers` MUST be present and MUST enumerate every performer, including the human ones. This is the record-layer twin of `community.lexicon.preference.ai`'s rule that an omitted field means *no declared preference*, never *consent*.

**SET-12 — Agency is per-performer; "mixed" is derived.**
Two humans, two machines, or any blend is a property of the `performers` array, not a field on the set. Do not store it. SET-4 already says what happens when a derived field and its primary disagree.

**SET-13 — `#venue` is a strict profile of `community.lexicon.location.address`, not a ref to it.**
Field names mirror `address` exactly, so a venue lifts with zero transformation whenever `country` is present. `name` is **required** here and optional there. `country` is **optional** here and required there — deliberately, because a writer who cannot name the country would otherwise have to omit `venue` entirely and lose the venue *name* along with it. Absent `country` costs interop only. The lift is one-way: our venue always publishes as an address; not every address imports as a venue. Reject on import, never repair.

**SET-7 — Authorship follows performance.**
A `set` is written by the performing DID. Audience listening is out of scope for this record and belongs in teal's namespace, where it already works.

### Status invariants

`setStatus` is governed separately, because it is a different kind of object.

**STATUS-1 — Status is state; sets are attestations.**
`setStatus` is the sole record in this namespace that is mutated in place. It is never append-only, never superseded, never corrected. It makes no historical claim and therefore cannot be wrong about the past — only stale about the present. **Only attestations may be promoted to the kernel.** See K-2.

**STATUS-2 — Absent expiry defaults to ten minutes; stale never renders as live.**
When `expiry` is absent, consumers MUST treat the status as expiring ten minutes past `updatedAt` — convention borrowed from `fm.teal.alpha.actor.status`. A status past its expiry MUST NOT be rendered as live. Silence from a performer's phone is indistinguishable from a dead battery, and the safe reading of both is *not live*.

**STATUS-3 — A status is never promoted into a set by mutation.**
The `set` is written once, after the performance is over, as its own record. `setUri` points forward from the status to that record. It is never derived backward, and a status is never rewritten into a set. A live status accumulates nothing; if it did, every `putRecord` during a four-hour night would rewrite the whole set and SET-9 would be a fiction.

---

## 5. Ingest conformance: lifting a teal `playView`

`playRef` mirrors `playView` field-for-field so that a conforming object lifts with **zero transformation**. It is deliberately *stricter* on the required set.

| Field | teal `playView` | skaists `playRef` |
|---|---|---|
| `trackName` | required | required |
| `artists` | required | required, `minLength: 1` |
| `playedTime` | optional | **required** |
| everything else | optional | optional |

**Rule.** A `playView` missing `playedTime` is **rejected at ingest**. It is not repaired. It is not backfilled from `now()`, from rkey TID, or from position. There is no timestamp to recover — synthesizing one manufactures evidence.

This is the whole reason the record cannot be wrapped and the object can be embedded: wrapping would have inherited teal's optionality along with its shape.

---

## 6. Namespace and licensing

- Records are published under `social.skaists.*`. Never under `fm.teal.*`. That namespace is not ours; the domain `teal.fm` is verified to their org.
- Upstream is MIT. MIT flows one-way into AGPL-3.0-only. Structural derivation of `playView`'s field names is attributed in `defs.json` and in this document. Nothing is copied that isn't a schema field name.
- No cross-namespace `ref` is emitted. `playRef` is a mirror, not an alias. This costs us interop discoverability and buys us immunity from their alpha churn. That trade is the correct one while their files carry a "subject to change" banner.

---

## 7. Open questions — founder gate

Deliberately **not** answered here. Each would expand scope.

1. ~~**Event linkage.**~~ **RESOLVED — see §9.** Neither minting nor natural-key. Adopt `community.lexicon.calendar.event` by unpinned `at-uri`. `eventUri` added to `set.json`; `venue` remains load-bearing.

2. ~~**Live status.**~~ **RESOLVED.** `social.skaists.alpha.performance.setStatus` drafted. Original text: **Live status.** teal's `actor.status` is `"key": "literal:self"`, singleton per actor, 10-minute default expiry. A performer broadcasting a live set would **collide with their own personal scrobbling status**. A `performance.setStatus` record is the obvious answer and has not been drafted.
3. ~~**`transition.technique` vocabulary.**~~ **RESOLVED — see §11.** No tokens. `knownValues` plus the escape rule.
4. ~~**Set mutability.**~~ **RESOLVED — see §12.** Append-only, superseding, content-pinned, never deleted.
5. **Kernel binding.** Nothing here touches the kernel tree. Whether a `set` becomes a `CanonicalEventV1` — and if so, whether it is an Event or an Evidence — is unanswered and should not be answered casually. The seven primitives are a constitutional interface.

---

## 8. Next actions

- [ ] Founder gate on §7.1–§7.5.
- [ ] File the upstream issue (`UPSTREAM-ISSUE-teal-getPlay.md`). Free goodwill, real bug.
- [ ] Property tests for SET-1 through SET-4 before any implementation. Red first.
- [ ] Decide whether `cadet` (teal's Rust jetstream ingester, MIT) is a reference or a dependency. It is architecturally the same shape as the `chain-*` workers. It is **not** yet proposed as either.

---

## 9. Event linkage — resolution

The gate presented two options. Both were wrong, and the fork was false.

**Option A — natural key `(venue, startedAt)`.** Fails immediately. Two DJs at one party have two different `startedAt`. The natural key identifies the *set*, not the event. Recovering the event requires fuzzy-matching user-typed venue strings within a time window — `Berghain` / `berghain` / `Berghain, Berlin` — forever, in the record layer. Festivals with forty stages break it outright.

**Option B — mint `social.skaists.alpha.performance.event`.** Fails on authorship. An ATProto record lives in *a* repo. An event has no natural single author. Promoter? Venue? First DJ to post? Whoever we pick, everyone else's event is a duplicate, and we own a merge problem we invented.

**Option C — the record already exists.** `community.lexicon.calendar.event`, MIT, community-governed, pinned above. `key: tid`. It carries `name`, `startsAt`, `endsAt`, `mode` (inperson/virtual/hybrid), `status` (planned/scheduled/rescheduled/cancelled/postponed), and a `locations` union over `address` / `geo` / `fsq` / `hthree`. Its sibling `community.lexicon.calendar.rsvp` already references events via `com.atproto.repo.strongRef`. The pattern is established; we are not inventing it.

### Why the reference does not violate SET-5

SET-5 says *embed, never reference*, and the justification was that a set must not rot when someone else deletes their repo. That reasoning is preserved exactly:

- `venue` stays **embedded and load-bearing**. A set is complete, orderable, and renderable with zero network calls.
- `eventUri` is an unpinned `at-uri` and is **enrichment**. Dangling is a legal state.

Authorship duplication does not disappear; it becomes **addressable**. Two event records for one party is a disagreement two performers can settle by pointing at the same URI. A natural key gives them nothing to point at.

### Benefit / risk

| | |
|---|---|
| **Benefit** | Stable addressable identity. Free interop with every Smoke Signal–class calendar consumer. RSVP, cancellation, and rescheduling semantics we do not have to design, argue about, or maintain. Zero new namespace surface. |
| **Cost** | One optional field. No external ref; the lexicon closes over its own defs. |
| **Risk** | Duplicate event records. **Unmitigated at record layer; index-layer merge, by design.** Deliberately not solved here. |
| **Risk** | Dangling or cancelled `eventUri`. **Mitigated by SET-8** — dangling is legal, `venue` carries the meaning. |
| **Risk** | Promoter edits the event after the fact; the set now points at a different claim, undetectably. **Accepted.** See §9a. |
| **Risk** | Community lexicon churn. **Mitigated by the pin.** No "subject to change" banner, unlike teal. |

Benefit exceeds adverse risk. Adopted.

### 9a — RESOLVED: unpinned `at-uri`, not `strongRef`

**Founder call: "I played at this party."**

I had proposed `com.atproto.repo.strongRef` — `{uri, cid}` — with SET-8 written to make a broken pin survivable. That was the weaker choice, for a reason I did not surface when I framed the fork:

**Adding an optional field to a lexicon later is non-breaking. Relaxing a required sub-field is not.**

`strongRef` *requires* `cid`. Choosing it would have forced every writer to resolve the promoter's record and obtain its cid before writing their own set — a network dependency on the write path, for a field that is enrichment. And backing that requirement out later would break every record already written. Choosing the bare `at-uri` first leaves the door open in the direction that opens: an optional `eventCid` can be added at any time, additively, without invalidating a single existing set.

The founder took the reversible door. The lead did not.

**What is given up:** tamper-evidence. A promoter can rename, move, or cancel the event after the fact, and no consumer can detect from the set record that anything changed. This is accepted, and it is accepted because it is *correct*, not merely cheap:

> A set is a **backward-looking attestation of presence**. An RSVP is a **forward-looking commitment to a description**.

`community.lexicon.calendar.rsvp` pins its `subject` with a `strongRef`, and should — you are agreeing to attend the event *as described*. We deliberately diverge from that convention, in the same namespace, because the tense is different. The performer was there. What the party was called is the promoter's to revise, and revising it does not make the performer's claim false.

### 9b — RESOLVED: rename, do not replace. And the premise was wrong.

I framed 9b as *"aligning buys the whole `community.lexicon.location.*` union for free."* That was false, and reading the remaining three defs is what showed it.

**The union is not ours to carry.** `locations` is a field on `community.lexicon.calendar.event` — the *promoter's* record. `eventUri` already reaches it. There was never anything to buy.

**And adopting the union would have broken SET-8.** All four defs — `address`, `geo`, `fsq`, `hthree` — declare `name` as an *optional* string; `address` requires only `country`. A set could then legally carry `{fsq_place_id: "..."}` and nothing else, and would not render offline. `venue` is load-bearing. The union cannot carry that load.

So: **rename the fields, keep the def.** `venueName` → `name`, `countryCode` → `country`, plus `street` and `postalCode` picked up for free. `venueUrl` stays as a marked skaists extension; address consumers ignore it. Verified mechanically — six shared fields, one extension, no unmet `address` requirement.

This is the same decision, for the fourth time: **mirror the shape, profile it stricter, never cross-namespace-ref a load-bearing field.** `playRef` mirrors `playView`. teal wraps foreign records in `playView`. Spark wraps `fm.plyr.track` in `AudioView`. The convention was never ours to invent; we keep rediscovering it.

Incidental confirmation of **SET-6**: `community.lexicon.location.geo` stores `latitude` and `longitude` as **strings**. They hit the lexicon-v1 no-float wall independently and took the string escape. We took milli-integers for `bpmMilli`. Both are right: coordinates need arbitrary precision and are never range-compared inside a record; BPM is sorted and filtered, so it must remain numerically ordered.

### Deferred, not decided

An **H3 cell** (`community.lexicon.location.hthree`) on `#venue` would let an underground venue publish coarse proximity without publishing an address. That is a real want — unlisted parties have no event record, so `venue` is all they have — and it is a **privacy** control, not a location feature. It is also purely additive, so the door opens in the direction that opens. **Not added.** It deserves its own gate, argued on privacy grounds, not smuggled in under a rename.

Noted for whoever files upstream: `community.lexicon.location.fsq` uses `fsq_place_id`, snake_case, in an otherwise camelCase corpus.

---

## 10. Unrelated find, directly load-bearing for bQueenBee

`community.lexicon.preference.ai` (pinned above, MIT) is a record by which a user declares, in their own repository, what AI systems may do with their public data. Four independent switches — `training`, `inference`, `syntheticContent`, `embedding` — each an explicit boolean with its own `updatedAt`. Scopes are global (`key: self`), per-entity (a specific DID or domain), or per-collection (a specific NSID).

An omitted field means *no declared preference*, not *consent*.

If bQueenBee holds a DID and reads the network, this record is the network telling her what she may do. `entityScope` means a user can name her DID specifically and deny her alone. `collectionScope` means a user can permit her to read their `performance.set` records while denying her their posts.

This is a mechanism for honoring a preference that already exists whether or not we read it. I would treat honoring it as a launch gate, not a feature. **Not drafted, not scoped, not decided.**


---

## 11. Vocabularies — resolution

**No tokens.** `knownValues` everywhere, governed by two rules instead.

Tokens are NSIDs. Minting one is a governance act, and their only structural payoff is discriminating a `union`. We union nothing. A token per dance move buys a maintenance burden and no type safety.

**Rule V-1 — Closed where code branches; open where humans read.**
`transition.technique` is display-only. It stays open forever, values preserved verbatim, never coerced, never migrated. `setKind` and `#performer.kind` are branched on, so they get the escape rule below rather than a closed set.

**Rule V-2 — A required `knownValues` field MUST carry an explicit escape value.**
`undisclosed` or `unknown`, present in the vocabulary. This is now enforced by the schema verifier, not by taste. It caught `#performer.kind`, which was required with no escape and would have forced every writer to assert `human` or `machine` when the truthful answer is sometimes neither.

The danger a closed vocabulary hides is that **absence gets a default**, and the default is always the flattering one. `human` for agency. `live` for kind. V-2 makes the unflattering answer sayable so silence never has to be interpreted.

## 12. Set mutability — resolution

**Append-only. Superseding. Content-pinned. Never deleted.**

`supersedes` is a `com.atproto.repo.strongRef`. This looks like a contradiction of §9a, and it is the opposite of one — it is the same rule applied honestly:

| Reference | Whose record | What is claimed | Form |
|---|---|---|---|
| `eventUri` | someone else's | *I was present* | bare `at-uri` |
| `supersedes` | **your own** | *this replaces that content* | `strongRef` |

Pinning the promoter's event forced a network round-trip on the write path to assert something the pin did not support. Pinning your own predecessor costs nothing — the cid is local — and the claim being made is **explicitly about the prior record's content**. Where the cid is free and the claim is about bytes, pin. Where the cid is expensive and the claim is about presence, don't.

teal punts this: `actor.status` instructs clients to *"delete or tombstone earlier records."* We do not. A tracklist a performer corrected a week later is evidence, and the correction is more trustworthy for the original still standing beside it.

## 13. Kernel binding — constraint recorded, question still open

§7.5 stays gated. Whether a `set` is a `CanonicalEventV1`, and whether it is an **Event** or an **Evidence**, is a constitutional question.

Two constraints are now recorded, because they are not constitutional and they bind whatever the answer turns out to be:

**K-1 — `CanonicalEventV1` never carries media, on any path.**
Not bytes, not blobs. If a `set` is ever promoted into the kernel, what crosses is the **manifest digest** — never the manifest, and never the recording. The set record was designed as a manifest for exactly this reason: `playRef` carries `isrc`, `trackMbId`, `duration`, `bpmMilli`, and no audio. Media lives on Autonomi, anchored to Arweave. Nothing about a two-hour recording belongs on a deterministic indexing path.

**K-2 — `setStatus` never enters the kernel.**
It is ephemeral state with a ~10-minute expiry, mutated in place, possibly every few minutes for hours. Feeding that to a deterministic indexer is unbounded write amplification for zero durable claim. Status is state. Sets are attestations. **Only attestations may be promoted.**


---

## 14. Decision log

Every entry below is a founder call, recorded so it is not re-litigated. Re-opening any of them requires a new version of this document, not a conversation.

| # | Decision | Call | Date |
|---|---|---|---|
| §7.1 / §9 | Event linkage | Adopt `community.lexicon.calendar.event`. Neither mint nor natural-key. | 2026-07-09 |
| §9a | `eventUri` form | **Bare `at-uri`, not `strongRef`.** A set attests presence, not description. Reversible door. | 2026-07-09 |
| §7.2 | Live status | Draft `performance.setStatus`. Distinct collection; no teal collision. | 2026-07-09 |
| §7.3 / §11 | Vocabularies | **No tokens.** `knownValues` + rules V-1 and V-2. | 2026-07-09 |
| §7.4 / §12 | Set mutability | **Append-only, superseding, content-pinned, never deleted.** `supersedes` is a `strongRef` because the cid is local and the claim is about content. | 2026-07-09 |
| §9b | `#venue` | **Rename, do not replace.** Strict profile of `community.lexicon.location.address`. The union was never ours to carry. | 2026-07-09 |
| §7.5 | Kernel binding | **CARRIED OPEN.** Constraints K-1 and K-2 recorded; the constitutional question is untouched. | — |
| §9b-def | `PRIV-1` (H3 venue) | **DEFERRED, unfiled.** Additive, therefore free to defer. Must be argued on privacy grounds. | — |

### Corrections the lead made to his own work

Recorded because a ledger that only lists the founder's calls is a flattering one.

- Framed §9a as a tradeoff between tamper-evidence and edit-tolerance, omitting that `strongRef` forces a network round-trip on the write path and that relaxing a required sub-field is breaking. The founder took the reversible door; the lead had not identified it as one.
- Asserted that aligning `#venue` "buys the `location.*` union for free." False. The union lives on the promoter's event record and `eventUri` already reaches it. Discovered only by reading `geo`, `fsq`, and `hthree`, which had not been read when the gate item was written.
- Wrote `#performer.kind` as required with no escape value, which would have forced every writer to assert `human` or `machine` when the truthful answer is sometimes neither. Caught by the verifier, not by the author.
- Cited `STATUS-2` and `STATUS-3` from `setStatus.json` without defining them, and offered the document for approval in that state. Caught by the reference check, one step before the stamp.
