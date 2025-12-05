use crate::utils::load_puzzle_input;
use std::collections::HashMap;

pub fn solution(input_path: &str) {
    let mut dir_map: HashMap<&str, i32>= HashMap::new();
    dir_map.insert("R", 1);
    dir_map.insert("L", -1);

    let mut dial_pos = 50;
    let mut part1_password = 0;
    let mut part2_password = 0;
    if let Ok(lines) = load_puzzle_input(input_path) {
        // Consume the file iterator for each line of input
        for line in lines.map_while(Result::ok) {
            // get the string slice for rotation direction and distance
            let side = &line[..1];
            let distance = line[1..].parse::<i32>().unwrap();

            // multiply distance by direction
            if let Some(dir) = dir_map.get(side) {
                let (new_dial_pos, zero_count) = rotate_dial_and_count(dial_pos, *dir, distance);
                dial_pos = new_dial_pos;
                part2_password += zero_count;
            }
            if dial_pos == 0 { part1_password += 1; }
        }
        println!("Day 1 Part One answer: {part1_password}");
        println!("Day 2 Part Two answer: {part2_password}");
    }
}

// Return the new dial position after rotation
// Return both the new dial position and number of times a click hits 0 after rotation
fn rotate_dial_and_count(start_pos: i32, direction: i32, distance: i32) -> (i32, i32) {
    let mut dial_pos = start_pos;
    let mut zero_count = 0;
    //println!("Rotating the dial {} clicks", direction * distance);
    for _ in 0..distance {
        dial_pos += direction;
        if dial_pos == -100 || dial_pos == 0 || dial_pos == 100  { 
            zero_count += 1; 
            dial_pos = 0;
        }
    }
    //println!("The dial is now at position: {}", dial_pos);

    (dial_pos, zero_count)
}
