use itertools::Itertools;

fn main() {
    // let numbers = read_numbers("./input.txt").expect("Failed to open file");
    let numbers = include_str!("input.txt")
        .lines()
        .map(str::parse::<i32>)
        .filter_map(Result::ok)
        .collect::<Vec<i32>>();

    // TODO try to use iterators here
    let sum = numbers
        .iter()
        .tuple_combinations::<(_, _, _)>()
        .find(|(a, b, c)| *a + *b + *c == 2020)
        .expect("Failed to find numbers that sum up to 2020");

    /*
    for i in 0..numbers.len() - 1 {
        let left = numbers[i];
        for j in (i + 1)..numbers.len() {
            let right = numbers[j];
            if left + right == 2020 {
                println!("{} + {} = 2020", left, right);
                sum = left * right;
                break;
            }
        }
    }
    */

    dbg!(sum);
}
