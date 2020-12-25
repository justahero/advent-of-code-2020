# Day 04

[Advent Of Code Day 4](https://adventofcode.com/2020/day/4).

## Part 1

Parse the lines from the text files, group all lines into blocks separated by empty lines. This seems the more difficult part.

The approach is to split somehow the lines by an empty lines, check if there is a good way to partition / group lines separated by empty lines.

* [itertools#group_by](https://docs.rs/itertools/0.8.0/itertools/trait.Itertools.html#method.group_by)


## Part 2