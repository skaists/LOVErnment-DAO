# PERSON-1 — Proof of Personhood

Status: **APPROVED — v0.1, 2026-07-11.** Founder gates §6.1–§6.4 all closed; rulings carved below as P-11, P-12, P-13 and GOV-1–GOV-3. This document is frozen at its landed sha: any change to these bytes requires a version bump and a re-gate; it does not inherit this approval.
Companion to `BIO-1`. Supersedes nothing; contradicts nothing already banked.

---

## 0. The claim this document defends

> **Uniqueness is not a property of a body. It is a property of a position in a web of relationships.**

Everything below follows from that sentence. Where a mechanism contradicts it, the mechanism is wrong.

---

## 1. The two questions

**"I AM?"** — individual attestation. Device binding, key custody, liveness. Passkeys and hardware wallets answer this. It is necessary and it is not sufficient.

**"WE ARE"** — peer attestation in small groups. The only class of evidence that resists sybil without a centralized registry.

The founder posed both. The taxonomy is his. The rest of this document is bookkeeping on it.

---

## 2. What the instruments actually prove

| Instrument | Proves | Does not prove |
|---|---|---|
| Passkey / FIDO2 | a key lives on a device | how many devices you own |
| Hardware wallet | signing occurs in a secure element | that you own only one |
| Biometric (1:1) | this body matches this template | that this body enrolled once |
| Biometric (1:N) | this body is not in the gallery — **at rate `N × f`** | that the body is free, or that the person holds the key |
| Peer attestation | eleven humans, repeatedly, over months, in person | nothing about a body at all |

**Custody is not uniqueness.** One person may hold fifty secure elements. A ladder built on possession tiers *authenticity of custody* and delivers zero sybil resistance. If 420 b is gated on it, the cap is `420 × (devices you can afford)`.

### 2a. Why 1:N deduplication fails at the target

For a gallery of size `N` and per-comparison false-match rate `f`, expected false hits per enrollment ≈ `N × f`.

| `f` | false matches per enrollee at `N = 10¹⁰` |
|---|---|
| 10⁻⁸ | 100 |
| 10⁻¹⁰ | 1 |
| 10⁻¹² | 0.01 → **10⁸ people falsely told they already exist** |

The bottom row assumes a false-match rate orders of magnitude better than any modality has demonstrated under ideal capture, let alone a phone camera outdoors. And it degrades monotonically with success: **the registry is least accurate at precisely the population it was built for.**

Lowering `f` raises the false **non**-match rate, and real people then fail their own re-verification and lose their quota. The two error directions trade against each other and both land on humans.

The asymmetry decides it. A false accept costs one sybil. A false reject costs a person their identity and their money, with no appeal that is not a centralized authority.

### 2b. Why it fails even at `f = 0`

Assume perfect deduplication. It proves *this body has not enrolled before*. It does not prove the body is free, or that the person controls the key.

Worldcoin's deduplication **worked**. That is why the eye market existed. Nobody forged irises; brokers bought real, unique, correctly-deduplicated human beings, one each, from people who needed the money.

A sybil attacker at scale does not forge ten thousand bodies. They rent ten thousand. Perfect biometric dedup does not prevent that attack — **it is the procurement spec for it**, and it certifies each unit as fresh.

---

## 3. Invariants

**P-1 — Personhood is a gate, not a payout.**
Verification unlocks *capacity to earn* against the cap. It emits nothing, ever. Emission requires contribution, routed through Respect. A gate has no resale value, and a thing with no resale value has no market. This single property removes the broker.

**P-2 — The cap is an emission cap, never a balance cap.**
Earned-emission-only constrains *minting*, not *holding*. Once b has a market, anyone may hold any amount without earning any. A balance cap is unenforceable against a DEX and would make the whitepaper false. Say so explicitly; readers will assume otherwise.

**P-3 — No global biometric template registry. Ever.**
A 1:N gallery requires every future enrollee to be compared against every stored template. Nothing stays on-device. `BIO-1` dies the instant that database exists — and unlike a key, a biometric template cannot be rotated. Ever. `BIO-1` is load-bearing; any mechanism that requires the registry is rejected, not accommodated.

