/// Play next move, return modified list
fn next_move(current: u64, circle: &[u64]) -> Vec<u64> {
    circle.to_vec()
}

/// Play the game a number of rounds
fn play_game(num_rounds: u64, circle: &[u64]) -> Vec<u64> {
    circle.to_vec()
}

fn main() {
    let cups = vec![3, 9, 4, 6, 1, 8, 5, 2, 7];
}

#[cfg(test)]
mod tests {
    use crate::{next_move, play_game};

    #[test]
    fn test_next_move() {
        let cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        assert_eq!(vec![3, 2, 8, 9, 1, 5, 4, 6, 7], next_move(0, &cups));
        assert_eq!(vec![3, 2, 5, 4, 6, 7, 8, 9, 1], next_move(0, &cups));
        assert_eq!(vec![7, 2, 5, 8, 9, 1, 3, 4, 6], next_move(0, &cups));
        assert_eq!(vec![3, 2, 5, 8, 4, 6, 7, 9, 1], next_move(0, &cups));
        assert_eq!(vec![9, 2, 5, 8, 4, 1, 3, 6, 7], next_move(0, &cups));
    }

    #[test]
    fn test_play_game() {
        let cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        assert_eq!(vec![9, 2, 5, 8, 4, 1, 3, 6, 7], play_game(10, &cups));
    }
}
