use std::{fmt::Debug, ops::Range};

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

impl TicketValidator {
    pub fn parse(content: &str) -> anyhow::Result<Self> {
        Ok(Self::default())
    }
}

fn main() -> anyhow::Result<()> {
    let validator = TicketValidator::parse(include_str!("tickets.txt"))?;
    println!("Hello, world!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Rule, TicketValidator};

    #[test]
    fn test_rule_debug_format() {
        assert_eq!("rule: 1-3 or 5-7", format!("{:?}", Rule::new("rule", 1..4, 5..8)))
    }

    #[test]
    fn test_parse_ticket_validator() {
        let validator = TicketValidator::parse(r#"
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
        "#);

        assert!(validator.is_ok());
    }
}
