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

fn count_trees(map: &[String], dir: Vector2) -> usize {
    let mut pos = Vector2 { x: 0, y: 0 };
    let mut count = 0;

    while let Some(row) = map.get(pos.y) {
        if let Some('#') = row.chars().cycle().nth(pos.x) {
            count += 1;
        }
        pos.add(&dir);
    }

    count
}

fn main() {
    let map = include_str!("map.txt")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let trees = vec![
        count_trees(&map, Vector2{ x: 1 , y: 1 }),
        count_trees(&map, Vector2{ x: 3 , y: 1 }),
        count_trees(&map, Vector2{ x: 5 , y: 1 }),
        count_trees(&map, Vector2{ x: 7 , y: 1 }),
        count_trees(&map, Vector2{ x: 1 , y: 2 }),
    ];

    dbg!(&trees);

    let count: usize = trees
        .into_iter()
        .product();

    dbg!(count);
}
