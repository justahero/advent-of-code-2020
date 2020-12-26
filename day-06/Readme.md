# Day 06

[Advent Of Code Day 6](https://adventofcode.com/2020/day/6).

## Part 1

This challenge requires a better understanding, after reading it the first it was not necessarily clear how it works. Therefore the challenge is re-iterated here as I understood it.

* there are 26 yes / no question (letters of the alphabet)
* questions whose answer is "yes" are written down one per line
```
abcx
abcy
abcz
```
* concludes answers of 3 people, "yes" answers for *a*, *b*, *c*, *x*, *y*, *z* weren given
* same questions found in a group are redundant, given example means total answers of 6
* every group is separated by a blank line
* within each group a line contains the answers of one person

For example given the following input:

```
abc

a
b
c

ab
ac

a
a
a
a

b
```

There are 5 groups in total

* 1 - 1 person, 3 answers
* 2 - 3 persons, 1 answer each = 3 answers
* 3 - 2 persons, 3 answers (*a* is redundant)
* 4 - 4 persons, 1 answer (all answered *a*)
* 5 - 1 person, 1 answer

The task is to count all positive answers from all groups. For the given example the total amount is:

```
3 + 3 + 3 + 1 + 1 = 11
```

a total of 11.

The algorithm uses the [itertools#unique](https://docs.rs/itertools/0.7.6/itertools/trait.Itertools.html#method.unique) function to generate an iterator over all unique entries.

* each line contains unique answers


## Part 2


