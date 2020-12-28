use itertools::Itertools;

fn find_differences(adapters: &[u64]) -> Vec<u64> {
    let highest = adapters.iter().max().unwrap();
    let sorted = adapters.iter().sorted().cloned().collect_vec();
    let adapters = itertools::concat(vec![vec![0u64], sorted, vec![highest + 3]]);

    let mut result = vec![];
    for (index, &number) in adapters.iter().skip(1).enumerate() {
        result.push(number - adapters[index]);
    };
    
    result
}

fn find_distribution(adapters: &[u64]) -> (u64, u64) {
    let differences = find_differences(adapters);
    let joltage_1 = differences.iter().filter(|&x| *x == 1).count() as u64;
    let joltage_3 = differences.iter().filter(|&x| *x == 3).count() as u64;
    (joltage_1, joltage_3)
}

fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(str::parse::<u64>)
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
}

fn main() {
    let adapters = parse(include_str!("adapters.txt"));
    let (left, right) = find_distribution(&adapters);
    dbg!(left * right);
}

#[cfg(test)]
mod tests {
    use crate::{find_differences, find_distribution};

    #[test]
    fn test_jolt_differences() {
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        assert_eq!(
            vec![1, 3, 1, 1, 1, 3, 1, 1, 3, 1, 3, 3],
            find_differences(&adapters),
        );
        assert_eq!(
            (7, 5),
            find_distribution(&adapters),
        )
    }

    #[test]
    fn test_jolt_distribution() {
        let adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        assert_eq!(
            (22, 10),
            find_distribution(&adapters),
        )
    }
}
