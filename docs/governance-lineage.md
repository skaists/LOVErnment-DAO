# Governance Lineage

Design-input sources for the skaists LOVErnment, ranked in founder priority
order. **This document banks sources; it ratifies nothing.** Founder-gated
questions (Q-2 through Q-5, kernel ledger CD-17) remain gated. Any mechanism
touching emission must reconcile with tokenomics invariants TE-1 through TE-7
(frozen) and passes independent audit (GLM seat) before promotion.

Import discipline: mechanisms are extracted and reimplemented in this
project's contracts — reference code is never adopted as a dependency. Every
parameter is source-pinned or marked UNVERIFIED.

---

## S-1 · fractally — headwater (founder priority 1)

Site: https://fractally.com (live, verified 2026-07-07)
Code: https://github.com/gofractally/fractally (frozen since 2022-04-15)
Book: *More Equal Animals* (D. Larimer)

**Core loop (source-pinned to fractally.com/blog/what-is-fractal-democracy,
fetched 2026-07-07):** members are randomly grouped into circles of 5–6; each
circle must reach consensus on who best represents its interests; chosen
representatives are randomly regrouped and the process recurses. There is no
pre-defined ballot — the anti-capture property: no filter by media, party, or
whale. Community practice runs the loop weekly, rewarding the most valued
contributions of that period.

**Exit doctrine (same source):** democratic consent requires the right AND
the ability to secede; fractal structure is what makes exit real. Mapping:
the LOVErnment factory pattern (S-3, banked at CD-17) is the machinery that
supplies exit — a market of LOVErnments is the "many countries" condition.
skaists is genesis; the factory is the exit fabric.

**Value for skaists:**
- Candidate answer-shape for CD-17 Q-2 (what the 7,777 do): recurring
  fractal consensus rounds as the base governance heartbeat.
- Core tech team mandate and budget: emerge from recurring peer-consensus
  rounds rather than appointment — the weekly loop is the funding mechanism.
- Power model arrives **pre-inverted**: influence is earned and unbuyable
  (Respect lineage), consistent with the kernel invariant that capability
  derives from evidence-based reputation, never wealth. (Contrast S-3, which
  required inversion.)

**Numerical note (observation, not ruling):** 6^5 = 7,776. A body of 7,777
in circles of six resolves through exactly five fractal rounds
(1,296 → 216 → 36 → 6 → 1) to a single seat; the membership cap is a perfect
senary fractal plus one.

**Deep-mine docket (Cowork seat):** the whitepaper link on fractally.com
redirects to an auth-walled Google Drive viewer; mine the PDF via the frozen
`gofractally/fractally` repo or direct retrieval. Targets: exact Respect
weights (Fibonacci schedule), in-circle consensus threshold, team-fractal
rules, council/averaging windows. **All such parameters are UNVERIFIED until
mined.** Status amended 2026-07-07: Eden leg CLOSED — see
docs/research/D-2 (this tree), contract-pinned. Whitepaper leg OPEN —
founder-hands retrieval pending; Respect schedule 1,2,3,5,8,13
corroborated (fractally blog + Hive-community secondary) with the 55→5
source conflict UNRESOLVED until the PDF.

**Not imported:** fractally's own token/exchange economics; any runtime
dependency on fractally software.

## S-2 · Eden — process reference

Code: https://github.com/gofractally/Eden (last push 2023-09-19, dormant)

Antelope-family (EOSIO) community governance experiment: peer-induction
ceremony for membership; layered fractal elections producing delegates and
budgets. Same VM family as the coordination ledger (Vaulta), making mechanism
extraction unusually direct — extraction discipline still applies (extract
the mechanism, reimplement; never adopt).

**Value for skaists:**
- Induction ceremony as a privacy-compatible sybil-resistance candidate for
  the 7,777 gate: peer attestation, no KYC, no biometrics — pairs with
  `did:autonomi` and the personhood gate.
- Layered election as the council-emergence shape; per-level delegate
  budgets as a team-funding shape.

**Status of specifics:** repo-pinned only, not yet source-mined — docketed to
Cowork. CLOSED 2026-07-07: mechanics contract-pinned at gofractally/Eden
@ 2d779d476f8bb6bc14dc30eadae9f7d70264b6fc — see docs/research/D-2.
Summary: groups computed per body (max 12, within-round sizes differ by
≤1; canonical mid-branch {5,6,…,6,remainder}); winner requires strictly
>2/3 of votes cast AND own self-vote (consent-to-serve in the
arithmetic); budget 5% per 30 days, equal per-rank tranches, per-head
rising with rank; board authority 2/3+1; elections trigger at +10%
membership. Dual-instrument byte cross-check beside D-2 (lead raw-fetch
+ Code clone/blob, all four identical):
4c7a838899b9822dee00cb901f06fbb0f1b06a2bf75e8cfd01fb00688046f6fd
elections.hpp ·
932976eef02f84ffed08af29d659b365bdada1c5dfbfaa5ac02bbbdfdec2c2f7
elections.cpp ·
252696f865d5b9b5918089e8467fc032bed4aa022fd8b0182cef4d7c81d211a3
distributions.cpp ·
7201708b9de0d80841164ec32c8adedd2261921268412e9d5277e5608ff23c5f
distributions.hpp

## S-3 · eosDAC — prior art, inverted (banked 2026-07-07, identity-day session)

Site: https://eosdac.io · Code: https://github.com/eosdac

The DAC Factory pattern: factory → custodians → worker proposals →
constitution-gated membership. **Import the shape, invert the power model:**
earned-emission not token-weighted; personhood-gated not wealth-gated.
Cross-reference: kernel ledger CD-17.

## S-4 · ORDAO / OREC — descendant, already banked

Code: https://github.com/Optimystics/ordao

Continuation of the fractally Respect lineage. Already adopted into the tree:
non-transferable Respect-style weighting (DRO Tier 3 jury ruling) and
optimistic ratification with Respect-weighted veto (Article VI draft
reference). Cross-reference only — no re-import.

## S-5 · psibase constellation — parallel platform (monitored, never adopted)

Org: https://github.com/gofractally — living work as of 2026-07-07:
`psibase` (serverless full-stack web3 protocol, pushed 2026-07-07), `spring`
(Antelope/Savanna consensus — the coordination ledger's node lineage),
`arbtrie`, `psio`.

Status: prior art and parallel evolution of the serverless-web3 thesis.
**Ruling stance: the architecture is closed.** No psibase dependency, ever;
monitored for mechanism ideas only.

## skaists divergences from headwater (v1.1)

(i) Circles are decision AND co-creation containers: emission-bearing
attestation of member milestones rides the same rotating circles that
govern — the beyond-headwater fusion. Full capture, risk findings (2 RED
/ 4 YELLOW / 2 QUESTION), and candidate invariants live in the kernel
backlog: anchor "Fractal co-creation". (ii) Ranking consensus 4/6
inherited from the headwater; the ATTESTATION quorum and voter set are
UNRULED — founder-gate A-1. (iii) Instrument name founder-ruled
"bLoveRai stock" — kernel backlog anchor "bLoveRai". (iv) Circle size:
NO divergence — fractally original design by founder ruling 2026-07-07,
on evidence (supersession history at the kernel's "Fractal co-creation"
capture). (v) The +1 seat above the perfect senary cascade remains
UNNAMED, founder-gated.

---

*Lineage banked 2026-07-07 by the lead seat from the live public record.
Liveness dates above follow the verified-then-stale principle: true at
verification time, aging since.*

*v1.1 — 2026-07-07, amended by lead dispatch, landed by Code; v1 digest
ac9105c7… preserved in git history as the genesis text.*
