use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut times: Vec<String> = Vec::new();
    let mut distances: Vec<String> = Vec::new();
    let mut count = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut words = line.split_whitespace();
        let mut ct = 0;
        if count == 0 {
            while let Some(word) = words.next() {
                if ct > 0{
                    times.push(word.to_string());
                }
                ct += 1;
            }
        }
        else{
            while let Some(word) = words.next() {
                if ct > 0{
                    distances.push(word.to_string());
                }
                ct += 1;
            }
        }
        count += 1;
    }
    let mut result = 1;
    for i in 0..times.len(){
        let time = times[i].parse::<i32>().unwrap();
        let mut count = 0;
        for j in 0..time{
            let distance = distances[i].parse::<i32>().unwrap();
            let distance_can_be_travelled = j * (time - j);
            if distance_can_be_travelled > distance{
                count += 1;
            }
        }
        println!("count: {}", count);
        result *= count;
    }

    println!("Result: {}", result);
}
