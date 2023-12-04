use::std::fs::File;
use::std::io::{self, BufRead};

fn main(){
    let file = File::open("input.txt").unwrap();
    let mut result = 0;
    for line in io::BufReader::new(file).lines(){
        let line = line.expect("Failed to read line");
        let mut count = 0;
        let mut is_card_number = true;
        let mut card_numbers = vec![];
        let mut card_score = 0;
        for word in line.split_whitespace(){
            if count > 1{
                if word == "|"{
                    is_card_number = false;
                    continue;
                }
                if is_card_number{
                    card_numbers.push(word);
                }
                else{
                    if card_numbers.contains(&word){
                        if card_score == 0{
                            card_score = 1;
                        }
                        else{
                            card_score *= 2;
                        }
                    }
                }
            }
            count += 1;
        }
        result += card_score;
    }
    println!("Result: {}", result);
}
