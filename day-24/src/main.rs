use std::collections::HashMap;


#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
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

    /// Move along directions and return final tile position as tuple (x,y). Movement is based on
    /// this article: https://www.redblobgames.com/grids/hexagons/#coordinates-axial using
    /// Axial Coordinates.
    ///
    /// The reference tile is located at (0, 0)
    ///
    pub fn last_tile(&self) -> (i32, i32) {
        self.directions
            .iter()
            .fold((0, 0), |pos, dir| {
                match dir {
                    Dir::E => (pos.0 + 1, pos.1),
                    Dir::SE => (pos.0, pos.1 + 1),
                    Dir::SW => (pos.0 - 1, pos.1 + 1),
                    Dir::W => (pos.0 - 1, pos.1),
                    Dir::NW => (pos.0, pos.1 - 1),
                    Dir::NE => (pos.0 + 1, pos.1 - 1),
                }
            })
    }
}

struct Floor {
    pub tiles: Vec<Tile>,
}

impl Floor {
    pub fn new(tiles: Vec<Tile>) -> Self {
        Self {
            tiles,
        }
    }

    /// Move all tiles
    pub fn last_tiles(&self) -> Vec<(i32, i32)> {
        self.tiles
            .iter()
            .map(|tile| tile.last_tile())
            .collect::<Vec<_>>()
    }

    /// Count flipped tiles that are black
    pub fn black_tiles(&self) -> u64 {
        let tiles = self.last_tiles();
        let frequencies = tiles
            .iter()
            .fold(HashMap::<(i32, i32), u64>::new(), |mut map, &pos| {
                *map.entry(pos).or_default() += 1;
                map
            });

        frequencies
            .values()
            .filter(|&count| count % 2 == 1)
            .count() as u64
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

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Dir, Floor, Tile, parse_tiles};

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
        assert_eq!((3, 0), Tile::parse("esenee").unwrap().last_tile());
        assert_eq!((0, 0), Tile::parse("nwwswee").unwrap().last_tile());
    }

    #[test]
    fn test_flip_tiles() {
        let floor = Floor::new(parse_tiles(TILES).unwrap());
        assert_eq!(10, floor.black_tiles());
    }
}
