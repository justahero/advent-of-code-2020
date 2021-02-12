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
            = ingredients:ingredients() "(contains " allergens:allergens() ")" { Food { ingredients, allergens: Vec::new() } }
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

fn main() -> anyhow::Result<()> {
    let food = parse_food(include_str!("food.txt"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{parse_food, parse_rule};

    #[test]
    fn test_parse_rule() {
        assert!(parse_rule("sqjhc fvjkl (contains soy)").is_ok());
        assert!(parse_rule("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)").is_ok());
    }

    #[test]
    fn test_parse_food() {
        let food = r#"
            mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
            trh fvjkl sbzzf mxmxvkd (contains dairy)
            sqjhc fvjkl (contains soy)
            sqjhc mxmxvkd sbzzf (contains fish)
        "#;

        let result = parse_food(food);
        dbg!(&result);
        assert!(result.is_ok());
        assert_eq!(4, result.unwrap().len());
    }
}