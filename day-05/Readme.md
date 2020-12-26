# Day 05

[Advent Of Code Day 5](https://adventofcode.com/2020/day/5).

## Part 1

Binary Boarding

A boarding passs has the following structure: `BFBFFFFLRR`, where the first 7 letters (`BFBFFFF`) define the row, while the last 3 letters (`LRR`) define the seat in this row.

The first part of the boarding pass are instructions to find the row (0..127) via binary space partitioning.

* for each line in the text file parse the boarding pass
* split into two parts, the `row` (7 letters) and `column` (3 letters)
* use binary search on generated list to find element
  * use [slice#binary_search_by](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search_by) to get element
  * implement custom cmp function, but read ordering from given vec
* iterate over all boarding passes and calculate ids
* get max id of all passes


## Part 2

This part is more tricky, from the given list of boarding passes only a few are empty. Find the seat that is empty and is not in front or last row.

* determine all boarding passes, get rows and columns (instead of ids)
* map that somehow to an existing seating plan
* check which seat is left and is not front or back row

```
...
xxxx xxxx
xxxx xxxx
xxxx xx.x    <--- how to find this?
xxxx xxxx
...
```
