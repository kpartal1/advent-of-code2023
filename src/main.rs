mod advent;

/// Main function that reads environment arguments and runs advent days.
/// # Panics
/// This will panic if the enviroment arguments are not input to specification.
/// The args specification can be found in `README.md` under `Usage`
/// # Example
/// ```
/// cargo run --release 1 17 23
/// ```
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let days: Vec<_> = match args.len() {
        1 => (1..=25).collect(),
        _ => args.iter().skip(1).map(|d| d.parse().unwrap()).collect(),
    };
    for day in days {
        println!("Day {}:", day);
        advent::day(day);
    }
}
