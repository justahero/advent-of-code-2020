use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type FoodResult = (Vec<Vec<String>>, HashMap<String, String>);

/// A single food
#[derive(Debug, Clone)]
struct Food {
    /// List of all ingredients for this food
    pub ingredients: Vec<String>,
    /// List of all allergens
    pub allergens: Vec<String>,
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
                    ingredients,
                    allergens,
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
fn unique_allergens(map: &[Food]) -> Vec<String> {
    map.iter()
        .flat_map(|item| item.allergens.clone())
        .unique()
        .collect_vec()
}

/// Filter the given map of allergens to ingredients to the remaining ingredients.
fn filter_allergens(map: &[Food]) -> anyhow::Result<FoodResult> {
    let mut food = map.to_vec();
    let mut allergens_map = HashMap::new();

    loop {
        // get all unique allergens
        let allergens = unique_allergens(&food);

        if allergens.is_empty() {
            break;
        }

        for allergen in &allergens {
            let items = food
                .iter()
                .filter(|&item| item.allergens.contains(&allergen))
                .collect::<Vec<_>>();

            if !items.is_empty() {
                let ingredient = items[0].ingredients.iter().cloned().collect::<HashSet<_>>();

                let result = items
                    .iter()
                    .skip(1)
                    .fold(ingredient, |acc, &item| {
                        let other = &item.ingredients.iter().cloned().collect::<HashSet<_>>();
                        acc.intersection(other).cloned().collect()
                    })
                    .into_iter()
                    .collect::<Vec<_>>();

                if result.len() == 1 {
                    // remove allergen / ingredient pair from food
                    let ingredient = &result[0];
                    for item in &mut food {
                        if let Some(pos) = item.allergens.iter().position(|x| x == allergen) {
                            item.allergens.remove(pos);
                        }
                        if let Some(pos) = item.ingredients.iter().position(|x| x == ingredient) {
                            item.ingredients.remove(pos);
                        }
                    }

                    // add pair of allergen to ingredient
                    allergens_map.insert(allergen.clone(), ingredient.clone());

                    break;
                }
            }
        }
    }

    let items = food.iter().map(|x| x.ingredients.clone()).collect::<Vec<_>>();

    Ok((items, allergens_map))
}

/// Sorts all ingredients and creates a single string
fn ingredients_to_string(map: &HashMap<String, String>) -> String {
    let mut result = Vec::new();
    for key in map.keys().sorted() {
        result.push(map.get(key).unwrap().clone());
    }
    result.join(",")
}

fn main() -> anyhow::Result<()> {
    let food = parse_food(include_str!("food.txt"))?;

    let (remaining_ingredients, allergens) = filter_allergens(&food)?;
    let count = remaining_ingredients
        .iter()
        .map(|list| list.len())
        .sum::<usize>();

    assert_eq!(2826, count);

    let sorted_ingredients = ingredients_to_string(&allergens);
    dbg!(sorted_ingredients);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{filter_allergens, ingredients_to_string, parse_food, parse_rule, unique_allergens};

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
        assert!(ingredients.contains(&String::from("mxmxvkd")));
        assert!(ingredients.contains(&String::from("kfcds")));
        assert!(ingredients.contains(&String::from("sqjhc")));
        assert!(ingredients.contains(&String::from("nhms")));

        let allergens = &result[0].allergens;
        assert_eq!(2, allergens.len());
        assert!(allergens.contains(&String::from("dairy")));
        assert!(allergens.contains(&String::from("fish")));
    }

    #[test]
    fn test_unique_allergens() {
        let food = parse_food(FOOD).unwrap();
        
        assert_eq!(
            vec![String::from("dairy"), String::from("fish"), String::from("soy")],
            unique_allergens(&food),
        );
    }

    #[test]
    fn test_filter_allergens() {
        let food = parse_food(FOOD).unwrap();
        let (ingredients, allergens) = filter_allergens(&food).unwrap();

        let expected_ingredients: Vec<Vec<String>> = vec![
            vec!["kfcds".into(), "nhms".into()],
            vec!["trh".into(), "sbzzf".into()],
            vec![],
            vec!["sbzzf".into()],
        ];
        assert_eq!(4, ingredients.len());
        assert_eq!(expected_ingredients, ingredients);

        assert_eq!(Some(&String::from("mxmxvkd")), allergens.get("dairy"));
        assert_eq!(Some(&String::from("sqjhc")), allergens.get("fish"));
        assert_eq!(Some(&String::from("fvjkl")), allergens.get("soy"));
    }

    #[test]
    fn test_sort_ingredients() {
        let food = parse_food(FOOD).unwrap();
        let (_, allergens) = filter_allergens(&food).unwrap();

        assert_eq!(String::from("mxmxvkd,sqjhc,fvjkl"), ingredients_to_string(&allergens));
    }
}
