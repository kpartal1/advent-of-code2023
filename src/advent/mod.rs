use std::fs;

mod days;

/// Runs the advent of code problem for a specific day
/// # Panics
/// Panics if input isn't `0 < n ≤ 25`
pub fn day(n: u8, a: bool, b: bool) -> String {
    let path = format!("src/inputs/day{:02}.txt", n);
    let input = fs::read_to_string(&path);
    if let Ok(input) = input {
        let input = input.trim();
        let day_func = match n {
            1 => days::day01::run,
            2 => days::day02::run,
            3 => days::day03::run,
            4 => days::day04::run,
            5 => days::day05::run,
            6 => days::day06::run,
            7 => days::day07::run,
            8 => days::day08::run,
            9 => days::day09::run,
            10 => days::day10::run,
            11 => days::day11::run,
            12 => days::day12::run,
            13 => days::day13::run,
            14 => days::day14::run,
            15 => days::day15::run,
            16 => days::day16::run,
            17 => days::day17::run,
            18 => days::day18::run,
            19 => days::day19::run,
            20 => days::day20::run,
            21 => days::day21::run,
            22 => days::day22::run,
            23 => days::day23::run,
            24 => days::day24::run,
            25 => days::day25::run,
            _ => panic!("Invalid day, must be in the range [1,25]."),
        };
        day_func(input, a, b)
    } else {
        format!("ERROR: No data for day {n}")
    }
}
