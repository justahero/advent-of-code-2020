# Day 15

[Advent Of Code Day 15](https://adventofcode.com/2020/day/15).

## Part 1

A type of memory game. The rules are:

* players take turns (number of players do not matter?)
* there is a list of starting numbers
* each turn consists of considering the **most recently spoken number**
* if that was the **first** time the number has been spoken, the current player says `0`
* otherwise the number had been spoken before; the current player announces **how many turns apart** the number is from when it was previously spoken (**age**)

Given the following example:

```
starter sequence
[0, 3, 6]

list                            last    new value
------------------------------------------------
[0, 3]                           (6) -> 0 = 0
[0, 3, 6]                        (0) -> 3 - 0 = 3
[0, 3, 6, 0]                     (3) -> 4 - 1 = 3
[0, 3, 6, 0, 3]                  (3) -> 5 - 4 = 1
[0, 3, 6, 0, 3, 3]               (1) -> 0 = 0
[0, 3, 6, 0, 3, 3, 1]            (0) -> 7 - 3 = 4
[0, 3, 6, 0, 3, 3, 1, 0]         (4) -> 0 = 0
[0, 3, 6, 0, 3, 3, 1, 0, 4]      (0) -> 9 - 7 = 2
[0, 3, 6, 0, 3, 3, 1, 0, 4, 0]   (2) ...
```

these turns follow

* `turn 1`: the 1st number spoken is a starting number: `0`
* `turn 2`: the 2nd number spoken is a starting number: `3`
* `turn 3`: the 2nd number spoken is a starting number: `6`
* `turn 4`: now, consider the last number spoken (`6`), since it was first time, 4th number is `0`
* `turn 5`: next, consider last number spoken (`0`), it had been spoken before, therefore next number to speak is the difference between turn number when it was last spoken (previous turn `4`) and the turn number of the time was most recently spoken before then (turn `1`), therefore `4 - 1 = 3`
* `turn 6`: last number spoken `3` (turn `5`), had been spoken before on previous turn `2`, therefore `5 - 2 = 3`
* `turn 7`: since last number spoken is `3` and it was spoken the turn before that, the diff is `1`
* `turn 8`: since `1` is a new number, the number for this turn is `0`
* etc..
