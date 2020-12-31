use std::ops::AddAssign;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    const ZERO: Point = Point{ x: 0, y: 0 };

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

const DIRS: [Point; 4] = [Point { x: 1, y: 0 }, Point { x: 0, y: -1 }, Point { x: -1, y:  0 }, Point { x: 0, y: 1 }];

/// Navigates the ship from start position 0, 0 until all instructions are processed
/// Returns the final position of the ship
fn navigate(instructions: &[&str]) -> Point {

    let mut pos = Point{ x: 0, y: 0 };
    let mut dir_index: usize = 0; // east

    for &instruction in instructions.iter() {
        let count = instruction[1..].parse::<i32>().unwrap();
        let new_pos: Point = match &instruction[0..1] {
            "N" => Point::new(0, count),
            "E" => Point::new(count, 0),
            "S" => Point::new(0, -count),
            "W" => Point::new(-count, 0),
            "L" => { dir_index = (dir_index as i32 - (count / 90)).rem_euclid(4) as usize; Point::ZERO },
            "R" => { dir_index = (dir_index + (count / 90) as usize) % 4; Point::ZERO },
            "F" => Point::new(count * DIRS[dir_index].x, count * DIRS[dir_index].y),
            _ => panic!("Unexpected instruction found"),
        };

        pos += new_pos;
    }

    pos
}

fn navigate_waypoint(instructions: &[&str]) -> Point {
    const DIRS: [(i32, i32); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

    let mut pos = Point::new(0, 0);
    let mut waypoint = Point::new(10, 1);
    let mut dir_index: usize = 0; // east

    for &instruction in instructions.iter() {
        let count = instruction[1..].parse::<i32>().unwrap();

    }

    pos
}

fn main() {
    let instructions = include_str!("ferry.txt")
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .collect::<Vec<_>>();

    let point = navigate(&instructions);
    dbg!(point.manhattan());

    let point = navigate_waypoint(&instructions);
    dbg!(point.manhattan());
}

#[cfg(test)]
mod tests {
    use crate::{Point, navigate, navigate_waypoint};

    #[test]
    fn test_navigate_ship() {
        let instructions = vec!["F10", "N3", "F7", "R90", "F11"];
        assert_eq!(Point::new(17, -8), navigate(&instructions));
    }

    #[test]
    fn test_navigation_with_turns() {
        let instructions = vec!["R90", "L90", "L90", "L90", "L90", "R270", "F10"];
        assert_eq!(Point::new(10, 0), navigate(&instructions));
    }

    #[test]
    fn test_navigate_with_waypoint() {
        let instructions = vec!["F10", "N3", "F7", "R90", "F11"];
        assert_eq!(Point::new(214, -72), navigate_waypoint(&instructions));
    }
}