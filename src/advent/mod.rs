mod days;

/// Runs the advent of code problem for a specific day
/// # Panics
/// Panics if input `0 ≥ n > 25`
pub fn day(n: u8) {
    let path = format!("src/inputs/day{:02}.txt", n);
    let input = std::fs::read_to_string(&path);
    if let Ok(input) = input {
        let input = input.trim();
        let day_func = match n {
            1 => days::day01::run,
            _ => todo!(),
        };
        day_func(input);
    } else {
        println!("ERROR: No data for day {n}");
    }
}
