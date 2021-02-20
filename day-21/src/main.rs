use itertools::Itertools;

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

/// Get the list of unique allergens
fn unique_allergens(map: &[Food]) -> Vec<String> {
    map
        .iter()
        .fold(Vec::new(), |mut allergens, food| {
            allergens.append(&mut food.allergens.clone());
            allergens
        })
        .into_iter()
        .unique()
        .collect::<Vec<_>>()
}

/// Filter the given map of allergens to ingredients to the remaining ingredients.
fn filter_allergens(map: &[Food]) -> Vec<Vec<String>> {
    // get all unique allergens

    dbg!(map);
    Vec::new()
}

fn main() -> anyhow::Result<()> {
    let food = parse_food(include_str!("food.txt"))?;

    let _map = filter_allergens(&food);

    Ok(())
}

#[cfg(test)]
mod tests {
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
        assert_eq!(vec!["mxmxvkd", "kfcds", "sqjhc", "nhms"], result[0].ingredients);
        assert_eq!(vec!["dairy", "fish"], result[0].allergens);
    }

    #[test]
    fn test_unique_allergens() {
        let food = parse_food(FOOD).unwrap();
        assert_eq!(
            vec!["dairy", "fish", "soy"],
            unique_allergens(&food),
        );
    }

    #[test]
    fn test_filter_allergens() {
        let food = parse_food(FOOD).unwrap();
        let result = filter_allergens(&food);

        let expected: Vec<Vec<String>> = vec![
            vec!["kfcds".into(), "nhms".into()],
            vec!["trh".into(), "sbzzf".into()],
            vec![],
            vec!["sbzzf".into()],
        ];

        assert_eq!(4, result.len());
        assert_eq!(expected, result);
    }
}