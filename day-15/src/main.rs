/// Find the sequence with given start numbers.
fn find_sequence(starter: &[u64], turns: u64) -> Vec<u64> {
    let mut sequence: Vec<u64> = starter.into();

    // try to refactor the function to be more efficient
    'outer: for index in sequence.len()..turns as usize {
        let last = *sequence.last().unwrap();

        // try to find previous occurence
        for i in (0..index - 1).rev() {
            if sequence[i] == last {
                sequence.push((sequence.len() - i - 1) as u64);
                continue 'outer;
            }
        }

        sequence.push(0);
    }

    sequence
}


fn main() {
    let input = vec![0, 12, 6, 13, 20, 1, 17];

    let result = find_sequence(&input, 2020);
    dbg!(result.last());

    let result = find_sequence(&input, 30000000);
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

    #[test]
    fn test_find_very_long_sequences() {
        assert_eq!(Some(&175594), find_sequence(&[0, 3, 6], 30000000).last());
        assert_eq!(Some(&2578), find_sequence(&[1, 3, 2], 30000000).last());
        assert_eq!(Some(&3544142), find_sequence(&[2, 1, 3], 30000000).last());
    }
}