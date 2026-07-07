⟨Research · D-2 Eden (gofractally/Eden) dossier · contract-pinned · re-stamped 2026-07-07⟩

# D-2 — Eden on EOS: induction, elections, delegate budget (source-pinned)

**PROVENANCE PIN.** All election/distribution values below are pinned to `gofractally/Eden @
2d779d476f8bb6bc14dc30eadae9f7d70264b6fc` (dormant head, stable), files:
`contracts/eden/include/elections.hpp`, `contracts/eden/src/elections.cpp`,
`contracts/eden/src/distributions.cpp`, `contracts/eden/include/distributions.hpp`.
Source files sha256-verified on receipt (2026-07-07): elections.hpp `4c7a83…6f6fd` (305 ln),
elections.cpp `932976…c2c7f7` (1111 ln), distributions.cpp `252696…d211a3` (482 ln),
distributions.hpp `720170…f23c5f` (132 ln) — all MATCH the relayed digests.

**Methods / verification layering (carried per lead ruling).** The induction facts are pinned to
contemporaneous public guides (the 4 contract files do not cover induction). The election/budget
values were promoted derived→pinned only after: (a) each file's sha256 matched; (b) the 7,777
example was **re-derived independently from `count_rounds`+`get_group_sizes`**, not inherited; and
(c) the consensus cancellation was **confirmed against full `elections.cpp` context (~640–700)**.
Note the layering: the round *count* is re-derivable from the stated `12·4^k` rule alone, but the
per-round size vector and the consensus arithmetic required reading the source — done here.

## 1. Induction ceremony — steps & exact requirements *(pinned to public guides; not in the 4 contract files)*
- **Invitation:** invited by **1 existing member**; **2 additional members vouch as witnesses**.
- **Ceremony:** **4 participants** (inviter, 2 witnesses, invitee), live **video (Zoom)**.
- **Oath:** invitee **agrees to the Eden "Peace Treaty."**
- **Donation:** inductee **donates 3 EOS** to the community.
- **Genesis seed (context):** new members were funded **200 EOS** (EOS Foundation) ahead of the
  first election (Oct 9, 2021) — distinct from induction donation and delegate budget.
*(Pinned: crypto.writer.io "Joining EdenOS"; eosgo.io Eden guides; Medium, Hramtsov.)*

## 2. Election round mechanics — **CONTRACT-PINNED**
- **Round count** — `count_rounds(num_members)` (`src/elections.cpp:96–104`): `result = 1; for (i
  = 12; i <= num_members; i *= 4) ++result;` → **1 round below 12 members, +1 per `12·4^k` step.**
  **Worked (independently re-derived): 7,777 → 6 rounds** (steps 12, 48, 192, 768, 3072 all ≤ 7777;
  12288 > 7777).
- **Group sizing** — `get_group_sizes(num_members, num_rounds)` (`src/elections.cpp:106–152`):
  `basic_group_size = int_root(num_members, num_rounds)`; three branches — `==3` → groups of 4/5
  with final 3; **`>=6` → `{5,6,…,6,N}`**; **else → `{basic}×(num_rounds−large_rounds)` then
  `{basic+1}×large_rounds`**, where `large_rounds = ⌊ln(N / basic^rounds) / ln((basic+1)/basic)⌋`.
  Invariants: **`group_max_size ≤ 12`**, and within-round sizes **differ by ≤ 1**.
  **Worked (independently re-derived): 7,777 → `int_root(7777,6)=4` (else branch),
  `large_rounds = ⌊ln(7777/4⁶)/ln(5/4)⌋ = ⌊2.873⌋ = 2` → `[4,4,4,4,5,5]`.** Matches; not inherited.
- **In-group consensus rule** — `finish_group()` (`src/elections.cpp:648–683`): each cast vote adds
  `+1` to its candidate and `+1` to `total_votes`; a **self-vote adds an extra `+group_size`** to
  that candidate. Winner set iff **`3·best > 2·total_votes + 3·group_size`**.
  **Cancellation (confirmed against full context):**
  - A candidate who did **not** self-vote can never win — it would require `total_votes >
    3·group_size`, impossible since `total_votes ≤ group_size`. → **the winner must have
    self-voted (consent-to-serve encoded in the arithmetic).**
  - For a self-voter with `a` actual votes: `3(a+group_size) > 2·total_votes + 3·group_size`
    reduces to `3a > 2·total_votes` → **`a > ⅔` of votes cast.**
  - Net: **elect iff the winner self-voted AND holds strictly more than two-thirds of votes cast.**

