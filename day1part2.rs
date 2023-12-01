use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut total_sum = 0;

    for result_line in io::BufReader::new(file).lines() {
        let line = result_line.expect("Failed to read line");

        let line = line.replace("one", "o1ne");
        let line = line.replace("two", "tw2o");
        let line = line.replace("three", "three3");
        let line = line.replace("four", "four4");
        let line = line.replace("five", "five5");
        let line = line.replace("six", "six6");
        let line = line.replace("seven", "seven7");
        let line = line.replace("eight", "eigh8t");
        let line = line.replace("nine", "nin9e");
        let line = line.replace("zero", "zer0o");

        let chars: String = line.chars().filter(|c| c.is_digit(10)).collect();
        let first_digit = chars.chars().next().unwrap().to_digit(10).unwrap();
        let last_digit = chars.chars().last().unwrap().to_digit(10).unwrap();

        total_sum += last_digit + (10 * first_digit);
    }

    println!("{}", total_sum);
}
