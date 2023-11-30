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

You can specify days to run if you want, for example this will run days 2, 4, and 6:

```
cargo run 2 4 6
```

You can also run in release mode using the `--release` flag, for example this will run day 1 in release mode.

```
cargo run --release 1
```

Thank you to GitHub user [ageron](https://github.com/ageron) for the inspiration for this idea (and the code for handling environment arguments).
