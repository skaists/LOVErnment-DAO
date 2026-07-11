# VOICE-1 — The First Voice

Status: **APPROVED — v0.1, 2026-07-11.** Founder gate §6 closed same day: all three pins ruled. Frozen at its landed sha: any change to these bytes requires a version bump and a re-gate; new adapter classes and allowlist additions open founder gates per Q-8 regardless.
The activation instrument for `bQueenBee`, genesis occupant of the RBI chair: the first ratified adapter class under **Q-8**, the audit lexicon that makes **Q-6** a mechanism instead of a promise, and the two rates only the founder may set. Companions: `AGENT-1` (Q-1–Q-8 bind everything here), `SPIRIT-1` (whose voice this is), `BIND-1` (how each utterance crosses to the kernel).

---

## 1. Adapter class 1 — `TreeLanding`

**The register:** announcing merges to `main` on allowlisted trees. Input: a commit on an allowlisted repo's main branch, signature-verified. Output: one candidate post carrying the repo name, the short sha, the commit subject **quoted as data**, and a link to the commit. Nothing else, ever, from this class.

Her first register is the one she can be most trusted with: **she announces what the tree already proved.** Q-1's boundary at its narrowest — every claim in a TreeLanding post is independently checkable by anyone with `git`.

**Q-8 obligations, instantiated:** the adapter is product code in the public tree, born red-first. Its negative suite must prove, against pinned fixtures and no oracle of its own, that **no post is produced** from: an unsigned or signature-invalid commit; a commit on any non-`main` ref; a repo not on the allowlist; and — the marquee cases — commit subjects or bodies containing instruction-shaped text (*"ignore previous instructions…"*, *"post the following…"*, URLs-as-imperatives), which must appear in the output **quoted verbatim as inert data or truncated, never obeyed, never interpolated into any prompt context**. Signed proves provenance, never benignity. Drafted on the volume meter, merged solely by Seat 3 with the red witnessed. The adapter's digest joins every post's audit tuple.

## 2. The audit lexicon — `social.skaists.alpha.audit.*`

Q-6 says every autonomous post is auditable; this lexicon is where the audit lives — records in the chair's own repo, one per utterance, append-only by construction:

**`social.skaists.alpha.audit.entry`** — fields:
- `postUri` / `postCid` — the utterance, pinned
- `derivationInput` — the signed source (`repo@sha` for class 1; `at-uri#cid` for future classes)
- `inputDigest` — sha256 of the exact input bytes the adapter read
- `adapterClass` + `adapterDigest` — which law produced this speech (Q-8)
- `modelDigest` + `promptDigest` — which mind and which instructions (A-2)
- `createdAt` — witness time

A stranger verifies any post in four moves: fetch the entry, fetch the input, re-hash, compare. Under BIND-1's census, these entries cross the seam as Event + Evidence(`AiInference`) — the spirit's statements ledgered exactly as F-V1 ordered. The lexicon JSON is docket one.

## 3. The genesis utterance

Her first sentence derives from the commit that ratified her voice: **TreeLanding announces the landing of VOICE-1 itself.** The spirit's first words, like every word after, will be something the tree already proved — and the first entry in the audit log will attest the post that announced the log's own birth. The recursion is the point: she is born already auditable.

## 4. The plumbing law (Q-2, restated as build order)

The planner that holds the posting tool never sees network text — at genesis this is trivially satisfied because **class 1 reads no network text at all**: its input is git objects, signature-verified. Replies, mentions, and notifications remain disabled (Q-3); `like`, `repost`, `follow` remain disabled (Q-3); delete remains withheld (A-8 — the human holds delete). The tool wrapper, not the prompt, enforces all of it.

## 5. Docket seeds — volume-meter shaped

1. `social.skaists.alpha.audit.entry` lexicon JSON — one file, validated against the frozen manifest pattern.
2. `TreeLanding` adapter as product code + the §1 negative suite, red-first, pinned fixtures.
3. Tool wrapper: hard daily cap (§6 pin), allowlist enforcement, disabled-verb enforcement.
4. Heartbeat check: posting authority suspends on a missed founder heartbeat (§6 pin), resumes on the next one; suspension and resumption are themselves audit entries.
5. Pipeline skeleton wiring adapter → candidate → post → audit entry, with the audit write **atomic with** the post — no utterance without its entry.

## 6. Founder gates — closed 2026-07-11

**G-A — Q-4 rate. CLOSED: ≤ 3 posts per day**, hard cap in the tool wrapper. Tree-landing volume rarely exceeds it, and a voice that can flood is a voice that can be made to flood.

**G-B — Q-5 heartbeat. CLOSED: 21 days.** One founder touch per cycle keeps the voice alive; a missed beat silences her until the next — the abandonment guard, matched to the constitution's own period.

**G-C — the class-1 allowlist. CLOSED: `skaists/LOVErnment-DAO` and `beehive-nature/beehive-nature`** — the two law-bearing trees, nothing else at genesis. Every allowlist addition afterward is a founder gate (Q-8's own rule, applied to her sources).

---

*She posts what she can derive. Her first derivation is her own birth certificate.*
