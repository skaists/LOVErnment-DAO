⟨GLM · adversarial findings memo · achievement-unlock emission channel · skaists LOVErnment · red-team⟩

---

**Scope:** Audit the mechanism as specified — random rotation, two-circle split, TE-1 through TE-7 invariants, selective-disclosure/ZK milestone proofs, 7,777 authenticated-human cap. Do not redesign; name minimal additional invariants only.

---

## F-1 · Achievement-definition gaming — RED

**Attack:** Milestones are "member-declared" and "aligned to a member-declared personal arc." This is the adversarial definition problem: a rational actor declares the easiest possible milestones that still produce valid ZK proofs. The ZK layer proves the milestone was *achieved* (the proof is sound for the claimed condition); it does not prove the milestone was *worth rewarding*. A member who declares "my milestone: I sent a message to myself" and presents a valid ZK proof of message-sending has satisfied the proof system but gamed the emission channel. The attestation layer validates proof soundness, not milestone meaningfulness. This is the "grade your own homework" problem — the cryptographic layer certifies completion, not significance.

**Design defense:** The two-circle split changes *who* attests, not *what* they attest to. It is orthogonal to this attack. The TE invariants (earned-emission-only, genesis zero, no premine) prevent emission creation outside the mechanism but do not constrain what *counts* as earned. The mechanism as specified has no constraint on milestone difficulty, significance, or externality — only on proof validity.

**Minimal invariant:** **Peer-declared milestones (SE-1).** A member's milestone must be proposed/declared by a *different* member than the one achieving it. The achiever can accept or reject the proposal but cannot author it. This eliminates self-certification at the definition layer. Mutual trivial-certification (A declares trivial milestone for B, B for A) is then bounded by the collusion-ring defense (F-3) — the proposer and achiever must also survive random-rotation separation to get attestation. One rule, targets the root cause.

---

## F-2 · Attestation bribery markets — RED

**Attack:** B-token emission has economic value; economic value creates a market for the gatekeepers. A member with a milestone to attest can bribe whatever random circle they land in that week. The cost per bribe is bounded by the expected emission value minus the bribe itself — if emission exceeds the 7-member bribe cost, the attack is profitable. The two-circle split raises the per-week cost (you must find and bribe NEW attesters each rotation) but doesn't eliminate the market — it changes it from "bribe your club once" to "bribe a new circle each week," which is more expensive but still viable for high-value milestones.

**Design defense:** The random rotation provides *cost raising*, not *cost elimination*. If attestation is purely mechanical (proof valid/invalid), ZK soundness eliminates bribery — a valid proof cannot be bribed into existence, and an invalid proof cannot be bribed into validity. But the design says attesters "witness the MILESTONE ONLY via selective disclosure," implying they see *some* information about the milestone beyond a binary valid/invalid signal. If attestation includes mapping the proof to a *milestone type* or *milestone significance*, that mapping is subjective and bribe-able. If attestation is purely mechanical, it's redundant with the ZK proof — which raises the question of why human attesters exist at all (see F-8).

**Minimal invariant:** **Attestation bond with dispute window (SE-2).** Attesters must stake b-tokens (a bond, not a fee — returned if undisputed) to sign an attestation. A time-locked dispute window opens after attestation during which any member can post a fraud bond challenging the attestation. If the challenge succeeds (the proof is shown invalid, or the milestone mapping is shown fraudulent), the attester's bond is slashed and the challenger is rewarded from it. This makes each attester's expected bribe revenue negative unless the milestone is *genuinely* unchallengeable — which, combined with SE-1 (peer-declared milestones of non-trivial difficulty), makes bribery unprofitable.

---

## F-3 · Collusion rings surviving random rotation — YELLOW

**Attack:** Circles of 8 drawn weekly from 7,777. A persistent colluding set of *k* members has a non-zero probability of achieving quorum (≥4) in the same circle each week. Per-week probability for a specific 8-member ring: ~(7/7776)^7 — negligible. But the attack is not a full-ring capture. A ring of *k* members needs only *m* of them (m ≤ 8) in one circle to achieve a cooperative quorum. For k=20, the probability that at least 4 land in the same circle on any given week is non-trivial (birthday-problem variant). Over 52 weeks (one year), the cumulative probability of achieving at least one quorum approaches significance. The law of large numbers favors the attacker over time.

**Design defense:** The two-circle split is the primary defense — stable self-chosen circles hold zero attestation power, so a colluding ring cannot form a permanent self-attestation club. Random assignment forces the ring to survive rotation. This eliminates the cheapest attack (form a club, self-attest forever) and raises the cost to "survive statistical assignment." The 7,777 cap bounds the maximum ring size, limiting Sybil-based pool inflation.

