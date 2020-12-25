use anyhow::anyhow;

struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn add(&mut self, dir: &Dir, width: usize) {
        self.x += dir.x % width;
        self.y += dir.y;
    }
}

struct Dir {
    pub x: usize,
    pub y: usize,
}

fn main() -> anyhow::Result<()> {
    let map = include_str!("map.txt")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let width = map.first()
        .ok_or_else(|| anyhow!("No first row found"))?
        .chars()
        .count();

    let dir = Dir { x: 3, y: 1 };
    let mut pos = Pos { x: 0, y: 0 };
    let mut count = 0;

    loop {
        if let Some(row) = map.get(pos.y) {
            if let Some('#') = row.chars().nth(pos.x) {
                count += 1;
            }
            pos.add(&dir, width);
        } else {
            break;
        }
    }

    dbg!(count);

    Ok(())
}
