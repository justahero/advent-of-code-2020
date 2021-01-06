# Day 14

[Advent Of Code Day 14](https://adventofcode.com/2020/day/14).

## Part 1

Bit mask problem, your mission is to connect the computer system from the sea port with the docking program on the ferry.

The initialization program can either update the bitmask or write to a value in memory.

* values and memory addresses are both 36-unsigned integers
* `mem[8] = 11` means writing value `11` to memory address `8`.
* bitmask is always given as a String of 36 bits, most significant bit on the left, least on the right
* the current bitmask is applied to values immediately before they are written to memory: a `0` or `1` overwrites the corresponding bit in the value, while an `X` leaves the bit in the value unchanged.
* the entire 36 bit address space begins initialized to the value `0` at every address

Example:

```
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
```

* this program starts by specifying the bitmask (`mask = ...`)
* the mask it specifies will overwrite two bits in every written value:
  * bit `2` bit is overwritten with `0`
  * bit `7` bit is overwritten with `1`
* the program then attempts to write the value `11` to memory address `8`

```
value:  000000000000000000000000000000001011  (decimal 11)
mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
result: 000000000000000000000000000001001001  (decimal 73)
```

* the value `73` is written to memory address `8` instead
* then the program tries to write `101` to memory address `7`

```
value:  000000000000000000000000000001100101  (decimal 101)
mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
result: 000000000000000000000000000001100101  (decimal 101)
```

* this time the value is not affected by the bit mask, value `101` is written to memory address `7`
* last the program tries to write `0` to memory address `8`

```
value:  000000000000000000000000000000000000  (decimal 0)
mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
result: 000000000000000000000000000001000000  (decimal 64)
```

* the value `64` is written to address `8` instead, overwriting the previous value at this memory address

The result is the sum of all values left in memory after initialization completes.

In the example above only two values in memory are not `0`, producing a sum of `165`.
