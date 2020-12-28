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

fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(str::parse::<u64>)
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
}

fn main() {
    let adapters = parse(include_str!("adapters.txt"));

    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::find_differences;

    #[test]
    fn test_jolt_differences() {
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        assert_eq!(
            vec![1, 3, 1, 1, 1, 3, 1, 1, 3, 1, 3, 3],
            find_differences(&adapters),
        )
    }
}