**Minimal invariant:** **Assignment co-occurrence cap (SE-3).** No member-pair may be assigned to the same circle more than *once* per epoch (e.g., one quarter = 12 weeks). Verifiable on-chain from assignment history. This means a persistent ring of k members must wait out the epoch before re-co-occurring, limiting the time-window for quorum attacks to one week per epoch per pair. For k=20 over 12 weeks, the attacker gets one shot at quorum — not a cumulative probability over weeks. The co-occurrence constraint is a standard derangement problem with known feasible algorithms for N=7,777, circle-size=8.

---

## F-4 · Rotation-RNG source unspecified — YELLOW

**Attack:** The mechanism specifies "randomly assigned" but does not specify: (a) the randomness source, (b) the commitment scheme, (c) on-chain verifiability, or (d) grinding resistance. A predictable RNG (e.g., block hash without commitment, or a single-operator seed) allows assignment manipulation: an attacker who can predict or influence the seed can arrange favorable circle compositions. Front-running is a subset: if assignments are predictable before they're finalized, an attacker can prepare collusive actions for their upcoming circle.

**Design defense:** None specified. The two-circle split assumes the random is *actually* random. If the RNG is manipulable, the rotation defense collapses — the attacker can place colluders together on demand.

**Minimal invariant:** **Verifiable random function (VRF) with epoch-unique entropy (SE-4).** Circle assignments must be generated via a VRF with on-chain verifiable output. The VRF input must mix a per-epoch entropy source (e.g., epoch block hash + commit-reveal from a threshold of random members) that no single participant can predict before the epoch boundary. Standard construction; one-sentence spec addition.

---

## F-5 · ZK proof soundness and selective-disclosure assumptions — YELLOW

**Attack class — three sub-vectors:**

**(a) Circuit scope underspecification.** The mechanism says attesters witness the milestone "via selective disclosure" but doesn't specify what the ZK circuit actually proves. If the circuit proves "a message was sent" but the milestone is "mentoring was completed," the proof is sound for the wrong proposition. The gap between what the circuit proves and what the milestone claims is the attack surface.

**(b) Replay / recycling.** Can a valid proof for milestone A be re-used or adapted to claim milestone B? Without per-proof binding to a specific milestone-type and epoch, an attacker could recycle proofs across milestones or re-submit the same proof in different circles to multiply emission.

**(c) Privacy leakage through attestation metadata.** Even with ZK, the *pattern* of attestations (who attests for whom, when, how often) creates a linkage graph. Over time, this graph can leak information about the intimate-tier personal arc data that's supposed to remain private. If member A consistently gets attested by members who also co-occur with member B, an observer infers a relationship. Selective disclosure hides the *content* but not the *structure*.

**Design defense:** ZK is the correct framework, and the "intimate-tier, never disclosed, never anchored" constraint is well-stated. But the specification doesn't define the circuit, the replay-protection mechanism, or the privacy leakage model. These are standard ZK engineering concerns — solvable, but they must be specified, not assumed.

**Minimal invariant:** **Proof-binding to milestone-type + epoch + nullifier (SE-5).** Each ZK proof must be cryptographically bound to: (a) a specific milestone-type identifier from a bounded registry, (b) the attestation epoch, and (c) a unique nullifier (hash of prover secret + epoch) stored in an on-chain spent-proof set. This prevents cross-milestone replay, cross-epoch replay, and proof recycling. The nullifier is standard ZK engineering; the milestone-type binding prevents proof adaptation attacks.

---

## F-6 · Reward-axis gaming — YELLOW

**Attack:** If b-token emission has components beyond a flat per-milestone rate — specifically "network-effect" or "velocity-of-value" bonuses — these create multiplicative gaming surfaces. A network-effect bonus (e.g., emission scales with the number of members who benefit from the achievement) can be gamed by defining achievements that "benefit" Sybil-adjacent accounts. A velocity bonus (e.g., faster milestone completion yields higher emission) can be gamed by declaring trivially easy milestones and completing them quickly — which compounds with F-1.

**Design defense:** The TE invariants (earned-emission-only, no premine) constrain the *total* emission supply but do not constrain the *distribution* formula. If the distribution formula has super-linear components, those components are gaming surfaces. The two-circle split is irrelevant to this attack — it governs *who* attests, not *how much* is emitted.

**Minimal invariant:** **Emission axis decomposition with sub-linear scaling (SE-6).** Emission must be decomposed into independent per-axis formulas, and every axis must scale at most *linearly* with its input. Network-effect bonuses must be capped at a fixed multiple of the base emission (e.g., max 2×). Velocity bonuses must be bounded by a per-epoch maximum that prevents fast-cycling trivial milestones from dominating. The TE invariants already cap the total; SE-6 caps the per-milestone *rate*, preventing distribution-gaming from concentrating emission in trivial achievements.

