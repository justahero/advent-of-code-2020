use ndarray::{Array2, ArrayView1, ArrayView2, s};
use std::fmt::Debug;

#[derive(Debug)]
struct Image {
    /// Width of the image
    pub pixel_size: usize,
    /// Image pixels
    pub data: Array2<u8>,
}

impl From<Grid> for Image {
    fn from(grid: Grid) -> Self {
        let grid_size = grid.side();
        let tile_size = grid.tiles[0].size() - 2;
        let pixel_size = grid_size * tile_size;

        let data = Array2::default((pixel_size, pixel_size));
        for y in 0..grid_size {
            for x in 0..grid_size {
                if let Some(tile) = grid.tile(x, y) {
                    // copy content from tile into the image data 2d array

                    // get view on tile
                    let view = tile.image().to_owned();
                }
            }
        }

        Self {
            pixel_size,
            data,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
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
    pub id: u64,
    /// Full Grid
    pub grid: ndarray::Array2<u8>,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = self.grid
            .outer_iter()
            .map(|row| format!("{:?}", row))
            .collect::<Vec<_>>();
        write!(f, "Id: {}\n{}", self.id, lines.join("\n"))
    }
}

impl Tile {
    /// Return the edge size of the tile
    pub fn size(&self) -> usize {
        self.grid.nrows()
    }

    pub fn edge(&self, dir: Dir) -> ArrayView1<'_, u8>  {
        let size = self.size();
        match dir {
            Dir::Top => self.grid.row(0),
            Dir::Right => self.grid.column(size - 1),
            Dir::Bottom => self.grid.row(size - 1),
            Dir::Left => self.grid.column(0),
        }
    }

    /// Return a view of the inner image, without the border edges
    pub fn image(&self) -> ArrayView2<'_, u8> {
        let size = self.size() as isize;
        self.grid.slice(s![1..size-1, 1..size-1])
    }

    /// Rotates the edges in clockwise order
    pub fn rotate(&mut self) -> &mut Self {
        self.grid = self.grid.slice(s![..; -1, ..]).reversed_axes().into_owned();
        self
    }

    /// Flip edges horizontally
    pub fn flip_h(&mut self) -> &mut Self {
        self.grid = self.grid.slice(s![..; -1, ..]).into_owned();
        self
    }

    /// Flip edges vertically
    pub fn flip_v(&mut self) -> &mut Self {
        self.grid = self.grid.slice(s![.., ..; -1]).into_owned();
        self
    }

    /// Create a list of all tile edge combinations
    pub fn combinations(&self) -> Vec<Tile> {
        let mut current = self.clone();
        let mut items = Vec::new();

        (0..4).for_each(|_| items.push(current.rotate().clone()));

        current = self.clone();
        current.flip_h();
        (0..4).for_each(|_| items.push(current.rotate().clone()));

        current = self.clone();
        current.flip_v();
        (0..4).for_each(|_| items.push(current.rotate().clone()));

        items
    }

    /// Check if this tile links to another one by iterating combinations
    /// If there is a link between the current and next tile return the rotated tile
    pub fn find_link(&self, rhs: &Tile, dir: &Dir) -> Option<Tile> {
        rhs.combinations()
            .iter()
            .find(|&tile| self.edge(*dir) == tile.edge(dir.opposite()))
            .cloned()
    }
}

#[derive(Debug)]
struct Grid {
    /// The list of all tiles
    pub tiles: Vec<Tile>,
}

impl Grid {
    /// Returns the side length of the grid
    pub fn side(&self) -> usize {
        (self.tiles.len() as f64).sqrt() as usize
    }

