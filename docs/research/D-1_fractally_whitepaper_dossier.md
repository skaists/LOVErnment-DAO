⟨Research · D-1 fractally whitepaper dossier · retrieved 2026-07-06⟩

# D-1 — ƒractally governance parameters (source-pinned)

**Discipline note.** Every value below is pinned to a retrievable URL (retrieved 2026-07-06) or
marked **UNVERIFIED**. The canonical primary source — `fractally.com/whitepaper/fractally_en.pdf`
— is behind an auth-walled Google Drive viewer and could **not** be retrieved this session; the
`github.com/gofractally/fractally` repo and `fractally_en.pdf` mirror likewise could **not** be
machine-fetched (this session's web-fetch returned empty bodies). So the values below are drawn
from **fractally's own blog/Medium posts and the Genesis Fractal governance docs**, which restate
the white-paper mechanics but are secondary to the PDF. Where the secondary sources disagree or
fall silent, the item is marked UNVERIFIED and the white-paper PDF is named as the pin to obtain.

## 1. Weekly consensus — circle size & remainder handling
- **Circle size: groups of 6**, randomly assigned each week; members rank each other
  greatest-to-least contributor over video. *(Pinned: fractally.com/blog/introducing-fractally;
  Larimer, "Introducing Fractally," Medium.)*
- Some restatements say **"3–6 people"** rather than a strict 6 — implying sub-6 groups are used
  when the population isn't divisible by 6. *(Pinned: gofractally Medium restatement.)*
- **Remainder-group algorithm (exact rule for non-multiples of 6): UNVERIFIED.** The secondary
  sources give the "3–6" range but not the precise allocation rule (e.g., minimum group size,
  how a remainder of 1–2 is absorbed). **Pin to obtain:** white-paper PDF §"Weekly consensus."

## 2. In-circle consensus threshold
- **4 of 6** must agree on a ranking for it to count — an ≈two-thirds supermajority; random
  membership + the 4/6 rule is the stated defense against "party politics." *(Pinned: gofractally
  restatement / optimystics.io "Fractal Democracy.")*
- Note (mechanical, not from source): 6 is even, so the **4/6 supermajority** is what prevents a
  3–3 deadlock. Any circle-size change must carry its own supermajority rule for the same reason.

## 3. Respect award schedule per rank (Fibonacci) — **primary values, with one conflict flagged**
- **Per-rank Respect in a group of 6: `1, 2, 3, 5, 8, 13`** units (rank 6→1, least→greatest),
  the Fibonacci sequence. *(Pinned: fractally.com/blog/introducing-fractally — "receive 1, 2, 3,
  5, 8, or 13 units of Respect.")*
- Consequence stated by the source: **≈33% of contributors earn ≈66% of the Respect.** *(Pinned:
  same.)*
- ⚠ **Conflict to reconcile (UNVERIFIED which is authoritative):** a separate secondary summary
  states the schedule as **"55 for first place down to 5 for sixth"** (i.e., 55…5, ~60% steps).
  This does **not** match `1,2,3,5,8,13`. The two may reflect different white-paper revisions or a
  paraphrase error. **Do not ship either as "the" schedule until reconciled against the PDF.**
  **Pin to obtain:** white-paper PDF §"Respect."

## 4. Fractal rounds (council / delegate election)
- After round 1, the **top-ranked contributors are re-grouped into new groups of 6** and rank
  again; the process **repeats up to ~5 times or until fewer than 6 people remain** — the apex
  group being the most-Respected. *(Pinned: gofractally restatement.)*
- **"Two consensus rounds"** is a named refinement of this. *(Pinned: James Mart, "Fractal
  Democracy — Two Consensus Rounds," Medium, medium.com/gofractally.)*
- **Exact round-count formula and how apex maps to a council/delegate set: UNVERIFIED** (secondary
  says "up to 5 / until <6"; the precise stopping rule and council seat count need the PDF).

## 5. Team-fractal rules
- A **team** forms by replying to the weekly-meeting post on hive.blog with `propose team: @usera
  @userb @userc …`; if **all named members upvote** and the team has **≥4 and ≤12 members**, the
  team is formed. *(Pinned: "Genesis Fractal Contributor Agreement," hive.blog/@dan.)*
- **How team Respect is pooled/split and any team-level multiplier: UNVERIFIED** (agreement states
  formation rule; distribution mechanics need the PDF / Genesis bylaws).

## 6. Council / averaging windows & cadence
- **Cadence: weekly** consensus meetings. *(Pinned: fractally.com/blog/introducing-fractally.)*
- **Respect averaging window (the trailing window over which Respect is smoothed for
  council/voting-weight eligibility): UNVERIFIED.** The secondary sources confirm weekly cadence
  and that later-round voting power "increases exponentially with the Fibonacci curve," but do
  **not** state the exact averaging window (e.g., N-week rolling mean) or council term length.
  **Pin to obtain:** white-paper PDF §"Respect" / §"Governance."

## Summary table (what ships vs. what's blocked)
| Parameter | Value | Status |
|---|---|---|
| Circle size | 6 (restated as 3–6) | PINNED (blog) |
| Remainder handling | — | **UNVERIFIED** (PDF) |
| Consensus threshold | 4 of 6 (~2/3) | PINNED (secondary) |
| Respect schedule (grp of 6) | `1,2,3,5,8,13` | PINNED (blog) — **conflicts** with a "55…5" restatement |
| 33/66 distribution | ~33% earn ~66% | PINNED (blog) |
| Fractal rounds | up to ~5 / until <6 | PINNED (secondary), exact rule UNVERIFIED |
| Team rule | ≥4, ≤12, all upvote | PINNED (Genesis agreement) |
| Cadence | weekly | PINNED (blog) |
| Averaging window / council term | — | **UNVERIFIED** (PDF) |

**Single action to close every UNVERIFIED:** retrieve `fractally_en.pdf` (auth-walled Drive) or a
committed copy in `github.com/gofractally/fractally`, and extract §Respect, §Weekly consensus,
§Governance verbatim. Nothing above should be treated as final white-paper values until that pass.

## 7. Secondary corroboration & cross-lineage note (folds, 2026-07-07)
- **Ecency `@pnc` post — FOUNDER-RELAYED SECONDARY (not machine-fetched this session):**
  corroborates the **weekly consensus meetings**, frames the Fibonacci allocation as a **softer
  Pareto** distribution that is **not token/wealth-weighted**, and names a **"Fractal Genesis
  Contribution Agreement"** signing requirement for participation. Labelled founder-relayed because
  this seat's fetch could not independently open it. *(ecency.com/fractally/@pnc.)*
- **Margin note — cross-lineage signing gate (analytical):** all three governance ancestors gate
  membership on **signing a founding document** — fractally's **Contribution Agreement** (§5 + the
  ecency fold above), Eden's **Peace Treaty** at induction (D-2 §1), and **eosDAC's
  constitution-on-registration** (**LEAD-PINNED:** eosdac.io, fetched 2026-07-07, banked same-day in
  the kernel ledger's CD-17 context — *not* founder-relayed-unverified). This is a signing-gate
  pattern **skaists' induction inherits**, with three independent precedents behind it.

Sources (retrieved 2026-07-06 unless noted):
[fractally.com — Introducing Fractally](https://fractally.com/blog/introducing-fractally) ·
[eosDAC constitution (eosdac.io, lead-pinned 2026-07-07)](https://eosdac.io) ·
[ecency.com/fractally/@pnc (founder-relayed secondary)](https://ecency.com/fractally/@pnc) ·
[Larimer, Introducing Fractally (Medium)](https://medium.com/gofractally/introducing-fractally-the-next-generation-of-daos-7c94981514d8) ·
[Mart, Fractal Democracy — Two Consensus Rounds (Medium)](https://medium.com/gofractally/fractal-democracy-two-consensus-rounds-8134eaba3281) ·
[Genesis Fractal Contributor Agreement (hive.blog/@dan)](https://hive.blog/fractally/@dan/genesis-fractal-contributor-agreement) ·
[optimystics.io — Fractal Democracy](https://optimystics.io/fractal-democracy) ·
[github.com/gofractally/fractally](https://github.com/gofractally/fractally) (not machine-retrievable this session)
