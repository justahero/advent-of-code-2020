use itertools::Itertools;
use std::{fmt::Debug, ops::Range};

#[derive(Debug, Default, PartialEq, Eq)]
struct Ticket {
    pub numbers: Vec<u64>,
}

impl From<&Vec<u64>> for Ticket {
    fn from(v: &Vec<u64>) -> Self {
        Self { numbers: v.clone() }
    }
}

impl Ticket {
    pub fn new(numbers: &[u64]) -> Self {
        Self {
            numbers: numbers.to_vec(),
        }
    }

    pub fn sum(&self) -> u64 {
        self.numbers.iter().sum()
    }
}

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

// Parses the input
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

    pub fn is_ticket_valid(&self, ticket: &Ticket) -> bool {
        ticket
            .numbers
            .iter()
            .all(|value| self.is_valid(value))
    }

    /// Returns true if the given value is in first or second range
    pub fn is_valid(&self, value: &u64) -> bool {
        self.first.contains(value) || self.second.contains(value)
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
    pub my_ticket: Ticket,
    /// The list of all nearby tickets
    pub nearby_tickets: Vec<Ticket>,
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
            .map(|ticket| ticket.sum())
            .sum()
    }

    /// Detect all tickets are valid and detect its fields from
    pub fn find_valid_tickets(&self) {
        // detect valid tickets
        let valid_tickets = self.nearby_tickets
            .iter()
            .fold(Vec::new(), |mut result, ticket| {
                let valid = ticket
                    .numbers
                    .iter()
                    .all(|&value| self.is_valid(value));

                if valid {
                    result.push(ticket);
                }
                result
            });
    }

    /// Find all invalid numbers in the tickets
    pub fn find_invalid_numbers(&self) -> Vec<Ticket> {
        self.nearby_tickets
            .iter()
            .fold(Vec::new(), |mut result, ticket| {
                // check all numbers of each ticket
                let invalid_numbers = ticket
                    .numbers
                    .iter()
                    .filter(|&&value| !self.is_valid(value))
                    .copied()
                    .collect::<Vec<u64>>();

                if !invalid_numbers.is_empty() {
                    result.push(Ticket::new(&invalid_numbers));
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
        Ok(Ticket { numbers })
    }

    /// Returns true if the ticket is valid for all rules, otherwise false
    fn is_ticket_valid(&self, ticket: &Ticket) -> bool {
        self.rules
            .iter()
            .all(|rule| rule.is_ticket_valid(ticket))
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
    dbg!(&validator);

    let numbers = validator.find_invalid_numbers();
    dbg!(&numbers);
    let result = validator.find_invalid_sum();
    dbg!(&result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Rule, Ticket, TicketValidator};

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
        assert_eq!(vec![Ticket::new(&[4]), Ticket::new(&[55]), Ticket::new(&[12])], numbers);
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
        let fields = validator.detect_fields();
    }
}
