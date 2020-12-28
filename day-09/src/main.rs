use itertools::Itertools;

fn parse(lines: &str) -> Vec<u64> {
    lines
        .lines()
        .map(str::trim)
        .map(str::parse::<u64>)
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
}

fn find_sum(preamble: &[u64], sum: u64) -> Vec<(u64, u64)> {
    Vec::new()
}

fn find_first_number(preamble: u64, numbers: &[u64]) -> Option<u64> {
    None
}

fn main() {
    let numbers = parse(include_str!("numbers.txt"));
    let result = find_first_number(25, &numbers);
    dbg!(&result);
}

#[cfg(test)]
mod tests {
    use crate::{find_sum, parse};

    #[test]
    fn test_find_sum() {
        assert_eq!(2, find_sum(&[1, 2, 3, 4, 5], 7).len());
    }

    #[test]
    fn test_find_first_number() {

    }
}
