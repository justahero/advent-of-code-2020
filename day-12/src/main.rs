/// Navigates the ship from start position 0, 0 until all instructions are processed
/// Returns the final position of the ship
fn navigate(instructions: &[&str]) -> (i32, i32) {
    const DIRS: [(i32, i32); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

    let mut pos = (0, 0);
    let mut dir_index: usize = 0; // east

    for &instruction in instructions.iter() {
        let count = instruction[1..].parse::<i32>().unwrap();
        let new_pos = match &instruction[0..1] {
            "N" => (0, count),
            "E" => (count, 0),
            "S" => (0, -count),
            "W" => (-count, 0),
            "L" => { dir_index = (dir_index as i32 - (count / 90)).rem_euclid(4) as usize; (0, 0) },
            "R" => { dir_index = (dir_index + (count / 90) as usize) % 4; (0, 0) },
            "F" => (count * DIRS[dir_index].0, count * DIRS[dir_index].1),
            _ => panic!("Unexpected instruction found"),
        };

        pos.0 += new_pos.0;
        pos.1 += new_pos.1;
    }

    pos
}

fn main() {
    let instructions = include_str!("ferry.txt")
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .collect::<Vec<_>>();

    let (x, y) = navigate(&instructions);
    dbg!(x.abs() + y.abs());
}

#[cfg(test)]
mod tests {
    use crate::navigate;

    #[test]
    fn test_navigate_ship() {
        let instructions = vec!["F10", "N3", "F7", "R90", "F11"];
        assert_eq!((17, -8), navigate(&instructions));
    }

    #[test]
    fn test_navigation_with_turns() {
        let instructions = vec!["R90", "L90", "L90", "L90", "L90", "R270", "F10"];
        assert_eq!((10, 0), navigate(&instructions));
    }
}