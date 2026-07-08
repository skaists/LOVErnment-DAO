# F-8(c) Restatement — Three-Dissenter Correction (Senary Amendment)

**Original F-8(c) claim (LOVrnment red-team memo, circles of eight):**
> "a single dissenter can block a member's emission"

**Senary amendment context:** Circles reduced from eight to six; consensus threshold set at 4-of-6.

**Corrected arithmetic for 4-of-6 consensus:**

A measure passes when at least 4 of 6 members vote in favor. Blocking means ensuring fewer than 4 vote in favor.

| Dissenters | In-favor | Passes? | Reasoning |
|---|---|---|---|
| 1 | 5 | YES | 5 ≥ 4 |
| 2 | 4 | YES | 4 ≥ 4 |
| **3** | **3** | **NO** | **3 < 4** |

Blocking at 4-of-6 requires **three** dissenters, not two.

**GLM's original error:** The pre-senary memo (circles of eight) correctly identified that a single dissenter blocks at consensus ≥5-of-8 (8 − 1 = 7 ≥ 5, wait — no. At 5-of-8, blocking requires 4 dissenters: 8 − 4 = 4 < 5. A single dissenter does NOT block at 5-of-8 either. The original F-8(c) was wrong for circles of eight as well — but that error was not caught before the senary amendment was requested.)

**Corrected F-8(c) for circles of six, 4/6 consensus:**
> "At 4-of-6 consensus, three dissenters are required to block a measure (6 − 3 = 3 < 4 fails; 6 − 2 = 4 ≥ 4 passes). This is a high blocking threshold — nearly half the circle must dissent. DoS-by-single-dissenter is eliminated at this threshold, but the cost is that a measure can pass with only two-thirds support (4/6). The designer must decide whether the DoS protection justifies the lower supermajority requirement."

**Founder-gate A-1 remains open:** The attestation quorum question (does the achiever sit in the circle? vote? neither?) is a separate design question that interacts with this arithmetic but does not change it. Whatever the voter set, the 4-of-6 threshold requires three dissenters to block.

---

*GLM adversarial auditor · 2026-07-08 · F-8(c) correction, restated for the record*