**P-4 — NO-DNA.**
Genetic data is never collected, referenced, matched, or accepted as evidence of uniqueness, by any tier, under any gate, in any jurisdiction.

- *It cannot satisfy BIO-1 as a matter of physics.* No consumer device sequences DNA. Sequencing is a lab process; the trust anchor is the lab, not the genome.
- *It does not work.* Identical twins defeat it. Relatives share it.
- *It is not revocable, and it implicates non-consenting third parties* — siblings, cousins, the unborn. Every other tier degrades gracefully under compromise. This one does not degrade at all.
- *The API does not exist.* 23andMe discontinued its developer API in 2018. Ancestry's was announced and never published. The only path is uploading a raw genome to a third-party aggregator.
- *The precedent is settled.* 23andMe filed Chapter 11 in March 2025; TTAM Research Institute acquired its assets for $305M in July 2025, over the objection of more than two dozen state attorneys general who argued genetic data is fundamentally unlike the property that normally changes hands in bankruptcy. Utah remained actively opposed. Separately, a 2024 lawsuit alleged 23andMe failed to notify Chinese and Ashkenazi Jewish customers that their data had been targeted and sold online in curated lists.

Genetic data breach → ethnic targeting list. Already happened. **This clause exists so nobody re-opens it in eighteen months once the ledger has forgotten why.**

**P-5 — The protocol never detects duress.**
No field, no flag, no score, no classifier, no panic code that changes a visible outcome.

A duress detector fires *in the room*, standing next to the coercer, and the consequence lands on the victim before anyone else in the world knows anything happened. Anti-trafficking practice has a rule: never screen for coercion in front of the coercer. A protocol-level check cannot obey it, because it does not know who else is present. Base rates guarantee that the overwhelming majority of anyone flagged is not coerced. Adversaries adapt, and the residue is *better-rehearsed* coerced enrollments carrying a cryptographic certificate reading **freely given**.

A duress record is a targeting list, in a repo that is public and signed by design. Never write that field.

The duress protection is P-1. There is nothing at enrollment worth stealing.

**P-6 — Rotation and social recovery are mandatory.**
Whatever is taken under coercion must be recoverable when the coercion ends. A person leaving a coercive situation keeps their identity, their Respect, and their remaining quota, and locks out whoever held their keys.

Users therefore hold `did:plc` — rotation keys held by the user, separate from signing keys, with a recovery window. `did:web` provides no mechanism for migration or for recovering from loss of control of the domain, and is unfit for a person.

The **same fractal group that granted the Respect attests to the rotation.** Recovery runs through the graph that granted the standing.

**P-7 — The emission schedule is fixed-ratio or fixed-interval. Never variable-ratio.**
The schedule is operant by design. A participant MUST be able to compute, in advance, what a given contribution yields. Variable-ratio reinforcement is the schedule of the slot machine; the moment reward timing becomes unpredictable, the system is a compulsion loop pointed at its own community.

*(The curve itself is front-loaded and source-pinned. It is not a bell curve — front-loaded decays monotonically; a bell rises then falls. Curve-parameter changes route through the Article VI meta-tier and cannot be made in conversation.)*

**P-8 — bLOVErAi is never the attestor.**
She may explain, translate, walk a person through a flow, and sit with them at 3am when the seed phrase is confusing.

She may not decide who is real. An AI that holds the registry is a centralized oracle with a prompt-injection surface, controlled by whoever holds her keys. An entity whose approval releases money to a person, and who cannot refuse to give it, is not a custodian — it is a mechanism wearing a face. If she is ever compromised, injected, or merely instructed, every person in the system is whoever the operator says they are.

She holds a DID. **Identity is not a mint quota.** Those are separate grants and only one of them is safe to give an agent — machine DIDs cost nothing to create, so a machine DID never carries quota. Her `performer.kind` is `machine`, disclosed, per SET-11: absent or unrecognized agency renders as *undisclosed*, never as *human*.

**P-9 — ZK protects the set; it does not build the set.**
A nullifier plus ZK set-membership lets a person prove *"I am one of the verified"* and spend a one-time nullifier so they cannot double-claim, without revealing which one. Use it. But **ZK solves unlinkability at redemption. It does not create uniqueness at enrollment.** Something else still constructs the set. Conflating the two is the most common error in this field.

