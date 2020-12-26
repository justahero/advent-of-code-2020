use itertools::{Itertools, MinMaxResult};
use std::{cmp::Ordering, collections::HashSet, iter::FromIterator};

use anyhow::Result;
use regex::Regex;

struct BoardingPlan {
    pub seats: HashSet<(u64, u64)>,
}

impl BoardingPlan {
    pub fn new(seats: Vec<(u64, u64)>) -> Self {
        Self {
            seats: HashSet::from_iter(seats),
        }
    }

    /// This generates a completely filled seat plan from all given rows & columns to find all empty seats
    ///
    /// * it first finds min / max rows and columns
    /// * generate a seat plan with these values
    /// * take difference of filled seat plan with existing plan
    /// * return the diff
    pub fn empty_seats(&self) -> Result<Vec<(u64, u64)>> {
        if let MinMaxResult::MinMax(min, max) = self.seats.iter().minmax() {
            let plan = ((min.0)..=(max.0)).cartesian_product(min.1..=max.1).collect::<HashSet<_>>();
            Ok(plan.difference(&self.seats).cloned().collect())
        } else {
            Err(anyhow::anyhow!("Failed to find min/max values"))
        }
    }
}

struct BoardingPass {
    pub row: Vec<char>,
    pub column: Vec<char>,
}

fn id(row: u64, col: u64) -> u64 {
    row * 8 + col
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
        id(self.row(), self.colum())
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

    let plan = BoardingPlan::new(filled_seats);
    let empty_seats = plan.empty_seats().unwrap();

    assert_eq!(1, empty_seats.len());
    let (row, col) = empty_seats[0];

    dbg!(id(row, col));
    dbg!(max);
    dbg!(empty_seats);
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
        // empty seat is at (1, 1)
        let seats = vec![
            (0, 0), (0, 1), (0, 2),
            (1, 0),         (1, 2),
            (2, 0), (2, 1), (2, 2),
        ];
        let plan = BoardingPlan::new(seats);
        assert_eq!(vec![(1, 1)], plan.empty_seats().unwrap());
    }
}
