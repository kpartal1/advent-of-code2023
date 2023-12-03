use std::{
    env, iter,
    time::{Duration, Instant},
};

mod advent;
const NUM_RUNS: u32 = 10_000;

/// Main function that reads environment arguments and runs advent days.
/// # Panics
/// This will panic if the enviroment arguments are not input to specification.
/// The args specification can be found in `README.md` under `Usage`
/// # Example
/// `cargo run --release 1a 17 23b`
fn main() {
    let args: Vec<String> = env::args().collect();
    let days: Vec<_> = match args.len() {
        1 => (1..=25).zip(iter::repeat(None)).collect(),
        _ => args
            .iter()
            .skip(1)
            .map(|d| (d.get(0..1).unwrap().parse().unwrap(), d.get(1..2)))
            .collect(),
    };
    let n = days.len();
    let mut avg_duration = Duration::default();
    let mut tot_duration = Duration::default();
    for (day, ab) in days {
        let (mut a, mut b) = (true, true);
        if let Some(ab) = ab {
            if ab == "a" {
                b = false;
            } else if ab == "b" {
                a = false;
            }
        }
        println!("Day {}:", day);
        let start_tot = Instant::now();
        println!("{}", advent::day(day, a, b));
        let end_tot = Instant::now();
        println!("Duration: {:?}", end_tot - start_tot);
        if n == 1 {
            let start_avg = Instant::now();
            for _ in 0..NUM_RUNS {
                advent::day(day, a, b);
            }
            let end_avg = Instant::now();
            avg_duration += end_avg - start_avg;
        } else if n == 25 {
            tot_duration += end_tot - start_tot;
        }
    }
    if n == 1 {
        println!("Average Duration: {:?}", avg_duration / NUM_RUNS);
    } else if n == 25 {
        println!("Total Duration: {:?}", tot_duration);
    }
}
