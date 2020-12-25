# Day 02

[Advent Of Code Day 02](https://adventofcode.com/2020/day/2).

* read in a list of password policies
* each line contains a policy
* a password policy contains
  * prefix, e.g. `1-3 a:`
  * the password, e.g. `abcde`
  * numbers `1-3` means character `a` needs to appear a minimum of 1, and maximum of 3
* check each line separately
* filter the list if policies
* count the number of valid passwords

## Part 1

First we use the [regex](https://docs.rs/regex/1.4.2/regex/) crate to parse strings with regular expressions. This splits the specific parts into their policy components. To find a good regular expression we can apply the [Rustexp](https://rustexp.lpil.uk/) is a good tool to figure out what the regex needs to look like.

