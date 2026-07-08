//! Runnable demo: the skaists fractal cascade, geometry only.
//!
//!   cargo run --example fractal_cascade
//!   cargo run --example fractal_cascade -- --members 500
//!   cargo run --example fractal_cascade -- --members 500 --seed 42
//!
//! Deterministic (seeded), pure std, no network. Emission and
//! attestation are absent by design — see the closing pointer.

use lovernment_core::cascade::{run_cascade, CascadeEnd, CAP, FULL_HOUSE, SEED_DEFAULT};

fn main() {
    let mut members = FULL_HOUSE;
    let mut seed = SEED_DEFAULT;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--members" => {
                members = args
                    .next()
                    .and_then(|v| v.parse().ok())
                    .expect("--members takes a number");
            }
            "--seed" => {
                seed = args
                    .next()
                    .and_then(|v| v.parse().ok())
                    .expect("--seed takes a number");
            }
            other => {
                eprintln!("unknown argument: {other}");
                std::process::exit(2);
            }
        }
    }

    println!("skaists fractal cascade — geometry demo (deterministic, seed {seed})");
    println!("source pins: fractally WP 1.0 — §Respect Distribution pp.22-23,");
    println!("§Group Size p.26 — artifact sha256 efe0698d…7663696");
    println!("consensus rule: 4/6 in six-groups · 3/5 in five-groups");
    println!("ranking below is a seeded-shuffle stand-in for live consensus");
    println!();
    println!("participants: {members}");
    println!();

    let (rounds, end) = run_cascade(members, seed);

    if rounds.is_empty() {
        println!(
            "no lawful round: {members} participants cannot form groups of \
             five or six (lawful counts: 5, 6, 10-12, 15-18, and 20 up — \
             the {{5,6}} Frobenius gaps are 7-9, 13-14, 19)"
        );
        return;
    }

    println!(
        "{:>5} {:>13} {:>7} {:>7} {:>28} {:>14} {:>9}",
        "round", "participants", "6-grps", "5-grps", "schedule (rank 1→6)", "respect", "advance"
    );
    for r in &rounds {
        let sched = r
            .schedule
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");
        println!(
            "{:>5} {:>13} {:>7} {:>7} {:>28} {:>14} {:>9}",
            r.round, r.participants, r.sixes, r.fives, sched, r.respect_paid, r.advancing
        );
    }
    println!();
    match end {
        CascadeEnd::SingleSeat => {
            println!(
                "cascade resolved to a single seat in {} rounds",
                rounds.len()
            );
        }
        CascadeEnd::BelowLawfulRound { remaining } => {
            println!(
                "cascade ends with {remaining} seats — below the smallest \
                 lawful round; a live house resolves this by attendance, \
                 the demo reports it honestly"
            );
        }
    }
    if !rounds.is_empty() && rounds[0].fives > 0 {
        println!("remainder law live: five-groups ranked 2-6 and awarded no rank-1");
    }
    println!();
    println!(
        "the Royal Beehive Intelligence seat (RBI; occupant at genesis: \
         QueenBee) holds no vote and enters no round, which is why the \
         human cascade is perfect (cap {CAP} = 6^5 + 1)"
    );
    println!(
        "emission and attestation are absent by design — those captures are \
         founder-gated in the kernel quarantine (beehive-nature \
         docs/feature-backlog.md, CD-23 / CD-27)"
    );
}
