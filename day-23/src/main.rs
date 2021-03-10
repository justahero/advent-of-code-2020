
/// Play the game a number of rounds
///
fn play_game(num_rounds: usize, cups: &[usize]) -> Vec<usize> {
    let max = *cups.iter().max().unwrap();
    let min = *cups.iter().min().unwrap();

    // Nice, lambda works similar to a function
    let next_dest = |cup: usize| -> usize { if cup > min { cup - 1 } else { max } };

    let mut circle = vec![0; max + 1];
    (0..cups.len()).for_each(|i| circle[cups[i]] = cups[(i + 1) % cups.len()]);

    let mut current_cup = cups[0];
    for _ in 0..num_rounds {
        let mut extra_cups = [0; 3];
        extra_cups[0] = circle[current_cup];
        extra_cups[1] = circle[extra_cups[0]];
        extra_cups[2] = circle[extra_cups[1]];

        let mut destination = next_dest(current_cup);
        while extra_cups.contains(&destination) {
            destination = next_dest(destination);
        }

        circle.swap(destination, current_cup);
        circle.swap(current_cup, extra_cups[2]);

        current_cup = circle[current_cup];
    }

    remap_cups(&circle)
}

/// Creates a long list with a 1_000_000 entries
fn create_long_list(initial: &[usize]) -> Vec<usize> {
    let mut result = Vec::new();
    result.append(&mut initial.to_vec());
    result.append(&mut (initial.len()+1..=1_000_000).collect::<Vec<_>>());
    result
}

/// Remaps the list of cups back to numbers instead of indices.
fn remap_cups(cups: &[usize]) -> Vec<usize> {
    let mut result = Vec::new();
    let mut i = 1;
    (1..cups.len()).for_each(|_| {
        result.push(cups[i]);
        i = cups[i];
    });
    result
}

/// Transforms the given list of cups into a string
/// This function assumes the list of cups is ordered / remapped
fn part1(cups: &[usize]) -> String {
    let pos = cups.iter().position(|&item| item == 1).unwrap();
    cups.iter()
        .cycle()
        .skip(pos + 1)
        .take(cups.len() - 1)
        .map(|&item| item.to_string())
        .collect::<Vec<_>>()
        .join("")
}

fn main() {
    let cups = vec![3, 9, 4, 6, 1, 8, 5, 2, 7];
    let number = part1(&play_game(100, &cups));
    assert_eq!("78569234", number);

    let long_cups = create_long_list(&cups);
    let result = play_game(10_000_000, &long_cups);
    let x = result[0];
    let y = result[1];

    dbg!(x * y);
}

#[cfg(test)]
mod tests {
    use crate::{create_long_list, part1, play_game};

    #[test]
    fn test_single_game() {
        let cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        assert_eq!(vec![5, 4, 6, 7, 3, 2, 8, 9, 1], play_game(1, &cups));
    }

    #[test]
    fn test_cups_to_string() {
        let cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        assert_eq!(String::from("67384529"), part1(&play_game(100, &cups)));
    }

    #[test]
    fn test_part1_game() {
        let cups = vec![3, 9, 4, 6, 1, 8, 5, 2, 7];
        assert_eq!(String::from("78569234"), part1(&play_game(100, &cups)));
    }

    #[test]
    fn test_create_list() {
        let cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        assert_eq!(1_000_000, create_long_list(&cups).len());
        assert_eq!(1_000_000, *create_long_list(&cups).last().unwrap());
    }

    #[test]
    fn test_long_play_game() {
        let cups = create_long_list(&[3, 8, 9, 1, 2, 5, 4, 6, 7]);
        let result = play_game(10_000_000, &cups);

        assert_eq!(vec![934001, 159792], result.iter().take(2).cloned().collect::<Vec<_>>());
    }
}
