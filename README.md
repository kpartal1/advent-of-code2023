Advent of Code 2023 in Rust
===========================
Advent of Code is a yearly 25-day coding competition in the form of a winter advent calendar. It is super fun and I would recommend checking it out if you like competitive programming: [adventofcode.com](https://adventofcode.com/about)

This repo is specific to the 2023 problems which can be found at [AoC 2023](https://adventofcode.com/2023)

I wanted to make this repo to practice good documentation and get some practice with Rust's module system.

Hopefully anybody who wants to learn something about Rust or this year's advent of code will be satisfied with this repo!

Timing
------

I will record my times for each problem here (*hopefully* same hardware every day):

This will be averaged over a few thousand runs.

| Day | a     | b     | Both  |
|-----|-------|-------|-------|
| 1   | 110µs | 1.4ms | 1.5ms |

Big O Analysis
--------------

Day 1: I think my solution to part (a) is O(2n) because it scans forward and backwards to find the first then the last numeric characters and then indexes the string for those values. Part (b) is more complicated, but I think it is O(kn) where k is the length of the number string, so O(144n) because there are 36 letters in the numbers one through nine and it goes through twice, also for each of these it goes through one more time to check for a numeric character. This honestly might be an underestimate. So technically these are both O(n) but this might explain why part (b) is so agonizingly slow.

Notes
-----

Day 1: I'm not super happy with the performance of part (b), I can imagine some ways to make it faster, but it's late at night and it took me long enough to make the function lol. Focusing on positives though, I didn't do too bad, and part (a) is a nice and elegant solution. Most important thing is: I learned something! Turns out Option::None is always greater than Option::Some(T), seemingly explicitly for cases like the one I encountered when doing part (b), so that's super interesting! Also tonight I've listened to the entirety of Utopia and Vespertine by Björk which I enjoyed so all around wins tonight. Looking forward to tomorrow.

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

You can also specify which parts of which days you want to run, for example this will run part (a) of day 4, part (b) of day 6, and both parts of day 17.

```
cargo run --release 4a 6b 17
```

## Output

The output will show the answers for the specified days and parts and the final average runtime of all of them.

Here is an example input and output:

```
Day 1:
Part a: 54573
Day 4:
Part a: Unimplemented
Part b: Unimplemented
Average Duration: 145.158µs
```

Thank you to GitHub user [ageron](https://github.com/ageron) for the inspiration for this idea (and a lot of the code lol <3).
