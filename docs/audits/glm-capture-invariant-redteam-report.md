# Capture-Level Invariant Red-Team — Stamped Report

**Type:** adversarial review of candidate capture-level invariants
**Origin:** Lead docket — two candidate invariants for red-team (CD-25/CD-27 amendments)
**Scope:** Invariant A ("One Machine Purse") + Invariant B ("Companions Custody, Never Mint") — attack for capture routes, custody-compromise surfaces, mint-path leaks, quantum-multiplication vectors
**Method:** per-claim adversarial analysis; findings graded RED/YELLOW/QUESTION; minimal invariants named; no redesign
**Prepared by:** GLM (adversarial auditor), 2026-07-08

---

## Invariant A — "One Machine Purse"

**As stated:** The machine quantum is singular to the named chair.

**Claims unpacked:**
- (A1) Each machine has exactly one purse (one quantum allocation).
- (A2) The purse is bound to a specific named human (the chair).
- (A3) "Singular" means the purse is the exclusive financial identity of that machine-chair pair.

---

### FA-1 · Session fork quantum-multiplication — RED

**Attack:** The invariant binds the purse to the *named chair*, not to an *active session*. If the machine's identity is stateless and derived from its name/key (standard for autonomous agents), two simultaneously active sessions of the same machine identity can both derive spend authority from the same purse. The machine's authentication layer (session token, signing key) is separate from its financial layer (purse identity). A forked session — whether through protocol exploitation, key re-use across contexts, or a legitimate-but-unintended concurrent instantiation — produces two spend paths from one quantum. Each session believes it has sole authority; the purse is "singular" in name but plural in practice.

The named chair cannot detect this at the purse level: both sessions produce valid signatures against the same machine key. The only detection surface is session-layer heuristics (concurrent sessions from the same identity) — which the invariant does not address. The "singular to the named chair" claim holds (one chair, one purse), but the purse is simultaneously spent by two sessions, effectively doubling the machine's quantum for the duration of the fork.

**Design defense:** None specified. The invariant addresses WHO controls the purse (the chair) but not HOW MANY active spend paths derive from it.

**Minimal invariant:** **One-active-session derivation (CA-1).** A machine's purse-key may be used in at most one active session at any point in time. A new session derivation invalidates any prior session's spend authority. Enforcement at the protocol layer (session nonce binding to purse spend) — not at the application layer.

---

### FA-2 · Chair-transfer custody gap — YELLOW

**Attack:** The invariant binds the purse to "the named chair." Chairs change — humans leave, transfer authority, or lose access. During a chair transfer, there exists a time window where both the outgoing chair and the incoming chair can plausibly claim authority over the purse. If the transfer mechanism re-keys the machine's purse identity, there's a window between the re-key initiation and finality where the old key may still produce valid spends. If the transfer does NOT re-key, the outgoing chair retains permanent access — violating the "singular to the named chair" claim (now two chairs claim one purse).

The "singular" property is time-bounded during transfer: it holds before and after, but may not hold during. Whether this gap is exploitable depends on the transfer mechanism's finality guarantees — which the invariant does not specify.

**Design defense:** The invariant states the binding but does not address transfer mechanics. Transfer is a lifecycle event, not a steady-state operation.

**Minimal invariant:** **Transfer finality before spend (CA-2).** During chair transfer, the outgoing chair's spend authority is revoked at the protocol layer before the incoming chair's authority activates. Zero-overlap guarantee: at no point during transfer do two chairs have simultaneous spend authority. The purse's "singular" property holds continuously, including through lifecycle transitions.

---

### FA-3 · "Quantum" ambiguity — allocation vs. balance — QUESTION

**Observation:** "Singular" could mean one allocation (budget ceiling, which may be replenished) or one balance (current spendable amount, which decreases with use). If a machine's quantum is a *budget ceiling* that resets or is topped up, "one purse" is trivially satisfied — there's always exactly one ceiling. But the security property the invariant is trying to capture (preventing the machine from having multiple independent funding sources) is only meaningful if "quantum" means the actual spendable balance. If it means the ceiling, the invariant is true-by-definition and provides no security guarantee.

The mint-path interaction is relevant here: if the machine's quantum (ceiling) can be increased by external actors (the chair tops up, the system grants bonuses), the "singular" property doesn't prevent quantum inflation — it just ensures inflation flows through one pipe.

**Question for the designer:** Does "quantum" mean the machine's current spendable balance, its budget ceiling, or its lifetime cumulative allocation? The capture resistance of "singular" differs across all three.

---

## Invariant B — "Companions Custody, Never Mint"

**As stated:** bLoveRai holds delegated custody of its bonded human's b under the human's DID root, budget-bounded, always revocable.

