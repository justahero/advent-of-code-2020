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

        rule numbers() -> String
            = _ n:number() ** _ { "".into() }

        /// A single letter enclosed by double quotes
        rule letter() -> String
            = "\"" s:$(['a'..='z' | 'A'..='Z']+) "\"" { s.into() }

        /// Refactor to (Vec<u64>, Vec<u64>)
        rule tuples() -> String
            = l:numbers() " | " r:numbers() { "".into() }

        /// White spaces
        rule _() = [' ']?

        /// Can be a single number
        /// Can be a list / pair of numbers
        /// Can be two pairs 
        rule list() -> String
            // consecutive numbers
            = tuples:tuples()
            // single letter
            / letter()
            // pairs of numbers separated by | symbol
            / numbers:numbers()

        pub(crate) rule parse() -> (u64, String)
            = index:number() ":" _ l:list() { (index, l) }
    }
}

/// Parses all rules and messages
/// For now return all rules and the messages as tuple
fn parse(content: &str) -> anyhow::Result<(Vec<String>, Vec<String>)> {
    let mut rules = Vec::new();
    let mut messages = Vec::new();

    content
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .for_each(|line| {
            let result = rule_parser::parse(line);
            println!("Line: {} - result: {:?}", line, result);
            match result {
                Ok((_index, rule)) => rules.push(rule),
                Err(_) => messages.push(line.into()),
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
