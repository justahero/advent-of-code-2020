# Day 16

[Advent Of Code Day 16](https://adventofcode.com/2020/day/16).

## Part 1

Try to decode / learn the language to understand the text on the ticket.
You collect the **rules for ticket fields**, the **numbers on your ticket must have** and the **valid range**.

* you collect **rules for ticket fields**, **numbers on your ticket** and the **numbers on other tickets** for the same train service. (the puzzle input)
* rules for ticket fields specify a list of fields that exist **somewhere** on the ticket and the **valid ranges of values** for each field.
* example: a rule like `class: 1-3 or 5-7` means that one of the fields in every ticket is named `class` and can be any value in the ranges `1-3` or `5-7` (inclusive, such that `3` and `5` are both valid in this field, but `4` is not)
* each ticket is represented by single line of comma separated values.