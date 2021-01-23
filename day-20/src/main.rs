use bitvec::prelude::*;
use std::fmt::Debug;

/// A tile contains image data
struct Tile {
    /// The tile id number
    pub number: u32,
    /// Grid
    pub grid: Vec<BitVec>,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = self.grid
            .iter()
            .map(|row| format!("{:010b}", row))
            .collect::<Vec<_>>();
        write!(f, "{}", lines.join("\n"))
    }
}

impl Tile {
    pub fn length(&self) -> usize {
        self.grid.len()
    }
}

/// Parses a single tile block
fn parse_tile(content: &str) -> anyhow::Result<Tile> {
    let result = content
        .lines()
        .filter(|&line| !line.is_empty())
        .map(|line| line.trim())
        .collect::<Vec<_>>();

    let size = result[0].len();
    let number = result[0][5..size - 1].parse()?;

    let grid = result[1..]
        .iter()
        .map(|&line| line.chars().map(|x| x == '.').collect::<BitVec>())
        .collect::<Vec<_>>();

    Ok(Tile {
        number,
        grid,
    })
}

/// Parses images tiles from text
fn parse_image_tiles(content: &str) -> anyhow::Result<Vec<Tile>> {
    let parts = content
        .split("\n\n")
        .map(|tile| parse_tile(tile))
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    Ok(parts)
}

fn main() -> anyhow::Result<()> {
    let tiles = parse_image_tiles(include_str!("images.txt"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{parse_image_tiles, parse_tile};

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
    fn test_parse_single_tile() {
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
        let result = parse_tile(content);
        dbg!(&result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_image_tiles() {
        let tiles = parse_image_tiles(TILES);
        assert!(tiles.is_ok());
    }
}