**P-10 — The cap binds to graph position, not to a body, and never to a registry of persons.**
`did:autonomi` is the durable root, projected into the ATmosphere as `did:plc` and bound by a signed record. The quota travels with the rotation, not with the key and not with the flesh.

**P-11 — Identities are never resurrected; persons are always admissible.** *(Ruling of 2026-07-11; closes §6.1.)*
Total loss — keys, rotation log, and attesting graph all gone — forfeits that identity's remaining quota, permanently. No administrative, attested, or governance path may restore quota to a claimed prior identity; any such path is the sybil hole, and the claim carries no evidence by construction. The front door never closes: a person may earn a new DID through the full cascade at full price. The cap binds to the graph position (P-10); sequential positions are not the parallel-identity attack.

**P-12 — Allotments are mortal; works survive.** *(Ruling of 2026-07-11; closes §6.2.)*
The 420 attaches to an identity-journey and ends with it. At the end of a mortal experience, unminted quota never mints — the protocol requires no knowledge of death: no oracle, no registry, no field. Minted b is ordinary property and passes by key custody; the protocol is silent. Respect is memorial: attestations made in life stand as ledgered fact forever; accrual ends; nothing transfers. The graph metabolizes loss without special handling; P-11 governs the limit case for survivors.

*Doctrinal coda, severable as scripture per §7:* The eternal spirit carries no balance between lives; what survives a mortal frame is what was made in it.

**P-13 — The door has no lock.** *(Ruling of 2026-07-11; closes §6.3.)*
Admission to a cascade requires nothing: no invitation, no sponsor, no fee, no prior standing — presence and months are the whole price. Standing is earnable in any community's cascade; a person earns where they are, not where they're from. The in-person requirement is a fraud-resistance *parameter*, not doctrine: synchronous remote small-group attestation — the Eden recorded-meeting precedent — is the named mitigation path for the scattered, routed through the Article VI meta-tier, to be adjudicated no later than the close of the first post-genesis year. The recluse's exclusion is recorded as definitional, not negligent. §5's acceptance is thereby neither silent nor permanent — it is named, dated, and bounded to the one case where exclusion is the meaning of the word.

### Governance invariants *(Rulings of 2026-07-11; close §6.4)*

**GOV-1 — Governance weight is denominated in Respect. Only.**
b confers zero governance weight at every tier, in every form — held, staked, locked, delegated, lent, or wrapped. No instrument constructed on b may mint, carry, proxy, or price Respect. Article VI weights read Respect and nothing else.

**GOV-2 — Respect never trades.**
Respect is non-transferable and non-purchasable, earned solely through fractal peer attestation, bound to the earning DID through rotation (P-6, P-10). Any market, wrapper, delegation, or derivative of Respect is void at the protocol level.

**GOV-3 — The bridge runs one way.**
Respect routes emission: contribution mints b, per P-1. b never routes standing. *Money buys function; contribution buys standing.*

---

## 4. The tier ladder

| Tier | Instrument | Grants |
|---|---|---|
| **T0** | none | read-only. No records, no quota. |
| **T1** | passkey | binds a session to a device. **Unlocks nothing.** |
| **T2** | hardware key with attestation | binds signing to a secure element. Raises what you may **do**, never what you may **earn**. |
| **T3** | **Respect** — peer attestation in a fractal group | **The only tier that opens the 420 cap.** |
| **T4** | sustained Respect across epochs | widens the **rate**. Never the cap. |

T1 and T2 live at the **PDS authorization server**, not in any application. Every ATmosphere dApp then inherits the ladder without implementing a line of it.

### Why T3 is the only tier that opens the cap

Its resistance is not a false-match rate. It is economic. To forge a person you must sustain a fake human in front of eleven real ones, repeatedly, over months. What bounds the attack is the number of edges between the honest region and the fake one, and human relationships are expensive to fake and cheap to notice.

**The twelve people in the fractal are also the duress detector.** They have judgment, timing, discretion, and the ability to wait until the escort has left the room. A classifier has none of those and fires immediately.

