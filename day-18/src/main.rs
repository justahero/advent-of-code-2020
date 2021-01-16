
peg::parser!{
    /// Equation parser, parses the input and calculates the value of the equation
    ///
    /// # Example
    /// ```
    /// 3 + 6 + (3 * 9)
    /// ```
    ///
    grammar parser() for str {
        rule number() -> u64
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule separator() = [' ']?

        /// Left associated parser, multiplication & addition have same precedence
        /// For more details see: https://docs.rs/peg/0.6.3/peg/#precedence-climbing
        pub(crate) rule part1() -> u64
            = precedence! {
                x:(@) separator() "+" separator() y:@ { x + y }
                x:(@) separator() "*" separator() y:@ { x * y }
                "(" e:part1() ")" { e }
                n:number() { n }
            }

        /// Addition has precedence over multiplication
        pub(crate) rule part2() -> u64
            = precedence! {
                x:(@) separator() "*" separator() y:@ { x * y }
                --
                x:(@) separator() "+" separator() y:@ { x + y }
                --
                "(" e:part2() ")" { e }
                n:number() { n }
            }
    }
}

/// Parses the string content as a list of equations
fn parse(content: &str) -> anyhow::Result<Vec<String>> {
    let lines = content
        .lines()
        .map(|line| line.trim())
        .filter(|&line| !line.is_empty())
        .map(|line| line.into())
        .collect::<Vec<String>>();
    Ok(lines)
}

fn main() -> anyhow::Result<()> {
    let result = parse(include_str!("equations.txt"))?
        .iter()
        .map(|line| parser::part1(line))
        .filter_map(Result::ok)
        .sum::<u64>();

    dbg!(result);

    let result = parse(include_str!("equations.txt"))?
        .iter()
        .map(|line| parser::part2(line))
        .filter_map(Result::ok)
        .sum::<u64>();

    dbg!(result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse_equations() {
        assert!(parse("1 + 2 * 3 + 4 * 5 + 6").is_ok());
        assert!(parse("2 * 3 + (4 * 5)").is_ok());
        assert!(parse("5 + (8 * 3 + 9 + 3 * 4 * 3)").is_ok());
    }

    #[test]
    fn test_solve_equations() {
        assert_eq!(71, parser::part1("1 + 2 * 3 + 4 * 5 + 6").unwrap());
        assert_eq!(51, parser::part1("1 + (2 * 3) + (4 * (5 + 6))").unwrap());
        assert_eq!(26, parser::part1("2 * 3 + (4 * 5)").unwrap());
        assert_eq!(437, parser::part1("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap());
        assert_eq!(12240, parser::part1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap());
        assert_eq!(13632, parser::part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap());
    }

    #[test]
    fn test_solve_equations_2() {
        assert_eq!(231, parser::part2("1 + 2 * 3 + 4 * 5 + 6").unwrap());
        assert_eq!(51, parser::part2("1 + (2 * 3) + (4 * (5 + 6))").unwrap());
        assert_eq!(46, parser::part2("2 * 3 + (4 * 5)").unwrap());
        assert_eq!(1445, parser::part2("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap());
        assert_eq!(669060, parser::part2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap());
        assert_eq!(23340, parser::part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap());
    }
}
