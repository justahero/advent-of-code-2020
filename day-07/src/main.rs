
peg::parser!{
    grammar parser() for str {
        rule bag() -> String
            = adj:$(['a'..='z']+) " " color:$(['a'..='z']+) " bags" { format!("{} {}", adj, color) }

        rule empty() -> Vec<String>
            = "no other bags" { vec![] }

        rule bags() -> Vec<String>
            = "" { vec![] }

        rule contents() -> Vec<String>
            = empty() / bags()

        pub(crate) rule string() -> (String, Vec<String>)
            = l:bag() " contain " c:contents() "." { (l, vec![]) };
    }
}

/// Parses the given rule, splits its components
/// Instead of using a grammar parser
fn parse_rule(line: &str) -> anyhow::Result<()> {
    let (bag, contents) = parser::string(line)?;
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