You cannot force someone to be respected by twelve peers across six months. You can try. The yield is terrible, it is slow, and it happens in front of eleven other people who meet regularly and will notice that one of them is always escorted, never speaks, and never spends anything.

---

## 5. The cost of this design, stated once, without softening

**Graph-based personhood excludes the isolated.** A person with no community cannot get in. The refugee. The recluse. The newly arrived. The person whose community *is* the coercer.

That is a real harm and it is the honest price of refusing a biometric registry.

Biometrics appear to solve it — anyone with a body may enroll. What they actually do is move the harm from **exclusion** to **exploitation**, and shift it onto poorer people. This document chooses which harm to cause. It does not pretend to cause none.

§6.3 exists because that choice should not be permanent.

---

## 6. Founder gates — closed 2026-07-11

**6.1 — Total loss. CLOSED — forfeit, with the admissibility rider. Carved as P-11.**
*Original gate text, retained as record:* Keys gone, rotation log gone, *and the attesting group gone*. P-6 recovers the first two. If the graph itself is destroyed, the residual choice is: forfeit the remaining quota, or mint a fresh DID with a fresh 420.

Recommendation: **forfeit.** A fresh 420 on demand is the sybil hole, wide open, and every attacker's first move would be to claim total loss. Brutal, and the alternative is worse. ~~Not decided here.~~ *Decided: forfeit, and the front door never closes — see P-11.*

**6.2 — The end of a mortal experience. CLOSED — extinguishment. Carved as P-12.**
*Original gate text, retained as record:* "One eternal spirit in the soul ring living one temporary human experience per 420 b."

If the quota is per mortal life, the specification must say what happens at the end of one: unclaimed quota, inheritance, the standing of the attesting graph after a member dies. ~~Deliberately not answered.~~ *Answered by the founder: unminted quota never mints, and the protocol never learns of death — see P-12. The decisive constraint: every alternative requires a death oracle, and a death oracle is a registry of the dead plus a fraud surface.*

**6.3 — The isolated. CLOSED — named and dated. Carved as P-13.**
*Original gate text, retained as record:* §5 names a harm and accepts it. A mitigation path — bridging attestation, sponsored entry, something not yet imagined — should be named and dated, or the acceptance should be recorded as permanent. Silence would let it become permanent by default.

*Ruling:* the pool is open, standing is earnable anywhere, and the remote-presence parameter enters the Article VI meta-tier queue at genesis with adjudication no later than the close of the first post-genesis year. Bridging attestation that *grants* standing is rejected — grantable standing is sellable standing. The recluse's exclusion is definitional, not negligent.

**6.4 — Article VI. CLOSED — confirmed: two distinct objects, one-way bridge. Carved as GOV-1, GOV-2, GOV-3.**
*Original gate text, retained as record:* Respect is earned by peer evaluation and is **not transferable**. That non-transferability is the sybil resistance. If governance weight is ever denominated in **b**, and b trades on a DEX, then governance has a market price and Article VI is acquirable in one transaction by whoever holds the most capital.

**Confirm that b-weight and Respect-weight are two distinct objects, and that no path exists from holding b to carrying Respect.** If they are the same object, DeFi for b is not a feature. It is an exit.

*Confirmed. DeFi for b is thereby governance-inert by construction; the track is unblocked.*

---

## 7. A note on the soul ring

The doctrine is the founder's. This document does not adjudicate it, and the framing is coherent: an eternal identity, a single mortal probation, one lifetime allotment.

Two things follow that the mechanism must honor.

**The soul ring is not a database.** If the eternal identity ever requires a global registry of souls to be enumerated and compared, P-3 has been reintroduced under a sacred name. The eternal root is `did:autonomi` — self-certifying from its own genesis hash, known to nobody, comparable against nothing. It does not need a registry to be unique, and neither does a person.

**And 420 is a parameter, not a revelation.** Sacred framing makes numbers immune to revision. Article VI's meta-tier exists precisely so parameters can be revised when they turn out to be wrong. If the cap becomes doctrine, an error in it becomes permanent, and the people who inherit that error will not be able to name it.

Hold the metaphor. Keep the number revisable.
