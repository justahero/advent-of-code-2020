
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
        pub(crate) rule expression() -> u64
            = precedence! {
                x:(@) separator() "+" separator() y:@ { x + y }
                x:(@) separator() "*" separator() y:@ { x * y }
                "(" e:expression() ")" { e }
                n:number() { n }
            }
    }
}

/// Parse line, evaluate equation and return result
fn parse_equation(line: &str) -> anyhow::Result<u64> {
    Ok(parser::expression(line)?)
}

/// Parses the string content as a list of equations
fn parse(content: &str) -> anyhow::Result<Vec<u64>> {
    let equations = content
        .lines()
        .map(|line| line.trim())
        .filter(|&line| !line.is_empty())
        .map(|line| parse_equation(line))
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    Ok(equations)
}

fn main() -> anyhow::Result<()> {
    let result = parse(include_str!("equations.txt"))?
        .iter()
        .sum::<u64>();

    dbg!(result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{parse, parse_equation};

    #[test]
    fn test_parse_equations() {
        assert!(parse("1 + 2 * 3 + 4 * 5 + 6").is_ok());
        assert!(parse("2 * 3 + (4 * 5)").is_ok());
        assert!(parse("5 + (8 * 3 + 9 + 3 * 4 * 3)").is_ok());
    }

    #[test]
    fn test_solve_equations() {
        assert_eq!(71, parse_equation("1 + 2 * 3 + 4 * 5 + 6").unwrap());
        assert_eq!(51, parse_equation("1 + (2 * 3) + (4 * (5 + 6))").unwrap());
        assert_eq!(26, parse_equation("2 * 3 + (4 * 5)").unwrap());
        assert_eq!(437, parse_equation("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap());
        assert_eq!(12240, parse_equation("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap());
        assert_eq!(13632, parse_equation("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap());
    }
}
