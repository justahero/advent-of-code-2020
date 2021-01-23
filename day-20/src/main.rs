use bitvec::prelude::*;
use itertools::Itertools;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Dir {
    Top = 0,
    Right = 1,
    Bottom = 2,
    Left = 3,
}

impl Dir {
    pub fn opposite(&self) -> Self {
        match self {
            Dir::Top => Dir::Bottom,
            Dir::Right => Dir::Left,
            Dir::Bottom => Dir::Top,
            Dir::Left => Dir::Right,
        }
    }
}

#[derive(Clone)]
/// A tile contains image data
struct Tile {
    /// The tile id number
    pub id: u32,
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
        write!(f, "Id: {}\n{}", self.id, lines.join("\n"))
    }
}

impl Tile {
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

    /// Flip edges horizontally
    pub fn flip_h(&mut self) -> &mut Self {
        self.edges.swap(0, 2);
        self.edges[1].reverse();
        self.edges[3].reverse();
        self
    }

    /// Flip edges vertically
    pub fn flip_v(&mut self) -> &mut Self {
        self.edges.swap(1, 3);
        self.edges[0].reverse();
        self.edges[2].reverse();
        self
    }

    /// Create an iterator over all combinations
    pub fn combinations(&self) -> Vec<Tile> {
        let mut current = self.clone();
        let mut items = Vec::new();

        for _ in 0..4 {
            items.push(current.rotate().clone());
        }
        current.flip_h();
        for _ in 0..4 {
            items.push(current.rotate().clone());
        }
        current.flip_v();
        for _ in 0..4 {
            items.push(current.rotate().clone());
        }

        items
    }

    /// Check if this tile links to another one by iterating combinations
    /// If there is a link between the current and next tile return the rotated tile
    pub fn links(&self, rhs: &Tile, dir: &Dir) -> Option<Tile> {
        rhs.combinations()
            .iter()
            .find(|&tile| self.edge(dir.clone()) == tile.edge(dir.opposite()))
            .cloned()
    }
}

#[derive(Debug)]
struct Grid {
    /// The list of all tiles
    pub tiles: Vec<Tile>,
}

impl Grid {
    pub fn count(&self) -> usize {
        self.tiles.len()
    }

    /// Match algorithm to find the grid layout of all tiles
    ///
    /// This function iterates over all tiles and matches neighboring tiles
    /// by rotating, flipping them. All tiles need to match the grid, e.g. 3x3 or 4x4
    ///
    pub fn find_layout(&self) -> anyhow::Result<Grid> {
        let size = (self.tiles.len() as f64).sqrt() as u32;

        // find initial tile and all others
        for (index, next) in self.tiles.iter().enumerate() {
            for current in next.combinations().iter() {
                let mut tiles = self.tiles.clone();
                tiles.remove(index);
                if let Some(tiles) = Self::find_tiles(vec![current.clone()], tiles, size, 0, 0) {
                    return Ok(Grid { tiles })
                }
            }
        }

        Err(anyhow::anyhow!("No matching grid found"))
    }

    /// Find the next tile, depth search first
    fn find_tiles(visited: Vec<Tile>, tiles: Vec<Tile>, size: u32, x: u32, y: u32) -> Option<Vec<Tile>> {
        println!("-- FIND TILES - VISITED {}", visited.len());

        if tiles.is_empty() {
            return Some(visited);
        }

        let dir = if x < size - 1 { Dir::Right } else { Dir::Bottom };
        let current = match dir {
            Dir::Right => visited.last().unwrap(),
            _ => visited.get(visited.len() - size as usize).unwrap(),
        };

        for (index, next) in tiles.iter().enumerate() {
            if let Some(next) = current.links(&next, &dir) {
                if let Some(pos) = Self::next_pos(size, x, y) {
                    let mut cloned = tiles.clone();
                    cloned.remove(index);
                    let mut copy = visited.clone();
                    copy.push(next);

                    if let Some(tiles) = Self::find_tiles(copy, cloned, size, pos.0, pos.1) {
                        return Some(tiles);
                    }
                }
            }
        }

        None
    }

    /// Returns the next possible location
    fn next_pos(size: u32, x: u32, y: u32) -> Option<(u32, u32)> {
        if x < size - 1 {
            Some((x + 1, y))
        } else if y < size - 1{
            Some((0, y + 1))
        } else {
            None
        }
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
    let id = result[0][5..size - 1].parse()?;

    let grid = result[1..]
        .iter()
        .map(|&line| line.chars().map(|x| x == '#').collect::<BitVec>())
        .collect::<Vec<_>>();

    // extract edges of grid, top, right, bottom, left
    let edges = [
        grid[0].clone(),
        grid.iter().map(|vec| vec[size-1]).collect::<BitVec>(),
        grid[size-1].clone(),
        grid.iter().map(|vec| vec[0]).rev().collect::<BitVec>(),
    ];

    Ok(Tile {
        id,
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
    let layout = grid.find_layout()?;

    dbg!(&layout);

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
        let tile = parse_tile(content);
        assert!(tile.is_ok());

        let tile = tile.unwrap();
        assert_eq!(12, tile.combinations().len());
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
    fn test_flip_tile_horizontally() {
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
        tile.flip_h();

        assert_eq!(&bitvec![0, 0, 1, 1, 1, 0, 0, 1, 1, 1], tile.edge(Dir::Top));
        assert_eq!(&bitvec![0, 1, 1, 1, 1, 1, 0, 0, 1, 0], tile.edge(Dir::Left));
        assert_eq!(&bitvec![0, 0, 1, 1, 0, 1, 0, 0, 1, 0], tile.edge(Dir::Bottom));
        assert_eq!(&bitvec![1, 0, 0, 1, 1, 0, 1, 0, 0, 0], tile.edge(Dir::Right));
    }

    #[test]
    fn test_flip_tile_vertically() {
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
        tile.flip_v();

        assert_eq!(&bitvec![0, 1, 0, 0, 1, 0, 1, 1, 0, 0], tile.edge(Dir::Top));
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

    #[test]
    fn test_find_layout() {
        let grid = parse_tile_grid(TILES).unwrap();

        let layout = grid.find_layout();
        assert!(layout.is_ok());

        let grid = layout.unwrap();
        let ids = vec![1951, 2311, 3079, 2729, 1427, 2473, 2971, 1489, 1171];
        assert_eq!(ids, grid.tiles.iter().map(|t| t.id).collect::<Vec<_>>());
    }
}
