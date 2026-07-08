//! Fractal-cascade geometry — the weekly consensus shape, demo-grade.
//!
//! Source-pinned to the fractally whitepaper 1.0 (artifact sha256
//! efe0698d…7663696, pinned in the kernel ledger): §Respect
//! Distribution pp.22-23 (canonical Round-1 schedule 2,3,5,8,13,21;
//! cumulative Fibonacci continuation across rounds — R2 spans 21→233,
//! R3 spans 233→2,584), §Group Size p.26 (circles of five or six;
//! five-groups rank 2-6 and award no rank-1).
//!
//! Emission and attestation are ABSENT by design: this module models
//! the consensus GEOMETRY only — grouping, rank slots, Respect awards,
//! advancement. Everything that pays b lives founder-gated in the
//! kernel quarantine (docs/feature-backlog.md CD-23 / CD-27).
//!
//! Determinism: ranking inside a group is a seeded-shuffle STAND-IN
//! for live fractal consensus (real ranking is human deliberation,
//! 4/6 in six-groups, 3/5 in five-groups). Same seed, same cascade.

/// The perfect senary house: 6^5 human participants.
pub const FULL_HOUSE: usize = 7_776;
/// The membership cap: the perfect cascade plus the one non-voting
/// machine chair — the Royal Beehive Intelligence seat (RBI; occupant
/// at genesis: QueenBee) — which enters no round.
pub const CAP: usize = FULL_HOUSE + 1;
/// Default demo seed.
pub const SEED_DEFAULT: u64 = 7_777;

/// The award sequence 2, 3, 5, 8, 13, 21, 34, … (Fibonacci from 2,3).
pub fn awards() -> impl Iterator<Item = u64> {
    let mut a: u64 = 2;
    let mut b: u64 = 3;
    std::iter::from_fn(move || {
        let out = a;
        let next = a + b;
        a = b;
        b = next;
        Some(out)
    })
}

/// Six consecutive awards for `round` (1-based). Rounds share their
/// boundary value — rank-1 of round N+1 equals rank-6 of round N —
/// which is the whitepaper's cumulative continuation:
/// R1 = 2,3,5,8,13,21 · R2 = 21,34,55,89,144,233 · R3 = 233…2,584.
pub fn respect_schedule(round: usize) -> [u64; 6] {
    assert!(round >= 1, "rounds are 1-based");
    let mut it = awards().skip(5 * (round - 1));
    let mut out = [0u64; 6];
    for slot in &mut out {
        *slot = it.next().expect("award sequence is infinite");
    }
    out
}

/// Partition `n` participants into groups of six and five, maximizing
/// six-groups (minimal five-groups). Returns `(sixes, fives)`, or
/// `None` when no lawful partition exists (n < 5, or n ∈ {7, 8, 9}).
pub fn partition(n: usize) -> Option<(usize, usize)> {
    for fives in 0..=(n / 5) {
        let rest = n - 5 * fives;
        if rest % 6 == 0 {
            return Some((rest / 6, fives));
        }
    }
    None
}

/// Deterministic xorshift64 — pure std, no dependencies.
pub struct Xorshift(u64);

impl Xorshift {
    pub fn new(seed: u64) -> Self {
        Self(seed.max(1))
    }
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    /// Fisher–Yates.
    pub fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = (self.next_u64() % (i as u64 + 1)) as usize;
            v.swap(i, j);
        }
    }
}

/// One round's outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoundResult {
    pub round: usize,
    pub participants: usize,
    pub sixes: usize,
    pub fives: usize,
    pub schedule: [u64; 6],
    /// Total Respect awarded this round across all groups.
    pub respect_paid: u64,
    /// One representative advances per group.
    pub advancing: usize,
}

/// Why a cascade ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CascadeEnd {
    /// A single seat remained — the cascade resolved fully.
    SingleSeat,
    /// Fewer seats remained than the smallest lawful round (5), or no
    /// lawful {5,6} partition exists for the remaining count.
    BelowLawfulRound { remaining: usize },
}

