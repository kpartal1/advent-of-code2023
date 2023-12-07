use rayon::prelude::*;
use std::cmp::Ordering;

pub fn run(input: &str, a: bool, b: bool) -> String {
    match (a, b) {
        (true, true) => "Part a: ".to_string() + &part_a(input) + "\nPart b: " + &part_b(input),
        (true, false) => "Part a: ".to_string() + &part_a(input),
        (false, true) => "Part b: ".to_string() + &part_b(input),
        (false, false) => unreachable!(),
    }
}

fn byte_to_strength_a(b: u8) -> usize {
    match b {
        b'A' => 12,
        b'K' => 11,
        b'Q' => 10,
        b'J' => 9,
        b'T' => 8,
        b'9' => 7,
        b'8' => 6,
        b'7' => 5,
        b'6' => 4,
        b'5' => 3,
        b'4' => 2,
        b'3' => 1,
        b'2' => 0,
        _ => panic!("{} not a valid char", b),
    }
}

fn byte_to_strength_b(b: u8) -> usize {
    match b {
        b'A' => 12,
        b'K' => 11,
        b'Q' => 10,
        b'T' => 9,
        b'9' => 8,
        b'8' => 7,
        b'7' => 6,
        b'6' => 5,
        b'5' => 4,
        b'4' => 3,
        b'3' => 2,
        b'2' => 1,
        b'J' => 0,
        _ => panic!("{} not a valid char", b),
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn classify_hand_a(h: &str) -> (Kind, &str) {
    let mut counts = vec![0; 13];
    for b in h.bytes().map(|b| byte_to_strength_a(b)) {
        counts[b] += 1;
    }
    let kind = match counts.iter().max() {
        Some(5) => Kind::FiveOfAKind,
        Some(4) => Kind::FourOfAKind,
        Some(3) => {
            if counts.contains(&2) {
                Kind::FullHouse
            } else {
                Kind::ThreeOfAKind
            }
        }
        Some(2) => {
            if counts.iter().filter(|&&x| x == 2).count() == 2 {
                Kind::TwoPair
            } else {
                Kind::OnePair
            }
        }
        Some(1) => Kind::HighCard,
        _ => Kind::HighCard,
    };
    (kind, h)
}

fn classify_hand_b(h: &str) -> (Kind, &str) {
    let mut counts = vec![0; 13];
    for b in h.bytes().map(|b| byte_to_strength_b(b)) {
        counts[b] += 1;
    }
    let js = counts[0];
    counts[0] = 0;
    let kind = match counts.iter().max() {
        Some(5) => Kind::FiveOfAKind,
        Some(4) => {
            if js == 1 {
                Kind::FiveOfAKind
            } else {
                Kind::FourOfAKind
            }
        }
        Some(3) => {
            if js == 2 {
                Kind::FiveOfAKind
            } else if js == 1 {
                Kind::FourOfAKind
            } else {
                if counts.contains(&2) {
                    Kind::FullHouse
                } else {
                    Kind::ThreeOfAKind
                }
            }
        }
        Some(2) => {
            if js == 3 {
                Kind::FiveOfAKind
            } else if js == 2 {
                Kind::FourOfAKind
            } else if js == 1 {
                if counts.iter().filter(|&&x| x == 2).count() == 2 {
                    Kind::FullHouse
                } else {
                    Kind::ThreeOfAKind
                }
            } else {
                if counts.iter().filter(|&&x| x == 2).count() == 2 {
                    Kind::TwoPair
                } else {
                    Kind::OnePair
                }
            }
        }
        Some(1) => {
            if js == 4 {
                Kind::FiveOfAKind
            } else if js == 3 {
                Kind::FourOfAKind
            } else if js == 2 {
                Kind::ThreeOfAKind
            } else if js == 1 {
                Kind::OnePair
            } else {
                Kind::HighCard
            }
        }
        Some(0) => Kind::FiveOfAKind,
        _ => Kind::HighCard,
    };
    (kind, h)
}

fn order_kinds_a((k, h): (Kind, &str), (ok, oh): (Kind, &str)) -> Ordering {
    if k > ok {
        Ordering::Greater
    } else if k < ok {
        Ordering::Less
    } else {
        for (b1, b2) in h
            .bytes()
            .map(|b| byte_to_strength_a(b))
            .zip(oh.bytes().map(|b| byte_to_strength_a(b)))
        {
            if b1 > b2 {
                return Ordering::Greater;
            } else if b1 < b2 {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

fn order_kinds_b((k, h): (Kind, &str), (ok, oh): (Kind, &str)) -> Ordering {
    if k > ok {
        Ordering::Greater
    } else if k < ok {
        Ordering::Less
    } else {
        for (b1, b2) in h
            .bytes()
            .map(|b| byte_to_strength_b(b))
            .zip(oh.bytes().map(|b| byte_to_strength_b(b)))
        {
            if b1 > b2 {
                return Ordering::Greater;
            } else if b1 < b2 {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

fn part_a(input: &str) -> String {
    let mut hands: Vec<((Kind, &str), u32)> = input
        .lines()
        .collect::<Vec<_>>()
        .par_iter()
        .map(|l| {
            let mut hb = l.split_whitespace();
            (hb.next().unwrap(), hb.next().unwrap().parse().unwrap())
        })
        .map(|(h, b)| (classify_hand_a(h), b))
        .collect();
    hands.par_sort_unstable_by(|&(h, _), &(oh, _)| order_kinds_a(h, oh));
    hands
        .into_iter()
        .zip(1..)
        .fold(0, |acc, ((_, b), i)| acc + b * i)
        .to_string()
}

fn part_b(input: &str) -> String {
    let mut hands: Vec<((Kind, &str), u32)> = input
        .lines()
        .collect::<Vec<_>>()
        .par_iter()
        .map(|l| {
            let mut hb = l.split_whitespace();
            (hb.next().unwrap(), hb.next().unwrap().parse().unwrap())
        })
        .map(|(h, b)| (classify_hand_b(h), b))
        .collect();
    hands.par_sort_unstable_by(|&(h, _), &(oh, _)| order_kinds_b(h, oh));
    hands
        .iter()
        .zip(1..)
        .fold(0, |acc, ((_, b), i)| acc + b * i)
        .to_string()
}
