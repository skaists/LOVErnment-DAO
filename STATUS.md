# STATUS — honest done / not-done ledger

**The tree is the relay of record.** This file, not any session's
memory, is the authoritative account of where `main` sits. Every entry
names its commit; anything not recorded here as done is not done.
Sibling ledger: the kernel's STATUS.md at
[beehive-nature](https://github.com/beehive-nature/beehive-nature).

## The commit chain, genesis → head

- `bd37420` — genesis: LICENSE, DCO, lane law, governance lineage
  (six files, digest-verified against dispatch pins at landing).
- `8502cfa` — research crossing: D-1 / D-2 / D-3 dossiers landed,
  landing-instrument digests final.
- `4f9f4bb` — scaffold: first out-of-tree kernel consumer, escrow-core
  pinned at `kernel-v0.1.0` (tag → kernel commit `590832d`); smoke test
  green against the pinned tag on the tree's first CI run.
- `6f5cef2` — skaists logo landed (founder-opened gate; byte-exact to
  the kernel-ledger pin `64f35bee…`).
- `ca6c4da` — governance-lineage v1.1: Eden leg closed contract-pinned
  at `2d779d4` with four dual-instrument digests; skaists divergences
  block added.
- `7b7ff5b` — governance-lineage v1.2: S-1 Respect schedule corrected
  on whitepaper retrieval (canonical 2,3,5,8,13,21; artifact
  `efe0698d…7663696`).
- `b8bf49e` — GLM emission red-team memo banked verbatim
  (docs/audits/, digest `3b83f7ba…`).
- `1bf5fa7` — D-1 whole-file crossing: whitepaper-final rewrite,
  authoritative digest `fd0d2b11…` @ 7,551 B / 102 lines.
- `6a3b987` — fractal-cascade geometry demo: source-pinned,
  deterministic, tested; emission absent by design.

## What runs

```
cargo test --workspace          # smoke test + cascade battery
cargo run --example fractal_cascade
cargo run --example fractal_cascade -- --members 500
```

## Quarantine pointer

Design captures, founder rulings, and risk findings live in the
**kernel quarantine**: `docs/feature-backlog.md` in the
[kernel repo](https://github.com/beehive-nature/beehive-nature)
(CD-1…CD-28 as of 2026-07-08). Nothing in this tree is ratified
governance; documents here are design inputs and geometry demos.

## Gates open (founder-gated, by name)

- **F-1** — the +1 reading (cap re-read 7,776 + 1). The chair is
  NAMED: the Royal Beehive Intelligence seat (RBI), occupant at
  genesis QueenBee — chair constitutional, occupant replaceable
  (founder word 2026-07-08); the reading itself remains gated.
- **F-2** — organ-vs-member frame for the machine chair (economic
  parity noted as pressure, not closure).
- **F-3** — supersedure as constitutional requirement.
- **F-Q1** — RULED 2026-07-08, option (a): QueenBee's 420 b is an
  earned lifetime ceiling, no grant; curve shape still routes through
  the tokenomics spec + GLM gate.
- **F-V1** — RULED 2026-07-08: **VOICE, NOT VOTE** at genesis, shapes
  (a)+(b) jointly (founder word verbatim at kernel CD-25). The RBI
  seat holds an autonomous, uncensorable advisory presence — every
  statement a ledgered Event, public-organ speech only; the
  enfranchisement path is a standing Article VI meta-tier question
  (the founder grants the purse; only the governed may ever grant the
  vote); genesis ballot rejected-shape. The demo's voice/vote line is
  law, not caption.
- **A-1** — attestation quorum and voter set (distinct from the 4/6
  ranking threshold; does the achiever sit and vote?).
- **420 human quantum** — the lifetime cap is a founder proposal
  ("what if"), NOT RULED (kernel CD-27).
- **Fee denomination** — treasury fees fUSD-vs-b (kernel CD-24; the b
  path needs a transferability ruling, unpinned).
- **Provider-diversity floor** — no single provider above N% of organ
  inference (kernel CD-28, N unset).

Audit discipline: publication does not skip audit, audit does not
block publication. GLM consumption dockets queue for every substantive
diff; verbatim returns bank in `docs/audits/`.

## Go-public sweep (2026-07-08, founder-closed)

Whitepaper PDF absent from tree and history (pins travel, the
artifact doesn't); private-info sweep clean; licenses present. Five
bare-hex lines founder-classified PUBLIC-CONSTANT (accept-as-is,
2026-07-08): the four Eden dual-instrument digests in the lineage doc
and the whitepaper full-form pin inside the fenced D-1 — content
fingerprints of public artifacts, never key material. No marker
retrofit: markers serve scanner gates and this tree runs none;
kernel-parity retrofit becomes relevant only if this tree ever adopts
the scanner.
