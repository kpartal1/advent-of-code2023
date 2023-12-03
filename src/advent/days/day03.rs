pub fn run(input: &str, a: bool, b: bool) -> String {
    match (a, b) {
        (true, true) => "Part a: ".to_string() + &part_a(input) + "\nPart b: " + &part_b(input),
        (true, false) => "Part a: ".to_string() + &part_a(input),
        (false, true) => "Part b: ".to_string() + &part_b(input),
        (false, false) => unreachable!(),
    }
}

fn part_a(input: &str) -> String {
    let info = input
        .lines()
        .map(|l| {
            let mut nums = Vec::with_capacity(l.len() / 2);
            let (mut first, mut last) = (0, l.len());
            let mut number = String::with_capacity(3);
            let mut reading = false;
            for (i, c) in l.chars().enumerate() {
                if !reading && ('0'..='9').contains(&c) && i == l.len() - 1 {
                    number.push(c);
                    first = i;
                    last = i;
                    nums.push(((first, last), number.parse::<u32>().ok()));
                } else if !reading && ('0'..='9').contains(&c) {
                    number.push(c);
                    first = i;
                    last = i;
                    reading = true;
                } else if !reading && c != '.' {
                    nums.push(((i, i), None));
                } else if reading && ('0'..='9').contains(&c) && i == l.len() - 1 {
                    number.push(c);
                    last = i;
                    nums.push(((first, last), number.parse::<u32>().ok()));
                } else if reading && ('0'..='9').contains(&c) {
                    number.push(c);
                    last = i;
                } else if reading && !('0'..='9').contains(&c) {
                    nums.push(((first, last), number.parse::<u32>().ok()));
                    number = String::new();
                    reading = false;
                    if c != '.' {
                        nums.push(((i, i), None));
                    }
                }
            }
            nums
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    for i in 0..info.len() {
        for j in 0..info[i].len() {
            let ((x, y), num) = &info[i][j];
            if let Some(num) = num {
                let x = if *x == 0 { 0 } else { x - 1 };
                let y = if *y == info.len() - 1 { *y } else { y + 1 };
                for k in x..=y {
                    let mut found = false;
                    for w in [
                        if i == 0 { 0 } else { i - 1 },
                        i,
                        if i == info.len() - 1 { i } else { i + 1 },
                    ] {
                        if info[w]
                            .iter()
                            .find(|((f, l), sym)| (f..=l).contains(&&k) && sym.is_none())
                            .is_some()
                        {
                            found = true;
                            sum += num;
                            break;
                        }
                    }
                    if found {
                        break;
                    }
                }
            }
        }
    }
    sum.to_string()
}

fn part_b(input: &str) -> String {
    let info = input
        .lines()
        .map(|l| {
            let mut nums = Vec::with_capacity(l.len() / 2);
            let (mut first, mut last) = (0, l.len());
            let mut number = String::with_capacity(3);
            let mut reading = false;
            for (i, c) in l.chars().enumerate() {
                if !reading && ('0'..='9').contains(&c) && i == l.len() - 1 {
                    number.push(c);
                    first = i;
                    last = i;
                    nums.push(((first, last), number.parse::<u32>().ok()));
                } else if !reading && ('0'..='9').contains(&c) {
                    number.push(c);
                    first = i;
                    last = i;
                    reading = true;
                } else if !reading && c == '*' {
                    nums.push(((i, i), None));
                } else if reading && ('0'..='9').contains(&c) && i == l.len() - 1 {
                    number.push(c);
                    last = i;
                    nums.push(((first, last), number.parse::<u32>().ok()));
                } else if reading && ('0'..='9').contains(&c) {
                    number.push(c);
                    last = i;
                } else if reading && !('0'..='9').contains(&c) {
                    nums.push(((first, last), number.parse::<u32>().ok()));
                    number = String::new();
                    reading = false;
                    if c == '*' {
                        nums.push(((i, i), None));
                    }
                }
            }
            nums
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    for i in 0..info.len() {
        for j in 0..info[i].len() {
            let ((x, y), num) = &info[i][j];
            if let None = num {
                let x = if *x == 0 { 0 } else { x - 1 };
                let y = if *y == info.len() - 1 { *y } else { y + 1 };
                let mut visited = std::collections::HashSet::new();
                let mut found = 0;
                let mut prod = 1;
                for k in x..=y {
                    for w in [
                        if i == 0 { 0 } else { i - 1 },
                        i,
                        if i == info.len() - 1 { i } else { i + 1 },
                    ] {
                        if let Some((fl, sym)) = info[w].iter().find(|((f, l), sym)| {
                            (f..=l).contains(&&k)
                                && sym.is_some()
                                && !visited.contains(&(w, &(*f, *l)))
                        }) {
                            visited.insert((w, fl));
                            if found == 2 {
                                prod = 0;
                                break;
                            }
                            prod *= sym.unwrap();
                            found += 1;
                        }
                    }
                }
                if found == 2 {
                    sum += prod;
                }
                visited.clear();
            }
        }
    }
    sum.to_string()
}
