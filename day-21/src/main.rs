use std::collections::HashSet;
use std::iter::FromIterator;

use itertools::Itertools;

/// A single food
#[derive(Debug, Clone)]
struct Food {
    /// List of all ingredients for this food
    pub ingredients: HashSet<String>,
    /// List of all allergens
    pub allergens: HashSet<String>,
}

peg::parser!{
    grammar food_parser() for str {
        /// White spaces
        rule ws()
            = " "

        /// Comma separated list
        rule comma()
            = ", "

        /// A single word
        rule word() -> String
            = s:$(['a'..='z' | 'A'..='Z']+) { s.into() }

        rule ingredients() -> Vec<String>
            = (i:word() ws()* { i })*

        rule allergens() -> Vec<String>
            = (a:word() comma()* { a })*

        pub(crate) rule parse() -> Food
            = ingredients:ingredients() "(contains " allergens:allergens() ")" {
                Food {
                    ingredients: ingredients.iter().cloned().collect(),
                    allergens: allergens.iter().cloned().collect(),
                }
            }
    }
}

/// Parses a single food rule
fn parse_rule(line: &str) -> anyhow::Result<Food> {
    Ok(food_parser::parse(line)?)
}

/// Parses the list of food, line by line
fn parse_food(content: &str) -> anyhow::Result<Vec<Food>> {
    let food = content
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .map(|line| parse_rule(line))
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    Ok(food)
}

/// Get the list of unique allergens
fn unique_allergens(map: &[Food]) -> HashSet<String> {
    let mut result = HashSet::new();
    for item in map {
        for allergen in &item.allergens {
            result.insert(allergen.clone());
        }
    }
    result
}

/// Filter the given map of allergens to ingredients to the remaining ingredients.
fn filter_allergens(map: &[Food]) -> anyhow::Result<Vec<Vec<String>>> {
    let mut food = map.iter().cloned().collect::<Vec<_>>();

    loop {
        // get all unique allergens
        let mut allergens = unique_allergens(&food);

        if allergens.is_empty() {
            break;
        }

        for allergen in allergens.drain() {
            let ingredients = food
                .iter()
                .filter(|&item| item.allergens.contains(&allergen))
                .map(|item| HashSet::from_iter(item.ingredients.clone()))
                .collect::<Vec<HashSet<String>>>();

            if !ingredients.is_empty() {
                let result = ingredients
                    .iter()
                    .skip(1)
                    .fold(ingredients[0].clone(), |acc, item| {
                        acc.intersection(item).cloned().collect()
                    })
                    .into_iter()
                    .collect_vec();

                if result.len() == 1 {
                    // remove allergen / ingredient pair from food
                    for item in &mut food {
                        item.allergens.remove(&allergen);
                        item.ingredients.remove(&result[0]);
                    }
                    break;
                }
            }
        }
    }

    let food = food
        .iter()
        .map(|item| {
            let mut items = item.ingredients.iter().cloned().collect::<Vec<_>>();
            items.sort();
            items
        })
        .collect::<Vec<_>>();

    Ok(food)
}

fn main() -> anyhow::Result<()> {
    let food = parse_food(include_str!("food.txt"))?;

    let remaining_ingredients = filter_allergens(&food)?;
    let count = remaining_ingredients
        .iter()
        .map(|list| list.len())
        .sum::<usize>();

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{filter_allergens, parse_food, parse_rule, unique_allergens};

    const FOOD: &str = r#"
        mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
        trh fvjkl sbzzf mxmxvkd (contains dairy)
        sqjhc fvjkl (contains soy)
        sqjhc mxmxvkd sbzzf (contains fish)
    "#;

    #[test]
    fn test_parse_rule() {
        assert!(parse_rule("sqjhc fvjkl (contains soy)").is_ok());
        assert!(parse_rule("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)").is_ok());
    }

    #[test]
    fn test_parse_food() {
        let result = parse_food(FOOD).unwrap();
        assert_eq!(4, result.len());

        let ingredients = &result[0].ingredients;
        assert_eq!(4, ingredients.len());
        assert!(ingredients.contains("mxmxvkd"));
        assert!(ingredients.contains("kfcds"));
        assert!(ingredients.contains("sqjhc"));
        assert!(ingredients.contains("nhms"));

        let allergens = &result[0].allergens;
        assert_eq!(2, allergens.len());
        assert!(allergens.contains("dairy"));
        assert!(allergens.contains("fish"));
    }

    #[test]
    fn test_unique_allergens() {
        let food = parse_food(FOOD).unwrap();
        
        assert_eq!(
            vec![String::from("dairy"), String::from("fish"), String::from("soy")].into_iter().collect::<HashSet<_>>(),
            unique_allergens(&food),
        );
    }

    #[test]
    fn test_filter_allergens() {
        let food = parse_food(FOOD).unwrap();
        let result = filter_allergens(&food).unwrap();

        let expected: Vec<Vec<String>> = vec![
            vec!["kfcds".into(), "nhms".into()],
            vec!["sbzzf".into(), "trh".into()],
            vec![],
            vec!["sbzzf".into()],
        ];

        assert_eq!(4, result.len());
        assert_eq!(expected, result);
    }
}