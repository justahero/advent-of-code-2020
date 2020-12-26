use std::{fmt::Debug, cmp::Ordering};

use anyhow::Result;
use regex::Regex;

struct BoardingPlan {
    pub seats: Vec<(u64, u64)>,
}

impl BoardingPlan {
    pub fn new(mut seats: Vec<(u64, u64)>) -> Self {
        seats.sort();
        Self { seats }
    }

    pub fn empty_seat(&self) -> (u64, u64) {
        (0, 0)
    }
}

impl Debug for BoardingPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();
        let mut row = 0;
        let mut col = 0;

        for seat in &self.seats {
            if seat.0 > row {
                text.push('\n');
                row = seat.0;
                col = 0;
            }

            if seat.1 == col {
                text.push('x');
            } else {
                text.push('.');
            }
            col += 1;
        }

        write!(f, "{}", text)
    }
}

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
        .iter()
        .map(|pass| pass.id())
        .max();

    // get all filled seats
    let filled_seats = passes
        .iter()
        .map(|pass| (pass.row(), pass.colum()))
        .collect::<Vec<_>>();

    dbg!(max);
    dbg!(filled_seats);
}

#[cfg(test)]
mod tests {
    use crate::{BoardingPass, BoardingPlan};

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

    #[test]
    fn test_find_empty_boarding_seat() {
        let seats = vec![
            (0, 0), (0, 1), (0, 2),
            (1, 0), (1, 2),
            (2, 0), (2, 1), (2, 2),
        ];
        let plan = BoardingPlan::new(seats);
        assert_eq!((1, 1), plan.empty_seat());
    }
}
