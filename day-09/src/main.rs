use itertools::Itertools;

fn parse(lines: &str) -> Vec<u64> {
    lines
        .lines()
        .map(str::trim)
        .map(str::parse::<u64>)
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
}

fn find_sums(preamble: &[u64], sum: u64) -> Vec<(u64, u64)> {
    preamble
        .iter()
        .tuple_combinations()
        .filter(|(left, right)| left != right)
        .filter(|(left, right)| *left + *right == sum)
        .map(|(x, y)| (*x, *y))
        .collect::<Vec<_>>()
}

fn find_first_number(preamble: u64, mut numbers: Vec<u64>) -> Option<u64> {
    let mut queue = numbers
        .drain(0..preamble as usize)
        .collect::<Vec<_>>();

    for number in &numbers {
        let result = find_sums(&queue, *number);
        if result.is_empty() { return Some(*number); }

        queue.remove(0);
        queue.push(*number);
    }

    None
}

fn main() {
    let numbers = parse(include_str!("numbers.txt"));
    dbg!(find_first_number(25, numbers));
}

#[cfg(test)]
mod tests {
    use crate::{find_first_number, find_sums};

    #[test]
    fn test_find_sums() {
        assert_eq!(vec![(2, 5), (3, 4)], find_sums(&[1, 2, 3, 4, 5], 7));
    }

    #[test]
    fn test_find_first_number() {
        let numbers: Vec<u64> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576
        ];
        assert_eq!(Some(127), find_first_number(5, numbers));
    }
}
