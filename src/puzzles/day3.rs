use std::usize;

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

    // part 1
    let mut total_output_joltage: usize = 0;
    for bank in &banks {
        total_output_joltage+= find_output_joltage(&bank, 2);
    }
    println!("Day 3 Part One answer: {total_output_joltage}");

    // part 2 
    for bank in &banks {
        total_output_joltage+= find_output_joltage(&bank, 12);
    }
    println!("Day 3 Part Two answer: {total_output_joltage}");
}

/* 
Solution for part2
---------------------
Previously in part1 I did the same solution but hard coded for 2 batteries in the output joltage.
Here I just abstracted that to arbitrary number of batteries `n`.
So this works for part1 as well.
Idea is you have to have at least `n` batteries from the end,
so you can make sure you fill up the requisite `n` batteries in the output joltage.
Worst case scenario O(n^2) so uhh not great but alas.
 */
fn find_output_joltage(bank: &str, joltage_size: usize) -> usize {
    let bank_size = bank.len();
    let mut current_largest_joltage = "0";
    let mut next_start_index = 0;
    let mut output_joltage = String::new();

    for digits_remaining in (0..joltage_size).rev() {
        for i in next_start_index..bank_size-digits_remaining {
            let joltage = &bank[i..i+1];
    
            if joltage > current_largest_joltage {
                current_largest_joltage = joltage;
                next_start_index = i + 1;
            }
        }  
       //println!("Found largest joltage {current_largest_joltage} at index {}. Starting next iteration in this bank at index {next_start_index}", next_start_index - 1);
        output_joltage.push_str(current_largest_joltage);
        current_largest_joltage = "0";
    }
    //println!("{output_joltage}");
    output_joltage.parse::<usize>().unwrap()

}
// fn find_largest_possible_joltage(bank: &str) -> usize {
//     let mut current_largest_joltage = "0";
//     let mut clj_index = 0;
//     for i in 0..bank.len()-1 {
//         let joltage = &bank[i..i+1];

//         if joltage > current_largest_joltage {
//             current_largest_joltage = joltage;
//             clj_index = i;
//         }
//     }

//     let mut second_largest_joltage = "0";
//     for i in clj_index+1..bank.len() {
//         let joltage = &bank[i..i+1];

//         if joltage > second_largest_joltage {
//             second_largest_joltage = joltage;
//         }
//     }

//     let maximum_joltage = format!("{current_largest_joltage}{second_largest_joltage}").parse::<usize>().unwrap();
//     //println!("{maximum_joltage}");
//     maximum_joltage
// }