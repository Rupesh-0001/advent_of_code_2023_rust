use::std::fs::File;
use::std::io::{self, BufRead};

fn main(){
    let file = File::open("input.txt").unwrap();
    let mut result = 0;
    let mut resulting_vec = vec![1; 1000];
    let mut line_count = 0;
    for line in io::BufReader::new(file).lines(){
        let line = line.expect("Failed to read line");
        let mut count = 0;
        let mut is_card_number = true;
        let mut card_numbers = vec![];
        let mut score = 0;
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
                        score += 1;
                    }
                }
            }
            count += 1;
        }
        line_count += 1;
        for i in line_count..line_count+score{
            resulting_vec[i] += resulting_vec[line_count-1];
        }
    }
    for i in 0..line_count{
        result += resulting_vec[i];
    }
    println!("{}", result);
}
