/// Play next move, return modified list
///
/// * arrange list in cyclic order with index offset
/// * pick first element from list, the current cup
/// * remove next three cups after current cup
/// * find index of destination (with applied rules)
/// * re-arrange the result list
/// * adjust the initial cycle with index offset
///
/// TODO refactor for part 2 !!! :(
///
fn next_move(current_index: usize, circle: &[u64]) -> Vec<u64> {
    let mut list = circle
        .iter()
        .cycle()
        .skip(current_index)
        .take(circle.len())
        .cloned()
        .collect::<Vec<_>>();

    let max = *list.iter().max().unwrap() + 1;
    let mut cups = list.drain(1..4).collect::<Vec<_>>();

    let mut destination = list[0] - 1;
    let mut pos;

    loop {
        pos = list.iter().position(|&item| item == destination);

        if cups.contains(&destination) || pos.is_none() {
            destination = (destination + (max - 1)) % max;
        } else {
            break;
        }
    }

    let pos = pos.unwrap();
    let new_cup = list.remove(pos);

    // arrange result list
    let mut result = vec![list[0]];
    result.append(&mut list[1..pos].to_vec());
    result.push(new_cup);
    result.append(&mut cups);
    result.append(&mut list[pos..].to_vec());
    assert_eq!(result.len(), circle.len());

    result
        .iter()
        .cycle()
        .skip(circle.len() - current_index)
        .take(circle.len())
        .cloned()
        .collect::<Vec<_>>()
}

/// Play the game a number of rounds
fn play_game(num_rounds: usize, circle: &[u64]) -> Vec<u64> {
    (0..num_rounds)
        .fold(circle.to_vec(), |result, index| {
            next_move(index % circle.len(), &result)
        })
}

/// Creates a long list with a 1_000_000 entries
fn create_long_list(initial: &[u64]) -> Vec<u64> {
    let mut result = Vec::new();
    result.append(&mut initial.to_vec());
    result.append(&mut (initial.len() as u64..1_000_000u64).collect::<Vec<_>>());
    result
}

/// Creates the ordered string
fn ordered_number(numbers: &[u64], length: usize) -> Vec<u64> {
    let index = numbers.iter().position(|&item| item == 1).unwrap();

    numbers
        .iter()
        .cloned()
        .cycle()
        .skip(index + 1)
        .take(length)
        .collect::<Vec<_>>()
}

fn main() {
    let cups = vec![3, 9, 4, 6, 1, 8, 5, 2, 7];
    let result = play_game(100, &cups);
    dbg!(&result);

    let order = ordered_number(&result, result.len() - 1)
        .iter()
        .map(|val| val.to_string())
        .collect::<Vec<_>>()
        .join("");
    dbg!(order);

    let long_cups = create_long_list(&cups);
    let result = play_game(10_000_000, &long_cups);
    let order = ordered_number(&result, 2);
    dbg!(order);
}

#[cfg(test)]
mod tests {
    use crate::{create_long_list, next_move, ordered_number, play_game};

    #[test]
    fn test_next_move() {
        let cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        assert_eq!(vec![3, 2, 8, 9, 1, 5, 4, 6, 7], next_move(0, &cups));
    }

    #[test]
    fn test_play_game() {
        let cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        assert_eq!(vec![3, 2, 8, 9, 1, 5, 4, 6, 7], play_game(1, &cups));
        assert_eq!(vec![3, 2, 5, 4, 6, 7, 8, 9, 1], play_game(2, &cups));
        assert_eq!(vec![7, 2, 5, 8, 9, 1, 3, 4, 6], play_game(3, &cups));
        assert_eq!(vec![3, 2, 5, 8, 4, 6, 7, 9, 1], play_game(4, &cups));
        assert_eq!(vec![9, 2, 5, 8, 4, 1, 3, 6, 7], play_game(5, &cups));
        assert_eq!(vec![5, 8, 3, 7, 4, 1, 9, 2, 6], play_game(10, &cups));
    }

    #[test]
    fn test_ordered_number() {
        let cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        let result = play_game(100, &cups);
        assert_eq!(vec![6, 7, 3, 8, 4, 5, 2, 9], ordered_number(&result, cups.len() - 1));
    }

    #[test]
    fn test_create_list() {
        let cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        assert_eq!(1_000_000, create_long_list(&cups).len());
    }

    #[test]
    fn test_long_play_game() {
        let cups = create_long_list(&[3, 8, 9, 1, 2, 5, 4, 6, 7]);
        let result = play_game(10_000_000, &cups);
        assert_eq!(vec![934001, 159792], ordered_number(&result, 2));
    }
}
