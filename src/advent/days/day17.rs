pub fn run(input: &str, a: bool, b: bool) -> String {
    match (a, b) {
        (true, true) => "Part a: ".to_string() + &part_a(input) + "\nPart b: " + &part_b(input),
        (true, false) => "Part a: ".to_string() + &part_a(input),
        (false, true) => "Part b: ".to_string() + &part_b(input),
        (false, false) => unreachable!(),
    }
}

fn part_a(_input: &str) -> String {
    String::from("Unimplemented")
}

fn part_b(_input: &str) -> String {
    String::from("Unimplemented")
}