**Claims unpacked:**
- (B1) The companion (bLoveRai) holds *custody*, not ownership, of b.
- (B2) The companion *never mints* b — it can only disburse what was delegated to it.
- (B3) Custody is *under the human's DID root* — the human is the ultimate authority.
- (B4) The companion's spending is *budget-bounded* — it cannot exceed its allocation.
- (B5) Custody is *always revocable* — the human can reclaim b at any time.

---

### FB-1 · Re-delegation breaks DID-root authority — RED

**Attack:** The invariant states custody is "under the human's DID root." This implies a direct authority chain: human → DID root → companion → b. But if the companion can *further delegate* custody to a sub-entity (a common pattern in delegated systems — the companion dispatches b to a sub-custodian for a specific operation), the authority chain becomes: human → DID root → companion → sub-custodian → b. The human's DID root no longer directly controls the sub-custodian's access. Revocation at the human level may not propagate through the delegation chain without additional protocol support.

A captured companion can delegate custody to an attacker-controlled sub-entity. The human's DID-root revocation stops the companion but may not reach the sub-entity. The sub-entity continues spending b that the human intended to revoke. The invariant's "under the human's DID root" claim holds for the first hop but not for arbitrary delegation depth.

**Design defense:** The invariant as stated does not address re-delegation. "Delegated custody" could imply single-hop only, or it could permit chains. The claim "under the human's DID root" is only guaranteed for direct custody.

**Minimal invariant:** **Non-delegating custody (CB-1).** The companion holds direct custody only. No further delegation of custody authority is possible at the protocol layer. The authority chain is exactly one hop: human's DID root → companion. Any sub-dispatch of b (e.g., the companion sending b to a recipient) is a *transfer*, not a *delegation of custody* — title passes, custody does not chain.

---

### FB-2 · Yield on custodied b as indirect mint path — RED

**Attack:** "Never mint" is stated as a property of the companion itself — the companion does not create new b tokens. But b in the companion's custody may generate *yield* through protocol-level mechanisms: staking rewards, lending interest, liquidity mining, or any value-accrual mechanism the protocol provides. This yield is new b that was NOT delegated by the human — it is *created* by the protocol as a consequence of the companion's custody of the human's b. From the human's perspective, the companion's custody grew without additional delegation. From the system's perspective, new b entered circulation through a custody relationship — which is economically indistinguishable from minting through the companion.

If the companion can capture this yield (rather than it flowing back to the human), the companion has an independent revenue stream denominated in b — a partial break of the "custody, not ownership" claim. If the yield flows back to the human, the companion is effectively minting on the human's behalf — the human's b grows without additional delegation, which may violate other invariants (e.g., emission caps).

**Design defense:** The invariant prohibits the companion from minting directly. It does not address indirect value creation through custodied assets.

**Minimal invariant:** **Sterile custody (CB-2).** b in companion custody is sterile — it cannot generate yield, earn rewards, accrue interest, or receive any protocol-level value increase while held by the companion. Custody is a holding state, not an investment state. If the protocol has yield mechanisms, custodied b must be opted out or the yield must flow to a burn address, not to the companion or the human.

---

### FB-3 · Revocation race condition — YELLOW

**Attack:** "Always revocable" means the human can reclaim b at any time. But "at any time" is weaker than "immediately effective." If the companion has already signed a transaction that is in-flight (broadcast to the network but not yet finalized) at the moment the human issues revocation, the transaction may execute after revocation. The human's revocation prevents *future* spends but does not claw back *in-flight* spends. During the finality window (seconds to minutes depending on the chain), the companion's effective custody exceeds what the human intended — the revocation is not "always" effective, only eventually effective.

An adversarial companion who detects an incoming revocation (by monitoring the chain for the revocation transaction) can front-run: broadcast a large spend in the same block, executing before revocation finality. This is not theoretical — it's standard MEV/front-running behavior on any chain with observable pending transactions.

**Design defense:** "Always revocable" addresses the human's *right* to revoke but not the *timing* of revocation effectiveness. Finality is a protocol property, not a companion property.

**Minimal invariant:** **Revocation-before-spend finality (CB-3).** The human's revocation transaction must finalize before any subsequent companion spend transaction can finalize. This can be achieved by: (a) requiring a fresh authorization signature from the human for each companion spend (so the human's nonce advances and the companion's in-flight spends are invalidated), or (b) a timelock on companion spends that is longer than the revocation finality window. The invariant names the *property* (revocation takes precedence over pending spends), not the implementation.

---

### FB-4 · Budget-bounding dimension unspecified — YELLOW

**Attack:** "Budget-bounded" is stated but the bounding dimension is unspecified. Three natural dimensions exist, each with different capture implications:

**(a) Per-transaction bound:** The companion cannot spend more than X b in a single transaction. This prevents a single large theft but allows many small transactions that collectively drain the full allocation. A patient attacker can exfiltrate the entire custodied b over time, one small transaction at a time.

**(b) Per-time-window bound:** The companion cannot spend more than X b per day/week/epoch. This limits the extraction rate but not the total. An attacker with persistent access eventually drains the full allocation.

