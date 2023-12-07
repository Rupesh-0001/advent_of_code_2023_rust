use std::fs::File;
use std::io::{BufRead, BufReader};
// will improve this code later after learning more about rust
fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let mut smallest_location = 100000000000;
    let mut map = "seeds";
    let mut seeds_map = Vec::new();
    let mut seed_to_soil_map = Vec::new();
    let mut soil_to_fertilizer_map = Vec::new();
    let mut fertilizer_to_water_map = Vec::new();
    let mut water_to_light_map = Vec::new();
    let mut light_to_temperature_map = Vec::new();
    let mut temperature_to_humidity_map = Vec::new();
    let mut humidity_to_location_map = Vec::new();

    for line in BufReader::new(file).lines() {
        let our_line = line.expect("Cannot read line").clone();

        for word in our_line.split_whitespace() {
            let mut chars = word.chars();

            if let Some(digit) = chars.next() {
                if digit.is_digit(10) {
                    if map == "seeds" {
                        seeds_map.push(word.to_string());
                    }
                    if map == "seed-to-soil" {
                        seed_to_soil_map.push(word.to_string());
                    }
                    if map == "soil-to-fertilizer" {
                        soil_to_fertilizer_map.push(word.to_string());
                    }
                    if map == "fertilizer-to-water" {
                        fertilizer_to_water_map.push(word.to_string());
                    }
                    if map == "water-to-light" {
                        water_to_light_map.push(word.to_string());
                    }
                    if map == "light-to-temperature" {
                        light_to_temperature_map.push(word.to_string());
                    }
                    if map == "temperature-to-humidity" {
                        temperature_to_humidity_map.push(word.to_string());
                    }
                    if map == "humidity-to-location" {
                        humidity_to_location_map.push(word.to_string());
                    }
                } else {
                    if word == "seed-to-soil" {
                        map = "seed-to-soil";
                    }
                    if word == "soil-to-fertilizer" {
                        map = "soil-to-fertilizer";
                    }
                    if word == "fertilizer-to-water" {
                        map = "fertilizer-to-water";
                    }
                    if word == "water-to-light" {
                        map = "water-to-light";
                    }
                    if word == "light-to-temperature" {
                        map = "light-to-temperature";
                    }
                    if word == "temperature-to-humidity" {
                        map = "temperature-to-humidity";
                    }
                    if word == "humidity-to-location" {
                        map = "humidity-to-location";
                    }
                }
            }
        }
    }

    for i in (0..seeds_map.len()).step_by(2){
        if i + 1 >= seeds_map.len(){
            break;
        }
        let current_seed = seeds_map[i].parse::<i64>().unwrap();
        let range_seeds = current_seed+seeds_map[i+1].parse::<i64>().unwrap();
        for k in current_seed..range_seeds{

            let soil;
            let fertilizer;
            let water;
            let light;
            let temperature;
            let humidity;
            let location;
            let seed_value = k;
            let mut is_in_range = false;
            let mut difference = 0;
            for j in (0..seed_to_soil_map.len()).step_by(3){
                if j + 2 >= seed_to_soil_map.len(){
                    break;
                }
                let source_range_start = seed_to_soil_map[j+1].parse::<i64>().unwrap();
                let source_range_end = source_range_start + &seed_to_soil_map[j+2].parse::<i64>().unwrap();
                if seed_value >= source_range_start && seed_value < source_range_end {
                    // get soil value according to source range
                    difference = source_range_start - seed_to_soil_map[j].parse::<i64>().unwrap();
                    is_in_range = true;
                    break;
                }
            }
            if is_in_range {
                soil = seed_value - difference;
            }
            else{
                soil = seed_value;
            }
            is_in_range = false;
            difference = 0;
            for j in (0..soil_to_fertilizer_map.len()).step_by(3){
                if j + 2 >= soil_to_fertilizer_map.len(){
                    break;
                }
                let source_range_start = soil_to_fertilizer_map[j+1].parse::<i64>().unwrap();
                let source_range_end = source_range_start + soil_to_fertilizer_map[j+2].parse::<i64>().unwrap();
                if soil >= source_range_start && soil < source_range_end {
                    difference = source_range_start - soil_to_fertilizer_map[j].parse::<i64>().unwrap();
                    is_in_range = true;
                    break;
                }
            }
            if is_in_range {
                fertilizer = soil - difference;
            }
            else{
                fertilizer = soil;
            }
            is_in_range = false;
            difference = 0;
            for j in (0..fertilizer_to_water_map.len()).step_by(3){
                if j + 2 >= fertilizer_to_water_map.len(){
                    break;
                }
                let source_range_start = fertilizer_to_water_map[j+1].parse::<i64>().unwrap();
                let source_range_end = source_range_start + fertilizer_to_water_map[j+2].parse::<i64>().unwrap();
                if fertilizer >= source_range_start && fertilizer < source_range_end {
                    // get water value according to source range
                    difference = source_range_start - fertilizer_to_water_map[j].parse::<i64>().unwrap();
                    is_in_range = true;
                    break;
                }
            }
            if is_in_range {
                water = fertilizer - difference;
            }
            else{
                water = fertilizer;
            }
            is_in_range = false;
            difference = 0;
            for j in (0..water_to_light_map.len()).step_by(3){
                if j + 2 >= water_to_light_map.len(){
                    break;
                }
                let source_range_start = water_to_light_map[j+1].parse::<i64>().unwrap();
                let source_range_end = source_range_start + water_to_light_map[j+2].parse::<i64>().unwrap();
                if water >= source_range_start && water < source_range_end {
                    // get light value according to source range
                    difference = source_range_start - water_to_light_map[j].parse::<i64>().unwrap();
                    is_in_range = true;
                    break;
                }
            }
            if is_in_range {
                light = water - difference;
            }
            else{
                light = water;
            }
            is_in_range = false;
            difference = 0;
            for j in (0..light_to_temperature_map.len()).step_by(3){
                if j + 2 >= light_to_temperature_map.len(){
                    break;
                }
                let source_range_start = light_to_temperature_map[j+1].parse::<i64>().unwrap();
                let source_range_end = source_range_start + light_to_temperature_map[j+2].parse::<i64>().unwrap();
                if light >= source_range_start && light < source_range_end {
                    // get temperature value according to source range
                    difference = source_range_start - light_to_temperature_map[j].parse::<i64>().unwrap();
                    is_in_range = true;
                    break;
                }
            }
            if is_in_range {
                temperature = light - difference;
            }
            else{
                temperature = light;
            }
            is_in_range = false;
            difference = 0;
            for j in (0..temperature_to_humidity_map.len()).step_by(3){
                if j + 2 >= temperature_to_humidity_map.len(){
                    break;
                }
                let source_range_start = temperature_to_humidity_map[j+1].parse::<i64>().unwrap();
                let source_range_end = source_range_start + temperature_to_humidity_map[j+2].parse::<i64>().unwrap();
                if temperature >= source_range_start && temperature < source_range_end {
                    // get humidity value according to source range
                    difference = source_range_start - temperature_to_humidity_map[j].parse::<i64>().unwrap();
                    is_in_range = true;
                    break;
                }
            }
            if is_in_range {
                humidity = temperature - difference;
            }
            else{
                humidity = temperature;
            }
            is_in_range = false;
            difference = 0;
            for j in (0..humidity_to_location_map.len()).step_by(3){
                if j + 2 >= humidity_to_location_map.len(){
                    break;
                }
                let source_range_start = humidity_to_location_map[j+1].parse::<i64>().unwrap();
                let source_range_end = source_range_start + humidity_to_location_map[j+2].parse::<i64>().unwrap();
                if humidity >= source_range_start && humidity < source_range_end {
                    // get location value according to source range
                    difference = source_range_start - humidity_to_location_map[j].parse::<i64>().unwrap();
                    is_in_range = true;
                    break;
                }
            }
            if is_in_range {
                location = humidity - difference;
            }
            else{
                location = humidity;
            }
            if location < smallest_location{
                smallest_location = location;
            }
        }
        // print the complete path
        // println!("seed: {}, soil: {}, fertilizer: {}, water: {}, light: {}, temperature: {}, humidity: {}, location: {}", seed_value, soil, fertilizer, water, light, temperature, humidity, location);
    }


    // println!("seeds_map: {:?}", seeds_map);
    // println!("seed_to_soil_map: {:?}", seed_to_soil_map);
    // println!("soil_to_fertilizer_map: {:?}", soil_to_fertilizer_map);
    // println!("fertilizer_to_water_map: {:?}", fertilizer_to_water_map);
    // println!("water_to_light_map: {:?}", water_to_light_map);
    // println!("light_to_temperature_map: {:?}", light_to_temperature_map);
    // println!("temperature_to_humidity_map: {:?}", temperature_to_humidity_map);
    // println!("humidity_to_location_map: {:?}", humidity_to_location_map);

    println!("smallest_location: {:?}", smallest_location);
    
}
