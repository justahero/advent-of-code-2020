use anyhow::Result;
use regex::Regex;

struct BoardingPass {
    pub row: Vec<char>,
    pub column: Vec<char>,
}

impl BoardingPass {
    pub fn new(pass: &str) -> Result<Self> {
        let pattern = Regex::new(r"^(?P<row>[BF]{7})(?P<column>[LR]{3})$").unwrap();

        let captures = pattern.captures(pass)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse pass {}", pass))?;

        let row = String::from(&captures["row"]).chars().collect();
        let column = String::from(&captures["column"]).chars().collect();

        Ok(Self {
            row,
            column,
        })
    }

    /// Finds the row between 0..127
    pub fn row(&self) -> u64 {
        let mut min = 0;
        let mut max = 127;
        let mut step = 64;
        for row in &self.row {
            if row == &'F' {
                max -= step;
            } else {
                min += step;
            }

            step /= 2;
        }
        min
    }

    pub fn colum(&self) -> u64 {
        let mut min = 0;
        let mut max = 7;
        let mut step = 4;
        for column in &self.column {
            if column == &'L' {
                max -= step;
            } else {
                min += step;
            }

            step /= 2;
        }
        min
    }

    pub fn id(&self) -> u64 {
        self.row() * 8 + self.colum()
    }
}

fn main() {
    let passes = include_str!("passes.txt")
        .lines()
        .map(BoardingPass::new)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
}

#[cfg(test)]
mod tests {
    use crate::BoardingPass;

    #[test]
    fn test_new_boarding_pass() {
        assert!(BoardingPass::new("BFBFFFFLRR").is_ok());
        assert!(BoardingPass::new("AFBFFFFLRR").is_err());
        assert!(BoardingPass::new("BFBFFFFLRRL").is_err());
    }

    #[test]
    fn test_boarding_pass() {
        let pass = BoardingPass::new("FBFBBFFRLR").unwrap();
        assert_eq!(44, pass.row());
        assert_eq!(5, pass.colum());
        assert_eq!(357, pass.id());
    }
}
