pub fn run(input: &str, a: bool, b: bool) -> String {
    match (a, b) {
        (true, true) => "Part a: ".to_string() + &part_a(input) + "\nPart b: " + &part_b(input),
        (true, false) => "Part a: ".to_string() + &part_a(input),
        (false, true) => "Part b: ".to_string() + &part_b(input),
        (false, false) => unreachable!(),
    }
}

fn part_a(input: &str) -> String {
    input
        .lines()
        .map(|i| {
            let first = i.find(|s: char| s.is_numeric()).unwrap();
            let last = i.rfind(|s: char| s.is_numeric()).unwrap();
            i.get(first..first + 1).unwrap().parse::<usize>().unwrap() * 10
                + i.get(last..last + 1).unwrap().parse::<usize>().unwrap()
        })
        .sum::<usize>()
        .to_string()
}

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn part_b(input: &str) -> String {
    input
        .lines()
        .map(|i| {
            let (_, first) = (1..)
                .zip(NUMS)
                .map(|(idx, n)| {
                    (
                        i.find(n).unwrap_or(usize::MAX).min(
                            i.find(|s: char| s == char::from_digit(idx, 10).unwrap())
                                .unwrap_or(usize::MAX),
                        ),
                        idx,
                    )
                })
                .min()
                .unwrap();
            let (_, last) = (1..)
                .zip(NUMS)
                .map(|(idx, n)| {
                    (
                        (i.rfind(n))
                            .max(i.rfind(|s: char| s == char::from_digit(idx, 10).unwrap())),
                        idx,
                    )
                })
                .max()
                .unwrap();
            first * 10 + last
        })
        .sum::<u32>()
        .to_string()
}
