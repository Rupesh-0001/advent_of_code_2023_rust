fn main() {
    let time_array = [58819676];
    let mut result = 1;
    for i in 0..time_array.len(){
        let time = time_array[i];
        let mut count = 0;
        for j in 0..time{
            let distance:i64 = 434104122191218;
            let distance_can_be_travelled = j * (time - j);
            if distance_can_be_travelled > distance{
                count += 1;
            }
        }
        result *= count;
    }
    println!("Result: {}", result);
}
