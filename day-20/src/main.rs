use bitvec::prelude::*;
use std::fmt::Debug;

#[derive(Debug)]
struct Grid {
    /// The list of all tiles
    pub tiles: Vec<Tile>,
}

impl Grid {
    pub fn count(&self) -> usize {
        self.tiles.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Dir {
    Top = 0,
    Right = 1,
    Bottom = 2,
    Left = 3,
}

#[derive(Clone)]
/// A tile contains image data
struct Tile {
    /// The tile id number
    pub number: u32,
    /// Full Grid
    pub grid: Vec<BitVec>,
    /// Get edges
    pub edges: [BitVec; 4],
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = self.grid
            .iter()
            .map(|row| format!("{:010b}", row))
            .collect::<Vec<_>>();
        write!(f, "Id: {}\n{}", self.number, lines.join("\n"))
    }
}

impl Tile {
    pub fn grid(&self) -> usize {
        self.grid.len()
    }

    pub fn edge(&self, dir: Dir) -> &BitVec {
        &self.edges[dir as usize]
    }

    /// Rotates the edges in clockwise order
    pub fn rotate(&mut self) -> &mut Self {
        self.edges[2].reverse();
        self.edges[3].reverse();
        self.edges.rotate_right(1);
        self
    }
}

/// Parses a single tile block
fn parse_tile(content: &str) -> anyhow::Result<Tile> {
    let result = content
        .lines()
        .map(|line| line.trim())
        .filter(|&line| !line.is_empty())
        .collect::<Vec<_>>();

    if result.is_empty() {
        return Err(anyhow::anyhow!("No lines found"));
    }

    let size = result[0].len();
    let number = result[0][5..size - 1].parse()?;

    let grid = result[1..]
        .iter()
        .map(|&line| line.chars().map(|x| x == '#').collect::<BitVec>())
        .collect::<Vec<_>>();

    // extract edges of grid, top, right, bottom, left
    let edges = [
        grid[0].clone(),
        grid.iter().map(|vec| vec[size-1]).collect::<BitVec>(),
        grid[size-1].clone(),
        grid.iter().map(|vec| vec[0]).collect::<BitVec>(),
    ];

    Ok(Tile {
        number,
        grid,
        edges,
    })
}

/// Parses images tiles from text
fn parse_tile_grid(content: &str) -> anyhow::Result<Grid> {
    let tiles = content
        .split("\n\n")
        .map(|tile| parse_tile(tile))
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    Ok(Grid { tiles })
}

fn main() -> anyhow::Result<()> {
    let grid = parse_tile_grid(include_str!("images.txt"))?;
    dbg!(&grid);

    Ok(())
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::*;
    use bitvec::bitvec;
    use crate::{Dir, parse_tile, parse_tile_grid};

    const TILES: &str = r#"
        Tile 2311:
        ..##.#..#.
        ##..#.....
        #...##..#.
        ####.#...#
        ##.##.###.
        ##...#.###
        .#.#.#..##
        ..#....#..
        ###...#.#.
        ..###..###

        Tile 1951:
        #.##...##.
        #.####...#
        .....#..##
        #...######
        .##.#....#
        .###.#####
        ###.##.##.
        .###....#.
        ..#.#..#.#
        #...##.#..

        Tile 1171:
        ####...##.
        #..##.#..#
        ##.#..#.#.
        .###.####.
        ..###.####
        .##....##.
        .#...####.
        #.##.####.
        ####..#...
        .....##...

        Tile 1427:
        ###.##.#..
        .#..#.##..
        .#.##.#..#
        #.#.#.##.#
        ....#...##
        ...##..##.
        ...#.#####
        .#.####.#.
        ..#..###.#
        ..##.#..#.

        Tile 1489:
        ##.#.#....
        ..##...#..
        .##..##...
        ..#...#...
        #####...#.
        #..#.#.#.#
        ...#.#.#..
        ##.#...##.
        ..##.##.##
        ###.##.#..

        Tile 2473:
        #....####.
        #..#.##...
        #.##..#...
        ######.#.#
        .#...#.#.#
        .#########
        .###.#..#.
        ########.#
        ##...##.#.
        ..###.#.#.

        Tile 2971:
        ..#.#....#
        #...###...
        #.#.###...
        ##.##..#..
        .#####..##
        .#..####.#
        #..#.#..#.
        ..####.###
        ..#.#.###.
        ...#.#.#.#

        Tile 2729:
        ...#.#.#.#
        ####.#....
        ..#.#.....
        ....#..#.#
        .##..##.#.
        .#.####...
        ####.#.#..
        ##.####...
        ##..#.##..
        #.##...##.

        Tile 3079:
        #.#.#####.
        .#..######
        ..#.......
        ######....
        ####.#..#.
        .#...#.##.
        #.#####.##
        ..#.###...
        ..#.......
        ..#.###...
    "#;

    #[test]
    fn test_parse_tile() {
        let content = r#"
            Tile 2311:
            ..##.#..#.
            ##..#.....
            #...##..#.
            ####.#...#
            ##.##.###.
            ##...#.###
            .#.#.#..##
            ..#....#..
            ###...#.#.
            ..###..###
        "#;
        let grid = parse_tile(content);
        assert!(grid.is_ok());

        let grid = grid.unwrap();
        assert_eq!(10, grid.grid());
    }

    #[test]
    fn test_rotate_tile() {
        let content = r#"
            Tile 2311:
            ..##.#..#.
            ##..#.....
            #...##..#.
            ####.#...#
            ##.##.###.
            ##...#.###
            .#.#.#..##
            ..#....#..
            ###...#.#.
            ..###..###
        "#;
        let mut tile = parse_tile(content).unwrap();
        tile.rotate();

        assert_eq!(&bitvec![0, 0, 1, 1, 0, 1, 0, 0, 1, 0], tile.edge(Dir::Right));
        assert_eq!(&bitvec![0, 0, 0, 1, 0, 1, 1, 0, 0, 1], tile.edge(Dir::Bottom));
        assert_eq!(&bitvec![1, 1, 1, 0, 0, 1, 1, 1, 0, 0], tile.edge(Dir::Left));
        assert_eq!(&bitvec![0, 1, 0, 0, 1, 1, 1, 1, 1, 0], tile.edge(Dir::Top));
    }

    #[test]
    fn test_parse_grid() {
        let grid = parse_tile_grid(TILES);
        assert!(grid.is_ok());

        let grid = grid.unwrap();
        assert_eq!(9, grid.count());
    }

    #[test]
    fn test_match_tiles() {
        let left = r#"
            Tile 2311:
            ..##.#..#.
            ##..#.....
            #...##..#.
            ####.#...#
            ##.##.###.
            ##...#.###
            .#.#.#..##
            ..#....#..
            ###...#.#.
            ..###..###
        "#;
        let right = r#"
            Tile 2311:
            ..##.#..#.
            .#..#.....
            ....##..#.
            ####.#...#
            .#.##.###.
            ##...#.###
            ##.#.#..##
            ..#....#..
            .##...#.#.
            #.###..###
        "#;

        let left = parse_tile_grid(left).unwrap().tiles[0].clone();
        let right = parse_tile_grid(right).unwrap().tiles[0].clone();

        assert_eq!(left.edge(Dir::Right), right.edge(Dir::Left));
    }
}
