
/// Parses all rules and messages
/// For now return all rules and the messages as tuple
fn parse(content: &str) -> anyhow::Result<(Vec<String>, Vec<String>)> {
    let rules = Vec::new();
    let messages = Vec::new();

    content
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .for_each(|_line| {
            // TODO
        });

    Ok((rules, messages))
}

fn main() -> anyhow::Result<()> {
    let (rules, messages) = parse(include_str!("messages.txt"))?;
    dbg!(rules, messages);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn test_parse_rules() {
        let content = r#"
            0: 4 1 5
            1: 2 3 | 3 2
            2: 4 4 | 5 5
            3: 4 5 | 5 4
            4: "a"
            5: "b"
        "#;

        assert!(parse(content).is_ok());
        let (rules, messages) = parse(content).unwrap();

        assert_eq!(6, rules.len());
        assert_eq!(0, messages.len());
    }
}