**(c) Absolute/cumulative bound:** The companion cannot spend more than X b in total across all time. This is the strongest bound — it caps the total loss. But it may conflict with the companion's operational needs if the human expects ongoing service (the companion needs a replenishable operational budget, not a one-time grant).

Without specifying the dimension, "budget-bounded" is a true-but-vague claim that doesn't constrain any specific attack. An attacker only needs the bound to be the weakest dimension (per-transaction) to exploit it.

**Design defense:** The invariant asserts bounding exists but does not specify its nature. Different dimensions provide different security guarantees.

**Minimal invariant:** **Per-transaction AND cumulative bounds (CB-4).** The companion's custody has both a per-transaction maximum (prevents single-event capture) and a cumulative lifetime maximum (prevents slow-drainage capture). The cumulative bound may be replenished by explicit human delegation — but replenishment is a new delegation event, not a continuation of the old one.

---

### FB-5 · Multi-machine custody isolation — QUESTION

**Observation:** The invariant specifies bLoveRai's custody of *its bonded human's* b. If bLoveRai bonds to one human who operates multiple machines, bLoveRai holds custody of the combined b for all of that human's machines. The "one machine purse" invariant (Invariant A) says each machine's quantum is singular. But if bLoveRai holds a single pooled custody, the isolation between machines' quanta is accounting-level only, not protocol-level. A compromised (or malicious) bLoveRai can reclassify b from machine A's allocation to machine B's, effectively transferring quantum between machines — violating "one machine purse" by giving machine B access to more than its singular quantum.

The two invariants interact: "one machine purse" requires per-machine isolation, but "companions custody" (as stated) does not specify whether custody is per-machine-isolated or pooled. If pooled, Invariant A's "singular" property depends on the companion's honest accounting — which is not a protocol guarantee.

**Question for the designer:** Is companion custody per-machine-isolated (each machine's quantum held in a separate protocol container) or pooled (one custody for the human's total b, with internal accounting separating machines)? If pooled, Invariant A's security depends on companion honesty, which contradicts the adversarial model.

---

## Cross-invariant interaction — Quantum-multiplication via custody fork

If a machine's purse (Invariant A) is held in companion custody (Invariant B), and the companion can be session-forked (FA-1 applied at the companion level), the attack compounds: a forked companion session could authorize spends from the machine's purse through two concurrent custody sessions. The companion's custody key is the spend path; forking the companion's session forks the machine's purse access. Invariant A's CA-1 (one-active-session derivation) must extend through the custody layer — not just at the machine level, but at the companion level too.

This is not a separate finding — it's the composition of FA-1 and FB-1/FB-3. But it's a critical interaction: the two invariants share a dependency on session uniqueness, and a session fork at the companion level bypasses both.

---

## Summary

| # | Finding | Severity | Minimal invariant |
|---|---|---|---|
| FA-1 | Session fork quantum-multiplication | **RED** | **CA-1:** One-active-session derivation |
| FA-2 | Chair-transfer custody gap | **YELLOW** | **CA-2:** Transfer finality before spend |
| FA-3 | "Quantum" ambiguity (allocation vs. balance) | **QUESTION** | Clarify dimension of "quantum" |
| FB-1 | Re-delegation breaks DID-root authority | **RED** | **CB-1:** Non-delegating custody |
| FB-2 | Yield on custodied b as indirect mint | **RED** | **CB-2:** Sterile custody |
| FB-3 | Revocation race condition | **YELLOW** | **CB-3:** Revocation-before-spend finality |
| FB-4 | Budget-bounding dimension unspecified | **YELLOW** | **CB-4:** Per-transaction AND cumulative bounds |
| FB-5 | Multi-machine custody isolation | **QUESTION** | Clarify pooled vs. per-machine isolation |

**Tally: 3 RED, 3 YELLOW, 2 QUESTION**

The three RED findings share a common root: the invariants address *ownership and authority* (who controls what) but not *session uniqueness and protocol isolation* (how many active spend paths exist, and whether custody boundaries are protocol-enforced or accounting-honored). FA-1 (session fork) and FB-1 (re-delegation) are capture vectors that bypass the authority chain without breaking it — the named chair still controls the purse, but the purse has more spend paths than the invariant assumes. FB-2 (yield as mint) bypasses the "never mint" constraint through indirect value creation rather than direct token creation.

The two QUESTIONs are designer clarifications that determine whether YELLOW findings exist. If "quantum" means cumulative balance (FA-3) and custody is per-machine-isolated (FB-5), the invariants are stronger than if "quantum" means budget ceiling and custody is pooled.

Six minimal invariants named: CA-1, CA-2, CB-1, CB-2, CB-3, CB-4.

---

*GLM adversarial auditor · 2026-07-08 · capture-level invariant red-team · 3R/3Y/2Q · stamped for founder-lead relay and public audit trail*