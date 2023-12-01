Advent of Code 2023 in Rust
===========================
Advent of Code is a yearly 25-day coding competition in the form of a winter advent calendar. It is super fun and I would recommend checking it out if you like competitive programming: [adventofcode.com](https://adventofcode.com/about)

This repo is specific to the 2023 problems which can be found at [AoC 2023](https://adventofcode.com/2022)

I wanted to make this repo to practice good documentation and get a brief introduction to Rust's module system.

Hopefully I end up documenting each day's code well and I hope anybody who wants to learn something about Rust or this year's advent of code will be satisfied with this repo!

Usage
-----

Make sure you have Rust and Git installed, then open a terminal and, in your desired directory, run:

```shell
git clone https://github.com/kpartal1/advent-of-code2023
cd advent-of-code2023
cargo run --release
```

To run all parts of all 25 days, do

```
cargo run --release
```

The `--release` flag is optional, but I would recommend it for faster runtime.

You can specify days to run if you want, for example this will run days 2, 4, and 6:

```
cargo run --release 2 4 6
```

You can also specify which parts of which days you want to run, for example this will run part a of day 4, part b of day 6, and both parts of day 17.

```
cargo run --release 4a 6b 17
```

## Output

The output will show the answers for the specified parts and days, the runtime for each of those parts, and the final total runtime of all of them.

Here is an example input and output:

```
# Input
cargo run --release 1a 4
# Output
Day 1:
Part a: Unimplemented
Duration: 52.322µs
Day 4:
Part a: Unimplemented
Part b: Unimplemented
Duration: 34.472µs
Total Duration: 86.794µs
```

Thank you to GitHub user [ageron](https://github.com/ageron) for the inspiration for this idea (and some of the code lol <3).
