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

    // part 2
    id_ranges.sort();
    let (mut x, mut y) = id_ranges[0];

    let mut total_fresh_ids = 0;
    for (a, b) in &id_ranges[1..] {
        let is_overlapping = *a >= x && *a <= y;

        if is_overlapping {
            match *b > y {
                true => y = *b,
                false => continue
            }
        } else {
            total_fresh_ids += 1 + (y - x);
            x = *a;
            y = *b;
        }
    }
    total_fresh_ids += 1 + (y - x);
    println!("Day 5 Part Two Answer: {}", total_fresh_ids);

    // brute force answer takes forever on non-sample puzzle input
    // let mut all_fresh_ids = HashSet::new();
    // for (lower, upper) in &id_ranges {
        // for i in *lower..=*upper {
            // all_fresh_ids.insert(i);
        // }
    // }
}

fn parse_range(range: &str) -> (usize, usize) {
    let split: Vec<&str> = range.split('-').collect();
    (split[0].parse::<usize>().unwrap(), split[1].parse::<usize>().unwrap())
}