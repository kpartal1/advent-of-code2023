use std::{
    env, iter,
    time::{self, Duration},
};

mod advent;

/// Main function that reads environment arguments and runs advent days.
/// # Panics
/// This will panic if the enviroment arguments are not input to specification.
/// The args specification can be found in `README.md` under `Usage`
/// # Example
/// ```
/// cargo run --release 1a 17 23b
/// ```
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
    let mut total_duration = Duration::default();
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
        let start = time::Instant::now();
        println!("{}", advent::day(day, a, b));
        let end = time::Instant::now();
        println!("Duration: {:?}", end - start);
        total_duration += end - start;
    }
    println!("Total Duration: {:?}", total_duration);
}
