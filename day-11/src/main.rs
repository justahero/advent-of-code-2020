use std::fmt::Display;

#[derive(Debug, Eq, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl Display for Seat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            Seat::Floor => '.',
            Seat::Empty => 'L',
            Seat::Occupied => '#',
        };
        write!(f, "{}", s)
    }
}

impl From<char> for Seat {
    fn from(c: char) -> Self {
        match c {
            '.' => Seat::Floor,
            'L' => Seat::Empty,
            '#' => Seat::Occupied,
            _ => panic!("Unknown character found"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SeatPlan {
    pub width: usize,
    pub height: usize,
    pub seats: Vec<Seat>,
}

impl Display for SeatPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.seats
            .chunks(self.width)
            .flat_map(|row| row.iter().map(|s| format!("{}", s)).collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .join(" ");

        write!(f, "{}", s)
    }
}

impl SeatPlan {
    pub fn update(&self) -> Self {
        Self {
            width: 0,
            height: 0,
            seats: vec![],
        }
    }
}

/// Parses the input and generates a seat plan
fn parse_seat_plan(input: &str) -> anyhow::Result<SeatPlan> {
    let seats = input
        .lines()
        .map(str::trim)
        .collect::<Vec<_>>();

    let height = seats
        .iter()
        .count();

    let width = seats
        .iter()
        .map(|x| x.len())
        .max()
        .unwrap();

    let mut result: Vec<Seat> = vec![];
    for &row in seats.iter() {
        for seat in row.chars().into_iter() {
            result.push(seat.into());
        }
    }

    Ok(SeatPlan {
        width,
        height,
        seats: result,
    })
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::parse_seat_plan;

    const PLAN: &str = r#"
        L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL
    "#;

    #[test]
    fn test_parse_seat_plan() {
        assert!(parse_seat_plan(PLAN).is_ok());
    }

    #[test]
    fn test_update_seat_plan() {
        let plan = parse_seat_plan(PLAN).unwrap();
        let updated = plan.update();

        let expected = parse_seat_plan(r#"
            #.##.##.##
            #######.##
            #.#.#..#..
            ####.##.##
            #.##.##.##
            #.#####.##
            ..#.#.....
            ##########
            #.######.#
            #.#####.##
        "#).unwrap();

        assert_eq!(expected, updated);
    }
}
