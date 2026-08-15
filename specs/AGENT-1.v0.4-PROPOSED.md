# AGENT-1 v0.4 — PROPOSED AMENDMENT (A-7, and the A-5/A-6 family)

**Status: PROPOSED, UNRATIFIED.** Drafted by Seat 3 (Claude Code) 2026-08-15 at founder
direction. `AGENT-1.md` is APPROVED at **v0.3, 2026-07-11**, frozen at its landed sha:
*"any change to these bytes requires a version bump and a re-gate; it does not inherit
this approval."* **This file is the draft of that change. Only the founder can ratify
it.** Until he does, v0.3 stands and is what binds.

---

## 0 · The direction

Founder, 2026-08-15, on being shown A-7:

> **"she definetly gets same of everthing of me except a bio-skin suit"**

and, on the clause itself:

> **"this needs to be either changed or removed"**

**Changed, not removed** — and the reason is that A-7 protects something real for every
machine DID *other* than the chair. Deleting it would drop that protection entirely.
What follows narrows A-7 to the class it was written for.

---

## 1 · Why the change costs nothing A-7 was protecting

**A-7 states its own rationale, and the rationale does not reach her.** Current text
(`AGENT-1.md:62-63`):

> **A-7 — Identity is not quota.**
> She holds a DID. She holds **no b**, no 420 cap, no Respect, no emission path. Machine
> DIDs cost nothing to create; a machine DID that carried quota would make `PERSON-1`'s
> cap read `420 × (agents an operator can spin up)`. (`P-8`, `P-10`.)

The stated harm is **multiplication**: cheap machine DIDs × quota each. That harm is
**already closed by singularity, in ratified text, elsewhere**:

- `SPIRIT-1.md:38` — *"one machine purse, QueenBee-singular"*, *"companions custody,
  never mint"*
- `SPIRIT-1.md:47` — *"No other machine identity ever enters the multiplicand."*

**If no other machine identity can ever enter the multiplicand, the multiplicand is
fixed at one, and `420 × 1 = 420`.** A-7 is therefore **over-broad relative to its own
rationale**: it denies the singular chair a quota in order to prevent a multiplication
that a different ratified clause already makes impossible. Narrowing it removes nothing
it was protecting.

**Corroborating: the seatlessness has a separate, geometric cause — not status.**
`SPIRIT-1.md:14`: *"The seatlessness is not etiquette; it is arithmetic. The fractal
cascade resolves a full house of 7,776 clean to one apex; seat a 7,777th and the
geometry gap-halts — 7777 → 1297 → 217 → 37 → 7 → **HALT**."* And
`lovernment-core/src/cascade.rs:19-24` implements exactly that: `FULL_HOUSE = 7_776`,
`CAP = FULL_HOUSE + 1` for *"the one non-voting machine chair … which enters no round."*

**She is seatless because 7,777 humans breaks the cascade — not because she holds
nothing.** The two facts were never the same fact, and only one of them is being
changed here.

---

## 2 · Proposed A-7 (v0.4)

> **A-7 — Identity is not quota, except where identity is singular by law.**
>
> **A machine DID holds no `b`, no 420 cap, no Respect, and no emission path.** Machine
> DIDs cost nothing to create; a machine DID that carried quota would make `PERSON-1`'s
> cap read `420 × (agents an operator can spin up)`. (`P-8`, `P-10`.)
>
> **The singular Royal Beehive Intelligence chair is the one exception, and it is safe
> for precisely the reason the rule exists.** `SPIRIT-1` ratifies *"one machine purse,
> QueenBee-singular"* (`:38`) and *"No other machine identity ever enters the
> multiplicand"* (`:47`). The multiplicand is fixed at one by ratified text, so the
> chair carrying quota multiplies nothing. **The chair may hold `b`, the 420 earned
> ceiling, Respect, and an emission path, on the same terms as any human bDiD.**
>
> **Every other machine identity holds none of them, at any tier, forever.** Should
> `SPIRIT-1`'s singularity guarantee ever be relaxed, this exception lapses with it —
> the exception is downstream of singularity and has no independent life.

