use std::collections::HashMap;

pub fn run(input: &str, a: bool, b: bool) -> String {
    match (a, b) {
        (true, true) => "Part a: ".to_string() + &part_a(input) + "\nPart b: " + &part_b(input),
        (true, false) => "Part a: ".to_string() + &part_a(input),
        (false, true) => "Part b: ".to_string() + &part_b(input),
        (false, false) => unreachable!(),
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point(usize, usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Dir {
    N,
    S,
    W,
    E,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Pipe {
    Vertical,
    Horizontal,
    NEBend,
    NWBend,
    SWBend,
    SEBend,
    Ground,
    Start,
}

impl Pipe {
    fn next(
        self,
        dir: Dir,
        point @ Point(i, j): Point,
        pipes: &Vec<Vec<Pipe>>,
    ) -> (Dir, Point, Pipe) {
        let pipe = pipes[i][j];
        match dir {
            Dir::N => match pipe {
                Pipe::Vertical => (Dir::N, Point(i - 1, j), pipes[i - 1][j]),
                Pipe::SEBend => (Dir::E, Point(i, j + 1), pipes[i][j + 1]),
                Pipe::SWBend => (Dir::W, Point(i, j - 1), pipes[i][j - 1]),
                Pipe::Start => {
                    let (Point(nx, ny), nxt) = pipes[i][j].neighbors(point, pipes)[0];
                    let dir = if nx > i {
                        Dir::S
                    } else if nx < i {
                        Dir::N
                    } else if ny > j {
                        Dir::E
                    } else {
                        Dir::W
                    };
                    (dir, Point(nx, ny), nxt)
                }
                _ => panic!("Was going {dir:?} and encountered {pipe:?} at {point:?}"),
            },
            Dir::S => match pipe {
                Pipe::Vertical => (Dir::S, Point(i + 1, j), pipes[i + 1][j]),
                Pipe::NEBend => (Dir::E, Point(i, j + 1), pipes[i][j + 1]),
                Pipe::NWBend => (Dir::W, Point(i, j - 1), pipes[i][j - 1]),
                Pipe::Start => {
                    let (Point(nx, ny), nxt) = pipes[i][j].neighbors(point, pipes)[0];
                    let dir = if nx > i {
                        Dir::S
                    } else if nx < i {
                        Dir::N
                    } else if ny > j {
                        Dir::E
                    } else {
                        Dir::W
                    };
                    (dir, Point(nx, ny), nxt)
                }
                _ => panic!("Was going {dir:?} and encountered {pipe:?} at {point:?}"),
            },
            Dir::W => match pipe {
                Pipe::Horizontal => (Dir::W, Point(i, j - 1), pipes[i][j - 1]),
                Pipe::SEBend => (Dir::S, Point(i + 1, j), pipes[i + 1][j]),
                Pipe::NEBend => (Dir::N, Point(i - 1, j), pipes[i - 1][j]),
                Pipe::Start => {
                    let (Point(nx, ny), nxt) = pipes[i][j].neighbors(point, pipes)[0];
                    let dir = if nx > i {
                        Dir::S
                    } else if nx < i {
                        Dir::N
                    } else if ny > j {
                        Dir::E
                    } else {
                        Dir::W
                    };
                    (dir, Point(nx, ny), nxt)
                }
                _ => panic!("Was going {dir:?} and encountered {pipe:?} at {point:?}"),
            },
            Dir::E => match pipe {
                Pipe::Horizontal => (Dir::E, Point(i, j + 1), pipes[i][j + 1]),
                Pipe::SWBend => (Dir::S, Point(i + 1, j), pipes[i + 1][j]),
                Pipe::NWBend => (Dir::N, Point(i - 1, j), pipes[i - 1][j]),
                Pipe::Start => {
                    let (Point(nx, ny), nxt) = pipes[i][j].neighbors(point, pipes)[0];
                    let dir = if nx > i {
                        Dir::S
                    } else if nx < i {
                        Dir::N
                    } else if ny > j {
                        Dir::E
                    } else {
                        Dir::W
                    };
                    (dir, Point(nx, ny), nxt)
                }
                _ => panic!("Was going {dir:?} and encountered {pipe:?} at {point:?}"),
            },
        }
    }

    fn neighbors(self, Point(i, j): Point, pipes: &Vec<Vec<Pipe>>) -> Vec<(Point, Pipe)> {
        match self {
            // |
            Pipe::Vertical => {
                vec![
                    (Point(i - 1, j), pipes[i - 1][j]),
                    (Point(i + 1, j), pipes[i + 1][j]),
                ]
            }
            // -
            Pipe::Horizontal => {
                vec![
                    (Point(i, j - 1), pipes[i][j - 1]),
                    (Point(i, j + 1), pipes[i][j + 1]),
                ]
            }
            // L
            Pipe::NEBend => {
                vec![
                    (Point(i - 1, j), pipes[i - 1][j]),
                    (Point(i, j + 1), pipes[i][j + 1]),
                ]
            }
            // J
            Pipe::NWBend => {
                vec![
                    (Point(i - 1, j), pipes[i - 1][j]),
                    (Point(i, j - 1), pipes[i][j - 1]),
                ]
            }
            // 7
            Pipe::SWBend => {
                vec![
                    (Point(i + 1, j), pipes[i + 1][j]),
                    (Point(i, j - 1), pipes[i][j - 1]),
                ]
            }
            // F
            Pipe::SEBend => {
                vec![
                    (Point(i + 1, j), pipes[i + 1][j]),
                    (Point(i, j + 1), pipes[i][j + 1]),
                ]
            }
            // .
            Pipe::Ground => Vec::new(),
            // S
            Pipe::Start => {
                let mut n = Vec::with_capacity(2);
                for x in [-1, 0, 1] {
                    for y in [-1, 0, 1] {
                        if x == 0 && y == 0 {
                            continue;
                        }
                        let ii = (i as i32 + x) as usize;
                        let jj = (j as i32 + y) as usize;
                        if pipes[ii][jj]
                            .neighbors(Point(ii, jj), pipes)
                            .contains(&(Point(i, j), self))
                        {
                            n.push((Point(ii, jj), pipes[ii][jj]));
                        }
                    }
                }
                n
            }
        }
    }
}

impl From<u8> for Pipe {
    fn from(value: u8) -> Self {
        match value {
            b'|' => Self::Vertical,
            b'-' => Self::Horizontal,
            b'L' => Self::NEBend,
            b'J' => Self::NWBend,
            b'7' => Self::SWBend,
            b'F' => Self::SEBend,
            b'.' => Self::Ground,
            b'S' => Self::Start,
            _ => panic!(
                "UnsNported pipe character: {}",
                char::from_u32(value as u32).unwrap()
            ),
        }
    }
}

fn build_graph(input: &str) -> (Point, Vec<Vec<Pipe>>) {
    let n = input.chars().take_while(|&c| c != '\n').count() + 1;
    let mut origin = Point(0, 0);
    let mut graph = vec![vec![Pipe::Ground; n]; 1];
    graph.extend(input.lines().enumerate().map(|(i, l)| {
        let mut line = vec![Pipe::Ground];
        line.extend(l.bytes().enumerate().map(|(j, p)| {
            if p == b'S' {
                origin = Point(i + 1, j + 1);
            }
            Pipe::from(p)
        }));
        line.push(Pipe::Ground);
        line
    }));
    graph.push(vec![Pipe::Ground; n]);
    (origin, graph)
}

fn part_a(input: &str) -> String {
    let (origin @ Point(ox, oy), pipes) = build_graph(input);
    let mut points = Vec::from([origin]);
    let (mut dir, mut point, mut found) = pipes[ox][oy].next(Dir::W, origin, &pipes);
    while found != Pipe::Start {
        points.push(point);
        (dir, point, found) = found.next(dir, point, &pipes);
    }
    (points.len() / 2).to_string()
}

fn part_b(input: &str) -> String {
    let (origin @ Point(ox, oy), pipes) = build_graph(input);
    let mut steps = 0;
    let mut points = HashMap::from([(origin, steps)]);
    let (mut dir, mut point, mut found) = pipes[ox][oy].next(Dir::S, origin, &pipes);
    while found != Pipe::Start {
        steps += 1;
        points.insert(point, steps);
        (dir, point, found) = found.next(dir, point, &pipes);
    }
    let mut enclosed = 0;
    for i in 0..pipes.len() {
        let mut crossing = false;
        for j in 0..pipes[0].len() {
            if let Some(steps) = points.get(&Point(i, j)) {
                if let Some(n) = points.get(&Point(i + 1, j)) {
                    if crossing
                        && (*n == (steps + 1) % points.len() || (*n + 1) % points.len() == *steps)
                    {
                        crossing = false;
                    } else if !crossing
                        && (*n == (steps + 1) % points.len() || (*n + 1) % points.len() == *steps)
                    {
                        crossing = true;
                    }
                }
            } else if crossing {
                enclosed += 1;
            }
        }
    }
    enclosed.to_string()
}
