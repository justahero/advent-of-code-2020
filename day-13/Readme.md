# Day 13

[Advent Of Code Day 13](https://adventofcode.com/2020/day/13).

## Part 1

**Shuttle Search**: travel mode is bus

* each bus has an **ID** number that also indicates how **often** the bus leaves for the airport
* bus schedules are defined based on a **timestamp** that measures the **number of minutes** since some fixed reference point.
* at timestamp `0` every bus simultaneously departed from the sea port to the air port.
* after that each bus travels to the airport (then various other locations), finally returns to the sea port to repeat the journey
* the time this loop takes a particular bus is also its **ID** number
  * bus with number `5` departs from sea port at timestamps: `0`, `5`, `10`, `15`, `20` ...
  * in case you are there you can take the bus

Goal is to figure out the earliest **timestamp** you could depart on a bus

Example:

```
939
7,13,x,x,59,x,31,19
```

Earliest timestamp is `939`, bus ids in service are: `7, 13, 59, 31, 19`.

The bus schedule around timestamp `939` is:

```
time   bus 7   bus 13  bus 59  bus 31  bus 19
929      .       .       .       .       .
930      .       .       .       D       .
931      D       .       .       .       D
932      .       .       .       .       .
933      .       .       .       .       .
934      .       .       .       .       .
935      .       .       .       .       .
936      .       D       .       .       .
937      .       .       .       .       .
938      D       .       .       .       .
939      .       .       .       .       .
940      .       .       .       .       .
941      .       .       .       .       .
942      .       .       .       .       .
943      .       .       .       .       .
944      .       .       D       .       .
945      D       .       .       .       .
946      .       .       .       .       .
947      .       .       .       .       .
948      .       .       .       .       .
949      .       D       .       .       .
```

which means bus `59` departs at timestamp `944`. You have to wait `5` minutes.

Solution:

Multiply the ID of the earliest bus by the number of minutes you have to wait. In the example above that's: `5 * 59 = 295`.


## Part 2

This can be computationally more complex if a naive approach is taken. For example testing increasing timestamp until the set of all bus ids result with correct positions can take a long time.

Given the following bus ids:

```
67,7,59,61
```

* the first timestamp needs to fit departure time of the bus
* next bus `7` needs to depart one minute later
* bus `59` departs one minute later
* bus `61` departs one minute later

```
time   | 67 |  7 | 59 | 61 |
----------------------------
754015 |  - |  - |  - |  - |
754016 |  - |  - |  - |  - |
754017 |  - |  - |  - |  - |
754018 |  D |  - |  - |  - |  =  11_254
754019 |  - |  D |  - |  - |  = 107_717
754020 |  - |  - |  D |  - |  =  12_780
754021 |  - |  - |  - |  D |  =  12_361
754022 |  - |  - |  - |  - |
754023 |  - |  - |  - |  - |
```

These numbers have something to do with prime numbers, which makes it more efficient to calculate the timestamp from.
Maybe we should check each number if it's a prime and if so then just calculate the timestamp?

Ok, all are primes, given the examples and the input, which makes it fairly easy to calculate the final number:

```
67 * 7 * 59 * 61 + 1 + 2 + 3?

67 * 6 * 57 * 58

754018
```
