# Day 10

[Advent Of Code Day 10](https://adventofcode.com/2020/day/10).

## Part 1

There is a charging outlet produces the wrong number of **jolts**. Chain all joltage adapters.

* each joltage adapter is rated for a specific **output joltage**
* any given adapter can take 1, 2 or 3 jolts **lower** than its rating & still produce its rated output joltage
* the **device** to charge has a built-in joltage adapter rated for 3 jolts **higher** than the highest rated adapter
  * for example: adapters 3, 9, 6 would mean the device's built in adapter would be rated for 12 jolts
* the charging outlet near your seat has an effective joltage rating of 0
* use every adapter
* what is the distribution of joltage differences between the charging outlet, the adapters and the device?

Example, given the following adapters with joltage.

```
16
10
15
5
1
11
7
19
6
12
4
```

The device built-in joltage adapter would be rated 22 (`19 + 3`) jolts. The idea is to plug in all adapters in a compatible way until all adapters are used and the device can be charged.

* the initial joltage from the outlet is `0`
* only adapters with (the difference of) `1`, `2` or `3` joltage can be plugged in directly, in this case `1`
* next one is `4` due to difference of `3` jolts
* then `5`, `6` or `7` can be plugged in, in order to not skip one, the order is `5`, `6`, `7`
* after that `10` (only possible adapter)
* from `10` there are two possible ones `11` and `12`
* then `15`, `16`, `19`
* once all adapters are plugged in, count the number of jolt differences of `1` and `3`.
* resulting in 7 differences of 1, and 5 differences of 3

```
adapters:  0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22 
             +1    +1 +1 +1      +1  +1      +1          // 7
                +3           +3          +3      +3  +3  // 5
```

## Part 2

Given the list of `16 10 15 5 1 11 7 19 6 12 4` it generates the following list of differences

```
1 3 1 1 1 3 1 1 3 1 3 3
```

Find all arrangments of adapters that still produce a valid adapter chain.
It looks like calculating all possible arrangments is out of the question when there are *trillion* of combinations available. Instead this number can be calculated by the list of differences.

* find each group of differences with `1`, separated by `3`
* determine how to calculate options with list of `1`
* `3 1 3` - 1 possible arrangement
* `3 1 1 3` - 2 possible arrangements
* `3 1 1 1 3` - 4 possible arrangments

Analysis, given the following list of jolts and the differences

```
list: 0,  1,  4,  5,  6,  7, 10, 11, 12, 15, 16, 19, 22
          1   3   1   1   1   3   1   1   3   1   3   3
```

allows for the following arrangments

```
list: 0,  1,  4,  5,  6,  7, 10, 11, 12, 15, 16, 19, 22
      0,  1,  4,  5,  6,  7, 10,     12, 15, 16, 19, 22
      0,  1,  4,  5,      7, 10, 11, 12, 15, 16, 19, 22
      0,  1,  4,  5,      7, 10, 12, 15, 16, 19, 22
      0,  1,  4,      6,  7, 10, 11, 12, 15, 16, 19, 22
      0,  1,  4,      6,  7, 10,     12, 15, 16, 19, 22
      0,  1,  4,          7, 10, 11, 12, 15, 16, 19, 22
      0,  1,  4,          7, 10,     12, 15, 16, 19, 22
```

It seems instead of testing, the list of `1` groups can be calculated as a product of possible arrangments.
Please note that removing all inbetween `1` may result in a jolt difference larger than 3.

```
2, 3, 4, 5
2,    4, 5
2, 3,    5
2,       5
```

```
2, 3, 4, 5, 6
2,    4, 5, 6
2, 3,    5, 6
2, 3, 4,    6
2,       5, 6
2, 3,       6
2,    4,    6
-------------
   1, 1, 1, 1
```

For example given the sequence `2, 3, 4, 5, 6, 7` results in these arrangements.

```
2, 3, 4, 5, 6, 7
2, 3,    5, 6, 7
2, 3, 4,    6, 7
2, 3, 4, 5,    7
2, 3,       6, 7
2, 3, 4,       7
2,    4, 5, 6, 7
2,    4,    6, 7
2,    4, 5,    7
2,    4,       7
2,       5, 6, 7
2,       5,    7
----------------
   1  1  1  1  1
```

A difference group of `1, 1, 1, 1, 1` means there are `12` arrangments.

* 2 `1`s  
* 3 `1`s are 4
* 4 `1`s are 7
* 5 `1`s are 12
