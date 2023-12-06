pub fn run(input: &str, a: bool, b: bool) -> String {
    match (a, b) {
        (true, true) => "Part a: ".to_string() + &part_a(input) + "\nPart b: " + &part_b(input),
        (true, false) => "Part a: ".to_string() + &part_a(input),
        (false, true) => "Part b: ".to_string() + &part_b(input),
        (false, false) => unreachable!(),
    }
}

fn part_a(input: &str) -> String {
    let mut lines = input.lines().map(|l| {
        l.split_whitespace()
            .skip(1)
            .map(|x| x.parse::<f64>().unwrap())
    });
    let times = lines.next().unwrap();
    let records = lines.next().unwrap();
    times
        .zip(records)
        .map(|(time, record)| {
            let root = (time * time - 4.0 * record).sqrt();
            let upper = (time + root) / 2.0;
            let lower = (time - root) / 2.0;
            upper.ceil() as u64 - lower.floor() as u64 - 1
        })
        .product::<u64>()
        .to_string()
}

fn part_b(input: &str) -> String {
    let mut lines = input.lines().map(|l| {
        l.chars()
            .skip_while(|&c| !c.is_numeric())
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .parse::<f64>()
            .unwrap()
    });
    let time = lines.next().unwrap();
    let record = lines.next().unwrap();
    let root = (time * time - 4.0 * record).sqrt();
    let upper = (time + root) / 2.0;
    let lower = (time - root) / 2.0;
    (upper.ceil() as u64 - lower.floor() as u64 - 1).to_string()
}
