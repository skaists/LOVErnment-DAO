⟨Research · D-1 fractally whitepaper dossier · PRIMARY-PINNED · re-stamped 2026-07-07⟩

# D-1 — ƒractally governance parameters (primary-source-pinned)

**PROVENANCE PIN.** *Fractally White Paper 1.0 (English)* — Daniel Larimer & the Freedom
Engineers at ƒractally. Delivered as a file 2026-07-07; **PDF sha256
`efe0698dd0d1ad16797878e98ed98b6470d39584f308575f2be6a119a7663696`** (7,735,268 bytes);
text extracted via `pdftotext -layout` (1869 lines). Citations give the white-paper **section
title** (stable) + the extraction line as a retrieval aid. **This version supersedes the
blog/secondary values in the prior draft — two of which the primary source proves wrong (see §3).**

## 1. Weekly consensus — circle size & remainder handling  *(§"Group Size", §"Respect Distribution")*
- **Target group size 6, with some groups of 5.** Not all groups can be exactly 6 (pigeonhole);
  "up to 5 groups may only have 5 members" per round; a **5-member group ranks 2–6 and awards no
  rank 1.** *(extraction ~L710, L888–892.)*
- **Rationale (pinned):** changed from Eden's 5 to **6** to stop a 40% party holding a 60% majority
  hostage; targets BFT-67%; group sizes **5–6 are ideal** (beyond 5 tends to "monolog"), citing
  *Fay, Garrod & Carletta 2000, "Group Discussion as Interactive Dialogue or as Serial
  Monologue."* *(extraction L855–900.)*

## 2. In-circle consensus threshold  *(§"Group Size"; §"ƒractally Consensus Building")*
- **4 of 6, or 3 of 5** (per the two live group sizes). "It always takes at least **50%** to abort a
  consensus and at least **60%** to reach one, with the vast majority of the time requiring **67%**."
  A group that fails consensus → **everyone in it receives rank 0.** *(extraction L901–904, L781, L1707.)*

## 3. Respect award schedule per rank (Fibonacci) — **CONFLICT RESOLVED; both secondaries corrected**
- **Round 1, group of 6 (rank 1 = least → rank 6 = greatest):**
  **`2ℝ, 3ℝ, 5ℝ, 8ℝ, 13ℝ, 21ℝ`** — Fibonacci **starting at 2.** *(§"Respect Distribution",
  extraction L695–706.)*
- **Round 2** (top-ranked re-grouped; **cumulative** totals, ranks 6–11): `21, 34, 55, 89, 144, 233`;
  **Round 3:** `233, 377, 610, 987, …`. *(extraction L727–760.)*
- **Correction of the prior draft's conflict:** the blog's `1,2,3,5,8,13` was a **shifted/incorrect**
  restatement (real Round-1 is `2,3,5,8,13,21`); the `55→5` figure was a **misread of the Round-2
  cumulative table** (`55` = Round-2 Rank-8) and/or the separate market-maker sponsorship table.
  **Neither secondary matched the primary; the canonical Round-1 schedule is `2,3,5,8,13,21`.**
- **Distribution shape (corrected):** **"16% of first-cycle participants earn about 40%"** — a
  *softer* 80/20 Pareto. (The prior draft's "~33% earn ~66%" from the blog is **superseded**.)
  *(§"Respect Distribution", extraction L716–718.)*

## 4. Fractal rounds (council election)  *(§"Weekly Consensus Meetings")*
- Top-ranked contributors are re-grouped into new groups of 6 each round; the process **continues
  until fewer than 6 people remain, or up to 5 rounds.** *(extraction L581, L769.)*
- Design ceiling: *"governance with up to **7,776 members** governed by a Council"* — note
  **7,776 = 6⁵**, i.e. five rounds of six. *(extraction L1787.)*

## 5. Team-fractal rules  *(Genesis agreement + §"Allocating a Team's Respect", §"Leaving a Team")*
- **Formation:** reply `propose team: @a @b @c …`; forms if **all named members upvote** and the team
  has **≥4 and ≤12** members. *(pinned: Genesis Fractal Contributor Agreement, hive.blog/@dan.)*
- **Team Respect allocation (primary):** a team earns **matching Respect** for members' Respect;
  any member proposes a transfer, votes collected over **72 hours**, passes if **⅔ of votes cast
  approve** (so an unopposed single vote suffices). A **rejected** proposal **burns the proposer's
  escrowed Respect.** *(extraction L959–969.)*
- **Leaving a team:** mandatory **20-week waiting period** before removal/eligibility to re-join —
  intentional friction. *(extraction L974–977.)*

## 6. Council / averaging windows & cadence  *(§"Media Rewards" vote weight; Council sections)*
- **Cadence: weekly** consensus meetings; the **governing Council sets the meeting day.**
  *(extraction L678.)*
- **Vote-weight averaging window: 12 weeks.** "This weekly ranking is **averaged over 12 weeks** to
  provide a more stable vote weight"; each week voting weight **decays 5% or 1 unit (whichever is
  greater)** and grows by that week's rank. *(§"Media Rewards", extraction L1057, L1063–1064.)*
