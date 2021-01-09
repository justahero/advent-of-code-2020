use std::collections::HashMap;

/// Find the sequence with given start numbers.
fn find_sequence(starter: &[u64], turns: u64) -> Option<u64> {
    let mut sequence = starter[0..starter.len() - 1]
        .iter()
        .enumerate()
        .map(|(index, &value)| (value, index as u64))
        .collect::<HashMap<u64, u64>>();

    let mut last = *starter.last().unwrap();
    println!("START {:?} ({})", sequence, last);

    // try to refactor the function to be more efficient
    for index in starter.len()..turns as usize {
        // check if number was last spoken
        last = if let Some((_, &i)) = sequence.get_key_value(&last) {
            let new_index = index as u64 - i;
            sequence.insert(last, new_index);
            new_index
        } else {
            sequence.insert(last, index as u64);
            0
        };
    }

    Some(last)
}

fn main() {
    let input = vec![0, 12, 6, 13, 20, 1, 17];

    let result = find_sequence(&input, 2020);
    dbg!(result);

    let result = find_sequence(&input, 30000000);
    dbg!(result);
}

#[cfg(test)]
mod tests {
    use crate::find_sequence;

    #[test]
    fn test_find_sequence() {
        let input = vec![0, 3, 6];
        assert_eq!(Some(4), find_sequence(&input, 9));
        assert_eq!(Some(0), find_sequence(&input, 10));

        let result = find_sequence(&input, 2020);
        assert!(result.is_some());
        assert_eq!(436, result.unwrap());
    }

    #[test]
    fn test_first_puzzle_answer() {
        let input = vec![0, 12, 6, 13, 20, 1, 17];
        assert_eq!(620, find_sequence(&input, 2020).unwrap());
    }

    #[test]
    fn test_find_more_sequences() {
        assert_eq!(Some(1), find_sequence(&[1, 3, 2], 2020));
        assert_eq!(Some(10), find_sequence(&[2, 1, 3], 2020));
        assert_eq!(Some(27), find_sequence(&[1, 2, 3], 2020));
        assert_eq!(Some(78), find_sequence(&[2, 3, 1], 2020));
        assert_eq!(Some(438), find_sequence(&[3, 2, 1], 2020));
        assert_eq!(Some(1836), find_sequence(&[3, 1, 2], 2020));
    }

    #[test]
    fn test_find_very_long_sequences() {
        assert_eq!(Some(175594), find_sequence(&[0, 3, 6], 30000000));
        assert_eq!(Some(2578), find_sequence(&[1, 3, 2], 30000000));
        assert_eq!(Some(3544142), find_sequence(&[2, 1, 3], 30000000));
    }
}