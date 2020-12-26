use std::cmp::Ordering;

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

    fn binary_search(mut list: Vec<char>, greater: char) -> u64 {
        let max = 2u64.pow(list.len() as u32);
        list.reverse();

        (0..max).collect::<Vec<u64>>().binary_search_by(|_| {
            match list.pop() {
                Some(x) if x == greater => Ordering::Greater,
                Some(_) => Ordering::Less,
                None => Ordering::Equal,
            }
        }).unwrap() as u64
    }

    /// Finds the row between 0..127
    pub fn row(&self) -> u64 {
        Self::binary_search(self.row.clone(), 'F')
    }

    pub fn colum(&self) -> u64 {
        Self::binary_search(self.column.clone(), 'L')
    }

    pub fn id(&self) -> u64 {
        self.row() * 8 + self.colum()
    }
}

fn main() {
    // create list of all boarding passes
    let passes = include_str!("passes.txt")
        .lines()
        .map(BoardingPass::new)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    // find maximum boarding pass id
    let max = passes
        .into_iter()
        .map(|pass| pass.id())
        .max();

    dbg!(max);
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

    #[test]
    fn test_boarding_pass_ids() {
        assert_eq!(567, BoardingPass::new("BFFFBBFRRR").unwrap().id());
        assert_eq!(119, BoardingPass::new("FFFBBBFRRR").unwrap().id());
        assert_eq!(820, BoardingPass::new("BBFFBBFRLL").unwrap().id());
    }
}
