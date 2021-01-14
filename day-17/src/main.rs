use ndarray::*;
use itertools::Itertools;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum CubeState {
    Active,
    Inactive,
}

impl Default for CubeState {
    fn default() -> Self {
        Self::Inactive
    }
}

#[derive(Debug)]
struct Grid {
    pub cubes: Array3::<CubeState>,
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
            .unwrap();

        let height = lines
            .iter()
            .count();

        let mut cubes = Array3::<CubeState>::from_elem((width, height, 1), CubeState::Inactive);

        for (y, &row) in lines.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let state = if c == '#' { CubeState::Active } else { CubeState::Inactive };
                cubes[[x, y, 0]] = state;
            }
        }

        Ok(Self { cubes })
    }

    /// Returns width of the grid
    pub fn width(&self) -> usize {
        self.cubes.dim().0
    }

    /// Returns height of the grid
    pub fn height(&self) -> usize {
        self.cubes.dim().1
    }
    
    /// Returns depth of the grid
    pub fn depth(&self) -> usize {
        self.cubes.dim().2
    }

    /// Returns the cube at 3-dimensional coordinates
    pub fn cube(&self, x: usize, y: usize, z: usize) -> Option<&CubeState> {
        self.cubes.get([x, y, z])
    }

    /// Conway cycle
    pub fn cycle(&self) -> anyhow::Result<Grid> {
        let width = self.width() + 2;
        let height = self.height() + 2;
        let depth = self.depth() + 2;
        let mut cubes = Array3::<CubeState>::from_elem((width, height, depth), CubeState::Inactive);

        for z in 0..depth {
            for y in 0..height {
                for x in 0..width {
                    let result = self.cube(x + 1, y + 1, z + 1);
                    let state = match result {
                        Some(state) => {
                            let adjacent = self.neighbors(x + 1, y + 1, z + 1);
                            if *state == CubeState::Active {
                                if adjacent == 2 || adjacent == 3 { CubeState::Active } else { CubeState::Inactive }
                            } else if adjacent == 3 { CubeState::Active } else { CubeState::Inactive }
                        }
                        None => CubeState::Inactive,
                    };

                    cubes[[x, y, z]] = state;
                }
            }
        }

        Ok(Grid{ cubes })
    }

    /// Returns the number of active cells
    pub fn num_active(&self) -> usize {
        self.cubes.iter().filter(|&state| *state == CubeState::Active).count()
    }

    /// Returns the number of active neighbors
    pub fn neighbors(&self, x: usize, y: usize, z: usize) -> u64 {
        let list: [[i32; 3]; 3] = [
            [-1, 0, 1],
            [-1, 0, 1],
            [-1, 0, 1],
        ];

        // generate all neighbors
        let adjacent = list
            .iter()
            .map(IntoIterator::into_iter)
            .multi_cartesian_product()
            .map(|v| (*v[0] as usize + x, *v[1] as usize + y, *v[2] as usize + z))
            .collect::<Vec<_>>();

        adjacent
            .iter()
            .filter(|(x, y, z)| *x != 0 || *y != 0 || *z != 0)
            .map(|(x, y, z)| {
                match self.cube(*x, *y, *z) {
                    Some(state) => *state,
                    None => CubeState::Inactive,
                }
            })
            .filter(|&state| state == CubeState::Active)
            .count() as u64
    }
}

fn main() -> anyhow::Result<()> {
    let mut grid = Grid::parse(INPUT)?;

    for _ in 0..6 {
        grid = grid.cycle()?;
    }
    dbg!(grid.num_active());

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
        dbg!(&grid);
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
