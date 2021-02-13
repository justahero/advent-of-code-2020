use std::collections::HashMap;

/// A single food
#[derive(Debug)]
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
            = ingredients:ingredients() "(contains " allergens:allergens() ")" { Food { ingredients, allergens } }
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

/// Links all allergens to ingredients
fn map_allergens(items: &[Food]) -> HashMap<String, Vec<Vec<String>>> {
    items
        .iter()
        .fold(HashMap::new(), |mut result, item| {
            item.allergens
                .iter()
                .for_each(|allergen| {
                    if !result.contains_key(allergen) {
                        result.insert(allergen.clone(), Vec::new());
                    }

                    let list = result.get_mut(allergen).unwrap();
                    list.push(item.ingredients.clone());
                });
            result
        })
}

fn main() -> anyhow::Result<()> {
    let food = parse_food(include_str!("food.txt"))?;

    map_allergens(&food);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{map_allergens, parse_food, parse_rule};

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
        assert_eq!(vec!["mxmxvkd", "kfcds", "sqjhc", "nhms"], result[0].ingredients);
        assert_eq!(vec!["dairy", "fish"], result[0].allergens);
    }

    #[test]
    fn test_map_allergens() {
        let food = parse_food(FOOD).unwrap();
        let result = map_allergens(&food);

        assert_eq!(3, result.len());
        assert_eq!(
            &vec![
                vec!["mxmxvkd", "kfcds", "sqjhc", "nhms"],
                vec!["trh", "fvjkl", "sbzzf", "mxmxvkd"],
            ],
            result.get("dairy").unwrap()
        );
        assert_eq!(
            &vec![
                vec!["mxmxvkd", "kfcds", "sqjhc", "nhms"],
                vec!["sqjhc", "mxmxvkd", "sbzzf"],
            ],
            result.get("fish").unwrap(),
        );
        assert_eq!(
            &vec![vec!["sqjhc", "fvjkl"]],
            result.get("soy").unwrap(),
        );
    }
}