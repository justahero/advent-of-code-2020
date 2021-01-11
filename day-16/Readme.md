# Day 16

[Advent Of Code Day 16](https://adventofcode.com/2020/day/16).

## Part 1

Try to decode / learn the language to understand the text on the ticket.
You collect the **rules for ticket fields**, the **numbers on your ticket must have** and the **valid range**.

* you collect **rules for ticket fields**, **numbers on your ticket** and the **numbers on other tickets** for the same train service. (the puzzle input)
* rules for ticket fields specify a list of fields that exist **somewhere** on the ticket and the **valid ranges of values** for each field.
* example: a rule like `class: 1-3 or 5-7` means that one of the fields in every ticket is named `class` and can be any value in the ranges `1-3` or `5-7` (inclusive, such that `3` and `5` are both valid in this field, but `4` is not)
* each ticket is represented by single line of comma separated values.

## Part 2

* Discard invalid tickets entirely.
* use remaining valid tickets to determine which field is which

* using the valid ranges for each field, determine which order the fields appear on the tickets
* order is consistent between all tickets

For example given the input:

```
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
```

* based on nearby ticekts, first position must be **row**, second **class**, third must be **seat**
* you can conclude that in **your ticket**, class is `12`, row is `11` and seat is `13`
* once determined all fields, get all fields that start the word `departure`.
* multiply all six values together

Analysis

```
       rules |  class        |  row          |  seat
numbers      |  0-1 || 4-19  |  0-5 || 8-19  |  0-13 || 16-19
--------------------------------------------------------------
11, 3, 15, 5 |            -  |            x  |              -
12, 9, 1, 14 |            x  |            x  |              -
13, 18, 5, 9 |            x  |            x  |              x
```

Program output to verify is:

```
rule: 0
(1, [9, 1, 14, 12]),
(2, [18, 5, 9, 13]),

rule: 1
(0, [3, 15, 5, 11]),
(1, [9, 1, 14, 12]),
(2, [18, 5, 9, 13]),

rule: 2
(2, [18, 5, 9, 13]),
```

This means multiple candidate sets of numbers can be assigned to a single rule.
