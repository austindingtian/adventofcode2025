use crate::load_puzzle_input;
use std::collections::HashMap;

pub fn solution() {
    let mut dir_map: HashMap<&str, i32>= HashMap::new();
    dir_map.insert("R", 1);
    dir_map.insert("L", -1);

    let mut dial_pos = 50;
    let mut password = 0;
    if let Ok(lines) = load_puzzle_input("./src/day1/input.txt") {
        // Consume the file iterator for each line of input
        for line in lines.map_while(Result::ok) {
            // get the string slice for rotation direction and distance
            let side = &line[..1];
            let distance = &mut line[1..].parse::<i32>().unwrap();

            // multiply distance by direction
            if let Some(dir) = dir_map.get(side) {
                *distance *= *dir;
            }

            // rotate dial and update password
            dial_pos = rotate_dial(&dial_pos, *distance);
            if dial_pos == 0 { password += 1; }
        }
        println!("Day 1 Part One answer: {password}");
    }
}

// Return the new dial position after rotation
fn rotate_dial(start_pos: &i32, rotation: i32) -> i32 {
    let mut dial_pos = *start_pos;
    //println!("Moving the dial {rotation} steps");
    dial_pos += rotation;

    // dial overflow handling
    dial_pos = dial_pos % 100;
    if dial_pos < 0 {
        dial_pos = 100 + dial_pos;
    }
    //println!("The dial is now at position: {}", dial_pos);

    dial_pos
}
