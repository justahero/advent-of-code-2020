
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
    pub fn parse(line: &str) -> anyhow::Result<Self> {
        let directions = parser::tile(line)?;

        Ok(Self {
            directions,
        })
    }
}

/// Parse the list of tiles / directions
fn parse_tiles(content: &str) -> anyhow::Result<Vec<Tile>> {
    Ok(vec![])
}

fn main() -> anyhow::Result<()> {
    let tiles = parse_tiles(include_str!("tiles.txt"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Dir, Tile};

    #[test]
    fn test_parse_tile() {
        let tile = Tile::parse("esenee").unwrap();

        assert_eq!(
            vec![Dir::E, Dir::SE, Dir::NE, Dir::E],
            tile.directions,
        )
    }
}