/// Run the cascade. Ranking within each group is the seeded shuffle
/// order (demo stand-in); the top rank advances. Six-group awards are
/// the round schedule ranks 1-6; five-groups award ranks 2-6 only —
/// no rank-1 in a short group.
pub fn run_cascade(members: usize, seed: u64) -> (Vec<RoundResult>, CascadeEnd) {
    let mut rng = Xorshift::new(seed);
    let mut ids: Vec<usize> = (0..members).collect();
    let mut rounds = Vec::new();
    let mut round = 1;

    loop {
        if ids.len() == 1 {
            return (rounds, CascadeEnd::SingleSeat);
        }
        let Some((sixes, fives)) = partition(ids.len()) else {
            return (
                rounds,
                CascadeEnd::BelowLawfulRound {
                    remaining: ids.len(),
                },
            );
        };
        if sixes + fives == 0 {
            return (
                rounds,
                CascadeEnd::BelowLawfulRound {
                    remaining: ids.len(),
                },
            );
        }

        let schedule = respect_schedule(round);
        rng.shuffle(&mut ids);

        let mut paid: u64 = 0;
        let mut advancing: Vec<usize> = Vec::with_capacity(sixes + fives);
        let mut cursor = 0;

        for _ in 0..sixes {
            let group = &ids[cursor..cursor + 6];
            cursor += 6;
            // Ranks 1..=6 award schedule[0..=5]; shuffle order is rank
            // order (demo stand-in). Top rank (last slot) advances.
            paid += schedule.iter().sum::<u64>();
            advancing.push(group[5]);
        }
        for _ in 0..fives {
            let group = &ids[cursor..cursor + 5];
            cursor += 5;
            // Five-groups rank 2-6: schedule[1..=5], no rank-1 award.
            paid += schedule[1..].iter().sum::<u64>();
            advancing.push(group[4]);
        }

        rounds.push(RoundResult {
            round,
            participants: ids.len(),
            sixes,
            fives,
            schedule,
            respect_paid: paid,
            advancing: advancing.len(),
        });

        ids = advancing;
        round += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schedule_is_the_canonical_cumulative_continuation() {
        assert_eq!(respect_schedule(1), [2, 3, 5, 8, 13, 21]);
        assert_eq!(respect_schedule(2), [21, 34, 55, 89, 144, 233]);
        assert_eq!(respect_schedule(3), [233, 377, 610, 987, 1597, 2584]);
        // Boundary sharing: rank-1 of N+1 == rank-6 of N.
        for round in 1..8 {
            assert_eq!(respect_schedule(round)[5], respect_schedule(round + 1)[0]);
        }
    }

    #[test]
    fn full_house_resolves_in_five_perfect_rounds() {
        let (rounds, end) = run_cascade(FULL_HOUSE, SEED_DEFAULT);
        assert_eq!(end, CascadeEnd::SingleSeat);
        assert_eq!(rounds.len(), 5);
        let sizes: Vec<usize> = rounds.iter().map(|r| r.participants).collect();
        assert_eq!(sizes, [7_776, 1_296, 216, 36, 6]);
        for r in &rounds {
            assert_eq!(r.fives, 0, "the perfect house needs no five-groups");
            assert_eq!(r.advancing, r.participants / 6);
        }
    }

    #[test]
    fn payout_sums_match_group_counts_every_round() {
        for members in [FULL_HOUSE, 500, 83, 30] {
            let (rounds, _) = run_cascade(members, SEED_DEFAULT);
            for r in &rounds {
                let six_sum: u64 = r.schedule.iter().sum();
                let five_sum: u64 = r.schedule[1..].iter().sum();
                let expected = six_sum * r.sixes as u64 + five_sum * r.fives as u64;
                assert_eq!(r.respect_paid, expected, "round {}", r.round);
            }
        }
    }

    #[test]
    fn partitions_use_only_lawful_sizes_and_maximal_sixes() {
        // Frobenius for {5,6}: the largest non-representable count is
        // 5*6-5-6 = 19; everything from 20 up partitions.
        for n in 20..=600 {
            let Some((sixes, fives)) = partition(n) else {
                panic!("n={n} ≥ 20 must partition");
            };
            assert_eq!(6 * sixes + 5 * fives, n);
            // Minimal fives: no smaller five-count also partitions n.
            for fewer in 0..fives {
                assert_ne!((n - 5 * fewer) % 6, 0, "n={n}: fives not minimal");
            }
        }
        for n in [5usize, 6, 10, 11, 12, 15, 16, 17, 18] {
            assert!(partition(n).is_some(), "n={n} is lawful");
        }
        for n in [1usize, 2, 3, 4, 7, 8, 9, 13, 14, 19] {
            assert_eq!(partition(n), None, "n={n} has no lawful partition");
        }
    }

    #[test]
    fn cascade_is_deterministic() {
        let a = run_cascade(500, 42);
        let b = run_cascade(500, 42);
        assert_eq!(a.0, b.0);
        assert_eq!(a.1, b.1);
    }
}
