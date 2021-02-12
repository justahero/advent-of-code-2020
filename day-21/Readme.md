# Day 21

[Advent Of Code Day 21](https://adventofcode.com/2020/day/21).

## Part 1

* Ingredients in an unknown local language.
* Allergens are in a language that can be read.
* one food per line

Some constraints / rules

* each allergen is found in exactly one ingredient
* each ingredient contains 0 or more allergens
* allergens are not always marked
  * when they are listed, the ingredient that contains each listed allergen will be **somewhere in the corresponding ingredients list**.
  * however, if an allergen is not listed, the ingredient that contains the allergen could still be present

Example

```
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
```

* first food item has `4` ingredients: `mxmxvkd`, `kfcds`, `sqjhc`, and `nhms`
* contains at least two allergens: `dairy`, `fish`
* determine which ingredients can't possibly contain any of the allergens in any food in your list
* none of the ingredients `kfcds`, `nhms`, `sbzzf`, `trh` can contain any allergen

Detailed analysis

```
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
```

Solution

* count the number of occurrences of all ingredients that do not contain any allergens.