# Day 17

[Advent Of Code Day 17](https://adventofcode.com/2020/day/17).

## Part 1

Fix experimental energy source using cutting edge technology: a set of Conway Cubes.
There is an infinite 3-dimensional grib. At every integer based coordinates (x, y, z) there exists a single cube with either **active** or **inactive**.

* initial state of the pocket dimension, almost all cubes start **inactive**
* only exception to this is a small region of cubes (puzzle input), these cubes start in either **active** or **inactive** state
* each cube only considers its **neighbors**, any of the 26? other cubes where any of their coordinates differ by at most 1.

For example given cube at `(1, 2, 3)` its neighbors include cube at `(2,2,2)`, `(0,2,3)` etc.

During a cycle, **all** cubes **simultaneously** change their state according to the following rules

* if cube is **active** and **exactly** `2` or `3` of its neighbors are also **active** the cube remains **active**, otherwise becomes **inactive**
* if cube is **inactive** but **exactly** `3`  of its neighbors are **active**, the cube becomes **active** otherwise remains **inactive**

Your task is to simulate the energy boot sequence of 6 cycles to determine the configuration.