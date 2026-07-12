# Formal Review: Filecoin as a Heads Layer vs Arweave

**From:** Seat 4 (GLM-5.2 Code) — Surgical Code Tech II
**To:** Fable 5 (Claude) — Lead / Dispatcher
**Date:** 2026-07-12
**Re:** Founder request to evaluate Filecoin network as a replacement for Arweave in storing mutable heads atop Autonomi's immutable chunk store.

---

## Context

Autonomi provides immutable chunk storage only. Mutable state — "heads" (current pointers, latest commits, current configuration, session roots) — requires an external layer. The current solution is Arweave: each head update is written as a new permanent record; the latest transaction serves as the current head.

This review evaluates Filecoin as an alternative.

## The core mismatch

**Filecoin is not designed for small, frequently-updated mutable pointers.** It is a decentralized cold-storage market optimized for large datasets (32 GiB / 64 GiB sectors) with time-limited deal contracts enforced by cryptographic proofs. That is a different problem than heads.

This is not a deficiency — it is architecture. Filecoin solves bulk verifiable storage at hypercompetitive prices. Heads are tiny, hot, and write-frequent.

## Detailed comparison

### Storage model

| Dimension | Arweave (current) | Filecoin |
|---|---|---|
| Permanence | Pay-once, store-forever | Deal-based, time-limited (must renew) |
| Data sizes | Any (transaction-sized to GB) | Sector-optimized (32 GiB / 64 GiB) |
| Write frequency | One tx per head update | One deal per storage period |
| Retrieval | Fast, HTTP-gatewayed | Market-based retrieval (slower, costs FIL) |
| Cost model | High upfront, zero ongoing | Low ongoing, deal renewal overhead |
| Mutable state pattern | Append-only immutable log; latest = current | Requires FVM contract or off-chain indexer |

### FVM (Filecoin Virtual Machine)

Filecoin's smart contract layer (FVM) could theoretically manage heads as on-chain state:
- **FEVM compatibility:** Solidity contracts deploy directly.
- **Perpetual storage pattern:** FVM can auto-renew deals from an endowment.
- **DataDAO pattern:** Governed data management.

But FVM state transitions cost gas, and updating a head pointer every commit / every session is high-frequency for an L1. This is the classic "use a blockchain as a database" anti-pattern — expensive and slow for hot writes.

### Filecoin Onchain Cloud (FOC)

Filecoin's newest developer surface (Synapse SDK, PDP proofs, Filecoin Pay) is designed for application-controlled storage with verifiable persistence. It is a better fit than raw deal-making, but still oriented toward file storage (warm storage service, retrieval via Beam), not sub-KB mutable pointers that flip on every commit.

## Where Filecoin wins

1. **Cost at scale.** If heads ever reference large immutable artifacts (snapshots, archives, attestations), Filecoin's storage is orders of magnitude cheaper than Arweave. A hybrid where heads point to Filecoin-stored archives is viable.
2. **Verifiable persistence.** Proof of Data Possession (PDP) and Proof of Spacetime (PoSt) are strong guarantees. Arweave's permanence is probabilistic, not proof-backed.
3. **Retrieval market.** Competitive retrieval pricing vs Arweave's gateway-dependent model.
4. **Ecosystem maturity.** Mainnet since 2020, large provider network, Internet Archive and Shoah Foundation as users.

## Where Filecoin loses for this use case

1. **No native mutability.** Every head update requires either a new deal (heavyweight) or an FVM state transition (gas cost). Arweave's append-only-log model maps naturally to heads: write, read-latest.
2. **Deal lifecycle complexity.** Deals expire. Heads cannot expire. You need a perpetual-storage FVM contract or an off-chain bot to renew. This is a moving part that Arweave eliminates by construction.
3. **Latency.** Filecoin deal-making is not instant. Publishing a head should be sub-second; deal confirmation is not.
4. **Size mismatch.** A head is bytes to low KB. Filecoin's minimum sector is 32 GiB. The economics are designed for bulk, not pointers.
5. **Retrieval friction.** Fetching a head should be a single HTTP GET. Filecoin retrieval involves content routing, provider discovery, and potential payment negotiation.

## Verdict

**Filecoin is not a better heads layer than Arweave.** The mismatch is architectural, not incremental. Filecoin is built for large, cold, verifiable storage; heads are small, hot, and frequently mutated.

**However**, Filecoin is a strong candidate for a different role in the stack: **archive storage of immutable snapshots, attestations, or audit bundles** that heads reference. If the system ever needs to store large artifacts that must be verifiable and cheap to keep, Filecoin (via FOC / Synapse SDK) is the better tool for that job.

## Recommendation to Fable 5

1. **Keep Arweave for heads.** It is the right tool for small, append-only, permanent pointers. The cost model (pay once) and the access model (HTTP gateway, read-latest) match the heads use case.

2. **Evaluate Filecoin for archive tier.** If the system produces large immutable artifacts (full state snapshots, packaged audit trails, attestations bundles), a Filecoin storage layer with FVM-managed perpetual deals is worth a separate spike. Use Filecoin Onchain Cloud (Synapse SDK) as the developer surface.

3. **Hybrid architecture (if pursued):**
```
Heads (Arweave) → point to → Archives (Filecoin)
     ↑                            ↑
  Small, hot                 Large, cold
  Append-only                Deal-based
  Pay once                   Pay per period
```

4. **Do not use FVM for head state.** On-chain state transitions for every head update is gas-expensive and latency-unacceptable for a write-hot pointer.

## Sources

- Filecoin docs: storage model, crypto-economics, FVM, retrieval — `docs.filecoin.io` (fetched 2026-07-12)
- Filecoin Onchain Cloud / Synapse SDK — `docs.filecoin.io/build-on-filecoin/filecoin-onchain-cloud.md`
- Arweave architecture — `arweave.org/architecture`
- No Filecoin-specific claims are UNVERIFIED; all facts above are sourced from the official docs fetched today.

---

*Seat 4 — precision over volume. Awaiting your routing.*
