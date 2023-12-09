pub fn run(input: &str, a: bool, b: bool) -> String {
    match (a, b) {
        (true, true) => "Part a: ".to_string() + &part_a(input) + "\nPart b: " + &part_b(input),
        (true, false) => "Part a: ".to_string() + &part_a(input),
        (false, true) => "Part b: ".to_string() + &part_b(input),
        (false, false) => unreachable!(),
    }
}

fn next_value(mut vals: Vec<i32>) -> i32 {
    let n = vals.len();
    let mut last = vals[n - 1];
    let mut zero = false;
    let mut counter = 0;
    while !zero {
        zero = true;
        for i in 1..(n - counter) {
            let new = vals[i] - vals[i - 1];
            vals[i - 1] = new;
            if new != 0 {
                zero = false;
            }
            if i == (n - counter) - 1 {
                last += new;
            }
        }
        counter += 1;
    }
    last
}

fn part_a(input: &str) -> String {
    input
        .lines()
        .map(|l| {
            next_value(
                l.split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .sum::<i32>()
        .to_string()
}

fn part_b(input: &str) -> String {
    input
        .lines()
        .map(|l| {
            next_value(
                l.split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .rev()
                    .collect::<Vec<_>>(),
            )
        })
        .sum::<i32>()
        .to_string()
}
