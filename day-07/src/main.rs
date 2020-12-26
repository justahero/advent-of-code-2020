#[derive(Debug)]
struct Bag {
    pub color: String,
    pub contents: Vec<String>,
}

impl Bag {
    pub fn new(color: String, contents: Vec<String>) -> Self {
        Self {
            color,
            contents,
        }
    }
}

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

        pub(crate) rule line() -> Bag
            = bag:bag() " contain " contents:contents() "." { Bag::new(bag, contents) };
    }
}

fn parse_rule(line: &str) -> anyhow::Result<Bag> {
    Ok(line_parser::line(line)?)
}

fn count_bag_colors(lines: &[&str], color: &str) -> anyhow::Result<u64> {
    let mut count = 0;

    let rules = lines
        .iter()
        .map(|rule| parse_rule(*rule))
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    for bag in rules {
        if bag.contents.contains(&color.into()) {
            count += 1;
        }
    }

    Ok(count)
}

fn main() {
    let lines = include_str!("luggage.txt")
        .lines()
        .collect::<Vec<_>>();

    let result = count_bag_colors(&lines, "shiny gold").unwrap();

    dbg!(&result);
}

#[cfg(test)]
mod tests {
    use crate::{count_bag_colors, parse_rule};

    #[test]
    fn test_parser_rules() {
        assert!(parse_rule("faded blue bags contain no other bags.").is_ok());
        assert!(parse_rule("bright white bags contain 1 shiny gold bag.").is_ok());
        assert!(parse_rule("light red bags contain 1 bright white bag, 2 muted yellow bags.").is_ok());
    }

    #[test]
    fn test_count_bag_colors() {
        const CONTENT: &str = r#"
        light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.
        "#;

        let lines = CONTENT
            .lines()
            .collect::<Vec<_>>();

        assert_eq!(4, count_bag_colors(&lines, "shiny gold").unwrap());
    }
}
