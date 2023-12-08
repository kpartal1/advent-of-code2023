use std::collections::HashMap;

pub fn run(input: &str, a: bool, b: bool) -> String {
    match (a, b) {
        (true, true) => "Part a: ".to_string() + &part_a(input) + "\nPart b: " + &part_b(input),
        (true, false) => "Part a: ".to_string() + &part_a(input),
        (false, true) => "Part b: ".to_string() + &part_b(input),
        (false, false) => unreachable!(),
    }
}

fn part_a(input: &str) -> String {
    let mut lines = input.lines();
    let dirs = lines.next().unwrap().as_bytes();
    lines.next();
    let locs = lines.fold(HashMap::new(), |mut acc, l| {
        acc.insert(&l[0..3], (&l[7..10], &l[12..15]));
        acc
    });
    let mut curr = "AAA";
    let mut steps = 0;
    for dir in dirs.iter().cycle() {
        steps += 1;
        match dir {
            b'R' => {
                curr = locs[curr].1;
            }
            b'L' => {
                curr = locs[curr].0;
            }
            _ => panic!("Invalid direction: {dir}"),
        }
        if curr == "ZZZ" {
            return steps.to_string();
        }
    }
    steps.to_string()
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn part_b(input: &str) -> String {
    let mut lines = input.lines();
    let dirs = lines.next().unwrap().as_bytes();
    lines.next();
    let mut starts = Vec::new();
    let locs = lines.fold(HashMap::new(), |mut acc, l| {
        if l[0..3].ends_with('A') {
            starts.push(&l[0..3]);
        };
        acc.insert(&l[0..3], (&l[7..10], &l[12..15]));
        acc
    });
    let mut prod = Vec::with_capacity(starts.len());
    for start in starts.iter_mut() {
        let mut r = 0;
        let mut curr = *start;
        for dir in dirs.iter().cycle() {
            if curr.ends_with('Z') {
                break;
            }
            r += 1;
            match dir {
                b'R' => {
                    curr = locs[curr].1;
                }
                b'L' => {
                    curr = locs[curr].0;
                }
                _ => panic!("Invalid direction: {dir}"),
            }
        }
        prod.push(r);
    }
    lcm(&prod).to_string()
}
