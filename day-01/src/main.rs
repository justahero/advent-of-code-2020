use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let numbers = read_numbers("./input.txt").expect("Failed to open file");
    let mut sum = 0;

    for i in 0..numbers.len() - 1 {
        let left = numbers[i];
        for j in i..numbers.len() {
            let right = numbers[j];
            if left + right == 2020 {
                println!("{} + {} = 2020", left, right);
                sum = left * right;
                break;
            }
        }
    }

    println!("SUM: {}", sum);
}

fn read_numbers(path: &str) -> Result<Vec<i32>, std::io::Error> {
    let mut numbers = Vec::new();

    let lines = BufReader::new(File::open(path)?).lines();
    for line in lines {
        if let Ok(line) = line {
            if let Ok(number) = line.trim().parse::<i32>() {
                numbers.push(number);
            }
        }
    }

    Ok(numbers)
}
