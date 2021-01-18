use std::collections::HashMap;

#[derive(Debug)]
enum Rule {
    /// Use single u8 character
    Letter(u8),
    /// List of Rule indices
    List(Vec<u64>),
    /// Tuples separated by | symbol
    Tuples((Vec<u64>, Vec<u64>)),
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
        rule tuples() -> (Vec<u64>, Vec<u64>)
            = l:numbers() " | " r:numbers() { (l, r) }

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
        .filter(|message| {
            match_rule(message.as_bytes(), rules, &rules.get(&0).unwrap())
                .map(|result| result.iter().any(|r| r.is_empty()))
                .unwrap_or(false)
        })
        .count() as u64
}

/// Tries to apply the rule to the given message
fn match_rule<'a>(message: &'a [u8], rules: &HashMap<u64, Rule>, rule: &Rule) -> Option<Vec<&'a [u8]>> {
    match rule {
        Rule::Letter(c) if message.first()? == c => Some(vec![&message[1..]]),
        Rule::Letter(_) => None,
        Rule::List(list) => {
            let mut results = vec![message];
            for entry in list {
                let mut new_results = results
                    .iter()
                    .filter_map(|previous_result| match_rule(previous_result, rules, rules.get(entry).unwrap()))
                    .peekable();
                if new_results.peek().is_some() {
                    results = new_results.flatten().collect();
                } else {
                    return None;
                }
            }

            Some(results)
        }
        Rule::Tuples((lhs, rhs)) => {
            let sequence = vec![Rule::List(lhs.clone()), Rule::List(rhs.clone())];

            let mut results = sequence
                .iter()
                .filter_map(|option| match_rule(message, rules, option))
                .peekable();
            if results.peek().is_some() {
                Some(results.flatten().collect())
            } else {
                None
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let (mut rules, messages) = parse(include_str!("messages.txt"))?;
    let result = validate(&rules, &messages);
    dbg!(result);

    rules.insert(8, Rule::Tuples((vec![42], vec![42, 8])));
    rules.insert(11, Rule::Tuples((vec![42, 31], vec![42, 11, 31])));
    let result = validate(&rules, &messages);
    dbg!(result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Rule, parse, validate};

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

    #[test]
    fn test_validate_part2_messages() {
        let content = r#"
            42: 9 14 | 10 1
            9: 14 27 | 1 26
            10: 23 14 | 28 1
            1: "a"
            11: 42 31
            5: 1 14 | 15 1
            19: 14 1 | 14 14
            12: 24 14 | 19 1
            16: 15 1 | 14 14
            31: 14 17 | 1 13
            6: 14 14 | 1 14
            2: 1 24 | 14 4
            0: 8 11
            13: 14 3 | 1 12
            15: 1 | 14
            17: 14 2 | 1 7
            23: 25 1 | 22 14
            28: 16 1
            4: 1 1
            20: 14 14 | 1 15
            3: 5 14 | 16 1
            27: 1 6 | 14 18
            14: "b"
            21: 14 1 | 1 14
            25: 1 1 | 1 14
            22: 14 14
            8: 42
            26: 14 22 | 1 20
            18: 15 15
            7: 14 5 | 1 21
            24: 14 1

            abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
            bbabbbbaabaabba
            babbbbaabbbbbabbbbbbaabaaabaaa
            aaabbbbbbaaaabaababaabababbabaaabbababababaaa
            bbbbbbbaaaabbbbaaabbabaaa
            bbbababbbbaaaaaaaabbababaaababaabab
            ababaaaaaabaaab
            ababaaaaabbbaba
            baabbaaaabbaaaababbaababb
            abbbbabbbbaaaababbbbbbaaaababb
            aaaaabbaabaaaaababaa
            aaaabbaaaabbaaa
            aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
            babaaabbbaaabaababbaabababaaab
            aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
        "#;

        let (mut rules, messages) = parse(content).unwrap();
        assert_eq!(3, validate(&rules, &messages));

        rules.insert(8, Rule::Tuples((vec![42], vec![42, 8])));
        rules.insert(11, Rule::Tuples((vec![42, 31], vec![42, 11, 31])));
        assert_eq!(12, validate(&rules, &messages));
    }
}
