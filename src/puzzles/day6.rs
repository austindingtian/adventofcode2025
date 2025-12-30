use crate::utils::load_puzzle_input;

pub fn solution(input_path: &str) {
    // just get the raw puzzle input lines from the iterator
    let mut lines = Vec::new();
    if let Ok(input) = load_puzzle_input(input_path) {
        for line in input.map_while(Result::ok) {
            lines.push(line);
        }
    }
    
    // isolate the line of puzzle input containing just the problem operations
    let problem_line = &lines[lines.len()-1];
    let operations: Vec<&str> = problem_line.as_str().split_ascii_whitespace().collect();

    // populate an answers vector based on the number of problems and associated operations
    let mut answers: Vec<usize> = Vec::with_capacity(operations.len());
    for op in &operations {
        match *op {
            "+" => answers.push(0),
            "*" => answers.push(1),
            _ => (),
        }
    }

    // proces every line of octopus math from the puzzle input
    for l in &lines[..lines.len()-1] {
        let mut index = 0;
        for num in l.split_whitespace() {
            let n = num.parse::<usize>().unwrap();
            let op = operations[index];
            
            match op {
                "+" => answers[index] += n,
                "*" => answers[index] *= n,
                _ => (),
            }
            index += 1;
        }
    }

    let total = &answers.iter().fold(0,|acc, e| acc + *e);
    println!("Day 6 Part One Answer: {total}");
}