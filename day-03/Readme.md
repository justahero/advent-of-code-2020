# Day 03

[Advent of Code Day 3](https://adventofcode.com/2020/day/3).

## Part 1

Requirements

* the map consists of width / height entries
* the width is repeated / cycled over and over
* movement down requires to adjust the map horizontally, integrate cycling
* it's over once the toboggan travelled the full height of the map
* it seems a horizontal pointer needs to be kept and put into the next row
* it seems it is worth to keep the width of the map

A position with x, y coordinates is used to navigate the grid / map. For now [Iterator#cycle](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cycle) is used to repeatedly find the x coordinate.


## Part 2

Main goal is to have a function that calculates the tree count and call it several times. Then multiply all counts.
