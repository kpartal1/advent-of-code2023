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
        .map(|l| {
            let mut v = l
                .split(" | ")
                .map(|side| {
                    side.split(char::is_whitespace)
                        .filter(|s| !s.is_empty())
                        .map(|e| e.parse::<u32>().unwrap_or(0))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            v[0] = v[0][2..].to_vec();
            let mut num_overlap = 0;
            for v1 in &v[0] {
                for v2 in &v[1] {
                    if v1 == v2 {
                        num_overlap += 1;
                    }
                }
            }
            if num_overlap == 0 {
                0
            } else {
                2u32.pow(num_overlap - 1)
            }
        })
        .sum::<u32>()
        .to_string()
}

fn part_b(input: &str) -> String {
    let n = input.lines().count();
    let mut mults = vec![1; n];
    input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let mut v = l
                .split(" | ")
                .map(|side| {
                    side.split(char::is_whitespace)
                        .filter(|s| !s.is_empty())
                        .map(|e| e.parse::<u32>().unwrap_or(0))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            v[0] = v[0][2..].to_vec();
            let mut num_overlap = 0;
            for v1 in &v[0] {
                for v2 in &v[1] {
                    if v1 == v2 {
                        num_overlap += 1;
                    }
                }
            }
            for j in 0..num_overlap {
                mults[i + 1 + j] += mults[i];
            }
            mults[i]
        })
        .sum::<u32>()
        .to_string()
}
