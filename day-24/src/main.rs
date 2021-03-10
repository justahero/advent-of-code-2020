
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

    /// Move along directions and return final tile position as tuple (x,y)
    /// Movement is based on this article: https://www.redblobgames.com/grids/hexagons/
    /// (odd-r horizontal layout shoves odd rows right)
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
    use crate::{Dir, Tile, parse_tiles};

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
}
