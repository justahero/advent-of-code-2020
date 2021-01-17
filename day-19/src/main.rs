use std::collections::HashMap;

#[derive(Debug)]
enum Rule {
    /// Use single u8 character
    Letter(u8),
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
        rule letter() -> u8
            = "\"" s:$(['a'..='z' | 'A'..='Z']+) "\"" { s.as_bytes()[0] }

        /// List of number pairs
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

/// Validate the messages by the given set of rules
fn validate(rules: &HashMap<u64, Rule>, messages: &[String]) -> u64 {
    messages
        .iter()
        .filter(|&message| {
            match_rule(message.as_bytes(), rules, 0)
                .map(|n| n == message.len())
                .unwrap_or(false)
        })
        .count() as u64
}

/// Tries to apply the rule to the given message
fn match_rule(message: &[u8], rules: &HashMap<u64, Rule>, rule: u64) -> Option<usize> {
    if message.is_empty() {
        return None;
    }

    match rules.get(&rule).unwrap() {
        Rule::Letter(c) if &message[0] == c => Some(1),
        Rule::Letter(_) => None,
        Rule::List(numbers) => {
            numbers
                .iter()
                .try_fold(0, |count, &r| match_rule(&message[count..], rules, r).map(|n| n + count))
        }
        Rule::Tuples(tuples) => {
            tuples
                .iter()
                .map(|numbers| {
                    numbers
                        .iter()
                        .try_fold(0, |count, &r| match_rule(&message[count..], rules, r).map(|n| n + count))
                })
                .find(|x| x.is_some())
                .unwrap()
        }
    }
}

fn main() -> anyhow::Result<()> {
    let (rules, messages) = parse(include_str!("messages.txt"))?;
    let result = validate(&rules, &messages);
    dbg!(result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{parse, validate};

    const CONTENT: &str = r#"
        0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: "a"
        5: "b"

        ababbb
        bababa
        abbbab
        aaabbb
        aaaabbb
    "#;

    #[test]
    fn test_parse_rules() {
        let result = parse(CONTENT);
        assert!(result.is_ok());

        let (rules, messages) = result.unwrap();
        assert_eq!(6, rules.len());
        assert_eq!(5, messages.len());
    }

    #[test]
    fn test_validate_messages() {
        let (rules, messages) = parse(CONTENT).unwrap();
        assert_eq!(2, validate(&rules, &messages));
    }
}
