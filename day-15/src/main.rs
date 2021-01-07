/// Finds the sequence of numbers that are spoken.
fn find_sequence(starter: &[u32], turns: u32) -> Vec<u32> {
    Vec::new()
}

fn main() {
    let input = vec![0, 12, 6, 13, 20, 1, 17];
    println!("Hello, world!");
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
}