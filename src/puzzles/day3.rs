use crate::utils::load_puzzle_input;

/*
There are batteries nearby that can supply emergency power to the escalator for just such an occasion. 
The batteries are each labeled with their joltage rating, a value from 1 to 9. 
You make a note of their joltage ratings (your puzzle input). For example:

987654321111111
811111111111119
234234234234278
818181911112111

The batteries are arranged into banks; each line of digits in your input corresponds to a single bank of batteries. 
Within each bank, you need to turn on exactly two batteries; 
the joltage that the bank produces is equal to the number formed by the digits on the batteries you've turned on. 
For example, if you have a bank like 12345 and you turn on batteries 2 and 4, the bank would produce 24 jolts. 
(You cannot rearrange batteries.)

The total output joltage is the sum of the maximum joltage from each bank, 
so in this example, the total output joltage is 98 + 89 + 78 + 92 = 357.

There are many batteries in front of you. 
Find the maximum joltage possible from each bank; what is the total output joltage?
*/

pub fn solution(input_path: &str) { 
    let mut banks = Vec::new();

    if let Ok(input) = load_puzzle_input(input_path) {
        for line in input.map_while(Result::ok) {
            banks.push(line);
        }
    }
    let mut total_output_joltage: usize = 0;
    for bank in banks {
        total_output_joltage+= find_largest_possible_joltage(&bank);
    }
    println!("Day 3 Part One answer: {total_output_joltage}");
}

fn find_largest_possible_joltage(bank: &str) -> usize {
    let mut current_largest_joltage = "0";
    let mut clj_index = 0;
    for i in 0..bank.len()-1 {
        let joltage = &bank[i..i+1];

        if joltage > current_largest_joltage {
            current_largest_joltage = joltage;
            clj_index = i;
        }
    }

    let mut second_largest_joltage = "0";
    for i in clj_index+1..bank.len() {
        let joltage = &bank[i..i+1];

        if joltage > second_largest_joltage {
            second_largest_joltage = joltage;
        }
    }

    let maximum_joltage = format!("{current_largest_joltage}{second_largest_joltage}").parse::<usize>().unwrap();
    //println!("{maximum_joltage}");
    maximum_joltage
}