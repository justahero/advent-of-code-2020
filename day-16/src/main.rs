use itertools::Itertools;
use std::{fmt::Debug, collections::HashMap, ops::Range};

type Ticket = Vec<u64>;

peg::parser!{
    grammar line_parser() for str {
        rule number() -> u64
            = n:$(['0'..='9']+) { n.parse::<u64>().unwrap() }

        rule range() -> Range<u64>
            = start:number() "-" end:number() { start..end + 1 }

        rule name() -> String
            = s:$(['a'..='z' | 'A'..='Z' | ' ']+) { s.into() }

        pub(crate) rule line() -> Rule
            = name:name() ": " first:range() " or " second:range() { Rule::new(&name, first, second) }
    }
}


#[derive(Clone, PartialEq, Eq)]
struct Rule {
    /// For now store the name of the rule (may be relevant later)
    pub name: String,
    /// First range
    pub first: Range<u64>,
    /// Second range
    pub second: Range<u64>,
}

impl Rule {
    pub fn new(name: &str, first: Range<u64>, second: Range<u64>) -> Self {
        Self {
            name: name.into(),
            first,
            second,
        }
    }

    /// Returns true if the given value is in first or second range
    pub fn is_valid(&self, value: &u64) -> bool {
        self.first.contains(value) || self.second.contains(value)
    }

    /// Returns true if all the given numbers are valid for this rule
    pub fn valid_numbers(&self, numbers: &[u64]) -> bool {
        numbers.iter().all(|number| self.is_valid(number))
    }
}

impl Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let first = format!("{}-{}", self.first.start, self.first.end);
        let second = format!("{}-{}", self.second.start, self.second.end);
        write!(f, "{}: {} or {}", self.name, first, second)
    }
}

#[derive(Debug, Default)]
struct TicketValidator {
    /// The list of rules
    pub rules: Vec<Rule>,
    /// My personal ticket
    pub my_ticket: Vec<u64>,
    /// The list of all nearby tickets
    pub nearby_tickets: Vec<Vec<u64>>,
}

enum ReadState {
    Rule,
    YourTicket,
    NearbyTickets,
}

impl Default for ReadState {
    fn default() -> Self {
        Self::Rule
    }
}

impl TicketValidator {
    pub fn parse(content: &str) -> anyhow::Result<Self> {
        let lines = content
            .lines()
            .map(|line| line.trim())
            .filter(|&line| !line.is_empty())
            .collect::<Vec<_>>();

        let mut validator = Self::default();

        // first a list of rules are given until the line "your ticket:" appears
        let mut state = ReadState::default();
        for line in lines {
            if line.starts_with("your ticket:") {
                state = ReadState::YourTicket;
                continue;
            } else if line.starts_with("nearby tickets:") {
                state = ReadState::NearbyTickets;
                continue;
            }

            match state {
                ReadState::Rule => validator.rules.push(line_parser::line(line)?),
                ReadState::YourTicket => validator.my_ticket = Self::parse_ticket(line)?,
                ReadState::NearbyTickets => validator.nearby_tickets.push(Self::parse_ticket(line)?),
            }
        }

        Ok(validator)
    }

    /// Returns the sum of all invalid numbers from nearby tickets
    pub fn find_invalid_sum(&self) -> u64 {
        let tickets = self.find_invalid_numbers();
        tickets
            .iter()
            .map(|ticket| ticket.iter().sum::<u64>())
            .sum()
    }

    /// Maps all numbers from rows to columns
    pub fn flip_rows_to_cols(numbers: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
        if numbers.is_empty() {
            return Vec::new();
        }

        let mut result = vec![Vec::with_capacity(numbers.len()); numbers[0].len()];
        for row in numbers {
            for i in 0..row.len() {
                result[i].push(row[i]);
            }
        }

        result
    }

    /// Detects all valid tickets, reverse map numbers to rules
    pub fn map_valid_rules(&self) -> HashMap<usize, Rule> {
        // get list of all valid tickets
        let valid_tickets = self.find_valid_tickets();

        // flip numbers from rows to columns, map index by group of numbers
        let mut mapped_numbers = Self::flip_rows_to_cols(&valid_tickets)
            .iter()
            .enumerate()
            .map(|(index, numbers)| (index, numbers.clone()))
            .collect::<HashMap<usize, Vec<u64>>>();

        // check all rules, pick the set of numbers for which only one rule applies, then remove set
        // this should eliminate all possible multiple candidate sets until only one rule applies
        let mut result = HashMap::new();
        let mut rules = Vec::new();

        // find the best candidate for every rule
        // first find the only matching candidate, then mark it as seen
        // remove the candidates from the mapped numbers list
        loop {
            for index in 0..self.rules.len() {
                let rule = self.rules.get(index).unwrap();

                let candidates = mapped_numbers
                    .iter()
                    .filter(|(_i, numbers)| rule.valid_numbers(numbers))
                    .collect::<Vec<_>>();
    
                // there may be multiple candidates, only consider the single matching one
                if candidates.len() == 1 {
                    let i = *candidates.first().unwrap().0;
                    mapped_numbers.remove(&i);
                    rules.push(i);
                    result.insert(i, (*rule).clone());
                }
            }

            if mapped_numbers.is_empty() {
                break;
            }
        }

        result
    }

