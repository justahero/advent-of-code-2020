#[derive(Debug)]
struct Vector2 {
    pub x: usize,
    pub y: usize,
}

impl Vector2 {
    pub fn add(&mut self, dir: &Vector2) {
        self.x += dir.x;
        self.y += dir.y;
    }
}

fn main() {
    let map = include_str!("map.txt")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let dir = Vector2 { x: 3, y: 1 };
    let mut pos = Vector2 { x: 0, y: 0 };
    let mut count = 0;

    while let Some(row) = map.get(pos.y) {
        if let Some('#') = row.chars().cycle().nth(pos.x) {
            count += 1;
        }
        pos.add(&dir);
    }

    dbg!(count);
}
