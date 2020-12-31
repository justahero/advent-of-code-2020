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

## Part 2

The instructions are now interpreted as follows. Instead of starting the instruction with ship pos `(0, 0)`, the instructions are given in relation to a **waypoint** relative to the ship's position, e.g. `(10, 1)` (10 units east and 1 unit north).

* **N** means to move the waypoint north by the given value
* **S** means to move the waypoint south by the given value
* **E** means to move the waypoint east by the given value
* **W** means to move the waypoint webst by the given value
* **L** means to rotate the waypoint around the ship (counter-clockwise) the given number of degrees
* **R** means to rotate the waypoint around the ship (clockwise) the given number of degrees
* **F** means to move forward to the waypoint a number of times equal the to the given value
* the waypoint starts at (east: 10, north: 1) relative to the ship
* if the ship moves, the waypoint moves with it

Example, given the previous instructions:

```
F10
N3
F7
R90
F11
```

* `F10` moves the ship toward to the waypoint 10 times (100 units east, 10 units north)
* `N3` moves the waypoint 