## 3. Board multisig authority — **CONTRACT-PINNED (distinct from §2 circle consensus)**
- `src/elections.cpp:714`: board `authority.threshold = board.size() * 2 / 3 + 1` (a 2/3+1
  multisig over the final board). **This is a different mechanism from the in-group consensus rule
  in §2 — do not conflate the two.** (Note `:731` performs a threshold inversion for the paired
  authority; the board-consent figure is the `*2/3 + 1` at `:714`.)

## 4. Election trigger — **CONTRACT-PINNED**
- `src/elections.cpp:367`: `new_threshold = active_members + (active_members + 9) / 10` =
  `active_members + ⌈active_members/10⌉` ≈ **+10% membership growth**, clamped to
  `[min_election_threshold, max_active_members]`.

## 5. Delegate / distribution budget — **CONTRACT-PINNED**
- **Rate:** `monthly_distribution_pct = 5` (`src/distributions.cpp:16, :148`; field in
  `include/distributions.hpp:26`).
- **Cadence:** every **30 days** — `next_time = distribution_time + eosio::days(30)`
  (`src/distributions.cpp:131`).
- **Pro-rating near elections:** `src/distributions.cpp:160–163` branches between a prorated
  amount (`prorate_num * monthly_distribution_pct * …`) and the plain `pct * balance / 100`.
- **Split (`make_distribution`, `src/distributions.cpp`):** `per_rank = amount / (ranks.size() − 1)`
  — an **equal tranche per rank above the bottom rank**. Iterating top→bottom, `total` accumulates
  the member count and each rank's **per-head = `per_rank / total`**. **Precise correction to the
  relayed summary:** `total` is the **cumulative** member count from the top rank down to the
  current rank (not "members at rank"); because `total` grows as rank descends, **per-head payout
  rises with rank** — conclusion unchanged, mechanism stated exactly.

## Summary table (all PINNED @ 2d779d4)
| Item | Value | Source |
|---|---|---|
| Round count | 1 + one per `12·4^k` ≤ N; **7777 → 6** | elections.cpp:96 |
| Group sizes | `int_root` + branch rule; **7777 → [4,4,4,4,5,5]** | elections.cpp:106 |
| Group max size | ≤ 12; within-round Δ ≤ 1 | elections.cpp:106 |
| Consensus | self-vote required **AND** > ⅔ of votes cast | elections.cpp:660–683 |
| Board multisig | `size·2/3 + 1` (distinct) | elections.cpp:714 |
| Election trigger | +10% growth (`active + ⌈active/10⌉`) | elections.cpp:367 |
| Distribution rate | 5% | distributions.cpp:16,148 |
| Cadence | 30 days, prorated near elections | distributions.cpp:131,160–163 |
| Split | equal per-rank tranche ÷ cumulative members → per-head rises with rank | distributions.cpp make_distribution |

## Still open (not in scope of these 4 files)
- Induction remains pinned to public guides (above); the contract files cover elections +
  distributions only. Induction-contract verification would need the membership/induction contract.

Sources — contract (pinned): `gofractally/Eden @ 2d779d4` files as listed above (received + sha256-verified 2026-07-07).
Induction (public guides, retrieved 2026-07-06):
[EdenOS onboarding (crypto.writer.io)](https://crypto.writer.io/p/what-you-need-to-know-about-joining) ·
[EOS Go — Eden Guide: Getting Started](https://www.eosgo.io/news/eden-guide-getting-started/) ·
[EOS Go — First On-Chain Election, 200 EOS](https://www.eosgo.io/news/eden-first-onchain-election) ·
[Medium — Hramtsov, What is Eden on EOS](https://medium.com/@vladislavhramtsov/what-is-eden-on-eos-ac90e965c397)
