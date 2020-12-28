# Day 11

[Advent Of Code Day 11](https://adventofcode.com/2020/day/11).

## Part 1

Seat plan in the waiting area, filling in a seat layout, similar to [Conway's Game Of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

* seat layout fits a grid (rows / rows)
* each position in the layout is either `.` (floor), `L` (empty seat), `#` (occupied)
* the goal is to "predict" / determine how people will seat
* all decisions are based on the number of occupied seats adjacent to a given seat

```
L.L
#?.
LL.
```

where `?` is the seat in question.

The following seat rules are applied to every seat simultaneously

* if a seat is empty (`L`) and there are `0` occupied seats adjacent to it, the seat becomes occupied
* if a seat is occupied (`#`) and for or more seats adjacent to it are also occupied, the seat becomes **empty**
* otherwise the seat's state does not change
* floor `.` never changes

Given the following initial seat plan

```
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
```

After one round of rules, every seat in the layout becomes occupied:

```
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
```

Then after the second round:

```
#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##
```

This goes on for another three rounds until the seat stabilizes.

round 3

```
#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##
```

round 4

```
#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##
```

round 5

```
#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##
```

Finally, count the number occupied seats, in this case 7.
