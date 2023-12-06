# Advent of Code 2023

<p>I've never used Rust before, so this challenge provided a good opportunity to learn this language.</p>

<p>Some comments about each day:

1. This was my first foray into Rust territory and struggled with the compiler so that the fairly simple task took a few hours.
2. Rust felt easier on this day, and the exercise was, perhaps also a little easier than the first day's exercise.
3. The day's exercise was quite hard for me. I wasn't in the best of shape for coding and couldn't find a simple and fast way of implementing the solution. I actually finished the first part just before the next day's exercise was revealed and the second part some time after that. The implementation is very slow (O(n^2), I believe) but I didn't want to spend any more time with it.
4. This one wasn't too hard to solve but my implementation for part 2 is quite unoptimal. It does give the result within a reasonable time but it could be optimized way better.
5. I didn't have much trouble coming up with the solution for this day's exercise but implementation took time, especially for the second part. The solution is ugly, I think, because handling each different "fitting scenario" had to be coded separately. It seems to perform very well, however, so I was quite happy with it. The idea in the second part is to handle range of values instead of single values, and split the ranges when they cannot (completely) fit within the range of the filter.
6. I came up with the solution and implementation for this one very easily. The solution for both parts are essentially the same -- the parsing was just about the only thing that had to be changed.
</p>