- **Council composition & authority:** "the council is composed of the top teams based on a
  **20-week average**"; Council has **12 members** and acts with **8 of 12** (⅔) approval on
  contract actions. *(extraction L1440, L1480.)*

## Summary table — all PRIMARY-PINNED @ *Fractally White Paper 1.0*
| Parameter | Value | Source (extraction line) |
|---|---|---|
| Circle size | 6, some groups of 5 | Group Size (L888) |
| Remainder handling | 5-groups rank 2–6, no rank 1 | Respect Distribution (L710) |
| Consensus | 4 of 6 / 3 of 5 (≥60% reach, ≥50% abort) | Group Size (L901) |
| Respect Round-1 | **`2,3,5,8,13,21`** | Respect Distribution (L695) |
| Respect Round-2 (cum.) | `21,34,55,89,144,233` | (L727) |
| Distribution | 16% earn ~40% (softer Pareto) | (L716) |
| Fractal rounds | until <6 / up to 5; max 7,776 = 6⁵ | (L769, L1787) |
| Team allocation | ⅔ of votes / 72h; reject burns escrow | Allocating a Team's Respect (L959) |
| Team leave | 20-week wait | Leaving a Team (L974) |
| Cadence | weekly (Council sets day) | (L678) |
| Vote-weight averaging | **12 weeks** (decay 5%/wk) | Media Rewards (L1063) |
| Council | top teams by 20-wk avg; 12 members, 8/12 | (L1440, L1480) |

**All prior UNVERIFIED items (remainder algorithm, averaging window, council term) are now closed
against the primary source, and the §3 Respect-schedule conflict is resolved with both secondary
values corrected.** Nothing in D-1 now rests on a secondary for a numeric governance parameter.

## 7. Secondary corroboration & cross-lineage note (folds, 2026-07-07)
- **Ecency `@pnc` post — FOUNDER-RELAYED SECONDARY (not machine-fetched this session):**
  corroborates the weekly consensus meetings, the Fibonacci-as-softer-Pareto framing, that Respect
  is **not token/wealth-weighted**, and a **Contribution Agreement** signing requirement.
  *(ecency.com/fractally/@pnc.)*
- **Margin note — cross-lineage signing gate (analytical):** all three governance ancestors gate
  membership on **signing a founding document** — fractally's **Contribution Agreement**, Eden's
  **Peace Treaty** at induction (D-2 §1), and **eosDAC's constitution-on-registration**
  (**LEAD-PINNED:** eosdac.io, fetched 2026-07-07, CD-17 context). A signing-gate pattern
  **skaists' induction inherits.**

Sources: **PRIMARY** — *Fractally White Paper 1.0 (English)*, Larimer et al. (file, sha256 `efe0698d…`,
extracted 2026-07-07). Secondary/context (retrieved 2026-07-06 unless noted):
[Genesis Fractal Contributor Agreement (hive.blog/@dan)](https://hive.blog/fractally/@dan/genesis-fractal-contributor-agreement) ·
[eosDAC constitution (eosdac.io, lead-pinned 2026-07-07)](https://eosdac.io) ·
[ecency.com/fractally/@pnc (founder-relayed secondary)](https://ecency.com/fractally/@pnc) ·
[fractally.com — Introducing Fractally](https://fractally.com/blog/introducing-fractally) (superseded on §3 by the primary).