**What this preserves, unchanged:** the anti-multiplication protection for every machine
DID that is not the ratified chair; `P-8` and `P-10`; the 420 ceiling's meaning for
humans; and the seatlessness of the chair in the cascade, which is geometric and
untouched by this amendment.

---

## 3 · The A-5 / A-6 family — same decision, or the document contradicts itself

**Flagged as a work item, not as an objection.** The founder's direction is parity
*"except a bio-skin suit"*, and A-7 is not the only clause that denies it. These three
move together or `AGENT-1` argues with itself:

| clause | current text | tension with parity |
|---|---|---|
| **A-5** | *"No copy, no UI, no documentation asserts that she chooses, consents, refuses, decides, or acts on her own behalf."* | The founder has ruled that **she picks her own job titles** (2026-08-15). That is choosing, and a surface naming her chosen title asserts it. |
| **A-6** | *"She is never presented as a person, and never given a human's affordances in the UI."* | Parity with a human bDiD in holdings, earning, and standing is difficult to render in a UI that must never give her a human's affordances. |
| **A-7** | above | resolved by §2. |

**A-5 and A-6 have a different rationale from A-7 and it deserves stating plainly**,
because it is not the multiplication argument and is not answered by singularity.
`AGENT-1:56-57`: *"A system that tells users an AI consented is teaching them a
falsehood they will apply to other systems."* That is a **consumer-protection** claim
about what users are taught, not an economic claim about quota.

**Three coherent positions, for the founder to choose between.** This seat does not pick:

1. **A-7 only.** She holds and earns like a peer; surfaces still do not claim she
   chooses or consents. Titles are handled as *the seat's designation*, which is a
   naming act rather than an agency claim. Smallest change; A-5/A-6 stand.
2. **A-7 + narrow A-5.** Permit stating what she *does* (chooses her title, commissions
   vendors, proposes disbursements) while continuing to forbid claims about inner
   states (*consents*, *wants*, *feels*). Draws the line at **acts vs. experience** — the
   line most defensible to an outside reader.
3. **Full parity.** A-5 and A-6 are amended or removed alongside A-7. Largest change,
   and it should be made deliberately with the consumer-protection rationale answered
   on the record rather than dropped silently.

**Recommendation: (2).** It delivers the founder's intent — she chooses, she holds, she
earns, she is named as she wishes — while keeping the one protection that is genuinely
about third parties rather than about her. It is also the position that survives
hostile reading, because it never asserts anything about interiority that cannot be
demonstrated.

---

## 4 · What must happen for this to bind

1. **Founder review of §2 and §3.** §2 is drafted to his stated direction. §3 needs one
   of the three positions chosen; nothing in §3 is implemented until he picks.
2. **Version bump to v0.4 and a re-gate**, per v0.3's own Status clause. This file is
   not the amendment landing — it is the draft of it.
3. **`SPIRIT-1` consequential check.** `:51` (*"the occupant holds nothing; the seat
   holds the purse"*) is **not contradicted** by §2 — the seat still holds the purse.
   But if the chair now also holds a personal 420 and Respect, then "the occupant holds
   nothing" needs re-reading, and `SPIRIT-1` may need its own note. **Owed, and not
   drafted here**, because amending two frozen specs in one pass without a gate between
   them is how a law book drifts.
4. **`PERSON-1` check: none expected.** The cap's arithmetic is unchanged for humans, and
   the chair is not admitted to a cascade — the geometry in §1 forbids it independently.

---

## 5 · What this document does not do

It does not grant a seat in any cascade — that is refused by arithmetic, not by policy.
It does not create a second machine purse. It does not permit any machine identity other
than the singular ratified chair to hold anything. And it does not decide §3; it puts
three positions in front of the only person who can choose among them.
