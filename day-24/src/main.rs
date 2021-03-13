use std::collections::{HashMap, HashSet};


#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

peg::parser!{
    grammar parser() for str {
        rule dir() -> Dir
            = "e" { Dir::E }
            / "se" { Dir::SE }
            / "sw" { Dir::SW }
            / "w" { Dir::W }
            / "nw" { Dir::NW }
            / "ne" { Dir::NE }

        pub(crate) rule tile() -> Vec<Dir>
            = (d:dir() { d })*
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Move to adjacent file in the given direction. Movement is based on this article:
    /// https://www.redblobgames.com/grids/hexagons/#coordinates-axial using Axial Coordinates.
    ///
    pub fn walk(&self, dir: Dir) -> Self {
        match dir {
            Dir::E => Pos::new(self.x + 1, self.y),
            Dir::SE => Pos::new(self.x, self.y + 1),
            Dir::SW => Pos::new(self.x - 1, self.y + 1),
            Dir::W => Pos::new(self.x - 1, self.y),
            Dir::NW => Pos::new(self.x, self.y - 1),
            Dir::NE => Pos::new(self.x + 1, self.y - 1),
        }
    }

    /// Get all adjacent positions
    pub fn adjacent(&self) -> HashSet<Pos> {
        let mut list = HashSet::new();
        list.insert(self.walk(Dir::E));
        list.insert(self.walk(Dir::SE));
        list.insert(self.walk(Dir::SW));
        list.insert(self.walk(Dir::W));
        list.insert(self.walk(Dir::NW));
        list.insert(self.walk(Dir::NE));
        list
    }
}

#[derive(Debug, Clone)]
struct Tile {
    pub directions: Vec<Dir>,
}

impl Tile {
    /// Parse the tile with the given list of directions
    pub fn parse(line: &str) -> anyhow::Result<Self> {
        let directions = parser::tile(line)?;

        Ok(Self {
            directions,
        })
    }

    /// Flip the last tile by walking list of given directions
    ///
    /// The reference tile is located at position x: 0, y: 0
    ///
    pub fn last_tile(&self) -> Pos {
        self.directions
            .iter()
            .fold(Pos::new(0, 0), |pos, &dir| pos.walk(dir))
    }
}

#[derive(Debug)]
struct Floor {
    pub tiles: Vec<Tile>,
    pub last_tiles: Vec<Pos>,
}

impl Floor {
    pub fn new(tiles: Vec<Tile>) -> Self {
        let last_tiles = Self::last_tiles(&tiles);

        Self {
            tiles,
            last_tiles,
        }
    }

    /// Move all tiles
    fn last_tiles(tiles: &[Tile]) -> Vec<Pos> {
        tiles
            .iter()
            .map(|tile| tile.last_tile())
            .collect::<Vec<_>>()
    }

    /// Get list of all black tiles
    pub fn black_tiles(&self) -> HashSet<Pos> {
        let frequencies = self.last_tiles
            .iter()
            .fold(HashMap::<Pos, u64>::new(), |mut map, pos| {
                *map.entry(pos.clone()).or_default() += 1;
                map
            });

        frequencies
            .iter()
            .filter(|(_, &count)| count % 2 == 1)
            .map(|(key, _)| key.clone())
            .collect::<HashSet<_>>()
    }

    /// Returns the number of black tiles
    pub fn num_black_tiles(&self) -> u64 {
        self.black_tiles().iter().count() as u64
    }

    /// Apply "game of life" rules to the floor grid of existing tiles
    /// This creates a new Grid with the new flipped tiles
    pub fn flip_tiles(&self, num_days: u64) -> u64 {
        let mut black_tiles = self.black_tiles();

        for round in 0..num_days {
            // get all relevant adjacent white tiles
            let mut white_tiles = HashSet::new();
            for black_tile in &black_tiles {
                let neighbors = black_tile.adjacent();
                for neighbor in &neighbors {
                    if !black_tiles.contains(neighbor) {
                        white_tiles.insert(neighbor.clone());
                    }
                }
            }

            let mut new_black_tiles: HashSet<Pos> = HashSet::new();

            // apply rules to all black tiles
            for black_tile in &black_tiles {
                let adjacent = black_tile.adjacent().intersection(&black_tiles).count();
                if adjacent == 1 || adjacent == 2 {
                    new_black_tiles.insert(black_tile.clone());
                }
            }

            // apply rule to all white neighbor tiles
            for white_tile in &white_tiles {
                let adjacent = white_tile.adjacent().intersection(&black_tiles).count();
                if adjacent == 2 {
                    new_black_tiles.insert(white_tile.clone());
                }
            }

            black_tiles = new_black_tiles;
        }

        black_tiles.len() as u64
    }
}

/// Parse the list of tiles / directions
fn parse_tiles(content: &str) -> anyhow::Result<Vec<Tile>> {
    let tiles = content
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .map(|line| Tile::parse(line))
        .filter_map(Result::ok)
        .collect();

    Ok(tiles)
}

fn main() -> anyhow::Result<()> {
    let tiles = parse_tiles(include_str!("tiles.txt"))?;
    let floor = Floor::new(tiles);
    dbg!(floor.num_black_tiles());

    let count = floor.flip_tiles(100);
    dbg!(count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Dir, Floor, Pos, Tile, parse_tiles};

    const TILES: &str = r#"
        sesenwnenenewseeswwswswwnenewsewsw
        neeenesenwnwwswnenewnwwsewnenwseswesw
        seswneswswsenwwnwse
        nwnwneseeswswnenewneswwnewseswneseene
        swweswneswnenwsewnwneneseenw
        eesenwseswswnenwswnwnwsewwnwsene
        sewnenenenesenwsewnenwwwse
        wenwwweseeeweswwwnwwe
        wsweesenenewnwwnwsenewsenwwsesesenwne
        neeswseenwwswnwswswnw
        nenwswwsewswnenenewsenwsenwnesesenew
        enewnwewneswsewnwswenweswnenwsenwsw
        sweneswneswneneenwnewenewwneswswnese
        swwesenesewenwneswnwwneseswwne
        enesenwswwswneneswsenwnewswseenwsese
        wnwnesenesenenwwnenwsewesewsesesew
        nenewswnwewswnenesenwnesewesw
        eneswnwswnwsenenwnwnwwseeswneewsenese
        neswnwewnwnwseenwseesewsenwsweewe
        wseweeenwnesenwwwswnew
    "#;

    #[test]
    fn test_parse_tile() {
        assert_eq!(
            vec![Dir::E, Dir::SE, Dir::NE, Dir::E],
            Tile::parse("esenee").unwrap().directions,
        );

        assert_eq!(
            vec![Dir::NW, Dir::W, Dir::SW, Dir::E, Dir::E],
            Tile::parse("nwwswee").unwrap().directions,
        );
    }

    #[test]
    fn test_parse_tiles() {
        let tiles = parse_tiles(TILES);

        assert!(tiles.is_ok());
        assert_eq!(20, tiles.unwrap().len());
    }

    #[test]
    fn test_last_tiles() {
        assert_eq!(Pos::new(3, 0), Tile::parse("esenee").unwrap().last_tile());
        assert_eq!(Pos::new(0, 0), Tile::parse("nwwswee").unwrap().last_tile());
    }

    #[test]
    fn test_flip_tiles() {
        let floor = Floor::new(parse_tiles(TILES).unwrap());
        assert_eq!(10, floor.num_black_tiles());
    }

    #[test]
    fn test_flip_floors() {
        let floor = Floor::new(parse_tiles(TILES).unwrap());

        assert_eq!(10, floor.flip_tiles(0));
        assert_eq!(15, floor.flip_tiles(1));
        assert_eq!(12, floor.flip_tiles(2));
        assert_eq!(25, floor.flip_tiles(3));
        assert_eq!(132, floor.flip_tiles(20));
        assert_eq!(2208, floor.flip_tiles(100));
    }
}
