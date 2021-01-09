use std::{fmt::Debug, ops::RangeInclusive};

// Parses the input
struct Rule {
    /// For now store the name of the rule (may be relevant later)
    pub name: String,
    /// First range
    pub first: RangeInclusive<u64>,
    /// Second range
    pub second: RangeInclusive<u64>,
}

impl Rule {
    pub fn new(name: &str, first: RangeInclusive<u64>, second: RangeInclusive<u64>) -> Self {
        Self {
            name: name.into(),
            first,
            second,
        }
    }
}

impl Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let first = format!("{}-{}", self.first.start(), self.first.end());
        let second = format!("{}-{}", self.second.start(), self.second.end());
        write!(f, "{}: {} or {}", self.name, first, second)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::Rule;

    #[test]
    fn test_rule_parser() {
        assert_eq!("rule: 1-3 or 5-7", format!("{:?}", Rule::new("rule", 1..=3, 5..=7)))
    }
}
