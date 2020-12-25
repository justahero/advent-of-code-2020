# Day 01

[Advent Of Code Day 01](https://adventofcode.com/2020/day/1)

## Part 1

There is a input text file with a list of numbers, see [input.txt](./input.txt). Each line in this file contains a single number.

The basic idea is as follows.

* open text file
* read all numbers into a vec(?)
* find two numbers that add up to 2020
* read / check [Vec documentation](https://doc.rust-lang.org/std/vec/struct.Vec.html)
* multiply both numbers and print it

Maybe there is a better way to iterate over the `Vec` of numbers. One initial version is to use something like:

```rust
for i in 0..numbers.len() - 1 {
    let left = numbers[i];
    for j in (i + 1)..numbers.len() {
        let right = numbers[j];
        // calculate things
    }
}
```

This uses 2 `for` loops, the outer one the sequence over all numbers, the inner sequence from the successor index until the end of the list.

The result is as follows:

```
PAIR: 1209 + 811 = 2020
PRODUCT: 980499
```

## Part 2

Part 2 expands on the first part. Instead of finding a pair of numbers that sum up 2020, a tuple consisting of three numbers should be found whose sum is 2020.

This is a good opportunity to refine the existing algorithm to find pairs to use something more generic. For example the crate [itertools](https://docs.rs/itertools/0.9.0/itertools/) expands the existing Rust iterators to provide more functionality.


## Useful References / Solutions

* https://fasterthanli.me/series/advent-of-code-2020/part-1
