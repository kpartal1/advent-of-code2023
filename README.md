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

Setting a new goal for myself because I saw a blog post about it: all problems within less than 1 second total. We'll see how that goes.

| Day   | a     | b     | Both  |
|-------|-------|-------|-------|
| 1     | 110µs | 1.4ms | 1.5ms |
| 2     | 190µs | 230µs | 400µs |
| 3     | 430µs | 360µs | 760µs |
| 4     | 250µs | 260µs | 480µs |
| 5     | 65µs  | 120µs | 150µs |
| 6     | 45µs  | 42µs  | 49µs  |
| 7     | 2ms   | 2ms   | 4ms   |
| Total |       |       |       |

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
# Input
cargo run --release 2
# Output
Day 2:
Part a: 2268
Part b: 63542
Average Duration: 405.898µs
```

Big O Analysis
--------------

### Day 1: 
I think my solution to part (a) is O(2n) where n is the length of a string because it scans forward and backwards to find the first then the last numeric characters and then indexes the string for those values. Part (b) is more complicated, but I think it is O(kn) where k is the length of the number string and n is the length of the code string, so O(144n) because there are 36 letters in the numbers one through nine and it goes through twice, also for each of these it goes through one more time to check for a numeric character. This honestly might be an underestimate. So technically these are both O(n) but this might explain why part (b) is so agonizingly slow. Also these are both multiplied by the number of strings in the list, but I wanted to analyze specifically the process for one string in the list.

### Day 2: 
Both solutions are O(kn) where n is the length of the string and k is the number of strings. Both of the solutions just fold linearly over the whole collection and return the number. So O(n) for both.

### Day 3:
Both solutions are O(n<sup>3k</sup>) where n is the length of the strings and k is the number of important pieces of the string (numbers and special character (not .)). It starts as O(n) when it reads in the strings as a vec of vec that keeps the words and special characters with their indices. Then it goes through each vec in the vec and checks if the previous and next contain any overlaps. Thus, for each n vecs in the vec, it checks 3k items for overlap.

### Day 4:
Technically both algorithms are O(n<sup>2</sup>) (kinda O(n<sup>3</sup>) because we do it for each line but its ok) because we compare all pairs of the winning numbers and the checker numbers. This could be "optimized" to be O(n) using set intersections, but since the arrays are only ever like 20 long, its faster to use the n<sup>2</sup> algorithm. There is also a prefix sum in part (b) which adds a second iteration over [0, n).

### Day 5:
Pretty sure that these are both O(kn) where n is the number of seeds and k is the length of the longest map. This is because for each seed, we go through each map to find the new seed value. Technically O(kn<sup>2</sup>) because we have to do this for every line, but I'm looking more granularly at how we operate on each map rather than on the whole dataset. For part (b) n is actually the number of pairs of seeds and the complexity is more like O(n<sup>2</sup>) because of the way the intervals break off from each other and expand.

### Day 6:
This algorithm is O(1) because it uses math to get the answer. I was initially using a binary search which is O(log(n)), but I found a solution that explained how to find the answer using math, and it's pretty simple too its just the quadratic formula! I should probably go back and see how I can optimize day 1...

### Day 7:
This algorithm is O(nlog(n)) because it sorts the list to get the orders of the hands. Initially, it goes through each hand and finds its kind. Then it sorts that list based on a custom comparator function which is either O(1) or O(n), so I guess technically worst case the sorting is O(n<sup>2</sup>log(n)), but that's not the realistic case.

Notes
-----

### Day 1:
I'm not super happy with the performance of part (b), I can imagine some ways to make it faster, but it's late at night and it took me long enough to make the function lol. Focusing on positives though, I didn't do too bad, and part (a) is a nice and elegant solution. Most important thing is: I learned something! Turns out Option::None is always greater than Option::Some(T), seemingly explicitly for cases like the one I encountered when doing part (b), so that's super interesting! Also tonight I've listened to the entirety of Utopia and Vespertine by Björk which I enjoyed so all around wins tonight. Looking forward to tomorrow.

### Day 2:
I might need to chill out with my desire to answer the problems in functional ways because a few for loops might've made this easier and my code more readable. I liked these problems though. I had to finish them later in the day though because I wasn't at home for the problem drop, so L. Now I don't know where I would've been on the leaderboards.

### Day 3:
This was hard!!! My solution is so ugly but I am proud that I got it to work at all. Took me far longer than I was expecting or hoping for but its not the end of the world and I'm trying to focus on positivity recently so it's cool that I finished it at all! Hoping for a good day 4.

### Day 4:
Really enjoyed this problem. Learned very concretely how prefix sum works and when to use it. In part (b), it felt like such a cool realization that I just "invented" prefix sum before I realized that what I was doing was prefix sum. Also, I was using HashSet intersections initially and it was fun to "optimize" that away into a brute n<sup>2</sup> check. I also just tried an O(nlogn) check and it was slower! I guess the overhead of sorting the Vec is worse than just doing direct comparison. Listening to some new Björk, Drawing Restraint #9, this is weird but I'm also kinda digging it and its cool to hear sounds that made their way into Volta, especially Vertebrae by Vertebrae. It really shows off the time and work that goes into making music.

### Day 5:
This problem was hard, but I'm satisfied with my answer! I especially enjoyed part (b) because I initially did it in a way that would've taken a *really* long time to run. Once I optimized it though, it's one of the fastest parts of the days I've done so far. I find that super cool because optimizing these problems is one of the goals I have for AoC this year.

### Day 6:
This problem was easier finally! I brute forced it initially because I wanted to get a good ranking on the leaderboard, but after lots of optimization and finding out you can use math (thank you [Dragon-Hatcher](https://github.com/Dragon-Hatcher) on GitHub) to find the answer, I got it running super fast and with accurate results. So now this is my fastest solution, very cool!

### Day 7:
I used parallel iteration for the first time in this problem, which I think is pretty cool. It was super easy because of the Rayon crate so thanks to the creators and contributors of that. But overall fun problem, I really enjoyed working on it! It does kinda suck that the runtime is so slow, 4ms for both parts, but I assume it'll get worse as AoC continues, and the parallelization got rid of a whole millisecond, so whatever! I wonder if there's a faster way to do this problem?

Thank you to GitHub user [ageron](https://github.com/ageron) for the inspiration for this idea (and a lot of the code lol <3).
