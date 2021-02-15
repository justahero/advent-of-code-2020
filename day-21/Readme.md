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

=> first step, map allergens to possible ingredient lists

dairy = [{mxmxvkd, kfcds, sqjhc, nhms}, {trh, fvjkl, sbzzf, mxmxvkd}]
fish  = [{mxmxvkd, kfcds, sqjhc, nhms}, {sqjhc, mxmxvkd, sbzzf}]
soy   = [{sqjhc, fvjkl}]

=> iterate over this list and find unique ingredient in lists

* union of all lists per line => dairy = mxmxvkd
* then remove found ingredient from all other lists
* repeat until all allergens are associated
```

Steps to filter map

```
0. initial
{dairy: [mxmxvkd, kfcds, sqjhc, nhms], fish: [mxmxvkd, kfcds, sqjhc, nhms]}
{dairy: [trh, fvjkl, sbzzf, mxmxvkd]}
{soy: [sqjhc, fvjkl]}
{fish: [sqjhc, mxmxvkd, sbzzf]}

1. dairy: mxmxvkd
{fish: [kfcds, sqjhc, nhms]}
{dairy: [trh, fvjkl, sbzzf]} ✅
{soy: [sqjhc, fvjkl]}
{fish: [sqjhc, sbzzf]}

2. dairy: mxmxvkd, fish: sqjhc
{fish: [kfcds, nhms]} ✅
{dairy: [trh, fvjkl, sbzzf]} ✅
{soy: [fvjkl]}
{fish: [sbzzf]} ✅

3. dairy: mxmxvkd, fish: sqjhc, soy: fvjkl
{fish: [kfcds, nhms]} ✅
{dairy: [trh, sbzzf]} ✅
{soy: []} ✅
{fish: [sbzzf]} ✅
```

Solution

* count the number of occurrences of all ingredients that do not contain any allergens.
