# Day 23

[Advent Of Code Day 23](https://adventofcode.com/2020/day/23).

## Part 1

Small crab game challenge:

* cups will be arranged in a circle, labelled **clockwise**
* for exampe: `32415`, 5 cups
* first cup in list is the **current** cup
* crab is then going to do `100` units

Each move the crab does the following actions

* crab picks up **three** cups that are immediately **clockwise** of the **current** cup
  * these are removed from the circle
* crab select a **destination cup**, the cup with a **label** equal to the **current** cup's label minus one
  * if this would select one of the cups that was just picked up, the cup will keep subtracting one until it finds a cup that wasn't just picked up.
  * wraps around the cup circle if necessary
* crab places the cups it just picked up so that they are **immediately clockwise** of the destination cup, they keep the same order
* crab selects a new **current** cup, the cup which is immediately clockwise of the current cup

Example:

```
389125467
```

After 10 moves

```
-- move 1
cups: (3) 8 9 1 2 5 4 6 7    // 3 - 1 = 2
pick up: 8, 9, 1
destination: 2

-- move 2
cups: 3 (2) 8 9 1 5 4 6 7    // 2 - 1 = 1
pick up: 8, 9, 1
destination: 7


2 8 9 1 5 4 6 7 3   // 2 - 1 = 1
2 5 4 6 7 3         // [8, 9, 1]

```

## Part 2

Make this function more efficient, set up a much bigger starting list.

* `1_000_000` cups
* initial set of cups still correct, then increasing number of labels up to 1 million
* `10_000_000` million moves
