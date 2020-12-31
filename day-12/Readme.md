# Day 12

[Advent Of Code Day 12](https://adventofcode.com/2020/day/12).

## Part 1

Navigate the ferry by following the instructions.

* input consists of sequence of instructions, single character **action** paired with **integer** values

The following actions are available

* **N** - move *north* by given value, e.g. `N10` move north by 10 steps
* **S** - move *south* by given value
* **E** - move *east* by given value
* **W** - move *west* by given value
* **L** - turn *left* by given number of degrees
* **R** - turn *right* by given number of degrees
* **F** - move *forward* by the given value in the direction the ship is currently facting

A few more constraints

* ship starts facing *east*.
* only `L` and `R` change direction of the ship
* compass movements do not change direction
* initial position of ship is `0, 0`
* looks like turns are in `90` degree angles

For example, given the following instructions:

```
F10
N3
F7
R90
F11
```

These instructions would be handled as follows:

* `F10` moves ship forward 10 units: `east: 10, north: 0`
* `N3` moves ship **north**: `east: 10, north: 3`
* `F7` moves ship **east**: `east: 17, north: 3`
* `R90` turns ship **right**: `east: 17, north: 3`
* `F11` moves ship **forward**: `east 17, south: 8`

Calculate the [Manhattan Distance](https://en.wikipedia.org/wiki/Taxicab_geometry) from the coordinates by summing up both east / west and north / south distances. For example `east: 17, south: 8` means `17 + 8 = 25`.

Idea

* use a movement vector of (x, y) where a pair can point to one of the cardinal directions.
* a turn changes the movement vector
