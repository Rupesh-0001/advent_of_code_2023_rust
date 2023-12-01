use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut total_sum = 0;

    for result_line in io::BufReader::new(file).lines() {
        let line = result_line.expect("Failed to read line");

        let chars: String = line.chars().filter(|c| c.is_digit(10)).collect();
        let first_digit = chars.chars().next().unwrap().to_digit(10).unwrap();
        let last_digit = chars.chars().last().unwrap().to_digit(10).unwrap();

        total_sum += last_digit + (10 * first_digit);
    }

    println!("{}", total_sum);
}
