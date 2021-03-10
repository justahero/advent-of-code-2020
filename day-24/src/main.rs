
#[derive(Debug)]
enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

#[derive(Debug)]
struct Tile {
    pub directions: Vec<Dir>,
}

impl Tile {
    pub fn parse(line: &str) -> anyhow::Result<Self> {
        Ok(Self {
            directions: Vec::new(),
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
    #[test]
    fn test_parse_tile() {
        let tile = Tile::parse("esenee");

        assert_eq!(
            [Dir::E, Dir::SE, Dir::NE, Dir::E],
            &tile.directions,
        )
    }
}