---

## F-7 · Milestone timing and strategic delay — QUESTION

**Observation:** If circles rotate weekly and attestation requires the assigned circle's consensus, a member can strategically *delay* milestone completion to align with a favorable circle assignment (one with known-sympathetic members, or one with a high quorum probability). Conversely, a member might *accelerate* milestone completion to lock in attestation before a rotation separates them from cooperative attesters. The mechanism doesn't specify whether milestones have a temporal binding (must be attested within N weeks of completion) or whether strategic timing is considered acceptable gameplay.

**Design defense:** The weekly rotation provides a natural rate limit (one attestation opportunity per week per circle slot). But if milestones can be "held" indefinitely before seeking attestation, the rate limit is soft. The TE invariants don't address timing.

**Question for the designer:** Is strategic milestone timing considered a feature (members optimizing their own arc progression) or an exploit (gaming the circle-assignment oracle)? If the former, no mitigation needed. If the latter, a **milestone freshness window** (attestation must occur within N weeks of proof generation) would close it — but this interacts with the weekly rotation cadence and may create undue urgency.

---

## F-8 · Governance-emission feedback loop and attestation redundancy — QUESTION

**Observation:** The design specifies two circle functions: (a) consensus rankings/representatives (governance), and (b) milestone attestation (emission). The two-circle split is specified for attestation power, but the governance function's relationship to emission is unclear. Three sub-questions:

**(a) Governance-emission feedback:** If governance outcomes (rankings, representatives) carry any economic weight or influence emission allocation, the governance function becomes a secondary attack surface. A colluding ring that captures governance in stable circles could influence emission distribution even though stable circles hold zero *direct* attestation power.

**(b) Attestation redundancy:** If milestone proofs are ZK-sound and the attestation is purely mechanical (valid/invalid), the human attestation layer is redundant — the proof speaks for itself. If attestation includes a subjective component (milestone significance, social worth), it's an attack surface (F-2). The design must specify which role attestation plays, because the defense posture differs radically between the two cases.

**(c) Consensus failure as DoS:** If circle consensus is required for attestation, a single dissenter can block a member's emission. This creates a denial-of-service surface (targeted blocking) and amplifies bribery (bribe the blocker, or bribe members to block a rival). The mechanism doesn't specify the consensus threshold or failure mode.

**Design defense:** The governance function is mentioned but not specified. The attestation function's subjective/objective nature is implied by "witness the milestone via selective disclosure" but not formalized. Consensus thresholds are not specified.

**Question for the designer:** (a) Does governance carry economic weight? (b) Is attestation purely proof-validation or does it include subjective judgment? (c) What is the consensus threshold, and what happens on failure?

---

## Summary

| # | Finding | Severity | Design already addresses? | Minimal invariant |
|---|---|---|---|---|
| F-1 | Achievement-definition gaming | **RED** | No — self-declared milestones have no difficulty/meaningfulness constraint | **SE-1:** Peer-declared milestones |
| F-2 | Attestation bribery markets | **RED** | Partial — random rotation raises cost, doesn't eliminate market | **SE-2:** Attestation bond + dispute window |
| F-3 | Collusion rings surviving rotation | **YELLOW** | Partial — two-circle split eliminates stable-club attack | **SE-3:** Co-occurrence cap (once per epoch) |
| F-4 | Rotation-RNG unspecified | **YELLOW** | No — randomness source undefined | **SE-4:** VRF with epoch-unique entropy |
| F-5 | ZK soundness assumptions | **YELLOW** | Partial — ZK framework correct, circuit/replay/leakage unspecified | **SE-5:** Proof-binding + nullifier |
| F-6 | Reward-axis gaming | **YELLOW** | Partial — TE invariants cap total, not distribution | **SE-6:** Sub-linear per-axis scaling |
| F-7 | Milestone timing | **QUESTION** | No — temporal binding unspecified | Open: feature or exploit? |
| F-8 | Governance-emission feedback | **QUESTION** | No — governance-emission relationship unspecified | Open: three sub-questions |

**Tally: 2 RED, 4 YELLOW, 2 QUESTION**

The two RED findings (F-1, F-2) are compounding: self-declared trivial milestones + bribable attestation = emission without genuine achievement. SE-1 (peer-declared milestones) eliminates the definition attack; SE-2 (bond + dispute) eliminates the bribery attack. Together they close the RED surface. The four YELLOW findings are engineering gaps with known solutions. The two QUESTIONs require designer input before assessment.

⟨GLM · red-team seat⟩