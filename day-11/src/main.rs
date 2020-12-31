use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct SeatPlan {
    pub width: usize,
    pub height: usize,
    pub seats: Vec<Seat>,
}

impl Display for SeatPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.seats
            .chunks(self.width)
            .map(|row| {
                row.iter()
                    .map(|s| format!("{}", s))
                    .collect::<String>() + "\n"
            })
            .collect::<String>();

        write!(f, "{}", s)
    }
}

impl SeatPlan {
    /// Returns the longest dimension, either width or height
    pub fn max_dim(&self) -> u32 {
        u32::max(self.width as u32, self.height as u32)
    }

    /// Generates a new seat plan with updated seats, once cycle
    ///
    /// ## Parameters
    /// * `occupied` - number of occupied seats to take into account to switch from occupied to empty
    /// * `steps` - number of steps to check in each direction, mostly 1 or longest grid dimension
    pub fn update(&self, occupied: u32, steps: u32) -> Self {
        let mut new_plan = self.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let index = x + y * self.width;
                let adjacent = self.adjacent(x as i64, y as i64, steps);

                let seat = match &self.seats[index] {
                    Seat::Empty => if adjacent == 0 { Seat::Occupied } else { Seat::Empty },
                    Seat::Occupied => if adjacent >= occupied { Seat::Empty } else { Seat::Occupied },
                    Seat::Floor => Seat::Floor,
                };
                new_plan.seats[index] = seat;
            }
        }

        new_plan
    }

    /// Return number of occupied adjacent seats
    pub fn adjacent(&self, x: i64, y: i64, steps: u32) -> u32 {
        let mut result = 0;

        // define all the directions
        let dirs= vec![
            (-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)
        ];

        for (i, j) in dirs.iter() {
            let mut sx = x;
            let mut sy = y;

            for _ in 0..steps {
                sx += i;
                sy += j;

                // if adjacent seat is outside grid, advance to next
                if sx < 0 || sx >= self.width as i64 || sy < 0 || sy >= self.height as i64 {
                    continue;
                }

                let index = (sx + sy * self.width as i64) as usize;
                match self.seats[index] {
                    Seat::Occupied => {
                        result += 1;
                        break;
                    }
                    Seat::Empty => break,
                    Seat::Floor => (),
                }
            }
        }

        result
    }

    /// Returns the total number of occupied seats
    pub fn total_occupied(&self) -> usize {
        self.seats
            .iter()
            .filter(|&seat| *seat == Seat::Occupied)
            .count()
    }
}

/// Parses the input and generates a seat plan
fn parse_seat_plan(input: &str) -> SeatPlan {
    let seats = input
        .lines()
        .map(str::trim)
        .collect::<Vec<_>>();

    let height = seats
        .iter()
        .filter(|&row| !row.is_empty())
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

    SeatPlan {
        width,
        height,
        seats: result,
    }
}

fn take_seats(mut plan: SeatPlan, occupied: u32, steps: u32) -> anyhow::Result<(u64, SeatPlan)> {
    let mut iteration = 0u64;
    loop {
        let new_plan = plan.update(occupied, steps);
        if new_plan == plan {
            return Ok((iteration, new_plan));
        }
        plan = new_plan;
        iteration += 1;

        // let's skip something after a number of iterations
        if iteration >= 1_000 {
            return Err(anyhow::anyhow!("Cycles run too many iterations"));
        }
    }
}

fn take_seats_part_one(plan: SeatPlan) -> anyhow::Result<(u64, SeatPlan)> {
    take_seats(plan, 4, 1)
}

fn take_seats_part_two(plan: SeatPlan) -> anyhow::Result<(u64, SeatPlan)> {
    let dim = plan.max_dim();
    take_seats(plan, 5, dim)
}

fn main() {
    let plan = parse_seat_plan(include_str!("seats.txt"));

    let (iteration, new_plan) = take_seats_part_one(plan.clone()).unwrap();
    dbg!(iteration, new_plan.total_occupied());

    let (iteration, new_plan) = take_seats_part_two(plan).unwrap();
    dbg!(iteration, new_plan.total_occupied());
}

#[cfg(test)]
mod tests {
    use crate::{parse_seat_plan, take_seats};

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
        assert_eq!(0, parse_seat_plan(PLAN).total_occupied());
    }

    #[test]
    fn test_update_seat_plan() {
        let plan = parse_seat_plan(PLAN);
        let updated = plan.update(4, 1);

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
        "#);

        assert_eq!(expected, updated);
        assert_eq!(71, updated.total_occupied());

        let expected = parse_seat_plan(r#"
            #.LL.L#.##
            #LLLLLL.L#
            L.L.L..L..
            #LLL.LL.L#
            #.LL.LL.LL
            #.LLLL#.##
            ..L.L.....
            #LLLLLLLL#
            #.LLLLLL.L
            #.#LLLL.##
        "#);

        assert_eq!(expected, updated.update(4, 1));
    }

    #[test]
    fn test_run_take_seats() {
        let seat_plan = parse_seat_plan(PLAN);
        let (iterations, final_plan) = take_seats(seat_plan, 4, 1).unwrap();

        let expected = parse_seat_plan(r#"
            #.#L.L#.##
            #LLL#LL.L#
            L.#.L..#..
            #L##.##.L#
            #.#L.LL.LL
            #.#L#L#.##
            ..L.L.....
            #L#L##L#L#
            #.LLLLLL.L
            #.#L#L#.##
        "#);

        assert_eq!(5, iterations);
        assert_eq!(expected, final_plan);
        assert_eq!(37, final_plan.total_occupied());
    }

    #[test]
    fn test_update_plan_with_directions() {
        // empty seat sees a occupied seat in all directions
        let plan = parse_seat_plan(r#"
            .......#.
            ...#.....
            .#.......
            .........
            ..#L....#
            ....#....
            .........
            #........
            ...#.....
        "#);
        assert_eq!(8, plan.adjacent(3, 4, plan.width as u32));

        // empty seat blocks occupied seats from view
        let plan = parse_seat_plan(r#"
            .............
            .L.L.#.#.#.#.
            .............
        "#);
        assert_eq!(0, plan.adjacent(1, 1, plan.width as u32));

        // empty seat sees no occupied seats in any direction
        let plan = parse_seat_plan(r#"
            .##.##.
            #.#.#.#
            ##...##
            ...L...
            ##...##
            #.#.#.#
            .##.##.
        "#);
        assert_eq!(0, plan.adjacent(3, 3, plan.width as u32));
    }

    #[test]
    fn test_take_seats_with_part_2_directions() {
        let plan = parse_seat_plan(PLAN);
        let width = plan.max_dim();
        let (_, final_plan) = take_seats(plan, 5, width).unwrap();

        println!("{}", final_plan);

        let expected = parse_seat_plan(r#"
            #.L#.L#.L#
            #LLLLLL.LL
            L.L.L..#..
            ##L#.#L.L#
            L.L#.LL.L#
            #.LLLL#.LL
            ..#.L.....
            LLL###LLL#
            #.LLLLL#.L
            #.L#LL#.L#
        "#);

        // TODO add immediate steps here as well and check why the output is not correct!-

        assert_eq!(expected, final_plan);
        assert_eq!(26, final_plan.total_occupied());
    }
}