    /// Returns the tile at x, y coordinates
    pub fn tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(y * self.side() + x)
    }

    /// Match algorithm to find the grid layout of all tiles
    ///
    /// This function iterates over all tiles and matches neighboring tiles
    /// by rotating, flipping them. All tiles need to match the grid, e.g. 3x3 or 4x4
    ///
    pub fn find_layout(&self) -> anyhow::Result<Grid> {
        let size = self.side();

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
    fn find_tiles(visited: Vec<Tile>, tiles: Vec<Tile>, size: usize, x: u32, y: u32) -> Option<Vec<Tile>> {
        if tiles.is_empty() {
            return Some(visited);
        }

        let dir = if x < size as u32 - 1 { Dir::Right } else { Dir::Bottom };
        let current = match dir {
            Dir::Right => visited.last().unwrap(),
            _ => visited.get(visited.len() - size).unwrap(),
        };

        for (index, next) in tiles.iter().enumerate() {
            if let Some(next) = current.find_link(&next, &dir) {
                if let Some(pos) = Self::next_pos(size as u32, x, y) {
                    let mut tiles_copy = tiles.clone();
                    tiles_copy.remove(index);
                    let mut visited_copy = visited.clone();
                    visited_copy.push(next);

                    if let Some(tiles) = Self::find_tiles(visited_copy, tiles_copy, size, pos.0, pos.1) {
                        return Some(tiles);
                    }
                }
            }
        }

        None
    }

    /// Returns the next possible location in the grid
    fn next_pos(size: u32, x: u32, y: u32) -> Option<(u32, u32)> {
        if x < size - 1 {
            Some((x + 1, y))
        } else if y < size - 1{
            Some((0, y + 1))
        } else {
            None
        }
    }

    /// Calculate the product of ids from all corners
    pub fn product(&self) -> u64 {
        let size = self.side();
        [
            self.tiles[0].id,
            self.tiles[size - 1].id,
            self.tiles[(size - 1) * size].id,
            self.tiles[(size * size) - 1].id,
        ].iter().product::<u64>()
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

    let mut grid = Array2::default((size, size));
    result[1..]
        .iter().enumerate()
        .for_each(|(row, &line)| {
            line.chars().enumerate().for_each(|(col, c)| {
                grid[[row, col]] = if c == '#' { 1 } else { 0 };
            })
        });

    Ok(Tile { id, grid })
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
    // grid consists of 12x12 tiles
    let grid = parse_tile_grid(include_str!("images.txt"))?;
    assert_eq!(144, grid.tiles.len());

    let grid = grid.find_layout()?;
    dbg!(grid.product());

    let image: Image = grid.into();

    Ok(())
}

#[cfg(test)]
mod tests {
    use ndarray::arr1;
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
        assert_eq!(arr1(&[0, 0, 1, 1, 0, 1, 0, 0, 1, 0]), tile.edge(Dir::Top));
        assert_eq!(arr1(&[0, 0, 0, 1, 0, 1, 1, 0, 0, 1]), tile.edge(Dir::Right));
        assert_eq!(arr1(&[0, 0, 1, 1, 1, 0, 0, 1, 1, 1]), tile.edge(Dir::Bottom));
        assert_eq!(arr1(&[0, 1, 1, 1, 1, 1, 0, 0, 1, 0]), tile.edge(Dir::Left));
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

        assert_eq!(arr1(&[0, 1, 0, 0, 1, 1, 1, 1, 1, 0]), tile.edge(Dir::Top));
        // assert_eq!(arr1(&[0, 0, 1, 1, 0, 1, 0, 0, 1, 0]), tile.edge(Dir::Right));
        // assert_eq!(arr1(&[1, 0, 0, 1, 1, 0, 1, 0, 0, 0]), tile.edge(Dir::Bottom));
        // assert_eq!(arr1(&[0, 0, 1, 1, 1, 0, 0, 1, 1, 1]), tile.edge(Dir::Left));
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

        assert_eq!(arr1(&[0, 0, 1, 1, 1, 0, 0, 1, 1, 1]), tile.edge(Dir::Top));
        assert_eq!(arr1(&[1, 0, 0, 1, 1, 0, 1, 0, 0, 0]), tile.edge(Dir::Right));
        assert_eq!(arr1(&[0, 0, 1, 1, 0, 1, 0, 0, 1, 0]), tile.edge(Dir::Bottom));
        assert_eq!(arr1(&[0, 1, 0, 0, 1, 1, 1, 1, 1, 0]), tile.edge(Dir::Left));
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

        assert_eq!(arr1(&[0, 1, 0, 0, 1, 0, 1, 1, 0, 0]), tile.edge(Dir::Top));
        assert_eq!(arr1(&[0, 1, 1, 1, 1, 1, 0, 0, 1, 0]), tile.edge(Dir::Right));
        assert_eq!(arr1(&[1, 1, 1, 0, 0, 1, 1, 1, 0, 0]), tile.edge(Dir::Bottom));
        assert_eq!(arr1(&[0, 0, 0, 1, 0, 1, 1, 0, 0, 1]), tile.edge(Dir::Left));
    }

    #[test]
    fn test_parse_grid() {
        let grid = parse_tile_grid(TILES);
        assert!(grid.is_ok());

        let grid = grid.unwrap();
        assert_eq!(9, grid.tiles.len());
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
        assert!(left.find_link(&right, &Dir::Right).is_some());
    }

    #[test]
    fn test_find_layout() {
        let grid = parse_tile_grid(TILES).unwrap();

        let layout = grid.find_layout();
        assert!(layout.is_ok());

        let grid = layout.unwrap();
        let ids = vec![1951, 2729, 2971, 2311, 1427, 1489, 3079, 2473, 1171];
        assert_eq!(ids, grid.tiles.iter().map(|t| t.id).collect::<Vec<_>>());
        assert_eq!(20899048083289, grid.product());
    }
}
