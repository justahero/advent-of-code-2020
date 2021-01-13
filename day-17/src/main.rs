
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

#[derive(Debug, Default)]
struct Grid {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
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
            .unwrap();

        let height = lines
            .iter()
            .count();

        let mut cubes: Vec<Cube> = vec![];
        for (y, &row) in lines.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let state = if c == '#' { CubeState::Active } else { CubeState::Inactive };
                cubes.push(Cube::new(x as i32, y as i32, 0i32, state));
            }
        }

        Ok(Self {
            width,
            height,
            depth: 1,
            cubes,
        })
    }

    pub fn cycle(&self) -> anyhow::Result<Grid> {
        Ok(Self::default())
    }

    /// Returns the number of active cells
    pub fn num_active(&self) -> usize {
        self.cubes.iter().filter(|c| c.active()).count()
    }
}

fn main() -> anyhow::Result<()> {
    let grid = Grid::parse(INPUT);

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
        assert_eq!(3, grid.width);
        assert_eq!(3, grid.height);
        assert_eq!(1, grid.depth);
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
        assert_eq!(11, grid.unwrap().num_active());
    }

    #[test]
    fn test_multiple_cycles() {

    }
}
