use crate::utils::load_puzzle_input;

pub fn solution(input_path: &str) {
    let mut id_ranges: Vec<(usize, usize)>  = Vec::new();
    let mut ids: Vec<usize> = Vec::new();

    if let Ok(input) = load_puzzle_input(input_path) {
        for line in input.map_while(Result::ok) {
            if line.as_str().contains("-") {
                id_ranges.push(parse_range(&line));
            } else if line != "" {
                ids.push(line.parse::<usize>().unwrap())
            }
        }
    }
    // part 1
    let mut num_fresh = 0;
    for id in ids {
        for (lower, upper) in &id_ranges {
            if id >= *lower && id <= *upper {
                num_fresh += 1;
                break
            }
        }
    }
    println!("Day 5 Part One Answer: {num_fresh}");
}

fn parse_range(range: &str) -> (usize, usize) {
    let split: Vec<&str> = range.split('-').collect();
    (split[0].parse::<usize>().unwrap(), split[1].parse::<usize>().unwrap())
}