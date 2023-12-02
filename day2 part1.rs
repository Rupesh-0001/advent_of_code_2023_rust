use::std::fs::File;
use::std::io::{self, BufRead};

fn main(){
    let file = File::open("input.txt").unwrap();
    let mut result = 0;
    let mut line_no = 1;

    for result_line in io::BufReader::new(file).lines(){
        let line = result_line.expect("Failed to read line");
        let line = line.replace(" ", "");
        if check_line(line){
            result += line_no;
        }
        line_no += 1;
    }
    println!("{}", result);
}

fn check_line(line: String) -> bool{
    let mut count = 0;
    let mut itr = 0;
    for chars in line.chars(){
        if chars.is_digit(10){
            if count > 0{
                let next_char = line.chars().nth(itr + 1).unwrap();
                if next_char == 'b'{
                    let num = get_complete_num(line.clone(), itr, chars);
                    if num > 14{
                        return false;
                    }
                }
                else if next_char == 'g'{
                    let num = get_complete_num(line.clone(), itr, chars);
                    if num > 13{
                        return false;
                    }
                }
                else if next_char == 'r'{
                    let num = get_complete_num(line.clone(), itr, chars);
                    if num > 12{
                        return false;
                    }
                }
            }
            count += 1;
        }
        itr += 1;   
    }
    return true;
}

fn get_complete_num(line: String, itr: usize , chars: char ) -> u32{
    let mut num = chars.to_digit(10).unwrap();
    let mut prev_itr = itr - 1;
    loop{
        let prev_char = line.chars().nth(prev_itr).unwrap();
        if prev_char.is_digit(10){
            num = num + prev_char.to_digit(10).unwrap() * 10;
            prev_itr -= 1;
        }
        else{
            break;
        }
    }
    return num;
}
