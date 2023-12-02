pub fn run(input: &str, a: bool, b: bool) -> String {
    match (a, b) {
        (true, true) => "Part a: ".to_string() + &part_a(input) + "\nPart b: " + &part_b(input),
        (true, false) => "Part a: ".to_string() + &part_a(input),
        (false, true) => "Part b: ".to_string() + &part_b(input),
        (false, false) => unreachable!(),
    }
}

const NUM_RED: u32 = 12;
const NUM_GREEN: u32 = 13;
const NUM_BLUE: u32 = 14;

fn part_a(input: &str) -> String {
    (1u32..)
        .zip(input.lines())
        .map(|(i, l)| {
            l.chars()
                .skip_while(|&c| c != ':')
                .skip(2)
                .collect::<String>()
                .split("; ")
                .fold(1, |acc, s| {
                    if acc == 0 {
                        return 0;
                    }
                    s.split(", ")
                        .map(|s| s.split(char::is_whitespace).collect::<Vec<_>>())
                        .fold(1, |acc, v| {
                            if acc == 0 {
                                return 0;
                            }
                            let limit = match v[1].trim() {
                                "red" => NUM_RED,
                                "green" => NUM_GREEN,
                                "blue" => NUM_BLUE,
                                _ => unreachable!("{:?} must be a color", v[2]),
                            };
                            if v[0].parse::<u32>().unwrap() > limit {
                                0
                            } else {
                                i
                            }
                        })
                })
        })
        .sum::<u32>()
        .to_string()
}

fn part_b(input: &str) -> String {
    input
        .lines()
        .map(|l| {
            l.chars()
                .skip_while(|&c| c != ':')
                .skip(2)
                .collect::<String>()
                .split("; ")
                .fold([0, 0, 0], |[i, j, k], s| {
                    let [x, y, z] = s
                        .split(", ")
                        .map(|s| s.split(char::is_whitespace).collect::<Vec<_>>())
                        .fold([0, 0, 0], |[x, y, z], v| match v[1].trim() {
                            "red" => [x.max(v[0].parse::<u32>().unwrap()), y, z],
                            "green" => [x, y.max(v[0].parse::<u32>().unwrap()), z],
                            "blue" => [x, y, z.max(v[0].parse::<u32>().unwrap())],
                            _ => unreachable!("{:?} must be a color", v[2]),
                        });
                    [x.max(i), y.max(j), z.max(k)]
                })
                .iter()
                .product::<u32>()
        })
        .sum::<u32>()
        .to_string()
}
