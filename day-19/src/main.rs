use std::collections::HashMap;

#[derive(Debug)]
enum Rule {
    /// Use String for convenience for now
    Letter(String),
    /// List of Rule indices
    List(Vec<u64>),
    /// Tuples separated by | symbol
    Tuples(Vec<Vec<u64>>),
}

peg::parser!{
    /// Parses a rule
    grammar rule_parser() for str {
        /// A single or multiple digits number
        rule number() -> u64
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule numbers() -> Vec<u64>
            = _ n:number() ** _ { n }

        /// A single letter enclosed by double quotes
        rule letter() -> String
            = "\"" s:$(['a'..='z' | 'A'..='Z']+) "\"" { s.into() }

        /// Refactor to (Vec<u64>, Vec<u64>)
        rule tuples() -> Vec<Vec<u64>>
            = l:numbers() " | " r:numbers() { vec![l, r] }

        /// White spaces
        rule _() = [' ']?

        /// Can be a single number
        /// Can be a list / pair of numbers
        /// Can be two pairs 
        rule list() -> Rule
            // consecutive numbers
            = tuples:tuples() { Rule::Tuples(tuples) }
            // single letter
            / l:letter() { Rule::Letter(l) }
            // pairs of numbers separated by | symbol
            / numbers:numbers() { Rule::List(numbers) }

        pub(crate) rule parse() -> (u64, Rule)
            = index:number() ":" _ r:list() { (index, r) }
    }
}

/// Parses all rules and messages
/// For now return all rules and the messages as tuple
fn parse(content: &str) -> anyhow::Result<(HashMap<u64, Rule>, Vec<String>)> {
    let mut rules = HashMap::new();
    let mut messages = Vec::new();

    content
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .for_each(|line| {
            if let Ok((index, rule)) = rule_parser::parse(line) {
                rules.insert(index, rule);
            } else {
                messages.push(line.into());
            }
        });

    Ok((rules, messages))
}

fn main() -> anyhow::Result<()> {
    let (rules, messages) = parse(include_str!("messages.txt"))?;
    dbg!(rules, messages);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn test_parse_rules() {
        let content = r#"
            0: 4 1 5
            1: 2 3 | 3 2
            2: 4 4 | 5 5
            3: 4 5 | 5 4
            4: "a"
            5: "b"
        "#;

        assert!(parse(content).is_ok());
        let (rules, messages) = parse(content).unwrap();

        assert_eq!(0, messages.len());
        assert_eq!(6, rules.len());
    }
}
