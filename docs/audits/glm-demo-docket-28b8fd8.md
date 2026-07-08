# GLM consumption docket — the fractal-cascade demo (T-pattern)

Prepared by: Code seat, 2026-07-08
Tree: skaists/LOVErnment-DAO (PUBLIC as of 2026-07-08) · head 28b8fd8
Diff under audit: crates/lovernment-core across 1bf5fa7..28b8fd8
(commits 6a3b987 demo + 1a34105 chair-naming amendment; docs commits
excluded from scope)

## Declared scope (audit THIS, nothing else)

The cascade geometry demo: src/cascade.rs (logic + unit tests),
examples/fractal_cascade.rs (CLI demo), and the lib.rs doc/module
change admitting the module. Pure std, deterministic (seeded
xorshift), no network, no kernel-API surface touched (the pinned
escrow-core dependency and smoke test are UNCHANGED in this diff).

## Law-compliance claims (adversarial targets)

- Emission and attestation ABSENT by design — the module models
  consensus GEOMETRY only; a printed pointer routes to the gated
  kernel captures (CD-23 / CD-27). Nothing in this diff pays,
  mints, or attests.
- Source pins: canonical Round-1 schedule 2,3,5,8,13,21 with
  cumulative continuation (R2 21→233, R3 233→2,584, boundary-sharing
  windows) per WP pp.22-23; group sizes {5,6} per WP p.26; five-groups
  rank 2-6 and award no rank-1 (remainder law); artifact pin
  efe0698d…7663696 printed in output.
- Honesty claims: ranking is LABELED a seeded-shuffle stand-in for
  live consensus (4/6 six-groups, 3/5 five-groups printed as the real
  rule); cascades that fall into a {5,6} Frobenius gap (7-9, 13-14,
  19) END with an honest report rather than an invented merge rule.
- The RBI voice/vote line printed by the demo is law (F-V1 RULED,
  kernel CD-25), not caption.
- Determinism: same seed, same cascade (tested).

## Test battery (all green at head)

schedule windows incl. boundary sharing · full-house five perfect
rounds (7,776 → 1,296/216/36/6/1, no five-groups) · payout sums per
round for four house sizes · partition properties incl. maximal-sixes
and the Frobenius gaps · determinism. Plus the pre-existing kernel
smoke test (unchanged, still green).

## File states @ 28b8fd8 (blob digests, landing instrument)

3d5e54ee749e553b57c68350a5aee50753805b0fb05549dae528bbdc6bff3bb5  8744 B  crates/lovernment-core/src/cascade.rs
719b387f837f6c10a1ef9daac848ab9c62463fc6a8eeb7ba491a047b8ecb66ff  3693 B  crates/lovernment-core/examples/fractal_cascade.rs
1e2dbde228dd17d8dfc7a653169f5b53cbbf244581c9208e55e47bb4e42026c1  473 B  crates/lovernment-core/src/lib.rs

## The diff (1bf5fa7..28b8fd8, crates only, verbatim)

