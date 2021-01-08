/// Finds the sequence of numbers that are spoken.
fn find_sequence(starter: &[u64], turns: u64) -> Vec<u64> {
    let mut sequence: Vec<u64> = starter.into();

    loop {
        if sequence.len() >= turns as usize {
            return sequence;
        }

        let last = *sequence.last().unwrap();
    }
}

fn main() {
    let input = vec![0, 12, 6, 13, 20, 1, 17];

    let result = find_sequence(&input, 2020);
    dbg!(result.last());
}

#[cfg(test)]
mod tests {
    use crate::find_sequence;

    #[test]
    fn test_find_sequence() {
        let input = vec![0, 3, 6];
        assert_eq!(vec![0, 3, 6, 0, 3, 3, 1, 0, 4, 0], find_sequence(&input, 10));

        let result = find_sequence(&input, 2020);
        assert!(result.last().is_some());
        assert_eq!(436, *result.last().unwrap());
    }

    #[test]
    fn test_find_more_sequences() {
        assert_eq!(Some(&1), find_sequence(&[1, 3, 2], 2020).last());
        assert_eq!(Some(&10), find_sequence(&[2, 1, 3], 2020).last());
        assert_eq!(Some(&27), find_sequence(&[1, 2, 3], 2020).last());
        assert_eq!(Some(&78), find_sequence(&[2, 3, 1], 2020).last());
        assert_eq!(Some(&438), find_sequence(&[3, 2, 1], 2020).last());
        assert_eq!(Some(&1836), find_sequence(&[3, 1, 2], 2020).last());
    }
}