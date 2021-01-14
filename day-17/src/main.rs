use itertools::Itertools;
use std::ops::Range;

/// the input grid
const INPUT: &str = r#"
    .###..#.
    ##.##...
    ....#.#.
    #..#.###
    ...#...#
    ##.#...#
    #..##.##
    #.......
"#;

#[derive(Debug, Clone, PartialEq, Eq)]
enum CubeState {
    Active,
    Inactive,
}

#[derive(Debug, Clone)]
struct Cube {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub state: CubeState,
}

impl Cube {
    pub fn new(x: i32, y: i32, z: i32, state: CubeState) -> Self {
        Self {
            x, y, z, state
        }
    }

    pub fn active(&self) -> bool {
        self.state == CubeState::Active
    }
}

#[derive(Debug)]
struct Grid {
    pub x_range: Range<i32>,
    pub y_range: Range<i32>,
    pub z_range: Range<i32>,
    pub cubes: Vec<Cube>
}

impl Grid {
    pub fn parse(content: &str) -> anyhow::Result<Self> {
        let lines = content
            .lines()
            .map(str::trim)
            .filter(|&line| !line.is_empty())
            .collect::<Vec<_>>();

        let width = lines
            .iter()
            .map(|x| x.len())
            .max()
            .unwrap() as i32;

        let height = lines
            .iter()
            .count() as i32;

        let mut cubes = Vec::new();
        for (y, &row) in lines.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let state = if c == '#' { CubeState::Active } else { CubeState::Inactive };
                cubes.push(Cube::new(x as i32 - 1, y as i32 - 1, 0i32, state));
            }
        }

        Ok(Self {
            x_range: 0..width,
            y_range: 0..height,
            z_range: 0..1,
            cubes,
        })
    }

    /// Returns width of the grid
    pub fn width(&self) -> i32 {
        self.x_range.end - self.x_range.start
    }

    /// Returns height of the grid
    pub fn height(&self) -> i32 {
        self.y_range.end - self.y_range.start
    }
    
    /// Returns depth of the grid
    pub fn depth(&self) -> i32 {
        self.z_range.end - self.z_range.start
    }

    /// Conway cycle
    pub fn cycle(&self) -> anyhow::Result<Grid> {
        let mut grid = Grid {
            z_range: (self.z_range.start - 1)..(self.z_range.end + 1),
            y_range: (self.y_range.start - 1)..(self.y_range.end + 1),
            x_range: (self.x_range.start - 1)..(self.x_range.end + 1),
            cubes: Vec::new(),
        };

        let mut index = 0;
        for z in grid.z_range.start..=grid.z_range.end {
            for y in grid.y_range.start..=grid.y_range.end {
                for x in grid.x_range.start..=grid.x_range.end {
                    let adjacent = self.neighbors(x, y, z);
                    let state = if self.cubes[index].state == CubeState::Active {
                        if adjacent == 2 || adjacent == 3 { CubeState::Active } else { CubeState::Inactive }
                    } else if adjacent == 3 { CubeState::Active } else { CubeState::Inactive };

                    grid.cubes.push(Cube::new(x, y, z, state));
                    index += 1;
                }
            }
        }

        Ok(grid)
    }

    /// Returns the number of active cells
    pub fn num_active(&self) -> usize {
        self.cubes.iter().filter(|c| c.active()).count()
    }

    /// Returns the number of active neighbors
    pub fn neighbors(&self, x: i32, y: i32, z: i32) -> u64 {
        let list = [
            [-1, 0, 1],
            [-1, 0, 1],
            [-1, 0, 1],
        ];

        let adjacent = list
            .iter()
            .map(IntoIterator::into_iter)
            .multi_cartesian_product()
            .map(|v| (v[0], v[1], v[2]))
            .collect::<Vec<_>>();

        let mut result = 0;

        for (&x, &y, &z) in adjacent.iter() {
            if !self.x_range.contains(&x) || !self.y_range.contains(&y) || !self.z_range.contains(&z) {
                continue;
            }

            // let index = x + self.width() * (y + self.depth() * z);
        }

        /*

        for z in (cube.z - 1)..=(cube.z + 1) {
            for y in (cube.y - 1)..=(cube.y + 1) {
                for x in (cube.x - 1)..=(cube.x + 1) {
                    
                }
            }
        }
        */

        result
    }

/*

    /// Return number of occupied adjacent seats
    pub fn adjacent(&self, x: i64, y: i64, steps: u32) -> u32 {
        let mut result = 0;

        // define all the directions
        let dirs= vec![
            (-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)
        ];

        for (i, j) in dirs.iter() {
            let mut sx = x;
            let mut sy = y;

            for _ in 0..steps {
                sx += i;
                sy += j;

                // if adjacent seat is outside grid, advance to next
                if sx < 0 || sx >= self.width as i64 || sy < 0 || sy >= self.height as i64 {
                    continue;
                }

                let index = (sx + sy * self.width as i64) as usize;
                match self.seats[index] {
                    Seat::Occupied => {
                        result += 1;
                        break;
                    }
                    Seat::Empty => break,
                    Seat::Floor => (),
                }
            }
        }

        result
    }
*/
}

fn main() -> anyhow::Result<()> {
    let mut grid = Grid::parse(INPUT)?;

    for _ in 0..6 {
        grid = grid.cycle()?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Grid;

    #[test]
    fn test_initial_state() {
        let input = r#"
            .#.
            ..#
            ###
        "#;

        let grid = Grid::parse(input);
        assert!(grid.is_ok());

        let grid = grid.unwrap();
        assert_eq!(3, grid.width());
        assert_eq!(3, grid.height());
        assert_eq!(1, grid.depth());
        assert_eq!(5, grid.num_active());
    }

    #[test]
    fn test_single_cycle() {
        let input = r#"
            .#.
            ..#
            ###
        "#;

        let grid = Grid::parse(input).unwrap();
        let grid = grid.cycle();
        assert!(grid.is_ok());
        let grid = grid.unwrap();
        assert_eq!(5, grid.width());
        assert_eq!(5, grid.height());
        assert_eq!(3, grid.depth());
        assert_eq!(11, grid.num_active());
    }

    #[test]
    fn test_neighbors() {
        let input = r#"
            .#.
            ..#
            ###
        "#;

        let grid = Grid::parse(input).unwrap();
        assert_eq!(5, grid.neighbors(0, 0, 0));
    }

    #[test]
    fn test_multiple_cycles() {
        let input = r#"
            .#.
            ..#
            ###
        "#;


    }
}
