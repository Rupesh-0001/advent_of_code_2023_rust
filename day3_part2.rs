use::std::fs::File;
use::std::io::{self, BufRead};
use::std::collections::HashMap;

fn main(){
    let file = File::open("input.txt").unwrap();
    let mut lines: Vec<Vec<char>> = Vec::new();
    for result_line in io::BufReader::new(file).lines(){
        let line = result_line.expect("Failed to read line");
        let mut line_vec: Vec<char> = Vec::new();
        for chars in line.chars(){
            line_vec.push(chars);
        }
        lines.push(line_vec);
    }

    let mut x = 0;
    let mut y = 0;
    let mut to_be_added = false;
    let mut cur_num = 0;
    let mut stars_location = [0,0];
    let mut stars: HashMap<[usize; 2], Vec<u32>> = HashMap::new();
    while y < lines.len(){
        if to_be_added{
            let entry = stars.entry(stars_location).or_insert_with(Vec::new);
            entry.push(cur_num);
            to_be_added = false;
            stars_location = [0,0];
        }
        cur_num = 0;
        while x < lines[0].len(){
            if lines[y][x].is_digit(10){
                if cur_num != 0{
                    cur_num = cur_num * 10 + lines[y][x].to_digit(10).unwrap();
                }
                else{
                    cur_num = lines[y][x].to_digit(10).unwrap();
                }
                if check_surroundings_of_digit(&lines, x, y, &mut stars_location){
                    to_be_added = true;
                }
            }
            else{
                if to_be_added{
                    let entry = stars.entry(stars_location).or_insert_with(Vec::new);
                    entry.push(cur_num);
                    to_be_added = false;
                }
                cur_num = 0;
                stars_location = [0,0];
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }
    println!("{:?}", stars);
    let mut total = 0;
    for (key, value) in stars.iter(){
        if value.len() == 2{
            println!("{}: {:?}", key[0], value);
            let gear = value[0] * value[1];
            total += gear;
        }
    }
    println!("Total: {}", total);
}

fn check_surroundings_of_digit(lines: &Vec<Vec<char>>, x: usize, y: usize, stars_location: &mut [usize; 2]) -> bool{
    let mut left = '.';
    let mut right = '.';
    let mut up = '.';
    let mut down = '.';
    let mut left_up = '.';
    let mut right_up = '.';
    let mut left_down = '.';
    let mut right_down = '.';
    if x > 0{
        left = lines[y][x-1];
    }
    if x < lines[0].len()-1{
        right = lines[y][x+1];
    }
    if y > 0{
        up = lines[y-1][x];
    }
    if y < lines.len()-1{
        down = lines[y+1][x];
    }
    if x > 0 && y > 0{
        left_up = lines[y-1][x-1];
    }
    if x < lines[0].len()-1 && y > 0{
        right_up = lines[y-1][x+1];
    }
    if x > 0 && y < lines.len()-1{
        left_down = lines[y+1][x-1];
    }
    if x < lines[0].len()-1 && y < lines.len()-1{
        right_down = lines[y+1][x+1];
    }
    if left == '*'{
        stars_location[0] = x - 1;
        stars_location[1] = y;
        return true;
    }
    else if right == '*'{
        stars_location[0] = x + 1;
        stars_location[1] = y;
        return true;
    }
    else if up == '*'{
        stars_location[0] = x;
        stars_location[1] = y - 1;
        return true;
    }
    else if down == '*'{
        stars_location[0] = x;
        stars_location[1] = y + 1;
        return true;
    }
    else if left_up == '*'{
        stars_location[0] = x - 1;
        stars_location[1] = y - 1;
        return true;
    }
    else if right_up == '*'{
        stars_location[0] = x + 1;
        stars_location[1] = y - 1;
        return true;
    }
    else if left_down == '*'{
        stars_location[0] = x - 1;
        stars_location[1] = y + 1;
        return true;
    }
    else if right_down == '*'{
        stars_location[0] = x + 1;
        stars_location[1] = y + 1;
        return true;
    }
    
    return false;
}
