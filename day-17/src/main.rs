use std::fmt::Debug;

use itertools::{Itertools, MinMaxResult};

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
struct Cube {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone)]
struct Grid {
    pub cubes: Vec<Cube>,
}

impl Grid {
    pub fn parse(content: &str) -> anyhow::Result<Self> {
        let lines = content
            .lines()
            .map(str::trim)
            .filter(|&line| !line.is_empty())
            .collect::<Vec<_>>();

        let mut cubes = Vec::new();
        for (y, &row) in lines.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c == '#' {
                    cubes.push(Cube{ x: x as i32, y: y as i32, z: 1 });
                }
            }
        }

        Ok(Self { cubes })
    }

    /// Returns width of the grid
    pub fn min_x(&self) -> i32 {
        self.cubes.iter().map(|cube| cube.x).min().unwrap_or(0)
    }

    /// Returns width of the grid
    pub fn max_x(&self) -> i32 {
        self.cubes.iter().map(|cube| cube.x).max().unwrap_or(0)
    }

    /// Returns width of the grid
    pub fn min_y(&self) -> i32 {
        self.cubes.iter().map(|cube| cube.y).min().unwrap_or(0)
    }

    /// Returns width of the grid
    pub fn max_y(&self) -> i32 {
        self.cubes.iter().map(|cube| cube.y).max().unwrap_or(0)
    }

    /// Returns width of the grid
    pub fn min_z(&self) -> i32 {
        self.cubes.iter().map(|cube| cube.z).min().unwrap_or(0)
    }

    /// Returns width of the grid
    pub fn max_z(&self) -> i32 {
        self.cubes.iter().map(|cube| cube.z).max().unwrap_or(0)
    }

    /// Returns the cube at 3-dimensional coordinates
    pub fn cube(&self, x: i32, y: i32, z: i32) -> Option<&Cube> {
        self.cubes
            .iter()
            .find(|&cube| cube.x == x && cube.y == y && cube.z == z)
    }

    /// Conway cycle
    pub fn cycle(grid: &Grid, num_cycles: u32) -> anyhow::Result<Grid> {
        let mut grid = grid.clone();

        for _ in 0..num_cycles {
            let mut cubes = Vec::new();

            for x in grid.min_x()-1..grid.max_x()+2 {
                for y in grid.min_y()-1..grid.max_y()+2 {
                    for z in grid.min_z()-1..grid.max_z()+2 {
                        let neighbors = grid.neighbors(x, y, z);

                        if grid.cube(x, y, z).is_some() {
                            if neighbors == 2 || neighbors == 3 {
                                cubes.push(Cube{ x, y, z });
                            }
                        } else {
                            if neighbors == 3 {
                                cubes.push(Cube{ x, y, z });
                            }
                        }
                    }
                }
            }

            grid = Grid{ cubes };
        }

        Ok(grid)
    }

    /// Returns the number of active cells
    pub fn num_active(&self) -> usize {
        self.cubes.len()
    }

    /// Returns the number of active neighbors
    pub fn neighbors(&self, x: i32, y: i32, z: i32) -> u64 {
        let mut neighbors = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if (dx != 0 || dy != 0 || dz != 0) && self.cube(x + dx, y + dy, z + dz).is_some() {
                        neighbors += 1;
                    }
                }
            }
        }
        neighbors
    }
}

fn main() -> anyhow::Result<()> {
    let grid = Grid::parse(INPUT)?;

    let result = Grid::cycle(&grid, 6)?;
    dbg!(result.num_active());

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
        assert_eq!(5, grid.num_active());
        assert_eq!(5, grid.neighbors(1, 1, 0));
    }

    #[test]
    fn test_single_cycle() {
        let input = r#"
            .#.
            ..#
            ###
        "#;

        let grid = Grid::parse(input).unwrap();
        let grid = Grid::cycle(&grid, 1);
        assert!(grid.is_ok());
        let grid = grid.unwrap();

        assert_eq!(11, grid.num_active());
    }

    #[test]
    fn test_multiple_cycles() {
        let input = r#"
            .#.
            ..#
            ###
        "#;

        let grid = Grid::parse(input).unwrap();
        assert_eq!(21, Grid::cycle(&grid, 2).unwrap().num_active());
        assert_eq!(38, Grid::cycle(&grid, 3).unwrap().num_active());
        assert_eq!(112, Grid::cycle(&grid, 6).unwrap().num_active());
    }
}
