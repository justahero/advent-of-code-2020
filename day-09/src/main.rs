use FoldWhile::{Continue, Done};
use itertools::{FoldWhile, Itertools};

fn parse(lines: &str) -> Vec<u64> {
    lines
        .lines()
        .map(str::trim)
        .map(str::parse::<u64>)
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
}

fn find_sums(sum: u64, preamble: &[u64]) -> Vec<(u64, u64)> {
    preamble
        .iter()
        .tuple_combinations()
        .filter(|(left, right)| left != right)
        .filter(|(left, right)| *left + *right == sum)
        .map(|(x, y)| (*x, *y))
        .collect::<Vec<_>>()
}

fn find_first_number(preamble: usize, numbers: &[u64]) -> Option<u64> {
    numbers
        .iter()
        .skip(preamble)
        .enumerate()
        .find(|(index, &sum)| {
            find_sums(sum, &numbers[*index..index + preamble]).is_empty()
        })
        .map(|s| *s.1)
}

fn find_contiguous_numbers(sum: u64, numbers: &[u64]) -> Option<Vec<u64>> {
    numbers
        .iter()
        .batching(|iter| {
            let value = iter.clone().fold_while(vec![], |mut acc, &number| {
                if acc.iter().sum::<u64>() < sum {
                    acc.push(number);
                    Continue(acc)
                } else {
                    Done(acc)
                }
            }).into_inner();
            iter.next();
            Some(value)
        })
        .find(|list| list.iter().sum::<u64>() == sum)
}

fn main() {
    let numbers = parse(include_str!("numbers.txt"));

    let number = find_first_number(25, &numbers);
    dbg!(&number);

    let list = find_contiguous_numbers(number.unwrap(), &numbers);
    dbg!(&list);
}

#[cfg(test)]
mod tests {
    use crate::{find_contiguous_numbers, find_first_number, find_sums};

    #[test]
    fn test_find_sums_of_pairs() {
        assert_eq!(vec![(2, 5), (3, 4)], find_sums(7, &[1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_find_first_number() {
        let numbers: Vec<u64> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576
        ];
        assert_eq!(Some(127), find_first_number(5, &numbers));
    }

    #[test]
    fn test_find_contiguous_numbers() {
        let numbers: Vec<u64> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576
        ];

        assert_eq!(
            Some(vec![15, 25, 47, 40]),
            find_contiguous_numbers(127, &numbers),
        );
    }
}
