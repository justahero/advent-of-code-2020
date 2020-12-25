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

The following attributes are parsed for each line / policy:

* min / max
* the 'character' to check
* password as string

Once all these attributes are available the [Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html) can be used to determine the number of occurrences for this particular char in the password string.

A few options are available:

* [Iterator::partition](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.partition) to create list of all chars
* [Iterator::filter](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter) to remove all other chars (used this for now)


## Part 2

This time the validation code requires the following constraints

* `1-3 a` is interpreted as the character `a` has to be found in either 1st or 3rd position
* positions are not 0-based, the first position is at index 1
