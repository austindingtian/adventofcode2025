use crate::utils::load_puzzle_input;

pub fn solution(input_path: &str, part: Option<u8>) {
    let mut id_ranges: Vec<IDRange> = Vec::new();

    // read in product id ranges and process them into list of ID Ranges
    if let Ok(input) = load_puzzle_input(input_path) {
        for line in input.map_while(Result::ok) {
            let ranges = line.split(',');
            for range in ranges {
                id_ranges.push(IDRange::split_range(range));
            }
        }
    }

    // solve for part1 or part2
    match part {
        Some(1) => part1(&id_ranges),
        Some(2) => part2(&id_ranges),
        _ => {
            part1(&id_ranges);
            part2(&id_ranges);
        }
    }
}

fn part1(id_ranges: &Vec<IDRange>) {
    // process id ranges to find invalid ids
    let mut invalid_ids: Vec<usize> = Vec::new();
    let mut invalid_sum = 0;
    for id_range in id_ranges {
        let (invalids_in_range, isum) = process_id_range(id_range, self::id_is_valid);
        invalid_ids.extend(invalids_in_range);
        invalid_sum += isum;
    }
    println!("Day 1 Part Two answer: {}", invalid_sum);
}

fn part2(id_ranges: &Vec<IDRange>) {
    // process id ranges to find invalid ids
    let mut invalid_ids: Vec<usize> = Vec::new();
    let mut invalid_sum = 0;
    for id_range in id_ranges {
        let (invalids_in_range, isum) = process_id_range(id_range, self::id_is_valid_two);
        invalid_ids.extend(invalids_in_range);
        invalid_sum += isum;
    }
    println!("Day 2 Part Two answer: {}", invalid_sum);
}

struct IDRange(usize, usize);

impl IDRange {
    // range has to be a string in the form of `id-id`
    // that can be split by the hyphen and parsed into 2 integers
    fn split_range(range: &str) -> IDRange {
        let split: Vec<&str> = range.split('-').collect();
        IDRange(split[0].parse::<usize>().unwrap(), split[1].parse::<usize>().unwrap())
    }
}

// Iterates through the range of IDs and returns the list of invalid ones
fn process_id_range(id_range: &IDRange, validity_func: fn(usize) -> bool) -> (Vec<usize>, usize) {
    let mut invalid_ids = Vec::new();
    let mut invalid_sum = 0;

    let lower = id_range.0;
    let upper = id_range.1;

    for id in lower..upper+1 {
        if !validity_func(id) {
            invalid_ids.push(id);
            invalid_sum += id;
        }
    }
    (invalid_ids, invalid_sum)
}

// Counts the number of digits in a given unsigned integer
fn count_digits(n: usize) -> usize {
    let mut temp = n;
    let mut count = 0;
    
    while temp != 0 {
        temp /= 10;
        count += 1;
    }
    count
}

// Check if a given ID is invalid or not based on the repeated sequence rule given by the puzzle
fn id_is_valid(id: usize) -> bool {
    let id_length = count_digits(id);
    
    if id_length % 2 != 0 {
        return true
    }
    
    let mut temp = id;
    let mut half = Vec::with_capacity(id_length/2);
    for _ in 0..id_length/2 {
        half.push(temp % 10);
        temp /= 10;
    }
    
    for i in 0..id_length/2 {
        if temp % 10 != half[i] { return true }
        temp /= 10;
    }
    
    return false
}

// Check if a given ID is invalid or not based on AT LEAST twice repeated sequence rule in Part Two
fn id_is_valid_two(id: usize) -> bool {
    let id_len: usize = count_digits(id);
    let mut pattern_freqs = Vec::new();

    // find divisors
    for divisor in 2..=id_len {
        if id_len % divisor == 0 { 
            pattern_freqs.push(divisor) 
        };
    }
    
    // check possible patterns for repetition
    let id_as_str = format!("{id}");
    for freq in pattern_freqs {
        let pattern_len = id_len/freq;
        let pattern = &id_as_str[..pattern_len];
        let mut count = 1;
    
        for i in (pattern_len..id_len).step_by(pattern_len) {
            let next_sequence = &id_as_str[i..i+pattern_len];
            if next_sequence != pattern {
                continue
            } else {
                count += 1;
            }
        }
        // if the pattern count is equal to the expected frequency of the pattern,
        // we have an invalid id
        if count == freq { 
            //println!("{id_as_str}");
            return false 
        };
    }
    return true
}