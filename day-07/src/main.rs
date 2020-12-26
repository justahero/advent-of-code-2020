use anyhow::anyhow;

struct BagRule {
    pub name: String,
    pub contents: Vec<BagRule>,
}

/// Parses the given rule, splits its components
fn parse_rule(line: &str) -> anyhow::Result<()> {
    Ok(())
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::parse_rule;

    #[test]
    fn test_parse_rules() {
        assert!(parse_rule("faded blue bags contain no other bags.").is_ok());
        assert!(parse_rule("light red bags contain 1 bright white bag, 2 muted yellow bags.").is_ok());
    }
}
