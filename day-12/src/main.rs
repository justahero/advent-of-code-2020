use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    const ZERO: Point = Point{ x: 0, y: 0 };

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn rotate(&mut self, degress: i32) {
        *self = match degress {
            90 | -270 => Point::new(self.y, -self.x),
            180 | -180 => Point::new(-self.x, -self.y),
            270 | -90 => Point::new(-self.y, self.x),
            _ => *self,
        };
    }

    pub fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl MulAssign for Point {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Mul<Point> for i32 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl From<(i32, i32)> for Point {
    fn from(tuple: (i32, i32)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
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
    let mut pos = Point::new(0, 0);
    let mut waypoint = Point::new(10, 1);

    for &instruction in instructions.iter() {
        let count = instruction[1..].parse::<i32>().unwrap();
        match &instruction[0..1] {
            "N" => { waypoint += Point::new(0, count); },
            "E" => { waypoint += Point::new(count, 0); },
            "S" => { waypoint += Point::new(0, -count); },
            "W" => { waypoint += Point::new(-count, 0); },
            "F" => { pos += count * waypoint },
            "L" => { waypoint.rotate(-count); },
            "R" => { waypoint.rotate(count); },
            _ => panic!("unsupported instruction found"),
        };
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

    #[test]
    fn test_navigate_with_longer_waypoint() {
        let instructions = vec!["F5", "R90", "L270", "N2", "W5", "F3", "L90", "F2"];
        assert_eq!(Point::new(3, -22), navigate_waypoint(&instructions));
    }
}