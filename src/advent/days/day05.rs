use std::collections::HashSet;

pub fn run(input: &str, a: bool, b: bool) -> String {
    match (a, b) {
        (true, true) => "Part a: ".to_string() + &part_a(input) + "\nPart b: " + &part_b(input),
        (true, false) => "Part a: ".to_string() + &part_a(input),
        (false, true) => "Part b: ".to_string() + &part_b(input),
        (false, false) => unreachable!(),
    }
}

fn part_a(input: &str) -> String {
    let mut lines = input.split("\n\n");
    let mut seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .split(char::is_whitespace)
        .filter_map(|s| s.parse().ok())
        .collect();
    for line in lines {
        let map = line
            .lines()
            .skip(1)
            .map(|l| {
                let mut l = l
                    .split(char::is_whitespace)
                    .map(|c| c.parse::<u64>().unwrap());
                (l.next().unwrap(), l.next().unwrap(), l.next().unwrap())
            })
            .collect::<Vec<_>>();
        seeds.iter_mut().for_each(|seed| {
            for (d, s, l) in &map {
                if (s..&(s + l)).contains(&&*seed) {
                    *seed = d + (*seed - s);
                    break;
                }
            }
        });
    }
    seeds.into_iter().min().unwrap().to_string()
}

fn part_b(input: &str) -> String {
    let mut lines = input.split("\n\n");
    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .split(char::is_whitespace)
        .filter_map(|s| s.parse().ok())
        .collect();
    let mut seeds: Vec<Vec<(u64, u64)>> = seeds.chunks(2).map(|v| vec![(v[0], v[1])]).collect();
    for line in lines {
        let mut map = line
            .lines()
            .skip(1)
            .map(|l| {
                let mut l = l
                    .split(char::is_whitespace)
                    .map(|c| c.parse::<u64>().unwrap());
                (l.next().unwrap(), l.next().unwrap(), l.next().unwrap())
            })
            .collect::<Vec<_>>();
        map.sort_unstable_by(|a, b| a.1.cmp(&b.1));
        seeds = seeds
            .into_iter()
            .map(|v| {
                let mut intervals: HashSet<(u64, u64)> = HashSet::new();
                for (seed, n) in &v {
                    let mut begin = *seed;
                    let end = seed + n;
                    for (i, (d, s, l)) in map.iter().enumerate() {
                        if end < *s {
                            // println!("{end} < {s}");
                            intervals.insert((begin, *n));
                            break;
                        } else if begin < *s {
                            // println!("{begin} < {s}, {end} <= {s}");
                            intervals.insert((begin, s - begin));
                            begin = *s;
                        } else if begin >= *s && begin < s + l && end < s + l {
                            // println!("{begin} >= {s}, {begin} < {s} + {l}, {end} < {s} + {l}");
                            intervals.insert((d + (begin - s), end - begin));
                            break;
                        } else if begin >= *s && begin < s + l && end >= s + l {
                            // println!("{begin} >= {s}, {begin} < {s} + {l}, {end} >= {s} + {l}");
                            intervals.insert((d + (begin - s), (s + l) - begin));
                            begin = s + l;
                        } else if begin >= s + l && i == map.len() - 1 {
                            // println!("{begin} >= {s} + {l}");
                            intervals.insert((begin, end - begin));
                        }
                    }
                }
                intervals.into_iter().collect()
            })
            .collect::<Vec<_>>();
    }
    seeds
        .into_iter()
        .flatten()
        .min_by(|a, b| a.0.cmp(&b.0))
        .unwrap()
        .0
        .to_string()
}
