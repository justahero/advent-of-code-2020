# Day 07

[Advent Of Code Day 7](https://adventofcode.com/2020/day/7).

## Part 1

Luggage regulations are given in the example input. Given the following example set of regulation rules:

```
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
```

These rules define

* 9 different bag rules (maybe keys or nodes?)
* bags are color coded & must contain specific quantities of other color coded bags
* for this particular example the number of outer bags is searched to hold a *shiny gold bag*

With the above rules the following options are available to hold a *shiny gold bag*

* a *bright white bag* can hold 1 *shiny gold bag* directly
* a *muted yellow bag* can hold 2 *shiny gold bag* directly
* a *dark orange bag* can hold a *bright white bag* and therefore hold a *shiny gold bag*
* a *light red bag* can hold a *bright white bag* and therefore hold a *shiny gold bag*

### Ideas

* maybe it's a good candidate to build a directed acyclic graph that can be traversed?
  * a lot of repetition maybe?
* set of rules to look up and navigate in and apply these rules in an exhaustive way?
* what is a good underlying structure to hold all relations / rules?


## Part 2