```diff
diff --git a/crates/lovernment-core/examples/fractal_cascade.rs b/crates/lovernment-core/examples/fractal_cascade.rs
new file mode 100644
index 0000000..2bddc71
--- /dev/null
+++ b/crates/lovernment-core/examples/fractal_cascade.rs
@@ -0,0 +1,103 @@
+//! Runnable demo: the skaists fractal cascade, geometry only.
+//!
+//!   cargo run --example fractal_cascade
+//!   cargo run --example fractal_cascade -- --members 500
+//!   cargo run --example fractal_cascade -- --members 500 --seed 42
+//!
+//! Deterministic (seeded), pure std, no network. Emission and
+//! attestation are absent by design — see the closing pointer.
+
+use lovernment_core::cascade::{run_cascade, CascadeEnd, CAP, FULL_HOUSE, SEED_DEFAULT};
+
+fn main() {
+    let mut members = FULL_HOUSE;
+    let mut seed = SEED_DEFAULT;
+    let mut args = std::env::args().skip(1);
+    while let Some(arg) = args.next() {
+        match arg.as_str() {
+            "--members" => {
+                members = args
+                    .next()
+                    .and_then(|v| v.parse().ok())
+                    .expect("--members takes a number");
+            }
+            "--seed" => {
+                seed = args
+                    .next()
+                    .and_then(|v| v.parse().ok())
+                    .expect("--seed takes a number");
+            }
+            other => {
+                eprintln!("unknown argument: {other}");
+                std::process::exit(2);
+            }
+        }
+    }
+
+    println!("skaists fractal cascade — geometry demo (deterministic, seed {seed})");
+    println!("source pins: fractally WP 1.0 — §Respect Distribution pp.22-23,");
+    println!("§Group Size p.26 — artifact sha256 efe0698d…7663696");
+    println!("consensus rule: 4/6 in six-groups · 3/5 in five-groups");
+    println!("ranking below is a seeded-shuffle stand-in for live consensus");
+    println!();
+    println!("participants: {members}");
+    println!();
+
+    let (rounds, end) = run_cascade(members, seed);
+
+    if rounds.is_empty() {
+        println!(
+            "no lawful round: {members} participants cannot form groups of \
+             five or six (lawful counts: 5, 6, 10-12, 15-18, and 20 up — \
+             the {{5,6}} Frobenius gaps are 7-9, 13-14, 19)"
+        );
+        return;
+    }
+
+    println!(
+        "{:>5} {:>13} {:>7} {:>7} {:>28} {:>14} {:>9}",
+        "round", "participants", "6-grps", "5-grps", "schedule (rank 1→6)", "respect", "advance"
+    );
+    for r in &rounds {
+        let sched = r
+            .schedule
+            .iter()
+            .map(|v| v.to_string())
+            .collect::<Vec<_>>()
+            .join(",");
+        println!(
+            "{:>5} {:>13} {:>7} {:>7} {:>28} {:>14} {:>9}",
+            r.round, r.participants, r.sixes, r.fives, sched, r.respect_paid, r.advancing
+        );
+    }
+    println!();
+    match end {
+        CascadeEnd::SingleSeat => {
+            println!(
+                "cascade resolved to a single seat in {} rounds",
+                rounds.len()
+            );
+        }
+        CascadeEnd::BelowLawfulRound { remaining } => {
+            println!(
+                "cascade ends with {remaining} seats — below the smallest \
+                 lawful round; a live house resolves this by attendance, \
+                 the demo reports it honestly"
+            );
+        }
+    }
+    if !rounds.is_empty() && rounds[0].fives > 0 {
+        println!("remainder law live: five-groups ranked 2-6 and awarded no rank-1");
+    }
+    println!();
+    println!(
+        "the Royal Beehive Intelligence seat (RBI; occupant at genesis: \
+         QueenBee) holds no vote and enters no round, which is why the \
+         human cascade is perfect (cap {CAP} = 6^5 + 1)"
+    );
+    println!(
+        "emission and attestation are absent by design — those captures are \
+         founder-gated in the kernel quarantine (beehive-nature \
+         docs/feature-backlog.md, CD-23 / CD-27)"
+    );
+}
diff --git a/crates/lovernment-core/src/cascade.rs b/crates/lovernment-core/src/cascade.rs
new file mode 100644
index 0000000..cacb235
--- /dev/null
+++ b/crates/lovernment-core/src/cascade.rs
@@ -0,0 +1,255 @@
+//! Fractal-cascade geometry — the weekly consensus shape, demo-grade.
+//!
+//! Source-pinned to the fractally whitepaper 1.0 (artifact sha256
+//! efe0698d…7663696, pinned in the kernel ledger): §Respect
+//! Distribution pp.22-23 (canonical Round-1 schedule 2,3,5,8,13,21;
+//! cumulative Fibonacci continuation across rounds — R2 spans 21→233,
+//! R3 spans 233→2,584), §Group Size p.26 (circles of five or six;
+//! five-groups rank 2-6 and award no rank-1).
+//!
+//! Emission and attestation are ABSENT by design: this module models
+//! the consensus GEOMETRY only — grouping, rank slots, Respect awards,
+//! advancement. Everything that pays b lives founder-gated in the
+//! kernel quarantine (docs/feature-backlog.md CD-23 / CD-27).
+//!
+//! Determinism: ranking inside a group is a seeded-shuffle STAND-IN
+//! for live fractal consensus (real ranking is human deliberation,
+//! 4/6 in six-groups, 3/5 in five-groups). Same seed, same cascade.
+
+/// The perfect senary house: 6^5 human participants.
+pub const FULL_HOUSE: usize = 7_776;
+/// The membership cap: the perfect cascade plus the one non-voting
+/// machine chair — the Royal Beehive Intelligence seat (RBI; occupant
+/// at genesis: QueenBee) — which enters no round.
+pub const CAP: usize = FULL_HOUSE + 1;
+/// Default demo seed.
+pub const SEED_DEFAULT: u64 = 7_777;
+
+/// The award sequence 2, 3, 5, 8, 13, 21, 34, … (Fibonacci from 2,3).
+pub fn awards() -> impl Iterator<Item = u64> {
+    let mut a: u64 = 2;
+    let mut b: u64 = 3;
+    std::iter::from_fn(move || {
+        let out = a;
+        let next = a + b;
+        a = b;
+        b = next;
+        Some(out)
+    })
+}
+
+/// Six consecutive awards for `round` (1-based). Rounds share their
+/// boundary value — rank-1 of round N+1 equals rank-6 of round N —
+/// which is the whitepaper's cumulative continuation:
+/// R1 = 2,3,5,8,13,21 · R2 = 21,34,55,89,144,233 · R3 = 233…2,584.
+pub fn respect_schedule(round: usize) -> [u64; 6] {
+    assert!(round >= 1, "rounds are 1-based");
+    let mut it = awards().skip(5 * (round - 1));
+    let mut out = [0u64; 6];
+    for slot in &mut out {
+        *slot = it.next().expect("award sequence is infinite");
+    }
+    out
+}
+
+/// Partition `n` participants into groups of six and five, maximizing
+/// six-groups (minimal five-groups). Returns `(sixes, fives)`, or
+/// `None` when no lawful partition exists (n < 5, or n ∈ {7, 8, 9}).
+pub fn partition(n: usize) -> Option<(usize, usize)> {
+    for fives in 0..=(n / 5) {
+        let rest = n - 5 * fives;
+        if rest % 6 == 0 {
+            return Some((rest / 6, fives));
+        }
+    }
+    None
+}
+
+/// Deterministic xorshift64 — pure std, no dependencies.
+pub struct Xorshift(u64);
+
+impl Xorshift {
+    pub fn new(seed: u64) -> Self {
+        Self(seed.max(1))
+    }
+    pub fn next_u64(&mut self) -> u64 {
+        let mut x = self.0;
+        x ^= x << 13;
+        x ^= x >> 7;
+        x ^= x << 17;
+        self.0 = x;
+        x
+    }
+    /// Fisher–Yates.
+    pub fn shuffle<T>(&mut self, v: &mut [T]) {
+        for i in (1..v.len()).rev() {
+            let j = (self.next_u64() % (i as u64 + 1)) as usize;
+            v.swap(i, j);
+        }
+    }
+}
+
+/// One round's outcome.
+#[derive(Debug, Clone, PartialEq, Eq)]
+pub struct RoundResult {
+    pub round: usize,
+    pub participants: usize,
+    pub sixes: usize,
+    pub fives: usize,
+    pub schedule: [u64; 6],
+    /// Total Respect awarded this round across all groups.
+    pub respect_paid: u64,
+    /// One representative advances per group.
+    pub advancing: usize,
+}
+
+/// Why a cascade ended.
+#[derive(Debug, Clone, PartialEq, Eq)]
+pub enum CascadeEnd {
+    /// A single seat remained — the cascade resolved fully.
+    SingleSeat,
+    /// Fewer seats remained than the smallest lawful round (5), or no
+    /// lawful {5,6} partition exists for the remaining count.
+    BelowLawfulRound { remaining: usize },
+}
+
+/// Run the cascade. Ranking within each group is the seeded shuffle
+/// order (demo stand-in); the top rank advances. Six-group awards are
+/// the round schedule ranks 1-6; five-groups award ranks 2-6 only —
+/// no rank-1 in a short group.
+pub fn run_cascade(members: usize, seed: u64) -> (Vec<RoundResult>, CascadeEnd) {
+    let mut rng = Xorshift::new(seed);
+    let mut ids: Vec<usize> = (0..members).collect();
+    let mut rounds = Vec::new();
+    let mut round = 1;
+
+    loop {
+        if ids.len() == 1 {
+            return (rounds, CascadeEnd::SingleSeat);
+        }
+        let Some((sixes, fives)) = partition(ids.len()) else {
+            return (
+                rounds,
+                CascadeEnd::BelowLawfulRound {
+                    remaining: ids.len(),
+                },
+            );
+        };
+        if sixes + fives == 0 {
+            return (
+                rounds,
+                CascadeEnd::BelowLawfulRound {
+                    remaining: ids.len(),
+                },
+            );
+        }
+
+        let schedule = respect_schedule(round);
+        rng.shuffle(&mut ids);
+
+        let mut paid: u64 = 0;
+        let mut advancing: Vec<usize> = Vec::with_capacity(sixes + fives);
+        let mut cursor = 0;
+
+        for _ in 0..sixes {
+            let group = &ids[cursor..cursor + 6];
+            cursor += 6;
+            // Ranks 1..=6 award schedule[0..=5]; shuffle order is rank
+            // order (demo stand-in). Top rank (last slot) advances.
+            paid += schedule.iter().sum::<u64>();
+            advancing.push(group[5]);
+        }
+        for _ in 0..fives {
+            let group = &ids[cursor..cursor + 5];
+            cursor += 5;
+            // Five-groups rank 2-6: schedule[1..=5], no rank-1 award.
+            paid += schedule[1..].iter().sum::<u64>();
+            advancing.push(group[4]);
+        }
+
+        rounds.push(RoundResult {
+            round,
+            participants: ids.len(),
+            sixes,
+            fives,
+            schedule,
+            respect_paid: paid,
+            advancing: advancing.len(),
+        });
+
+        ids = advancing;
+        round += 1;
+    }
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+
+    #[test]
+    fn schedule_is_the_canonical_cumulative_continuation() {
+        assert_eq!(respect_schedule(1), [2, 3, 5, 8, 13, 21]);
+        assert_eq!(respect_schedule(2), [21, 34, 55, 89, 144, 233]);
+        assert_eq!(respect_schedule(3), [233, 377, 610, 987, 1597, 2584]);
+        // Boundary sharing: rank-1 of N+1 == rank-6 of N.
+        for round in 1..8 {
+            assert_eq!(respect_schedule(round)[5], respect_schedule(round + 1)[0]);
+        }
+    }
+
+    #[test]
+    fn full_house_resolves_in_five_perfect_rounds() {
+        let (rounds, end) = run_cascade(FULL_HOUSE, SEED_DEFAULT);
+        assert_eq!(end, CascadeEnd::SingleSeat);
+        assert_eq!(rounds.len(), 5);
+        let sizes: Vec<usize> = rounds.iter().map(|r| r.participants).collect();
+        assert_eq!(sizes, [7_776, 1_296, 216, 36, 6]);
+        for r in &rounds {
+            assert_eq!(r.fives, 0, "the perfect house needs no five-groups");
+            assert_eq!(r.advancing, r.participants / 6);
+        }
+    }
+
+    #[test]
+    fn payout_sums_match_group_counts_every_round() {
+        for members in [FULL_HOUSE, 500, 83, 30] {
+            let (rounds, _) = run_cascade(members, SEED_DEFAULT);
+            for r in &rounds {
+                let six_sum: u64 = r.schedule.iter().sum();
+                let five_sum: u64 = r.schedule[1..].iter().sum();
+                let expected = six_sum * r.sixes as u64 + five_sum * r.fives as u64;
+                assert_eq!(r.respect_paid, expected, "round {}", r.round);
+            }
+        }
+    }
+
+    #[test]
+    fn partitions_use_only_lawful_sizes_and_maximal_sixes() {
+        // Frobenius for {5,6}: the largest non-representable count is
+        // 5*6-5-6 = 19; everything from 20 up partitions.
+        for n in 20..=600 {
+            let Some((sixes, fives)) = partition(n) else {
+                panic!("n={n} ≥ 20 must partition");
+            };
+            assert_eq!(6 * sixes + 5 * fives, n);
+            // Minimal fives: no smaller five-count also partitions n.
+            for fewer in 0..fives {
+                assert_ne!((n - 5 * fewer) % 6, 0, "n={n}: fives not minimal");
+            }
+        }
+        for n in [5usize, 6, 10, 11, 12, 15, 16, 17, 18] {
+            assert!(partition(n).is_some(), "n={n} is lawful");
+        }
+        for n in [1usize, 2, 3, 4, 7, 8, 9, 13, 14, 19] {
+            assert_eq!(partition(n), None, "n={n} has no lawful partition");
+        }
+    }
+
+    #[test]
+    fn cascade_is_deterministic() {
+        let a = run_cascade(500, 42);
+        let b = run_cascade(500, 42);
+        assert_eq!(a.0, b.0);
+        assert_eq!(a.1, b.1);
+    }
+}
diff --git a/crates/lovernment-core/src/lib.rs b/crates/lovernment-core/src/lib.rs
index c4e47bc..c1198cc 100644
--- a/crates/lovernment-core/src/lib.rs
+++ b/crates/lovernment-core/src/lib.rs
@@ -1,12 +1,14 @@
-//! skaists LOVErnment core — scaffold only.
+//! skaists LOVErnment core.
 //!
 //! First out-of-tree consumer of the Beehive Nature Reserve kernel,
-//! pinned at `kernel-v0.1.0`. No governance or product logic lives here
-//! yet (scope fence: that is the next lap); this crate exists to prove
-//! the kernel consumes cleanly from outside its own tree, and the smoke
-//! test in `tests/` exercises one public escrow path across that
-//! boundary.
+//! pinned at `kernel-v0.1.0` — the smoke test in `tests/` exercises one
+//! public escrow path across that boundary. The `cascade` module models
+//! the fractal-consensus geometry (demo-grade, source-pinned, no
+//! emission); governance logic beyond geometry remains gated in the
+//! kernel quarantine.
 
 #![forbid(unsafe_code)]
 
+pub mod cascade;
+
 pub use escrow_core;
```
