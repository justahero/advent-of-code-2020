
peg::parser!{
    grammar line_parser() for str {
        rule number() -> u64
            = s:$(['0'..='9']+) { s.parse().unwrap() }

        rule separator()
            = ", "

        pub rule bag() -> String
            = adj:$(['a'..='z']+) " " color:$(['a'..='z']+) " bag" $(['s']?) { format!("{} {}", adj, color) }

        rule empty() -> Vec<String>
            = "no other bags" { vec![] }

        rule bags() -> String
            = number() " " s:bag() { s }

        pub rule contents() -> Vec<String>
            = empty() / (b:bags() separator()* { b })*

        pub(crate) rule line() -> (String, Vec<String>)
            = bag:bag() " contain " contents:contents() "." { (bag, contents) };
    }
}

/// Parses the given rule, splits its components
/// Instead of using a grammar parser
fn parse_rule(line: &str) -> anyhow::Result<()> {
    let result = line_parser::line(line)?;
    dbg!(result);
    Ok(())
}

fn main() {
    // let _x = line_parser::line("faded blue bags contain no other bags.");
    // let _ = line_parser::contents("1 shiny gold bag");
    // let _ = line_parser::contents("2 muted yellow bags");
    let result = parse_rule("light red bags contain 1 bright white bag, 2 muted yellow bags.");
    dbg!(result);
}

#[cfg(test)]
mod tests {
    use crate::parse_rule;

    #[test]
    fn test_parser_rules() {
        assert!(parse_rule("faded blue bags contain no other bags.").is_ok());
        assert!(parse_rule("bright white bags contain 1 shiny gold bag.").is_ok());
        assert!(parse_rule("light red bags contain 1 bright white bag, 2 muted yellow bags.").is_ok());
    }
}
