use itertools::Itertools;

fn main() {
    let numbers = include_str!("input.txt")
        .lines()
        .map(str::parse::<i32>)
        .filter_map(Result::ok)
        .collect::<Vec<i32>>();

    let (a, b, c) = numbers
        .into_iter()
        .tuple_combinations()
        .find(|(a, b, c)| a + b + c == 2020)
        .expect("Failed to find numbers that sum up to 2020");

    dbg!(a + b + c);
    dbg!(a * b * c);
}
