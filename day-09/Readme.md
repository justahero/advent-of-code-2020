# Day 9

[Advent Of Code Day 9](https://adventofcode.com/2020/day/9).

## Part 1

Analyse the data from the port, decode XMAS cypher. The following rules are given.

* 25 numbers are the preamble
* each number after that must be the sum of a pair of two previous numbers
* the pair of two numbers must be different
* there may be more than one pair that fits the sum
  * e.g. `10 = [1 + 9, 2 + 8, 3 + 7, 4 + 6, ...]`
* find the first number that does not is a sum

Ideas

* use a slice / window as preamble, move this slice along the list

For example a preamble of `5` with the following sequence is moved as follows

```
01 02 03 04 05 06 07 08 09 10 11
-------------- xx
   -------------- xx
      -------------- xx
         -------------- xx
            -------------- xx
               -------------- xx
```

* iterate over list of numbers, take preamble and next value
* use a function that takes a [slice](https://doc.rust-lang.org/std/vec/struct.Vec.html) of a list and the next value to check if next value is a sum of two numbers from the preamble, stop at the first pair?
* use [itertools](https://docs.rs/itertools/0.10.0/itertools/structs/struct.Combinations.html) combinations to get iterator over pairs of numbers
  * filter all pairs that have same value
  * calculate resulting list of all sums?
  * check if at least one sum is found