    /// Detect all valid tickets
    pub fn find_valid_tickets(&self) -> Vec<Ticket> {
        self.nearby_tickets
            .iter()
            .filter(|numbers| numbers.iter().all(|&number| self.is_valid(number)))
            .cloned()
            .collect::<Vec<Ticket>>()
    }

    /// Find all invalid numbers in the tickets
    pub fn find_invalid_numbers(&self) -> Vec<Vec<u64>> {
        self.nearby_tickets
            .iter()
            .fold(Vec::new(), |mut result, ticket| {
                // check all numbers of each ticket
                let invalid_numbers = ticket
                    .iter()
                    .filter(|&&value| !self.is_valid(value))
                    .copied()
                    .collect::<Vec<u64>>();

                if !invalid_numbers.is_empty() {
                    result.push(invalid_numbers);
                }
                result
            })
    }

    fn parse_ticket(line: &str) -> anyhow::Result<Ticket> {
        let numbers = line
            .split(',')
            .map(|number| number.parse::<u64>())
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        Ok(numbers)
    }

    /// Returns true if the given value is valid in any of the rules
    fn is_valid(&self, value: u64) -> bool {
        self.rules
            .iter()
            .any(|rule| rule.is_valid(&value))
    }
}

fn main() -> anyhow::Result<()> {
    let validator = TicketValidator::parse(include_str!("tickets.txt"))?;

    // let numbers = validator.find_invalid_numbers();
    let result = validator.find_invalid_sum();
    dbg!(&result);

    let mapped_rules = validator.map_valid_rules();
    dbg!(&mapped_rules);

    // find all rules with prefix "departure"
    let product = mapped_rules
        .iter()
        .filter(|(_, rule)| rule.name.starts_with("departure"))
        .map(|(&index, _)| validator.my_ticket[index as usize])
        .product::<u64>();
    dbg!(product);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{Rule, TicketValidator};

    const CONTENT: &str = r#"
        class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50

        your ticket:
        7,1,14

        nearby tickets:
        7,3,47
        40,4,50
        55,2,20
        38,6,12
    "#;

    #[test]
    fn test_rule_debug_format() {
        assert_eq!("rule: 1-3 or 5-7", format!("{:?}", Rule::new("rule", 1..3, 5..7)))
    }

    #[test]
    fn test_parse_ticket_validator() {
        let validator = TicketValidator::parse(CONTENT);
        assert!(validator.is_ok());

        let validator = validator.unwrap();
        assert_eq!(3, validator.rules.len());
        assert_eq!(4, validator.nearby_tickets.len());
    }

    #[test]
    fn test_find_invalid_numbers() {
        let validator = TicketValidator::parse(CONTENT).unwrap();

        let numbers = validator.find_invalid_numbers();
        assert_eq!(3, numbers.len());
        assert_eq!(vec![vec![4], vec![55], vec![12]], numbers);
    }

    #[test]
    fn test_flip_numbers_from_rows_to_cols() {
        let numbers = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let expected = vec![
            vec![1, 4, 7],
            vec![2, 5, 8],
            vec![3, 6, 9],
        ];

        assert_eq!(expected, TicketValidator::flip_rows_to_cols(&numbers));
    }

    #[test]
    fn test_find_valid_tickets() {
        let content = r#"
            class: 0-1 or 4-19
            row: 0-5 or 8-19
            seat: 0-13 or 16-19

            your ticket:
            11,12,13

            nearby tickets:
            3,9,18
            15,1,5
            5,14,9
        "#;

        let validator = TicketValidator::parse(content).unwrap();
        assert_eq!(3, validator.find_valid_tickets().len());
    }

    #[test]
    fn test_determine_valid_ticket_fields() {
        let content = r#"
            class: 0-1 or 4-19
            row: 0-5 or 8-19
            seat: 0-13 or 16-19

            your ticket:
            11,12,13

            nearby tickets:
            3,9,18
            15,1,5
            5,14,9
        "#;

        let validator = TicketValidator::parse(content).unwrap();
        assert_eq!(3, validator.find_valid_tickets().len());

        let rules = validator.map_valid_rules();
        // dbg!(&rules);

        let expected_rules: HashMap<usize, Rule> = vec![
            (0, Rule::new("row", 0..6, 8..20)),
            (1, Rule::new("class", 0..2, 4..20)),
            (2, Rule::new("seat", 0..14, 16..20)),
        ].into_iter().collect();

        assert_eq!(expected_rules, rules);
    }
}
