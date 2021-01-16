

struct Equation {
}

impl Equation {
    pub fn solve(&self) -> u64 {
        0
    }
}

/// Parses a line to an equation
fn parse_equation(line: &str) -> anyhow::Result<Equation> {
    Ok(Equation{})
}

/// Parses the string content as a list of equations
fn parse(content: &str) -> anyhow::Result<Vec<Equation>> {
    let equations = content
        .lines()
        .map(|line| line.trim())
        .filter(|&line| !line.is_empty())
        .map(|line| parse_equation(line))
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    Ok(equations)
}

fn main() {
    let equations = parse(include_str!("equations.txt"));
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
        assert_eq!(71, parse_equation("1 + 2 * 3 + 4 * 5 + 6").unwrap().solve());
        assert_eq!(51, parse_equation("1 + (2 * 3) + (4 * (5 + 6))").unwrap().solve());
        assert_eq!(26, parse_equation("2 * 3 + (4 * 5)").unwrap().solve());
        assert_eq!(437, parse_equation("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap().solve());
        assert_eq!(12240, parse_equation("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap().solve());
        assert_eq!(13632, parse_equation("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap().solve());
    }
}
