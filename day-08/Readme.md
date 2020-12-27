# Day 08

[Advent Of Code Day 8](https://adventofcode.com/2020/day/8).

## Part 1

Fix a handheld game console, the boot code contains an infinite loop. There are three instructions that can be executed by the program:

* `nop` is a no-op instruction, nothing is executed, the next instruction below is executed
* `acc` adds or subtracts the given number to the *accumulator*, a global register holding a value
  * the initial value is `0`
* `jmp` moves the current instruction cursor / counter to a new line
  * a negative value jumps above
  * a positive value jumps below

Start processing instructions from the first line of the input.
Execute the process until an instruction on the same line was previously run, then the infinite loop is found.
Detect this state, once it occurs check the *accumular* value and display it.

## Part 2
